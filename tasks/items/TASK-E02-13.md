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
| Bloqueia | `TASK-E02-15`, `TASK-E06-11` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–6 dias úteis |
| Risco | alto |

## Entrega

Transições CSS, séries rAF sob sinais determinísticos, constantes de AnalyserNode e balística e estilos de onda em formato consumível pelo Rust.

## Boundary

- **Não** inventa curva de medidor. As constantes vêm de `AnalyserNode` e da
  balística medidas no MVP, como fixa `design/07-pin-mvp.md` (commit `0d9860b901510021bcca4c31a25014373ea7c452`), e o formato é o que
  o Rust consome.
- **Não** desenha os estilos de onda no player nem implementa o visualizador: os 17 estilos
  são `TASK-E06-11`, que consome as séries daqui, e o visualizador é `TASK-E17-09` e
  `TASK-E17-10`. Aqui se mede e se descreve o MVP; desenhar aqui daria dois donos para o
  mesmo estilo.
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

1. **Estado do MVP.** A faixa ativa é um arquivo local registrado pelo fluxo de arquivo local do próprio MVP (`registerLocalFile`, que grava `demo: false`); o arquivo tem bytes que não são áudio, para o `decodeAudioData` de `localWave` rejeitar e `#wave` ficar com `data-source` `unavailable`, sem `setWave` do arquivo; o som vem da fonte trocada no item 2. `state.playing = true`; volume 1,0, posto pelo controle de volume do próprio MVP (`#volume` em 100, evento `input`) antes de `playTrack`, porque `setupAudio` lê `state.volume` uma vez, ao criar o ganho, e o padrão é 0,5; sem mudo; `state.fps = 30`; sem `reducedMotion` e com `prefers-reduced-motion: no-preference` emulado; aba visível; EQ desligado; restore e grafo inativos; `localStorage` limpo, de modo que cada estilo usa os valores de `defaultConfigs`. Nas execuções de medidor, `state.meterOff = false`, `state.meter` é `06-hifi-spectrum` ou `12-osciloscopio-crt` e nenhum estilo de onda ao vivo está selecionado. Nas execuções de estilo, `state.meterOff = true` e o estilo é selecionado pelo seletor do próprio MVP. Caminho para tocar, nesta ordem: o stub do item 2 instalado antes de qualquer chamada abaixo; `registerLocalFile(file)`; `VynampReference.playTrack(id, false)`, que é `selectTrack` e, pela faixa local, chama `localWave`, que roda o primeiro `setupAudio` (o grafo nasce aqui, sobre o stub) e o `decodeAudioData` nativo do contexto offline, que rejeita o arquivo; o gerador espera `#wave` com `data-source` `unavailable` antes de seguir; `VynampReference.getAudioGraphHost()`, que não faz nada além de devolver o grafo, porque `ctx` já existe; nas execuções de estilo, a seleção do estilo pelo seletor; `VynampReference.setProceduralTransport(true, 'playing')`, que liga `state.playing`, chama `startLoop` e dispara `vynamp:playback`. Nas execuções de estilo, o `pulseFrame` já foi agendado pelo `render()` da seleção, e `vynamp:playback` não o reagenda enquanto ele está pendente. `togglePlay` não é usado.
2. **Grafo de áudio.** `window.AudioContext` é trocado por um stub que devolve um `OfflineAudioContext` de 48 kHz e 96000 amostras (2,0 s exatos, sem cauda), para o próprio `setupAudio` do MVP construir o grafo sobre ele; o stub é instalado antes de `registerLocalFile`, porque o primeiro `setupAudio` roda dentro de `playTrack` (item 1). Como o contexto offline não tem `createMediaElementSource`, o stub o fornece, devolvendo uma fonte com o sinal sintético em estéreo idêntico no lugar da mídia. Os `AnalyserNode` e o resto do grafo são os do `setupAudio`, não uma reimplementação, e as leituras vêm do próprio `frame` do MVP. O gerador substitui `requestAnimationFrame` e `performance.now`: em cada quadro `k`, depois que a suspensão do item 6 resolve, faz `performance.now` devolver `t_k`, chama os callbacks pendentes de `requestAnimationFrame` com `t_k` e só então retoma o contexto.
3. **Sinais.** Portadora de 1 kHz, fase 0, bordas em corte seco, amplitude 0,5:
   - degrau: silêncio até 0,5 s, portadora de 0,5 s a 2,0 s;
   - seno: portadora de 0 a 2,0 s;
   - rajada: ligada em t = 0, 50 ms ligada e 450 ms desligada, por 2,0 s (4 rajadas).

   O RMS de −9,031 ± 0,02 dB do degrau, entre 0,5 s + 2048/48000 s e 2,0 s, é referência, não critério.
