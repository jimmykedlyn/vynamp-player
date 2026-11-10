# TASK-E17-11 — Implementar política de proxy e rede isolada

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E17-11` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E17-11` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E05-01` |
| Bloqueia | `TASK-E17-20` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Adapters system/manual/direct, vault, CONNECT/TLS e fixtures 407/DNS/redirect.

## Boundary

- **Não** implementa o adapter HTTP que executa o redirect, a autenticação de origem, o timeout e o cancelamento de cada hop. Isso é TASK-E05-02; a confusão é plausível porque o AC-3 exercita um 302 real e o AC-4 cancela um CONNECT em voo, mas a mecânica do hop é do adapter e o que esta TASK decide é a política aplicada a cada hop.
- **Não** entrega a política por URL filha em forma geral, o corpus completo de falhas nem o painel de saúde sanitizado do streaming. Isso é TASK-E05-09; manifests, segmentos e keys do AC-3 atravessam a mesma decisão de origem, e trazer a taxonomia inteira de falhas para cá deixaria dois donos do mesmo diagnóstico.
- **Não** define o SourceDescriptor nem o provider simulado que a política consome. Isso é TASK-E05-01, dependência declarada; é tentador começar pelo descriptor para ter contra o que testar, mas implementá-lo aqui inverte a dependência e duplica o contrato.
- **Não** implementa PAC, WPAD nem SOCKS5. A spec os declara capacidades opcionais de M5 e nenhuma TASK do backlog os possui hoje; quem declara por artefato o que de fato existe é TASK-E17-12, dona do manifest de capacidades, e anunciar PAC aqui sem PAC seria exatamente a promessa que aquele manifest tem de impedir, com a agravante de a spec exigir que PAC indisponível não caia em silêncio para direct.
- **Não** implementa OAuth refresh, direitos nem o catálogo do provider. Isso é TASK-E05-10; a separação de namespaces do AC-2 toca o token do provider, mas renovar token e resolver direito é do provider, e fazê-lo aqui criaria duas TASKs escrevendo no mesmo vault.
- **Não** fecha o gate combinado de codecs, DSP e visualizadores. Isso é TASK-E17-20, que esta TASK bloqueia; as fixtures de 407, DNS e redirect são insumo do gate, e declarar o gate fechado aqui aprovaria cobertura que ainda não existe.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e17-11-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E17-11-AC-1` | perfil manual apontando para o proxy de fixture em 127.0.0.1:18080, que responde 407 a todo CONNECT que chegue sem Proxy-Authorization, origem em 127.0.0.1:18081 que conta cada conexão TCP aceita, URL http://media.fixture.invalid/live.flac e política de retry com limite de 2 e backoff de 250 ms, sem nenhuma escolha do perfil direct registrada na sessão | o usuário inicia o playback da URL com o perfil manual e não toca em nenhum controle enquanto as 2 tentativas da política se esgotam contra o 407 | Proxy failure não cai direto |
| `TASK-E17-11-AC-2` | vault desbloqueado com três entradas em namespaces distintos (origin bearer orig-tok-1, proxy com usuário px-user e senha px-pass, provider API key prov-key-1), um proxy de fixture exigindo Basic de proxy, origem em 127.0.0.1:18081 que ecoa todo header recebido e log em nível debug aberto | o stream é aberto pela política manual e o CONNECT atravessa o fixture de proxy com o Basic aceito, sem que o player peça credencial ao usuário | credenciais origin/proxy/provider separadas |
| `TASK-E17-11-AC-3` | origem em 127.0.0.1:18081 que responde 302 de http://cdn.fixture.invalid/master.m3u8 para http://edge.fixture.invalid/master.m3u8, manifest que referencia o segmento http://edge2.fixture.invalid/seg1.ts e a key http://keys.fixture.invalid/k1, e bypass explícito configurado apenas para o NAS 10.0.0.5:445 | o usuário abre a URL do manifest e a política resolve, na ordem, o 302, o manifest, o primeiro segmento, a key e por fim a conexão ao NAS 10.0.0.5:445 | redirects/filhos respeitam política |
| `TASK-E17-11-AC-4` | proxy de fixture que aceita o CONNECT e trava o handshake TLS por 30 s, política com limite de 2 tentativas e backoff de 250 ms, sessão tocando há 40 s com 1 job de análise ativo, contador de tentativas exposto no log e perfil system com adapter de fixture validado | o usuário cancela a reprodução com o CONNECT ainda em voo e, em seguida, troca o perfil de manual para system | cancel/reconnect bounded. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test --manifest-path apps/player/Cargo.toml --locked --test proxy_policy, rodado da raiz do umbrella, sobre apps/player/tests/proxy_policy.rs novo, com dois servidores de loopback em processo: proxy CONNECT em 127.0.0.1:18080 que responde 407 ao CONNECT sem Proxy-Authorization e 200 depois do Basic correto, e origem em 127.0.0.1:18081 que grava cada requisição, cada header e cada conexão TCP aceita | Esperado: no AC-1 o contador de conexões TCP da origem em 18081 fica em 0, o erro exposto recebe a variante de falha de proxy, distinta de 401 e de 403, e o perfil registrado no log continua manual; Proxy-Authorization aparece em 100 por cento dos CONNECT que o proxy de fixture recebe e em 0 das requisições que a origem grava, e nem orig-tok-1 nem prov-key-1 aparecem no fixture de proxy; no AC-3 o host do segmento fora da política gera 1 negação e 0 requisições, o host do manifest já autorizado gera 1 CONNECT e a conexão a 10.0.0.5:445 sai no bypass, sem CONNECT; no AC-4 o cancel fecha o CONNECT pendurado em menos de 100 ms e o reconnect após a troca de manual para system conta exatamente 2 tentativas no log, sem novo job de análise e sem reabrir o áudio em 0 s. Nada disso existe hoje: o repo apps/player tem um único arquivo .rs, src/main.rs, com só println!("vynamp_player 0.1.0"), zero dependências, nenhum diretório tests/ e o pacote se chama vynamp_player com underscore, então o alvo proxy_policy não existe, o teste não compila e nenhuma dessas contagens foi medida; o próprio cargo responde error: no test target named proxy_policy in default-run packages. |
| Hardware | nenhum: a prova usa só dois servidores de loopback em 127.0.0.1 e um trust store de teste em arquivo; este ambiente é Windows 11 x86_64 sem placa de áudio profissional, sem aparelho Android, sem aparelho iOS e sem máquina macOS, e nenhum AC toca ASIO, CoreAudio ou WASAPI exclusivo — o perfil system do AC-4 usa o adapter de fixture declarado no Given, não o WinHTTP real da máquina. | ausência é limitação registrada, não aprovação |

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
