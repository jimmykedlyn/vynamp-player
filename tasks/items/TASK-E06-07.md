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
| Critérios da spec | `specs/temas.md` §5 (rascunho, não aprovado) |
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

- **Não** define o schema de tema nem o validador, e não estabelece o contrato de erro por JSON Pointer. Isso é `TASK-E06-01`, com os schemas de `schemas/`; a confusão é plausível porque o editor também aponta o campo errado por ponteiro, mas aqui o ponteiro vem do validador e o editor só o exibe, e reimplementar a validação daria duas fontes para o mesmo erro.
- **Não** instala tema, não faz staging, não faz commit atômico por versão nem restaura a versão anterior. Instalação é `TASK-E06-02`; o exportar e importar deste AC usa o caminho dela, e reproduzir o swap aqui criaria dois caminhos de instalação com estados intermediários diferentes.
- **Não** implementa o renderer de nós e bindings que desenha o layout editado. Renderer, ações permitidas e fallback por modo são `TASK-E06-03`; o editor edita os documentos que ele consome, e transformar o editor em renderer faria o preview deixar de medir o que o app real mostra.
- **Não** autora os três temas originais nem licencia os assets deles. Classic Graphite, Midnight Modern e Copper são `TASK-E06-04`, dependência desta TASK; os starter templates do editor são cópias desses temas, não um quarto conjunto de originais a manter.
- **Não** constrói a galeria local, o armazenamento por usuário nem o safe mode. É `TASK-E06-08`, que esta TASK bloqueia; a persistência do rascunho do editor não é a galeria, e tratar as duas como uma só faria o safe mode ter de saber desfazer edição, o que não é papel dele.
- **Não** automatiza a regressão visual por DPI no CI nem produz o relatório de fidelidade dos temas. A suíte automática é `TASK-E10-06` e o relatório de fidelidade é `TASK-E06-10`; o preview daqui é ferramenta de autoria, e um preview aprovado não é captura de regressão nem aprovação de acessibilidade.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e06-07-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E06-07-AC-1` | um tema derivado de `assets/design-tokens.json` com 4 tokens alterados (`accent` #7A5CFF, `bg` #101014, `surface` #18181F, `radius` 8), 2 nós de layouts/classic.json movidos em x e y e 1 PNG de 512 por 512 em assets/, aberto no editor, e um perfil temporário vazio onde esse tema nunca foi instalado | o editor exporta o pacote `.vyntheme`, o perfil vazio importa o pacote e os dois documentos são canonicalizados e comparados campo a campo, com o sha256 de cada um registrado | Exportar e importar preserva configuração |
| `TASK-E06-07-AC-2` | um tema em edição no buffer do editor, que já passa pelo validador de `TASK-E06-01`, com o par text-muted #8D97AB sobre panel-raised #333A4D medido em 3,86:1, um nó `panel` que declara as constraints `min_width` 900 e `max_width` 600, e os pares conformes text-primary #F1F3F7 sobre chassis #252936 em 13,04:1 e lime #B8F35A sobre display #080D0B em 14,92:1 | o comando `validate` do editor roda sobre o tema em edição e devolve a lista de avisos, cada um com o JSON Pointer do campo e a razão de contraste medida | avisar problemas de contraste/constraints |
| `TASK-E06-07-AC-3` | o tema A ativo com 30 s de faixa de 48 kHz tocados, o tema B aberto no preview em 100, 150 e 200% de DPI nos estados normal, hover, selected e disabled, com os hashes do pacote de A e os contadores do stream anotados antes | as 12 combinações de DPI e estado são percorridas em sequência e ao fim se leem o identificador do tema ativo, os hashes do pacote de A e os contadores de frames e de underrun do stream | preview não altera tema ativo nem áudio. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --test theme_editor` no repo `vynamp-player`, com o tema derivado de `assets/design-tokens.json` com 4 tokens e 2 constraints alterados exportado e reimportado num perfil temporário, a fixture de contraste e de geometria fora de limite, e o preview percorrido em 100, 150 e 200% de DPI nos quatro estados, tudo offscreen por `pixels_per_point`. Hoje não existe editor, importador nem preview: o repo do player tem um único `.rs` de 54 bytes com `println!`, `[dependencies]` vazio e nenhum `tests/`; o `schemas/theme-layout.schema.json` não tem anchor, mínimo, máximo nem fallback de constraint, e o `schemas/README.md` declara que o modelo completo de constraints precisa ser ampliado antes de implementar. Os 4 tokens, os 2 constraints e o PNG de 512 por 512 somam o que o critério do backlog pede na entrada; a fixture ainda não existe. | Esperado: o round trip preserva o documento canonicalizado byte a byte, com os 4 tokens alterados, os 2 nós movidos e o PNG saindo com os mesmos valores, sha256 igual antes e depois e 0 campos perdidos; a validação emite 1 aviso de contraste para text-muted #8D97AB sobre panel-raised #333A4D com razão medida 3,86:1 contra o limite 4,5:1, e 1 aviso de geometria para o `panel` com `min_width` 900 maior que `max_width` 600, cada um apontando o JSON Pointer do campo, sem emitir aviso para os pares conformes text-primary #F1F3F7 sobre chassis #252936 em 13,04:1 e lime #B8F35A sobre display #080D0B em 14,92:1 — um validador que acuse esses dois está errado; nas 12 combinações de preview o tema ativo continua sendo A, o hash do pacote de A não muda e o stream entrega 1 440 000 frames em 30 s com 0 underrun. Um limite de contraste que hoje não está escrito em lugar nenhum do repo: o mínimo 4,5:1 vem da spec de UI, não de um número constante em código. E nada disso roda, porque não há código de tema no player. |
| Hardware | nenhum: o editor, a validação e o preview rodam offscreen com `pixels_per_point` em 1,0, 1,5 e 2,0 sobre arquivos, hashes e JSON Pointers, com o áudio no backend null, e nenhum dos três critérios pede monitor físico, placa ou aparelho; a conferência humana em 200% de DPI no monitor de 1920x1080 desta máquina e o workflow com leitor de tela pertencem a `TASK-E10-06` e ficam fora desta prova, sem que isso reprove qualquer AC daqui | ausência é limitação registrada, não aprovação |

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