4. **Execuções.** Uma execução por sinal e por medidor (6) e uma por sinal e por estilo de onda ao vivo (42), 48 no total, cada uma num contexto novo; nenhuma outra leitura de analisador acontece no mesmo contexto, para que o `smoothingTimeConstant` avance só nas leituras do protocolo.
5. **Grade e cadência.** Quadros `k` de 0 a 119, com `t_k = k*1000/60` ms avaliado em float64 nessa ordem de operações. `lastMeter` começa em 0. Há leitura no quadro em que `t_k - lastMeter >= 1000/30`, a regra literal do `frame` do MVP. Isso dá 42 leituras, com prefixo 2, 4, 7, 10, 12 e última no quadro 118; o teste confere a contagem e o prefixo. Nas execuções de estilo vale a cadência de `pulseFrame` (item 5 de "Estilos de onda"), com `lastPulse` começando em 0: nos estilos de 33 ms, 59 leituras, nos quadros pares de 2 a 118; em `spectrum-pulse`, uma leitura por quadro, 120 (quadros 0 a 119). O teste confere as contagens de 59 e 120.
6. **Instante de leitura.** O contexto é suspenso na amostra `ceil(t_k*48/128)*128` (arredondada para cima ao quantum de 128 amostras; um instante já na fronteira fica nela). A amostra real é gravada, e o oráculo calcula sobre ela; se a amostra real diferir da fórmula, o critério falha. Antes do início do sinal, o pré-roll é zero.
7. **Séries.** Por canal e por leitura:
   - nível, gravado nas duas execuções de medidor (06 e 12): RMS das 2048 amostras de `getFloatTimeDomainData` → `20*log10(max(1e-8, rms))` recortado em [−60, 0] dB; o desvio é medido em dB; teto 0,1 dB;
   - espectro (`06-hifi-spectrum`): os bins que o medidor lê a partir de `frequenciesHz`, gravados depois do recorte em [−60, 0] dB (sem `-Infinity` nem `null`); teto 0,5 dB;
   - onda (`12-osciloscopio-crt`): `sampleBuffers[ch][i*4]` para `i` de 0 a 180, em amplitude; teto 1e-4;
   - estilos de onda: as séries e os tetos do item 6 de "Estilos de onda".

   Cada série registra o analisador que a alimenta (`fftSize` e `smoothingTimeConstant`). Balística: nenhuma; `paintMeter` mapeia o dB direto, e a única suavização é o `smoothingTimeConstant` do analisador. O JSON grava a balística como nula, com esse motivo.
8. **Oráculo.** Código de teste em `tests/` do `vynamp-player`, nunca de produção. Regenera o sinal analiticamente a partir do índice de amostra `n`, como `f32(0,5·sin(2π·1000·n/48000))` multiplicado pelo envelope do item 3, sem ler o áudio gravado; reimplementa RMS → dB, o `AnalyserNode` (janela, FFT, suavização, conversão em dB) e a aritmética de cada estilo de onda, e calcula sobre os índices de amostra gravados no item 6.
9. **Reprodutibilidade.** `movimento.json` grava a versão do Chrome, do sistema operacional e do Node; os floats são gravados por `JSON.stringify` em float64. Duas execuções do gerador com as mesmas versões de Chrome, sistema operacional e Node produzem sha256 idêntico. Essa garantia vale só para a conferência local, na máquina do mantenedor, cuja evidência vai para o PR; o CI não a repete: o `cargo test` do CI compara o `movimento.json` commitado contra o oráculo.
10. **Caminhos.** No `vynamp-player`: o gerador em `tools/gabarito-movimento/`; o gabarito em `assets/gabarito/movimento.json`; a tolerância em `evidence/PR/task-e02-13/tolerancia.json`, um objeto com uma chave por série (`<execução>/<série>/<canal>`) e o maior desvio absoluto como valor; o oráculo em `tests/`. O MVP é lido de um clone fora do repo, cujo caminho é passado ao gerador por argumento; o gerador lê `git rev-parse HEAD` desse clone, recusa rodar se ele não for `0d9860b901510021bcca4c31a25014373ea7c452` e grava esse SHA em `movimento.json`; o teste afirma o SHA literal.

## Estilos de onda

Vale para o AC-2 e o AC-3. Fonte: `waveform-styles.js` e `spectrum-pulse.js` do MVP, no commit `0d9860b901510021bcca4c31a25014373ea7c452`.

