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

- **Não** valida a matriz de empacotamento Linux (AppImage/DEB, tool hash, libs/notices e manifesto assinado); essa validação é de TASK-E15-07, que esta TASK bloqueia — aqui se produz o pacote instalável, lá se audita e assina, e quem executar E12-01 pode achar que o aceite de TASK-E15-07 já está coberto porque o AppImage roda.
- **Não** cobre ARM64 Linux nem helpers adicionais; essa é TASK-E12-03, listada em `blocks`, e a tentação de publicar `aarch64` junto é real porque o mesmo pipeline cross-compila, mas a matriz ARM64 é expansão separada e não requisito implícito do Linux x86_64.
- **Não** executa o corpus e os outputs em todos os alvos (Windows, macOS, Android); aqui só o alvo Linux. O relatório por OS/ABI é TASK-E14-04, também em `blocks`, e a confusão surge porque o mesmo harness de corpus é usado pelos dois.
- **Não** define os traits portáveis de Path, clock, media controls, config e credenciais; o dono é TASK-E14-02, dependência desta TASK. Aqui só se implementa o adapter Linux (Secret Service, MPRIS) que consome esses traits, e trazer o trait para cá criaria dois donos do mesmo contrato multiplataforma.
- **Não** entrega DSD nativo, DoP nem PCM 768 kHz, mesmo que o caminho ALSA direto Linux dê a impressão de que basta abrir o dispositivo em taxa alta; essa entrega é TASK-E12-04 e continua condicionada a DAC compatível.
- **Não** fecha o release 1.0 agregado nem a matriz de evidência de todos os alvos; isso é TASK-E14-06, apontada explicitamente em `platforms/01-matriz-suporte.md` como dependente de TASK-E12-01, e aprovado aqui não significa lançamento multiplataforma.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e12-01-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E12-01-AC-1` | um host Ubuntu 22.04 x86_64 com PipeWire 0.3.48 (pulse compatível) e áudio onboard HDA, e o binário release compilado no próprio host via `cargo build --release --target x86_64-unknown-linux-gnu`, com o corpus comum em `fixtures/corpus` (FLAC 44,1 kHz/16, FLAC 96 kHz/24, MP3 320, Vorbis q6 e WAV 44,1 kHz, mais um HLS de rádio real) | o binário nativo é executado sob o harness de gate com `cargo xtask gate --os linux --milestones M1,M2,M3 --corpus fixtures/corpus`, reproduzindo cada fixture até o fim e pausando/retomando e buscando no meio | Passar corpus comum e gates M1–M3 no Linux nativo |
| `TASK-E12-01-AC-2` | duas sessões gráficas no mesmo host Linux x86_64, uma GNOME em Wayland e outra em Xorg, ambas com PipeWire ativo, som onboard e um DAC USB conectado por um segundo cartão de áudio | o app é aberto em cada sessão, a reprodução começa na saída onboard, a saída é trocada para o DAC USB com o áudio tocando e a lista de dispositivos e taxas é lida na UI e em `pw-cli list-objects` | Wayland/X11 e portas/saída testados |
| `TASK-E12-01-AC-3` | o pacote de release Linux (DEB e AppImage) instalado em um container Ubuntu 22.04 x86_64 limpo, sem Wine, sem Mono e sem runtime Windows de qualquer tipo | o conteúdo e as dependências dos artefatos são inspecionados com `file`, `ldd` nos binários do pacote e `ldd` no binário do AppImage extraído, e o grafo de processos do app em execução é listado com `ps -eo pid,comm,args`, procurando qualquer processo `wine`, `wineserver` ou `wine-preloader` | não usar Wine como substituto |
| `TASK-E12-01-AC-4` | um host Ubuntu 22.04 x86_64 e um Fedora estável de referência, ambos sem nenhuma instalação prévia do Vynamp e sem toolchain Rust | o DEB é instalado com `dpkg -i vynamp-player_*_amd64.deb` seguido de `apt-get install -f`, e o AppImage é copiado para `/opt` com `chmod +x` e executado; depois cada um é lançado pelo menu de aplicativos e pela linha de comando | instalar pacote real. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo xtask gate --os linux --milestones M1,M2,M3 --corpus fixtures/corpus` em Ubuntu 22.04 x86_64 nas sessões Wayland e Xorg, mais `dpkg -c vynamp-player_*_amd64.deb`, `./Vynamp-*.AppImage --appimage-extract` seguido de `ldd` nos binários, `busctl --user introspect org.mpris.MediaPlayer2.vynamp` e `ps -eo comm,args` durante a reprodução para provar ausência de processo Wine. | Evidência esperada: relatório por milestone com a mesma contagem de fixtures aprovadas e os mesmos hashes de decodificação do golden do Windows (M1, M2 e M3 completos, nenhum box de silêncio ou formato omitido), introspect do MPRIS listando `Properties` de metadados/posição e o nó `Playlists`, troca onboard para DAC USB sem erro de device no log e `ldd` sem lib ausente nos dois pacotes. Hoje nada disso existe: o repo `vynamp-player` tem 1 arquivo `.rs` (`src/main.rs`) com um `println!("vynamp_player 0.1.0")`, zero dependências e nenhum `tests/`, logo não há binário de áudio, nem harness de gate, nem MPRIS, nem fixture de corpus para medir. |
| Hardware | ausente: o critério exige host Linux x86_64 real (Ubuntu 22.04 e Fedora) com PipeWire/PulseAudio, duas sessões Wayland e Xorg, DAC USB e leitor de tela Orca; este ambiente é Windows 11 x86_64 sem Linux instalado e sem dispositivo de áudio profissional, então o Linux nativo e a captura do Orca ficam bloqueados até haver máquina Linux de referência. | ausência é limitação registrada, não aprovação |

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
