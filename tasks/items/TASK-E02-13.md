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
| `TASK-E02-13-AC-2` | o medidor `06-hifi-spectrum` (espectro e nível), a onda do medidor `12-osciloscopio-crt` e os 14 estilos de onda ao vivo de `waveform-styles.js`, que juntos são "o medidor e o visualizador de onda" do UI-AC-13, no estado fixado em "Protocolo de medição", recebendo degrau, seno e rajada sintéticos com os parâmetros dessa seção; o modo demo fica fora deste critério | cada sinal é medido pelo protocolo; os parâmetros do AnalyserNode, a balística e a resposta de cada série são registrados; a tolerância de cada série é o maior desvio absoluto contra o oráculo Rust, documentada em `evidence/PR/task-e02-13/tolerancia.json`; o `cargo test` afirma que cada desvio fica no teto (0,1 dB no nível, 0,5 dB no espectro, 1e-4 na onda e, nos estilos de onda, os tetos de "Estilos de onda") e, em asserção separada, que a tolerância documentada é igual à recalculada; acima de um teto, o critério falha | medidores e ondas respondem a degrau, seno e rajada com tolerância documentada |
| `TASK-E02-13-AC-3` | cada uso de Web Audio do MVP identificado na varredura, incluindo os 17 estilos de onda de `waveform-styles.js` | o uso é traduzido para um comportamento descrito em formato consumível pelo Rust; cada estilo de onda vira uma entrada com os parâmetros literais de "Estilos de onda", e a lista esperada de estilos é derivada do código do MVP, não do próprio JSON | Web Audio entra só como comportamento reimplementável. |

## Protocolo de medição

Vale para o AC-2, junto com "Estilos de onda". Cada item abaixo é parte do critério, não sugestão.

1. **Estado do MVP.** A faixa ativa é um arquivo local registrado pelo fluxo de arquivo local do próprio MVP (`registerLocalFile`, que grava `demo: false`); o conteúdo do arquivo não importa, porque a fonte é trocada no item 2. `state.playing = true`; volume 1,0; sem mudo; `state.fps = 30`; sem `reducedMotion` e com `prefers-reduced-motion: no-preference` emulado; aba visível; EQ desligado; restore e grafo inativos; `localStorage` limpo, de modo que cada estilo usa os valores de `defaultConfigs`. Nas execuções de medidor, `state.meterOff = false`, `state.meter` é `06-hifi-spectrum` ou `12-osciloscopio-crt` e nenhum estilo de onda ao vivo está selecionado. Nas execuções de estilo, `state.meterOff = true` e o estilo é selecionado pelo seletor do próprio MVP.
2. **Grafo de áudio.** `window.AudioContext` é trocado por um stub que devolve um `OfflineAudioContext` de 48 kHz e 96000 amostras (2,0 s exatos, sem cauda), para o próprio `setupAudio` do MVP construir o grafo sobre ele. Como o contexto offline não tem `createMediaElementSource`, o stub o fornece, devolvendo uma fonte com o sinal sintético em estéreo idêntico no lugar da mídia. Os `AnalyserNode` e o resto do grafo são os do `setupAudio`, não uma reimplementação, e as leituras vêm do próprio `frame` do MVP. O gerador substitui `requestAnimationFrame` e `performance.now`: em cada quadro `k`, depois que a suspensão do item 6 resolve, faz `performance.now` devolver `t_k`, chama os callbacks pendentes de `requestAnimationFrame` com `t_k` e só então retoma o contexto.
3. **Sinais.** Portadora de 1 kHz, fase 0, bordas em corte seco, amplitude 0,5:
   - degrau: silêncio até 0,5 s, portadora de 0,5 s a 2,0 s;
   - seno: portadora de 0 a 2,0 s;
   - rajada: ligada em t = 0, 50 ms ligada e 450 ms desligada, por 2,0 s (4 rajadas).

   O RMS de −9,031 ± 0,02 dB do degrau, entre 0,5 s + 2048/48000 s e 2,0 s, é referência, não critério.
