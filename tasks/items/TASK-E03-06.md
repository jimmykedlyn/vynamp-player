# TASK-E03-06 — Entregar playlists import/export e CUE

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E03-06` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E03-06` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E03-02`, `TASK-E03-04` |
| Bloqueia | `TASK-E03-07` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | médio |

## Entrega

M3U/M3U8/PLS/XSPF/CUE, relative locators e virtual ranges.

## Boundary

- **Não** implementa smart playlists nem o AST allowlisted de filtro; a smart playlist é TASK-E03-07, e uma playlist importada aqui é lista estática de locators, sem regra que se reavalie.
- **Não** implementa a fila, o shuffle nem a persistência de QueueSession; fila e shuffle são TASK-E03-02, e o importador entrega entradas de playlist, não o estado de reprodução.
- **Não** escreve tags nos arquivos nem converte o CUE em arquivos separados no disco; escrita em lote é TASK-E03-08, e aqui o range CUE é virtual por cima do mesmo media asset.
- **Não** faz playback nem seek dentro dos virtual ranges; quem consome os offsets é a fila de TASK-E03-02, e esta TASK só prova que os offsets em 75 fps estão corretos.
- **Não** deduplica nem reorganiza os arquivos referenciados por uma playlist; organização e dedupe são TASK-E08-04, e mover o arquivo por baixo de um locator relativo é exatamente o que a exportação relativa evita.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e03-06-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E03-06-AC-1` | quatro fixtures no mesmo diretório: a.m3u8 com 2 faixas FLAC próprias e uma referência a b.m3u8, b.m3u8 com 1 faixa própria e uma referência de volta a a.m3u8, self.m3u8 com 1 faixa e uma referência a si mesma, e uma cadeia de 40 playlists encadeadas cujo último elo tem 5 faixas e nada referencia de volta | o importador resolve as referências a partir de a.m3u8, de self.m3u8 e do topo da cadeia de 40, registrando por import as entradas materializadas, a profundidade alcançada e o erro final | Loop de playlists bloqueado |
| `TASK-E03-06-AC-2` | um CUE sheet com 6 TRACKs sobre um FLAC de 44,1 kHz, com INDEX 00 03:27:22 e INDEX 01 03:27:37 na TRACK 02, TRACK 03 começando em 03:45:07 e os três TRACKs restantes em offsets também escritos em frames de 1/75 s que não são múltiplos de 1/100 s | o importador cria os virtual ranges e cada faixa é aberta pelo range, com offset de início, offset de fim e duração reportados | CUE75fps preserva offsets |
| `TASK-E03-06-AC-3` | uma playlist de 150 entradas cujos paths contêm &, # e nomes em japonês, com um token de sessão de 32 bytes presente no estado em memória do player | o usuário exporta a playlist para .m3u8, .pls e .xspf em D:\exports\ e os três arquivos são lidos de volta byte a byte | export sem token |
| `TASK-E03-06-AC-4` | uma playlist com 200 entradas e revision 0, aberta na sessão atual | o harness aplica 100 operações de inserir, remover e reordenar, depois 100 undos seguidos de 100 redos, e compara o estado após cada passo | undo/redo100 ops da sessão. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test -p vynamp_player --test playlist_io com as fixtures tests/fixtures/playlists/loop (a, b, self e a cadeia de 40), tests/fixtures/cue/six-tracks.cue sobre um asset com duração em frames declarada, e um log de 100 operações de edição reaplicado em undo e redo com o estado serializado gravado a cada passo | espera-se, partindo de a.m3u8, exatamente 3 entradas materializadas (2 de a mais 1 de b) e Cycle nomeando a.m3u8 e b.m3u8, sem segunda passagem por a; self.m3u8 devolve 1 entrada e Cycle nomeando ele próprio; a cadeia de 40 devolve as 5 faixas do último elo com profundidade 40, dentro do limite declarado, sem estouro de pilha; TRACK 02 com INDEX 01 em 15562 frames de 1/75 s, que é 9.150.456 samples a 588 samples por frame, e TRACK 03 em 16882 frames, que é 9.926.616 samples, idênticos ao literal e sem arredondar para milissegundo, com a soma das 6 durações igual à duração total do asset a menos de 588 samples; os três arquivos exportados com 0 ocorrências do valor do token e nenhum cabeçalho de autorização, e reimportáveis com os mesmos 150 locators; 100 undos levando revision de 100 a 0 com as 200 entradas na ordem original e 100 redos voltando a revision 100. Hoje o repo vynamp-player tem 1 arquivo .rs (src/main.rs) com println!("vynamp_player 0.1.0"), zero dependências, nenhum tests/ e nenhum parser de M3U, PLS, XSPF ou CUE, então nada disso existe nem roda. O Then do AC-3 diz export sem token sem definir o token: a leitura adotada é credencial de acesso, e qual token é esse precisa ser registrado em spec antes de a TASK sair de planned. |
| Hardware | nenhum: playlists e CUE são texto e o asset apontado pelo CUE entra só como registro com contagem de frames declarada, sem decodificar áudio, sem placa de áudio no caminho e sem nenhum aparelho externo | ausência é limitação registrada, não aprovação |

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
