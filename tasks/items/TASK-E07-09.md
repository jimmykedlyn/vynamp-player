# TASK-E07-09 — Preparar registry e revogação de plugins

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E07-09` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E07-09` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M4 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E07-08`, `TASK-E09-09` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 8–20 dias úteis |
| Risco | alto |

## Entrega

Registry assinado, versões imutáveis e identificação de publisher.

## Boundary

- **Não** entrega a interface do gerenciador, com instalação, atualização, disable, revoke, rollback e safe mode. Isso é TASK-E07-07; a revogação desta TASK alimenta a tela e por isso parece que a tela é daqui.
- **Não** produz a CLI de pack, sign e validate dos pacotes. Isso é TASK-E07-08; assinar um pacote na publicação e validá-lo na instalação parecem a mesma ferramenta, mas aqui só se verifica o que já foi assinado.
- **Não** define o formato .vynplug nem o schema do manifest, incluindo sha256 por arquivo e sdk_version. Isso é TASK-E07-02; a verificação de hash lê esses campos, e quem executa tende a estendê-los enquanto implementa o registry.
- **Não** implementa a revogação de grants de permissão concedidos ao usuário, nem o broker que resolve network.domains e files.read.selected_roots. Isso é TASK-E07-03; revogar plugin e revogar permissão aparecem lado a lado na mesma lista do gerenciador.
- **Não** cobre a assinatura do instalador do aplicativo nem a distribuição por loja, incluindo notarização e TestFlight. Isso é TASK-E13-09 e TASK-E20-06; o publisher desta TASK assina plugins, e confundir com assinatura do app levaria a colocar chave de distribuição no caminho errado.
- **Não** implementa a política de rede isolada e proxy por onde o download do registry trafega. Isso é TASK-E17-11; a verificação de hash detecta o download corrompido, mas não substitui a política de origem, e é tentador tratar o hash como suficiente.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e07-09-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E07-09-AC-1` | registry assinado de fixture listando br.mock.radio 1.0.0 e 1.1.0 com publisher verificado, mais uma lista de revogação assinada que revoga o build 1.1.0 pelo hash do pacote, e o plugin 1.0.0 já instalado no perfil | os dois fluxos são acionados contra esse registry: instalar 1.1.0 em um perfil limpo e atualizar 1.0.0 para 1.1.0 no perfil que já tem 1.0.0 | Build revogado não instala/atualiza |
| `TASK-E07-09-AC-2` | registry configurado e plugin br.mock.radio 1.0.0 instalado, com o usuário em posse do arquivo br.mock.radio-1.2.0.vynplug baixado por conta própria, assinado por publisher que existe no registry e que não está revogado | o usuário arrasta o pacote manual para o gerenciador e inicia a instalação, sem que exista qualquer download pelo registry | pacote manual ainda exige consentimento |
| `TASK-E07-09-AC-3` | registry informando sha256 por arquivo para br.mock.radio 1.0.0, com module.wasm de 47 KiB, e um servidor de fixture que devolve o módulo com 1 byte alterado na comparação e outro que devolve o módulo truncado em 1 KiB | o cliente baixa o pacote dos dois servidores e compara o sha256 de cada arquivo com o declarado no registry antes de instalar | hashes de download são verificados. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test -p vynamp-plugin-registry --test revocation_and_hash, com os fixtures tests/fixtures/registry/registry-signed.json, revocation-signed.json e três respostas de servidor de teste: pacote íntegro, pacote com 1 byte alterado e pacote truncado em 1 KiB | evidência esperada: no fluxo de instalação e no de atualização do build revogado, 2 tentativas e 0 instalações, com 2 linhas de recusa citando o id do build revogado; a instalação manual exige 1 consentimento explícito e recusa com 0 consentimentos, contando 1 prompt exibido; os dois downloads adulterados são rejeitados com 2 falhas de sha256 e 0 arquivos gravados no diretório de plugins. Hoje nada disso existe: o repo vynamp-player tem 1 arquivo .rs (src/main.rs) com println!("vynamp_player 0.1.0"), zero dependências e nenhum tests/, então o schema de registry, a lista de revogação e a verificação de hash não têm código nem fixture |
| Hardware | nenhum: registry e lista de revogação são fixtures assinadas em disco e os downloads usam um servidor HTTP de teste servido pelo próprio processo; nenhum aparelho nem placa de áudio entra no fluxo | ausência é limitação registrada, não aprovação |

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
