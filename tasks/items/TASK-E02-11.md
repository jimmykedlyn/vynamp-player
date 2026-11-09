# TASK-E02-11 — Congelar MVP de referência e extrair SVGs, ícones e temas

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. Critério de aceite: **PENDENTE** de autoria —
> a coluna `Then` é o requisito aprovado, copiado literal; `Given` e `When` ainda
> não existem.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-11` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-11` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M0 |
| Prioridade | P0 |
| Estado | in_progress |
| Depende de | nenhuma |
| Bloqueia | `TASK-E02-02`, `TASK-E02-12`, `TASK-E02-13`, `TASK-E02-14` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–4 dias úteis |
| Risco | médio |

## Entrega

Pacote assets/reference/mvp com MVP rev.37 congelado por hash, 12 temas resolvidos, 91 ícones, 16 medidores e SVGs renderizados, extraídos por ferramenta reprodutível.

## Boundary

<!-- PENDENTE: o que esta TASK não faz, mesmo parecendo relacionado. -->

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e02-11-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E02-11-AC-1` | cada SVG do pacote, com zoom 2, renderizado no Chrome e no resvg sobre branco e sobre preto | o svgcheck compara os dois, com todo `<text>` removido | Cada SVG renderiza em resvg sem pixel de geometria divergente do Chrome (delta>48 fora do antialiasing de 1 px |
| `TASK-E02-11-AC-2` | um SVG cuja diferença só aparece na rasterização de glifos | o svgcheck isola o caso | texto registrado e resolvido em E02-12 |
| `TASK-E02-11-AC-3` | um SVG que usa `pathLength` ou `filter: blur()` de CSS | o svgcheck compara o resvg sobre o arquivo normalizado com o Chrome sobre o original | recurso sem suporte no resvg vira requisito do carregador nativo) |
| `TASK-E02-11-AC-4` | cada tema do pacote | os tokens `--*` de `:root` são resolvidos e as variáveis referenciadas são conferidas contra `unsetVars` | cada tema traz resolvidos todos os tokens --* que define em :root, e toda variável referenciada que o tema não define em :root consta de unsetVars com fallback, origem e temas afetados (valor por estado em E02-12) |
| `TASK-E02-11-AC-5` | o pacote extraído uma vez | a ferramenta de extração roda de novo | reextração reproduz os mesmos hashes. |

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

Preenchido durante a execução, não antes.
