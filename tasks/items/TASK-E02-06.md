# TASK-E02-06 — Entregar Mini, preferências e estados de erro

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-06` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-06` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E02-05` |
| Bloqueia | `TASK-E02-07`, `TASK-E02-08`, `TASK-E05-03`, `TASK-E06-03`, `TASK-E07-07`, `TASK-E08-06`, `TASK-E10-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | médio |

## Entrega

Mini420×112, navegação preferences e empty/loading/offline/recovery.

## Boundary

- **Não** implementa o Library Workspace nem o inspector cujo drawer este TASK só reusa. `TASK-E02-05` é a dona da sidebar, do grid, da lista e do inspector; aqui entram o Mini, as Preferências e os estados de tela, e refazer o workspace faria duas telas com o mesmo nome.
- **Não** cria o motor de reprodução nem o transporte. `TASK-E01-03` entrega a state machine e o cancellation por generation; `TASK-E02-03` já liga commands e events do Classic Stack. Esta TASK prova que trocar de modo não interrompe o que esses dois já fazem, e não que passar a existir áudio.
- **Não** grava as preferências nem define o schema delas. O esquema com defaults e migração é `TASK-E02-14`, e a persistência com keychain e arquivo de config é `TASK-E11-05`; aqui só se navega pela tela e se lê o que o esquema expõe.
- **Não** constrói a cadeia de áudio nem o editor de EQ. `TASK-E02-07` é a dona; a seção DSP das Preferências desta TASK é uma entrada de navegação que abre o painel de outra TASK, não o painel.
- **Não** é dona do docking nem da geometria por monitor. `TASK-E02-08` entrega painéis destacáveis, snap de 12 DIP e restauração por monitor; aqui o Mini e as Preferências têm geometria fixa declarada e nenhuma posição persistida por tela.
- **Não** substitui a localização das mensagens de erro nem a pluralização. `TASK-E02-09` entrega PT-BR e EN por chave; esta TASK escreve as chaves e as strings de origem, e traduzir ou pluralizar é da TASK seguinte.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e02-06-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E02-06-AC-1` | uma faixa de 5 minutos tocando no Classic Stack a 600×760 DIP, com a posição de reprodução em 00:01:37, a sessão de áudio aberta e o identificador de sessão e a revisão de transporte lidos antes da troca, sem pausa nem seek entre as leituras | o modo é trocado para Library Workspace, depois para Mini em 420×112 DIP e depois de volta a Classic Stack, cada troca pelo controle da UI, e após cada uma são lidos o identificador de sessão, a revisão de transporte, a posição com tolerância de um período de buffer e o contador de underruns | Trocar modo mantém áudio |
| `TASK-E02-06-AC-2` | o diálogo de Preferências em 880×680 DIP aberto pelo menu a partir de um botão identificado, com a navegação lateral nas dez seções, o foco dentro do diálogo e um item de seção selecionado; ao lado, um painel de inspetor aberto como drawer | pressiona-se Escape com o foco em três posições diferentes (primeira ação segura, último item da navegação lateral e um campo de texto com edição em andamento), e em seguida fecha-se o drawer pelo seu botão, lendo depois de cada fechamento qual elemento recebeu o foco | Escape/focus return corretos |
| `TASK-E02-06-AC-3` | quatro estados de erro montados com dados falsos: biblioteca vazia na primeira execução, carregamento de pasta cancelável em andamento, raiz offline com arquivos locais válidos e um arquivo corrompido junto de três válidos, mais uma falha de migration de banco com o texto de erro real contendo caminho absoluto e nome de crate | cada estado é renderizado e lido por árvore de acessibilidade, e o texto visível na área principal é varrido por padrões de stacktrace, caminho absoluto, `panicked at`, nome de crate e endereço de memória | erros mostram ações úteis sem stacktrace no fluxo. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | Duas frentes concretas. (1) `cargo test -p vynamp-player-ui --test mode_swap --locked`, com um transporte falso que registra cada comando recebido, a posição e o identificador de sessão: trocar Classic → Library → Mini → Classic tem que emitir zero comandos de stop/open e manter identificador de sessão, posição dentro de um período de buffer e contador de underruns em zero. (2) `cargo test -p vynamp-player-ui --test states_and_focus --locked`, que renderiza os quatro estados de erro e o diálogo de Preferências e lê a árvore de acessibilidade, mais um `grep` sobre as strings de erro montadas para falha de migration e arquivo corrompido. | Esperado: nas três trocas de modo, o identificador de sessão é o mesmo, a posição anda só o tempo de reprodução decorrido e o contador de underruns fica em 0; Escape fecha o diálogo e devolve o foco ao botão invocador nas três posições de foco testadas, e fechar o drawer devolve o foco ao seu acionador; cada um dos quatro estados tem CTA nomeado e nenhuma string principal casa com `panicked at`, `src\`, `Backtrace`, nome de crate ou `0x`; o detalhe técnico aparece só no painel opcional de diagnóstico. Hoje nada disso existe: `apps/player` tem 1 arquivo `.rs` de 3 linhas só com `println!`, `[dependencies]` vazia, nenhum `tests/`, nenhuma dependência de UI e `cargo test --all --locked` roda 0 testes; nao ha transporte falso, nem diálogo, nem árvore de acessibilidade para ler, e `apps/player/specs/player.md` §5 e §6 seguem PENDENTE. |
| Hardware | nenhum: o critério de áudio é indireto — identificador de sessão, geração de transporte e contador de underruns lidos de um transporte falso em processo, sem abrir saída de áudio; a parte de acessibilidade usa a árvore semântica programática, não um leitor de tela instalado, e a validação com NVDA/Narrator está registrada como pendência em `TASK-E02-10` | ausência é limitação registrada, não aprovação |

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
