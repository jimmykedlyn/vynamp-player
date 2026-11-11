# TASK-E08-08 — Entregar scrobblers e presence

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E08-08` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E08-08` |
| Critérios da spec | `specs/player.md` §5 (rascunho, não aprovado) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E03-09`, `TASK-E07-03` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

Adapters autorizados de Last.fm/ListenBrainz/Libre.fm/Discord, opt-in e revogáveis.

## Boundary

- **Não** implementa o keychain em si, o schema de config nem a redaction de log. Isso é TASK-E11-05; o token tem que terminar no keychain, mas escrever armazenamento seguro e migração de config aqui faria dois donos do mesmo arquivo de credenciais.
- **Não** define grants de rede nem a política de egress do processo. Isso é TASK-E07-03; adapters autorizados soa como permissionamento, e a permissão de abrir socket para `*.last.fm` é do broker, não do scrobbler.
- **Não** implementa OIDC/PKCE, roles nem isolamento por conta do serviço Vynamp. Isso é TASK-E09-03; revogar conta e revogar token de scrobble parecem a mesma operação e não são, e aqui o token é do usuário contra o serviço de terceiro.
- **Não** implementa deep links nem callback HTTP de autorização de provider. Isso é TASK-E16-10; o fluxo de autorização de scrobbler costuma pedir uma URL de retorno, mas quem registra esquema e callback no SO é essa outra TASK.
- **Não** entrega histórico local de reprodução, bookmarks de faixa nem os provedores opt-in de letra. Isso é TASK-E03-09; o deliverable de lá também fala em opt-in providers, e uma sessão de escuta de 200 reproduções parece histórico desta TASK.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e08-08-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E08-08-AC-1` | instalação nova com `config.toml` padrão, `scrobble.lastfm=off`, `scrobble.listenbrainz=off`, `scrobble.librefm=off` e `presence.discord=off`, e um proxy de captura em `127.0.0.1:8080` exportado como `HTTP_PROXY` e `HTTPS_PROXY` do processo, que conta cada conexão aceita e cada requisição recebida | o app reproduz uma faixa local de 3 min 30 s de um FLAC de 44,1 kHz desde o início até 100% e o arquivo é fechado | Sem requests antes de opt-in |
| `TASK-E08-08-AC-2` | opt-in concedido para Last.fm e ListenBrainz, com as chaves `lfm_test_9f3c` e `lb_test_7a12` devolvidas pelo stub de autorização em `127.0.0.1:18080`, o Windows Credential Manager do host sem nenhuma entrada desses dois serviços e o log do app em nível debug | o app recebe as duas chaves pelo fluxo de autorização, o processo é encerrado e o app é reaberto carregando o estado de autorização dos dois adapters | tokens no keychain |
| `TASK-E08-08-AC-3` | stub Last.fm em `127.0.0.1:18080` que responde 503 nas três primeiras chamadas de `track.scrobble` e 200 na quarta, com a chave de idempotência `(artist, track, timestamp)` já gravada no spool em disco para a faixa tocada | a fila de scrobble reprocessa a sessão de escuta de uma faixa de 3 min 30 s depois do backoff, e o processo do app é encerrado e reiniciado no meio da espera | retries idempotentes |
| `TASK-E08-08-AC-4` | Last.fm e Discord autorizados, um scrobble já aceito com 200 no stub, o botão Revogar acionado na tela de conta removendo as duas autorizações e o proxy de captura ainda ativo em `127.0.0.1:8080` | uma nova faixa de 3 min 30 s toca até 100% e o app executa o ciclo de envio de scrobble e de presence do Discord | revoke impede novos envios. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --test scrobble` no repo `vynamp-player` contra o stub de Last.fm e ListenBrainz em `127.0.0.1:18080` com a sequência 503, 503, 503, 200 e um contador de requisições por rota, mais o contador do proxy de captura em `127.0.0.1:8080` nas quatro fases, `grep` de `config.toml` e do log depois do opt-in e `cmdkey /list` para o keychain | Hoje o repo `vynamp-player` tem 1 arquivo `.rs` (`src/main.rs`) com `println!("vynamp_player 0.1.0")`, zero dependências, nenhum `tests/` e nenhuma entrada de `config.toml` de scrobble, e as chaves de teste ainda não existem em lugar nenhum: não há cliente HTTP, nem keychain, nem fila de scrobble, então o alvo de teste não compila. Depois da TASK a evidência esperada é 0 conexões contadas pelo proxy antes do opt-in, exatamente 4 POSTs de `track.scrobble` para uma faixa depois do retry com 1 scrobble aceito e 0 duplicatas no spool, 0 ocorrências de `lfm_test_9f3c` ou `lb_test_7a12` em `config.toml` e no log (grep de 0 linhas) com 2 entradas novas listadas por `cmdkey /list`, e 0 conexões novas ao proxy depois da revogação. |
| Hardware | nenhum: os adapters falam com stub HTTP em loopback e o keychain usado é o Windows Credential Manager do próprio host; nenhuma conta real de Last.fm, ListenBrainz, Libre.fm ou Discord e nenhum dispositivo de áudio entram no caminho. | ausência é limitação registrada, não aprovação |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final
## Notas de execução

Preenchido durante a execução, não antes.
