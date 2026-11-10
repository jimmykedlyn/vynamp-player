# TASK-E07-03 — Implementar broker de permissões

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E07-03` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E07-03` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E07-02`, `TASK-E05-02` |
| Bloqueia | `TASK-E07-04`, `TASK-E08-08`, `TASK-E08-10`, `TASK-E10-05` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Acesso mediado a rede, roots selecionadas e biblioteca, com grants persistidos.

## Boundary

- **Não** implementa o transporte HTTP, Range, ICY nem o cancelamento de stream. O adapter com redirects, autenticacao, timeouts e seek negociado e `TASK-E05-02`; aqui o redirect so e revalidado contra o grant, e a conexao em si nao pertence a esta TASK.
- **Não** define o schema do manifest nem o vocabulario de capabilities. Os 8 nomes de permissao, os codigos de erro do parser e a regra de major sao `TASK-E07-02`; duplicar essa lista aqui faria o validador e o broker discordarem.
- **Não** roda o lifecycle nem aplica circuit breaker e quotas. Init/start/event/stop/unload, idempotencia de cleanup e o limite de 3 falhas em 60 s sao `TASK-E07-04`, e o broker so responde permitido ou negado a cada chamada.
- **Não** desenha a tela de consentimento, revoke e safe mode. O gerenciador com intencao explicita para permissao nova, disable e rollback e `TASK-E07-07`; o broker persiste e obedece o grant, nao o apresenta.
- **Não** implementa a biblioteca. Schema SQLite, migrations e os dados que library.read e library.modify alcancam sao `TASK-E03-01`, e o broker so media o acesso, sem conhecer tabela ou query.
- **Não** abre porta, loopback nem API local. Controle remoto desligado por padrao, pairing e opcao LAN com TLS sao `TASK-E08-10`, e conceder loopback automatico a um plugin e exatamente o que esta TASK recusa.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e07-03-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E07-03-AC-1` | um plugin de teste com grant de network.domains para radio.example.invalid, grant de files.read.selected_roots para uma root selecionada, um pedido de library.modify respondido com negar e nenhuma capability de escrita, com 2 http-fetch em voo e 2 handles de arquivo abertos | o usuario revoga o grant de rede antes da primeira resposta e, em outra rodada, nega o grant no prompt enquanto a primeira requisicao ja saiu | Negar e revogar bloqueia requests/handles |
| `TASK-E07-03-AC-2` | grant de network.domains apenas para radio.example.invalid e um transporte de teste que responde 200 no primeiro salto, 301 para o mesmo host com outro path, 302 para cdn.other.invalid e 308 de http para https | o plugin pede http-fetch na URL concedida e o broker acompanha cada salto do redirecionamento antes de entregar a resposta ao modulo | redirect revalida domínio |
| `TASK-E07-03-AC-3` | o world vynamp-extension de schemas/plugin-host.wit com as 4 funcoes de host-capabilities, http-fetch, read-selected-file, selected-metadata e notify, e nenhuma outra importacao | o modulo Wasm de teste tenta abrir C:/Windows/win.ini, ler uma variavel de ambiente, listar o diretorio da raiz e ler um caminho com ../ fora do root-handle concedido, uma tentativa por chamada | plugin não recebe shell, env vars ou filesystem completo. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --test permissions_broker` em apps/player com transporte HTTP stub em processo, sem socket, e um modulo Wasm de teste que faz as quatro tentativas de escape; o corpus de redirect cobre 200, 301, 302, 303, 307 e 308, e o par concedido contra negado e o mesmo modulo rodado com dois grants | com grant so para radio.example.invalid, a requisicao para esse dominio e aceita, a de other.example.invalid sai com network.denied sem tocar o stub e o revoke em voo fecha os 2 handles abertos e nenhuma resposta e entregue depois dele; dos 6 saltos, os que mudam de host sao recusados com 0 requisicoes alem do primeiro salto e o 301 de mesmo host com outro path e aceito; o world compilado expoe exatamente 4 funcoes de host, http-fetch, read-selected-file, selected-metadata e notify, nenhuma recebe caminho absoluto, e as 4 tentativas de escape falham com erro tipado, com o caso ../ saindo com files.escape antes de qualquer leitura; o log nao contem ocorrencia de ambiente, shell ou raiz plena. Hoje nao existe broker, nao existe stub e nao existe wasmtime nem wit-parser em nenhum Cargo.toml do repo; apps/player tem so src/main.rs com um println! e nenhum diretorio tests/. |
| Hardware | nenhum: os saltos de redirect, o revoke em voo e as quatro tentativas de escape rodam contra um transporte stub em processo, sem socket real, sem dispositivo de audio e sem placa profissional | ausência é limitação registrada, não aprovação |

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
