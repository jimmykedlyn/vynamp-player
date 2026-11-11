# TASK-E16-05 — Entregar document types, Quick Actions e Finder nos dois Macs

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E16-05` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E16-05` |
| Critérios da spec | `specs/player.md` §5 (rascunho, não aprovado) |
| Fase | M2 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E16-02`, `TASK-E16-09`, `TASK-E14-02` |
| Bloqueia | `TASK-E16-07`, `TASK-E18-01` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

UTTypes/document types, Quick Actions (NSServices) no Finder, callbacks LaunchServices e pacotes de ícones in-app.

## Boundary

- **Não** reescreve o bundle nem troca os .icns assinados para mudar os ícones no Finder. A spec, seção 3, diz que isso quebraria a assinatura e a notarização, e o bundle assinado é TASK-E15-08; aqui os pacotes de ícones mudam só a UI interna.
- **Não** torna o Vynamp o handler padrão dos 17 tipos no LaunchServices. O Info.plist declara LSHandlerRank=Alternate e a escolha é sempre do usuário; certificar o estado das associações é TASK-E16-07, e chamar LSSetDefaultHandler aqui violaria a regra de não tomar o default sem permissão.
- **Não** implementa o motor de playback nem o processo único para onde os serviços roteiam as URLs. O dono do envelope e do IPC é TASK-E16-02; a spec pede que application(_:openFiles:) roteie sem recriar processo, o que pressupõe esse canal pronto.
- **Não** gera os UTTypes e os document types a partir do build verificado. O dono é TASK-E16-09; aqui o Info.plist é consumidor dessa lista, não a fonte dela.
- **Não** valida DMG, ZIP Universal2 nem notarização. O dono é TASK-E15-08, e a confusão é plausível porque o AC-1 fala de duas arquiteturas e alguém pode achar que o Universal2 desta TASK inclui o empacotamento.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e16-05-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E16-05-AC-1` | Vynamp.app Universal2 assinado com Info.plist declarando 17 CFBundleDocumentTypes com CFBundleTypeRole=Viewer e LSHandlerRank=Alternate e 4 chaves NSServices, instalado em /Applications num Mac Intel com macOS 14.5 e num Mac Apple Silicon M1 com macOS 14.5, cada um com 6 .flac de 44,1 kHz em ~/Musica | o Finder envia os 6 arquivos por Abrir com Vynamp com o app fechado e, depois, envia mais 4 arquivos ao app já em execução, em cada uma das duas arquiteturas | Intel e Silicon recebem cold/warm batches via application(_:openFiles:) |
| `TASK-E16-05-AC-2` | Info.plist com NSServices contendo Reproduzir no Vynamp, Adicionar à fila do Vynamp, Converter com Vynamp e Restaurar com Vynamp Restore, e o app já aberto ao menos uma vez para o pbs registrar os serviços no LaunchServices | o usuário seleciona 2 arquivos .flac no Finder e abre o menu de Serviços e a categoria Quick Actions do menu de contexto | Quick Actions no Finder ativas |
| `TASK-E16-05-AC-3` | bundle com as três famílias de ícones empacotadas como .icns e como PNGs in-app para os 17 formatos, e a família document marcada como ativa em Preferências | o usuário escolhe a família signal no seletor de Pacote de Ícones e abre a lista de faixas com os 17 formatos visíveis | pacotes de ícones aplicados na UI interna |
| `TASK-E16-05-AC-4` | Safari definido pelo usuário como handler de public.mp3 e Preview como handler de com.microsoft.waveform-audio em com.apple.launchservices.secure.plist, com o Vynamp declarando LSHandlerRank=Alternate e o sha256 desse plist anotado antes da instalação | o Vynamp é instalado e aberto e o usuário aciona Abrir configurações do sistema sem escolher o Vynamp como aplicativo padrão de nenhum tipo | defaults preservados. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | Fixture shell nova em scripts/e16-05-macos.sh, a rodar num Mac Intel e num Mac Apple Silicon com macOS 14.5. O repo vynamp-player tem hoje só src/main.rs com println!("vynamp_player 0.1.0"), zero dependências e nenhum tests/, e o Info.plist do umbrella, packaging/macos/Info.plist, tem 9 chaves e nenhuma delas é CFBundleDocumentTypes ou NSServices, com LSMinimumSystemVersion=13.0. A fixture roda plutil -lint nos dois Info.plist, plutil -extract CFBundleDocumentTypes xml1 -o - e plutil -extract NSServices xml1 -o -, captura log stream com predicate de subsystem com.vynamp durante as 6 mais 4 aberturas e compara o sha256 de com.apple.launchservices.secure.plist antes e depois. | A evidência exigida é plutil sem erro, 17 CFBundleDocumentTypes com LSHandlerRank=Alternate em todos e 4 chaves NSServices; 2 eventos de openFiles por arquitetura, com 6 e 4 URLs na ordem do Finder, sempre 1 processo Vynamp e 0 duplicados; e sha256 de launchservices.secure.plist idêntico antes e depois. Hoje nada disso é produzível: não há bundle, .icns nem chave de document type no Info.plist versionado, e este host não tem macOS para rodar plutil ou log stream. |
| Hardware | ausente: o AC-1 exige um Mac Intel com macOS 14.5 e um Mac Apple Silicon M1 com macOS 14.5, e este host é Windows 11 x86_64 sem macOS, sem Xcode, sem Finder e sem plutil; nenhum dos quatro ACs do arquivo é executável aqui | ausência é limitação registrada, não aprovação |

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
