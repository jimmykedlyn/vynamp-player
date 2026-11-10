# TASK-E02-09 — Completar localização e testes visuais

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-09` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-09` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E02-07`, `TASK-E02-08` |
| Bloqueia | `TASK-E02-10`, `TASK-E06-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

PT-BR/EN, screenshot suite de temas/DPI e strings pluralizadas.

## Boundary

- **Não** implementa o harness de fidelidade com `egui_kittest` nem compara tela contra o MVP congelado: isso é `TASK-E02-16`, que por decisão roda só na máquina do mantenedor. A confusão é plausível porque a entrega fala em screenshot suite; aqui a suíte cobre locale, contraste e movimento contra os próprios tokens, não contra a referência externa.
- **Não** produz o gabarito visual do MVP, o dump de geometria por elemento nem a estratégia de fontes por plataforma: é `TASK-E02-12`. Esta TASK consome as larguras e os pares de cor já decididos e mede o resultado; reextrair o gabarito aqui daria duas autoridades sobre a mesma geometria.
- **Não** desenha as telas nem cria os componentes semânticos: telas são `TASK-E02-03` e componentes e tokens são `TASK-E02-02`. Se o texto CJK estoura numa linha, a correção é do dono do painel; o que esta TASK entrega é a medida que reprova.
- **Não** define o texto dos temas nem confere a fidelidade visual deles: a autoria do tema é `TASK-E06-04` e a conferência é `TASK-E06-10`. Aqui a string é só o insumo que não pode quebrar a UI, qualquer que seja o tema aplicado.
- **Não** automatiza a captura por DPI no pacote de evidência nem no CI: isso é `TASK-E10-06`, que consome as capturas e reprova quando elas mudam sem atualização. Publicar o PNG como a própria aprovação é exatamente o que o AC-2 proíbe.
- **Não** entrega RTL completo de interface: a spec marca RTL como alvo M5 e o fechamento acessível dele é `TASK-E12-10`. Em M3 o escopo aqui é dado RTL renderizar sem quebrar a UI, não espelhar navegação, ordem de leitura nem ícones direcionais.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e02-09-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E02-09-AC-1` | um catálogo de strings por chave com pt-BR e en completos e uma fixture adversarial carregada nele: título de 120 caracteres, álbum em CJK (晴天, 晚安), emoji com ZWJ (U+1F469 U+200D U+1F3A4), nome com marca combinada (a + U+0301) e metadados em árabe e hebraico de 60 caracteres, exibidos no Display do Classic Stack, numa linha da playlist e no inspector da Library Workspace | as três telas são renderizadas offscreen nas larguras úteis de 480, 576 e 1280 DIP e cada retângulo de controle é medido contra a viewport, contando overflow, texto essencial cortado e rolagem horizontal, com a linha da playlist conferida com e sem elipse e tooltip | Textos longos/CJK/emoji e dados RTL não quebram UI |
| `TASK-E02-09-AC-2` | os pares de cor de design/01-sistema-visual.md (background #11131B, chassis #252936, panel-raised #333A4D, panel-inset #191D2A, display #080D0B, text-primary #F1F3F7, text-secondary #B8C0CF, text-muted #8D97AB, lime #B8F35A, cyan #66D9EF, amber #FFBF69, danger #FF6B7A, selection #32453A) aplicados aos estados normal, selected e disabled, com a preferência de reduced motion desligada e depois ligada na mesma build | o harness calcula a razão de contraste de cada par texto sobre fundo por estado e mede duração e deslocamento de cada transição, do marquee do título e da atualização do visualizador, nas duas passadas | estilos passam contraste e reduced motion. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test` no repo `vynamp-player` sobre uma suíte nova de render offscreen, com a fixture de strings adversariais versionada em arquivo (PT-BR, EN, CJK, emoji, árabe, hebraico e marca combinada) e as razões de contraste calculadas a partir de `assets/design-tokens.json` do umbrella; hoje o repo tem um único `.rs`, `apps/player/src/main.rs` com 3 linhas e `println!("vynamp_player 0.1.0")`, `[dependencies]` vazio no `Cargo.toml` e nenhum diretório `tests/`, então nem catálogo de strings, nem render, nem medição de contraste existem para rodar | Esperado: nas três larguras, zero controle com retângulo fora da viewport, zero rolagem horizontal e nenhum alvo essencial abaixo dos 24x24 DIP do mínimo de desktop; o texto exibido sai idêntico à fixture, sem normalização NFC aplicada ao que se mostra. Contraste: texto normal ≥4,5:1 e texto grande e indicadores essenciais ≥3:1 pelos valores medidos, com os pares selected e disabled reportados separadamente, como pede o Aceite visual de design/01-sistema-visual.md. Reduced motion: com a preferência ligada, a duração medida de toda transição, do marquee e da atualização do visualizador é 0 ms, contra os 80/160/120 ms declarados com ela desligada. HONESTIDADE MEDIDA: com os 15 tokens de `assets/design-tokens.json` calculados por WCAG 2.1 nesta máquina, `text-muted` #8D97AB sobre `panel-raised` #333A4D dá 3,86:1 e sobre `selection` #32453A dá 3,49:1, isto é, o par disabled de hoje reprova o limite de 4,5:1, enquanto `text-primary` #F1F3F7 sobre `chassis` #252936 mede 13,04:1 e sobre `display` #080D0B mede 17,62:1; um relatório que mostre só esses dois últimos está escondendo o par reprovado, e o estado disabled não se aprova por opacidade. |
| Hardware | nenhum: a suíte mede texto, contraste e duração em render offscreen determinístico, sem dispositivo de áudio e sem leitor de tela; a execução com NVDA e Narrator em 100% e 200% de DPI é de `TASK-E10-06` e não desta TASK | ausência é limitação registrada, não aprovação |

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
