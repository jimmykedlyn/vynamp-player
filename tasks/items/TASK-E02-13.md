# TASK-E02-13 — Gerar gabarito de movimento, medidores e ondas

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-13` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-13` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M0 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E02-11` |
| Bloqueia | `TASK-E02-15` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–6 dias úteis |
| Risco | alto |

## Entrega

Transições CSS, séries rAF sob sinais determinísticos, constantes de AnalyserNode e balística e estilos de onda em formato consumível pelo Rust.

## Boundary

- **Não** inventa curva de medidor. As constantes vêm de `AnalyserNode` e da
  balística medidas no MVP, e o formato é o que o Rust consome.
- **Não** implementa o visualizador: `TASK-E17-09` e `TASK-E17-10`.
- **Não** congela o instante da captura: atributo animado por script é gravado em
  repouso e listado em `neutralized`, para o arquivo não depender do momento.
- **Não** reextrai as curvas de DSP: `out/svg/meters/dynamic.json` registra quais
  elementos animam, e as curvas pertencem a esta TASK, não ao pacote de `E02-11`.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e02-13-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E02-13-AC-1` | a folha de estilo do MVP e um sinal determinístico injetado em requestAnimationFrame | cada transição observável é medida quanto a duração, easing e keyframes ou série temporal | Cada animação do MVP tem duração, easing e keyframes ou série temporal |
| `TASK-E02-13-AC-2` | o medidor e o visualizador de onda do MVP recebendo degrau, seno e rajada sintéticos | os parâmetros do AnalyserNode, a balística e a resposta de cada sinal são registrados com a tolerância observada | medidores e ondas respondem a degrau, seno e rajada com tolerância documentada |
| `TASK-E02-13-AC-3` | cada uso de Web Audio do MVP identificado na varredura | o uso é traduzido para um comportamento descrito em formato consumível pelo Rust | Web Audio entra só como comportamento reimplementável. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | o gerador de séries `rAF` sob sinais determinísticos, comparado contra as transições CSS e as constantes de `AnalyserNode` do MVP | as séries são reproduzíveis entre duas execuções e batem com o MVP dentro da tolerância declarada; hoje não há gerador nem comparação no repo |
| Hardware | nenhum: as séries são numéricas e não dependem de GPU real | ausência é limitação registrada, não aprovação |

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
