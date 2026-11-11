# TASK-E07-04 — Implementar lifecycle e circuit breaker

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E07-04` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E07-04` |
| Critérios da spec | `specs/plugins.md` §5 (rascunho, não aprovado) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E07-03` |
| Bloqueia | `TASK-E07-06`, `TASK-E07-07` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Init/start/event/stop/unload, quotas, timeouts e estado do plugin.

## Boundary

- **Não** prova o mecanismo de interrupcao. Fuel, epoch, limite de memoria e o modulo que estoura sao `TASK-E07-01`; o circuit breaker desta TASK conta falhas de um host cujo limite o spike ainda nao mediu.
- **Não** decide nem aplica permissao. Grant, negar, revogar e revalidacao de redirect sao `TASK-E07-03`; aqui so se trata a chamada que ja passou pelo broker.
- **Não** e a tela de diagnostico nem o safe mode. Instalar, desabilitar, revogar, rollback e a acao segura com ID de diagnostico sao `TASK-E07-07`, e mostrar o plugin isolado e dele.
- **Não** e a maquina de estados do playback. Play, pause, seek, stop, geracao e cancelamento sao `TASK-E01-03`; esta TASK apenas prova que o audio local nao para quando o plugin falha.
- **Não** cobre crash do helper nativo. Processo separado, RPC autenticado e handles minimos sao `TASK-E07-05`; aqui o plugin e modulo Wasm no mesmo processo e o isolamento e logico, nao de sistema operacional.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e07-04-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E07-04-AC-1` | dois plugins de teste com relogio do host injetado, um cujo invoke entra em trap 3 vezes dentro de 60 s e outro que falha 2 vezes em 90 s | o host executa as chamadas dos dois plugins sobre o relogio injetado, avanca esse relogio e marca cada falha como ocorrida e consulta o estado de cada plugin depois de cada falha e ao fim da janela de 90 s | Três falhas em 60 s isolam plugin |
| `TASK-E07-04-AC-2` | um plugin em estado Started com 64 MiB de memoria Wasm reservada, 2 handles de arquivo concedidos e 1 timer registrado, e em outra rodada o mesmo plugin ja saiu por trap | o host recebe stop e unload, repete cada um tres vezes e repete de novo depois do trap | cleanup é idempotente |
| `TASK-E07-04-AC-3` | um plugin com os budgets da spec, 64 MiB de memoria, 10 chamadas concorrentes, payload de 1 MiB e taxa de 100 eventos por segundo | o gestor entrega 1 000 eventos em um segundo com payload de 1 MiB, dispara 11 chamadas concorrentes e, em seguida, uma chamada com payload de 2 MiB | floods são limitados |
| `TASK-E07-04-AC-4` | o null host de vynamp-output renderizando um tom deterministico de crates/vynamp-output-testkit, com um plugin que falha pela terceira vez no meio de um evento | o driver do null host roda o tom a partir do mesmo instante, a terceira falha acontece no meio de um evento e o render prossegue por mais 10 s de relogio do device, com o contador de frames lido ao fim de cada segundo | playback local continua. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --test plugin_lifecycle` em apps/player com relogio falso injetado, sem tokio real, e o null host de libs/output como fonte de frames; as fixtures sao um modulo que da trap, um que tenta alocar 128 MiB e um driver que emite 1 000 eventos em um segundo | o terceiro trap dentro de 60 s leva o plugin a Isolated e as chamadas seguintes saem com codigo tipado sem entrar no convidado, com o contador de entradas em 0; o plugin com 2 falhas em 90 s continua Started porque a primeira saiu da janela; stop e unload repetidos 3 vezes, e repetidos de novo depois do trap, deixam memoria reservada 0, handles 0 e timers 0 desde a primeira chamada, sem panic e sem liberacao dupla; de 1 000 eventos em um segundo exatamente 100 sao admitidos e 900 recusados, a 11a chamada concorrente falha por limite e o payload de 2 MiB e recusado pelo teto de 1 MiB; o null host segue renderizando e o contador de frames nao tem buraco na janela em que o plugin isolou. Hoje nada disso tem alvo: apps/player tem so src/main.rs com um println!, vynamp-engine tem so src/lib.rs, e o unico pedaco real de playback e o null host em libs/output/crates/vynamp-output-core/src/null.rs, com tests/null_host.rs ja existente. |
| Hardware | nenhum: falhas, quotas, cleanup e a continuidade do playback sao medidos com relogio falso e o null host de vynamp-output, que nao abre dispositivo de audio, nao depende de DAC e nao toca placa profissional | ausência é limitação registrada, não aprovação |

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
