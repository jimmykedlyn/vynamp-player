# TASK-E08-09 — Entregar audiobooks e progresso offline

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E08-09` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E08-09` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M4 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E05-10`, `TASK-E03-09` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 8–20 dias úteis |
| Risco | médio |

## Entrega

Agrupamento de livros, capítulos, bookmarks e adapter de sync de progresso.

## Boundary

- **Não** entrega os bookmarks genéricos de faixa nem o histórico de reprodução. Isso é TASK-E03-09; o deliverable daqui fala em bookmarks de livro, e bookmark por posição dentro de um arquivo é a mesma mecânica com outro dono, o que duplicaria o modelo persistido.
- **Não** implementa o progresso de conteúdo longo do provider de música. Isso é TASK-E05-10; progresso offline e progresso de conteúdo longo se parecem, mas lá o estado vem do adapter da plataforma e das restrições de direito, não do arquivo local.
- **Não** implementa feeds, episódios nem downloads de podcast. Isso é TASK-E05-05; um livro e um podcast são ambos conteúdo longo dividido em partes, e o agrupamento por capítulo não autoriza agrupar feed.
- **Não** envia scrobble nem presence ao terminar um capítulo. Isso é TASK-E08-08; o sync daqui transporta posição de retomada, e quem fala com Last.fm, ListenBrainz, Libre.fm ou Discord é aquela TASK, com opt-in próprio.
- **Não** implementa sleep timer nem agendamento por fim de faixa. Isso é TASK-E08-05; audiobook é o caso de uso clássico de dormir ouvindo, então quem executa tende a incluir o timer aqui.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e08-09-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E08-09-AC-1` | um livro importado com 12 capítulos MP3 de 48 kHz e 02:00:00 cada, velocidade padrão 1,0x e velocidade por categoria gravada na configuração com audiobook em 1,25x e música em 1,0x | o capítulo 3 do livro é aberto e o relógio consumido é amostrado por 20 s de parede, depois a reprodução para e uma faixa de música da mesma biblioteca é aberta pelo mesmo transporte, com o relógio consumido amostrado pelos mesmos 20 s | Velocidade por categoria |
| `TASK-E08-09-AC-2` | capítulo 5 do livro pausado em 01:12:30 de 02:00:00, o livro fechado, o app reaberto e `resume.mode=explicit` na configuração | o livro é reaberto na tela de livros, a posição exibida é lida antes de qualquer gesto e depois o usuário aciona a ação Continuar | retomada explícita |
| `TASK-E08-09-AC-3` | progresso local em 04:10:00 do capítulo 7 gravado em 2026-11-09T22:14:00Z e progresso remoto em 02:00:00 do capítulo 4 gravado em 2026-11-09T23:02:00Z, com sync bidirecional em opt-in ligado, e o mesmo par de registros em ordem invertida, com a posição menor vinda do lado mais antigo | a sincronização de progresso reconcilia os dois registros e grava o resultado no estado local e no stub remoto em `127.0.0.1:18081` nas duas ordens do fixture, registrando em cada uma qual dos dois valores foi escolhido | conflito de retrocesso não usa máximo sempre |
| `TASK-E08-09-AC-4` | livro local de 12 capítulos já importado, sync de progresso habilitado e o adaptador de rede forçado a falhar com DNS e handshake TLS recusados para todos os hosts | o app é iniciado sem rede, o capítulo 2 é aberto e reproduzido do início ao fim de 02:00:00 até 100% | sem rede continua reproduzindo. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --test audiobook` no repo `vynamp-player` sobre `fixtures/audiobook/` com 12 MP3 de 48 kHz e os dois JSON de progresso (capítulo 7 em 04:10:00 de 22:14:00Z e capítulo 4 em 02:00:00 de 23:02:00Z), mais um stub de sync em `127.0.0.1:18081` que responde 504 na primeira chamada e 200 depois, contando requisições | Hoje o repo `vynamp-player` tem 1 arquivo `.rs` (`src/main.rs`) com `println!("vynamp_player 0.1.0")`, zero dependências e nenhum `tests/`: não existe agrupamento de livro, persistência de progresso nem cliente de sync, e a fixture `fixtures/audiobook/` ainda não foi criada, logo o alvo de teste não compila. Depois da TASK a evidência esperada é 1 grupo de livro com 12 capítulos na ordem correta, 25 s de áudio consumido em 20 s de parede no capítulo 3 a 1,25x contra 20 s em 20 s na faixa de música a 1,0x (tolerância de 1 bloco de decodificação), 0 reproduções iniciadas na reabertura com `resume.mode=explicit` e posição 01:12:30 exata depois de Continuar; no conflito, 2 registros de decisão que nomeiam os dois lados (04:10:00 com 22:14:00Z e 02:00:00 com 23:02:00Z) e escolhas que diferem entre as duas ordens, provando que o resultado não é o máximo em todas elas; o teste falha se as duas ordens devolverem 04:10:00; no cenário offline, 100% do capítulo 2 consumido com 0 conexões TLS concluídas. |
| Hardware | nenhum: os fixtures são arquivos MP3 locais e o sync usa stub em `127.0.0.1`; a posição é medida pelo relógio consumido do engine, sem depender de placa de som, e o cenário offline é obtido recusando DNS e TLS no adaptador de teste — este ambiente não tem dispositivo de áudio profissional, aparelho Android, aparelho iOS nem macOS. | ausência é limitação registrada, não aprovação |

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
