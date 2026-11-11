# TASK-E16-01 — Provar envelope de ativação e protocolo comum

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E16-01` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E16-01` |
| Critérios da spec | `specs/player.md` §5 (rascunho, não aprovado) |
| Fase | M0 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E11-01` |
| Bloqueia | `TASK-E16-02`, `TASK-E16-06`, `TASK-E16-09`, `TASK-E16-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Contrato OpenRequest, limites e spikes de entrega nativa em cinco alvos.

## Boundary

- **Não** implementa `OpenRequest` no produto: fixa o envelope e os limites; quem o
  serve é `TASK-E16-02` e o IPC do engine.
- **Não** configura associações de arquivo: os menus por sistema são `TASK-E16-*`
  seguintes e a spec `platforms/04-ativacao-ownership.md`.
- **Não** inclui iOS: a FASE 6 é dona da ativação iOS, e a matriz desta TASK é a
  dos cinco alvos de 1.0, a de `platforms/01-matriz-suporte.md`, que não conta o
  iOS.
- **Não** trata o handoff de `Windows.UI` nem de `NSDocumentController` em
  detalhe: `platforms/03-integracao-sistema.md` fixa o contrato, e aqui se prova o
  spike de entrega em cada alvo e se registra o que não foi provado.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e16-01-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E16-01-AC-1` | o app fechado (cold) e o app aberto (warm), com lotes de arquivos enviados ao entrypoint | lotes de 1, 2, 20 e 100 OpenRequest são enviados e a ordem de chegada é registrada | Cold/warm e lotes 1/2/20/100 preservam ordem |
| `TASK-E16-01-AC-2` | arquivos cujo nome contém dois hífens, aspas, quebra de linha e espaço à esquerda | o envelope de ativação é montado e o app lê o pedido | nomes especiais são dados |
| `TASK-E16-01-AC-3` | o contrato OpenRequest e os limites de tamanho e de contagem | o contrato é confrontado com os spikes de entrega nativa nos cinco alvos | limites e ACK/generation definidos. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | validação do exemplo contra `schemas/open-request.schema.json` em `cargo test -p xtask`, mais um spike de entrega por alvo dos cinco | o exemplo passa no schema e o limite de 1 MiB e de 8192 bytes de locator é exercitado; os spikes de entrega, porém, não foram executados e o repo registra isso em `90d7bf4` |
| Hardware | os quatro alvos não-Windows: macOS, Linux e um aparelho Android; **nenhum** disponível, e a ausência está declarada no commit do repo | ausência é limitação registrada, não aprovação |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final
## Notas de execução

Medido em 2026-11-09. **Esta TASK não foi executada: não havia o que executar.**

| Medição | Resultado |
|---|---|
| Tamanho do repo | `wc -l src/*.rs` → **3 linha(s)**, contendo só `pub const VERSION` e o comentário de módulo. `src/` tem 1 arquivo(s) |
| Worktree suja? | Não: `git status --short` limpo antes da medição |
| Especificação do módulo | `platforms/04-ativacao-ownership.md` — **especificação proposta, implementação não iniciada** |

O critério de aceite abaixo exige o contrato `OpenRequest` implementado com limites, e cold/warm mais lotes 1/2/20/100 preservando ordem nos cinco alvos. Nenhum dos dois existe neste
repositório: não há código para compilar, medir, interromper ou comparar bit a bit.
Registrar uma nota de execução dizendo que o critério foi verificado seria falsear
a evidência — então a TASK permanece `planned` e esta nota registra o motivo
medido, não um resultado.

**Por que o motivo é o repo vazio e não uma limitação de ambiente:** uma limitação
de ambiente (hardware ausente, linker ausente) aparece como comando que falha. Aqui
os comandos não têm alvo: não há função a chamar, nenhum crate de terceiro no
`[dependencies]` para exercer. O que falta é implementação.

**O que destrava:** implementar o handler de `OpenRequest` (o schema existe em `schemas/open-request.schema.json` e é validado pelo `xtask`, mas não há servidor) e ter os cinco alvos instaláveis para medir cold/warm. Sem app não há ativação a medir

**Custo de registrar assim:** se o mantenedor esperava que o spike M0 desta TASK
fosse escrito nesta rodada do passo 42, ele não foi — o passo 42 mede e registra o
estado, não implementa. Nenhum estado mudou: `tasks/backlog.json` continua com esta
TASK em `planned`.
