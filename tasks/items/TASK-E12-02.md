# TASK-E12-02 — Entregar macOS Intel e Apple Silicon nativos na 1.0

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E12-02` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E12-02` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E14-02`, `TASK-E02-05` |
| Bloqueia | `TASK-E12-03`, `TASK-E14-04`, `TASK-E14-05`, `TASK-E15-08` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 10–25 dias úteis |
| Risco | alto |

## Entrega

CoreAudio, UI/acessibilidade, Now Playing, Keychain e distribuição x86_64+arm64 nativas.

## Boundary

- **Não** assina, notariza nem faz staple do DMG/ZIP nem audita slices e runtime inside-out; essa é TASK-E15-08, em `blocks`, e a confusão é plausível porque esta TASK gera o artefato Universal2 e quem executar pode considerar a validação de notarização parte do mesmo trabalho.
- **Não** entrega ARM64 Windows/Linux nem o inventário de helpers da matriz adicional; o dono é TASK-E12-03, também em `blocks`. Confirmar duas slices no macOS não cobre os alvos ARM64 extras, e o mesmo script de bundling tenta os dois.
- **Não** define os traits portáveis de keychain, media controls/Now Playing, clock e Path; o dono é TASK-E14-02, dependência desta TASK. Aqui se escreve o adapter macOS (CoreAudio, MPNowPlayingInfoCenter, Keychain) sobre esses traits; mover o contrato genérico para cá duplicaria o dono do contrato multiplataforma.
- **Não** executa o corpus e os outputs de todos os alvos; o relatório por OS/ABI é TASK-E14-04, em `blocks`, e a sobreposição é plausível porque as fixtures de decodificação são as mesmas que esta TASK roda nos dois Macs.
- **Não** entrega DSD nativo, DoP nem PCM 768 kHz, ainda que CoreAudio aceite taxa alta em DAC externo; essa entrega é TASK-E12-04 e depende de hardware condicionante.
- **Não** fecha o release 1.0 agregado nem a matriz de evidência multiplataforma; isso é TASK-E14-06, e `platforms/01-matriz-suporte.md` exige dois relatórios reais independentes de macOS só no gate por alvo, que não substitui o gate agregado.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e12-02-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E12-02-AC-1` | dois Macs reais com macOS 13.4 ou superior — um Intel x86_64 e um Apple Silicon arm64 — cada um com a slice nativa correspondente instalada a partir do DMG Universal2, VoiceOver ativo e um par AirPods ou DAC USB disponível | o harness `cargo xtask gate --os macos --arch x86_64` e `--arch arm64` roda as fixtures e a navegação por VoiceOver em cada máquina, e durante a reprodução o dispositivo de saída é trocado do onboard para o AirPods e a taxa do dispositivo é mudada de 44,1 kHz para 48 kHz | Fixtures, VoiceOver, device/rate changes e instalação passam em Mac Intel e Silicon reais |
| `TASK-E12-02-AC-2` | o DMG e o ZIP Universal2 do release, com o bundle `Vynamp.app` e todos os seus helpers (helper de áudio e executáveis auxiliares) gerados pelo pipeline | cada executável e helper do bundle é inspecionado com `lipo -info` e `lipo -archs` depois de montar o DMG com `hdiutil attach` e de descompactar o ZIP | Universal2/helpers contêm ambas slices nativas nos DMG/ZIP Universal2 obrigatórios |
| `TASK-E12-02-AC-3` | um Mac Apple Silicon arm64 real com Rosetta 2 não instalado (o payload de Rosetta ausente e `/Library/Apple/usr/libexec/oah` inexistente) | o app é aberto pelo Finder a partir do DMG Universal2, se reproduz e se navega por VoiceOver, e em seguida `sysctl -n sysctl.proc_translated` é consultado para o processo principal e para cada helper, junto de `lipo -archs` nos binários | sem exigir Rosetta. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `hdiutil attach Vynamp-*.dmg` e `unzip -l Vynamp-*-universal2.zip`, seguidos de `lipo -info` e `lipo -archs` em `Vynamp.app/Contents/MacOS/*` e em cada helper do bundle, mais `codesign -dvvv --entitlements :-` por executável; nos dois Macs reais, `cargo xtask gate --os macos --arch x86_64` e `--arch arm64` com VoiceOver ligado e um roteiro de troca de dispositivo e taxa (onboard para AirPods, 44,1 kHz para 48 kHz) durante a reprodução. | Evidência esperada: `lipo -archs` retornando `x86_64 arm64` para o binário principal e para todos os helpers (nenhum helper com slice única), relatórios dos dois Macs com a mesma contagem de fixtures aprovadas e os mesmos hashes do golden do Windows, log de device/rate change em 44,1 para 48 kHz sem erro de stream nem underrun, e `sysctl.proc_translated` igual a 0 no processo principal e nos helpers de um Mac Silicon sem Rosetta. Hoje nada disso existe: o repo `vynamp-player` tem 1 arquivo `.rs` (`src/main.rs`) com um `println!("vynamp_player 0.1.0")`, zero dependências e nenhum `tests/`, portanto não há bundle, helper, backend CoreAudio nem código de Now Playing a inspecionar. |
| Hardware | ausente: o critério exige um Mac Intel x86_64 e um Mac Apple Silicon arm64 reais com macOS 13+, CoreAudio, VoiceOver e instalação por DMG; este ambiente é Windows 11 x86_64, sem máquina macOS, sem CoreAudio e sem Rosetta para testar, então os dois relatórios reais de macOS ficam bloqueados até haver esse hardware. | ausência é limitação registrada, não aprovação |

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
