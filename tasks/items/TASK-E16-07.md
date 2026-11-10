# TASK-E16-07 — Certificar ativação e associações em cinco alvos

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E16-07` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E16-07` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E16-03`, `TASK-E16-04`, `TASK-E16-05`, `TASK-E16-06` |
| Bloqueia | `TASK-E14-06`, `TASK-E17-20` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Relatórios por OS/ABI de instalação, defaults, lote, erros e concorrência.

## Boundary

- **Não** implementa o registro de associações, o submenu do Explorer e os pacotes .ico no Windows: ProgIDs HKCU, pacote esparso e servidor COM IExplorerCommand são TASK-E16-03, e confundir os dois faz a certificação virar implementação de plataforma.
- **Não** implementa MIME, desktop actions nem extensões de Dolphin, Nautilus, Thunar e Nemo no Linux: a entrega de vynamp.desktop, servicemenus e update-mime-database é TASK-E16-04; aqui só se verifica o que ele registrou.
- **Não** implementa CFBundleDocumentTypes, NSServices e Quick Actions do macOS: Info.plist, bundle Universal2 e assinatura são TASK-E16-05, e a matriz aqui consome esse bundle pronto, não o altera.
- **Não** implementa os filtros ACTION_VIEW, ACTION_SEND e ACTION_SEND_MULTIPLE nem o diálogo Reproduzir agora/Adicionar à fila do Android: manifesto e SAF são TASK-E16-06; certificar no aparelho não autoriza editar o AndroidManifest.xml por conta desta TASK.
- **Não** cobre portable, backup e limpeza de integração: o ZIP sem registro automático, o staging/rollback e a remoção de sobras de upgrade são TASK-E16-08, e é tentador incluir a limpeza do desinstalador aqui porque ela aparece no dump de antes e depois.
- **Não** fecha o release 1.0 multiplataforma: consolidar os relatórios de TASK-E16-07 em um gate de release, com SBOM, assinatura e nota de versão, é TASK-E14-06, que esta TASK apenas bloqueia com a evidência.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e16-07-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E16-07-AC-1` | build M3 candidato do Vynamp instalado nos cinco alvos de referência (Windows 11 x86_64 build 22000+, Linux x86_64 em Ubuntu 22.04 com sessão X11 e outra Wayland, macOS 13 Intel x86_64, macOS 13 Silicon arm64 e aparelho Android API 29+ arm64-v8a), com defaults de terceiros fixados antes da instalação e três fixtures de lote de 1, 2 e 20 arquivos misturando .flac, .mp3, .opus e .m4a | a matriz dos cenários QA-022, QA-023 e QA-024 é executada alvo a alvo sobre esse build, disparando o handler candidato por Abrir com, desktop action, Finder Quick Action e Intent ACTION_VIEW com o mesmo lote de 20 arquivos | QA-022/023/024 passam nos cenários aplicáveis M1–M3 dos cinco alvos |
| `TASK-E16-07-AC-2` | build M3 com o scheme vynamp:// e o client OAuth do provedor desligados pela flag de fase M4 e ausentes do manifesto de cada alvo, e a matriz QA-022/023/024 restrita a Abrir com, desktop action, Quick Action e Intent ACTION_VIEW/ACTION_SEND | um URL vynamp://play apontando para um .flac real e um callback OAuth de redirect são enviados ao processo primário depois que os cinco alvos rodaram a matriz | deep links/OAuth só em M4 |
| `TASK-E16-07-AC-3` | defaults de terceiros gravados antes da instalação em cada alvo: no Windows UserChoice e ProgID de outro player para .mp3 sob HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\FileExts, no Linux mimeapps.list com outro player como default de audio/flac, no macOS LaunchServices com outro app como handler de public.mp3 e no Android um app de terceiro como handler de ACTION_VIEW audio/mpeg | o instalador do Vynamp é aplicado, a integração é ativada pelas Preferências e em seguida o Vynamp é desinstalado, com dump das chaves e arquivos de associação antes, entre e depois de cada passo | terceiros preservados |
| `TASK-E16-07-AC-4` | relatório consolidado da TASK-E16-07 com 1 dos 5 alvos executado (Windows 11 x86_64) e 4 marcados sem execução por não haver máquina macOS Intel, nem Apple Silicon, nem Linux x86_64 com X11 e Wayland, nem aparelho Android arm64 real neste ambiente | o relatório é submetido como evidência de TASK-E16-07 e lido pelo gate de fechamento do release em TASK-E14-06 | ausência de hardware não vira aprovação. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | fixture de certificação com uma linha por alvo e por QA (5 alvos x QA-022/023/024) preenchida a partir de dump de registro e de associação antes, entre e depois do ciclo instalar/ativar/desinstalar: reg export de HKCU\Software\Classes e de FileExts\UserChoice no Windows, cat de ~/.config/mimeapps.list e xdg-mime query default no Linux, lsregister -dump no macOS e dumpsys package mais cmd package query-activities no Android; o mesmo lote de 20 arquivos é disparado duas vezes por handler para checar commit único e idempotência | a tabela tem que sair com 15 células, cada uma PASS, FAIL ou NOT-RUN com motivo por escrito, e o consolidado só conta como aprovado quando 15 de 15 são PASS nos alvos executados e as demais aparecem como limitação declarada; hoje o repo vynamp-player tem 1 arquivo .rs, src/main.rs, com um println! que imprime vynamp_player 0.1.0, zero dependências e nenhum tests/, então não existe harness, nem fixture, nem handler candidato que produza qualquer célula: o estado de partida é 15 de 15 NOT-RUN e nenhum dos comandos acima roda contra este repo sem que E16-03 a E16-06 entreguem seus instaladores |
| Hardware | ausente: a certificação exige aparelho Android API 29+ arm64-v8a real com outro app instalado como handler, máquina macOS 13 Intel x86_64, máquina macOS 13 Apple Silicon arm64 e Linux x86_64 com sessão X11 e Wayland; este ambiente é apenas Windows 11 x86_64 sem placa de áudio profissional, então 1 dos 5 alvos é executável e os outros 4 ficam NOT-RUN com a lacuna escrita no relatório, nunca aprovados por omissão | ausência é limitação registrada, não aprovação |

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
