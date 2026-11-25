# TASK-E02-13 — Gerar gabarito de movimento, medidores e ondas

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-13` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-13` |
| Critérios da spec | `specs/ui.md` §5 (aprovada em 2026-11-18) |
| Fase | M0 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E02-11` |
| Bloqueia | `TASK-E02-15` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–6 dias úteis |
| Risco | alto |

## Entrega

Transições CSS, séries rAF sob sinais determinísticos, constantes de AnalyserNode e balística e estilos de onda em formato consumível pelo Rust.

## Boundary

- **Não** inventa curva de medidor. As constantes vêm de `AnalyserNode` e da
  balística medidas no MVP, como fixa `design/07-pin-mvp.md`, e o formato é o que
  o Rust consome.
- **Não** implementa o visualizador: `TASK-E17-09` e `TASK-E17-10`.
- **Não** congela o instante da captura: atributo animado por script é gravado em
  repouso e listado em `neutralized` pela extração de `TASK-E02-11`, para o arquivo
  não depender do momento.
- **Não** reextrai as curvas de DSP: `out/svg/meters/dynamic.json` registra quais
  elementos animam, e as curvas pertencem a esta TASK, não ao pacote de
  `TASK-E02-11`, dona da extração.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e02-13-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E02-13-AC-1` | a folha de estilo do MVP e um sinal determinístico injetado em requestAnimationFrame | cada transição observável é medida quanto a duração, easing e keyframes ou série temporal | Cada animação do MVP tem duração, easing e keyframes ou série temporal |
| `TASK-E02-13-AC-2` | o medidor `06-hifi-spectrum` (espectro e nível) e a onda do medidor `12-osciloscopio-crt` do MVP, que é o "visualizador de onda" do UI-AC-13, no estado fixado em "Protocolo de medição", recebendo degrau, seno e rajada sintéticos com os parâmetros dessa seção; o modo demo fica fora deste critério, e os estilos de onda de `waveform-styles.js` são cobertos pelo AC-3, sem série temporal (a série deles é do visualizador, de `TASK-E17-09` e `TASK-E17-10`) | cada sinal é medido pelo protocolo; os parâmetros do AnalyserNode, a balística e a resposta de cada série são registrados; a tolerância de cada série é o maior desvio absoluto contra o oráculo Rust, documentada em `evidence/PR/task-e02-13/`; o `cargo test` afirma que cada desvio fica no teto (0,1 dB no nível, 0,5 dB no espectro, 1e-4 na onda) e, em asserção separada, que a tolerância documentada é igual à recalculada; acima de um teto, o critério falha | medidores e ondas respondem a degrau, seno e rajada com tolerância documentada |
| `TASK-E02-13-AC-3` | cada uso de Web Audio do MVP identificado na varredura, incluindo os estilos de onda de `waveform-styles.js` listados em "Estilos de onda" | o uso é traduzido para um comportamento descrito em formato consumível pelo Rust; cada estilo de onda vira uma entrada com os parâmetros literais daquela seção, sem série temporal, e a lista esperada de estilos é derivada do código do MVP, não do próprio JSON | Web Audio entra só como comportamento reimplementável. |

## Protocolo de medição

Vale para o AC-2. Cada item abaixo é parte do critério, não sugestão.

1. **Estado do MVP.** Faixa não demo; `state.playing = true`; volume 1,0; sem mudo; `state.meterOff = false`; `state.fps = 30`; sem `reducedMotion` e com `prefers-reduced-motion: no-preference` emulado; aba visível; EQ desligado; restore e grafo inativos; nenhum visualizador de `waveform-styles.js` ativo. `state.meter` é `06-hifi-spectrum` ou `12-osciloscopio-crt`, conforme a execução.
2. **Grafo de áudio.** `window.AudioContext` é trocado por um stub que devolve um `OfflineAudioContext` de 48 kHz e 96000 amostras (2,0 s exatos, sem cauda), para o próprio `setupAudio` do MVP construir o grafo sobre ele. Como o contexto offline não tem `createMediaElementSource`, o stub o fornece, devolvendo uma fonte com o sinal sintético em estéreo idêntico no lugar da mídia. Os `AnalyserNode` e o resto do grafo são os do `setupAudio`, não uma reimplementação, e as leituras vêm do próprio `frame` do MVP.
3. **Sinais.** Portadora de 1 kHz, fase 0, bordas em corte seco, amplitude 0,5:
   - degrau: silêncio até 0,5 s, portadora de 0,5 s a 2,0 s;
   - seno: portadora de 0 a 2,0 s;
   - rajada: ligada em t = 0, 50 ms ligada e 450 ms desligada, por 2,0 s (4 rajadas).

   O RMS de −9,031 ± 0,01 dB do degrau, entre 0,5 s + 2048/48000 s e 2,0 s, é referência, não critério.
