# TASK-E13-01 — Provar Android arm64, bridge Rust e output

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. Critério de aceite: **PENDENTE** de autoria —
> a coluna `Then` é o requisito aprovado, copiado literal; `Given` e `When` ainda
> não existem.

| Campo | Valor |
|---|---|
| ID | `TASK-E13-01` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E13-01` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M0 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E11-01`, `TASK-E01-01`, `TASK-E01-02` |
| Bloqueia | `TASK-E13-02` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Spike NDK/ABI Rust, ownership de buffers, AAudio direto (vynamp-output) e adapter Media3, com decisões de UI/runtime.

## Boundary

<!-- PENDENTE: o que esta TASK não faz, mesmo parecendo relacionado. -->

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e13-01-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E13-01-AC-1` | um aparelho arm64-v8a real com Android 10 ou superior e o spike instalado | um WAV e um FLAC do spike são tocados pelo vynamp-output via AAudio | Tocar WAV/FLAC em aparelho arm64 real |
| `TASK-E13-01-AC-2` | a callback de áudio do spike com instrumentação de alocação e de travessia JNI | uma reprodução completa roda com o perfil de alocação ligado | callback sem JNI/alloc |
| `TASK-E13-01-AC-3` | o APK gerado pelo spike com as bibliotecas nativas | o tamanho de página e a compatibilidade com 16 KB são conferidos no artefato | native build compatível com requisitos de páginas |
| `TASK-E13-01-AC-4` | as duas opções de UI e runtime em avaliação | o mesmo fluxo de abrir, tocar, pausar e rotacionar roda em cada uma | comparar lifecycle e runtime antes congelar. |

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

Medido em 2026-11-09. **Esta TASK não foi executada: não havia o que executar.**

| Medição | Resultado |
|---|---|
| Tamanho do repo | `wc -l src/*.rs` → **3 linha(s)**, contendo só `pub const VERSION` e o comentário de módulo. `src/` tem 1 arquivo(s) |
| Worktree suja? | Não: `git status --short` limpo antes da medição |
| Especificação do módulo | `platforms/02-android.md` — **especificação proposta, implementação não iniciada** |

O critério de aceite abaixo exige tocar WAV/FLAC em aparelho arm64 real, callback AAudio sem JNI/alloc, build nativo compatível com requisitos de página, e comparação de lifecycle com Media3. Nenhum dos dois existe neste
repositório: não há código para compilar, medir, interromper ou comparar bit a bit.
Registrar uma nota de execução dizendo que o critério foi verificado seria falsear
a evidência — então a TASK permanece `planned` e esta nota registra o motivo
medido, não um resultado.

**Por que o motivo é o repo vazio e não uma limitação de ambiente:** uma limitação
de ambiente (hardware ausente, linker ausente) aparece como comando que falha. Aqui
os comandos não têm alvo: não há função a chamar, nenhum crate de terceiro no
`[dependencies]` para exercer. O que falta é implementação.

**O que destrava:** um aparelho Android arm64 físico — que não está disponível nesta máquina — mais o adapter AAudio em `vynamp-output` (hoje só existe `NullHost`) e o crate de plataforma Android em `apps/player`. Faltam código, hardware e NDK

**Custo de registrar assim:** se o mantenedor esperava que o spike M0 desta TASK
fosse escrito nesta rodada do passo 42, ele não foi — o passo 42 mede e registra o
estado, não implementa. Nenhum estado mudou: `tasks/backlog.json` continua com esta
TASK em `planned`.
