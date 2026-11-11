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
| Critérios da spec | `specs/player.md` §5 (rascunho, não aprovado) |
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

- **Não** gera as declarações de tipo a partir das capabilities do build verificado. O dono é TASK-E16-09; o AppxManifest.xml desta TASK consome a lista de 17 formatos como entrada, e derivá-la aqui criaria duas fontes para a mesma lista.
- **Não** implementa o canal IPC nem a eleição do processo primário único. O dono é TASK-E16-02; o método Invoke só transmite o lote pelo canal que já existe, e quem executar pode achar que tratar IShellItemArray inclui o protocolo de entrega.
- **Não** adquire certificado Authenticode comercial nem valida a assinatura do instalador. O dono é TASK-E15-06; aqui só se instala o autoassinado de desenvolvimento em Cert:/CurrentUser/Root, e a spec mantém o certificado público como decisão aberta de infraestrutura.
- **Não** cobre modo portátil, backup de associações nem limpeza seletiva de integração. O dono é TASK-E16-08; o desinstalador daqui remove só o que instalou e não audita integração alheia.
- **Não** trata MIME, ServiceMenus nem hierarquia hicolor do Linux. O dono é TASK-E16-04; a confusão é forte porque as três famílias de ícones são compartilhadas e a mesma família vira .ico no Windows e PNG no Linux.
- **Não** executa a certificação cruzada de ativação nos cinco alvos. O dono é TASK-E16-07; esta TASK entrega o mecanismo do Windows, não o relatório de aceite por alvo.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e16-03-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E16-03-AC-1` | Windows 11 build 22631 com o pacote esparso Vynamp.MediaPlayer registrado por PackageManager e o submenu desktop4:FileExplorerContextMenus ativo, e um Windows 10 build 19045 com os 17 ProgIDs Vynamp.<format> em HKCU/Software/Classes, a chave shell/Vynamp com MUIVerb=Vynamp e SubCommands e MultiSelectModel=Player, cada máquina com 9 .flac de 44,1 kHz em 16 bits numa pasta local | o usuário seleciona os 9 arquivos com Ctrl em ordem invertida ao alfabeto e aciona Vynamp > Reproduzir no menu de contexto, primeiro no Explorer moderno do Windows 11 e depois no clássico do Windows 10 | Explorer moderno (Win11) e clássico (Win10) abrem lote ordenado por IShellItemArray |
| `TASK-E16-03-AC-2` | NSIS per-user já instalado em %LOCALAPPDATA%/Programs/Vynamp, AppxManifest.xml com Identity Vynamp.MediaPlayer, MinVersion 10.0.22000.0 e as extensões windows.fileTypeAssociation e desktop4:fileExplorerContextMenus, certificado autoassinado em Cert:/CurrentUser/Root e Get-AppxPackage Vynamp.MediaPlayer devolvendo 0 pacotes, como devolve hoje nesta máquina | o instalador chama PackageManager.AddPackageByUriAsync sobre o .msix esparso e aguarda o fim da IAsyncOperation sem reiniciar a máquina | pacote esparso registrado via PackageManager |
| `TASK-E16-03-AC-3` | instalação completa com o pacote esparso registrado, 17 ProgIDs Vynamp.<format> em HKCU/Software/Classes, entrada em HKCU/Software/RegisteredApplications e 240 MB de dados do usuário em %APPDATA%/Vynamp | o desinstalador NSIS do escopo de usuário executa a remoção e chama PackageManager.RemovePackageAsync para Vynamp.MediaPlayer | remoção limpa |
| `TASK-E16-03-AC-4` | três pacotes de ícones completos para os 17 formatos, cada .ico com 16, 24, 32, 48, 64, 128 e 256 px (119 arquivos por família), família document ativa no seletor de Preferências e DefaultIcon dos 17 ProgIDs ainda apontando para o .ico dela | o usuário seleciona a família signal no seletor de Pacote de Ícones e aciona o botão Aplicar aos arquivos do sistema, que reescreve DefaultIcon dos 17 ProgIDs | suporte às 3 famílias de ícones. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | Fixture PowerShell nova em scripts/e16-03-shell.ps1, que não existe hoje, mais uma biblioteca COM nova: apps/player tem só src/main.rs com println!("vynamp_player 0.1.0"), zero dependências e nenhum tests/, e nem o repo nem o umbrella têm AppxManifest.xml, servidor IExplorerCommand ou um único .ico — packaging/icons/README.md diz que os exports ainda não existem e packaging/windows/installer.nsi só grava as chaves de Uninstall. A fixture gera 9 .flac de 1 s, assina o .msix com signtool, registra com Add-AppxPackage -Register, chama IExplorerCommand::Invoke com um IShellItemArray na ordem invertida ao alfabeto e lê Get-AppxPackage, HKCU/Software/Classes/Vynamp.*, DefaultIcon e Get-Process vynamp. | A evidência exigida é Get-AppxPackage devolvendo 1 pacote com Status=Ok, 17 ProgIDs presentes antes da desinstalação e 0 depois, 119 .ico em cada uma das 3 famílias, DefaultIcon dos 17 trocado para a família signal, e 1 única instância de vynamp.exe recebendo os 9 caminhos na ordem do IShellItemArray e nenhum faltando; os 240 MB de %APPDATA%/Vynamp continuam íntegros depois da remoção. Hoje nenhum desses números é produzível: faltam o servidor COM, o manifesto, o certificado e os ícones, e a linha de Teste automatizado do arquivo promete cargo test como prova, o que um repositório sem tests/ não pode entregar: o binário só imprime vynamp_player 0.1.0 e sai. |
| Hardware | parcialmente disponível: o Windows 11 Pro build 26300 x86_64 desta máquina, mais novo que o 22631 do AC-1, cobre o Explorer moderno, o pacote esparso, o registro HKCU e o signtool do Windows Kits 10.0.19041, e Cert:/CurrentUser/My hoje tem 0 certificados de assinatura, de modo que o autoassinado precisa ser criado; não há Windows 10 build 19045 para o Explorer clássico nem certificado Authenticode comercial, então a metade Win10 do AC-1 e a assinatura de distribuição ficam sem medição | ausência é limitação registrada, não aprovação |

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
