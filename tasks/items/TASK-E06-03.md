# TASK-E06-03 — Implementar layouts declarativos e bindings

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E06-03` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E06-03` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E06-01`, `TASK-E02-06` |
| Bloqueia | `TASK-E06-04` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | alto |

## Entrega

Renderer de nós/ações permitidos com fallback para cada modo de interface.

## Boundary

- **Não** valida o schema do layout nem resolve ciclos e ações ausentes: esse veredito é `TASK-E06-01`. O renderer aqui confia no layout já validado; reimplementar a checagem faria o mesmo layout passar num ponto e falhar no outro.
- **Não** implementa a geometria dos três modos como produto: Classic Stack e transporte são `TASK-E02-03` e Mini 420×112, preferências e estados de erro são `TASK-E02-06`. Esta TASK fornece o renderer declarativo com fallback para o builtin de cada modo; redesenhar as telas aqui duplicaria a autoridade de `design/02-layouts.md`.
- **Não** entrega tokens nem os componentes semânticos: o loader de tokens builtin e os estados de botão, slider e linha de lista são `TASK-E02-02`. O renderer consome `color_token` por nome e não define cor.
- **Não** desenha medidores nem espectro: o nó `visualizer` desta TASK é slot de layout, e o canal de observação de áudio, FFT, VU e osciloscópio são `TASK-E06-05`. Uma animação de VU aqui pareceria renderização de tema e seria outro dono do mesmo pixel.
- **Não** monta o editor de layout com preview por DPI e exportação: `TASK-E06-07`. Editar constraints e ver o resultado é a ferramenta; aplicá-las em runtime é esta TASK.
- **Não** prova acessibilidade como campanha: workflows de teclado, screenshots por DPI e checklist de leitor de tela são `TASK-E10-06`. A leitura da árvore de acessibilidade aqui é o mínimo de AC-2 e não substitui NVDA, Narrator ou TalkBack.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e06-03-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E06-03-AC-1` | dois layouts em `fixtures/themes/layout/`: `hostile.json`, com um nó cujo `binding` é `exec:sh -c\"id\"` e outro com `action` fora das sete permitidas (`play`, `pause`, `stop`, `seek`, `open-url`, `show-library`, `mute`), e um `image` com `src` igual a `https://tracker.invalid/pixel.png`; e `clean.json`, derivado de `schemas/examples/theme-layout.json`, com bindings e ações todos na allowlist e assets por caminho relativo dentro do pacote | o renderer monta cada layout e o processo é observado por um instrumento de rede e de chamada de sistema — contagem de conexões de saída e de spawn de processo — enquanto os nós são resolvidos | Nenhum código ou acesso de rede arbitrário |
| `TASK-E06-03-AC-2` | um layout `mode: mini` em 420×112 que omite os nós de `action: stop` e de `action: mute` e cujo único botão de transporte perde `accessible_name`, mais o layout mini completo como controle | o renderer compõe o modo mini e a árvore de acessibilidade é lida nó por nó, com a lista de ações exercitáveis por teclado e o nome acessível de cada controle | controles essenciais e semântica acessível permanecem |
| `TASK-E06-03-AC-3` | um layout cujo `root` declara `x: 20, y: 20, width: 600, height: 760` num `reference_width_dip` de 420 e `reference_height_dip` de 112, com um filho de 44×32 cujo `min` declarado é 24×24 e outro de 12×12 abaixo do mínimo, e com `anchor` ligando filho a pai | o renderer aplica as constraints de geometria e o dump de retângulos por nó é comparado com a janela útil de cada modo, em 100% e em 200% de DPI | constraints de geometria são aplicadas. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --manifest-path apps/player/Cargo.toml theme_render` sobre `fixtures/themes/layout/` (os quatro layouts acima, mais `schemas/examples/theme-layout.json` como controle), com um dump de retângulo, `color_token`, `binding`, `action` e `accessible_name` por nó renderizado, e a contagem de conexões de saída e de spawn de processo medida por um gancho de `std::net::TcpStream::connect` e de `Command` contados no processo de teste | esperado: `hostile.json` sai recusado antes de montar qualquer nó, com `/nodes/<índice>/binding` e `/nodes/<índice>/action` nomeados, e a contagem de conexões de saída é 0 e a de spawn de processo é 0; `clean.json` renderiza e também mede 0 e 0; o mini sem stop e mute renderiza com os dois controles presentes vindos do fallback builtin e todo controle com `accessible_name` não vazio, contra o layout completo que mantém os do tema; o layout de geometria tem o `root` recortado para a janela útil de 420×112, o filho de 12×12 elevado ao mínimo de 24×24 e o filho de 44×32 preservado, com os retângulos idênticos em 100% e 200% de DPI porque a unidade é DIP. Nada disso existe hoje: `apps/player/src/main.rs` tem 3 linhas e nenhum framework de UI está declarado — o `Cargo.toml` do player tem `[dependencies]` vazio e o umbrella não declara `egui` em nenhum manifest —, não há `tests/`, e o único código que toca tema é `xtask/src/validate.rs::schema_example`, que só valida `theme-layout.json` contra o schema draft 2020-12 e não renderiza. |
| Hardware | nenhum para renderizar e medir geometria: o renderer é offscreen e o dump é numérico em DIP, sem abrir janela nem dispositivo de áudio. O que não existe neste ambiente é a prova de leitor de tela — esta máquina é Windows 11 e teria NVDA e Narrator, mas a campanha de acessibilidade é `TASK-E10-06`, e não há aparelho Android para TalkBack nem máquina macOS para VoiceOver. | ausência é limitação registrada, não aprovação |

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
