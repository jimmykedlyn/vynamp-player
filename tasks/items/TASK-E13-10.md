# TASK-E13-10 — Fechar aceite Android 1.0 em hardware

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E13-10` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E13-10` |
| Critérios da spec | `specs/player.md` §5 (rascunho, não aprovado) |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E13-09`, `TASK-E04-10`, `TASK-E05-09` |
| Bloqueia | `TASK-E14-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Relatório de formatos comuns/DSP/HLS, lifecycle/focus/storage/TalkBack e energia.

## Boundary

- **Não** monta a bridge JNI/UniFFI, o adapter Media3 nem o serviço foreground com MediaSession. Donos `TASK-E13-01` e `TASK-E13-02`; a confusão é plausível porque o relatório de aceite lista os gates que dependem deles, mas aqui só se coleta evidência de um build que eles produzem.
- **Não** implementa audio focus, duck, noisy/unplug nem seleção de rota BT. Dono `TASK-E13-03`; como o cenário de foco de QA-019 é o que mais reprova em homologação, a tentação de corrigir durante o aceite é alta, e corrigir aqui esconde o defeito em vez de reprová-lo no SHA medido.
- **Não** implementa MediaStore, SAF, grants persistentes nem negação de permissão. Dono `TASK-E13-04`; o cenário de revogação de QA-020 parece pedir código novo, mas o esperado aqui é registrar como o build do SHA final se comporta sob revogação, inclusive quando o comportamento é ruim.
- **Não** entrega checkpoint de process death, reconexão ao serviço, WorkManager nem wake lock limitado. Dono `TASK-E13-07`; medir bateria e memória agregada contra baseline se parece com implementar controle de energia, e não é: a métrica sai do relatório, o mecanismo é de outra TASK.
- **Não** decide assinatura, ABI, NDK, page size, formato APK/AAB nem o layout de tablet e foldable. Donos `TASK-E13-09` e `TASK-E13-05`; esta TASK consome o APK assinado e a UI que elas entregam, e refazer a matriz de QA-021 aqui duplicaria o artefato que a homologação apenas instala.
- **Não** reescreve a matriz numérica de DSP e formatos nem o endurecimento de rede e streaming. Donos `TASK-E04-10` e `TASK-E05-09`; o relatório Android cobre HLS e DSP por alvo, mas os oráculos numéricos vêm de lá, e afrouxá-los no aparelho transformaria reprovação em aprovação.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e13-10-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E13-10-AC-1` | um aparelho Android arm64 físico de porte médio (6 GiB de RAM, Android 14, arm64-v8a) com o corpus da spec copiado para o armazenamento interno, ou seja os FLAC 24/96 e 16/44,1 de libs/decode/tests/fixtures/flac (st24-96k.flac e st16-44k1.flac), o MP3 320 de libs/decode/tests/fixtures/mpeg/l3-cbr320-44k-lr.mp3, um HLS VOD de 10 min e um feed RSS com 3 episódios montados para a campanha porque o repo não tem nenhum .m3u8 nem feed, mais fone com fio, adaptador BT pareado e um tablet ou foldable de 600 dp ou mais para o cenário de tela grande de QA-020, todos rodando o APK assinado do SHA final instalado por adb install | a bateria dos gates comuns que o relatório Android cobre — QA-001 (open/pause/seek/stop local), QA-002 (gapless FLAC e MP3), QA-003 (hi-res e source-output), QA-005 (DSP e troca rápida de preset), QA-008 (HLS adverso) e QA-011 (device lost, sleep e resume) — é executada nesse aparelho junto com os seis cenários de QA-019 (notificação e lockscreen, tela apagada, rota BT, foco de áudio e noisy, rotação e process death) e os seis de QA-020 (SAF, MediaStore, revogação e negação de permissão, TalkBack em 200%, tablet e jobs de cache com bateria sem wake lock vazando), cada cenário com o log salvo em arquivo próprio por adb logcat | Gates comuns e Android aprovados no aparelho real |
| `TASK-E13-10-AC-2` | o mesmo aparelho arm64 físico com um DAC USB anexado cujo descritor declara 24 bit a 96 kHz, mixer do sistema em 48 kHz por padrão, e na fila o st24-96k.flac de libs/decode/tests/fixtures/flac, cuja taxa de 96 kHz é exatamente o dobro dos 48 kHz do mixer | Play é acionado com o DAC anexado e a linha de negociação de saída do relatório é lida antes do primeiro frame, e em seguida o DAC é desplugado com a faixa tocando e Play é acionado de novo, com a rota do relatório relida | saída hi-res apenas quando negociada |
| `TASK-E13-10-AC-3` | o mesmo aparelho arm64 físico rodando o backend AAudio com o mixer do sistema em 48 kHz e 16 bit, uma faixa st24-96k.flac de libs/decode/tests/fixtures/flac na fila e uma instrumentação de saída, que ainda não existe no repo, gravando em arquivo a taxa e a profundidade de bits que a rota aceitou em cada abertura de stream | a faixa é tocada até o fim e, com o relatório de saída e a saída de dumpsys media.audio_flinger da mesma sessão em mãos, a linha de AAudio do relatório de aceite é preenchida e comparada com a taxa e os bits medidos | flag AAudio não é prova de bit-perfect. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | em `apps/player`: `adb install -r dist/release/vynamp-1.0.0-android-arm64.apk`, que é o diretório de saída de `xtask/src/package.rs:430`, copiar `libs/decode/tests/fixtures/flac/st24-96k.flac` e `mpeg/l3-cbr320-44k-lr.mp3` para `/sdcard/Android/data/<pkg>/files/corpus/`, rodar cada um dos 12 cenários com `adb logcat -v threadtime` salvo em `evidence/android/<cenario>.log` (nenhuma tag de gate existe hoje no código, então a filtragem é pelo pid do processo), e medir a rota com `adb shell dumpsys media.audio_flinger` antes e depois de cada abertura de stream, mais 2 h de playback com a tela apagada e `adb shell dumpsys meminfo <pkg>` a cada 10 min | esperado no relatório: 12 linhas de cenário com resultado, sendo 6 de QA-019 e 6 de QA-020, todas com taxa, profundidade de bits e rota medidas; com o DAC USB anexado sai 96000 Hz e 24 bit no device USB, sem ele sai 48000 Hz e 16 bit no speaker; 0 underrun atribuível ao app nas 2 h, memória agregada abaixo de 200 MiB com 24/96 e abaixo de 100 MiB em idle, que são metas propostas na spec e não budgets já medidos, e 0 sessões de áudio abertas depois de force-stop. Hoje nada disso é produzível: `apps/player/src/main.rs` tem 3 linhas e só o `println!("vynamp_player 0.1.0")`, o `[dependencies]` do `Cargo.toml` está vazio e não existe `apps/player/tests/`, então `cargo test` roda 0 teste; `apps/android/` não existe, logo `xtask/src/ci.rs:204` aborta com 'Actual Android app absent'; `vynamp-output-core/src/support.rs:93` devolve `None` para `HostId::AAudio`, ou seja nenhum backend Android tem verificação; `config/project.json` tem `release_enabled` e `application_ready` em false e `xtask/src/release.rs:76` aborta com 'Release disabled'; não há `evidence/android/`, nem `.m3u8`, nem feed RSS em nenhum lugar do repo, e `adb` não está no PATH desta máquina |
| Hardware | precisa de aparelho Android arm64 físico de porte médio com 6 GiB e Android 14, mais DAC USB que declare 24 bit/96 kHz, alto-falante, fone com fio, adaptador BT pareado e um tablet ou foldable de 600 dp para o cenário de tela grande: ausente neste ambiente (Windows 11 Pro x86_64, sem `adb`, sem Android SDK, com ANDROID_HOME vazio, sem aparelho conectado, sem DAC USB e sem rota BT de celular), de modo que os três AC ficam declarados como bloqueados por hardware e nunca aprovados; o emulador x64 não substitui, porque quality/03-matriz-multiplataforma.md só o admite para automação de UI | ausência é limitação registrada, não aprovação |

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
