# TASK-E02-02 — Implementar tokens e componentes semânticos

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-02` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-02` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M1 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E02-01`, `TASK-E02-11` |
| Bloqueia | `TASK-E02-03`, `TASK-E06-01`, `TASK-E13-05` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–8 dias úteis |
| Risco | médio |

## Entrega

Theme token loader builtin, buttons/sliders/list rows com estados completos.

## Boundary

- **Não** desenha as telas. Componentes e tokens são a base; as geometrias são
  `TASK-E02-03` e `TASK-E02-04`.
- **Não** copia o CSS do MVP. O contrato é o valor semântico (`surface.raised`,
  `text.muted`), e o mapeamento de cada token está em `design/02-layouts.md`.
- **Não** implementa tema de usuário nem editor: a dona é `TASK-E06-07`, que entrega o
  editor e o preview.
- **Não** cobre DPI além de 200%: a matriz de DPI testada é 100/125/150/200, medida
  por `TASK-E10-06`.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e02-02-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E02-02-AC-1` | o pacote de temas de vynamp-mvp-reference no SHA do pin e o loader de tokens do player | cada componente é construído e suas propriedades visuais são lidas em runtime | Usar tokens canônicos |
| `TASK-E02-02-AC-2` | um botão, um slider e uma linha de lista em estado habilitado | o harness aplica focus, hover, pressed e disabled e compara os estilos computados dos quatro estados | focus/hover/pressed/disabled diferem |
| `TASK-E02-02-AC-3` | todos os controles do Classic Stack renderizados nos 12 temas do pacote | a razão de contraste entre texto e fundo é calculada e a árvore de acessibilidade é lida por controle | contraste medido e nome acessível em cada controle. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test` sobre o loader de tokens e uma passada visual por estado (hover, pressed, focused, disabled, selected) | cada um dos doze temas carrega e cada componente renderiza os cinco estados, conferidos contra os valores do token; hoje nenhum teste de UI existe no crate |
| Hardware | nenhum: tokens e componentes são determinísticos e testáveis em processo | ausência é limitação registrada, não aprovação |

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
