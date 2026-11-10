# TASK-E12-09 — Entregar casting e companion

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E12-09` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E12-09` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M5 |
| Prioridade | P2 |
| Estado | planned |
| Depende de | `TASK-E08-10`, `TASK-E09-10` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 10–25 dias úteis |
| Risco | alto |

## Entrega

Bridges Chromecast/UPnP/AirPlay conforme APIs e controles pareados.

## Boundary

- **Não** entrega a API de controle remoto, o pairing nem a rotação e revogação de token. Isso é `TASK-E08-10`, que esta TASK depende; aqui o companion consome a sessão pareada que aquele contrato expõe, e reimplementar o pairing criaria duas autoridades sobre o mesmo token.
- **Não** escreve os bindings de saída nem declara capability de device: host de saída, device profiles e capabilities são `TASK-E04-05`, e o caminho integer direct é `TASK-E04-06`. A linha 52 de `libs/output/specs/output.md` diz literalmente que casting e AirPlay são do player (`E12-09`) e que seleção de faixa, volume e EQ são da output; esta TASK negocia com o destino remoto e não toca no host local.
- **Não** decide ou promete hi-res, bit-perfect nem passthrough remoto. A promoção de malha de capacidade é `TASK-E04-10` e a verificação numérica de cadeia é `TASK-E04-06`; aqui o que existe é o que o destino declara, e alegação de qualidade depende do oráculo daquelas TASKs.
- **Não** entrega DASH, WebDAV ou clouds. A fonte remota que alimenta o cast é `TASK-E12-08`; confundir os dois faria a mesma capability de transferência ser negociada em dois lugares, uma pelo manifesto e outra pela sessão de cast.
- **Não** implementa o transcoder que o receiver CAST exige quando não aceita a taxa da fonte, nem o encoder AAC que ele implica. A lib de codificadores é `TASK-E18-02` e o pipeline de jobs é `TASK-E09-05`; a confusão é plausível porque o AC-1 manda um FLAC 88,2 kHz a um destino que só decodifica AAC 48 kHz e o caminho mais curto parece ser codificar aqui, mas isso poria um encoder de áudio dentro do cliente de cast e um segundo dono do mesmo codec.
- **Não** mede carga nem disponibilidade de serviço de plataforma. QA-014 e o teste de 1.000 sessões são `TASK-E09-10`, que bloqueia esta TASK; p95 de API verde não é prova de sessão de cast correta nem de cliente não pareado recusado.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e12-09-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E12-09-AC-1` | um receiver DIAL/CAST v2 emulado no servidor de fixtures que anuncia 48 kHz estéreo, AAC com teto de 256 000 bit/s e nenhum decoder FLAC, ao lado de um renderer UPnP AVTransport que anuncia 96 kHz e FLAC e de um destino AirPlay 2 que só aceita ALAC 44,1 kHz pelo bridge, e uma faixa FLAC 88,2 kHz/24 bits de 30 s no catálogo local | a UI de casting lista os três destinos, o usuário escolhe cada um e dispara a mesma faixa nos três, e o que é oferecido e o que é enviado aparecem no painel de status de cada sessão | Capacidade real do destino |
| `TASK-E12-09-AC-2` | o mesmo receiver CAST v2 emulado, agora com o relógio de mídia do protocolo atrasado 240 ms em relação ao relógio do host e o canal de transferência limitado a 512 kbit/s, mais um renderer UPnP que responde a `GetTransportInfo` com 1,8 s de atraso | uma sessão de 90 s de áudio é tocada com seek aos 30 s e aos 60 s e o painel de sessão é lido durante e depois de cada seek | limites de clock/transferência visíveis |
| `TASK-E12-09-AC-3` | o companion mobile emulado, um par de chaves já pareado e guardado no keychain de teste, mais um segundo cliente que envia comandos de play e seek sem ter feito pairing e outro cujo token pareado foi revogado na sessão anterior | os três clientes abrem o WebSocket do companion e enviam a mesma sequência de comandos de transporte, e a resposta e o estado da fila são lidos no host depois de cada um | negar cliente não pareado |
| `TASK-E12-09-AC-4` | um destino AirPlay 2 e um receiver CAST v2 emulados que declaram responder a `AudioCapabilities` sem informar `maxSampleRate` nem profundidade de bits, e uma faixa FLAC 192 kHz/24 bits na fila do host | a sessão de casting é aberta e a UI de status de destino é inspecionada nos dois casos, comparada ao que o host toca localmente nos mesmos 30 s | sem claims hi-res não verificados. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | os emuladores de protocolo em processo (DIAL/CAST v2, SSDP com UPnP AVTransport e AirPlay 2) no servidor de fixtures adversário e, para o pairing, três clientes WebSocket de teste, exercitados por `cargo test` no repo `vynamp-player`; hoje esse repo tem um único `.rs` com `println!("vynamp_player 0.1.0")`, zero dependências e um `cargo test` que imprime `0 passed`, `tests/fixtures/network/` do umbrella é linha de tabela em `tests/fixtures/README.md` e a pasta não existe, nenhum emulador DIAL, SSDP ou AirPlay 2 está versionado em qualquer repo, e `TASK-E08-10`, que esta TASK depende, ainda está `planned` com os 4 marcadores de template do passo 37 no arquivo | Esperado: em AC-1 o painel do receiver CAST v2 tem de declarar AAC 48 kHz com teto de 256 000 bit/s e a sessão não pode anunciar a faixa como FLAC 88,2 kHz direto, enquanto no renderer UPnP a mesma faixa tem de ir sem resample — taxa 88 200 Hz e codec FLAC lidos do header do corpo de mídia que o emulador grava, não do log da UI — e no AirPlay 2 o painel tem de declarar ALAC 44,1 kHz; onde a taxa da fonte não cabe, a sessão é recusada com motivo, nunca rebaixada em silêncio, e a produção do AAC recodificado depende de `TASK-E18-02`, hoje só uma constante `VERSION` em `libs/encode/src/lib.rs`. Em AC-2, o atraso de 240 ms aparece como valor de clock e os 512 kbit/s como limite de transferência, e os seeks aos 30 s e 60 s retomam na posição pedida com desvio em ms medido no emulador. Em AC-3, o cliente sem pairing e o de token revogado recebem recusa e o estado da fila do host não muda, enquanto o pareado altera — contado no log do host. Em AC-4, nenhuma tela mostra hi-res quando o destino não declarou `maxSampleRate`, e o texto reflete a taxa real do arquivo enviado. Nada disso é medível hoje: não há cliente de cast, nem emulador, nem a API de `TASK-E08-10`. |
| Hardware | nenhum para os emuladores de protocolo, que rodam em processo local; mas o critério de capacidade real do destino exige hardware físico ausente nesta bancada: Chromecast ou Google TV, receiver UPnP/DLNA e Apple TV ou HomePod para AirPlay 2 — este ambiente é Windows 11 x86_64 sem placa pro, sem Android, sem iOS e sem macOS, então discovery real, handshake AirPlay 2 e limite de clock de um receiver físico ficam registrados como limitação, e o que se prova é o contrato contra emulador | ausência é limitação registrada, não aprovação |

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
