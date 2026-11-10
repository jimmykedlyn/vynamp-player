# TASK-E05-05 — Entregar feeds e episódios de podcasts

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E05-05` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E05-05` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E05-04`, `TASK-E03-04` |
| Bloqueia | `TASK-E13-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | alto |

## Entrega

RSS/Atom, assinaturas, polling condicional, downloads e progresso por episódio.

## Boundary

- **Não** implementa o parser ICY nem o bloco de metadados da rádio: isso é TASK-E05-03, e a confusão é plausível porque as duas TASKs fazem polling HTTP contínuo de uma URL escolhida pelo usuário sobre o mesmo transporte.
- **Não** define o cache LRU em disco de 2 GiB nem a separação entre cache e downloads: é TASK-E05-04, e quem implementa download de episódio tende a encostar a política de expiração e ETag do cache aqui.
- **Não** implementa redirect, autenticação nem timeouts do transporte HTTP: são TASK-E05-02, e o importador de feed parece o lugar natural para tratar 301 e Authorization porque é o primeiro a buscar uma URL digitada pelo usuário.
- **Não** lê tags nem capas dos episódios baixados: o adapter de tags, o multivalue e a precedência de cover são TASK-E03-04, e o episódio já chega com itunes:image no XML, o que sugere que a capa se resolve nesta TASK.
- **Não** entrega a integração Android de podcast e downloads: é TASK-E13-06, que esta TASK bloqueia, e a tentação é fechar as duas juntas porque nenhuma cláusula de aceite aqui nomeia plataforma.
- **Não** implementa o progresso de conteúdo longo da API da plataforma: é TASK-E05-10, e o termo progresso por episódio da entrega puxa o executor para lá.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e05-05-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E05-05-AC-1` | fixture feed_xxe.xml (RSS 2.0, 412 bytes) com DOCTYPE que declara ENTITY xxe SYSTEM file:///C:/Windows/win.ini e usa &xxe; em title e em description, ao lado de uma entidade interna legítima declarada no mesmo DOCTYPE | o importador de assinaturas ingere a fixture e o leitor de metadados do episódio é chamado sobre o XML resultante | Desabilitar entidades XML externas |
| `TASK-E05-05-AC-2` | duas fixtures RSS válidas, feedA.example/rss e feedB.example/rss, que repetem o mesmo <guid isPermaLink="false">urn:episode:0001</guid> e diferem só no host e no título | as duas assinaturas são importadas na mesma biblioteca e o player pede a lista de episódios ordenada por data | GUID por feed |
| `TASK-E05-05-AC-3` | episódio de 18 MiB servido por HTTP local em blocos de 256 KiB, com 5 242 880 bytes já gravados em downloads/ep-0042.mp3.part e o download em andamento | o usuário aciona cancelar na control thread e em seguida manda retomar o mesmo episódio | cancelar download preserva parcial válido |
| `TASK-E05-05-AC-4` | download de 12,5 MiB emitindo DownloadProgress a cada 256 KiB, ou seja 50 eventos de seq 1 a 50, com um coletor que persiste (download_id, seq, bytes) na ordem de chegada | a inscrição no canal de progresso é refeita após queda e o coletor recebe de novo os eventos de seq 12 a 31 | eventos de progresso são idempotentes. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cd apps/player e cargo test --test podcast_feeds contra tests/fixtures/feeds/ (feed_xxe.xml, same_guid_two_feeds.xml, episode_18mib.mp3 servido por Range local em blocos de 256 KiB), mais inspeção de downloads/ep-0042.mp3.part e do log de metadados | Hoje o repo vynamp-player tem 1 arquivo .rs (src/main.rs com println!("vynamp_player 0.1.0")), zero dependências e nenhum tests/, então o comando falha com error: no test target named podcast_feeds in default-run packages. Depois: 4 testes; 0 ocorrências de win.ini e 0 de propriedade SYSTEM na saída do parser, com erro tipado ExternalEntityBlocked; 2 linhas em episodes para o guid urn:episode:0001, chaveadas por (feedA, urn:episode:0001) e (feedB, urn:episode:0001); .part com exatamente 5 242 880 bytes, isto é 20 blocos de 262 144 bytes, e resume por Range devolvendo 206 com Content-Range começando em 5242880; 50 eventos de seq 1 a 50 gerando 50 registros distintos e 0 duplicados quando 20 deles são reentregues. |
| Hardware | nenhum: o corpus é XML e um MP3 de 18 MiB servido por HTTP local com Range; nenhum teste abre dispositivo de áudio, placa ASIO, aparelho Android, aparelho iOS ou máquina macOS. | ausência é limitação registrada, não aprovação |

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
