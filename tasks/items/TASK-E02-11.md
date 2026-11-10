# TASK-E02-11 — Congelar MVP de referência e extrair SVGs, ícones e temas

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-11` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-11` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M0 |
| Prioridade | P0 |
| Estado | implemented |
| Depende de | nenhuma |
| Bloqueia | `TASK-E02-02`, `TASK-E02-12`, `TASK-E02-13`, `TASK-E02-14` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–4 dias úteis |
| Risco | médio |

## Entrega

Pacote assets/reference/mvp com MVP rev.37 congelado por hash, 12 temas resolvidos, 91 ícones, 16 medidores e SVGs renderizados, extraídos por ferramenta reprodutível.

## Boundary

- **Não** copia nenhum arquivo do MVP para este repositório. A spec cita URL, SHA e
  caminho no repositório de referência, e o código do MVP não é código de produto.
- **Não** congela a rev.37 como fonte atual. O pacote registra que a extração foi
  feita sobre a rev.37 e que a rev.38 (`0d9860b`) é a fonte; reescrever o histórico
  para "rev.38" apagaria a diferença que a reextração precisa explicar.
- **Não** extrai movimento, medidores nem comportamento: `TASK-E02-13` e `TASK-E02-14`.
- **Não** aceita o pacote antigo com `SOURCE.json`. O pin virou `tools/pin.cjs`, e o
  repositório não recebe o formato anterior.

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
| Oráculo independente | `npm run verify` no `vynamp-mvp-reference` no pin `0d9860b`, que reextrai para um diretório temporário e compara byte a byte com `out/` | a comparação fecha em 499 arquivos e 0 diferem na rev.38; uma diferença de cobertura aparece em `out/svg/rendered/click-failures.json`. Esta TASK está `implemented` e o número acima é o da rev.38, não uma medição nova |
| Hardware | nenhum: a extração é reprodutível a partir dos blobs do commit | ausência é limitação registrada, não aprovação |

## DoD — pronto para review

- [ ] Todos os critérios de aceite com evidência
- [ ] `cargo fmt --check` e `cargo clippy --all-targets -- -D warnings` limpos
- [ ] `cargo test` verde; testes ignorados listados com motivo
- [ ] Spec do módulo atualizada se a implementação revelou algo novo
- [ ] `backlog.json` com o estado correto
- [ ] Revisão feita por contexto limpo, não pelo implementador
- [ ] PR aberto com template preenchido; CI Gate verde no SHA final
## Notas de execução

Fechada em 2026-11-09 com o pacote apontando para `vynamp-mvp-reference`
(decisão do mantenedor). A extração foi provada em 2026-10-18 e confirmada de
novo hoje contra o estado atual do repositório de referência; nenhum arquivo do
MVP está ou volta a estar neste repositório.

| AC | Verificação executada | Resultado |
|---|---|---|
| AC-1 | `out/check/report.json` (svgcheck, rev.38) | 477 SVGs; **471 `pass`**; 6 reprovam com a razão registrada: 4 `blurFn` (todos `cropped: true`), 1 `pathLength` (com `text: true`) e 1 `text: true` com `gaps: []` |
| AC-2 | o mesmo relatório, campo `text` | os 6 reprovam ou por glifo rasterizado (`text: true`, 2 arquivos) ou por gap de renderizador (`blurFn`/`pathLength`, 5 arquivos); nenhum reprova por geometria de caminho |
| AC-3 | `gaps` do relatório | `blurFn` ×4 e `pathLength` ×1 — exatamente os dois recursos que o `Then` manda virar requisito do carregador nativo |
| AC-4 | `out/themes/themes.json` | 12 temas; `unsetVars` com **22** variáveis, cada uma com `fallbacks`, `setBy` (origem) e `missingIn` (temas afetados) |
| AC-5 | `node tools/pin.cjs` duas vezes + `npm run verify` | `contentSha256` **idêntico** nas duas execuções: `f00f766045edf4c1032c9a30c5826803c090f960ac9377d815c020079f398bba`; `git mvp/` = `f85fd662…`, `out/` = `1d9f9615…`, 203 arquivos, 2 118 012 bytes. `npm run verify`: **`compared 499 files, 0 differ`**, exit 0 |

O hash acima é o mesmo que `design/07-pin-mvp.md` registra para `0d9860b` —
ou seja, o conteúdo de `mvp/` e de `out/` **não mudou** desde o pin; o gitlink
avou para `e689d90` (PR #3, CI do MVP) sem tocar no pacote.

**Onde o pacote vive, hoje.** Este repo não tem `assets/reference/mvp` e não
volta a ter. O pacote extraído é `vynamp-mvp-reference` (público, branch
`main`, fixado em `e689d90` para a ferramenta; conteúdo em `0d9860b`), e o
consumidor passa a ser `vynamp-player` por **citação** — URL + SHA fixo +
caminho dentro daquele repositório, o mesmo formato de
`design/07-pin-mvp.md §2`. A ferramenta que o mantém reprodutível é
`tools/pin.cjs` (`npm run pin`), que imprime a própria tabela de proveniência;
`tools/package.cjs` foi removido no PR #1 do repositório de referência. Quem
precisar do pacote em disco clona `vynamp-mvp-reference` no SHA do pin — não
o copia para dentro do app.

**O que esta TASK não provou:** nenhuma comparação foi refeita contra a
rev.37 da evidência original, e o `svgcheck` roda sobre os 477 SVGs versionados
de `out/svg/`, não sobre a árvore de trabalho. Nenhum SVG novo foi gerado;
nenhum tema foi resolvido em runtime nativo. As TASKs que consomem o pacote
(`E02-12`..`E02-16`) continuam `planned` e sem critérios.

