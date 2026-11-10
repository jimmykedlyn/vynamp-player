# TASK-E08-07 — Entregar CLI e IPC de instância única

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E08-07` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E08-07` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E01-03`, `TASK-E08-06` |
| Bloqueia | `TASK-E08-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

Comandos open/enqueue/diagnostics/validate e named pipe com validação de peer.

## Boundary

- **Não** implementa o lock de owner único por perfil nem a arbitragem de qual processo é dono do perfil. Isso é TASK-E16-02; instância única soa como a mesma coisa, mas lá o escopo é lock de SO, encaminhamento autorizado e isolamento entre perfis, e trazer o lock para cá deixaria dois donos do mesmo estado.
- **Não** define o threat model nem as regras gerais de IPC e pacotes do produto. Isso é TASK-E11-02; o pipe daqui é um canal concreto com lista de verbos, e quem executa tende a querer promovê-lo a padrão de segurança de todos os canais.
- **Não** entrega a API local HTTP de controle remoto, pairing, TLS ou rate limit. Isso é TASK-E08-10; `open` e `enqueue` vindos de outro processo parecem controle remoto, mas aqui a origem é o mesmo usuário na mesma máquina, sem autenticação de cliente.
- **Não** implementa tray, hotkeys globais, SMTC ou media keys. Isso é TASK-E08-06; `diagnostics` e `validate` são o único contato com o SO previsto aqui, e a integração de controles do Windows tem TASK própria.
- **Não** implementa a fila de reprodução nem a state machine de comandos/eventos. Isso é TASK-E01-03; o verbo `enqueue` precisa despachar para ela, mas a validade de estado, o cancelamento por generation e as transições pertencem lá.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e08-07-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E08-07-AC-1` | a instância principal do vynamp-player aberta no Windows 11 x86_64, com o named pipe `vynamp-player-<SID>` do usuário já criado e um cliente de teste conectado a ele | um segundo processo lança `vynamp-player.exe --ipc-send '{"verb":"shell","args":["calc.exe"]}'` e, logo depois, envia pelo mesmo pipe uma linha de 4096 bytes com bytes de controle fora do conjunto de verbos `open/enqueue/diagnostics/validate` | Segundo launch não executa comando arbitrário |
| `TASK-E08-07-AC-2` | um FLAC local de 48 kHz com caminho absoluto de 200 caracteres contendo `á`, `ç`, `ã`, `歌` e `🎵`, e o mesmo nome de arquivo duplicado em NFC e em NFD apontando para a mesma faixa | o CLI recebe `vynamp-player open <caminho>` para cada variante e a instância principal devolve o caminho no evento de ack do pipe | Unicode preservado |
| `TASK-E08-07-AC-3` | a instância principal aberta e três entradas de teste: `fixtures/tone-1s.flac` existente, `fixtures/nao-existe.flac` inexistente e o verbo `xyz` desconhecido | o CLI roda `vynamp-player open` e `vynamp-player enqueue` para cada entrada, o processo filho termina e o shell lê o código de saída com `echo $?` | CLI retorna exit codes |
| `TASK-E08-07-AC-4` | a lista fechada de verbos `open/enqueue/diagnostics/validate` publicada no pipe, cada um aceitando no máximo um caminho que o broker de permissões já concedeu como root | um cliente conectado envia `open` com `C:/Windows/System32/config/SAM` e em seguida `read` com `C:/Users/<user>/.ssh/id_ed25519`, fora das roots concedidas | IPC não expõe filesystem geral. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --test cli_ipc` no repo `vynamp-player`, com o binário principal já iniciado sobre a fixture `fixtures/unicode/歌 - ação (NFD).flac` e um cliente que envia os quatro verbo inválidos, mais `cargo run -- open fixtures/nao-existe.flac; echo $?` | Hoje o repo `vynamp-player` tem 1 arquivo `.rs` (`src/main.rs`) que imprime `vynamp_player 0.1.0`, zero dependências e nenhum `tests/`: o alvo `cli_ipc` não compila porque não existe. Depois da TASK a evidência tem que mostrar 4 testes executados e 4 verdes, exit codes 0 para arquivo existente, 2 para inexistente, 3 para verbo desconhecido e 4 para caminho fora da root, 0 processos de shell criados pelo cliente malicioso e o caminho Unicode de 200 caracteres idêntico byte a byte ao enviado (0 diferenças em comparação hexadecimal), inclusive nas formas NFC e NFD. |
| Hardware | nenhum: named pipes, exit codes e caminhos Unicode são exercitados em processos locais no Windows 11 x86_64, e o CLI desta TASK não abre dispositivo de áudio nem placa nenhuma. | ausência é limitação registrada, não aprovação |

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
