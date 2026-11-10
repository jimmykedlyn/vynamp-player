# TASK-E02-05 — Construir Library Workspace e inspector

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-05` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-05` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E02-04`, `TASK-E03-05` |
| Bloqueia | `TASK-E02-06`, `TASK-E12-01`, `TASK-E12-02` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | médio |

## Entrega

Sidebar/search/album grid/track list/inspector/now playing.

## Boundary

- **Não** implementa o índice de busca, o FTS, os filtros nem a paginação que o campo da topbar consome. O dono é `TASK-E03-05`, e a confusão é plausível porque o AC-3 fala em busca cancelável: o que esta TASK entrega é a UI que dispara, mostra e cancela a consulta, não a consulta nem o schema por trás dela.
- **Não** constrói a fila, o snapshot nem a revisão que o AC-3 protege. `TASK-E03-02` é a dona; aqui só se prova que a busca não toca neles, lendo snapshot e revisão antes e depois, e trazer a fila para cá faria duas fontes de verdade para a mesma revisão.
- **Não** redefine a lista virtualizada da playlist. `TASK-E02-04` é a dona da lista com drag insert, seleção, context menu e queue actions; o Library Workspace usa o mesmo componente, mas não muda o comportamento dele nem o roteiro de teclado.
- **Não** entrega o Mini, a navegação de preferências nem os estados Empty/Loading/Offline/Recovery como tela navegável. `TASK-E02-06` é a dona; aqui os painéis aparecem em Populated e os demais estados entram na TASK seguinte, com CTA e anúncio próprios.
- **Não** implementa a cadeia de áudio nem o EQ avançado que o inspector lista como seção. Quem integra cadeia real, editor de curva e preset por dispositivo é `TASK-E02-07`; esta TASK reserva a seção e mostra o que `TASK-E04-04` e `TASK-E04-06` expuserem.
- **Não** cria tokens de tema nem componentes semânticos novos. `TASK-E02-02` entrega tokens e estados de botão, slider e linha de lista, e `TASK-E02-03` entrega o Classic Stack; o Library Workspace é um modo novo sobre os mesmos componentes, não um segundo sistema visual.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e02-05-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E02-05-AC-1` | um índice sintético de 100 000 faixas em 25 000 álbuns, gerado por semente fixa dentro do repo do player, sem rede e sem arquivo de mídia real, carregado pelo modelo de biblioteca e aberto no Library Workspace a 1280×800 DIP, com o grid de álbuns e a lista de faixas alternáveis | a lista e o grid são rolados do topo ao fim, primeiro por Page Down e depois por arraste da barra, e em cada quadro se conta quantos nós de faixa e de álbum foram materializados contra o total do modelo, mais o pico de alocação por quadro | 100k rows sem widget por item |
| `TASK-E02-05-AC-2` | o Library Workspace na largura útil exata de 1152 DIP, com sidebar de 200 DIP, painel central de 480 DIP e inspector direito de 280 DIP aberto em estado Populated com capa, créditos, tags, letras e cadeia de áudio | a janela é redimensionada 1 DIP por vez para 1151 DIP e depois para o mínimo de 960×640, e por fim volta a 1280×800, lendo em cada largura a posição do inspector, a largura do painel central, o retângulo de cada região e a ordem de foco | inspector vira drawer abaixo1152DIP |
| `TASK-E02-05-AC-3` | a faixa A tocando, com o snapshot e o número de revisão da fila capturados antes da busca, o campo de busca da topbar vazio e com foco, e o índice de 100 000 faixas já aquecido respondendo em até 100 ms | digita-se um termo caractere a caractere e, no meio da digitação, pressiona-se Escape para cancelar a busca em andamento; em seguida digita-se o termo de novo e espera-se o resultado completo, lendo ao fim o snapshot, a revisão da fila e o estado do transporte | search cancelável não altera fila. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | Três suites num crate de UI do próprio repo do player: `cargo test -p vynamp-player-ui --test library_virtualization --locked` sobre o índice sintético de 100 000 faixas em 25 000 álbuns gerado por semente fixa; `cargo test -p vynamp-player-ui --test inspector_breakpoint --locked` medindo 1152, 1151 e 960×640 DIP; e `cargo test -p vynamp-player-ui --test search_cancel --locked` com um índice falso que conta cancelamentos e um snapshot de fila lido antes e depois. A comparação visual fica no harness local de `TASK-E02-16`, que não roda no CI Gate. | Esperado: nós de faixa e de álbum materializados limitados ao viewport mais o overscan declarado (dezenas, não 100 000), nenhum widget criado por item no modelo e alocação por quadro estável durante a rolagem; em 1151 DIP o inspector vira drawer e o painel central mantém no mínimo 480 DIP, e em 1152 e 1280 DIP ele volta à coluna de 280 DIP; cancelar a busca deixa snapshot e revisão da fila idênticos ao valor lido antes. Hoje nada disso existe: `apps/player` tem 1 arquivo `.rs` (`src/main.rs`, 3 linhas, só `println!`), `[dependencies]` vazia, `Cargo.lock` com 1 pacote, nenhum diretório `tests/`, nenhuma dependência de UI e zero ocorrências de `egui` no lock; `cargo test --all --locked` roda 0 testes em 1 suíte e `apps/player/specs/player.md` §5 continua PENDENTE. Os valores de 100 000 faixas, 1152/1151 DIP e p95 de 100 ms vêm da especificação, não de uma medição. |
| Hardware | nenhum: a prova é contagem de nós materializados, geometria em DIP e revisão de snapshot sobre índice sintético, sem abrir dispositivo de áudio e sem exigir GPU dedicada (a máquina tem Intel UHD Graphics 620 integrada, e rasterizar 1280×800 DIP não pede mais que isso); o único recurso de ambiente é a tela única de 1920×1080 a 96 DPI medida aqui, suficiente para 1152 e 1280 DIP, e a ausência de segunda tela não afeta esta TASK (multi-monitor é `TASK-E02-08`) | ausência é limitação registrada, não aprovação |

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
