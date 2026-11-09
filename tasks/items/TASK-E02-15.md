# TASK-E02-15 — Mapear lacunas web para nativo por plataforma

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. Critério de aceite: **PENDENTE** de autoria —
> a coluna `Then` é o requisito aprovado, copiado literal; `Given` e `When` ainda
> não existem.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-15` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-15` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M0 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E02-12`, `TASK-E02-13`, `TASK-E02-14` |
| Bloqueia | `TASK-E02-16` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–4 dias úteis |
| Risco | médio |

## Entrega

Mapa de cada recurso web usado pelo MVP para o equivalente nativo em Windows, Linux, macOS e Android (iOS fora da 1.0), com biblioteca própria planejada onde faltar.

## Boundary

<!-- PENDENTE: o que esta TASK não faz, mesmo parecendo relacionado. -->

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e02-15-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E02-15-AC-1` | o inventário de cada recurso web usado pelo MVP, extraído do pacote no SHA do pin | cada recurso é mapeado para o equivalente nativo em Windows, Linux, macOS e Android e confrontado com o inventário | Zero lacunas sem plano |
| `TASK-E02-15-AC-2` | os recursos que não têm equivalente nativo direto | cada exceção é classificada como API intrínseca do navegador ou lacuna a resolver | exceções só para APIs intrínsecas do navegador com comportamento reimplementado |
| `TASK-E02-15-AC-3` | uma lacuna cuja única saída é uma biblioteca nova | o plano da biblioteca é escrito e o repositório próprio é criado sob jimmykedlyn | toda biblioteca nova tem repositório git próprio e modular. |

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
