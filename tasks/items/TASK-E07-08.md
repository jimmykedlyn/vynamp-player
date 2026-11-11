# TASK-E07-08 — Entregar documentação e harness do SDK

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E07-08` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E07-08` |
| Critérios da spec | `specs/plugins.md` §5 (rascunho, não aprovado) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E07-06`, `TASK-E07-07` |
| Bloqueia | `TASK-E07-09`, `TASK-E07-10`, `TASK-E09-09`, `TASK-E11-08`, `TASK-E13-08` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Docs de API/lifecycle/budgets, CLI de pack/sign/validate e relatório de compatibilidade.

## Boundary

- **Não** cria nem mantém os três plugins de exemplo (spectrum, letras locais, provider simulado). Isso é TASK-E07-06; o harness desta TASK roda em cima deles e por isso parece que corrigi-los pertence aqui.
- **Não** entrega a interface do gerenciador de plugins, com instalação, atualização, disable, revoke, rollback e safe mode. Isso é TASK-E07-07; a CLI de validate é confundida com a tela de instalação porque as duas respondem a pacote inválido.
- **Não** implementa o runtime Wasmtime real com fuel e epoch interruption configurados nos limites do módulo. Isso é TASK-E07-01 e TASK-E07-04; o host simulado aceita expectativa de timeout e isso não prova interrupção real de CPU, que a spec exige provar no runtime.
- **Não** entrega o registry assinado, a imutabilidade de versão e a revogação por publisher. Isso é TASK-E07-09; a CLI de pack e sign produz pacotes que parecem prontos para publicar, mas publicar é outra TASK e outro armazenamento.
- **Não** é a auditoria de segurança de isolamento das extensões. Isso é TASK-E07-10; corpus adversarial aprovado e auditoria independente parecem a mesma evidência, e tratá-las como uma só dispensaria a revisão de contexto limpo.
- **Não** implementa a política de rede isolada e proxy do player. Isso é TASK-E17-11; o caso de redirect de domínio morre no broker de permissões, e quem executa pode achar que endurecer a rede da aplicação é o que falta.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e07-08-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E07-08-AC-1` | host simulado com tabela de grants de fixture e 9 cenários adversariais da spec: zip malicioso, loop infinito, crescimento de memória, permissão negada, redirect de domínio, SDK defasado, enxurrada de eventos, crash de worker e revogação durante requisição | cargo xtask plugin harness --corpus all é executado apontando para tests/fixtures/adversarial e grava o relatório por cenário | Host simulado cobre erros e grants |
| `TASK-E07-08-AC-2` | corpus com os 9 casos adversariais mais 1 caso de controle benigno, cada um com a expectativa declarada no próprio fixture (trap, denied, timeout ou interrupção por fuel) | o harness roda o corpus inteiro offline e compara o veredito de cada caso com a expectativa declarada, sem rede e sem dispositivo de áudio | corpus adversarial aprovado |
| `TASK-E07-08-AC-3` | os três exemplos de TASK-E07-06 compilados e passando contra SDK 1.0.0, e um SDK 1.1.0 que apenas acrescenta um evento e um campo opcional de manifest, sem remover nem renomear símbolo | os exemplos são recompilados contra 1.1.0 sem nenhuma alteração no código deles e a suíte examples_offline é executada de novo | versão minor aditiva não quebra exemplos. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo xtask plugin validate tests/fixtures/plugins e depois cargo xtask plugin harness --corpus tests/fixtures/adversarial --report target/compat.json, seguido de cargo xtask plugin compat --from 1.0.0 --to 1.1.0 --examples | evidência esperada: compat.json com 10 entradas (9 adversariais mais 1 controle), veredito igual à expectativa declarada em 10 de 10, e a matriz de compatibilidade listando 3 de 3 exemplos compilando e passando em 1.1.0 sem edição; hoje nada disso existe, porque o repo vynamp-player tem 1 único arquivo .rs (src/main.rs) com println!("vynamp_player 0.1.0"), zero dependências e nenhum tests/, cargo xtask tem 14 variantes em Action em xtask/src/main.rs e nenhuma plugin, tests/fixtures/ na raiz do umbrella contém só README.md, nenhum Cargo.toml declara wasmtime ou wit-parser e nenhum instala wasm-tools na máquina, logo não há CLI xtask, não há fixtures e o relatório de compatibilidade não tem o que comparar |
| Hardware | nenhum: o harness roda em processo sobre arquivos .vynplug de fixture e nenhum cenário do corpus abre placa de áudio, aparelho Android, aparelho iOS ou máquina macOS | ausência é limitação registrada, não aprovação |

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
