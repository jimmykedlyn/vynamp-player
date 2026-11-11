# TASK-E02-16 — Implementar harness de fidelidade com egui_kittest

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-16` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-16` |
| Critérios da spec | `specs/ui.md` §5 (rascunho, não aprovado) |
| Fase | M1 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E02-15`, `TASK-E02-01` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 4–8 dias úteis |
| Risco | médio |

## Entrega

Crate de testes que renderiza telas nativas e as compara com os gabaritos visual, de movimento e de comportamento do MVP, executada localmente.

## Boundary

- **Não** roda no CI Gate. O harness depende de GPU e de captura, e o CI o marca
  como não executado — `quality/01-estrategia-testes.md` pela QA-034 já registra isso.
- **Não** aprova a fidelidade sozinho: compara com o gabarito e produz a diferença;
  a decisão sobre tolerância é do mantenedor, e a tolerância observada é registrada em
  `TASK-E02-13`.
- **Não** substitui a verificação em aparelho: compara render contra gabarito, não
  áudio nem comportamento em hardware real — isso é `TASK-E14-04`, que roda o corpus em
  cada alvo.
- **Não** cobre iOS, cuja UI é SwiftUI e não passa por este harness — o shell SwiftUI é
  `TASK-E20-03`.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e02-16-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E02-16-AC-1` | o harness com os gabaritos de E02-12, E02-13 e E02-14 e uma tela com um controle deslocado além da tolerância do gabarito | o teste roda e a tela é comparada com o gabarito | O harness falha quando uma tela ou estado excede a tolerância e aponta o elemento divergente |
| `TASK-E02-16-AC-2` | o clone local de vynamp-mvp-reference no SHA 0d9860b901510021bcca4c31a25014373ea7c452 apontado pela configuração do harness | o mantenedor executa o harness com cargo test no crate de testes, na própria máquina | roda apenas localmente, na máquina do mantenedor, com o clone de vynamp-mvp-reference no SHA 0d9860b901510021bcca4c31a25014373ea7c452 |
| `TASK-E02-16-AC-3` | o CI Gate rodando sem o clone do pacote de referência e sem GPU | os testes do harness são coletados na matriz de CI | o CI Gate o marca como não executado. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test -p vynamp-player-harness -- --ignored` localmente, comparando cada tela com os gabaritos de `TASK-E02-12`, `E02-13` e `E02-14` | cada tela tem diferença calculada e a tolerância declarada; o CI Gate registra o harness como não executado em vez de verde. Hoje o crate não existe |
| Hardware | uma GPU capaz de rodar o render de referência; a decisão de rodar localmente existe justamente porque o runner do CI não a tem | ausência é limitação registrada, não aprovação |

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
