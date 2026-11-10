# TASK-E06-04 — Produzir três temas originais

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E06-04` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E06-04` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E06-03`, `TASK-E06-02` |
| Bloqueia | `TASK-E06-07`, `TASK-E10-06`, `TASK-E13-08` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | alto |

## Entrega

Classic Graphite, Midnight Modern e Copper, com assets e licenças próprios.

## Boundary

- **Não** cria o renderer nem as constraints que exibem o tema: `TASK-E06-03` é a dona. Aqui só se produzem dados que passam por ele; corrigir o renderer no meio da produção do tema misturaria bug de código com decisão de design.
- **Não** implementa a instalação, o staging nem o rollback usados para carregar os três pacotes: `TASK-E06-02`. Os temas são entrada para esse instalador, e um caminho de carga próprio aqui daria duas portas de entrada ao mesmo `.vyntheme`.
- **Não** define a paleta canônica nem os tokens semânticos: `assets/design-tokens.json` e o loader de `TASK-E02-02` são a autoridade. Copper usa âmbar como accent e Midnight Modern usa a mesma família com menos bevel e display maior, mas nenhum dos três inventa nome de token novo — só valor por tema.
- **Não** entrega o editor, o preview por DPI e o export: `TASK-E06-07`, e não entrega a galeria, o armazenamento por usuário nem o safe mode: `TASK-E06-08`. Os três temas são o material de origem dessas duas TASKs, não a ferramenta que os edita ou lista.
- **Não** monta a regressão visual automatizada: screenshots por DPI, workflows de teclado e checklist de leitor de tela são `TASK-E10-06`, e o gabarito por tema, layout, largura e estado é `TASK-E02-12`. As capturas desta TASK provam os três pacotes; transformá-las em gate permanente invadiria o dono da regressão.
- **Não** produz os perfis phone e tablet do tema nem entrega tema para Android: `TASK-E13-08` é a dona de perfis mobile e runtime de extensões no Android, e `TASK-E14-03` do registry de perfis por OS. O fallback mobile builtin sem customização já é aceito pela spec e não é trabalho desta TASK.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e06-04-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E06-04-AC-1` | três pacotes `.vyntheme` em `themes/` — `org.vynamp.classic-graphite`, `org.vynamp.midnight-modern` e `org.vynamp.copper` — cada um com `manifest.json` de `schema_version: 1` e as três entradas `classic`, `library` e `mini` em `layouts`, apontando para `layouts/classic.json`, `layouts/library.json` e `layouts/mini.json` | cada pacote é instalado e os três modos são montados a partir dele, um de cada vez, sem cair no builtin no modo que o tema declara | Os três oferecem Classic/Library/Mini |
| `TASK-E06-04-AC-2` | os três temas instalados e o builtin como controle, com os tokens de cada um em `tokens.json` e os estados normal, hover, pressed, disabled, focus-visible e selecionado alcançados por teclado nos controles de transporte, volume e linha de lista | as capturas e os dumps de estilo computado por nó são gravados e comparados com os mesmos 15 nomes de cor de `assets/design-tokens.json`, com a razão de contraste calculada par a par | tokens, cores e estados conferidos em snapshots |
| `TASK-E06-04-AC-3` | os três pacotes com seus `assets/` (SVG e PNG próprios, todos abaixo de 4096 px por dimensão e do teto de 200 arquivos), o `license.txt` de cada um e o hash SHA256 declarado por asset no manifest | um verificador percorre os assets dos três pacotes, recomputa cada sha256 e confronta os SVGs e PNGs com o inventário já versionado de `assets/icons/` e `assets/branding/asset-manifest.json` | nenhum asset copiado. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --manifest-path apps/player/Cargo.toml themes_original` sobre os três pacotes de `themes/`, com captura offscreen e dump de estilo computado por nó nos seis estados, mais um verificador que recomputa sha256 de cada asset (`Get-FileHash -Algorithm SHA256` no Windows, `sha256sum` no Linux) e o confronto dos SVGs e PNGs com `assets/icons/manifest.json` e `assets/branding/asset-manifest.json` | esperado: os três manifests validam com 0 erro e cada um resolve os três modos com 0 fallback; o dump mostra 15 tokens de cor por tema e a razão de contraste medida fica em text-primary/chassis 13,04:1, text-secondary/chassis 7,92:1, lime/chassis 11,05:1, cyan/chassis 8,79:1 e amber/chassis 8,91:1 no Classic Graphite, todos acima de 4,5:1, com os pares de selected e disabled reportados separadamente; o sha256 recomputado de cada asset bate com o manifest e 0 asset coincide com qualquer arquivo de `assets/icons/` (15 SVG em 3 famílias) ou de `assets/branding/` (1 PNG aprovado de 1536×1024 e 2 referências). Hoje não existe nenhum dos três pacotes: a busca por `*.vyntheme` no repositório devolve 0 arquivo, e o único material de tema é `assets/design-tokens.json`, que tem 15 cores, 5 espaçamentos, 7 tipografias, 8 geometrias e 5 movimentos do Classic Graphite apenas, e `schemas/examples/theme-manifest.json`, que é declarado no `schemas/README.md` como dado fictício não instalável com `LicenseRef-ExampleOnly`. Não há captura por estado, nem harness de render — `apps/player/src/main.rs` tem 3 linhas de `println!` —, e a regressão visual do MVP é da referência web (`reference/vynamp-mvp/out/themes/themes.json`, 12 temas de famílias aimp, poweramp e winamp), não destes três. |
| Hardware | nenhum para produzir os pacotes: tokens, layouts, assets e licenças são arquivos versionados, e a conferência é hash mais captura offscreen. O que falta neste ambiente é o alvo móvel: esta máquina é Windows 11 x64, sem aparelho Android e sem macOS, então os perfis phone e tablet de `TASK-E13-08` e qualquer verificação em DPI/densidade de telefone ficam declarados ausentes — nenhum dos três temas desta TASK os exige. | ausência é limitação registrada, não aprovação |

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
