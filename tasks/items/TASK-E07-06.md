# TASK-E07-06 — Entregar templates e plugins de exemplo

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E07-06` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E07-06` |
| Critérios da spec | `specs/plugins.md` §5 (rascunho, não aprovado) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E07-04` |
| Bloqueia | `TASK-E07-08` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Templates Rust para WASM: letras locais, espectro e provider simulado.

## Boundary

- **Não** define o formato .vynplug, o schema do manifest nem as regras de compatibilidade major. Isso é TASK-E07-02; a confusão é plausível porque cada exemplo precisa de um manifest e a tentação é estender o schema enquanto se escreve o exemplo.
- **Não** implementa o host simulado completo, com injeção de falhas e corpus adversarial. Isso é TASK-E07-08; o harness que roda os três exemplos é quase o mesmo código, e quem executa tende a achar que endurecê-lo já pertence aqui.
- **Não** implementa o lifecycle de runtime, quotas, timeouts e circuit breaker. Isso é TASK-E07-04; o exemplo chama register/start/event/stop e parece que implementar a máquina de estados é pré-requisito do exemplo, quando o exemplo só consome o que ela expõe.
- **Não** implementa a resolução de grants pelo broker, incluindo network.domains e files.read.selected_roots. Isso é TASK-E07-03; os exemplos declaram essas permissões e portanto parecem donos do mecanismo que as aplica.
- **Não** entrega as cenas declarativas, ring/taps e famílias FFT do visualizer de produção. Isso é TASK-E17-09; o exemplo spectrum_view vira plugin real por acréscimo, e sem essa fronteira o escopo cresce de amostra para recurso.
- **Não** publica nem registra os exemplos no registry, com versões imutáveis e verificação de publisher. Isso é TASK-E07-09; distribuir os exemplos é tentador aqui, mas aqui eles só existem no repo.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e07-06-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E07-06-AC-1` | os três exemplos em Rust com target wasm32-unknown-unknown (spectrum_view sem permissões, lyrics_local com files.read.selected_roots sobre uma única pasta de fixture com 2 arquivos .lrc, radio_provider_mock com network.domains restrito a mock.local), cada um com manifest declarando exatamente esse conjunto de permissões | o harness offline é executado com o grant set lido do manifest de cada exemplo e dispara a sequência register, start, 1 evento de fixture e stop em cada módulo | Exemplos funcionam no harness offline com permissões mínimas |
| `TASK-E07-06-AC-2` | SDK interface version 1 com docs de lifecycle versionadas por sdk_version no próprio documento, e uma cópia do repo onde o sdk_version foi elevado para 1.1.0 apenas adicionando um evento e um campo opcional de manifest, sem remover nem renomear nada | o gerador de docs do SDK roda em modo de comparação contra os docs commitados nos dois estados, 1.0.0 e 1.1.0 | documentação de lifecycle acompanha a versão do SDK. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo xtask plugin build-examples --target wasm32-unknown-unknown mais cargo test -p vynamp-plugin-harness --test examples_offline, gravando target/plugin-harness-report.json com uma entrada por exemplo, e ainda cargo xtask plugin docs --check rodado nos dois estados do SDK: 1.0.0 e uma cópia com sdk_version 1.1.0, comparando o sdk_version do cabeçalho de cada documento de lifecycle com o do crate e o diff de símbolos entre os dois estados | o relatório tem que listar 3 módulos, cada um com denied_calls igual a 0, grant set idêntico ao declarado no manifest e a sequência register/start/event/stop concluída; a checagem de docs tem que acusar 0 documento com sdk_version divergente do crate em 1.0.0 e em 1.1.0, e o diff de 1.0.0 para 1.1.0 tem que mostrar 1 evento e 1 campo opcional de manifest acrescentados, 0 removidos e 0 renomeados; hoje nada disso existe: o repo vynamp-player tem 1 único arquivo .rs (src/main.rs) com println!("vynamp_player 0.1.0"), zero dependências e nenhum diretório tests/, o umbrella não tem diretório docs/ nem crate de SDK, schemas/plugin-host.wit expõe só init, invoke e stop e não os register, start, event e unload que a spec e o Given desta TASK exigem, este rustup tem 5 targets instalados e nenhum wasm32-*, e cargo xtask tem 14 variantes de subcomando em Action em xtask/src/main.rs e nenhuma plugin, então não há harness, não há exemplos e nenhum comando acima roda antes desta TASK |
| Hardware | nenhum: os três exemplos são módulos WASM executados em host simulado e o espectro consome FFT/PCM de fixture, nenhum teste abre dispositivo de áudio. O que falta nesta máquina é toolchain, não hardware: este rustup tem 5 targets instalados e nenhum wasm32-*, e wasmtime, wasm-tools, cargo-component e wasm-opt não estão instalados, então compilar e rodar os exemplos em Windows 11 x64 depende de provisionar os dois; aparelho Android, aparelho iOS e máquina macOS não existem aqui e pertencem a TASK-E13-08 | ausência é limitação registrada, não aprovação |

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
