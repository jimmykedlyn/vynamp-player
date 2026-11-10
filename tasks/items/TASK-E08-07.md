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
- **Não** define o threat model nem as regras gerais de IPC e de fronteira de pacotes do produto. Isso é TASK-E11-02; o pipe daqui é um canal concreto com lista fechada de verbos, e quem executa tende a promovê-lo a padrão de segurança de todos os canais.
- **Não** entrega a API local HTTP de controle remoto, pairing, TLS ou rate limit. Isso é TASK-E08-10; `open` e `enqueue` vindos de outro processo parecem controle remoto, mas aqui a origem é o mesmo usuário na mesma máquina, sem autenticação de cliente.
- **Não** implementa tray, hotkeys globais, SMTC ou media keys. Isso é TASK-E08-06; `diagnostics` é o único contato com o SO previsto aqui, e a integração de controles do Windows tem TASK própria.
- **Não** implementa a fila de reprodução nem a state machine de comandos e eventos. Isso é TASK-E01-03; o verbo `enqueue` precisa despachar para ela, mas a validade de estado, o cancelamento por generation e as transições pertencem lá.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e08-07-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E08-07-AC-1` | a instância principal do vynamp-player aberta no Windows 11 x86_64, com o named pipe `vynamp-player-<SID>` já criado, nenhum processo `calc.exe` na lista de processos do host e um cliente de teste conectado ao pipe com permissão de escrita | um segundo processo executa `vynamp-player.exe --ipc-send '{"verb":"shell","args":["calc.exe"]}'` e, pela conexão já aberta, envia uma linha de 4096 bytes com bytes de controle e JSON malformado, sem nenhum dos verbos `open/enqueue/diagnostics/validate` | Segundo launch não executa comando arbitrário |
| `TASK-E08-07-AC-2` | dois arquivos FLAC de 48 kHz com o mesmo conteúdo de uma faixa, em caminhos absolutos de 200 caracteres contendo `á`, `ç`, `ã`, `歌` e `🎵`, cujos nomes diferem apenas pela forma de normalização Unicode: um gravado em NFC e outro em NFD | o CLI executa `vynamp-player open <caminho>` para cada uma das duas variantes e o cliente de teste grava cada linha de ack do pipe em um arquivo de saída que é comparado byte a byte com o caminho enviado | Unicode preservado |
| `TASK-E08-07-AC-3` | a instância principal aberta, `fixtures/tone-1s.flac` presente no disco, `fixtures/nao-existe.flac` ausente e o verbo `xyz` fora da lista fechada publicada | o shell executa `vynamp-player open` e `vynamp-player enqueue` para cada uma das três entradas e lê `echo $?` do processo filho depois de cada chamada | CLI retorna exit codes |
| `TASK-E08-07-AC-4` | a lista fechada `open/enqueue/diagnostics/validate` publicada no pipe, cada verbo aceitando no máximo um caminho e só dentro de roots que o broker de permissões já concedeu, sem nenhuma root contendo `C:/Windows` ou o diretório do usuário | um cliente conectado envia `open` com `C:/Windows/System32/config/SAM`, depois `read` com `C:/Users/<user>/.ssh/id_ed25519` e em seguida `validate` com `../../../Windows` como caminho | IPC não expõe filesystem geral. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --test cli_ipc` no repo `vynamp-player` com o binário principal iniciado sobre `fixtures/unicode/歌 - ação (NFD).flac` e um cliente que envia os três verbos inválidos, mais a comparação hexadecimal do caminho de ack; fora do teste, `cargo run -- open fixtures/nao-existe.flac; echo $?` e a contagem de processos `calc.exe` antes e depois do envio do verbo `shell` | Hoje o repo `vynamp-player` tem 1 arquivo `.rs` (`src/main.rs`) com `println!("vynamp_player 0.1.0")`, zero dependências e nenhum `tests/`: o alvo `cli_ipc` não compila porque não existe, não há named pipe e nenhuma fixture Unicode foi criada. Depois da TASK a evidência tem que mostrar 4 testes executados e 4 verdes, código 0 para arquivo existente e um código não-zero distinto e estável para cada uma das outras classes de falha (inexistente, verbo desconhecido, caminho fora da root), 0 diferença em hexadecimal entre o caminho enviado e o recebido nas formas NFC e NFD, e 0 processos `calc.exe` novos depois do envio do verbo `shell`. |
| Hardware | nenhum: named pipes, códigos de saída e caminhos Unicode são exercitados entre processos locais no Windows 11 x86_64, e este CLI não abre dispositivo de áudio, placa de captura nem aparelho móvel. | ausência é limitação registrada, não aprovação |

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
