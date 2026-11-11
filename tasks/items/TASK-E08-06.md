# TASK-E08-06 — Integrar controles do Windows

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E08-06` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E08-06` |
| Critérios da spec | `specs/player.md` §5 (rascunho, não aprovado) |
| Fase | M2 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E02-06`, `TASK-E01-03` |
| Bloqueia | `TASK-E08-07`, `TASK-E16-03` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | médio |

## Entrega

SMTC, media keys, tray/taskbar, hotkeys globais configuráveis e associações consentidas.

## Boundary

- **Não** implementa associações de arquivo, menu de contexto do Explorer nem pacotes de ícones no Windows. O dono é `TASK-E16-03`, que esta TASK bloqueia; o AC-3 fala de associação, então é muito plausível achar que o registro de ProgID e o menu de contexto entram aqui, e trazê-los faria dois donos das mesmas chaves.
- **Não** implementa o owner único por perfil nem o protocolo de IPC autorizado entre instâncias. O dono é `TASK-E16-02`; o AC-1 pressupõe duas instâncias e uma delas dona, e a confusão é plausível porque a eleição de dono é o que garante que o SMTC aponte para a sessão certa.
- **Não** implementa a CLI nem o encaminhamento de comandos da segunda instância para a primeira. O dono é `TASK-E08-07`, também bloqueada por esta TASK; aqui a bandeja e as teclas só enviam comandos ao próprio processo, e a segunda instância chega por outro caminho.
- **Não** implementa as janelas Mini, a tela de preferências nem os estados de erro da interface. O dono é `TASK-E02-06`, dependência declarada; esta TASK entrega o mecanismo de hotkey e tray e o AC-4 é exercitado pelo mapa salvo, não pelo formulário redesenhado.
- **Não** implementa a integração equivalente no Linux (MIME, extensões de shell e abertura externa) nem no macOS. Os donos são `TASK-E16-04` e `TASK-E16-05`; controles do sistema operacional soa multiplataforma, mas SMTC, bandeja do Windows e `RegisterHotKey` são só deste alvo, e o código comum de atalho não deve prometer o que as outras TASKs decidem.
- **Não** implementa deep links nem callbacks de provider que também abrem a instância. O dono é `TASK-E16-10`; a confusão é plausível porque registrar um app para receber ativação externa no Windows parece a mesma coisa que registrar hotkey global.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e08-06-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E08-06-AC-1` | Duas instâncias do binário no mesmo perfil do Windows 11: a instância 1 tocando `fixture-a.flac` e a instância 2, dona do IPC autorizado, tocando `fixture-b.flac`, com duas sessões de SMTC distintas publicadas. | Um `WM_APPCOMMAND` com `APPCOMMAND_MEDIA_PAUSE` é injetado via `SendMessage` na janela da sessão SMTC da instância 2, como o flyout de mídia do Windows faz. | Controlar sessão correta |
| `TASK-E08-06-AC-2` | Preferência de ação do botão fechar definida explicitamente como minimizar para a bandeja, valor diferente do padrão de fábrica, com a janela principal visível e uma faixa tocando e o contador de reprodução avançando. | O usuário clica no X da janela principal e depois aciona o ícone da bandeja uma vez. | close-to-tray explícito |
| `TASK-E08-06-AC-3` | App rodando como usuário padrão no Windows 11, sem elevação e sem `runas`, com a opção de associação de arquivos nunca aceita e um snapshot da árvore `HKCU\Software\Classes` tirado antes do primeiro start. | O app inicia, registra o SMTC e o ícone da bandeja, e o usuário abre a aba de associações sem confirmar nada. | sem admin/associação involuntária |
| `TASK-E08-06-AC-4` | Um processo auxiliar já ocupa a hotkey global `Ctrl+Alt+V` por `RegisterHotKey` nesta sessão, e o mapa do app tem `Ctrl+Alt+V` para play e `Ctrl+Alt+Space` para próxima faixa, com `Ctrl+Alt+Space` livre. | O usuário salva o mapa de hotkeys globais pela tela de preferências. | detectar conflitos de hotkeys. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --test windows_shell_integration` no repo `vynamp-player`, rodando nesta máquina Windows 11 x86_64 com duas instâncias do binário, um processo auxiliar que segura `Ctrl+Alt+V` por `RegisterHotKey` e injeção de `WM_APPCOMMAND` com `APPCOMMAND_MEDIA_PAUSE` via `SendMessage` na janela da instância 2; conferir depois com `reg query HKCU\Software\Classes` contra o snapshot tirado antes do start e com `EnumWindows` para o estado das janelas do processo. | 1 comando de pause entregue à instância 2 e 0 à instância 1, com a faixa da instância 1 continuando sem troca; 1 janela escondida após o X com o processo vivo e a reprodução em andamento, e 1 restauração da janela pelo ícone da bandeja; 0 chaves de associação escritas em `HKCU\Software\Classes` (diff vazio contra o snapshot) e 0 pedido de elevação no primeiro start; 1 conflito listado para `Ctrl+Alt+V` e 1 hotkey efetivamente registrada para `Ctrl+Alt+Space`. Hoje nada disso roda: o repo `vynamp-player` tem 1 arquivo `.rs` (`src/main.rs`) com um `println!` de `vynamp_player 0.1.0`, zero dependências e nenhum `tests/`, então não há janela, bandeja, sessão de SMTC nem mapa de hotkey para exercitar. |
| Hardware | Windows 11 x86_64 desta máquina serve para os 4 critérios (User32, shell e SMTC do SO disponíveis), mas parcial: não há teclado físico com teclas de mídia dedicadas e o AC-1 usa duas instâncias no mesmo host em vez de dois aparelhos, então media keys são exercitadas por injeção de `WM_APPCOMMAND` via `SendMessage`, que é o mesmo caminho que o flyout usa. | ausência é limitação registrada, não aprovação |

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
