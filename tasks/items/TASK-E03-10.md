# TASK-E03-10 — Completar recovery, export e large-library gate

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E03-10` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E03-10` |
| Critérios da spec | `specs/biblioteca.md` §5 (rascunho, não aprovado) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E03-07`, `TASK-E03-08`, `TASK-E03-09` |
| Bloqueia | `TASK-E09-08`, `TASK-E10-09`, `TASK-E12-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

Crash restore/backup/export e relatório dataset100k.

## Boundary

- **Não** define o schema SQLite, o WAL nem o write actor das migrations; a base e o mecanismo de migration são TASK-E03-01, e aqui só se prova que a migration existente sobrevive a falha e rollback.
- **Não** implementa a QueueSession nem o checkpoint de 5 s; o snapshot, as revisions, o seed e o repeat são TASK-E03-02, e esta TASK apenas consome o snapshot para provar o restore sem autoplay.
- **Não** exporta playlist em M3U, M3U8, PLS, XSPF ou CUE; esses formatos são TASK-E03-06, e o export desta TASK é de dados e métricas do dataset, não de playlist para outro player.
- **Não** reavalia smart playlists nem grava tag em lote; o AST e o refresh são TASK-E03-07 e a escrita em lote é TASK-E03-08, e o gate de 100k só mede o que os dois entregaram.
- **Não** otimiza a query port nem a virtualização da lista para bater a meta de p95; a consulta indexada é TASK-E03-05, e o relatório desta TASK publica o p95 medido mesmo quando ele fica acima de 100 ms.
- **Não** implementa os adapters de scrobble nem migra histórico para provedor externo; publicação e sync são TASK-E08-08, e o rollback aqui é do banco local.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e03-10-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E03-10-AC-1` | uma QueueSession com 500 entradas, index 137, seek em 00:02:41 (161.000 ms), shuffle seed 0x8F2A1C e repeat all, checkpointada há 5 s, com o processo morto por TerminateProcess e o binário relançado com autoplay-on-startup desligado | o binário é iniciado e o usuário clica no botão Retomar do diálogo de recuperação antes de qualquer outra ação | Fila restaura sem autoplay |
| `TASK-E03-10-AC-2` | uma biblioteca de 10.000 itens em que os 250 arquivos do root E:\music ficaram inacessíveis com a unidade desmontada, e uma playlist de 600 entradas das quais 180 apontam para esse root | o reconcile do root roda e o relatório de missing é aberto pela view da playlist | missing files têm erro agrupado |
| `TASK-E03-10-AC-3` | um banco em schema revision N com 10.000 ratings e 4.000 bookmarks, migration para a revision N+1 morta por TerminateProcess no meio, e o binário da revision N relançado | o player abre o banco na revision N, aplica o rollback pela tela de recuperação e depois refaz o upgrade até o fim | ratings/bookmarks sobrevivem upgrade e rollback seguro. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test --manifest-path apps/player/Cargo.toml --locked --test recovery_export, rodado da raiz do umbrella, sobre fixtures tests/fixtures/queue/crash_500 (500 itens, index 137, seek 161.000 ms, seed 0x8F2A1C, repeat all), tests/fixtures/library/missing_250 (250 de 10.000 itens num root com a letra E: desmontada) e tests/fixtures/db/upgrade_fail (10.000 ratings e 4.000 bookmarks com a migration N+1 morta no meio), mais a medição de busca p95 em release sobre um banco de 100.000 itens gerado em tempdir para o relatório dataset100k | espera-se a fila restaurada com 500 entradas, index 137, seek 161.000 ms e seed 0x8F2A1C inalterados, e 0 eventos de play e 0 aberturas de device de áudio antes da confirmação do usuário; o relatório agrupa as 250 falhas em 1 entrada de root com 250 ids, 0 abortos e 180 entradas puladas na playlist de 600; depois da migration morta e do rollback a contagem de ratings é 10.000 e a de bookmarks 4.000, com 0 linhas perdidas e 0 rollback parcial; o relatório dataset100k publica o p95 de busca medido sobre os 100.000 itens, com o valor que sair. O repo vynamp-player tem 1 arquivo .rs (src/main.rs) com println!("vynamp_player 0.1.0"), [dependencies] vazio e nenhum tests/; hoje cargo test --locked ali responde running 0 tests com 0 passed e o comando acima falha com `error: no test target named recovery_export in default-run packages`, e não há restore, relatório nem número de dataset100k medido. |
| Hardware | nenhum: os testes usam bancos SQLite em tempdir, kill de processo filho e uma letra de unidade E: sem mídia no Windows 11 x86_64, que devolve ERROR_NOT_READY se a letra existe sem mídia e ERROR_PATH_NOT_FOUND se ela não existe, tratado como root offline nos dois casos; nenhum dispositivo de áudio, NAS ou aparelho externo é exigido | ausência é limitação registrada, não aprovação |

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
