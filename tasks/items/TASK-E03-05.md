# TASK-E03-05 — Entregar FTS, filtros e views de biblioteca

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E03-05` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E03-05` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E03-04` |
| Bloqueia | `TASK-E02-05`, `TASK-E03-07`, `TASK-E08-04`, `TASK-E10-07` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | médio |

## Entrega

Query port indexado, albums/artists/genres/folders e pagination.

## Boundary

- **Não** avalia smart playlists nem o AST allowlisted de field, operator e value; a avaliação indexada e cancelável é TASK-E03-07, e transformar filtro em AST aqui daria dois avaliadores para a mesma linguagem.
- **Não** constrói a Library Workspace, colunas configuráveis e inspector; a tela é TASK-E02-05, e esta TASK entrega só o query port e as views que ela consome.
- **Não** cria o schema, as tabelas nem o índice FTS; schema e migrations são TASK-E03-01, e quem declarar a tabela aqui quebra a versão de migration que já vai estar aplicada.
- **Não** indexa episódios e feeds de podcast; episódios são TASK-E05-05, e o FTS de músicas desta TASK não tem coluna de feed, publicado em ou duração de episódio.
- **Não** fecha o alvo de 1M de itens nem o gate de large library de 100k ponta a ponta; o gate formal é TASK-E03-10, e aqui a medição é p95 de query em fixture, não a carga completa do índice.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e03-05-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E03-05-AC-1` | um banco de fixture com 100.000 MediaItems, 12.000 álbuns e 4.000 artistas, FTS5 populado, num processo já aquecido por 1.000 queries de aquecimento | o harness executa 2.000 queries lidas de tests/fixtures/queries.txt (termos de 1 a 3 palavras) pelo query port e mede o p95 em modo release | Search100k warmp95≤100ms |
| `TASK-E03-05-AC-2` | um pedido de filtros combinados com campo artist op contains valor x'); DROP TABLE media_item;-- mais facetas genre=Rock e year entre 1990 e 1999 | o query port monta a consulta, executa e registra o SQL emitido e o plano de execução | filtros combinados não executam SQL do usuário |
| `TASK-E03-05-AC-3` | uma biblioteca com 3.700 itens empatados no mesmo sort_title e 10 inserts concorrentes disparados no meio da paginação | a mesma query de álbuns é paginada em páginas de 50 até o fim, duas vezes, e a ordem das duas passagens é comparada | ordering estável. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test -p vynamp-player --test fts_bench --release com tests/fixtures/lib100k.sqlite (100.000 itens e FTS5 populado) e tests/fixtures/queries.txt com 2.000 termos | espera-se p95 de 100 ms ou menos e p50 abaixo de 40 ms no warm, com a coluna do SQL emitido sem nenhum trecho do valor enviado pelo usuário (só parâmetro ligado), media_item ainda com 100.000 linhas depois do filtro com payload de injeção e 0 resultados para ele, e duas paginações completas devolvendo as mesmas 100.000 linhas na mesma ordem com 0 chaves repetidas e 0 faltantes. Hoje o repo vynamp-player tem 1 arquivo .rs (src/main.rs) com println!("vynamp_player 0.1.0"), zero dependências e nenhum tests/, então não há banco, nem FTS, nem harness para medir p95. |
| Hardware | nenhum: a fixture é um arquivo SQLite com 100.000 linhas e nenhum teste abre dispositivo de áudio ou saída de som | ausência é limitação registrada, não aprovação |

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
