# TASK-E03-03 — Implementar import incremental e roots offline

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E03-03` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E03-03` |
| Critérios da spec | `specs/biblioteca.md` §5 (rascunho, não aprovado) |
| Fase | M2 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E03-01`, `TASK-E01-07` |
| Bloqueia | `TASK-E03-04`, `TASK-E13-04` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | médio |

## Entrega

Scan jobs, root watcher/debounce, availability e locator identity.

## Boundary

- **Não** cria o schema SQLite nem as migrations de media_item e source; o schema é TASK-E03-01, e quem criar tabela aqui abre um segundo dono do mesmo schema e quebra a migration já versionada.
- **Não** implementa a varredura Android via MediaStore/SAF nem permissões de storage; o root Android é TASK-E13-04, e o watcher de filesystem do desktop não enxerga content provider.
- **Não** lê tags nem extrai capa dos arquivos durante o scan; os adapters de tag são TASK-E03-04, e puxar metadata aqui faria o import pagar leitura de container duas vezes pelo mesmo arquivo.
- **Não** faz dedupe por full hash nem move arquivos para organização; dedupe e organização são TASK-E08-04, e a identity desta TASK é deliberadamente barata (locator mais fingerprint), sem hashear o acervo no primeiro launch.
- **Não** fecha o gate de 100k nem a recuperação de crash do índice; esse gate e o recovery são TASK-E03-10, e antecipá-los aqui trocaria scan incremental por benchmark de carga.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e03-03-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E03-03-AC-1` | um root UNC \\nas\music registrado em Source com health available e 2.000 MediaItems indexados; o share é desmontado e o path passa a retornar ERROR_BAD_NETPATH, sem que ninguém remova o root do banco | o scan job de reconciliação periódica do root roda até o fim e a view de músicas do root é reaberta | NAS indisponível não deleta item |
| `TASK-E03-03-AC-2` | uma fixture de tempdir com a/ contendo symlink para ../b, b/ contendo symlink para ../a, c/ contendo symlink para si próprio e 2 arquivos .flac reais distribuídos nos níveis | o scan job desse root é executado até a conclusão e o relatório de conclusão é lido, comparando diretórios visitados, itens indexados e ciclos cortados com o esperado para 2 arquivos | symlink loop limitado |
| `TASK-E03-03-AC-3` | um root local com 5.000 arquivos .flac, commit em páginas de 100 itens e um scan job já com 1.200 itens commitados no SQLite | o cancel do job é acionado pela chamada ScanHandle::cancel e o job retorna ao chamador | cancel mantém itens já indexados |
| `TASK-E03-03-AC-4` | um MediaItem com rating 5, bookmark em 00:01:30 e locator D:\music\a\track.flac, cujo conteúdo é copiado para D:\music\b\track.flac e apagado do path antigo | o reconcile do root roda sobre os dois paths e o item é aberto pela view Favoritos | move preserva ratings. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test -p vynamp_player --test scan_roots com as fixtures tests/fixtures/roots/nas-offline (2.000 itens e o path do root respondendo ERROR_BAD_NETPATH), tests/fixtures/roots/loop (a aponta para b, b aponta para a e c aponta para si) e tests/fixtures/roots/move (rating 5 mais bookmark em 90 s) | espera-se 2.000 linhas ainda em media_item com health offline e 0 exclusões depois do scan do NAS; o scan do loop termina sem estouro de pilha, com os 3 ciclos cortados e registrados no relatório e exatamente 2 itens indexados, e o limite de profundidade usado é o que o relatório do job declara; o cancel preserva todos os 1.200 itens que já tinham indexed_at antes do pedido, com 0 deles removidos, o job fecha em estado cancelled e nenhuma transação fica aberta no write actor; o item movido mantém o mesmo id com rating 5 e bookmark 90000 ms. Hoje o repo vynamp-player tem 1 arquivo .rs (src/main.rs) com println!("vynamp_player 0.1.0"), zero dependências e nenhum tests/, então nenhum desses testes existe nem roda. |
| Hardware | nenhum: a indisponibilidade do NAS é induzida por um path UNC de host inexistente, que devolve ERROR_BAD_NETPATH no Windows 11 desta máquina sem share SMB montado, e os ciclos são symlink e junction locais em tempdir; nenhum passo do teste abre dispositivo de áudio ou aparelho externo | ausência é limitação registrada, não aprovação |

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
