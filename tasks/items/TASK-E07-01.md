# TASK-E07-01 — Provar sandbox WASM e capabilities

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E07-01` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E07-01` |
| Critérios da spec | `specs/plugins.md` §5 (rascunho, não aprovado) |
| Fase | M0 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E11-01` |
| Bloqueia | `TASK-E07-02`, `TASK-E17-09` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Spike Wasmtime/WIT com limites de memória, fuel, interruption e ADR-004.

## Boundary

- **Não** escolhe o runtime de plugin. Compara Wasmtime/WIT com a alternativa e
  entrega a evidência para a ADR-004.
- **Não** implementa o SDK nem o registry: `TASK-E07-*` seguintes e `TASK-E09-09`.
- **Não** promete sandbox em Android ou iOS: o spike é de desktop, e no iOS plugin de
  terceiro está fora de escopo por `platforms/05-ios.md`.
- **Não** mede áudio em tempo real: mede limites de memória, fuel e interrupção; áudio real é `TASK-E12-07`.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e07-01-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E07-01-AC-1` | um módulo Wasm de teste com loop infinito e outro que aloca sem limite, com fuel e limite de memória configurados | o host interrompe cada módulo pelo limite que estourar primeiro | Interromper loops e crescimento de memória |
| `TASK-E07-01-AC-2` | um módulo que tenta abrir arquivo e abrir soquete pelas importações disponíveis | o módulo roda no host com as capabilities padrão | nenhum acesso ambiente a arquivos/rede |
| `TASK-E07-01-AC-3` | o app iniciando sem nenhum plugin instalado | memória residente e tempo de start são medidos com e sem o runtime Wasm disponível | medir consumo com runtime não carregado. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test` sobre o spike com um módulo que estoura memória, um que consome fuel até o teto e um que é interrompido no meio | os três terminam com erro tipado e o host sobrevive; hoje não há spike nem teste no crate do player |
| Hardware | nenhum: o sandbox é medido em processo e o limite é do runtime | ausência é limitação registrada, não aprovação |

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
| Especificação do módulo | `apps/player/specs/plugins.md` — **especificação proposta, implementação não iniciada** |

O critério de aceite abaixo exige um host Wasmtime com fuel e limite de memória, um módulo que estoura cada limite, um módulo que tenta abrir arquivo e soquete, e medição de memória/start sem runtime carregado. Nenhum dos dois existe neste
repositório: não há código para compilar, medir, interromper ou comparar bit a bit.
Registrar uma nota de execução dizendo que o critério foi verificado seria falsear
a evidência — então a TASK permanece `planned` e esta nota registra o motivo
medido, não um resultado.

**Por que o motivo é o repo vazio e não uma limitação de ambiente:** uma limitação
de ambiente (hardware ausente, linker ausente) aparece como comando que falha. Aqui
os comandos não têm alvo: não há função a chamar, nenhum crate de terceiro no
`[dependencies]` para exercer. O que falta é implementação.

**O que destrava:** adicionar `wasmtime` como dependência, escrever o host de plugins e os módulos `.wasip1`/WIT de teste. `apps/player` hoje é um `main` que imprime a versão; não há host, não há dependência, não há módulo

**Custo de registrar assim:** se o mantenedor esperava que o spike M0 desta TASK
fosse escrito nesta rodada do passo 42, ele não foi — o passo 42 mede e registra o
estado, não implementa. Nenhum estado mudou: `tasks/backlog.json` continua com esta
TASK em `planned`.
