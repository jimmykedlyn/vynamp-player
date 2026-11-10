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
| Critérios da spec | pendente — `specs/player.md` §5 |
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

- **Não** prova a bridge Rust e a saída Android nem monta o serviço e o MediaSession. Isso é `TASK-E13-01` e `TASK-E13-02`; aqui só se coleta evidência do que elas entregaram, e misturar as duas coisas faria o aceite depender de código que ainda não existe no repo.
- **Não** implementa audio focus, rotas nem o comportamento de noisy. O dono é `TASK-E13-03`; como o cenário de QA-019 falha em foco, a tentação de consertar durante a homologação é grande, mas consertar aqui esconde o defeito em vez de reprová-lo.
- **Não** implementa MediaStore, SAF nem grants de permissão. O dono é `TASK-E13-04`; o teste de revogação e de acesso negado de QA-020 parece pedir código novo, e o que se espera nesta TASK é apenas registrar como o build do SHA final se comporta sob revogação.
- **Não** entrega recovery de processo nem controle de energia, dono `TASK-E13-07`. O relatório de energia desta TASK mede percentual de bateria e memória agregada contra baseline, e medir não é implementar wake lock nem restauração.
- **Não** decide assinatura, ABI, formato de pacote nem layout de tablet e foldable, donos `TASK-E13-09` e `TASK-E13-05`. A matriz de build e distribuição de QA-021 já tem TASK própria, e refazê-la aqui duplicaria o artefato que esta homologação consome.
- **Não** homologa a matriz de qualidade numérica de desktop nem o endurecimento de rede e streaming, donos `TASK-E04-10` e `TASK-E05-09`. O relatório Android cobre HLS e DSP por alvo, mas os oráculos numéricos compartilhados vêm de lá e não podem ser reescritos para o aparelho passar.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e13-10-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E13-10-AC-1` | o corpus compartilhado da spec (FLAC 24/96, FLAC 16/44,1, MP3 320, HLS VOD de 10 min e feed RSS com 3 episódios) copiado para o armazenamento interno de um aparelho Android arm64 físico de porte médio, 6 GiB de RAM, Android 14, com fone com fio e um adaptador BT pareado, instalado com o APK assinado do SHA final | a bateria de gates comuns mais os cenários de QA-019 (notificação e lockscreen, tela apagada, rota BT, foco de áudio e noisy, rotação, process death) e QA-020 (SAF, MediaStore, revogação de permissão, TalkBack em 200%, tablet) é executada nesse aparelho e o log é capturado por cenário | Gates comuns e Android aprovados no aparelho real |
| `TASK-E13-10-AC-2` | aparelho arm64 físico com um DAC USB anexado que declara 24 bit/96 kHz no descritor, e na fila um FLAC 24/96 cuja taxa não coincide com os 48 kHz que o mixer do sistema usa por padrão | a reprodução é iniciada com a negociação de saída registrada antes do primeiro frame, e o mesmo FLAC é tocado em seguida com o DAC removido, caindo para o alto-falante interno | saída hi-res apenas quando negociada |
| `TASK-E13-10-AC-3` | aparelho arm64 físico com o backend AAudio em uso e o relatório de saída instrumentado para registrar taxa e profundidade de bits efetivamente aceitas pela rota, numa faixa FLAC 24/96 reproduzida com o mixer do sistema ativo em 48 kHz/16 bit | a faixa é tocada até o fim e o relatório de aceite é preenchido com a flag AAudio marcada como sucesso, comparando-a com a taxa e os bits medidos na rota | flag AAudio não é prova de bit-perfect. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | copiar o corpus para o armazenamento interno do aparelho arm64 físico, instalar por `adb install` o APK assinado do SHA final, rodar cada cenário de QA-019 e QA-020 com `adb logcat -s VynampGate` salvo em arquivo por cenário, e medir taxa e bits da rota com dumpsys media.audio_flinger mais a instrumentação do relatório, incluindo 2 h de playback com a tela apagada | o relatório tem uma linha por cenário com resultado, taxa, profundidade de bits e rota medidas (por exemplo 96 kHz/24 bit no DAC USB e 48 kHz/16 bit no alto-falante), 0 underrun atribuível ao app em 2 h, memória agregada abaixo de 200 MiB em 24/96 e abaixo de 100 MiB em idle, e nenhum autoplay após kill. Nada disso existe hoje: o repo vynamp-player tem 1 arquivo .rs (src/main.rs) com um println de vynamp_player 0.1.0, zero dependências e nenhum tests/, logo não há gate Android nem harness de captura para executar |
| Hardware | aparelho Android arm64 físico de porte médio com 6 GiB e Android 14, mais DAC USB, alto-falante, fone com fio e adaptador BT: ausente neste ambiente (Windows 11 x86_64, sem aparelho Android, sem DAC USB e sem rota BT de celular), então o aceite fica como limitação registrada e não como aprovado | ausência é limitação registrada, não aprovação |

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
