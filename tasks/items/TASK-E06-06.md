# TASK-E06-06 — Entregar visualizações avançadas

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E06-06` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E06-06` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E06-05` |
| Bloqueia | `TASK-E06-07`, `TASK-E17-09` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Espectrograma, partículas, fullscreen, presets e fallback de CPU.

## Boundary

- **Não** entrega o canal de observação, a FFT, o VU, o espectro nem o osciloscópio. São `TASK-E06-05`, dependência declarada desta; a confusão é plausível porque o espectrograma consome o mesmo tap e os mesmos 2048 pontos, mas aqui se constrói a cena e o render, não a análise, e duplicar a análise faria dois donos do ring de observação.
- **Não** define o schema de preset de cena, as famílias, os taps nem o orçamento agregado por instância. Isso é `TASK-E17-09`, que esta TASK bloqueia; aqui os presets são insumo já validado por `schemas/visualizer-preset.schema.json`, e redefinir os limites aqui daria duas autoridades sobre o mesmo contrato.
- **Não** implementa o sandbox de runtime para renderer de terceiro, com fuel, deadline e capabilities. O spike Wasmtime e o WIT são `TASK-E07-01`; nesta TASK só existe cena declarativa e efeito builtin pinado, e preset de pacote não carrega GLSL, WGSL, JS nem binário na 1.0.
- **Não** certifica energia mobile, background Android, nem faz o soak de 8 h com matriz de CPU e GPU. É `TASK-E17-10`, dono da QA-028 como gate; aqui se prova o fallback numa sessão com device loss injetado, e apresentar isso como certificação repetiria o erro de aceitar material gráfico como aceite de áudio.
- **Não** constrói a janela Mini, as preferências nem os estados de erro da UI. `TASK-E02-06` é dona deles, e a geometria multi-monitor com snap e restauração é `TASK-E02-08`; o fullscreen daqui é um estado do visualizador, que devolve foco e posição, não um modo de janela novo.
- **Não** gera o gabarito de movimento nem as curvas de animação do MVP. Transições, séries de rAF e balística são `TASK-E02-13`; esta TASK anima com as constantes publicadas lá, e medir de novo aqui produziria um segundo gabarito divergente do primeiro.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e06-06-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E06-06-AC-1` | uma fixture WAV PCM estéreo de 48 kHz de 30 s, o factory de renderer trocado pelo stub `NoAdapter` que devolve `RequestAdapterError` na criação na primeira passagem e, na segunda, o renderer real com `device.lose()` injetado em t=10 s, com o tap em final e o áudio no backend null de `libs/output` com relógio virtual | a reprodução vai até o fim nos dois passes e os contadores do stream, os eventos recebidos e os frames de áudio entregues são comparados entre o instante anterior e o posterior à perda do device | GPU indisponível não interrompe áudio |
| `TASK-E06-06-AC-2` | o visualizador em fullscreen num monitor de 2560x1440 com escala 100%, aberto a partir do controle do Classic Stack que o abriu, com a janela do player com foco de teclado, nenhum diálogo modal aberto, a posição de transporte em 37,4 s e um atalho global de reprodução ativo | a tecla Escape é pressionada uma vez e, sem sair da tela, a barra de espaço é pressionada em seguida, com o log de foco e de eventos de teclado anexado | Escape sai do fullscreen |
| `TASK-E06-06-AC-3` | o pacote `evil.vyntheme` com `shader: assets/evil.wgsl` declarado no manifest, um `evil.wgsl` de 3 KiB dentro do pacote e um catálogo builtin com apenas `bars`, `wave` e `particles`, mais cinco presets hostis em `apps/player/tests/fixtures/presets/` | o pacote é importado pela galeria, o preset declarado nele é ativado no visualizador e cada fixture é carregada, com o resultado registrado em motivo estável e sem baixar nada da rede | presets não executam shaders arbitrários. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --test viz_presets` no repo `vynamp-player` com três cenários no mesmo binário de teste: o stub `NoAdapter` no lugar do factory de renderer mais um `device.lose()` em t=10 s, uma sessão headless em que Escape e espaço são injetados na janela em fullscreen, e o pacote `evil.vyntheme` mais os cinco presets hostis de `apps/player/tests/fixtures/presets/`. Hoje não roda nada: o `Cargo.toml` do player tem `[dependencies]` vazio, `apps/player/src/main.rs` tem 3 linhas e 54 bytes, não há wgpu, não há janela, não há fullscreen, e o `schemas/visualizer-preset.schema.json` existe sem um único consumidor no código — o Null host de `libs/output` força ausência de dispositivo de áudio, não de GPU, e não desenha nada. | Esperado: com o adaptador negado e com o device perdido, os 30 s ainda entregam 1 440 000 frames de áudio com `Counters::source_underruns` 0, 1 evento de perda de device e exatamente 1 aviso de fallback para CPU no log, e nenhum período zerado por causa do visualizador; o Escape fecha exatamente 1 fullscreen, devolve o foco ao controle que o abriu em 1 evento, mantém a posição de transporte em 37,4 s sem salto e a barra de espaço seguinte ainda alterna play e pause, ou seja o atalho de reprodução não foi sequestrado; dos seis pacotes de preset, 0 carregam, o `evil.vyntheme` é rejeitado por `shader` fora do schema allowlisted com 0 arquivos `.wgsl` compilados, os outros cinco são rejeitados com motivo estável e distinto, e nenhum dos seis abre socket ou baixa arquivo; nenhuma segunda engine é criada em fullscreen. Hoje não existe nenhuma dessas fixtures nem renderer para exercitá-las. |
| Hardware | monitor de 2560x1440 do AC-2, indisponível: esta máquina tem 1 monitor de 1920x1080 ligado à Intel UHD Graphics 620, que existe e renderiza, então a GPU exigida está presente mas a resolução do critério não, e a saída do fullscreen é provada em 1920x1080 com a geometria de 2560x1440 declarada como não reproduzida; AC-1 e AC-3 não pedem hardware algum, porque o adaptador ausente vem do stub `NoAdapter` e a perda de device é injetada. Não há aparelho Android, aparelho iOS nem máquina macOS neste ambiente | ausência é limitação registrada, não aprovação |

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
