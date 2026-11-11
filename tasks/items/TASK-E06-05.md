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
| Critérios da spec | `specs/temas.md` §5 (rascunho, não aprovado) |
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

- **Não** entrega espectrograma, partículas, fullscreen, presets de cena nem fallback de GPU. Tudo isso é `TASK-E06-06`, que esta TASK bloqueia; a confusão é plausível porque a entrega fala em espectro e o espectrograma é o mesmo dado em outra forma, mas aqui o produto é o canal de observação, a FFT e os quatro instrumentos base, e abrir a cena avançada aqui faria dois donos do mesmo orçamento de render.
- **Não** constrói o grafo DSP, o EQ nem o ganho que o medidor resume. A cadeia de reprodução é `TASK-E04-01` e o grafo compilado é `TASK-E04-03`; instrumento de exibição não é efeito e nunca altera a saída, então acrescentar um nó de análise no caminho de reprodução violaria a regra de que medição roda em worker sobre cópias.
- **Não** define a balística nem as constantes de `AnalyserNode` que servem de gabarito. Isso é `TASK-E02-13`, que mede o MVP e publica as curvas em formato consumível pelo Rust; esta TASK consome essas constantes, e reinventá-las aqui daria duas referências para o mesmo ponteiro.
- **Não** calcula LUFS, loudness integrado nem ReplayGain. A análise de loudness, o grouping por álbum e a escrita opcional de tag são `TASK-E08-03`; VU, peak, true peak e LUFS não podem mostrar o mesmo número com rótulos diferentes, e o LUFS é o único dos quatro que pertence a outra TASK.
- **Não** certifica o conjunto: soak de 8 h, matriz CPU e GPU, device-loss sob soak, fullscreen em matriz de telas e energia mobile são `TASK-E17-10`, dono da QA-028 como gate. Aqui se prova que o renderer lento perde frame visual e não causa underrun numa sessão; transformar essa prova pontual em certificado de release assumiria cobertura que não há.
- **Não** produz nem recolore os SVGs de VU. O pacote de referência congelado é `TASK-E02-11` e os tokens e componentes semânticos são `TASK-E02-02`; os arquivos aprovados são preservados byte a byte, e o host aqui fornece dimensão e foco, não desenho.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e06-05-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E06-05-AC-1` | uma fixture WAV PCM estéreo de 48 kHz de 60 s, com o tap de observação em final e o consumidor do ring artificialmente preso a 250 ms por frame, com o ring dimensionado pelo menor limite entre 2 MiB e 250 ms e o descarte contado em vez de esperado | os primeiros 30 s de reprodução são tocados com o renderer preso, e ao fim se leem o descarte do ring de observação e a contagem de underrun publicada pelo stream | Renderer lento não causa underrun |
| `TASK-E06-05-AC-2` | uma senoide de 1 kHz a 0,5 de amplitude amostrada em 44,1, 48, 96 e 192 kHz, com FFT de 2048 e 4096 pontos e janela Hann, e a tabela dourada de pico, largura de bin e Nyquist calculada à mão gravada em `apps/player/tests/fixtures/fft/` | cada combinação de taxa e tamanho de FFT é analisada pelo mesmo código de eixo de frequência e o índice do pico, a largura de bin e a posição de Nyquist são comparados com a tabela dourada | escalas de FFT correspondem à taxa |
| `TASK-E06-05-AC-3` | o visualizador de espectro com FFT de 2048 pontos e alvo de 30 fps numa janela de 1280 por 720, a mesma build e a mesma fixture em três passagens de 30 s: janela visível com movimento reduzido desligado, janela minimizada e janela visível com a preferência de reduzir movimento ligada | os ticks do renderer, as chamadas de FFT e o tempo de CPU do worker de análise são contados em cada uma das três passagens e comparados entre elas | minimizar e reduzir movimento diminuem processamento. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --test visualizer_observer` no repo `vynamp-player`, com fixtures numéricas geradas no próprio teste pela fórmula 0,5·sin(2πfn/Fs) e a tabela dourada de FFT em `apps/player/tests/fixtures/fft/`, mais um consumidor do ring de observação que dorme 250 ms por frame; hoje o repo do player tem um único `.rs`, `apps/player/src/main.rs` com 3 linhas e 54 bytes (`println!` de `vynamp_player 0.1.0`), `[dependencies]` vazio no `Cargo.toml` e nenhum diretório `tests/`, então não há tap, ring, FFT nem renderer para rodar. O único ring SPSC do projeto é o de `libs/output/crates/vynamp-output-core/src/ring.rs`, que conta overrun em vez de esperar, e é o padrão a imitar, não código que o player possa importar. | Esperado: 30 s a 48 kHz são 1 440 000 frames entregues ao dispositivo com `Counters::source_underruns` 0, lido do `libs/output/crates/vynamp-output-core/src/stream.rs`, que é público; o consumidor preso a 250 ms por frame desenha 4 por segundo, ou seja 120 frames em 30 s contra 900 pedidos a 30 fps, e os 780 restantes são descartados sem nenhum período zerado. Escalas: 2048 pontos a 44,1 kHz dá bin de 21,5332 Hz, o pico de 1 kHz cai no índice 46 em 990,53 Hz, erro 9,47 Hz contra 10,77 Hz de meio bin, e Nyquist no índice 1024 em 22 050 Hz; 2048 a 48 kHz dá bin de 23,4375 Hz e pico no índice 43 em 1 007,81 Hz; 4096 a 44,1 kHz dá bin de 10,7666 Hz e pico no índice 93 em 1 001,29 Hz; 2048 a 192 kHz dá bin de 93,75 Hz, pico no índice 11 em 1 031,25 Hz e Nyquist no índice 1024 em 96 000 Hz; são 1 025 bins com 2048 e 2 049 com 4096. Ring no menor limite: 250 ms a 48 kHz estéreo f32 são 12 000 frames e 96 000 bytes, muito abaixo de 2 MiB, mas a 384 kHz e 8 canais o teto de 2 MiB vale 65 536 frames, 170,67 ms, ou seja menos que os 250 ms — no endpoint WASAPI de 384 kHz desta máquina, registrado em `evidence/PR/task-e01-02/probe-steady.json`, com buffer de 3 840 frames. Processamento: em 30 s, 900 frames desenhados com a janela visível a 30 fps, 0 com ela minimizada e no máximo 150 com reduzir movimento ligado, com o worker de análise parado nas duas últimas passagens. Nada disso existe hoje: não há tap, nem FFT, nem contador de frame visual no repo do player. |
| Hardware | nenhum: o ring, o descarte e a FFT são provados sobre o backend null de `libs/output` com relógio virtual, sem abrir dispositivo de áudio e sem GPU. Esta máquina tem endpoint WASAPI de 384 kHz em 2 canais medido em 2026-10-18 e registrado em `evidence/PR/task-e01-02/environment.txt`, mas nenhuma prova desta TASK depende dele; não há placa de áudio profissional, aparelho Android, aparelho iOS nem máquina macOS neste ambiente | ausência é limitação registrada, não aprovação |

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
