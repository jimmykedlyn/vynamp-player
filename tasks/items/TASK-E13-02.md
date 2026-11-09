# TASK-E13-02 — Entregar serviço e MediaSession com engine Rust

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. Critério de aceite: **PENDENTE** de autoria —
> a coluna `Then` é o requisito aprovado, copiado literal; `Given` e `When` ainda
> não existem.

| Campo | Valor |
|---|---|
| ID | `TASK-E13-02` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E13-02` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M1 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E13-01`, `TASK-E01-03` |
| Bloqueia | `TASK-E13-03`, `TASK-E13-04`, `TASK-E13-05` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–8 dias úteis |
| Risco | alto |

## Entrega

Serviço foreground proprietário da reprodução, Player adapter, notificação/controles e bridge lifetime.

## Boundary

<!-- PENDENTE: o que esta TASK não faz, mesmo parecendo relacionado. -->

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e13-02-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E13-02-AC-1` | o app tocando e a Activity destruída por rotação | a reprodução é observada depois do onDestroy da Activity | Activity não é owner do áudio |
| `TASK-E13-02-AC-2` | a MediaSession publicada e a fila em Rust como fonte única de verdade | play, pause e next são acionados pelo lockscreen e pelo headset | lockscreen/play/pause usam fila única Rust |
| `TASK-E13-02-AC-3` | o serviço em execução com o engine aberto | o onDestroy do serviço roda duas vezes seguidas | cleanup idempotente |
| `TASK-E13-02-AC-4` | o target SDK do app e o serviço foreground declarado no manifesto | o serviço é iniciado a partir do background e a partir do foreground | serviço segue restrições do SDK. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | PENDENTE | comparação numérica, não "parece certo" |
| Hardware | PENDENTE | ausência é limitação registrada, não aprovação |

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
