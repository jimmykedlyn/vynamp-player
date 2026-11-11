# TASK-E02-01 — Validar toolkit Rust com UI clássica acessível

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-01` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-01` |
| Critérios da spec | `specs/ui.md` §5 (rascunho, não aprovado) |
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

- **Não** escolhe o toolkit. Compara egui/AccessKit com a alternativa e entrega a
  evidência para a ADR-003; a decisão é do mantenedor, e um spike que "já decidiu"
  não tem o que comparar.
- **Não** porta o MVP. Tokens, painéis e movimento são `TASK-E02-02` em diante.
- **Não** promete acessibilidade em Android: TalkBack é `TASK-E13-05`, e o mesmo
  código de UI não roda nas duas plataformas.
- **Não** mede áudio. O spike é de interface; a cadeia de playback é `TASK-E01-03`.

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
| Oráculo independente | `cargo test` em `vynamp-player` sobre o spike, mais NVDA e VoiceOver percorrendo as três telas de amostra em Windows e macOS | o checklist de leitor de tela é preenchido por alvo, com o que falhou nomeado; hoje o crate só imprime a versão e não há esse teste |
| Hardware | uma máquina Windows e uma macOS com leitor de tela; **nenhuma** das duas está disponível neste ambiente, que é Windows sem NVDA instalado | ausência é limitação registrada, não aprovação |

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
| Especificação do módulo | `apps/player/specs/ui.md` — **especificação proposta, implementação não iniciada** |

O critério de aceite abaixo exige um spike egui/AccessKit rodando em Windows, Linux, macOS Intel e Silicon, com o layout Classic e o workflow de transporte por Narrator/NVDA/Orca/VoiceOver funcionando, e DPI/zoom 200% sem esconder ações. Nenhum dos dois existe neste
repositório: não há código para compilar, medir, interromper ou comparar bit a bit.
Registrar uma nota de execução dizendo que o critério foi verificado seria falsear
a evidência — então a TASK permanece `planned` e esta nota registra o motivo
medido, não um resultado.

**Por que o motivo é o repo vazio e não uma limitação de ambiente:** uma limitação
de ambiente (hardware ausente, linker ausente) aparece como comando que falha. Aqui
os comandos não têm alvo: não há função a chamar, nenhum crate de terceiro no
`[dependencies]` para exercer. O que falta é implementação.

**O que destrava:** escolher o toolkit de UI e escrever a primeira tela. `apps/player/src/main.rs` tem 3 linhas e imprime a versão; não há egui, não há janela, não há árvore de acessibilidade. E o workflow de leitor de tela exige os quatro leitores instalados, dos quais esta máquina tem no máximo o Narrator

**Custo de registrar assim:** se o mantenedor esperava que o spike M0 desta TASK
fosse escrito nesta rodada do passo 42, ele não foi — o passo 42 mede e registra o
estado, não implementa. Nenhum estado mudou: `tasks/backlog.json` continua com esta
TASK em `planned`.
