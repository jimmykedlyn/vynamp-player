# TASK-E12-08 — Entregar DASH, WebDAV e clouds

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E12-08` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E12-08` |
| Critérios da spec | `specs/streaming.md` §5 (rascunho, não aprovado) |
| Fase | M5 |
| Prioridade | P2 |
| Estado | planned |
| Depende de | `TASK-E05-10`, `TASK-E09-10` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 10–25 dias úteis |
| Risco | alto |

## Entrega

Providers autorizados, DASH áudio, locators WebDAV e capability discovery.

## Boundary

- **Não** toca no parser de HLS, no ABR de HLS nem no timeshift: master e media playlist, discontinuity e janela deslizante são `TASK-E05-06` e `TASK-E05-07`. A confusão é plausível porque o DASH reusa as ideias de variant e de segmento, mas duplicar o seletor de rendição aqui daria dois ABR com políticas diferentes sobre o mesmo buffer.
- **Não** implementa OAuth, refresh token, sync de dados do ouvinte nem reconciliação de catálogo. Identidade e autorização são `TASK-E09-03` e o sync é `TASK-E09-08`; esta TASK consome o provider API que `TASK-E05-10` integra e não reimplementa o fluxo de login nem o cursor de sync.
- **Não** escreve a política de proxy, o bypass por CIDR nem a defesa contra DNS rebinding. Isso é `TASK-E17-11` e `TASK-E05-09`; aqui a root privada é uma entrada de locator que passa pela política existente, e mover a checagem de loopback para dentro do cliente WebDAV deixaria um segundo lugar onde a política de rede pode divergir.
- **Não** decodifica os codecs que o DASH anuncia nem decide o perfil de saída local. O binding de container e a seleção de faixa são `TASK-E17-03`, e a taxa e o formato que o dispositivo local aceita são `TASK-E04-05`; aqui se negocia só o que o manifesto remoto oferece, e deixar o manifesto decidir formatação de placa inverteria a autoridade das duas TASKs.
- **Não** licencia nem habilita Widevine, FairPlay ou PlayReady: decidir o que entra na build e por qual licença é `TASK-E11-03`, dona do inventário e da política de distribuição. `reference/audio/02-formats.md` marca `encrypted-drm` como `unsupported`, e o que esta TASK entrega é detecção e erro explícito, nunca remoção de proteção nem chave de teste promovida a suporte.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e12-08-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E12-08-AC-1` | um MPD de áudio com dois AdaptationSet por idioma (pt-BR e en), quatro Representations AAC-LC declaradas em 96 000, 160 000, 256 000 e 320 000 bit/s a 48 kHz dentro de um SegmentTemplate com SegmentTimeline de 2 s cobrindo 10 segmentos, um BaseURL do servidor de fixtures que responde 206 a Range e um bearer de provider com TTL de 60 s | o resolve abre o DASH e seleciona uma Representation pela política de throughput do cliente com 1 200 000 bit/s medidos, depois fixa à mão a de 160 000 bit/s, com o bearer renovado entre as duas escolhas e um seek para o meio do segmento 5 executado depois da renovação | Negociar auth/seek/codec/qualidade |
| `TASK-E12-08-AC-2` | três MPDs no mesmo servidor de fixtures: um com ContentProtection de schemeIdUri urn:uuid:e2719d58-a985-b3c9-781a-b030af78d30e (ClearKey), um com urn:uuid:9a04f079-9840-4286-ab92-e65be0885f95 (PlayReady) e um de controle sem nenhum ContentProtection | o resolve é chamado para abrir e selecionar a faixa de áudio dos três, nesta ordem, e o valor de retorno e o contador de GET do servidor de fixtures são lidos antes de qualquer segmento ser puxado | DRM desconhecido gera erro explícito |
| `TASK-E12-08-AC-3` | uma root WebDAV privada em https://192.168.18.28:5006/dav/musica com credenciais próprias, o perfil de rede em direct e sem bypass declarado para 192.168.18.28/32, mais um MPD público cujo BaseURL aponta para esse mesmo host privado | a root é adicionada pelo diálogo de fonte remota numa passada e, na outra, o BaseURL privado do MPD público é resolvido sem consentimento registrado para aquele host | roots privadas só com permissão. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | servidor de fixtures em processo local servindo os quatro MPDs (limpo, ClearKey, PlayReady e um com SegmentTimeline truncado no meio) mais um WebDAV mínimo em 192.168.18.28:5006, exercitados por `cargo test` no repo `vynamp-player`; hoje o repo tem um único `.rs`, `apps/player/src/main.rs` com 3 linhas e um `println!("vynamp_player 0.1.0")`, zero dependências em `Cargo.toml` e `cargo test` imprime `0 passed`, e `tests/fixtures/network/` do umbrella é só uma linha da tabela de `tests/fixtures/README.md`, pasta inexistente; nem `rclone` nem `davfs2` nem o módulo `wsgidav` estão instalados nesta bancada, medido com `which rclone davfs2` e `python -I -c "import wsgidav"`, então o servidor WebDAV tem de ser escrito junto do teste | Esperado: nos dois MPDs protegidos o retorno é erro de DRM não suportado carregando o schemeIdUri lido e o contador de GET do servidor de fixtures fica em 0 requisições de segmento — o gate é esse contador, não o texto da mensagem; a variante de erro tem de ser criada, porque hoje `vynamp-decode-core::Error` só tem `Unsupported(&'static str)`. No MPD limpo, com 1 200 000 bit/s medidos a escolha automática tem de ser a Representation de 320 000 bit/s declarados e a fixação manual na de 160 000 tem de aparecer na decisão registrada, e os 10 segmentos de 2 s a 48 kHz têm de somar 960 000 frames por canal com desvio 0 contra o declarado. No seek ao meio do segmento 5 (11 s) a requisição tem de sair com o bearer novo e o `Range` iniciado no offset de 11 s, não em 0, sem 401 e sem segundo GET do mesmo segmento. Na root WebDAV, a primeira passada aceita com consentimento registrado e a segunda é recusada com nada baixado, contado no log do servidor. Nada disso é medível hoje: não há cliente DASH nem locator WebDAV, nenhum MPD está versionado em repo algum e a pasta `tests/fixtures/network/` não existe. |
| Hardware | nenhum: DASH e WebDAV são protocolos de rede e os dois lados — cliente e servidor de fixtures — rodam em processo local nesta bancada Windows 11; o aparelho de destino que exigiria hardware é o casting, que é `TASK-E12-09`. A root WebDAV de teste é um processo local na própria máquina, não um NAS físico, então latência, fiação e comportamento de NAS real ficam sem prova | ausência é limitação registrada, não aprovação |

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
