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
| `TASK-E02-13-AC-2` | o medidor `06-hifi-spectrum` (espectro e nível) e o medidor `12-osciloscopio-crt` (onda) do MVP, no estado fixado em "Protocolo de medição", recebendo degrau, seno e rajada sintéticos com os parâmetros dessa seção; o modo demo e os estilos de `waveform-styles.js` ficam fora deste critério (o visualizador é de `TASK-E17-09` e `TASK-E17-10`) | cada sinal é medido pelo protocolo, os parâmetros do AnalyserNode, a balística e a resposta de cada série são registrados, e a tolerância de cada série é o maior desvio absoluto contra o oráculo Rust, nunca acima de 0,1 dB no nível, 0,5 dB no espectro e 1e-4 na onda; acima de um teto, o critério falha | medidores e ondas respondem a degrau, seno e rajada com tolerância documentada |
| `TASK-E02-13-AC-3` | cada uso de Web Audio do MVP identificado na varredura | o uso é traduzido para um comportamento descrito em formato consumível pelo Rust | Web Audio entra só como comportamento reimplementável. |

## Protocolo de medição

Vale para o AC-2. Cada item abaixo é parte do critério, não sugestão.

1. **Estado do MVP.** Faixa não demo; `state.playing = true`; volume 1,0; sem mudo; `state.fps = 30`; sem `reducedMotion`; aba visível; EQ desligado; restore e grafo inativos; nenhum visualizador de `waveform-styles.js` ativo. `state.meter` é `06-hifi-spectrum` ou `12-osciloscopio-crt`, conforme a execução.
2. **Grafo de áudio.** O `AudioContext` criado por `setupAudio` é trocado por um `OfflineAudioContext` de 48 kHz e 96000 amostras (2,0 s exatos, sem cauda); a fonte de mídia é trocada pelo sinal sintético em estéreo idêntico. Os `AnalyserNode` e o resto do grafo são os do próprio `setupAudio` do MVP, não uma reimplementação.
3. **Sinais.** Portadora de 1 kHz, fase 0, bordas em corte seco, amplitude 0,5:
   - degrau: silêncio até 0,5 s, portadora de 0,5 s a 2,0 s;
   - seno: portadora de 0 a 2,0 s;
   - rajada: ligada em t = 0, 50 ms ligada e 450 ms desligada, por 2,0 s (4 rajadas).

   O RMS de −9,03 dB do degrau, entre 0,5 s + 2048/48000 s e 2,0 s, é referência, não critério.
4. **Execuções.** Uma execução por medidor e por sinal (6 no total), cada uma num contexto novo; nenhuma outra leitura de analisador acontece no mesmo contexto, para que o `smoothingTimeConstant` avance só nas leituras do protocolo.
5. **Grade e cadência.** Quadros `k` de 0 a 119, com `t_k = k*1000/60` ms avaliado em float64 nessa ordem de operações. `lastMeter` começa em 0. Há leitura no quadro em que `t_k - lastMeter >= 1000/30`, a regra literal do `frame` do MVP. Isso dá 42 leituras, com prefixo 2, 4, 7, 10, 12 e última no quadro 118; o teste confere a contagem e o prefixo.
6. **Instante de leitura.** O contexto é suspenso na amostra `ceil(t_k*48/128)*128` (arredondada para cima ao quantum de 128 amostras; um instante já na fronteira fica nela). A amostra real é gravada, e o oráculo calcula sobre ela. Antes do início do sinal, o pré-roll é zero.
7. **Séries.** Por canal e por leitura:
   - nível: RMS das 2048 amostras de `getFloatTimeDomainData` → `20*log10(max(1e-8, rms))` recortado em [−60, 0] dB; o desvio é medido em dB; teto 0,1 dB;
   - espectro (`06-hifi-spectrum`): os bins que o medidor lê a partir de `frequenciesHz`, gravados depois do recorte em [−60, 0] dB (sem `-Infinity` nem `null`); teto 0,5 dB;
   - onda (`12-osciloscopio-crt`): `sampleBuffers[ch][i*4]` para `i` de 0 a 180, em amplitude; teto 1e-4.

   Cada série registra o analisador que a alimenta (`fftSize` e `smoothingTimeConstant`).
8. **Oráculo.** Código de teste em `tests/` do `vynamp-player`, nunca de produção. Reimplementa RMS → dB e o `AnalyserNode` (janela, FFT, suavização, conversão em dB) e calcula sobre as amostras gravadas no item 6.
9. **Reprodutibilidade.** `movimento.json` grava a versão do Chrome, do sistema operacional e do Node; os floats são gravados por `JSON.stringify` em float64. Duas execuções do gerador com os mesmos três valores produzem sha256 idêntico. Essa conferência é local, na máquina do mantenedor, e a evidência vai para o PR; o `cargo test` do CI compara o `movimento.json` commitado contra o oráculo.

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | o gerador de séries `rAF` sob tempo virtual, as transições CSS e as constantes de `AnalyserNode` do MVP; para o AC-2, o "Protocolo de medição" | duas execuções do gerador produzem `movimento.json` com sha256 idêntico (conferência local, item 9 do protocolo), e o `cargo test` mostra as séries dentro da tolerância declarada e dos tetos do AC-2; hoje não há gerador nem comparação no repo |
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