4. **Execuções.** Uma execução por sinal e por medidor (6) e uma por sinal e por estilo de onda ao vivo (42), 48 no total, cada uma num contexto novo; nenhuma outra leitura de analisador acontece no mesmo contexto, para que o `smoothingTimeConstant` avance só nas leituras do protocolo.
5. **Grade e cadência.** Quadros `k` de 0 a 119, com `t_k = k*1000/60` ms avaliado em float64 nessa ordem de operações. `lastMeter` começa em 0. Há leitura no quadro em que `t_k - lastMeter >= 1000/30`, a regra literal do `frame` do MVP. Isso dá 42 leituras, com prefixo 2, 4, 7, 10, 12 e última no quadro 118; o teste confere a contagem e o prefixo. Nas execuções de estilo vale a cadência de `pulseFrame` (item 5 de "Estilos de onda"), com `lastPulse` começando em 0: nos estilos de 33 ms, 59 leituras, nos quadros pares de 2 a 118; em `spectrum-pulse`, uma leitura por quadro em que `pulseFrame` roda. O teste confere a contagem de 59 e, em `spectrum-pulse`, que não falta nenhum quadro entre a primeira e a última leitura.
6. **Instante de leitura.** O contexto é suspenso na amostra `ceil(t_k*48/128)*128` (arredondada para cima ao quantum de 128 amostras; um instante já na fronteira fica nela). A amostra real é gravada, e o oráculo calcula sobre ela; se a amostra real diferir da fórmula, o critério falha. Antes do início do sinal, o pré-roll é zero.
7. **Séries.** Por canal e por leitura:
   - nível, gravado nas duas execuções de medidor (06 e 12): RMS das 2048 amostras de `getFloatTimeDomainData` → `20*log10(max(1e-8, rms))` recortado em [−60, 0] dB; o desvio é medido em dB; teto 0,1 dB;
   - espectro (`06-hifi-spectrum`): os bins que o medidor lê a partir de `frequenciesHz`, gravados depois do recorte em [−60, 0] dB (sem `-Infinity` nem `null`); teto 0,5 dB;
   - onda (`12-osciloscopio-crt`): `sampleBuffers[ch][i*4]` para `i` de 0 a 180, em amplitude; teto 1e-4;
   - estilos de onda: as séries e os tetos do item 6 de "Estilos de onda".

   Cada série registra o analisador que a alimenta (`fftSize` e `smoothingTimeConstant`).
8. **Oráculo.** Código de teste em `tests/` do `vynamp-player`, nunca de produção. Regenera o sinal analiticamente a partir do índice de amostra `n`, como `f32(0,5·sin(2π·1000·n/48000))` multiplicado pelo envelope do item 3, sem ler o áudio gravado; reimplementa RMS → dB, o `AnalyserNode` (janela, FFT, suavização, conversão em dB) e a aritmética de cada estilo de onda, e calcula sobre os índices de amostra gravados no item 6.
9. **Reprodutibilidade.** `movimento.json` grava a versão do Chrome, do sistema operacional e do Node; os floats são gravados por `JSON.stringify` em float64. Duas execuções do gerador com as mesmas versões de Chrome, sistema operacional e Node produzem sha256 idêntico. Essa garantia vale só para a conferência local, na máquina do mantenedor, cuja evidência vai para o PR; o CI não a repete: o `cargo test` do CI compara o `movimento.json` commitado contra o oráculo.
10. **Caminhos.** No `vynamp-player`: o gerador em `tools/gabarito-movimento/`; o gabarito em `assets/gabarito/movimento.json`; a tolerância em `evidence/PR/task-e02-13/tolerancia.json`, um objeto com uma chave por série (`<execução>/<série>/<canal>`) e o maior desvio absoluto como valor; o oráculo em `tests/`. O MVP é lido de um clone no commit fixado em `design/07-pin-mvp.md`, fora do repo, cujo caminho é passado ao gerador por argumento; `movimento.json` grava esse SHA.

## Estilos de onda

Vale para o AC-2 e o AC-3. Fonte: `waveform-styles.js` e `spectrum-pulse.js` do MVP, no pin de `design/07-pin-mvp.md`.

