# TASK-E20-03 — Entregar shell SwiftUI touch-first e acessibilidade

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E20-03` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E20-03` |
| Critérios da spec | `platforms/05-ios.md` §2 |
| Fase | F6 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E20-01`, `TASK-E02-02` |
| Bloqueia | `TASK-E20-05`, `TASK-E20-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–8 dias úteis |
| Risco | alto |

## Entrega

Shell SwiftUI com controles de 44pt, bottom sheets e editor paramétrico em tela cheia, com VoiceOver e Dynamic Type.

## Boundary

- **Não** reusa a geometria do desktop: `platforms/01-matriz-suporte.md` §21 pede
  compartilhar algoritmos e contratos, não copiar o layout para a tela do telefone.
- **Não** implementa conversor nem restaurador: os dois não existem no build iOS
  (`platforms/05-ios.md` §1.2).
- **Não** entrega editor de DSP avançado: a cadeia e os presets são `TASK-E20-05`.
- **Não** valida áudio: a acessibilidade aqui é de navegação e leitura, não de sinal.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e20-03-claude-<slug>`)
- [ ] Hardware Apple, Xcode e assinatura disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E20-03-AC-1` | o shell renderizado em viewport de iPhone SE (375x667) | controles de DSP e playlist são medidos | controles de DSP e playlist acessíveis sem scroll horizontal |
| `TASK-E20-03-AC-2` | o VoiceOver ativo | o transporte e o painel de DSP são percorridos só por gestos de leitor | VoiceOver nomeia e opera transporte e DSP |
| `TASK-E20-03-AC-3` | o Dynamic Type em 200% | as telas principais são abertas novamente | Dynamic Type em 200% não esconde ação |
| `TASK-E20-03-AC-4` | o app aberto em retrato e girado para paisagem | a rotação é aplicada com áudio tocando | rotação não reinicia áudio nem perde seleção. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `xcrun simctl` com o viewport de iPhone SE e o Accessibility Inspector | cada controle é listado pelo inspector com nome e frame dentro do viewport |
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
