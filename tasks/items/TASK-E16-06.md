# TASK-E16-06 — Entregar VIEW e Share seguros no Android

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E16-06` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E16-06` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E16-01`, `TASK-E16-09`, `TASK-E13-04`, `TASK-E13-05` |
| Bloqueia | `TASK-E16-07` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Manifest específico, ContentResolver/FD e encaminhamento ao serviço de sessão.

## Boundary

- **Não** implementa MediaStore, SAF de biblioteca nem o fluxo de permissões de leitura de mídia. O dono é TASK-E13-04; aqui só se abre o descritor via ContentResolver para o que chega por intent, e misturar os dois colocaria duas políticas de URI no mesmo módulo.
- **Não** implementa o serviço de sessão nem MediaSession. O dono é TASK-E13-02, e a Entrega desta TASK fala em encaminhar ao serviço de sessão, o que pode ser lido como construir esse serviço; aqui ele é apenas chamada.
- **Não** constrói a interface phone nem a navegação acessível do diálogo de escolha. O dono é TASK-E13-05; esta TASK garante que a escolha é pedida, não como ela é desenhada.
- **Não** define o envelope de ativação comum nem o protocolo compartilhado entre os cinco alvos. O dono é TASK-E16-01, e o formato de argumento que chega ao motor precisa ser o mesmo dos outros SO.
- **Não** executa o aceite Android em hardware real nem a distribuição APK/AAB. Os donos são TASK-E13-10 e TASK-E15-09; aqui se entrega o manifest e o caminho de intents, não a rodada de aceite em aparelho.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e16-06-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E16-06-AC-1` | Android 13 arm64 com targetSdk 34, AndroidManifest.xml com filtros ACTION_VIEW, ACTION_SEND e ACTION_SEND_MULTIPLE para audio/*, a activity do Vynamp já em execução com uma sessão de playback ativa e um lote de 5 content:// URIs de audio/flac do provedor de mídia do sistema | o gerenciador de arquivos dispara ACTION_VIEW com 1 .flac e, em seguida, um app de terceiros dispara ACTION_SEND_MULTIPLE com ClipData de 5 URIs, ambos com FLAG_GRANT_READ_URI_PERMISSION, chegando por onNewIntent | VIEW/SEND/MULTIPLE e onNewIntent preservam grants/ordem |
| `TASK-E16-06-AC-2` | activity do Vynamp com 3 faixas recebidas por ACTION_VIEW, a sessão de reprodução com exatamente 3 itens e a fila com o mesmo tamanho | o usuário gira o aparelho de retrato para paisagem e volta para retrato enquanto a reprodução segue no terceiro item | rotation não duplica |
| `TASK-E16-06-AC-3` | Vynamp aberto com uma playlist ativa de 12 faixas e a sessão tocando a faixa 4, sem diálogo de escolha pendente na tela | um app de mensagens dispara ACTION_SEND_MULTIPLE com 2 arquivos .mp3 e o Vynamp volta ao primeiro plano por cima do app de origem | Share pede ação |
| `TASK-E16-06-AC-4` | intent ACTION_VIEW de um app de nuvem de terceiros trazendo 1 .opus com FLAG_GRANT_READ_URI_PERMISSION e sem FLAG_GRANT_PERSISTABLE_URI_PERMISSION, e o Vynamp já com a fila vazia | o usuário escolhe Adicionar à fila no diálogo de escolha rápida e depois encerra e reabre o Vynamp com o processo morto pelo sistema | sem persistência de grant não oferecido. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | androidTest com am instrument que dispara três intents controlados (ACTION_VIEW com 1 URI, ACTION_SEND com 1 URI e ACTION_SEND_MULTIPLE com ClipData de 5 URIs em ordem conhecida) contra o manifest do módulo, contando chamadas a onNewIntent e takePersistableUriPermission, mais aapt2 dump xmltree do AndroidManifest.xml compilado | Hoje vynamp-player tem só src/main.rs com println!("vynamp_player 0.1.0"), zero dependências e nenhum tests/: não existe módulo Android nem AndroidManifest.xml. A evidência exigida é o manifest com 3 intent-filters e o grupo de MIME audio/*, 3 chamadas a onNewIntent com 1, 1 e 5 URIs preservando a ordem do ClipData, 0 itens duplicados nas 2 rotações (fila permanece com 3), e 0 chamadas a takePersistableUriPermission no caso sem FLAG_GRANT_PERSISTABLE_URI_PERMISSION |
| Hardware | ausente: exige aparelho Android API 29+ arm64 ou emulador arm64 com Play Services para disparar VIEW, SEND_MULTIPLE, rotação e revogação de grant, e este ambiente é Windows 11 x86_64 sem aparelho Android e sem ADB conectado | ausência é limitação registrada, não aprovação |

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
