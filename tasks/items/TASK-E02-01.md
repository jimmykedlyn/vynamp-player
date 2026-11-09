# TASK-E02-01 — Validar toolkit Rust com UI clássica acessível

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. Critério de aceite: **PENDENTE** de autoria —
> a coluna `Then` é o requisito aprovado, copiado literal; `Given` e `When` ainda
> não existem.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-01` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-01` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M0 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E11-01` |
| Bloqueia | `TASK-E02-02`, `TASK-E02-16` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Spike desktop egui/AccessKit em Windows/Linux/macOS Intel+Silicon, com Classic e ADR-003.

## Boundary

<!-- PENDENTE: o que esta TASK não faz, mesmo parecendo relacionado. -->

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e02-01-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E02-01-AC-1` | o spike rodando em Windows com Narrator e NVDA, em Linux com Orca e em macOS com VoiceOver | um roteiro só de teclado e leitor de tela executa abrir, play, pause, stop, próxima faixa e seek | Narrator/NVDA/Orca/VoiceOver workflow de transporte funciona |
| `TASK-E02-01-AC-2` | a janela Classic Stack em 480x660 com o texto do sistema em 200% | a árvore de acessibilidade é percorrida e cada controle de transporte é alcançado e acionado | DPI/zoom200% sem esconder ações |
| `TASK-E02-01-AC-3` | o relatório do spike com qualquer AC-1 ou AC-2 reprovado em alguma plataforma | a decisão de toolkit é registrada no ADR-003 com o motivo e o contexto medido | falhas mudam toolkit com ADR. Android tem gate E13-01/05. |

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
