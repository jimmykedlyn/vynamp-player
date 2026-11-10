# TASK-E20-05 — Adaptar DSP e equalização ao layout e ao orçamento iOS

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E20-05` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E20-05` |
| Critérios da spec | `platforms/05-ios.md` §2 |
| Fase | F6 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E20-03`, `TASK-E04-01` |
| Bloqueia | `TASK-E20-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–8 dias úteis |
| Risco | alto |

## Entrega

Editor de DSP adaptado a gestos, com cadeia de efeitos sobre o núcleo Rust compartilhado e orçamento de CPU medido no aparelho.

## Boundary

- **Não** reimplementa o DSP: a cadeia é o núcleo Rust compartilhado, e a TASK só
  adapta a superfície de controle e mede o orçamento no aparelho.
- **Não** promete paridade de CPU com desktop: a meta é o aparelho de referência, e
  o que não couber não é anunciado.
- **Não** entrega plugin de terceiro nem VST3 no iOS.
- **Não** decide formato de preset novo; os presets continuam os do contrato comum.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e20-05-claude-<slug>`)
- [ ] Hardware Apple, Xcode e assinatura disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E20-05-AC-1` | um preset do contrato comum aplicado no desktop e no iOS | a resposta em frequência é medida nos dois | presets respondem dentro de ±0.2 dB do desktop |
| `TASK-E20-05-AC-2` | a cadeia de DSP completa ativa no aparelho de referência | a callback é inspecionada durante a reprodução | nenhuma alocação na callback |
| `TASK-E20-05-AC-3` | um preset que excede o orçamento de CPU do aparelho | ele é ativado com áudio tocando | cadeia completa não causa underrun no aparelho de referência |
| `TASK-E20-05-AC-4` | o editor em tela cheia no aparelho | os controles são manipulados por toque | valor aplicado é o que a curva exibe. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | captura numérica da resposta em frequência nos dois alvos e Instruments para a callback | as duas curvas são comparadas por medição, não por inspeção visual |
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
