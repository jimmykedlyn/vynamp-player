# TASK-E17-10 — Certificar visualizers, fallback e energia mobile

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E17-10` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E17-10` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E17-09`, `TASK-E13-05`, `TASK-E14-04` |
| Bloqueia | `TASK-E17-20` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Testes CPU/GPU/device-loss, acessibilidade, fullscreen e background Android.

## Boundary

- **Não** entrega os presets declarativos, o ring bounded e os taps pre/EQ/pós-EQ que esta certificacao consome. Esses sao TASK-E17-09, dependencia declarada; certificar sem o runtime pronto e plausivel de tentar e faria a medicao virar implementacao escondida dentro de um teste.
- **Não** executa o corpus de fixtures e hardware por OS e ABI nem o clock/seek/drain de cada backend. Isso e TASK-E14-04, outra dependencia; aqui o objeto e o renderer e a energia, nao a matriz de outputs por alvo.
- **Não** entrega a navegacao acessivel do shell phone, os tokens compartilhados nem o TalkBack da interface. Isso e TASK-E13-05; ela e dependencia porque o criterio depende de reduzir movimento e preservar foco, mas a UI em si nao se reimplementa aqui.
- **Não** implementa checkpoint, reconexao ao servico, WorkManager e wake locks limitados. Isso e TASK-E13-07; medir energia de tela apagada encosta no mesmo terreno e faria a TASK de recovery ganhar um segundo dono.
- **Não** entrega o servico em primeiro plano, a MediaSession e a notificacao que mantem a reproducao viva em background. Isso e TASK-E13-02; o teste de tela apagada depende delas, nao as substitui.
- **Não** fecha o gate combinado de codecs, DSP e visualizadores por build e alvo. Isso e TASK-E17-20, que esta TASK bloqueia; a evidencia produzida aqui e insumo do gate, e declarar o gate fechado seria aprovar cobertura que nao existe.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e17-10-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E17-10-AC-1` | sessao tocando um FLAC de 48 kHz estereo com renderer wgpu ativo em 30 fps sobre adapter de software WARP, observer ring e tap final ligados, e com o contador de underrun do backend de audio visivel no log | a GPU e perdida de forma provocada pelo harness de teste: o device wgpu e destruido durante o playback e o evento DeviceLost chega ao runtime do visualizador sem que a reproducao seja pausada | GPU perdida mantém áudio |
| `TASK-E17-10-AC-2` | aparelho Android arm64 fisico com Android 14, app em foreground tocando a 60 fps com visualizador ativo em tela cheia e servico em primeiro plano, instrumentado por adb com contador de frames do renderer exposto em logcat | a tela e apagada e a activity vai para background com o audio seguindo no servico em primeiro plano por 10 min, enquanto se conta cada chamada de render registrada no logcat e se confere o estado da sessao de midia | zero rendering Android screen-off/background |
| `TASK-E17-10-AC-3` | sessao em janela com visualizador a 30 fps, preferencia de sistema reduced motion ligada, tema de alto contraste ativo e o rotulo do meter exposto a arvore de acessibilidade, com telemetria de frame time coletada por 60 s em cada configuracao | entra-se em tela cheia pelo atalho do player e sai-se com Escape, repetindo a medicao de frame time e coletando o fps por perfil de 30, 60 e 10 fps e o custo de CPU e GPU por frame | reduced motion, foco e FPS/budgets medidos. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test -p vynamp-player --features wgpu --test visualizer_device_loss com 20 ciclos de destruicao forcada do device, mais o soak de 8 h em scripts/soak-visualizer-8h.sh com renderer artificialmente lento, mais ./gradlew :app:connectedAndroidTest --tests VisualizerBackgroundEnergyTest no aparelho Android, mais a medicao de perfil com reduced motion e fullscreen em tests/visualizer_budget.rs. | Esperado: nos 20 ciclos de device loss o audio segue sem pausa, com zero underrun registrado e o motivo do fallback para meter CPU aparecendo no log uma vez por perda; no soak de 8 h, contador de underrun igual a zero e dropped_frames de visual maior que zero, provando que renderer lento perde frame e nao audio; no Android, zero chamadas de render durante os 10 min de tela apagada, contra fps proximo de 60 em foreground; no perfil, fps medido em 30 e 60 em foreground, 10 em economia e frame time com reduced motion igual ao de meter estatico, com pico de flashes abaixo de 3 por segundo. Hoje nada disso existe: o repo vynamp-player tem um unico arquivo .rs, src/main.rs, com so println!("vynamp_player 0.1.0"), zero dependencias e nenhum diretorio tests/, entao nenhum desses alvos de teste compila e nenhuma medicao foi feita. |
| Hardware | ausente: exige aparelho Android arm64 fisico com Android 14 para o AC-2, e nao ha aparelho Android nem aparelho iOS nem maquina macOS neste ambiente Windows 11 x86_64; o AC-1 de device loss roda no adapter wgpu WARP de software e o AC-3 em janela local, ambos sem GPU fisica dedicada, entao a prova de device loss em placa real fica pendente de registra-la como limitacao. | ausência é limitação registrada, não aprovação |

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
