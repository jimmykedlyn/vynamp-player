# TASK-E13-06 — Integrar streaming, podcasts e downloads Android

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E13-06` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E13-06` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E13-03`, `TASK-E13-04`, `TASK-E05-03`, `TASK-E05-05` |
| Bloqueia | `TASK-E13-09` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | alto |

## Entrega

Adapters do core para HTTP/ICY, podcasts e jobs de download respeitando condições do sistema.

## Boundary

- **Não** implementa o parser ICY nem o protocolo de rádio Icecast/SHOUTcast no core. Rádio Icecast é TASK-E05-03; aqui só se liga o adapter Android ao que o core expõe, e reescrever o parser aqui faria o mesmo protocolo ter duas implementações divergentes.
- **Não** busca o feed nem modela episódios e temporadas de podcast. Feeds e episódios de podcasts são TASK-E05-05; esta TASK respeita as condições de rede e de sistema dos jobs, e confundir as duas deixa o modelo de episódio preso ao Android.
- **Não** aplica audio focus, duck nem roteamento de saída do aparelho. Audio focus e rotas Android são TASK-E13-03; aqui o escopo é o transporte HTTP/ICY e o agendamento, e um duck aplicado neste ponto quebraria a política de foco já decidida.
- **Não** é dona do serviço em foreground nem do MediaSession que sobrevive à tela apagada. O serviço e o MediaSession com engine Rust são TASK-E13-02; esta TASK só prova que o adapter não perde a sessão, e implementar o serviço aqui duplicaria o owner do playback.
- **Não** define o checkpoint de processo nem o release de wake locks que sustenta a reprodução de 2 h. Recovery de processo e controle de energia são TASK-E13-07; o screen-off daqui depende daquela infraestrutura, e não é dela que se trata.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e13-06-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E13-06-AC-1` | um stream Icecast 2.4.4 em 128 kbps MP3 a 44,1 kHz com icy-metaint de 1 024 bytes, e um podcast RSS com episódio de 38 MiB servido por HTTP Range com throttle de 256 KiB/s | o usuário toca o stream, apaga a tela do aparelho, deixa 30 min de reprodução e depois cancela o download em 62% e o reinicia pelo botão de retry na notificação | Cancel/retry e rádio screen-off funcionam |
| `TASK-E13-06-AC-2` | aparelho Android com rede móvel marcada como metered pelo ConnectivityManager e 15% de bateria em BatteryManager, mais download agendado em WorkManager com constraint de Wi-Fi não metered e bateria acima de 20% | os jobs são enfileirados pelo app e a rede é alternada de Wi-Fi para metered durante o download, enquanto READ_MEDIA_AUDIO está negada e o episódio vem de feed por HTTPS | permissões, rede e jobs respeitados |
| `TASK-E13-06-AC-3` | sessão Rust tocando um FLAC local em 44,1 kHz estéreo pelo output AAudio do serviço, mais um episódio de podcast pausado cujo download termina e um toque no stream Icecast pela notificação enquanto a sessão local segue ativa | a ponte de comando troca a fonte para o podcast e depois para o Icecast, mantendo o mesmo serviço e o mesmo clock de sessão | não iniciar dois decoders produzindo áudio simultaneamente. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | Teste Rust novo em apps/player/tests/android_stream.rs (inexistente: o repo tem um único src/main.rs com println!("vynamp_player 0.1.0"), zero dependências e nenhum tests/): servidor TCP local que emula ICY com icy-metaint 1024 e serve 120 KiB de payload MP3, mais um vetor de constraints do WorkManager metered/charging/battery lido de JSON, mais um vetor de troca de fonte com contagem de decoders ativos por source id. | A evidência tem que mostrar os 120 KiB de payload entregues na ordem original, com exatamente 1 024 bytes de áudio entre blocos de metadado, 0 bytes de metadado chegando ao sink e 1 única conexão aceita no servidor; no vetor de constraints, 1 job agendado em rede metered e 0 executado, e 1 executado quando volta a Wi-Fi não metered; na troca de fonte, pico de 1 decoder com a saída de áudio ativa e 2 decoders nunca simultâneos em toda a sequência. Hoje nada disso roda porque não há teste, dependência HTTP nem adapter. |
| Hardware | ausente: exige aparelho Android com ConnectivityManager metered real, WorkManager e tela apagada por 30 min para o critério de rádio em screen-off; este ambiente é Windows 11 x86_64 sem aparelho Android, então só o servidor ICY local e o vetor de constraints são exercitáveis. | ausência é limitação registrada, não aprovação |

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
