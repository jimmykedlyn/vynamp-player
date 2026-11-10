# TASK-E03-04 — Implementar leitura de tags e capas segura

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E03-04` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E03-04` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E03-03` |
| Bloqueia | `TASK-E03-05`, `TASK-E03-06`, `TASK-E03-08`, `TASK-E03-09`, `TASK-E05-05` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | médio |

## Entrega

Tag adapters, multivalue metadata, thumbnails/cache e cover precedence.

## Boundary

- **Não** escreve tags de volta no arquivo nem grava capítulos em lote; write-back é TASK-E03-08, e quem gravar aqui corrompe o arquivo com um blob de unknown fields que esta TASK só preserva, não interpreta.
- **Não** indexa os campos lidos em FTS nem monta as views de álbum e artista; a consulta indexada é TASK-E03-05, e criar índice textual aqui duplicaria o schema de busca que ela vai definir.
- **Não** baixa capa remota de feed de podcast nem resolve cover precedence entre episódios; feed e episódios são TASK-E05-05, e a precedência desta TASK vale só para imagem embutida e arquivo de capa local.
- **Não** varre diretórios nem decide quando uma faixa entra no índice; scan job e watcher são TASK-E03-03, e o adapter é chamado com um path já conhecido, sem descobrir arquivo.
- **Não** deduplica itens que compartilham a mesma capa ou o mesmo hash de file; dedupe é TASK-E08-04, e o cache de thumbnail aqui é chaveado pelo conteúdo da imagem, não pelo acervo.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e03-04-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E03-04-AC-1` | um FLAC com Vorbis Comment contendo o par não mapeado MYCUSTOMFIELD=Vynamp e um APE tag cru anexado, mais um M4A com átomo freeform ----:com.apple.iTunes:iTunNORM de 32 bytes | o TagAdapter lê cada arquivo, o registro é persistido no store e a mesma faixa é lida de volta | Unknown fields preservados para futuro write |
| `TASK-E03-04-AC-2` | um PNG de capa embutido que declara 65535 x 65535 pixels em 4 KB comprimidos (deflate bomb) e um JPEG que declara 30000 x 30000 pixels, anexados como capa de dois FLACs válidos | o cover extractor é chamado com os limites configurados de 4096 px por lado e 16 MiB de imagem decodificada | image bomb limitada |
| `TASK-E03-04-AC-3` | um FLAC com TITLE não sei dançar em NFC, ALBUMARTIST Müller com trema combinante (NFD, dois code points) e ARTIST contendo o caractere fora do BMP 𝄞 | o tag reader lê o arquivo e o registro é gravado no SQLite e relido em outro processo | metadata Unicode mantém representação original. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test -p vynamp_player --test tag_read com as fixtures tests/fixtures/tags/unknown-fields.flac, unknown-fields.m4a, bomb.png e bomb.jpg, mais um teste de round-trip que grava e relê as três strings Unicode | espera-se o blob de unknown fields byte-idêntico ao original nos dois formatos (comparação de bytes, não de chave), a capa rejeitada com CoverTooLarge antes de qualquer alocação acima de 16 MiB e RSS do processo abaixo de 64 MiB durante os dois bomb (contra 17 GB se 65535 x 65535 x 4 bytes fosse decodificado), e TITLE, ALBUMARTIST e ARTIST lidos de volta code point a code point iguais aos do arquivo. Hoje o repo vynamp-player tem 1 arquivo .rs (src/main.rs) com println!("vynamp_player 0.1.0"), zero dependências e nenhum tests/, então nenhum adapter e nenhum teste existem. |
| Hardware | nenhum: as fixtures são arquivos estáticos com tags e imagens embutidas e nenhum teste abre dispositivo de áudio | ausência é limitação registrada, não aprovação |

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
