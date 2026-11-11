# TASK-E02-12 — Gerar gabarito visual e estratégia de fontes

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-12` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-12` |
| Critérios da spec | `specs/ui.md` §5 (rascunho, não aprovado) |
| Fase | M0 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E02-11` |
| Bloqueia | `TASK-E02-15` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–6 dias úteis |
| Risco | médio |

## Entrega

Capturas por tema, layout, largura e estado com dump de geometria e estilo por elemento e fontes efetivas por nó; estratégia de fontes por plataforma com licenças.

## Boundary

- **Não** implementa as telas. Produz o gabarito contra o qual `TASK-E02-16` compara.
- **Não** escolhe fonte por conta própria: entrega a estratégia com licença por
  plataforma, e a escolha final é do mantenedor, como decide
  `product/05-decisoes.md` (decisões pendentes com dono e prazo).
- **Não** cobre movimento nem medidores: `TASK-E02-13`.
- **Não** mede DPI além de 100/125/150/200 nem layout além dos que
  `design/01-sistema-visual.md` lista na régua de screenshots de referência.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e02-12-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E02-12-AC-1` | o MVP de vynamp-mvp-reference no SHA 0d9860b901510021bcca4c31a25014373ea7c452 e a lista de estados de rendered/click-failures.json | a varredura automatizada percorre cada tema, layout, largura e estado e grava PNG mais o dump de geometria, estilo computado e custom properties por elemento | Todo estado alcançável do MVP tem captura e dump (geometria, estilo computado e custom properties --* por elemento, inclusive as de unsetVars), incluindo os estados cujo clique expira na varredura de TASK-E02-11 (`rendered/click-failures.json`: link de salto com foco, fila limpa), acionados por teclado ou script |
| `TASK-E02-12-AC-2` | as famílias de fonte efetivas por nó, tiradas do dump de AC-1 | cada família é confrontada com as fontes disponíveis em Windows, Linux, macOS e Android | cada família de fonte usada tem fonte nativa ou substituta licenciada com métricas comparadas |
| `TASK-E02-12-AC-3` | o conjunto de capturas gerado em AC-1, medido em número de arquivos e bytes | a decisão de armazenamento é registrada e a referência ao pacote é escrita no formato de design/07-pin-mvp.md §2 | armazenamento dos PNG decidido. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | o gerador de captura, rodado sobre a rev.38, com dump de geometria e estilo por elemento e a fonte efetiva por nó | cada combinação de tema, layout, largura e estado tem captura e dump; hoje o gerador não existe no repo do player |
| Hardware | nenhum: a captura é feita sobre o MVP de referência, sem dispositivo de áudio | ausência é limitação registrada, não aprovação |

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
