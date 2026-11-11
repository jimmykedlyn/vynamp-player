# TASK-E05-01 — Implementar SourceDescriptor e provider simulado

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E05-01` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E05-01` |
| Critérios da spec | `specs/streaming.md` §5 (rascunho, não aprovado) |
| Fase | M2 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E01-03` |
| Bloqueia | `TASK-E05-02`, `TASK-E09-02`, `TASK-E17-11` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | alto |

## Entrega

Porta de resolução, descritor versionado e provider de teste sem dependência externa.

## Boundary

- **Não** implementa o cliente HTTP com Range, redirect e timeout; isso é TASK-E05-02, e trazer a pilha de rede para cá faria o descritor depender de I/O real antes de existir a porta de rede que ele descreve.
- **Não** faz o parse de ICY nem separa metadados de rádio dos bytes de áudio; é TASK-E05-03, e aqui live significa apenas ausência de duração e de seek, não saber ler icy-metaint.
- **Não** aplica a política de proxy e rede isolada contra loopback, link-local e metadata endpoint; é TASK-E17-11, e neste escopo ela aparece só como campo de restrictions do descritor, sem enforcement.
- **Não** implementa a state machine de player nem o cancelamento por generation; é TASK-E01-03, e o provider simulado aqui é síncrono e não tem ciclo de vida.
- **Não** publica o contrato OpenAPI v1 do resolvedor nem os contract tests; é TASK-E09-02, e o schema lá é consumidor deste descritor, não parte desta entrega.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e05-01-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E05-01-AC-1` | três fontes registradas no provider simulado de vynamp-player: um FLAC de 44,1 kHz e 16 bits com 180 s e 63 400 000 bytes, um stream ICY contínuo sem Content-Length e uma playlist VOD com 12 segmentos de 6 s | o resolver devolve um SourceDescriptor para cada uma das três URLs e os três descritores são serializados em JSON pelo teste | Distinguir capacidades de arquivo, live e VOD |
| `TASK-E05-01-AC-2` | um source cujo descriptor carrega signed query com token=abc123 e um header Authorization Bearer fornecido pelo provider, com log no nível trace capturado em buffer de teste | o descritor é serializado para a playlist e cada linha de log do caminho de resolução é comparada contra a substring do token | nenhum token em playlist/log |
| `TASK-E05-01-AC-3` | duas URLs com transporte que o provider simulado não registra, gopher://radio.exemplo:70/live e rtsp://exemplo/live.mp3, mais uma terceira com esquema file:///tmp/a.flac | o resolver é chamado e o erro devolvido é mapeado para a taxonomia pública de suporte | transporte desconhecido retorna erro de suporte. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | provider simulado em vynamp-player resolvendo três fixtures locais — FLAC de 180 s e 63 400 000 bytes, stream ICY sem Content-Length e playlist VOD de 12 segmentos — em tests/source_descriptor.rs, rodado com cargo test --test source_descriptor | os três descritores saem com seekable, duration e live window distintos, os esquemas gopher, rtsp e file devolvem erro de transporte não suportado e o JSON nem o buffer de log contêm token=abc123; hoje o repo vynamp-player tem 1 arquivo .rs (src/main.rs com println de vynamp_player 0.1.0), zero dependências e nenhum tests/, então nenhuma dessas asserções existe ainda. |
| Hardware | nenhum: o provider é simulado em memória e os testes não abrem socket, dispositivo de áudio nem placa de rede real | ausência é limitação registrada, não aprovação |

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
