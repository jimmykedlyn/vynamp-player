# TASK-E06-05 — Entregar meters e espectro

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E06-05` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E06-05` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E01-05`, `TASK-E02-03` |
| Bloqueia | `TASK-E06-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | alto |

## Entrega

Canal de observação de áudio independente, FFT, VU, espectro e osciloscópio.

## Boundary

- **Não** implementa o catálogo de presets nem os taps limitados de visualização. O dono é `TASK-E17-09`; o canal de observação entrega amostras para meters, e quem for executar pode achar que o contrato de tap allowlisted por binding cabe aqui.
- **Não** define o schema nem o validador do `.vyntheme`, incluindo o nó `visualizer` do layout. O dono é `TASK-E06-01`; aqui se consome o nó declarado, e trazer a definição do schema para cá criaria dois donos do mesmo formato.
- **Não** produz os três temas originais (Classic Graphite, Midnight Modern, Copper) que dão cor e geometria aos meters. O dono é `TASK-E06-04`; esta TASK entrega o sinal, e a aparência é decisão do tema.
- **Não** entrega espectrograma, partículas, fullscreen e fallback de CPU. O dono é `TASK-E06-06`, que depende desta TASK; o espectro escalar daqui não tem histórico de frames, e é fácil confundir os dois.
- **Não** altera o mixer, o drain e as transições. O dono é `TASK-E01-05`, dependência desta TASK; o tap apenas copia amostras já mixadas e não participa do caminho de mixagem.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e06-05-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E06-05-AC-1` | um motor de áudio a 48 kHz com blocos de 256 amostras, faixa WAV PCM 16-bit de 60 s tocando e o canal de observação implementado como tap não bloqueante de 8192 amostras que não aloca no callback | o renderer de meters é travado por 250 ms a cada frame durante 30 s de reprodução e o contador de xruns do callback é lido no fim | Renderer lento não causa underrun |
| `TASK-E06-05-AC-2` | FFT de 2048 pontos com janela Hann alimentada por uma senoide de 1 kHz a 44,1 kHz e pela mesma senoide a 96 kHz, cada uma com 96000 amostras de fixture | o eixo de frequência é calculado pelo mesmo código nas duas taxas e o índice do bin de pico é lido em cada caso | escalas de FFT correspondem à taxa |
| `TASK-E06-05-AC-3` | visualizador de espectro com FFT de 2048 pontos e alvo de 60 fps, contando ticks do renderer e chamadas de FFT por segundo, em janela 1280x720 com a opção de movimento reduzido desligada | a janela é minimizada por 10 s e, restaurada, a opção de movimento reduzido é ligada por mais 10 s, medindo ticks e FFTs em cada um dos três estados | minimizar e reduzir movimento diminuem processamento. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test --test meters_independent (fixture: `fixtures/audio/sine1k-48k-60s.wav` gerada por `scripts/gen_sine.py`, tap de 8192 amostras, sleep de 5 ms injetado por frame no renderer) | hoje `src/main.rs` tem apenas `println!("vynamp_player 0.1.0")`, zero dependências e nenhum `tests/`, então nenhum desses caminhos existe; a evidência exigida é 0 xruns em 30 s com o renderer travado 250 ms por frame, pico no bin 46 (mais ou menos 1) a 44,1 kHz e no bin 21 (mais ou menos 1) a 96 kHz com FFT de 2048 (resolução de 21,53 Hz e 46,88 Hz), e queda de pelo menos 50% nos ticks com janela minimizada e de pelo menos 30% com movimento reduzido. |
| Hardware | nenhum: o canal de observação é alimentado por WAV sintética e nenhum teste abre dispositivo de áudio; o único recurso exigido é filesystem local para a fixture. | ausência é limitação registrada, não aprovação |

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
