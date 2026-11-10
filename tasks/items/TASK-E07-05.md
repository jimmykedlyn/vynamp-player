# TASK-E07-05 — Implementar helper nativo isolado

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E07-05` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E07-05` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E07-02`, `TASK-E11-02` |
| Bloqueia | `TASK-E07-07`, `TASK-E11-07`, `TASK-E12-05` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Sandbox OS, RPC autenticado, memória compartilhada limitada e consentimento de plugin nativo.

## Boundary

- **Não** implementa nenhum codec nem decoder dentro do filho. O que o helper decodifica, codecs desktop e arquivos grandes, e `TASK-E01-08`; esta TASK entrega o transporte e a jaula, nao o trabalho do convidado.
- **Não** faz o fuzz do RPC. O corpus adversarial de parsers, pacotes e RPC e `TASK-E10-05`, e um fuzzer aqui seria um segundo dono da mesma entrada hostil.
- **Não** define o schema do manifest nem o vocabulario de capabilities. A classe de risco native-helper, o entrypoint e as permissions sao declarados em `TASK-E07-02`; aqui o consentimento e consumido e registrado, nao inventado.
- **Não** roda lifecycle, quotas e circuit breaker do plugin Wasm. Init/start/event/stop/unload e o isolamento por 3 falhas em 60 s sao `TASK-E07-04`, e o helper tem supervisor por processo.
- **Não** leva o helper para o instalador. Assinatura, notarizacao e artefato por plataforma sao `TASK-E11-07`; empacotar aqui duplicaria a distribuicao.
- **Não** e o worker de DSP de terceiros nem o scanner de VST3. Ponte opcional, blacklist e compensacao de latencia sao `TASK-E12-07` e ficam fora do formato .vynplug.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e07-05-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E07-05-AC-1` | o helper nativo de teste rodando em processo filho e falando RPC versionado por canal nomeado com token de 32 bytes por sessao, com uma chamada em voo | o filho e morto por TerminateProcess no meio da resposta e, em rodadas separadas, envia quadro com magic errado, quadro com length acima do buffer declarado e payload truncado | Crash ou RPC inválido do child não derruba main |
| `TASK-E07-05-AC-2` | o filho recem-criado sob Job Object com token restrito, com GetProcessHandleCount lido antes da primeira chamada e o conjunto herdado limitado ao canal e ao evento de shutdown | cem chamadas RPC completas rodam, o filho e encerrado e recriado, e a contagem de handles do main e do filho e lida de novo | handles mínimos |
| `TASK-E07-05-AC-3` | o filho criado dentro de AppContainer com capabilities nulas, sem rede e sem o diretorio do usuario mapeado, no Windows 11 x64 | o helper de teste tenta abrir C:/Windows/win.ini, conectar a 8.8.8.8:53, criar um processo e ler uma variavel de ambiente, uma tentativa por chamada | sandbox testado |
| `TASK-E07-05-AC-4` | um pacote .vynplug de runtime native-helper com o consentimento explicito ja gravado e uma DLL de teste plantada ao lado do manifest | o pacote e carregado, o helper e iniciado por CreateProcess e a lista de modulos do processo main e enumerada antes e depois do load | nenhuma DLL arbitrária carregada no main. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --test native_helper -- --test-threads=1` em apps/player numa conta Windows 11 x64, com windows-sys para AppContainer, Job Object, GetProcessHandleCount e EnumProcessModules, e um binario helper de teste no mesmo crate; hoje nao ha windows-sys em nenhum Cargo.toml do repo, entao a dependencia entra nesta TASK | morto no meio do RPC, o main sobrevive e a chamada devolve erro com retryable false antes do deadline; os 3 quadros malformados, magic errado, length acima do buffer e payload truncado, sao recusados antes de tocar qualquer decoder, e nenhum derruba o main nem deixa o filho vivo; depois de 100 RPCs e do recreate, o delta de handles do main e 0 e o do filho volta ao baseline; das 4 tentativas de escape as 4 falham com erro de OS tipado, e o codigo real medido fica no log, com o esperado 5 para win.ini, 10013 para o socket e 203 para a variavel; a enumeracao de modulos do main devolve a mesma lista antes e depois, com 0 entradas apontando para o diretorio do pacote. Hoje nao existe helper, nao existe supervisor e nao existe teste em apps/player, que tem so src/main.rs com um println!. |
| Hardware | Windows 11 x64 desta maquina, com AppContainer, Job Object e contagem de handles, cobre o sandbox Windows; faltam aparelho Android arm64 para o broker de arquivos por SAF e uma maquina macOS para o seatbelt, e nenhum dos dois existe aqui, entao o sandbox dessas duas plataformas fica como ausencia registrada, nao aprovacao | ausência é limitação registrada, não aprovação |

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
