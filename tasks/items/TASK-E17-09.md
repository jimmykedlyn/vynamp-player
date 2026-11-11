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
| Critérios da spec | `specs/temas.md` §5 (rascunho, não aprovado) |
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

- **Não** entrega o catalogo M2 de espectrograma, particulas, fullscreen e fallback para meter CPU. Esse pacote e TASK-E06-06, dependencia declarada daqui, e a confusao e plausivel porque a entrega desta TASK fala em familias spectrogram e particles; reimplementar o catalogo criaria dois donos do mesmo renderer, e o de E06-06 e o que ja consome o preset validado.
- **Não** implementa a analise que alimenta as familias: o canal de observacao, a janela Hann, a FFT de 2048 e 4096 pontos e o VU/RMS. Isso e TASK-E06-05, dependencia desta pela cadeia de E06-06; a entrega escreve "ring/taps e familias FFT/phase/spectrogram/particles", o que sugere que o calculo do espectro nasce aqui, mas esta TASK escolhe tap e familia e consome o que E06-05 publica.
- **Não** implementa o sandbox Wasmtime/WIT, os limites de fuel, a interruption e o ADR-004 do plugin visualizer. Isso e TASK-E07-01, tambem dependencia declarada, e a proximidade e plausivel porque a spec fala de plugin proprio com bytes, fuel e deadline; aqui o preset e declarativo e nao carrega modulo executavel.
- **Não** mede energia, zero rendering em Android com tela apagada nem o comportamento em background e fullscreen. Essa certificacao e TASK-E17-10, que esta TASK bloqueia; o ring e os contadores de drop autorados aqui sao insumo dela, nao a medicao de consumo.
- **Não** executa o corpus de outputs por alvo e ABI nem o clock, seek e drain dos backends. Isso e TASK-E14-04; verificar tap e generation exige playback funcionando, o que torna tentador subir um backend real para o teste, mas executar o corpus de plataforma e trabalho de outra TASK e inverteria a ordem das dependencias.
- **Não** define a tela de escolha de cena, o texto de fallback nem a navegacao acessivel do shell phone. A UI e TASK-E13-05; aqui existe apenas a API que carrega preset e reporta motivo de rejeicao, e o rotulo e o foco ficam com quem desenha a tela.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e17-09-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E17-09-AC-1` | sete arquivos em tests/fixtures/presets/ do repo apps/player, validados contra D:/Projects/vynamp/schemas/visualizer-preset.schema.json: tres validos derivados do exemplo D:/Projects/vynamp/schemas/examples/visualizer-preset.json e distintos entre si, a saber family spectrum com tap final e fft_size 2048, family phase com tap post-eq e fft_size 4096 e family particles com tap source e fft_size 2048, todos com window hann, channel mix e fps 30, e quatro malformados, a saber family "shadertoy" fora do enum de seis familias, chave "fragmentSource" com GLSL literal no topo do objeto contra additionalProperties false, "version" como o inteiro 1 em vez da string semver "1.0.0" e "fft_size" 8192 fora do enum [2048, 4096] | o loader de scene preset recebe cada um dos sete caminhos, um por vez, e serializa em JSON o resultado por arquivo com o identificador do preset aceito ou o nome do campo e da linha rejeitados | Preset inválido/código arbitrário rejeitado |
| `TASK-E17-09-AC-2` | tres instancias de visualizador sobre o mesmo observer ring de 2.097.152 bytes e janela de 250 ms, com orcamento agregado de worker em 64 MiB, alimentadas por PCM sintetico estereo f32 de 48 kHz publicado a 192.000 quadros por segundo, quatro vezes a taxa de reproducao, enquanto os tres consumers renderizam a 30 fps | o produtor publica sem pausa por 60 s, o que soma 11.520.000 quadros e 92.160.000 bytes, ou 240 s de audio, 960 vezes a janela de 250 ms que governa o ring por ser menor que 2 MiB, e cada consumer drena mais devagar que a publicacao, lendo-se ao final dropped_frames, budget_rejected e bytes_residentes por instancia | orçamento agregado e overflow descartam visuais |
| `TASK-E17-09-AC-3` | callback de audio simulada a 48 kHz com periodo de 10 ms publicando o clock por seqlock, e a thread de rendering travada por um sleep de 200 ms injetado dentro do consumer do observer ring | a callback executa 600 periodos consecutivos, 6 s de audio, com o consumer parado, gravando por periodo a latencia medida, a contagem de underrun e o clock publicado | nunca bloqueiam callback/clock. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cd apps/player e cargo test --test visualizer_preset -- --nocapture sobre tests/fixtures/presets/ com os 3 validos e os 4 malformados, mais cargo test --test visualizer_ring com o harness que publica a 192.000 quadros por segundo por 60 s, mais cargo test --test audio_callback_stall com o consumer travado 200 ms | Esperado: 4 de 4 malformados rejeitados com o nome do campo na mensagem (family, fragmentSource, version, fft_size) e 3 de 3 validos aceitos com hash canonico estavel; no ring, dropped_frames maior que 0 nas tres instancias e budget_rejected maior que 0 na instancia cujo conjunto de assets de fixture passa de 64 MiB, que e a fixture que o teste precisa montar porque o overflow do ring sozinho nao exercita o orcamento agregado, e bytes_residentes nunca acima de 96.000, que sao 250 ms a 48 kHz estereo f32 em 12.000 quadros e 8 bytes por quadro, o menor dos dois limites contra os 2.097.152 bytes de 2 MiB que dariam 262.144 quadros, e zero recusa de bloco de audio; na callback, 600 de 600 periodos executados, zero underrun e clock monotonico com o consumer parado 200 ms. Nada disso existe hoje: apps/player tem 1 arquivo .rs, src/main.rs, que so executa println!("vynamp_player 0.1.0"), com zero dependencias, nenhum diretorio tests/ e nenhum lib target, entao cd apps/player mais cargo test --test visualizer_preset falha com "error: no test target named `visualizer_preset` in default-run packages". Medido tambem que um pacote so-binario nao pode ser importado por teste de integracao (erro E0432), entao o alvo lib precisa nascer junto com o teste, e o pacote se chama vynamp_player com underscore, porque cargo test -p vynamp-player responde "package ID specification `vynamp-player` did not match any packages". As fixtures tambem nao existem: tests/fixtures/presets/ nao esta no repo. |
| Hardware | nenhum: os tres criterios rodam em processo e nenhum abre dispositivo de audio, GPU ou aparelho, porque o AC-1 e validacao de schema JSON, o AC-2 e ring de bytes com PCM sintetico e o AC-3 e callback simulada; a maquina tem Intel UHD Graphics 620 integrada, mas nenhum dos tres cria adapter wgpu. | ausência é limitação registrada, não aprovação |

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
