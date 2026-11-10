# TASK-E08-10 — Entregar controle remoto autenticado

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E08-10` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E08-10` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M4 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E07-03`, `TASK-E08-07`, `TASK-E09-03` |
| Bloqueia | `TASK-E12-09` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 8–20 dias úteis |
| Risco | médio |

## Entrega

API local desligada por padrão, pairing e opção LAN com TLS/rate limits.

## Boundary

- **Não** implementa casting, bridges Chromecast/UPnP/AirPlay nem o app companion. Isso é TASK-E12-09, que esta TASK bloqueia; controle remoto parece casting, mas aqui o cliente é um cliente HTTP qualquer contra a API local, e descoberta e protocolos de mídia são lá.
- **Não** implementa OIDC/PKCE, roles nem isolamento por conta do serviço Vynamp. Isso é TASK-E09-03; token e revogação aparecem nos dois escopos, e o token daqui é de pairing local, não credencial de conta na nuvem.
- **Não** implementa o IPC de instância única nem os verbos `open/enqueue/diagnostics/validate`. Isso é TASK-E08-07, dependência direta; os dois canais coexistem na mesma máquina e quem executa tende a reusar o pipe para a API HTTP.
- **Não** define o threat model do produto nem as regras gerais de fronteira de pacotes e sandbox. Isso é TASK-E11-02; TLS, rate limit e pairing parecem política de segurança global, mas aqui são propriedades de uma superfície concreta.
- **Não** entrega tray, media keys, hotkeys globais nem SMTC. Isso é TASK-E08-06; pause remoto e pause por tecla de mídia parecem o mesmo caminho, e a integração com o SO tem dono separado.
- **Não** configura proxy, túnel nem política de rede isolada para a opção LAN. Isso é TASK-E17-11; abrir a API em LAN exige regra de rede, e a política de egress e proxy do processo inteiro é de lá.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e08-10-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E08-10-AC-1` | API local no estado padrão de instalação (`remote.enabled=false`) e, em seguida, ligada manualmente em `127.0.0.1:8731` com `remote.lan=false`, zero clientes pareados e rate limit de 5 tentativas de autenticação por minuto por cliente | um cliente HTTP envia `POST /v1/transport/play` sem cabeçalho de autorização, depois com um token de 32 bytes aleatórios nunca emitido, e em seguida repete a chamada com esse token 10 vezes no mesmo minuto, lendo o estado do transporte entre as tentativas | Cliente sem pairing não controla |
| `TASK-E08-10-AC-2` | servidor no ar com o documento de rotas publicado em disco e o transporte em `playing` numa faixa local de 44,1 kHz | um cliente lê o documento de rotas e envia requisições para `/v1/exec`, `/v1/shell`, `/v1/fs/read` com `path=C:/Windows`, `/v1/fs/write` e `/v1/path/resolve` | nenhum endpoint de shell/filesystem |
| `TASK-E08-10-AC-3` | dois clientes pareados A e B com tokens distintos de 32 bytes, rotação de token de A habilitada e o token de B revogado entre duas chamadas consecutivas | o cliente A chama `POST /v1/transport/pause` com o token antigo e depois com o token rotacionado, e o cliente B chama a mesma rota com o token revogado | rotação e revogação de token funcionam. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --test remote_api` no repo `vynamp-player` com o servidor em `127.0.0.1:8731` e um cliente que faz as chamadas sem token, com token inventado, com token rotacionado e com token revogado, mais o snapshot do documento de rotas gravado em disco e um contador de estado do transporte antes e depois das tentativas | Hoje o repo `vynamp-player` tem 1 arquivo `.rs` (`src/main.rs`) com `println!("vynamp_player 0.1.0")`, zero dependências e nenhum `tests/`: não há servidor HTTP, rota, par TLS nem limitador de taxa, então o alvo de teste não compila e nenhuma destas chamadas tem onde chegar. Depois da TASK a evidência esperada é 0 portas em escuta enquanto `remote.enabled=false`, 401 nas duas primeiras chamadas sem credencial válida com 0 mudanças de estado do transporte, 401 também nas tentativas 3 a 5 e 429 da sexta em diante no mesmo minuto para o mesmo cliente, 0 correspondências para exec, shell, fs e path no documento de rotas publicado e 404 nas 5 tentativas de caminho de arquivo, token antigo de A recusado com 401 depois da rotação, token novo aceito com 200, e token revogado de B recusado com 403. |
| Hardware | nenhum: cliente e servidor são processos na mesma máquina Windows 11 x86_64 em loopback e o par TLS da opção LAN é exercitado com certificado de teste local; não há segundo host, placa de áudio, aparelho Android, aparelho iOS ou máquina macOS em jogo. | ausência é limitação registrada, não aprovação |

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
