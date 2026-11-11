# TASK-E08-04 — Entregar organização de arquivos e dedupe

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E08-04` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E08-04` |
| Critérios da spec | `specs/biblioteca.md` §5 (rascunho, não aprovado) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E08-01`, `TASK-E03-05` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

Templates seguros, plano de renomeação, detecção de colisão e dedupe com preview.

## Boundary

- **Não** implementa a escrita das tags que alimentam o template. O fluxo de write em lote é de `TASK-E03-08`; aqui o template lê tags já existentes, e corrigir o valor para melhorar o nome do arquivo faria dois donos do write — a confusão é plausível porque organizar a pasta costuma vir junto com corrigir metadado.
- **Não** implementa a consulta, o FTS nem os filtros que o dedupe usa para agrupar. O dono é `TASK-E03-05`, dependência declarada; esta TASK consome o conjunto de faixas já consultado e trazer a consulta para cá duplicaria o schema.
- **Não** implementa o import incremental nem a detecção de root offline que reconcilia o arquivo movido. O dono é `TASK-E03-03`; o AC-3 exige que o `file_id` sobreviva ao move, não que a biblioteca redescubra arquivos por conta própria, e é fácil achar que mover implica reescanear a árvore.
- **Não** executa a extração de faixas CUE nem o split de imagem única. O dono é `TASK-E03-06`; as duas coisas geram arquivos novos em disco a partir de um template de nome, então a confusão com renomear é plausível, mas aqui nenhum arquivo de áudio novo é criado.
- **Não** converte formato durante a organização nem usa o registry de encoders. O dono é `TASK-E08-02`; a ferramenta de organizar aparece na mesma lista de ferramentas M3 que o conversor e o AC-2 fala em 12 MiB contra 11 MiB, o que parece divergência de formato e é só tamanho diferente.
- **Não** implementa o gate de biblioteca grande nem o relatório de recovery pós-move. O dono é `TASK-E03-10`; o plano de renomeação de 100 mil arquivos e a retomada depois de interrupção pertencem a esse gate, não ao motor de template.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e08-04-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E08-04-AC-1` | Um template `{album artist}/{album}/{track} - {title}.{ext}` e cinco FLAC de 1 MiB cujas tags produzem, depois da substituição, os nomes `../../evil.flac`, `CON.flac`, `NUL.flac`, `a:b.flac` e um título de 300 caracteres, numa árvore temporária NTFS com raiz em `D:\tmp\organize-root` no Windows 11. | O plano de renomeação é gerado em modo somente plano (dry-run) e depois aplicado sobre a árvore. | Bloquear escape e nomes reservados |
| `TASK-E08-04-AC-2` | Uma raiz destino que já contém `Fixture Artist/Fixture Album/01 - Same Title.flac` de 12 MiB com hash SHA-256 diferente, e um arquivo de origem de 11 MiB cujo template resolve exatamente para esse mesmo caminho final. | O plano de organização é aplicado sem a opção de overwrite habilitada. | não sobrescrever por padrão |
| `TASK-E08-04-AC-3` | Uma faixa com `file_id` 4821 na biblioteca SQLite, bookmark de posição 123,4 s e rating 4, arquivo em `D:\Music\in\a.flac`, e uma regra de organização que move para `D:\Music\Artists\`. | O plano de renomeação e movimentação é aplicado e a biblioteca reconsulta a faixa pelo `file_id` 4821. | mover preserva identidade/bookmark |
| `TASK-E08-04-AC-4` | Quatro arquivos numa raiz de teste: dois FLAC de 8 MiB byte a byte idênticos (mesmo SHA-256) e dois FLAC de 6 MiB com áudio decodificado igual mas tags diferentes (similaridade 0,98), sem hash de áudio comum. | O dedupe roda, agrupa as duplicatas exatas e prováveis, e o usuário confirma o grupo na tela de preview. | duplicatas não são apagadas automaticamente. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --test organize_dedupe` no repo `vynamp-player` sobre uma árvore montada em `D:\tmp\organize-root` com os cinco arquivos de tags hostis (`../../evil`, `CON`, `NUL`, `a:b`, título de 300 caracteres) e os quatro duplicados (2 exatos por SHA-256, 2 prováveis com similaridade 0,98), listando a árvore antes e depois com `Get-ChildItem -Recurse` e comparando SHA-256 com `Get-FileHash`. | Os 5 nomes hostis saem do plano em forma sanitizada que ainda grava dentro de `D:\tmp\organize-root`: o `..` é removido do caminho resolvido (0 arquivos fora da raiz; nenhum `evil.flac` no diretório acima), nenhum arquivo final se chama `CON` ou `NUL` e nenhum título de 300 caracteres passa sem corte segundo o limite documentado; na colisão única de nome o arquivo de 12 MiB já existente mantém o SHA-256 original e o de 11 MiB entra com outro nome, totalizando 0 sobrescritas; 1 registro `file_id` 4821 aponta para o caminho novo com bookmark de 123,4 s e rating 4 inalterados; os 4 duplicados continuam em disco com o SHA-256 de origem depois de o preview mostrar 1 grupo exato de 2 e 1 grupo provável de 2, ou seja, 0 deleções automáticas. Hoje nada disso roda: o repo `vynamp-player` tem 1 arquivo `.rs` (`src/main.rs`) com um `println!` de `vynamp_player 0.1.0`, zero dependências e nenhum `tests/`, então não há runner, motor de template nem banco para exercitar. |
| Hardware | nenhum: tudo é filesystem local, com a árvore de teste em diretório temporário do volume D:, que esta máquina Windows 11 x86_64 tem como NTFS; nenhum critério abre dispositivo de áudio, unidade removível ou compartilhamento de rede, e o comportamento de nome reservado depende só da API de arquivos do próprio Windows. | ausência é limitação registrada, não aprovação |

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
