# TASK-E07-07 — Entregar gerenciador de plugins

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E07-07` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E07-07` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E07-04`, `TASK-E07-05`, `TASK-E02-06` |
| Bloqueia | `TASK-E02-10`, `TASK-E07-08` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Interface de instalação, atualização, disable, revoke, rollback e safe mode.

## Boundary

- **Não** valida assinatura nem hash do pacote .vynplug e não implementa as ferramentas pack, sign e validate. Isso é TASK-E07-08; a confusão é plausível porque instalar e atualizar exigem saber se o pacote é íntegro, e o gerenciador é quem mostra o erro da validação alheia.
- **Não** implementa a revogação publicada pelo registry nem a imutabilidade de versão. Isso é TASK-E07-09; aqui revoke é o estado local de desativar um plugin já instalado, e quem executa tende a tratar as duas revogações como uma só.
- **Não** implementa os budgets, timeouts, circuit breaker e a máquina de estados init/start/event/stop/unload. Isso é TASK-E07-04; o gerenciador exibe esses estados e por isso parece dono deles.
- **Não** implementa o sandbox de sistema operacional nem o consentimento do helper nativo, com RPC autenticado e memória compartilhada limitada. Isso é TASK-E07-05; instalar um plugin nativo passa pela mesma tela, mas o mecanismo de isolamento não é desta TASK.
- **Não** entrega as preferências do player nem os estados de erro gerais da interface. Isso é TASK-E02-06; a tela de falha do plugin fica colada às demais telas de erro, e a fronteira entre o erro do gerenciador e o erro do app precisa ficar explícita.
- **Não** implementa a revogação de grants de permissão concedidos ao usuário. Isso é TASK-E07-03; revogar o plugin inteiro e revogar um grant parecem o mesmo gesto na lista, e misturá-los apagaria a rastreabilidade de quem concedeu o quê.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e07-07-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E07-07-AC-1` | plugin br.mock.radio 1.0.0 instalado com permissions contendo apenas network.domains mock.local, e o pacote 1.1.0 assinado no mesmo publisher declarando adicionalmente library.read no manifest | o usuário arrasta o arquivo br.mock.radio-1.1.0.vynplug para a tela do gerenciador e aciona Atualizar sem marcar a confirmação da nova permissão | Novas permissões exigem intenção |
| `TASK-E07-07-AC-2` | plugin br.mock.radio 1.1.0 instalado e rodado uma vez, com a migration de estado v1 para v2 aplicada em namespace isolado e 40 registros persistidos; a versão 1.0.0 instalada anteriormente só conhece o schema v1 | o usuário escolhe Rollback para 1.0.0 no gerenciador, confirma, e o player é reaberto carregando o plugin revertido | rollback restaura estado compatível |
| `TASK-E07-07-AC-3` | plugin visualizer br.mock.viz 1.0.0 que entra em trap de WASM durante o playback de um FLAC local de 3 s, com o gerenciador aberto na lista de plugins | o evento de falha do plugin chega ao gerenciador e o usuário clica na linha do plugin com falha | falha mostra ação segura e ID de diagnóstico. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test -p vynamp-plugin-manager --test manager_flows com os fixtures tests/fixtures/plugins/radio-1.0.0.vynplug, radio-1.1.0.vynplug (com library.read a mais) e radio-1.1.0-trap.vynplug, mais um banco de estado de fixture com 40 registros já migrados para v2 | evidência esperada: o log do fluxo registra 1 bloqueio por nova permissão sem confirmação e 0 instalações, o rollback deixa o estado em schema v1 com 40 registros preservados e migration v2 marcada como revertida, e o clique na linha do plugin com falha devolve exatamente 1 ação segura nomeada mais 1 diagnostic id não vazio; hoje nada disso existe no repo vynamp-player, que tem só 1 arquivo .rs (src/main.rs) com println!("vynamp_player 0.1.0"), zero dependências e nenhum tests/ |
| Hardware | nenhum: o gerenciador é interface mais filesystem de fixture e o playback que gera a falha é disparado por clock falso, sem abrir dispositivo de saída de áudio | ausência é limitação registrada, não aprovação |

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
