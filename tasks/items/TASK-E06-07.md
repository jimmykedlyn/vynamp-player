# TASK-E06-07 — Entregar editor e preview de temas

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E06-07` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E06-07` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E06-04`, `TASK-E06-06` |
| Bloqueia | `TASK-E02-10`, `TASK-E06-08` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Editor de tokens/layout, undo, preview por DPI/estado e exportação.

## Boundary

- **Não** publica, assina ou envia o pacote para o registry. O dono é `TASK-E06-09`; exportar aqui é gerar o `.vyntheme` local, e quem for executar pode confundir exportação com publicação.
- **Não** constrói a galeria local, o armazenamento por usuário nem o safe mode. O dono é `TASK-E06-08`, que depende desta TASK; o editor exporta o arquivo e não o instala na galeria.
- **Não** define o schema nem o validador canônico do `.vyntheme` e do manifest. O dono é `TASK-E06-01`; o editor chama o validador existente e reporta erro por JSON pointer.
- **Não** implementa o espectrograma, as partículas e o fallback de CPU que o preview precisa desenhar. O dono é `TASK-E06-06`, dependência desta TASK; o preview só troca DPI, densidade e estado.
- **Não** faz o harness de fidelidade com `egui_kittest` nem o teste de experiência desktop ponta a ponta. Os donos são `TASK-E02-16` e `TASK-E02-10`; preview sem alterar o tema ativo não substitui regressão de UI.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e06-07-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E06-07-AC-1` | tema com 4 tokens alterados (`accent` #7A5CFF, `bg` #101014, `surface` #18181F, `radius` 8), 2 constraints movidos em layouts/classic.json e 1 PNG 512x512 em assets/, exportado uma vez para `edited.vyntheme` | o arquivo é importado numa instalação limpa e tokens.json e layouts/classic.json são comparados byte a byte com o tema de origem | Exportar e importar preserva configuração |
| `TASK-E06-07-AC-2` | tema em edição com `text` #777777 sobre `bg` #808080 (contraste 1,19:1) e um nó `panel` com `min_width` 900 e `max_width` 600 | a validação do editor é executada sobre o tema em edição pelo comando `validate` | avisar problemas de contraste/constraints |
| `TASK-E06-07-AC-3` | tema ativo Classic Graphite instalado com SHA256 conhecido, faixa de 48 kHz tocando há 20 s e editor aberto com preview em DPI 150% e densidade padrão | o preview é alternado para DPI 200%, densidade compacta e estado `playing` e depois fechado sem salvar | preview não altera tema ativo nem áudio. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test --test theme_editor (fixtures: `fixtures/themes/edited.vyntheme` com 4 tokens, 2 constraints e 1 PNG 512x512; `fixtures/themes/contrast-bad.vyntheme` com `text` #777777 sobre `bg` #808080) | hoje nada disso existe: `src/main.rs` é um `println!("vynamp_player 0.1.0")`, `Cargo.toml` não tem dependências e não há `tests/`; a evidência exigida é round-trip com 0 bytes de diferença em tokens.json e layouts/classic.json e SHA256 idênticos, exatamente 2 avisos apontando `/tokens/text` (contraste 1,19:1 contra mínimo 4,5:1) e `/layouts/classic/panel/constraints/min_width`, e SHA256 do tema ativo igual antes e depois do preview com 0 xruns durante a alternância. |
| Hardware | nenhum: o preview por DPI e estado é medido por hash de arquivo e por JSON pointer, e as fixtures de tema são arquivos locais; nenhum monitor físico, dispositivo de áudio ou aparelho externo entra no critério. | ausência é limitação registrada, não aprovação |

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
