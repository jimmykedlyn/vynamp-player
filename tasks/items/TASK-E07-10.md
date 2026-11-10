# TASK-E07-10 — Auditar isolamento das extensões

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E07-10` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E07-10` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E07-08`, `TASK-E10-05` |
| Bloqueia | `TASK-E10-10`, `TASK-E12-07` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Revisão de segurança focada e testes de falhas durante reprodução.

## Boundary

- **Não** fixa os defaults de limites, o threat model nem a prova de sandbox: o teto de 64 MiB de memória Wasm, o payload de 1 MiB, a taxa de 100 eventos por segundo e a regra de IPC e de pacote são `TASK-E11-02`. Esta TASK audita o comportamento observado contra esses números; redefinir aqui o teto de 64 MiB ou o deadline de 500 ms criaria dois conjuntos de limites que divergem na primeira revisão.
- **Não** escreve os fuzz targets nem o corpus adversarial de pacote: o ZIP de proporção 1000:1 com 200 arquivos e caminho `../`, o manifest HLS de 20 mil entradas e o payload RPC de 4 GiB são `TASK-E10-05`, dependência desta TASK. Aqui se consome o resultado do fuzz e se injetam falhas nomeadas e determinísticas; um trap escrito à mão não é alvo de fuzz e não pode ser contado como cobertura dele.
- **Não** implementa nem altera o broker de permissões: allowlist de domínios, root handles e persistência de grants são `TASK-E07-03`. O AC-2 prova que a recusa acontece, com que motivo e sem byte saindo do host; conceder, expirar ou revogar grant dentro desta TASK duplicaria a autoridade de acesso que o broker já tem.
- **Não** redesenha o lifecycle, as quotas nem o circuit breaker: init/start/event/stop/unload, o deadline de 500 ms em metadata, o timeout de 10 s do I/O de provider e o cooldown após 3 falhas em 60 s são `TASK-E07-04`. O trap e o loop daqui são injeções para observar o áudio local por fora, e corrigir a máquina de estados durante a auditoria faria o auditor consertar o objeto auditado.
- **Não** entrega o sandbox de OS do helper nem o consentimento de plugin nativo: RPC autenticado, memória compartilhada limitada e a política de SO do processo filho são `TASK-E07-05`. O crash é injetado de fora com `TerminateProcess`; se o helper morrer por defeito da própria sandbox, isso vira achado endereçado a `TASK-E07-05`, não código escrito aqui.
- **Não** fecha o gate de M3 nem monta a matriz de evidência REQ/TASK/QA: isso é `TASK-E10-10`, que esta TASK bloqueia e que consome o veredito daqui como linha QA-009 herdada. Um relatório sem P0 e P1 abertos não autoriza declarar REQ-E07-10 aceito nem substitui a matriz.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e07-10-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E07-10-AC-1` | o player com um `Stream` aberto por `vynamp-output` em `NullHost` com `NullClock::Virtual` alimentado por `SineF32 { freq: 440.0, rate: 48_000 }` do `vynamp-output-testkit`, um plugin WASM provider ativo e um helper nativo em processo separado, mais as quatro fixtures de falha em `tests/fixtures/plugins/isolation/`: módulo que executa `unreachable` (trap), módulo em `loop {}` com fuel esgotando, helper abortado no meio do bloco e helper morto por `TerminateProcess`, e o grant `network.domains` pronto para ser revogado durante um `http-fetch` pendente | cada uma das quatro falhas é injetada uma vez com o transporte tocando, depois de cada injeção o relógio virtual avança o mesmo número de períodos da rodada de controle, e ao fim se leem `Stream::counters()` (`callbacks`, `frames_delivered`, `source_underruns`, `os_xruns`), `Stream::state()` e a lista de `Event` acumulada, comparando com o mesmo avanço sem injeção | Traps, loops, crash de helper e revoke durante I/O não interrompem áudio local |
| `TASK-E07-10-AC-2` | um plugin WASM instalado cujo manifest declara só `metadata.read` e nenhuma `network.domains`, com a allowlist de domínios vazia no broker de `TASK-E07-03` e nenhuma root handle concedida, mais as cinco URLs filhas do corpus de `TASK-E10-05`: 127.0.0.1, 169.254.169.254, `file:///etc/passwd`, um destino que responde 302 para host fora da allowlist e um domínio que resolve para loopback | o plugin chama `http-fetch` para cada uma das cinco URLs e, em paralelo, `read-selected-file` com uma root handle que não lhe foi concedida, e cada tentativa é registrada lado a lado pelo broker (motivo da recusa) e por um sink de rede instrumentado que conta qualquer byte que saia do host | permissões negadas impedem exfiltração |
| `TASK-E07-10-AC-3` | o relatório da revisão de segurança focada desta TASK, com cada achado classificado em P0, P1, P2 ou P3, o arquivo citado com linha e a TASK dona quando o achado não é desta, mais o SHA em que a revisão foi aberta e o `tasks/backlog.json` nesse SHA | o relatório é triado antes do merge e cada achado P0 e P1 é confrontado com um commit que o corrija e com o teste que falha sem esse commit, enquanto os P2 e P3 restantes são registrados com TASK dona e prazo | críticos resolvidos. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --test plugin_isolation --test exfiltration -- --test-threads=1` em `apps/player`, sobre `tests/fixtures/plugins/isolation/`, diretório que hoje não existe: os quatro `.wasm` de falha (trap por `unreachable`, `loop {}` com fuel esgotando, helper abortado e helper morto por `TerminateProcess`), o plugin sem `network.domains`, o broker de `TASK-E07-03` com allowlist vazia e um sink de rede que conta bytes de saída, tudo com o transporte em `NullClock::Virtual` mais `VirtualClockHandle::advance` para que cada período seja determinístico e reexecutável | esperado: nas quatro injeções o stream continua em `Running` e `frames_delivered` cresce por período do mesmo tamanho do controle sem injeção, `source_underruns` e `os_xruns` sem acréscimo atribuível ao plugin e exatamente um diagnóstico por falha no registro do manager; no caso de permissão negada, 5 recusas registradas com motivo (host privado, link-local, esquema `file`, redirect fora da allowlist, resolução para loopback), 1 recusa para a root handle não concedida e zero byte no sink de rede. Hoje nada disso existe: `cargo test --manifest-path apps/player/Cargo.toml` roda 0 testes em 1 suíte, `apps/player/src/main.rs` tem 3 linhas e só `println!("vynamp_player 0.1.0")`, o `Cargo.toml` do player tem `[dependencies]` vazio, não há diretório `tests/`, nenhum `Cargo.toml` do repo depende de `wasmtime` e `libs/engine/src/lib.rs` só exporta `VERSION` — não há plugin carregado nem áudio para preservar. A instrumentação do veredito existe só em parte e fora do player: `libs/output/crates/vynamp-output-core/tests/stream.rs` tem 10 `#[test]` e `Counters` já conta `source_underruns`, `os_xruns`, `frames_delivered` e `callbacks`, `Stream::state()` devolve `Running`, `VirtualClockHandle::advance` existe, e `tests/null_host.rs` tem 7 `#[test]` cobrindo o evento de downgrade e a abertura DoP; o harness offline de plugin é `TASK-E07-08` e o que ele ainda não entrega não pode ser contado como medido. |
| Hardware | nenhum: o cenário é numérico e de processo — o transporte é `NullHost` sobre `NullClock`/`VirtualClockHandle` e nenhum teste abre dispositivo de áudio, enquanto o helper real precisa apenas de um processo filho do Windows, que este ambiente oferece (Windows 11 x64, sem placa profissional). Ficam ausentes e se registram como limitação, não como aprovação: aparelho Android e iOS, máquina macOS, saída ASIO ou CoreAudio, e o gate M0 de runtime WASM em Android do perfil mobile da spec — nenhum deles entra neste aceite. | ausência é limitação registrada, não aprovação |

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
