# TASK-E02-03 — Construir Classic Stack e transporte

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-03` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-03` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M1 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E02-02`, `TASK-E01-03` |
| Bloqueia | `TASK-E02-04`, `TASK-E06-05` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–8 dias úteis |
| Risco | médio |

## Entrega

Painéis clássicos nas geometrias da spec e ligação de commands/events.

## Boundary

- **Não** implementa a playlist. A lista virtualizada é `TASK-E02-04`, e trazê-la
  para cá faria dois donos do mesmo painel.
- **Não** inventa geometria. As medidas vêm de `design/02-layouts.md` e do dump de
  geometria de `TASK-E02-12`.
- **Não** liga o áudio: `commands/events` são o contrato, e o engine é `TASK-E01-03`.
- **Não** cobre dock nem multi-monitor: `TASK-E02-08`.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e02-03-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E02-03-AC-1` | o Classic Stack com uma faixa aberta e o engine ligado por commands e events | cada ação do transporte é disparada pela UI e o evento correspondente é observado no barramento | Abrir/play/pause/stop/seek/volume/shuffle/repeat operam |
| `TASK-E02-03-AC-2` | a janela redimensionada para 480x660, o mínimo declarado na spec de layouts | cada região do layout é medida e o pipeline de status é lido | resizing mínimo mantém targets e pipeline status. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo xtask check-tokens`-equivalente no repo do player e comparação da geometria renderizada contra o dump de `TASK-E02-12` | cada painel bate largura, altura e espaçamento do gabarito dentro da tolerância declarada; hoje o crate não tem render nem gabarito para comparar |
| Hardware | nenhum para a geometria; a leitura por leitor de tela depende de NVDA/Narrator, ausentes aqui | ausência é limitação registrada, não aprovação |

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
