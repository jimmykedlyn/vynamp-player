# TASK-E16-08 — Validar portable, backup e limpeza de integração

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E16-08` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E16-08` |
| Critérios da spec | `specs/player.md` §5 (rascunho, não aprovado) |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E16-02`, `TASK-E15-06`, `TASK-E15-07`, `TASK-E15-08` |
| Bloqueia | `TASK-E14-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Backup/restore consistente, portable relocável e uninstall de registros próprios (ProgIDs e pacote esparso).

## Boundary

- **Não** reimplementa o lock interprocesso nem a eleição de owner que o restore e o update consultam: o lock de SO e o encaminhamento por IPC são TASK-E16-02, e duplicar a trava aqui criaria dois donos do mesmo perfil de escrita.
- **Não** entrega o dump consistente do banco da biblioteca nem o checkpoint de WAL: esse mecanismo é TASK-E03-01, e esta TASK orquestra staging e rollback sobre o arquivo que ele produz, não define o formato dele.
- **Não** cria ProgID, submenu do Explorer nem pacotes de ícones: HKCU/Software/Classes, o pacote esparso e o servidor IExplorerCommand são TASK-E16-03, e o uninstall daqui só pode remover o que aquela registrou.
- **Não** assina o NSIS nem o ZIP portable nem adquire certificado Authenticode comercial: a assinatura dos artefatos Windows é TASK-E15-06, e um uninstall validado sobre binário não assinado não prova o canal de distribuição.
- **Não** gera AppxManifest.xml nem as declarações OS a partir do build: a geração é TASK-E16-09, e aqui se verifica a remoção do pacote já gerado e assinado, não a composição dele.
- **Não** implementa o canal de atualização, a verificação de assinatura do pacote nem o rollout: o updater assinado é TASK-E11-06, e o AC-4 prova apenas que o update espera o owner, não que ele exista.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e16-08-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E16-08-AC-1` | um perfil instalado em %APPDATA%/Vynamp com 3 GB de mídia em 12.480 arquivos .flac e .wav, um backup completo dessa mídia com manifest SHA-256 por arquivo e journal de staging vazio, e 6 GB livres no volume de destino, o dobro do perfil | o restore desse perfil é iniciado, o processo é encerrado por TerminateProcess durante a cópia para o staging, e a mesma invocação é repetida duas vezes seguidas sobre o mesmo perfil | Restore por staging e rollback preserva mídia |
| `TASK-E16-08-AC-2` | Windows 11 x86_64 com Vynamp 1.0.0 instalado, 17 ProgIDs Vynamp.* com 3 chaves cada sob HKCU/Software/Classes, 51 chaves ao todo, o pacote esparso vynamp-player_1.0.0.0_x64.msix registrado com Status=Ok e 12.480 arquivos de mídia do usuário fora do diretório de instalação | o desinstalador é acionado em Configurações > Aplicativos com a caixa de remover dados do usuário marcada e o owner encerra ao responder o diálogo de perfil em uso | desinstalação remove pacote esparso e HKCU sem sobras |
| `TASK-E16-08-AC-3` | diretório portable em D:/Vynamp-portable com vynamp_player.exe e o perfil em D:/Vynamp-portable/profile, 0 chaves Vynamp em HKCU/Software/Classes, 0 entradas de startup e a integração desativada nessa máquina | o usuário abre Portable > Integração, aciona Ativar, lê o resumo com 17 ProgIDs, 51 chaves e 1 pacote esparso e responde Não, repete a ação respondendo Sim e depois aciona Desativar | portable permite ativar/desativar integração com confirmação |
| `TASK-E16-08-AC-4` | duas sessões na mesma máquina, a sessão owner segurando o lock do perfil P1 com playback de FLAC 44,1 kHz em curso, e o pacote 1.0.1 baixado com hash conferido pela segunda instância | a segunda instância inicia o update, o diálogo lista a sessão owner e o usuário escolhe Aguardar em vez de Forçar, e em seguida o owner encerra o playback e responde ao ACK | update respeita owner. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | script novo tests/e16_08_restore.rs, que não existe hoje, gerando o perfil de 12.480 arquivos em tests/fixtures/perfil-3gb a partir de um seed fixo e comparando, antes e depois do uninstall, `MSYS_NO_PATHCONV=1 reg export HKCU\Software\Classes`, `Get-AppxPackage -Name vynamp-player` e o SHA-256 dos 12.480 arquivos; roda como `cargo test --test e16_08_restore -- --nocapture` | esperado 12.480 de 12.480 arquivos com SHA-256 igual ao manifest após o TerminateProcess e o rollback, 0 bytes de mídia removidos, 1 rollback completo no journal, 17 ProgIDs e 51 chaves removidos com 0 restantes, 0 pacotes esparsos no Get-AppxPackage e os 12.480 arquivos do usuário intactos; hoje o repo vynamp-player tem 1 arquivo .rs, src/main.rs, com println!("vynamp_player 0.1.0"), zero dependências e nenhum tests/, e `cargo test` mede 0 testes, então nenhum desses comandos roda e o alvo e16_08_restore responde 'no test target named'. `Get-AppxPackage -Name '*vynamp*'` devolve 0 pacotes; HKCU/Software/Classes tem 965 chaves e 0 delas casam com vynamp; a chave AppModelUnlock existe com 0 valores, ou seja Developer Mode está desligado, o que bloqueia o registro solto por Add-AppxPackage -Register e deixa o AC-2 dependente de um .msix assinado, que também não existe no repo |
| Hardware | parcial: o Windows 11 Pro build 26300 x86_64 desta máquina cobre escrita e leitura em HKCU (confirmadas com reg add seguido de reg delete), reg export, e o signtool e makeappx do Windows Kits 10.0.19041 em C:/Program Files (x86)/Windows Kits/10/bin/10.0.19041.0/x64, e New-SelfSignedCertificate existe; mas Developer Mode está desligado, porque a chave AppModelUnlock tem 0 valores e nenhum AllowDevelopmentWithoutDevLicense, então o pacote esparso do AC-2 não é instalável sem habilitar o modo e assinar o .msix; não há volume removível, só C: e D:, ambos drive type 3 Fixed, então o AC-3 roda em diretório no D: e a relocação entre volumes substitui o pendrive; não há dispositivo de áudio pro, máquina macOS nem aparelho Android, e esta TASK não abre dispositivo de áudio | ausência é limitação registrada, não aprovação |

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
