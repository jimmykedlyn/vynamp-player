# TASK-E16-02 — Implementar owner único por perfil e IPC autorizado

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E16-02` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E16-02` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M1 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E16-01`, `TASK-E01-03` |
| Bloqueia | `TASK-E16-03`, `TASK-E16-04`, `TASK-E16-05`, `TASK-E16-08` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Lock OS, encaminhamento IPC, idempotência e isolamento de perfis.

## Boundary

- **Não** implementa o protocolo: `OpenRequest` é `TASK-E16-01`.
- **Não** decide política de perfis além do owner único: quantos perfis existem e como
  são nomeados é do mantenedor.
- **Não** trata sync entre máquinas: `TASK-E09-08`.
- **Não** promete lock entre sistemas de arquivos: o lock é de OS, e o que não for
  atômico no alvo fica nomeado como limitação.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e16-02-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E16-02-AC-1` | o app fechado e vinte invocações simultâneas do entrypoint | cada instância tenta tomar o lock de perfil | Vinte launches elegem um owner |
| `TASK-E16-02-AC-2` | a versão instalada e a versão portátil apontando para o mesmo diretório de perfil | as duas tentam abrir o banco de biblioteca ao mesmo tempo | installed/portable não escrevem o mesmo perfil simultaneamente |
| `TASK-E16-02-AC-3` | um arquivo de lock com PID de processo já morto e outro cujo PID foi reutilizado por processo alheio | a elegibilidade de owner é avaliada | stale PID não libera lock |
| `TASK-E16-02-AC-4` | um owner já eleito e um segundo pedido de abertura | o encaminhamento por IPC responde ao pedido | ACK e Conflict testados. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test` no repo do player com vinte processos disputando o mesmo perfil e um lock stale de PID morto | um owner é eleito e o PID stale não autoriza reset; hoje o lock não existe e o crate não tem teste |
| Hardware | nenhum para o lock; o teste é de processo concorrente na mesma máquina | ausência é limitação registrada, não aprovação |

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
