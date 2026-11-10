# TASK-E05-06 — Entregar HLS sob demanda

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E05-06` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E05-06` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E05-04`, `TASK-E01-07` |
| Bloqueia | `TASK-E05-07` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Parser master/media, segmentos, discontinuity, byte ranges e AES-128 autorizado.

## Boundary

- **Não** implementa janela live, ABR por throughput nem ring de timeshift: é TASK-E05-07, que esta TASK bloqueia, e a confusão é plausível porque o parser de media playlist é o mesmo código nos dois casos e EXT-X-MEDIA-SEQUENCE já aparece na fixture VOD.
- **Não** valida nem decodifica AAC-LC ou HE-AAC dentro de TS, fMP4 ou ADTS: os adapters de codec são TASK-E01-07, e a cláusula tratar segmentos ausentes parece exigir decodificar o segmento para saber se ele serve.
- **Não** aplica a política de rede por URL filha, o corpus de falhas nem o painel de saúde sanitizado: isso é TASK-E05-09, e o parser de manifest é quem primeiro enxerga as URLs filhas, o que sugere que a política nasce aqui.
- **Não** define o cache LRU em disco, o cap de 2 GiB nem a separação cache/downloads: é TASK-E05-04, e byte ranges de segmento convidam a guardar pedaços no cache desta TASK.
- **Não** decide seekability, duração e live window no SourceDescriptor: o descritor versionado e o provider simulado são TASK-E05-01, e seek preserva timeline soa como pedido para mexer no descritor em vez de na timeline do loader.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e05-06-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E05-06-AC-1` | quatro fixtures em tests/fixtures/hls/: master_nest4.m3u8 com 4 níveis de aninhamento, manifest_2_5mib.m3u8 com 2 621 440 bytes, entries_20001.m3u8 com 20 001 linhas EXTINF e um master válido com 3 variants | cada fixture é entregue ao parser incremental de manifest e o resultado de cada chamada é coletado | Limitar manifests inválidos/aninhados |
| `TASK-E05-06-AC-2` | media playlist VOD de 12 segmentos de 4,0 s em que o segmento 7 responde 404 e os segmentos 6 e 8 respondem 200 com body de 96 KiB | a reprodução roda do início ao fim e, a cada falha de segmento, o loader refaz a requisição da playlist | tratar segmentos ausentes |
| `TASK-E05-06-AC-3` | media playlist VOD de 600,0 s em 150 segmentos de 4,0 s, EXT-X-MEDIA-SEQUENCE começando em 1432 e EXT-X-DISCONTINUITY em 180,0 s e 420,0 s | o usuário arrasta a barra para 300,0 s, depois 420,0 s, depois 60,0 s e deixa tocar 5 s em cada posição | seek preserva timeline |
| `TASK-E05-06-AC-4` | aes128_token.m3u8 com EXT-X-KEY METHOD=AES-128 e URI https://keys.example/k?id=abc&token=SEGREDO123, segmentos em https://origin.example/v/segN.ts?sig=XYZ, e o player em log nível trace escrevendo em arquivo local | o VOD é aberto, a chave e os 12 segmentos são baixados até o fim, e o arquivo de log e o cache de descriptor são varridos | não registrar chaves ou URLs secretas. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test -p vynamp-player --test hls_vod contra tests/fixtures/hls/ (master_nest4.m3u8, manifest_2_5mib.m3u8, entries_20001.m3u8, vod_600s_discontinuity.m3u8, missing_seg7/ e aes128_token.m3u8) mais grep no arquivo de log trace gerado pelo próprio teste | Hoje o repo vynamp-player tem 1 arquivo .rs e nenhum tests/, então o comando falha com error: no test target named hls_vod. Depois: 3 recusas tipadas citando o limite medido na mensagem (4 níveis contra 3, 2 621 440 bytes contra 2 097 152, 20 001 entries contra 20 000) e 1 aceite; o segmento 7 ausente pula 4 000 ms sem deslocar os 5 seguintes; os seeks para 300,0 s, 420,0 s e 60,0 s produzem 0 ms de salto indevido, com 150 segmentos de PTS estritamente crescente exceto os 2 discontinuity declarados; 0 ocorrências de SEGREDO123 e 0 de sig=XYZ no arquivo de log. |
| Hardware | nenhum: manifests e segmentos são fixtures de arquivo em tests/fixtures/hls/ e a timeline é medida em PTS do contêiner, sem abrir saída de áudio nem dispositivo de plataforma. | ausência é limitação registrada, não aprovação |

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
