# TASK-E02-10 — Validar experiência desktop ponta a ponta

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-10` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-10` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E02-09`, `TASK-E06-07`, `TASK-E07-07` |
| Bloqueia | `TASK-E10-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

Relatório de jornadas, usability/a11y e ajustes de interação.

## Boundary

- **Não** roda a regressão visual automatizada por DPI, locale e reduced motion: isso é `TASK-E10-06`, que executa as capturas em 100, 125, 150 e 200%. Aqui se percorrem jornadas e se registram problemas de interação, e uma captura aprovada não é um estado vazio verificado.
- **Não** constrói o harness de fidelidade ao MVP nem compara com o clone de vynamp-mvp-reference: é `TASK-E02-16`. A confusão é plausível porque o relatório cita telas dos três modos, mas a comparação aqui é contra o catálogo de estados da spec, não contra pixels do MVP.
- **Não** implementa as telas de preferências e Mini nem os estados empty/loading/offline/recovery: `TASK-E02-06` é dona delas. Esta TASK percorre e reporta; estado faltante vira achado no relatório, não código escrito aqui.
- **Não** implementa o gerenciador de plugins nem o safe mode: `TASK-E07-07` é dono. O Permission denied e o Unsupported da tela de plugins entram como estado a verificar, com o gerenciador tratado como dependência.
- **Não** implementa nem avalia o editor e o preview de temas: `TASK-E06-07` é dono. A jornada de tema termina em escolher e aplicar um tema existente; editar token, desfazer e exportar pacote ficam fora.
- **Não** fecha o gate de M3 nem monta a matriz de evidência REQ/TASK/QA: é `TASK-E10-10`, que consome este relatório. Um relatório de jornadas sem P0/P1 não autoriza declarar requisito aceito.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e02-10-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E02-10-AC-1` | o Classic Stack com uma faixa local aberta, volume em 50% e o mapa de teclado de product/04-jornadas.md (Space play/pause, Ctrl+O abrir, Ctrl+L URL, Ctrl+F buscar, Ctrl+P preferências, Ctrl+Q sair, Alt+Left/Right anterior/próxima, setas na lista, Delete remove da playlist), num Windows 11 com teclado físico e o mouse fisicamente desconectado | uma pessoa executa só pelo teclado a jornada abrir arquivo, tocar, seek, enfileirar, abrir preferências e trocar de tema, e o log de eventos da sessão mais a captura da árvore de acessibilidade são anexados ao relatório de jornadas | Sem mouse: abrir→tocar→seek→fila→saída→tema |
| `TASK-E02-10-AC-2` | as telas de M3 do player (Classic Stack, Library Workspace, Mini, Preferências, EQ avançado, detalhes da faixa, gerenciador de jobs, rádio, podcasts, temas e o gerenciador de plugins de `TASK-E07-07`) e o catálogo de design/02-layouts.md §Estados de tela obrigatórios: Empty com CTA, Loading cancelável, Populated, Partial error, Offline sem apagar local, Unsupported com motivo, Permission denied, Conflict e Recovery | para cada tela se força, por fixture ou por manipulação de estado, a ausência de dados, a espera, a falha parcial, a falha total, a negação de permissão e o conflito, e cada estado observado é confrontado com a linha correspondente do catálogo, com tela e estado faltantes nomeados no relatório | todas telas têm loading/empty/erro/permission state verificável. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | roteiro de teclado derivado de `TASK-E02-14`, executado no binário real em Windows 11 com o mouse fisicamente desconectado e a sessão gravada em vídeo mais o log de eventos anexados em `evidence/PR/task-e02-10/`, e, no mesmo teste, uma varredura que para cada tela força cada um dos nove estados do catálogo de design/02-layouts.md e grava tela, estado e resultado; hoje o repo do player tem um único `.rs`, `apps/player/src/main.rs` com 3 linhas e `println!("vynamp_player 0.1.0")`, `[dependencies]` vazio e nenhum `tests/`, então não há tela para percorrer nem jornada para gravar | Esperado: a jornada de teclado percorre abrir, tocar, seek, fila, preferências e troca de tema com zero evento de ponteiro no log, e a matriz tela x estado cobre as 11 telas contra os 9 estados, ou seja 99 células, das quais cada célula ausente vira linha nomeada no relatório de problemas com a tela e o estado explícitos; o relatório declara as duas contagens, encontrada e coberta, e célula vazia é achado registrado, nunca omissão silenciosa. |
| Hardware | um Windows 11 com teclado físico e o apontador desligado: esta máquina é Windows 11 Pro, tem teclado Aperfeiçoado de 101 ou 102 teclas e EnumDisplayMonitors devolve exatamente 1 monitor de 1920x1080, logo não há segundo monitor para a jornada de docking herdada de `TASK-E02-08`; o único apontador é o Synaptics ClickPad embutido no notebook, que não se desconecta fisicamente e só se desabilita por software, então o 'mouse fisicamente desconectado' da AC-1 fica mais fraco que o texto pede e isso se registra; NVDA não está em C:/Program Files/NVDA e VoiceOver, aparelho Android, aparelho iOS e máquina macOS não existem neste ambiente | ausência é limitação registrada, não aprovação |

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
