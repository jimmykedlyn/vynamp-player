# TASK-E20-01 — Provar iOS arm64, bridge Rust e CoreAudio

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E20-01` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E20-01` |
| Critérios da spec | `platforms/05-ios.md` §2 |
| Fase | F6 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E11-01`, `TASK-E01-01`, `TASK-E01-02` |
| Bloqueia | `TASK-E20-02`, `TASK-E20-03` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Spike de build `aarch64-apple-ios`, ownership de buffers entre Swift e Rust, saída CoreAudio (vynamp-output) e adapter AVAudioEngine, com decisão de runtime.

## Boundary

- **Não** porta o player: prova o build, a fronteira e a saída. A UI é `TASK-E20-03`.
- **Não** promete latência de desktop. O aparelho medido é o alvo, e o resultado do
  simulador é publicado como simulador — não como device. Publicar número de device é
  `TASK-E20-06`, que fecha o relatório de aceite em aparelho real.
- **Não** usa o plugin Gradle nem `apps/android`: a cadeia de build é outra, e
  `platforms/02-android.md` fixa o caminho que o alvo Android segue.
- **Não** implementa sessão de áudio, background nem comandos remotos: isso é
  `TASK-E20-02`, e fazê-lo aqui duplicaria a decisão de categoria de sessão.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e20-01-claude-<slug>`)
- [ ] Hardware Apple, Xcode e assinatura disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E20-01-AC-1` | o alvo `aarch64-apple-ios` e a árvore de crates do núcleo | o build roda para simulador e para device | artefato de simulador e de device são distinguidos no build |
| `TASK-E20-01-AC-2` | a callback CoreAudio ativa com o núcleo Rust tocando | a callback é inspecionada durante a reprodução | callback sem alocação nem travessia para o runtime Swift |
| `TASK-E20-01-AC-3` | um WAV PCM16 e um FLAC do mesmo tom em aparelho arm64 real | cada um é aberto e reproduzido | WAV e FLAC tocam em aparelho arm64 real |
| `TASK-E20-01-AC-4` | o build de simulador pronto e o de device ausente | o resultado é registrado como evidência | comparar lifecycle e sessão de áudio antes de congelar a decisão. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `xcodebuild -showBuildSettings` e `file` sobre os dois artefatos, mais inspeção da callback em Instruments | os artefatos de simulador e de device são distinguíveis pela própria saída dos comandos, e a callback é lida no traço |
| Hardware | aparelho Apple real com Xcode e assinatura; **nenhum** disponível neste ambiente (Windows) | ausência é limitação registrada, não aprovação |

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
