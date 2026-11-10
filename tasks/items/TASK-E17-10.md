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

- **Não** entrega os presets declarativos, o ring bounded e os taps source, pre-eq, post-eq e final que esta certificacao consome. Esses sao TASK-E17-09, dependencia declarada, e a confusao e plausivel porque medir o ring exige ajustar seu limite; corrigir o runtime dentro do teste faria a medicao virar implementacao escondida.
- **Não** executa o corpus de fixtures e hardware por OS e ABI nem o clock, seek e drain de cada backend. Isso e TASK-E14-04, outra dependencia; aqui o objeto e o renderer e o consumo de energia, nao a matriz de outputs por alvo, e a matriz pronta de E14-04 entra como linha de base.
- **Não** entrega a navegacao acessivel do shell phone, os tokens compartilhados nem o TalkBack da interface. Isso e TASK-E13-05, dependencia declarada; o AC-3 depende de reduced motion e de foco preservado, mas o que se mede aqui e frame time e custo por frame, nao a arvore de acessibilidade em si.
- **Não** implementa checkpoint, reconexao ao servico, WorkManager nem wake locks limitados. Isso e TASK-E13-07; medir energia de tela apagada encosta no mesmo terreno de gerenciar o ciclo de vida, e a TASK de recovery ganharia um segundo dono.
- **Não** entrega o servico em primeiro plano, a MediaSession e a notificacao que mantem a reproducao viva em background. Isso e TASK-E13-02; o teste de tela apagada depende delas e nao as substitui, e escrever o servico aqui esconderia a dependencia.
- **Não** fecha o gate combinado de codecs, DSP e visualizadores por build e alvo. Isso e TASK-E17-20, que esta TASK bloqueia; a evidencia produzida aqui e insumo do gate, e declarar o gate fechado aprovaria cobertura que nao existe.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e17-10-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E17-10-AC-1` | sessao tocando o FLAC estereo de 48 kHz libs/decode/tests/fixtures/flac/st16-level5.flac, cuja origem ref/st16-48k.wav esta registrada no MANIFEST.txt daquele diretorio, com renderer wgpu a 30 fps sobre o adapter de software WARP, observer ring e tap final ligados, e o contador source_underruns de vynamp-output-core visivel no log | o harness destroi o device wgpu no meio do playback e injeta o evento DeviceLost no runtime do visualizador, amostrando a cada 100 ms o valor de source_underruns, o estado de playback e a linha de motivo de fallback no log | GPU perdida mantém áudio |
| `TASK-E17-10-AC-2` | aparelho Android arm64 fisico com Android 14, app em foreground a 60 fps com visualizador em tela cheia e servico em primeiro plano, com cada chamada de render emitindo uma linha na tag VYNAMP_RENDER do logcat e a MediaSession consultavel por dumpsys media_session | com o playback em curso, adb shell input keyevent 26 apaga a tela e adb shell am start leva outra activity ao foreground por 10 min, contando as linhas VYNAMP_RENDER do intervalo e relendo o estado da MediaSession ao final | zero rendering Android screen-off/background |
| `TASK-E17-10-AC-3` | janela de 1280x720 com visualizador a 30 fps, reduced motion ligado na preferencia do sistema, tema de alto contraste ativo e o rotulo do meter presente na arvore de acessibilidade com valor e unidade, com telemetria de frame time coletada por 60 s em cada configuracao | entra-se em tela cheia pelo atalho do player e sai-se com uma tecla Escape, medindo de novo frame time, fps efetivo e custo de CPU e GPU por frame nos perfis de 30, 60 e 10 fps | reduced motion, foco e FPS/budgets medidos. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cd apps/player e cargo test --test visualizer_device_loss com 20 ciclos de destruicao forcada do device, mais o soak de 8 h com renderer artificialmente lento em script a criar, mais ./gradlew :app:connectedAndroidTest --tests VisualizerBackgroundEnergyTest no aparelho Android, mais cargo test --test visualizer_budget para reduced motion, fullscreen e os tres perfis de fps | Esperado: nos 20 ciclos de device loss o audio segue sem pausa, com source_underruns igual a 0 e uma linha de motivo de fallback para meter CPU por perda; no soak de 8 h, source_underruns 0 e dropped_frames de visual maior que 0; no Android, zero linhas VYNAMP_RENDER nos 10 min de tela apagada contra cerca de 36.000 linhas nos 10 min equivalentes a 60 fps em foreground; no perfil, 30 e 60 fps medidos em foreground, 10 em economia, frame time com reduced motion igual ao de meter estatico e no maximo 3 flashes por segundo. Nada disso existe hoje: apps/player tem 1 arquivo .rs (src/main.rs com um println!), zero dependencias e nenhum tests/, entao nenhum desses alvos compila; cargo test --features wgpu falha com "the package `vynamp_player` does not contain this feature: wgpu". Tambem nao existem o projeto Gradle, porque nem build.gradle nem gradlew aparecem em nenhum ponto do umbrella, nem o diretorio scripts/ para o soak, e nao ha nenhuma medicao de device loss registrada. |
| Hardware | ausente para o AC-2: exige aparelho Android arm64 fisico com Android 14, e este ambiente e Windows 11 x86_64 sem aparelho Android, sem adb no PATH, sem aparelho iOS e sem maquina macOS; o AC-1 e o AC-3 rodam em janela local sobre a Intel UHD Graphics 620 integrada, sem placa pro dedicada, e o AC-1 usa WARP por determinismo, entao a prova de device loss em driver de placa real fica como limitacao registrada. | ausência é limitação registrada, não aprovação |

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
