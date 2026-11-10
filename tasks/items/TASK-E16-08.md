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
| Critérios da spec | pendente — `specs/player.md` §5 |
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

- **Não** implementa o lock interprocesso nem a eleição de owner que o restore e o update consultam; essa regra é de TASK-E16-02, e reimplementá-la aqui criaria dois donos da mesma trava de escrita sobre o perfil.
- **Não** cria a associação ProgID, o menu de contexto nem os pacotes de ícones do Windows; isso é TASK-E16-03, e o uninstall desta TASK só pode remover o que aquela registrou, nunca definir o que é registrado.
- **Não** assina o instalador NSIS nem define o layout do pacote portable; assinatura e redistribuíveis são de TASK-E15-06, e um uninstall validado sobre binário não assinado não prova o canal real.
- **Não** gera as declarações OS do pacote esparso e do AppxManifest a partir do build; a geração é de TASK-E16-09, aqui se verifica apenas a remoção do pacote já gerado e assinado.
- **Não** fecha o release 1.0 multiplataforma nem o checklist de artefatos; isso é TASK-E14-06, que esta TASK apenas desbloqueia com evidência de portable, backup e limpeza.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e16-08-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E16-08-AC-1` | um perfil instalado com 3 GB de mídia, 12.480 arquivos FLAC e WAV, e um backup completo v3 gerado sob owner, com manifest SHA-256 por arquivo e journal de staging vazio | o restore é iniciado e o processo recebe kill -9 durante a cópia para o staging, e logo depois o restore é reinvocado sobre o mesmo perfil | Restore por staging e rollback preserva mídia |
| `TASK-E16-08-AC-2` | Windows 11 x86_64 com Vynamp 1.0.0 instalado, pacote esparso vynamp-player_1.0.0.0_x64.msix registrado, 43 chaves em HKCU/Software/Classes para os ProgIDs Vynamp.* e 12.480 arquivos de mídia do usuário fora do diretório de instalação | o desinstalador é executado a partir de Configurações > Aplicativos com a opção de remover dados do usuário marcada, e o owner é encerrado por ACK do diálogo quando solicitado | desinstalação remove pacote esparso e HKCU sem sobras |
| `TASK-E16-08-AC-3` | diretório portable E:/Vynamp com vynamp_player.exe e perfil em E:/Vynamp/profile, nenhum valor em HKCU/Software/Classes, nenhuma entrada de startup e a integração desativada nessa máquina | o usuário abre Portable > Integração, aciona Ativar, lê o resumo com 43 chaves e 1 pacote esparso e responde Não, repete a ação respondendo Sim e em seguida aciona Desativar | portable permite ativar/desativar integração com confirmação |
| `TASK-E16-08-AC-4` | duas sessões na mesma máquina, com o owner segurando o lock do perfil P1 e playback FLAC 44,1 kHz em curso, e o updater 1.0.0 para 1.0.1 baixado e com hash conferido pela segunda instância | a segunda instância inicia o update, o diálogo lista a sessão owner e o usuário escolhe Aguardar em vez de Forçar, e o owner fecha devolvendo ACK | update respeita owner. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | teste em tests/e16_08_restore.rs alimentado pela fixture tests/fixtures/perfil-3gb com 12.480 arquivos e manifest SHA-256, mais um snapshot de HKCU/Software/Classes e da lista de pacotes esparsos antes e depois do uninstall, rodando como cargo test --test e16_08_restore -- --nocapture | esperado 12.480 de 12.480 arquivos com SHA-256 igual ao manifest, 0 bytes de mídia removidos após o kill, 1 rollback completo registrado no journal, 43 de 43 chaves HKCU removidas e 0 pacotes esparsos restando; hoje o repo vynamp-player tem apenas src/main.rs com um println da versão 0.1.0, zero dependências e nenhum diretório tests/, então nenhum desses testes existe e o comando ainda não roda |
| Hardware | nenhum: restore, portable e uninstall operam sobre diretório e sobre o registro HKCU do Windows 11 x86_64 disponível aqui, sem abrir dispositivo de áudio, sem aparelho Android, iOS ou máquina macOS | ausência é limitação registrada, não aprovação |

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
