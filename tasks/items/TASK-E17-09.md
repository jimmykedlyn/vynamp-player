# TASK-E17-09 — Entregar presets e taps limitados de visualização

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E17-09` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E17-09` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E06-06`, `TASK-E07-01` |
| Bloqueia | `TASK-E17-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Scenes declarativas, ring/taps e famílias FFT/phase/spectrogram/particles.

## Boundary

- **Não** entrega o catalogo M2 de espectrograma, particulas, fullscreen e fallback para meter CPU. Esse pacote e TASK-E06-06 e e dependencia declarada daqui; quem executar pode achar que "familias FFT/phase/spectrogram/particles" autoriza reimplementar o catalogo visual existente, e passariam a existir dois donos do mesmo renderer.
- **Não** implementa o sandbox Wasmtime/WIT, os limites de fuel, interruption e o ADR-004 do plugin visualizer. Isso e TASK-E07-01, tambem dependencia declarada; a proximidade e plausivel porque a spec fala de plugins proprios com bytes/fuel/deadline, mas aqui o preset e declarativo e nao carrega modulo executavel.
- **Não** mede energia, zero rendering em Android screen-off e nem o comportamento em background. Essa certificacao e TASK-E17-10, que esta TASK bloqueia; o ring e os contadores de drop daqui sao insumo dela, nao a medicao de consumo.
- **Não** executa o corpus de outputs por alvo e ABI nem o clock/seek/drain dos backends. Isso e TASK-E14-04; verificar tap e generation exige playback funcionando, mas executar o corpus de plataforma e trabalho de outra TASK e confundir os dois inverteria a ordem das dependencias.
- **Não** define a tela de escolha de cena, o texto de fallback e a navegacao acessivel do shell phone. A UI e TASK-E13-05; aqui existe apenas a API de carregar preset e reportar motivo de rejeicao.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e17-09-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E17-09-AC-1` | sete arquivos em tests/fixtures/presets/: tres validos conformes a schemas/visualizer-preset.schema.json e quatro malformados, a saber familia desconhecida "shadertoy", campo "fragmentSource" com GLSL literal, "version" inteiro que nao casa com o schema e "parameters.fftSize" igual a 8192 fora do bound declarado | o loader de scene preset e chamado sobre cada um dos sete arquivos, um por vez, e o resultado e serializado em JSON por arquivo | Preset inválido/código arbitrário rejeitado |
| `TASK-E17-09-AC-2` | tres instancias de visualizador simultaneas sobre o mesmo observer ring bounded de 2 MiB e 250 ms com orcamento agregado de worker em 64 MiB, recebendo PCM sintetico de 48 kHz estereo empurrado a 4x a taxa real (384 mil quadros por segundo) enquanto os tres renderers consomem a 30 fps | o produtor de PCM roda por 60 s sem pausa e o consumidor de cada renderer e deliberadamente mais lento que o ring, coletando os contadores dropped_frames, budget_rejected e bytes_residentes ao final | orçamento agregado e overflow descartam visuais |
| `TASK-E17-09-AC-3` | callback de audio simulado em 48 kHz com periodo de 10 ms que publica o clock, e a thread de rendering congelada artificialmente com sleep de 200 ms dentro do consumer do observer ring durante a janela do teste | a callback executa 600 periodos consecutivos (6 s de audio) com o consumer congelado, registrando latencia por periodo, contagem de underrun e o valor publicado do clock | nunca bloqueiam callback/clock. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test -p vynamp-player --test visualizer_preset -- --nocapture mais as fixtures tests/fixtures/presets/ (tres JSON validos e quatro malformados) e o harness de ring em tests/visualizer_ring.rs alimentado a 4x taxa real, e o harness de callback em tests/audio_callback_stall.rs com o consumer congelado por 200 ms. | Esperado: 4 de 4 presets malformados rejeitados com erro que nomeia o campo e a linha, 3 de 3 validos aceitos com hash estavel do preset canonico; no ring, dropped_frames maior que zero e bytes_residentes nunca acima de 2097152 e nenhum bloco de audio recusado; na callback, 600 de 600 periodos executados, zero underrun e clock monotonico mesmo com o consumer parado 200 ms. Hoje nada disso existe: o repo vynamp-player tem um unico arquivo .rs, src/main.rs, com so println!("vynamp_player 0.1.0"), zero dependencias e nenhum diretorio tests/, entao o comando acima falha por ausencia de alvo de teste e nao por regressao. |
| Hardware | nenhum: os tres criterios sao exercitados por schema, ring de bytes e callback de audio simuladas em processo, alimentados por PCM sintetico; nenhum teste abre dispositivo de audio, GPU ou aparelho. | ausência é limitação registrada, não aprovação |

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
