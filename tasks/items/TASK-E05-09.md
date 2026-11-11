# TASK-E05-09 — Endurecer rede e diagnóstico de streaming

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E05-09` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E05-09` |
| Critérios da spec | `specs/streaming.md` §5 (rascunho, não aprovado) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E05-07`, `TASK-E05-08` |
| Bloqueia | `TASK-E05-10`, `TASK-E09-01`, `TASK-E13-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Política para cada URL filha, corpus de falhas e painel de saúde sanitizado.

## Boundary

- **Não** implementa a janela live, a troca de qualidade por throughput nem o timeshift de 30 min em disco. É TASK-E05-07, a dependência imediata: a política daqui vale para cada URL filha do live, mas quem decide sequência, variante e janela é ela, e trazer essa decisão para cá criaria dois donos do mesmo estado de playlist.
- **Não** grava rádio em arquivo, não divide por CUE e não monitora espaço em disco. É TASK-E05-08, a outra dependência: a gravação consome o mesmo stream que esta TASK endurece, e tratar falha de disco cheio como falha de rede é a confusão mais provável aqui.
- **Não** implementa os buffers de 32 MiB e 16 MiB, o cache LRU de 2 GiB nem o purge por conta. É TASK-E05-04, que já entrega esses limites; esta TASK só garante que painel, log e export do índice de cache não vazem a query assinada.
- **Não** define rights descriptor, território ou janela de disponibilidade. É TASK-E09-01, que aparece nas blocks desta TASK junto de TASK-E13-10: a política de rede daqui consome o que esse RFC declarar e não inventa permissão para compensar ausência de direito.
- **Não** renova token de provider nem decide se download é permitido. É TASK-E05-10, que esta TASK bloqueia: renovação mexe em credencial e em retry, e implementá-la aqui faria as 5 tentativas consecutivas da política e o refresh disputarem o mesmo contador.
- **Não** cobre DASH, WebDAV ou nuvens comerciais. É TASK-E12-08: a spec adia esses transportes para M5 e a política desta TASK é de HTTP(S) progressivo, HLS e ICY, então estender o escopo aqui anteciparia uma fase inteira.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e05-09-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E05-09-AC-1` | um servidor de fixture em loopback servindo um master manifest com quatro filhos hostis: segmento em 302 para 169.254.169.254, hostname que resolve 203.0.113.7 na primeira consulta e 127.0.0.1 no rebind, cadeia de 6 redirects e key URI em origem diferente da playlist | a master playlist é expandida e cada URL filha é submetida à política de rede antes do connect e de novo depois de cada resolução DNS | Revalidar SSRF/DNS/redirects conforme origem |
| `TASK-E05-09-AC-2` | um stream HLS cujo media playlist carrega query assinada com exp, Signature e Credential e cujo request leva header Authorization, com log em nível trace, export do painel de saúde e export do índice de cache habilitados | o segmento 3 responde 403 e o usuário exporta o painel de saúde, o arquivo de log e o índice de cache | esconder segredos |
| `TASK-E05-09-AC-3` | um fixture stalled que aceita a conexão, entrega 40 KiB e fica 60 s sem enviar byte, com 3 streams abertos e uma interface com frame budget de 16 ms que exibe o estado de cada stream | o usuário pressiona Stop durante a abertura do stream 1, o stall do stream 2 dispara aos 15 s e a rede volta 32 s depois | reconexão, cancelamento e stall não travam a interface. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test --test net_policy no repo vynamp-player contra tests/fixtures/net, que sobe em loopback o 302 para 169.254.169.254, o rebind de 203.0.113.7 para 127.0.0.1, a cadeia de 6 redirects, a key URI em origem terceira e o stalled de 40 KiB, com log em trace | esperado: as 4 URLs filhas hostis recusadas antes do connect, com 0 conexões TCP abertas para link-local e para 127.0.0.1; o sexto salto recusado porque o limite é 5; 0 reenvios de Authorization a origem diferente; 0 ocorrências da query assinada em log, painel e índice de cache; o Stop do stream 1 devolvido ao chamador em menos de 16 ms e o stall do stream 2 publicando Reconnecting aos 15 s sem segurar a thread de controle. Hoje não existe nada disso: vynamp-player tem 1 arquivo .rs (src/main.rs) que só imprime vynamp_player 0.1.0, zero dependências e nenhum tests/, então o teste não compila até esta TASK criar o módulo de política. A medida de frame da interface exige o harness egui_kittest de TASK-E02-16, que também não existe: até ele, o oráculo prova latência de cancelamento e de transição de estado na thread de controle, e o número de frame fica declarado como não medido |
| Hardware | nenhum: o corpus é de servidores HTTP em loopback, então nenhum teste abre dispositivo de áudio, placa pro nem aparelho Android, iOS ou macOS. O critério de interface é latência de frame, e o harness egui_kittest de TASK-E02-16 ainda não existe neste repo, então a latência de frame se mede pelo tempo de retorno do cancelamento e da transição de estado na thread de controle | ausência é limitação registrada, não aprovação |

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
