# TASK-E05-02 — Entregar HTTP Range e reprodução progressiva

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E05-02` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E05-02` |
| Critérios da spec | `specs/streaming.md` §5 (rascunho, não aprovado) |
| Fase | M2 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E05-01`, `TASK-E01-07` |
| Bloqueia | `TASK-E05-03`, `TASK-E05-04`, `TASK-E07-03` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | alto |

## Entrega

Adapter HTTP com redirects, autenticação, cancelamento, timeouts e seek negociado.

## Boundary

- **Não** implementa ICY, rádio nem indicador LIVE; é TASK-E05-03, e o Range aqui é transferência finita, sem metaint nem metadados intercalados.
- **Não** define os limites de 32 MiB codificados e 16 MiB decodificados, o cache LRU em disco nem downloads offline; é TASK-E05-04, e aqui o Range entrega bytes sem política de retenção.
- **Não** implementa a state machine de player nem o cancelamento por generation; é TASK-E01-03, e Stop aqui é só o sinal que aquela máquina emite, não a máquina.
- **Não** adiciona codecs nem demuxer para os bytes recebidos; é TASK-E01-07, e o adapter entrega bytes crus, não amostras decodificadas.
- **Não** decide autorização de header por origem via broker de permissões; é TASK-E07-03, e aqui a regra é a mínima de não reenviar Authorization nem aceitar downgrade tokenizado para http.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e05-02-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E05-02-AC-1` | servidor de fixture em TcpListener efêmero servindo um MP3 CBR de 128 kbps com 600 000 bytes, com três rotas: 206 e Content-Range para Range válido, 200 sem Accept-Ranges quando o Range é ignorado e 416 com Content-Range */600000 | o adapter HTTP abre a fonte, pede os bytes 0-65535 e depois os bytes 600000-700000, e registra status, offset e tamanho do corpo de cada resposta | Testar respostas 206/200/416 |
| `TASK-E05-02-AC-2` | URL inicial em https://cdn.exemplo/a.mp3 com header Authorization Bearer e redirect 302 para https://outro-cdn.exemplo/b.mp3 de origem diferente, mais um segundo caso de 302 para http://cdn.exemplo/b.mp3 | o adapter segue os dois redirects e o servidor de fixture grava os headers recebidos em cada uma das quatro requisições | não encaminhar auth para outra origem |
| `TASK-E05-02-AC-3` | reprodução de um MP3 remoto com 2 MiB já no buffer decodificado e uma requisição Range em voo, com o worker de rede e o decoder ativos e Stop acionado na control thread | Stop é chamado e o teste espera o join do worker de rede e do decoder por até 2 s antes de inspecionar os buffers | Stop cancela rede e decode e descarta buffers antigos. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | servidor de fixture em processo com TcpListener em porta efêmera e cinco rotas — 206 com Content-Range, 200 sem Accept-Ranges, 416 com Content-Range */600000, 302 cross-origin e 302 para http — exercitado por cargo test --test http_range | as três respostas geram estados distintos: 206 com offset 0 e 65 536 bytes lidos, 200 tratado como handled/restart e nunca como 206, 416 como seek indisponível; o servidor registra zero header Authorization na segunda origem e zero requisição após o Stop, que fecha o socket em menos de 2 s; hoje o repo vynamp-player tem 1 arquivo .rs com println, zero dependências e nenhum tests/, então nenhuma dessas rotas nem asserções existe. |
| Hardware | nenhum: os fixtures são um TcpListener local em porta efêmera e nenhum teste toca placa de áudio ou rede externa | ausência é limitação registrada, não aprovação |

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