1. **Estilos.** As 17 chaves de `defaultConfigs`, com os parâmetros copiados literais (por exemplo, `spectrum-pulse` com `bars` 64, `height` 42 e `peakDecay` 0,02; `spectrum-aurora` com `bands` 128, `height` 21 e `dbRange` 84). São 14 ao vivo, os de `liveModes`: `pulse`, `bars-pulse`, `columns-pulse`, `ribbon-pulse`, `prisma-neon`, `aurora-neon`, `spectrum-aurora`, `spectrum-ribbon`, `spectrum-stereo`, `spectrum-pulse`, `aurora-weave`, `neon-crystal`, `aurora-constellation` e `aurora-layers`. Os outros 3 (`original`, `dj` e `overview`) desenham o envelope do arquivo e não respondem ao áudio ao vivo: entram no AC-3 com esse motivo e sem série.
2. **Bandas de espectro** (`spectrumBands`, usada por `spectrum-aurora`, `spectrum-ribbon`, `spectrum-stereo` e, via `drawCreative`, `aurora-constellation`): analisador lido por `copyLiveSpectrum` (`smoothingTimeConstant` 0,75); amostras lidas por `copyLiveSamples`, com gate de áudio `|v| > 1e-6`; 128 bandas logarítmicas de 40 Hz a `min(20000, nyquist)`; bins a partir de `j = 1`; nível `clamp((db + range) / range, 0, 1)` com `range = dbRange || 84`; frequência de pico gravada só quando o máximo passa de −90 dB.
3. **Pulso de espectro** (`spectrum-pulse`): analisador lido por `copyPulseSpectrum` (`smoothingTimeConstant` 0); canais fundidos por máximo (`VynampSpectrumPulse.merge`); `VynampSpectrumPulse.update` com `bars` barras logarítmicas de 20 Hz a `min(20000, nyquist)`, nível `clamp((db + 90) / 80, 0, 1)`, subida imediata, queda com `keep = exp(-dt/70)`, pico que cai `dt*0,0008` por atualização, e `dt = min(250, now - last)`, ou 16 na primeira. `peakDecay` está em `defaultConfigs`, mas o código não o lê: é registrado como parâmetro sem efeito.
4. **Leitura temporal** (`pulse`, `bars-pulse`, `columns-pulse`, `ribbon-pulse`, `prisma-neon`, `aurora-neon`, `aurora-weave`, `neon-crystal` e `aurora-layers`): 2048 amostras por `copyLiveSamples`, com a aritmética de cada estilo (passo de amostragem, ganho, `smoothing` de `bars-pulse`, pico e RMS por segmento).
5. **Cadência** (`pulseFrame`): redesenho quando `t - lastPulse` atinge 100 ms com movimento reduzido, 0 ms em `spectrum-pulse` e 33 ms nos demais.
6. **Séries e tetos.** Por leitura de `pulseFrame`, o atributo numérico que o estilo publica, comparado com o oráculo:
   - `data-level` dos estilos de bandas de espectro: teto 0,5/84 + 5e-5 (0,5 dB na escala de `dbRange` 84, mais o arredondamento de `toFixed(4)`); `data-frequency-left` e `data-frequency-right`: mesmo bin de pico que o oráculo;
   - `data-level` de `spectrum-pulse`: teto 0,5/80 + 5e-5;
   - `data-level` de `aurora-weave`, `neon-crystal` e `aurora-layers` e `data-peak` de `columns-pulse`, `ribbon-pulse`, `prisma-neon` e `aurora-neon`: teto 1e-4 + 5e-5;
   - `data-level` de `bars-pulse` (em `#wave-reactive-bars`): teto 1e-4 + 5e-4 (arredondamento de `toFixed(3)`);
   - `pulse`, que não publica atributo numérico: as ordenadas do `d` de `#wave-pulse`; teto 22·1e-4 + 0,005 (amplitude 22 e arredondamento de `toFixed(2)`).
7. **Fora daqui.** Esta TASK grava as séries e o comportamento; desenhar os estilos no player é de `TASK-E17-09` e `TASK-E17-10`.

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
