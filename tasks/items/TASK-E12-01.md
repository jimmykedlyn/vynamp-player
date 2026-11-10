# TASK-E12-01 — Entregar Linux nativo e MPRIS na 1.0

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E12-01` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E12-01` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E14-02`, `TASK-E02-05` |
| Bloqueia | `TASK-E12-03`, `TASK-E14-04`, `TASK-E14-05`, `TASK-E15-07` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 10–25 dias úteis |
| Risco | alto |

## Entrega

Porta Linux x86_64, PipeWire/PulseAudio, MPRIS, Orca, Secret Service e packaging testado.

## Boundary

- **Não** valida a matriz de empacotamento Linux (AppImage/DEB, hash da ferramenta, libs/notices e manifesto assinado); essa validação é de TASK-E15-07, que esta TASK bloqueia — aqui se produz o pacote instalável, lá se audita e assina, e quem executar E12-01 pode achar que o aceite de TASK-E15-07 já está coberto porque o AppImage abre e toca.
- **Não** cobre ARM64 Linux nem helpers adicionais; essa é TASK-E12-03, listada em `blocks`, e a tentação de publicar `aarch64` junto é real porque o mesmo pipeline cross-compila, mas a matriz ARM64 é expansão separada e não requisito implícito do Linux x86_64.
- **Não** executa o corpus e os outputs em todos os alvos (Windows, macOS, Android); aqui só o alvo Linux. O relatório por OS/ABI é TASK-E14-04, também em `blocks`, e a confusão surge porque o mesmo harness de corpus é usado pelos dois.
- **Não** define os traits portáveis de Path, clock, media controls, config e credenciais; o dono é TASK-E14-02, dependência desta TASK. Aqui só se implementa o adapter Linux (MPRIS e Secret Service) que consome esses traits, e trazer o contrato genérico para cá criaria dois donos do mesmo contrato multiplataforma.
- **Não** entrega DSD nativo, DoP nem PCM 768 kHz, mesmo que o caminho ALSA direto no Linux dê a impressão de que basta abrir o dispositivo em taxa alta; essa entrega é TASK-E12-04 e continua condicionada a DAC compatível.
- **Não** fecha o release 1.0 agregado nem a matriz de evidência de todos os alvos; isso é TASK-E14-06, apontada em `platforms/01-matriz-suporte.md` como dependente de TASK-E12-01, e aprovar aqui não significa lançamento multiplataforma.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e12-01-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E12-01-AC-1` | um host Ubuntu 22.04 x86_64 com PipeWire 0.3.48 em modo pulse compatível e áudio onboard HDA, com o binário release do player construído no próprio host a partir de `apps/player` por `cargo build --release --target x86_64-unknown-linux-gnu`, e o corpus comum de seis entradas em `fixtures/corpus` do repositório do player: FLAC 44,1 kHz/16, FLAC 96 kHz/24, MP3 320, Vorbis q6, WAV 44,1 kHz e uma playlist HLS de rádio real | o harness de gate por alvo que esta TASK entrega é executado no host Linux com os marcos M1, M2 e M3 sobre as seis entradas do corpus inteiro, e para cada fixture o app toca até o último frame, pausa, retoma e busca para uma posição no meio da faixa | Passar corpus comum e gates M1–M3 no Linux nativo |
| `TASK-E12-01-AC-2` | duas sessões gráficas no mesmo host Linux x86_64, uma GNOME em Wayland e outra em Xorg, ambas com PipeWire ativo, som onboard e um DAC USB conectado como segundo cartão de áudio | o app é aberto em cada sessão, a reprodução começa na saída onboard, a saída é trocada para o DAC USB com o áudio tocando e a lista de dispositivos e taxas é lida na UI e em `pw-cli list-objects` | Wayland/X11 e portas/saída testados |
| `TASK-E12-01-AC-3` | o `dist/release/vynamp-1.0.0-linux-x64.deb` e o `dist/release/vynamp-1.0.0-linux-x64.AppImage` gerados por `cargo xtask package --platform linux-x64 --version 1.0.0` e instalados num container `ubuntu:22.04` limpo iniciado por `docker run --rm`, sem Wine, sem Mono e sem runtime Windows de qualquer tipo | os artefatos são inspecionados com `file`, com `dpkg -c` no DEB e com `ldd` no `usr/bin/vynamp` do AppDir extraído por `--appimage-extract` e no binário instalado, e o app em execução é listado com `ps -eo pid,comm,args` filtrando `wine`, `wineserver` e `wine-preloader` | não usar Wine como substituto |
| `TASK-E12-01-AC-4` | um host Ubuntu 22.04 x86_64 e um Fedora estável de referência, ambos sem nenhuma instalação prévia do Vynamp e sem toolchain Rust | o DEB é instalado com `dpkg -i vynamp-1.0.0-linux-x64.deb` seguido de `apt-get install -f`, e o `vynamp-1.0.0-linux-x64.AppImage` é copiado para `/opt` com `chmod +x` e executado; depois cada um é lançado pelo menu de aplicativos e pela linha de comando | instalar pacote real. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | Nesta máquina, hoje, medido: `cargo xtask ci detect` responde `{"android":"false","core":"false"}`; `cargo xtask ci gate` para em `Missing configuration: DOCS`; `cargo xtask ci core --target x86_64-unknown-linux-gnu` para em `Actual desktop app absent`; `cargo xtask package --platform linux-x64 --version 1.0.0` para em `Release disabled: actual player/build/hardware/signing acceptance missing` antes de chegar a `check_host` em `xtask/src/util.rs:384`, que fora de runner Linux recusaria com `Platform must be built on its native runner`. No host Linux da TASK a cadeia é a mesma mais `dpkg -c dist/release/vynamp-1.0.0-linux-x64.deb`, `ldd` no binário do pacote, `./dist/release/vynamp-1.0.0-linux-x64.AppImage --appimage-extract` mais `ldd` nele, `busctl --user introspect org.mpris.MediaPlayer2.vynamp` e `ps -eo comm,args` durante a reprodução. | Evidência esperada depois da TASK: as 6 entradas do corpus aprovadas nos três marcos, com o mesmo número de fixtures aprovadas nas sessões Wayland e Xorg e hashes de amostra idênticos aos de referência do decoder, que hoje são as 9 linhas `ref` de `libs/decode/tests/fixtures/flac/MANIFEST.txt` (WAV gerados por ffmpeg n8.1.3); `ldd` sem nenhuma lib `not found` nos dois pacotes; introspect do MPRIS com as Properties de metadados e posição e o nó de playlists; troca onboard para DAC USB sem erro de device no log; zero processos `wine`. O que ainda não existe, medido hoje: `apps/player` tem 1 arquivo `.rs` de 3 linhas (`println!("vynamp_player 0.1.0")`), zero dependências, nenhum `tests/` e `cargo test` com 0 testes; não há nenhuma ocorrência de MPRIS em código `.rs` de `apps`, `libs` ou `xtask`, só em prosa de task; `packaging/runtime-files.json` tem `audit_complete: false` com as três listas vazias e `appimagetool.url` e `.sha256` nulos em `packaging/tools.lock.json`, então nenhum DEB ou AppImage chega a ser produzido; o corpus comum não existe — `libs/decode` tem 52 FLAC, 21 MP3 e 17 WAV, e 0 arquivos Ogg/Vorbis e 0 HLS; e `libs/decode` fecha 508 testes aprovados com 3 ignorados em 47 suítes (`cargo test --locked`), que é o máximo de decoder que existe hoje. |
| Hardware | ausente: o critério exige host Linux x86_64 nativo (Ubuntu 22.04 e Fedora) com PipeWire/PulseAudio, duas sessões, uma Wayland e uma Xorg, DAC USB e leitor de tela Orca; esta máquina é Windows 11 x86_64 (PROCESSOR_ARCHITECTURE=AMD64), sem placa de áudio profissional e sem Orca; há WSL2 com uma distro Ubuntu parada e Docker instalado, mas `dpkg`, `dpkg-deb`, `pw-cli` e `busctl` não estão no PATH do host, não há sessão Wayland nem Xorg com cartão de áudio e o DAC USB do AC-2 não existe, então nenhum dos quatro critérios pode ser medido aqui. | ausência é limitação registrada, não aprovação |

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
