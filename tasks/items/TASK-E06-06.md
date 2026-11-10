# TASK-E06-06 — Entregar visualizações avançadas

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E06-06` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E06-06` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E06-05` |
| Bloqueia | `TASK-E06-07`, `TASK-E17-09` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Espectrograma, partículas, fullscreen, presets e fallback de CPU.

## Boundary

- **Não** define o catálogo de presets por binding nem a política de taps. O dono é `TASK-E17-09`, que esta TASK bloqueia; aqui existe o executor de preset e a recusa de shader, não o catálogo.
- **Não** faz o editor de tokens e layout nem o preview por DPI, densidade e estado. O dono é `TASK-E06-07`, que depende desta TASK; fullscreen e preview parecem o mesmo trabalho e não são.
- **Não** implementa a FFT, o VU, o espectro e o osciloscópio base. O dono é `TASK-E06-05`, dependência desta TASK; aqui se consome o sinal do canal de observação e não se refaz a transformada.
- **Não** valida fidelidade visual com `egui_kittest` nem contra screenshots de referência. O dono é `TASK-E06-10`; um teste de fumaça de fallback não aprova fidelidade de layout.
- **Não** define docking nem persistência de posição entre monitores. O dono é `TASK-E02-08`; entrar e sair de fullscreen muda a janela, mas restaurar multi-monitor é outro contrato.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e06-06-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E06-06-AC-1` | app iniciado com o backend de renderização forçado a falhar na criação do adapter (`WGPU_BACKEND=null`) e uma faixa de 48 kHz tocando desde o início | o visualizador é aberto em fullscreen e 30 s de reprodução são observados até o fim | GPU indisponível não interrompe áudio |
| `TASK-E06-06-AC-2` | visualizador em fullscreen num monitor de 2560x1440 com escala 100%, janela do player com foco de teclado e nenhum diálogo modal aberto | a tecla Escape é pressionada uma vez no teclado | Escape sai do fullscreen |
| `TASK-E06-06-AC-3` | pacote `evil.vyntheme` cujo manifest declara `shader: assets/evil.wgsl`, fora do catálogo allowlisted de presets builtin (`bars`, `wave`, `particles`) | o pacote é importado pela galeria e o preset declarado nele é ativado no visualizador | presets não executam shaders arbitrários. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test --test viz_presets (fixtures: `fixtures/viz/evil.vyntheme` com `shader: assets/evil.wgsl`, um `evil.wgsl` de 3 KiB no pacote e o preset builtin `bars`) | hoje não existe renderer: o repo tem 1 arquivo `.rs` com `println!("vynamp_player 0.1.0")`, zero dependências e nenhum `tests/`; a evidência exigida é 0 interrupções no áudio e 0 xruns em 30 s com adapter nulo mais exatamente 1 aviso de fallback para CPU no log, saída de fullscreen com 1 evento de Escape devolvendo a janela a 1280x720, e rejeição de `evil.vyntheme` com 0 arquivos `.wgsl` do pacote compilados. |
| Hardware | nenhum: a indisponibilidade de GPU é simulada por `WGPU_BACKEND=null` e a tela por uma janela winit em 1280x720; nenhum dos três critérios depende de GPU física desta máquina, de tela adicional ou de dispositivo de áudio. | ausência é limitação registrada, não aprovação |

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