4. **Execuções.** Uma execução por medidor e por sinal (6 no total), cada uma num contexto novo; nenhuma outra leitura de analisador acontece no mesmo contexto, para que o `smoothingTimeConstant` avance só nas leituras do protocolo.
5. **Grade e cadência.** Quadros `k` de 0 a 119, com `t_k = k*1000/60` ms avaliado em float64 nessa ordem de operações. `lastMeter` começa em 0. Há leitura no quadro em que `t_k - lastMeter >= 1000/30`, a regra literal do `frame` do MVP. Isso dá 42 leituras, com prefixo 2, 4, 7, 10, 12 e última no quadro 118; o teste confere a contagem e o prefixo.
6. **Instante de leitura.** O contexto é suspenso na amostra `ceil(t_k*48/128)*128` (arredondada para cima ao quantum de 128 amostras; um instante já na fronteira fica nela). A amostra real é gravada, e o oráculo calcula sobre ela; se a amostra real diferir da fórmula, o critério falha. Antes do início do sinal, o pré-roll é zero.
7. **Séries.** Por canal e por leitura:
   - nível, gravado nas duas execuções de medidor (06 e 12): RMS das 2048 amostras de `getFloatTimeDomainData` → `20*log10(max(1e-8, rms))` recortado em [−60, 0] dB; o desvio é medido em dB; teto 0,1 dB;
   - espectro (`06-hifi-spectrum`): os bins que o medidor lê a partir de `frequenciesHz`, gravados depois do recorte em [−60, 0] dB (sem `-Infinity` nem `null`); teto 0,5 dB;
   - onda (`12-osciloscopio-crt`): `sampleBuffers[ch][i*4]` para `i` de 0 a 180, em amplitude; teto 1e-4.

   Cada série registra o analisador que a alimenta (`fftSize` e `smoothingTimeConstant`).
8. **Oráculo.** Código de teste em `tests/` do `vynamp-player`, nunca de produção. Regenera o sinal analiticamente a partir do índice de amostra `n`, como `f32(0,5·sin(2π·1000·n/48000))` multiplicado pelo envelope do item 3, sem ler o áudio gravado; reimplementa RMS → dB e o `AnalyserNode` (janela, FFT, suavização, conversão em dB) e calcula sobre as amostras gravadas no item 6.
9. **Reprodutibilidade.** `movimento.json` grava a versão do Chrome, do sistema operacional e do Node; os floats são gravados por `JSON.stringify` em float64. Duas execuções do gerador com os mesmos três valores produzem sha256 idêntico. Essa garantia vale só para a conferência local, na máquina do mantenedor, cuja evidência vai para o PR; o CI não a repete: o `cargo test` do CI compara o `movimento.json` commitado contra o oráculo.

## Estilos de onda

Vale para o AC-3. Fonte: `reference/vynamp-mvp/mvp/waveform-styles.js` no pin de `design/07-pin-mvp.md`.

1. **Estilos.** Cada chave de `defaultConfigs` é um estilo, com os parâmetros dela copiados literais (por exemplo, `spectrum-pulse` com `bars` 64, `height` 42 e `peakDecay` 0,02; `spectrum-aurora` com `bands` 128, `height` 21 e `dbRange` 84).
2. **Bandas de espectro** (`spectrumBands`, estilos `spectrum-aurora`, `spectrum-ribbon` e `spectrum-stereo`): analisador lido por `copyLiveSpectrum` (`smoothingTimeConstant` 0,75); amostras lidas por `copyLiveSamples`, com gate de áudio `|v| > 1e-6`; 128 bandas logarítmicas de 40 Hz a `min(20000, nyquist)`; bins a partir de `j = 1`; nível `clamp((db + range) / range, 0, 1)` com `range = dbRange || 84`; frequência de pico gravada só quando o máximo passa de −90 dB.
3. **Pulso de espectro** (`spectrum-pulse`): analisador lido por `copyPulseSpectrum` (`smoothingTimeConstant` 0), os dois canais fundidos por `VynampSpectrumPulse.merge` e atualizados por `VynampSpectrumPulse` com `bars` barras.
4. **Cadência** (`pulseFrame`): redesenho quando `t - lastPulse` atinge 100 ms com movimento reduzido, 0 ms em `spectrum-pulse` e 33 ms nos demais.
5. **Fora daqui.** A resposta desses estilos a degrau, seno e rajada, como série temporal, é de `TASK-E17-09` e `TASK-E17-10`.

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | o gerador de séries `rAF` sob sinais determinísticos, comparado contra as transições CSS e as constantes de `AnalyserNode` do MVP; para o AC-2, o "Protocolo de medição" | as séries são reproduzíveis entre duas execuções e batem com o MVP dentro da tolerância declarada; para o AC-2, duas execuções do gerador produzem `movimento.json` com sha256 idêntico (conferência local, item 9 do protocolo), e o `cargo test` mostra cada desvio dentro do teto e a tolerância documentada igual à recalculada; hoje não há gerador nem comparação no repo |
| Hardware | nenhum: as séries são numéricas e não dependem de GPU real | ausência é limitação registrada, não aprovação |

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
