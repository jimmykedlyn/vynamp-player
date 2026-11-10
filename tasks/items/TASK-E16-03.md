# TASK-E16-03 — Entregar associação, menu de contexto e pacotes de ícones no Windows

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E16-03` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E16-03` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E16-02`, `TASK-E16-09`, `TASK-E08-06` |
| Bloqueia | `TASK-E16-07`, `TASK-E18-01` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Registro HKCU, pacote esparso MSIX com IExplorerCommand, verbos completos e pacotes de ícones .ico multi-res.

## Boundary

- **Não** gera as declarações de tipo a partir das capabilities do build verificado. Isso é TASK-E16-09, e a confusão é plausível porque o AppxManifest.xml desta TASK também lista extensões de arquivo: aqui elas são consumidas como entrada, e gerar de novo criaria duas fontes para a mesma lista de 17 formatos.
- **Não** implementa o canal IPC nem a eleição do processo primário único. O dono é TASK-E16-02, e o método Invoke só transmite o lote pelo canal que já existe; quem executar esta TASK pode achar que o tratamento de IShellItemArray inclui o protocolo de entrega.
- **Não** adquire certificado Authenticode comercial nem valida a assinatura do instalador. Isso é TASK-E15-06; aqui só se instala o autoassinado de desenvolvimento em Cert:/CurrentUser/Root, e a spec mantém o certificado público como decisão aberta de infraestrutura.
- **Não** cobre modo portátil, backup de associações nem limpeza seletiva de integração. O dono é TASK-E16-08, e o desinstalador desta TASK remove o que instalou, não audita integração alheia.
- **Não** trata MIME, ServiceMenus nem hierarquia hicolor do Linux. O dono é TASK-E16-04; a confusão é forte porque o sistema de três famílias de ícones é compartilhado e a mesma família vira .ico no Windows e PNG no Linux.
- **Não** executa a certificação cruzada de ativação nos cinco alvos. O dono é TASK-E16-07; aqui se entrega o mecanismo do Windows, não o relatório de aceite por alvo.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e16-03-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E16-03-AC-1` | Windows 11 build 22631 com o pacote esparso Vynamp.MediaPlayer registrado e um Windows 10 build 19045 com o registro HKCU clássico e MultiSelectModel=Player, cada um com 9 arquivos .flac de 44,1 kHz em 16 bits numa pasta, selecionados com Ctrl em ordem invertida ao alfabeto | o usuário clica com o botão direito na seleção de 9 arquivos e aciona Vynamp > Reproduzir em cada um dos dois Explorers | Explorer moderno (Win11) e clássico (Win10) abrem lote ordenado por IShellItemArray |
| `TASK-E16-03-AC-2` | NSIS per-user já instalado em %LOCALAPPDATA%/Programs/Vynamp, AppxManifest.xml com identidade Vynamp.MediaPlayer, MinVersion 10.0.22000.0 e as extensões windows.fileTypeAssociation e desktop4:fileExplorerContextMenus, certificado autoassinado presente em Cert:/CurrentUser/Root e nenhum pacote registrado para o usuário | o instalador chama PackageManager.AddPackageByUriAsync apontando para o .msix esparso e aguarda a conclusão da operação | pacote esparso registrado via PackageManager |
| `TASK-E16-03-AC-3` | instalação completa com o pacote esparso registrado, 17 ProgIDs Vynamp.<format> em HKCU/Software/Classes, a entrada em HKCU/Software/RegisteredApplications e 240 MB de dados do usuário em %APPDATA%/Vynamp | o desinstalador NSIS do escopo de usuário executa a remoção e chama PackageManager.RemovePackageAsync para Vynamp.MediaPlayer | remoção limpa |
| `TASK-E16-03-AC-4` | três pacotes de ícones completos para os 17 formatos, cada um com .ico de 16, 24, 32, 48, 64, 128 e 256 px (119 arquivos por família) e a família document marcada como ativa no seletor de Preferências | o usuário seleciona a família signal no seletor de Pacote de Ícones e aciona o botão Aplicar aos arquivos do sistema | suporte às 3 famílias de ícones. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | Fixture PowerShell que grava 5 .flac de 1 s em C:/tmp/e16-03/, invoca o verbo Vynamp > Reproduzir por IShellItemArray com os 5 itens numa ordem diferente da alfabética e, depois do instalador, roda Get-AppxPackage Vynamp.MediaPlayer e lê as chaves HKCU/Software/Classes/Vynamp.* | Hoje o repo vynamp-player tem só src/main.rs com println!("vynamp_player 0.1.0"), zero dependências e nenhum tests/: não existe servidor COM, AppxManifest.xml nem .ico. A evidência exigida é Get-AppxPackage devolvendo 1 pacote com Status=Ok, 17 ProgIDs presentes em HKCU, 119 .ico por família nos 3 pacotes, e a mensagem IPC chegando ao processo primário com 5 caminhos na ordem do IShellItemArray e 1 única instância de vynamp.exe viva |
| Hardware | parcialmente disponível: o Windows 11 x86_64 desta máquina cobre o Explorer moderno e o pacote esparso, mas não há Windows 10 build 19045 para o Explorer clássico nem certificado Authenticode comercial, então a metade Win10 do AC-1 fica sem medição | ausência é limitação registrada, não aprovação |

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
