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
| `TASK-E02-13-AC-2` | o medidor e o visualizador de onda do MVP, com volume 1,0, sem mudo, EQ desligado e restore e grafo inativos, recebendo a 48 kHz, em estéreo idêntico injetado no lugar da fonte de mídia e renderizado em `OfflineAudioContext` por 2,0 s exatos, sem cauda, três sinais com fase 0 e bordas em corte seco: degrau (portadora de 1 kHz em silêncio até 0,5 s e com amplitude 0,5 até 2,0 s, RMS esperado −9,03 dB de 0,5 s + 2048/48000 s até 2,0 s), seno de 1 kHz com amplitude 0,5 por 2,0 s e rajada de 1 kHz com amplitude 0,5, ligada em t = 0, 50 ms ligada e 450 ms desligada, por 2,0 s (4 rajadas); o modo demo do MVP fica fora deste critério | sob tempo virtual com t_k = k·1000/60 ms, há leitura dos analisadores só no quadro em que `t − lastMeter ≥ 1000/30`, a regra literal do medidor do MVP, e a lista dos quadros lidos é gravada; em cada leitura o contexto offline é suspenso e a amostra real da leitura (a fronteira de 128 amostras seguinte ao instante nominal) é gravada, e o oráculo calcula sobre essa amostra, não sobre o instante nominal; os parâmetros do AnalyserNode, a balística e a resposta de cada sinal são registrados, cada série dizendo qual analisador a alimenta (smoothing 0,75 ou 0), e a tolerância de cada série é medida como o maior desvio absoluto contra um oráculo Rust independente, só código de teste em `tests/` do player: medidor em dB (RMS → dB → fração), teto 0,1 dB; espectro do medidor só nos bins que ele lê, após o recorte em [−60, 0] dB, teto 0,5 dB; espectro do visualizador de onda (`copyLiveSpectrum` em `waveform-styles.js`) como série própria, recortado pela faixa desse consumidor, teto 0,5 dB; onda nas amostras no tempo que ela consome, teto 1e-4 em amplitude absoluta; acima de um teto, o critério falha | medidores e ondas respondem a degrau, seno e rajada com tolerância documentada |
| `TASK-E02-13-AC-3` | cada uso de Web Audio do MVP identificado na varredura | o uso é traduzido para um comportamento descrito em formato consumível pelo Rust | Web Audio entra só como comportamento reimplementável. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | o gerador de séries `rAF` sob tempo virtual e os sinais determinísticos do AC-2, comparado contra as transições CSS e as constantes de `AnalyserNode` do MVP; o oráculo Rust do AC-2 vive só em `tests/` e não é código de produção (o visualizador é de `TASK-E17-09` e `TASK-E17-10`) | duas execuções do gerador, na mesma máquina, com o Chrome do pin (`design/07-pin-mvp.md` do umbrella) e os floats gravados por `JSON.stringify` em float64, produzem `movimento.json` com sha256 idêntico, e as séries batem com o oráculo Rust dentro da tolerância declarada e dos tetos do AC-2 (0,1 dB no medidor, 0,5 dB nos dois espectros, 1e-4 na onda); hoje não há gerador nem comparação no repo |
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
