# TASK-E02-15 — Mapear lacunas web para nativo por plataforma

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

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

- **Não** implementa as bibliotecas que o mapa aponta como faltantes. Decide o que
  precisa existir e onde; a construção é `TASK-E02-16` em diante ou repo próprio.
- **Não** inclui iOS no mapa da 1.0. A coluna iOS entra com a FASE 6, e a
  `platforms/05-ios.md` §1.2 já fixa que não há conversor nem restaurador no build iOS.
- **Não** promete paridade onde a API não existe: a lacuna fica nomeada, não coberta
  por um equivalente aproximado.
- **Não** substitui a ADR de cada biblioteca nova.

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
| Oráculo independente | leitura do mapa contra a lista de APIs do MVP, item a item, com a coluna de equivalente nativo preenchida ou a lacuna declarada | nenhum recurso do MVP fica sem linha, e nenhuma linha afirma equivalente que o alvo não tem; hoje o mapa não existe no repo |
| Hardware | nenhum: o mapa é documental | ausência é limitação registrada, não aprovação |

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
