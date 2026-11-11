# TASK-E03-08 — Implementar escrita de tags e capítulos em lote

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E03-08` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E03-08` |
| Critérios da spec | `specs/biblioteca.md` §5 (rascunho, não aprovado) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E03-04`, `TASK-E08-01` |
| Bloqueia | `TASK-E03-10`, `TASK-E12-06`, `TASK-E17-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

Batch preview/write atomic, preserve unknown tags e backup.

## Boundary

- **Não** cria os adapters de leitura de tags nem a precedência de capa; ler o container é TASK-E03-04, e a escrita aqui tem de reusar o mesmo mapeamento de campo para não gravar um frame com semântica diferente da que a biblioteca indexa.
- **Não** move, renomeia nem deduplica arquivos a partir dos campos gravados; organização e dedupe são TASK-E08-04, e escrever tag não pode virar gatilho implícito de mover mídia.
- **Não** certifica tags técnicas, capítulos e sidecars contra o corpus de conformidade; a certificação é TASK-E17-06, e o que esta TASK prova é a atomicidade do lote, não a aderência a cada variante de container.
- **Não** implementa o backup do banco, o WAL nem o write actor do SQLite; esse backup é TASK-E03-01, e o backup desta TASK é por arquivo, uma cópia do original ao lado da mídia antes do write.
- **Não** faz o recovery pós-crash da biblioteca nem o rollback de migration; recovery e rollback são TASK-E03-10, e o kill do AC-2 é do processo escritor de tag, não do banco.
- **Não** implementa a revisão e o undo/redo da playlist editável; esse histórico de edição é TASK-E03-06, e desfazer uma escrita em lote exigiria reescrever mídia, o que está fora do escopo.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e03-08-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E03-08-AC-1` | um lote de 40 arquivos na mesma seleção, 12 deles com PERFORMER multivalue de 2 valores e 28 sem esse campo, com o preview do lote já gerado e a opção de limpar campos ausentes desmarcada | o lote é aplicado pela chamada tags.batch.apply preenchendo só o campo ALBUM | Fields mistos não são apagados sem escolha |
| `TASK-E03-08-AC-2` | um lote de 25 FLAC de 200 s cada em volume NTFS local, um deles aberto por outro processo com LockFileEx sem FILE_SHARE_WRITE, e o processo do player morto por TerminateProcess durante a escrita do 13º arquivo | o batch é aplicado e, depois do kill, os 25 arquivos são reabertos por um validador que decodifica o áudio e compara o CRC com o original | kill/locked file mantém mídia |
| `TASK-E03-08-AC-3` | um FLAC de 200 s de duração dentro de um lote que declara 6 capítulos, com o 4º em start 240.000 ms (além do fim da mídia) e o 5º com start menor que o do 4º | o write de capítulos é submetido pela chamada chapters.batch.write e o arquivo em disco é lido de volta pelo preview | capítulo inválido não é gravado. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test --manifest-path apps/player/Cargo.toml --locked --test tag_batch, rodado da raiz do umbrella, sobre fixtures tests/fixtures/tags/mixed_40 (40 arquivos, 12 com PERFORMER multivalue de 2 valores), tests/fixtures/tags/locked_25 (25 FLAC com um alvo de LockFileEx e kill do processo no 13º) e tests/fixtures/tags/chapters_invalid (FLAC de 200 s com 6 capítulos, o 4º em 240.000 ms) | espera-se 0 de 40 arquivos com PERFORMER apagado e os 12 mantendo exatamente 2 valores quando clean-missing não foi escolhido; o arquivo travado devolve Locked enquanto 24 dos 25 são escritos; depois do kill cada um dos 25 arquivos decodifica 200 s de áudio e o 13º tem CRC igual ao do original ou o CRC do novo conteúdo completo, nunca um terceiro valor; dos 6 capítulos declarados 0 são gravados, o hash do arquivo não muda e o erro nomeia os índices 4 e 5. Hoje o repo vynamp-player tem 1 arquivo .rs (src/main.rs) com println!("vynamp_player 0.1.0"), [dependencies] vazio e nenhum tests/; hoje cargo test --locked ali responde running 0 tests com 0 passed e o comando acima falha com `error: no test target named tag_batch in default-run packages`: não existe batch writer, preview nem backup por arquivo. |
| Hardware | nenhum: os alvos são arquivos de áudio locais em tempdir, o lock usa LockFileEx do Windows 11 x86_64 e o kill é de um processo filho; nenhum dispositivo de áudio é aberto e não há mídia óptica nem unidade de rede | ausência é limitação registrada, não aprovação |

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
