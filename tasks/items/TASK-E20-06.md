# TASK-E20-06 — Fechar assinatura, TestFlight e aceite iOS

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E20-06` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E20-06` |
| Critérios da spec | `platforms/05-ios.md` §2 |
| Fase | F6 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E20-02`, `TASK-E20-03`, `TASK-E20-04`, `TASK-E20-05` |
| Bloqueia | — |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Build assinado com Apple Developer Program, distribuição por TestFlight e relatório de aceite em aparelho real.

## Boundary

- **Não** publica na App Store. A entrega é TestFlight, a promoção a produção é
  decisão do mantenedor fora desta TASK, e `platforms/05-ios.md` fixa o escopo da
  fase em vez de uma entrega em loja.
- **Não** substitui hardware Apple: simulador não é aparelho, e ausência de device
  bloqueia o aceite em vez de o aprovar — a matriz de `platforms/01-matriz-suporte.md`
  declara o hardware como insumo do mantenedor.
- **Não** cria instalador: iOS não tem `dist/release`, e a distribuição é pela loja. Isso é `TASK-E11-07`.
- **Não** aprova o app inteiro: os gates comuns e os de iOS aqui fechados não
  afirmam paridade com os cinco alvos de 1.0; a matriz deles é `TASK-E14-06`.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e20-06-claude-<slug>`)
- [ ] Hardware Apple, Xcode e assinatura disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E20-06-AC-1` | o build assinado com o Apple Developer Program | ele é instalado por TestFlight em aparelho real | build assinado instala por TestFlight em aparelho real |
| `TASK-E20-06-AC-2` | os gates comuns de formato, DSP e playback | eles são executados no iOS | gates comuns aprovados no iOS |
| `TASK-E20-06-AC-3` | nenhum aparelho Apple disponível para o aceite | o relatório é preenchido | hardware ausente bloqueia o aceite em vez de aprová-lo |
| `TASK-E20-06-AC-4` | o build aprovado no TestFlight | a promoção para produção é considerada | abre sem violação de sandbox. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | relatório por gate no aparelho real e leitura do build no App Store Connect | o build aparece no TestFlight e cada gate tem sua linha; ausência de device é registrada como limitação |
| Hardware | aparelho Apple real com Xcode e assinatura; **nenhum** disponível neste ambiente (Windows) | ausência é limitação registrada, não aprovação |

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