1. **Estilos.** As 17 chaves de `defaultConfigs`, com os parâmetros copiados literais (por exemplo, `spectrum-pulse` com `bars` 64, `height` 42 e `peakDecay` 0,02; `spectrum-aurora` com `bands` 128, `height` 21 e `dbRange` 84). São 14 ao vivo, os de `liveModes`: `pulse`, `bars-pulse`, `columns-pulse`, `ribbon-pulse`, `prisma-neon`, `aurora-neon`, `spectrum-aurora`, `spectrum-ribbon`, `spectrum-stereo`, `spectrum-pulse`, `aurora-weave`, `neon-crystal`, `aurora-constellation` e `aurora-layers`. Os outros 3 (`original`, `dj` e `overview`) desenham o envelope do arquivo e não respondem ao áudio ao vivo: entram no AC-3 com esse motivo e sem série, junto com o que o código fixa fora de `defaultConfigs`: `envelope(values, height)` com `peakHeight` 22 e `bodyHeight` 20 por padrão (15 e 7 em `overview`), abscissas `i*1000/(n-1)` e ordenadas `24 ± valor·altura`, ambas em `toFixed(2)`.
2. **Bandas de espectro** (`spectrumBands`, usada por `spectrum-aurora`, `spectrum-ribbon`, `spectrum-stereo` e, via `drawCreative`, `aurora-constellation`): analisador lido por `copyLiveSpectrum` (`smoothingTimeConstant` 0,75); amostras lidas por `copyLiveSamples`, com gate de áudio `|v| > 1e-6`; 128 bandas logarítmicas de 40 Hz a `min(20000, nyquist)`; bins a partir de `j = 1`; nível `clamp((db + range) / range, 0, 1)` com `range = dbRange || 84`; frequência de pico gravada só quando o máximo passa de −90 dB.
3. **Pulso de espectro** (`spectrum-pulse`): analisador lido por `copyPulseSpectrum` (`smoothingTimeConstant` 0); canais fundidos por máximo (`VynampSpectrumPulse.merge`); `VynampSpectrumPulse.update` com `bars` barras logarítmicas de 20 Hz a `min(20000, nyquist)`, nível `clamp((db + 90) / 80, 0, 1)`, subida imediata, queda com `keep = exp(-dt/70)`, pico que cai `dt*0,0008` por atualização, e `dt = min(250, now - last)`, ou 16 quando `last` é 0; como `now` é 0 no quadro 0, `dt` é 16 nos quadros 0 e 1. `peakDecay` está em `defaultConfigs`, mas o código não o lê: é registrado como parâmetro sem efeito.
4. **Leitura temporal** (`pulse`, `bars-pulse`, `columns-pulse`, `ribbon-pulse`, `prisma-neon`, `aurora-neon`, `aurora-weave`, `neon-crystal` e `aurora-layers`): 2048 amostras por `copyLiveSamples`, com a aritmética de cada estilo (passo de amostragem, ganho, `smoothing` de `bars-pulse`, pico e RMS por segmento). Em `bars-pulse`, `pulseLevel` começa em 1 (`scaleBars(1)`). O ramo de rádio de `bars-pulse` (barras por amostra, só com faixa de rádio) não tem série, porque o protocolo usa faixa local: entra no AC-3 como comportamento, com os parâmetros e esse motivo.
5. **Cadência** (`pulseFrame`): redesenho quando `t - lastPulse` atinge 100 ms com movimento reduzido, 0 ms em `spectrum-pulse` e 33 ms nos demais.
6. **Séries e tetos.** Por leitura de `pulseFrame`, o atributo numérico que o estilo publica e a geometria que ele escreve, comparados com o oráculo.
   - Atributos, com o elemento que os carrega: `data-level` de `spectrum-aurora`, `spectrum-ribbon`, `spectrum-stereo` e `spectrum-pulse` em `#wave-spectrum-live`; `data-level` de `aurora-weave`, `neon-crystal`, `aurora-layers` e `aurora-constellation` em `#wave-creative-live`; `data-peak` de `columns-pulse`, `ribbon-pulse`, `prisma-neon` e `aurora-neon` em `#wave-live-styles`; `data-level` de `bars-pulse` em `#wave-reactive-bars`; `data-frequency-left` em `#wave-spectrum-live`, nos três estilos de bandas de espectro (`spectrum-aurora`, `spectrum-ribbon` e `spectrum-stereo`) e em `aurora-constellation`; `spectrum-pulse` não o escreve; `data-frequency-right` só em `spectrum-stereo`, o único em que o MVP o escreve.
   - Tetos dos atributos: `data-level` dos estilos de bandas de espectro e de `aurora-constellation`, 0,5/84 + 5e-5 (0,5 dB na escala de `dbRange` 84, mais o arredondamento de `toFixed(4)`); `data-level` de `spectrum-pulse`, 0,5/80 + 5e-5; `data-level` de `aurora-weave`, `neon-crystal` e `aurora-layers` e `data-peak`, 1e-4 + 5e-5; `data-level` de `bars-pulse`, 1e-4 + 5e-4 (arredondamento de `toFixed(3)`); `data-frequency-left` e `data-frequency-right`: o bin do MVP é o bin de pico do oráculo ou um bin cujo nível, no oráculo, fica a até 0,5 dB do pico (o teto do espectro), porque em degrau e rajada há bins quase iguais e o erro admitido troca o máximo.
   - Geometria: cada `d` listado abaixo (e o `transform` de `bars-pulse`) tem a mesma sequência de comandos SVG que o oráculo gera. As abscissas, que só dependem do índice, ficam a até 0,005 do oráculo (só o `toFixed(2)` do MVP). Cada ordenada absoluta e cada comprimento relativo (`v`) fica no teto da tabela vezes o fator com que o código multiplica a altura, mais 0,005 do `toFixed(2)`: fator 2 no `v` de `columns-pulse` e `prisma-neon` (`v(bh*2)`), fator 1 em todo o resto, inclusive o `v` das barras de `spectrum-pulse` e dos estilos de bandas.
   - Comparação: o oráculo calcula cada valor sem arredondar; o lado do MVP é o número lido da string que ele escreveu (`toFixed` incluído). A diferença entre os dois é que fica no teto; o oráculo nunca arredonda antes de comparar, e é por isso que cada teto soma a meia unidade do `toFixed` do MVP.

   | Estilo | Elemento e atributo | Escala | Teto da ordenada |
   |---|---|---|---|
   | `pulse` | `#wave-pulse` `d` | amplitude 22 | 22·1e-4 + 0,005 |
   | `bars-pulse` | `#wave-reactive-bars` `transform` (scale, `toFixed(4)`) | nível | 1e-4 + 5e-5 no scale |
   | `columns-pulse`, `prisma-neon` | `#wave-live-columns` `d` | `height` 21 · `gain` 1,4 | 29,4·1e-4 + 0,005 |
   | `ribbon-pulse` | `#wave-live-fill` `d` (RMS por segmento, suavizado 1-2-1) | 21 · 2,0 | 42·1e-4 + 0,005 |
   | `aurora-neon` | `#wave-live-line` e `#wave-live-halo` `d` | 22 | 22·1e-4 + 0,005 |
   | `spectrum-aurora` | `#wave-spectrum-left` e `#wave-spectrum-halo` `d` | `height` 21, `dbRange` 84 | 21·0,5/84 + 0,005 |
   | `spectrum-ribbon` | `#wave-spectrum-fill` `d` | 21, 84 | 21·0,5/84 + 0,005 |
   | `spectrum-stereo` | `#wave-spectrum-left` e `#wave-spectrum-right` `d` | 21, 84 | 21·0,5/84 + 0,005 |
   | `spectrum-pulse` | `#wave-spectrum-fill` (barras) e `#wave-spectrum-halo` (picos) `d` | `height` 42, escala 80 | 42·0,5/80 + 0,005 |
   | `aurora-weave` | `#wave-creative-left` e `#wave-creative-right` `d` | 22 | 22·1e-4 + 0,005 |
   | `neon-crystal` (32 segmentos), `aurora-layers` (64 segmentos) | `#wave-creative-outer` (pico, altura 21, ganho 2), `#wave-creative-middle` (RMS, 16, 2) e `#wave-creative-inner` (RMS, `round(16/2)` = 8) `d` | 21·2 | 42·1e-4 + 0,005 |
   | `aurora-constellation` | `#wave-creative-nodes` e `#wave-creative-mesh` `d` | 21, 84 | 21·0,5/84 + 0,005 |

   Limiar de estrutura: `spectrum-pulse` só desenha barra com altura acima de 0,2 e pico acima de 0,02, e `aurora-constellation` só desenha nó com valor acima de 0,01. Se a sequência de comandos divergir porque um valor caiu perto de um limiar, o critério falha e o PR mostra a leitura; não há folga.
7. **Asserções contra o vazio.** O teste deriva do protocolo, não do JSON, a lista esperada: as 48 execuções e as chaves de cada uma; confere que cada série tem a contagem do item 5 do protocolo (42, 59 ou 120); que, na execução de degrau, cada série tem ao menos dois valores distintos; e que `tolerancia.json` tem exatamente as chaves esperadas.
8. **Fora daqui.** Esta TASK grava as séries e o comportamento; desenhar os 17 estilos no player é de `TASK-E06-11`.

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
