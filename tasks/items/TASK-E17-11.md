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

- **Não** entrega a política por URL filha em geral, o corpus completo de falhas e o painel de saúde sanitizado do streaming. Isso é TASK-E05-09, e a confusão é plausível porque redirects, manifests, segmentos e keys atravessam a mesma decisão de origem descrita aqui; esta TASK implementa a eleição de perfil e o transporte proxy, não o endurecimento e o diagnóstico de todas as rotas.
- **Não** define o SourceDescriptor nem o provider simulado que a política consome. Isso é TASK-E05-01, dependência declarada; é tentador começar pelo descriptor para ter contra o que testar, mas implementá-lo aqui inverte a dependência e cria dois donos do mesmo contrato.
- **Não** implementa PAC, WPAD nem SOCKS5. `streaming/04-rede-proxy.md` os declara capacidades **opcionais de M5**, e nenhuma TASK do backlog os possui hoje: o que esta TASK entrega são os perfis system, manual e direct com adapter validado. Quem declara por artefato o que de fato existe é `TASK-E17-12`, dona do manifest de capacidades; anunciar PAC aqui sem PAC seria exatamente a promessa que aquele manifest tem de impedir. A spec também exige que PAC indisponível não caia em silêncio para direct.
- **Não** implementa OAuth refresh, direitos nem o catálogo do provider. Isso é TASK-E05-10; a separação de namespaces toca o token do provider, mas renovar token e resolver direito é do provider, e trazer isso para cá faria duas TASKs escreverem no mesmo vault.
- **Não** implementa rádio Icecast/SHOUTcast, ICY nem a reconexão de stream ao vivo. Isso é TASK-E05-03, e a proximidade é real porque a prova de cancel e reconnect aqui usa uma URL de stream; o objeto desta TASK é o limite de tentativas da politica de proxy, não o protocolo ICY.
- **Não** fecha o gate combinado de codecs, DSP e visualizadores. Isso é TASK-E17-20, que esta TASK bloqueia; as fixtures de 407, DNS e redirect são insumo do gate, e declarar o gate fechado aqui aprovaria cobertura que ainda não existe.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e17-11-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E17-11-AC-1` | perfil manual apontando para o fixture proxy-407 em 127.0.0.1:18080, que responde 407 a todo CONNECT que chegue sem Proxy-Authorization, mais origem em 127.0.0.1:18081 que registra cada conexao TCP recebida, URL de stream http://media.fixture.invalid/live.flac e politica de retry com limite 2 e backoff de 250 ms | o playback da URL é iniciado com o perfil manual e o proxy devolve 407 no CONNECT, sem que o usuário altere o perfil nem escolha direct | Proxy failure não cai direto |
| `TASK-E17-11-AC-2` | vault desbloqueado com três entradas em namespaces distintos — origin bearer 'orig-tok-1', proxy usuário 'px-user' e senha 'px-pass', provider API key 'prov-key-1' — fixture proxy-407 configurado para exigir Basic de proxy e origem em 127.0.0.1:18081 que ecoa todos os headers recebidos, com log de nível debug capturado | o stream é aberto pela política manual e o proxy responde 200 ao CONNECT depois de receber a credencial de proxy, sem que o usuário redigite nenhuma credencial | credenciais origin/proxy/provider separadas |
| `TASK-E17-11-AC-3` | origem em 127.0.0.1:18081 que responde 302 de http://cdn.fixture.invalid/master.m3u8 para http://edge.fixture.invalid/master.m3u8, manifest que referencia o segmento http://edge2.fixture.invalid/seg1.ts e a key http://keys.fixture.invalid/k1, e bypass explícito configurado apenas para o NAS 10.0.0.5:445 | o player segue o redirect e busca, na ordem, o manifest, o primeiro segmento e a key, sem que o usuário autorize nenhum dos três hosts | redirects/filhos respeitam política |
| `TASK-E17-11-AC-4` | perfil manual cujo proxy CONNECT aceita a conexão e trava o handshake TLS por 30 s, politica com limite de 2 tentativas e backoff de 250 ms, sessão tocando há 40 s com 1 job de análise ativo e contador de tentativas exposto no log | o usuário cancela a reprodução durante o CONNECT em voo e, em seguida, troca o perfil de manual para system, disparando o reconnect da politica | cancel/reconnect bounded. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | criar tests/proxy_policy.rs com o fixture local de dois servidores — proxy CONNECT em 127.0.0.1:18080 que responde 407 ao primeiro CONNECT sem Proxy-Authorization e 200 depois dela, e origem em 127.0.0.1:18081 que grava cada requisição e cada header recebido — e rodar cargo test -p vynamp-player --test proxy_policy nos 4 casos: CONNECT com 407, redirect 302 com manifest, segmento e key em hosts distintos, cancel com CONNECT pendurado por 30 s e troca de perfil com reconnect. | Esperado: com o 407 do proxy o contador de conexões TCP da origem fica em 0, provando que não há queda para direct, e o erro exposto é classificado como falha de proxy e não como 401/403 de origem; Proxy-Authorization aparece em todos os CONNECT do fixture de proxy e em 0 das requisições que a origem grava, e o bearer de origem aparece só na origem; dos 3 hosts filhos do redirect, o host do segmento fora da politica gera 1 negação e 0 requisições, o do manifest já autorizado gera 1 CONNECT, e a chamada ao NAS 10.0.0.5:445 sai sem CONNECT por estar no bypass; o cancel fecha o CONNECT pendurado em menos de 100 ms e o reconnect após a troca de perfil manual para system conta exatamente 2 tentativas no log, sem novo job de análise e sem reabrir o áudio a partir de 0 s. Hoje nada disso existe: o repo vynamp-player tem um único arquivo .rs, src/main.rs, com só println!("vynamp_player 0.1.0"), zero dependências e nenhum diretório tests/, então o alvo não compila e nenhuma dessas contagens foi medida. |
| Hardware | nenhum: a prova inteira é feita com dois servidores locais em 127.0.0.1, um proxy CONNECT de fixture e um trust store de teste em arquivo; não exige placa de áudio, aparelho Android, aparelho iOS nem máquina macOS, e o ambiente Windows 11 x86_64 roda os sockets de loopback sem hardware adicional. | ausência é limitação registrada, não aprovação |

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
