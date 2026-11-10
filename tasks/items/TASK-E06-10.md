# TASK-E06-10 — Validar fidelidade visual dos temas

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E06-10` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E06-10` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E06-08`, `TASK-E02-09` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Relatório de cores, medidas, estados e telas comparado à referência própria.

## Boundary

- **Não** implementa o harness de fidelidade ao MVP congelado com `egui_kittest` nem compara tela contra `vynamp-mvp-reference@0d9860b` rev.38. Isso é `TASK-E02-16`, que roda só na máquina do mantenedor e que o CI Gate marca como não executado pela QA-034; a confusão é plausível porque as duas entregas falam em captura e em fidelidade, mas aqui a referência é própria, `assets/*.svg` mais `assets/design-tokens.json`, não o MVP.
- **Não** produz o gabarito visual por DPI, o dump de geometria por elemento nem a estratégia de fontes com que se compara. É `TASK-E02-12`; esta TASK consome esses arquivos e reporta divergência, e reextrair o gabarito aqui daria duas autoridades sobre a mesma geometria.
- **Não** automatiza a regressão visual no CI nem mantém a matriz de capturas que reprova quando o PNG muda sem atualização. Isso é `TASK-E10-06`, dono da suíte automática por DPI com NVDA e Narrator; aqui o produto é um relatório de uma passada com decisão humana registrada, não um gate.
- **Não** autora os três temas nem licencia os assets deles, e não conserta o que reprova. A autoria é `TASK-E06-04` e os componentes e tokens semânticos são `TASK-E02-02`; esta TASK mede e aponta o elemento, e ajustar o token para passar no contraste é do dono, senão a medição vira autoaprovação.
- **Não** cobre locale, strings PT-BR e EN, CJK, emoji nem dados RTL, que também ocupam espaço e mudam a altura da linha. É `TASK-E02-09`, dependência declarada desta; aqui o texto é fixo por fixture e o que se mede é cor, medida e hierarquia do tema.
- **Não** valida o perfil Android nem o role mapping para Compose. Os temas e plugins mobile são `TASK-E13-08`; `assets/android-layout.svg` entra como geometria de referência própria, não como teste em aparelho, e nenhum DPI de telefone é simulado para substituir o aparelho.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e06-10-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E06-10-AC-1` | os três temas originais de `TASK-E06-04` (Classic Graphite, Midnight Modern e Copper) aplicados aos três modos de `design/02-layouts.md`: Classic Stack com janela útil 600x760 e mínimo 480x660, Library Workspace 1280x800 com mínimo 960x640, sidebar 200 e inspector de 280 que vira drawer abaixo de 1152 DIP, e Mini 420x112 mínimo; os nove estados obrigatórios normal, hover, pressed, checked, disabled, focus-visible, erro, playing e loading; e as escalas de 100, 125, 150 e 200% de DPI | cada combinação de tema, modo, estado e DPI é renderizada offscreen e o relatório confronta cor de token, medida de retângulo e posição na hierarquia contra `assets/design-tokens.json` e contra os desenhos próprios `assets/classic-layout.svg` (600x760), `assets/library-layout.svg` (1280x800), `assets/mini-layout.svg` (420x112) e `assets/android-layout.svg` (400x880), declarando quantas células foram encontradas e quantas foram cobertas | Conferir todos os modos/DPI e hierarchy clássica |
| `TASK-E06-10-AC-2` | um tema de teste com três falhas plantadas e o bloco de capturas correspondente já aprovado por revisão humana: `text-muted` #8D97AB sobre `selection` #32453A em texto de 13 DIP, que pela fórmula de contraste relativo WCAG dá 3,49:1 e fica abaixo de 4,5:1; o anel de focus-visible de 2 DIP removido por override de layout; e um botão de transporte cujo único nome acessível é o ícone, sem tooltip nem rótulo | o relatório de fidelidade é consolidado com o bloco de capturas marcado como aprovado e, no mesmo commit, a varredura de contraste por par de token e estado e a árvore semântica de controle a controle do tema são conferidas e reportadas em seção separada | falhas de acessibilidade não são ocultadas por aprovação de screenshots. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --test theme_fidelity` no repo `vynamp-player`, sobre os três temas de `TASK-E06-04` versionados em `apps/player/tests/fixtures/themes/` e os desenhos próprios do umbrella, gravando `evidence/PR/task-e06-10/report.md`. Hoje não existe harness algum: o repo do player tem um único `.rs`, `apps/player/src/main.rs` com 3 linhas e `println!` de `vynamp_player 0.1.0`, `[dependencies]` vazio no `Cargo.toml` e nenhuma dependência de egui, então nem render nem captura rodam; os três temas de `TASK-E06-04` também não existem, e `assets/*-preview.png` (classic 78,3 KiB, library 173,7 KiB, mini 8,4 KiB, android 65,0 KiB) são ilustrações estáticas, não captura do app | Esperado: a matriz 3 temas x 3 modos x 9 estados x 4 DPI soma 324 células e o relatório imprime encontradas e cobertas, com cada célula ausente nomeada por tema, modo, estado e DPI em vez de omitida; a cor é comparada por valor de token contra `assets/design-tokens.json`, e nenhuma célula pode passar com token divergente. No bloco de acessibilidade, as 3 falhas plantadas do AC-2 aparecem nomeadas mesmo com o bloco de capturas marcado como aprovado, e o par `text-muted` #8D97AB sobre `selection` #32453A é reportado com a razão medida 3,49:1 contra o limite 4,5:1; para os pares conformes, `text-primary` #F1F3F7 sobre `chassis` #252936 mede 13,04:1 e `lime` #B8F35A sobre `display` #080D0B mede 14,92:1, então um relatório que só mostre esses dois está escondendo o terceiro. |
| Hardware | um Windows 11 com GPU e monitor capaz de 100, 125, 150 e 200% de escala: o Windows 11 e a GPU integrada existem nesta máquina, um segundo monitor com escala independente e uma tela Retina de macOS não existem; as 324 células podem ser forçadas offscreen por `pixels_per_point`, mas isso não substitui a leitura humana das capturas em 200% num monitor real, e essa lacuna fica registrada como limitação. Aparelho Android e iOS não existem aqui | ausência é limitação registrada, não aprovação |

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
