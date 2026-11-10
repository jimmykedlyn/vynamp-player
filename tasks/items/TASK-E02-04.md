# TASK-E02-04 — Entregar playlist UI e keyboard workflow

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-04` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-04` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M1 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E02-03`, `TASK-E03-02` |
| Bloqueia | `TASK-E02-05` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–8 dias úteis |
| Risco | médio |

## Entrega

Lista virtualizada, drag insert, seleção, context menu e queue actions.

## Boundary

- **Não** constrói a fila. `TASK-E03-02` é dona do snapshot, revisões e seed; aqui
  só se exibe e se emite intenção.
- **Não** implementa drag entre janelas nem entre instâncias.
- **Não** faz edição de tags: a lista mostra o que a biblioteca já leu.
- **Não** decide atalhos novos. O mapa de teclado vem de `TASK-E02-14`.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e02-04-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E02-04-AC-1` | uma playlist de dez faixas e a fila acompanhada por snapshot | a mesma faixa é acionada por tocar agora, tocar em seguida e adicionar ao fim | Tocar/agendar/adicionar têm comportamentos distintos |
| `TASK-E02-04-AC-2` | a lista com o foco do teclado e uma faixa tocando | a navegação por teclado move a seleção sem tocar e as ações de fila são disparadas por atalho | teclado completo e faixa atual separada da seleção. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test` sobre a lista virtualizada com 100 mil linhas sintéticas e um roteiro de teclado ponta a ponta | o número de linhas materializadas fica limitado ao viewport e o roteiro passa sem mouse; hoje não há lista nem teste no crate |
| Hardware | nenhum: a lista é determinística e a virtualização é medida por contagem de nós | ausência é limitação registrada, não aprovação |

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
