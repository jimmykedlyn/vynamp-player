# TASK-E03-07 — Implementar smart playlists e templates

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E03-07` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E03-07` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E03-05`, `TASK-E03-06` |
| Bloqueia | `TASK-E03-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

AST allowlisted, folders rules, sort/group/lock e refresh.

## Boundary

- **Não** implementa a consulta indexada, o FTS e as views de biblioteca que as regras consomem; a query port é TASK-E03-05, e reimplementar o filtro dentro do avaliador criaria um segundo caminho de leitura do mesmo índice e duas semânticas de operador.
- **Não** implementa import/export M3U, M3U8, PLS, XSPF e CUE nem a revision e o undo/redo da playlist editável; esses formatos e o histórico de edição são TASK-E03-06, e a smart playlist é derivada por regra, sem entradas próprias para desfazer.
- **Não** implementa a fila persistida, o shuffle seed, o repeat policy ou o stop-after; a QueueSession é TASK-E03-02, e o AC-3 só exige que o lock não barre o play, não que a fila mude de dono.
- **Não** entrega o editor visual de regras nem os atalhos de teclado da tela de playlists; a playlist UI e o keyboard workflow são TASK-E02-04, e arrastar a árvore de regras para o renderer faria a avaliação depender de um widget.
- **Não** fecha o relatório dataset100k, o crash restore ou o export do banco; esses itens são TASK-E03-10, e medir 100k aqui trocaria avaliação cancelável por benchmark de carga.
- **Não** grava tag nem capítulo nos arquivos que a playlist seleciona; escrita em lote é TASK-E03-08, e uma regra que escreve no acervo deixaria de ser consulta somente leitura.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e03-07-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E03-07-AC-1` | uma smart playlist com AST allowlisted (field genre, operator equals, value jazz) editada para incluir operator regex e field genre_typo, sobre uma fixture SQLite com 5.000 MediaItems e índice de gênero preenchido | o save da playlist é submetido pela chamada smart_playlist.save e a resposta de erro é lida pelo chamador | Query invalid rejeitada |
| `TASK-E03-07-AC-2` | uma smart playlist com scope no acervo inteiro, limit 500 e avaliação indexada rodando em worker dedicado sobre 100.000 MediaItems em SQLite local | o handle devolvido por smart_playlist.refresh é cancelado pela chamada SmartPlaylistHandle::cancel depois de 10.000 itens avaliados | atualização cancelável |
| `TASK-E03-07-AC-3` | uma smart playlist com lock=true cuja regra seleciona 300 trilhas, e uma fila com 12 entradas dessa playlist tocando a faixa 4 com play generation 7 | o usuário dispara playback.playPlaylist na playlist travada e em seguida tenta inserir uma entrada com playlist.insert | playlist lock não bloqueia playback |
| `TASK-E03-07-AC-4` | uma smart playlist com 1.500 entradas, rating de 30 itens alterado e 5 arquivos removidos por transações concorrentes enquanto o refresh está em curso | duas leituras da playlist são feitas em paralelo pela view enquanto o refresh troca o snapshot | snapshots consistentes. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test --manifest-path apps/player/Cargo.toml --locked --test smart_playlists, rodado da raiz do umbrella, sobre fixtures tests/fixtures/smart/invalid_ast.json (operator regex e field genre_typo fora da allowlist), tests/fixtures/smart/library_100k (100.000 MediaItems gerados em tempdir), tests/fixtures/smart/locked_300 (300 trilhas com lock=true) e tests/fixtures/smart/snapshot_1500 (1.500 entradas com 30 updates de rating e 5 remoções concorrentes) | espera-se 2 de 2 queries inválidas recusadas com ValidationError e 0 linhas escritas em smart_playlist; a avaliação cancelada para com no máximo 10.000 itens avaliados, devolve status cancelled em até 50 ms e libera o worker; o play da playlist travada começa com 12 entradas na fila, o insert seguinte é recusado com PlaylistLocked e as 300 entradas continuam intactas; as duas leituras concorrentes devolvem o mesmo revision e 1.500 entradas cada. Nada disso existe hoje: o repo vynamp-player tem 1 arquivo .rs (src/main.rs) com um println!("vynamp_player 0.1.0"), [dependencies] vazio e nenhum tests/; hoje cargo test --locked ali responde running 0 tests com 0 passed e o comando acima falha com `error: no test target named smart_playlists in default-run packages`, então não há suíte para rodar antes desta TASK e nenhuma das quatro fixtures existe. |
| Hardware | nenhum: o corpus é SQLite mais fixtures sintéticas em tempdir e nenhum teste abre dispositivo de áudio; o cancelamento é observado por contador de itens avaliados, não por som | ausência é limitação registrada, não aprovação |

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
