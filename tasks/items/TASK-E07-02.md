# TASK-E07-02 — Definir pacote e SDK de plugins v1

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E07-02` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E07-02` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E07-01`, `TASK-E06-02` |
| Bloqueia | `TASK-E07-03`, `TASK-E07-05` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Manifest/schema, interfaces WIT, lifecycle e regras de compatibilidade major.

## Boundary

- **Não** faz cumprir as permissoes. O acesso mediado a rede, roots selecionadas e biblioteca, com grant persistido e revogacao, e `TASK-E07-03`; aqui so se declara o vocabulario e se recusa manifest invalido, e um broker que nascesse deste arquivo teria dois donos da mesma decisao.
- **Não** executa o ciclo init/start/event/stop/unload nem aplica quotas. O contrato dessas chamadas fica no WIT desta TASK, mas quem as roda, isola por tres falhas em 60 s e limita flood e `TASK-E07-04`.
- **Não** instala, atualiza nem faz rollback do pacote no disco. Validador de ZIP, staging e commit por versao sao `TASK-E06-02` para tema e `TASK-E07-07` para plugin; a entrega aqui e o manifest e o contrato, nao o ciclo do arquivo.
- **Não** assina nem empacota. O CLI de pack, sign e validate com o relatorio de compatibilidade e `TASK-E07-08`, e implementar assinatura aqui criaria dois formatos para o mesmo pacote.
- **Não** prova o sandbox nem os limites do runtime. Memoria, fuel, interrupcao de loop e de crescimento sao `TASK-E07-01`, que hoje e ausencia medida; repetir a medicao aqui afirmaria cobertura que o spike nao tem.
- **Não** define o RPC do helper nativo nem o sandbox de sistema operacional. RPC autenticado, memoria compartilhada limitada, handles minimos e consentimento explicito sao `TASK-E07-05`; aqui apenas se declara que o runtime native-helper existe no manifest.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e07-02-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E07-02-AC-1` | um corpus de manifests contra schemas/plugin-manifest.schema.json: kind fora do enum dos 5, runtime native-helper combinado com kind visualizer, supported_platforms com um alvo que nao esta na lista de 9, uma capability fora das 8 de permissions e um campo shell a mais no topo | o parser do manifest no player le cada arquivo e devolve o manifest tipado ou um erro com o JSON Pointer do campo, antes de o pacote chegar ao host | Validar tipo, plataformas e permissões |
| `TASK-E07-02-AC-2` | quatro manifests identicos exceto sdk_version, 0.9.0, 1.0.0, 1.4.2 e 2.0.0, contra um host que fala o package vynamp:plugin@1.0.0 de schemas/plugin-host.wit | o loader resolve a compatibilidade do sdk_version de cada um dos quatro antes de instanciar o modulo Wasm | major incompatível não carrega |
| `TASK-E07-02-AC-3` | um pacote .vynplug com assinatura de publisher valida e manifest que pede files.read.selected_roots apontando para a raiz inteira do disco e network.domains com curinga global, ao lado do mesmo manifest sem assinatura | o loader verifica a assinatura e em seguida submete o manifest ao mesmo validador de permissoes que o pacote nao assinado atravessa | assinatura não permite ignorar políticas de acesso. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo xtask check-tasks` no umbrella, que ja valida schemas/plugin-manifest.schema.json contra schemas/examples/plugin-manifest.json no laco de schemas de xtask/src/validate.rs, mais `cargo test --test plugin_manifest` em apps/player sobre um corpus de 12 manifests, 5 validos um por kind e 7 invalidos, e o mesmo corpus cruzado contra o schema publicado por python -I com jsonschema 4.26.0 | o corpus tem que fechar 5 aceitos e 7 recusados, cada recusa com JSON Pointer e codigo proprios, e nenhum invalido pode passar so porque veio assinado; sdk_version 2.0.0 recusado pelo loader antes de instanciar o modulo, 1.4.2 aceito e 0.9.0 recusado contra o host 1.0.0; o manifest assinado que pede a raiz inteira continua recusado, entao assinatura nao compra acesso. Hoje existe o par schema mais exemplo e ele valida com 0 erros, medido no jsonschema 4.26.0, e o exemplo com um campo extra shell e recusado por additionalProperties; o que nao existe e o parser no player, porque apps/player tem so src/main.rs com um println! e nenhum diretorio tests/, e o WIT de schemas/plugin-host.wit nunca foi compilado, porque nao ha wasmtime nem wit-parser em nenhum Cargo.toml do repo. |
| Hardware | nenhum: o corpus e de manifests e a validacao e pura sobre JSON e WIT; nenhum teste abre dispositivo de audio, socket ou processo, e alvo de plataforma e uma string comparada contra o enum do schema | ausência é limitação registrada, não aprovação |

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
