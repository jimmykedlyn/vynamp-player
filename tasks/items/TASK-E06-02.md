# TASK-E06-02 — Entregar instalação atômica de temas

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E06-02` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E06-02` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E06-01`, `TASK-E11-02` |
| Bloqueia | `TASK-E06-04`, `TASK-E07-02`, `TASK-E10-05` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | alto |

## Entrega

Validador de ZIP, staging, limites, checksums, instalação por versão e rollback.

## Boundary

- **Não** reescreve nem amplia o schema de manifest, tokens e layout, nem decide que ações são essenciais: essa autoridade é `TASK-E06-01`, e esta TASK consome o veredito dela. A confusão é plausível porque a instalação precisa validar antes de commitar, mas duplicar a regra faria o pacote passar por uma porta e falhar na outra.
- **Não** desenha as telas do tema instalado: o renderer de nós, bindings e fallback dos três modos é `TASK-E06-03`. Instalar e validar não prova que o layout abre; declarar renderização aqui seria aprovar sem o renderer existir.
- **Não** implementa a galeria, o armazenamento por usuário nem o safe mode: são `TASK-E06-08`. O rollback desta TASK devolve o tema anterior quando a instalação falha; o safe mode que restaura o builtin quando o renderer falha já com o tema ativo é outro gatilho e outra TASK.
- **Não** monta a UI de tema nem o editor com preview e export: `TASK-E06-07`. Aqui só existe a operação de instalar por versão; exportar e reimportar preservando configuração usa este instalador como consumidor, não como dono.
- **Não** escreve os fuzz targets nem o corpus adversarial aleatório de pacote: `TASK-E10-05` já declara o ZIP com proporção 1000:1, 200 arquivos e caminho `../`. As quatro fixtures daqui são casos nomeados e determinísticos, e contá-las como fuzz afirmaria cobertura que não há.
- **Não** define os budgets de recurso como política de segurança nem assina pacote: limites de threat model são `TASK-E11-02`, e a assinatura opcional v1 fica fora desta entrega porque nenhuma TASK de release está no escopo de tema até `TASK-E15-10`. Os 20 MiB, 200 arquivos, 4096 px e 64 MiB de decode vêm da spec, aplicados aqui como guarda de instalação.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e06-02-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E06-02-AC-1` | quatro arquivos `.vyntheme` adversariais gerados por fixture e versionados em `fixtures/themes/packages/`: `traversal.vyntheme` com uma entrada `layouts/../../evil.json`; `absolute.vyntheme` com uma entrada `/etc/cron.d/x`; `symlink.vyntheme` com uma entrada de symlink apontando para fora do staging; e `bomb.vyntheme` com um `assets/pad.bin` de 20 MiB de zeros deflacionado a ~20 KiB, proporção acima de 1000:1 | o instalador recebe cada pacote, extrai em staging e registra o veredito por entrada com o nome da entrada ofensora e o motivo da recusa | Rejeitar traversal, paths absolutos, symlinks e zip bombs |
| `TASK-E06-02-AC-2` | o tema `org.vynamp.classic` 1.0.0 já instalado e ativo, e um `org.vynamp.classic` 1.1.0 cujo `layouts/classic.json` referencia um `binding` fora da allowlist — o pacote passa no ZIP e é recusado pela validação de `TASK-E06-01` | o instalador executa a atualização e, depois dela, o tema ativo é lido de volta e a árvore de instalação é comparada com a de antes | update inválido mantém tema atual |
| `TASK-E06-02-AC-3` | dois pacotes que só diferem em bytes extraídos: `under.vyntheme` com 20 MiB − 1 byte somados nos `assets` declarados no manifest, e `over.vyntheme` com 20 MiB + 1 byte, ambos abaixo de 200 arquivos e dentro do limite de 256 KiB do manifest | o instalador mede o total extraído antes do commit e aplica ou recusa cada um, deixando o diretório de staging em observação depois do veredito | limite de 20 MiB extraídos. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --manifest-path apps/player/Cargo.toml theme_install` sobre `fixtures/themes/packages/` (sete pacotes: um válido de referência, os quatro adversariais de AC-1 e os dois de fronteira de bytes de AC-3), mais um script que, depois de cada recusa, lista o diretório de staging e o diretório de temas do usuário com contagem de arquivos e bytes (`Get-ChildItem -Recurse -File` seguido de `Measure-Object -Property Length -Sum` no Windows, e `find . -type f -ls` no Linux) | esperado: as quatro entradas adversariais saem recusadas com a entrada nomeada e nenhum byte escrito fora do staging — zero arquivos em `<temas>/` e contagem de staging igual a 0 no caso de recusa antes da extração, ou restos removidos no `Drop`; `org.vynamp.classic` 1.1.0 sai recusado e `org.vynamp.classic` 1.0.0 continua ativo com o mesmo conjunto de arquivos e o mesmo sha256 por arquivo, e `1.1.0` não aparece em `<temas>/`; `under.vyntheme` instala e reporta 20971519 bytes extraídos, `over.vyntheme` recusa reportando 20971521 contra o teto de 20971520. Hoje não existe instalador, nem leitor de ZIP no player: `apps/player/src/main.rs` tem 3 linhas, `[dependencies]` está vazio e não há `tests/`. O `zip` 9 está em `xtask/Cargo.toml` com `deflate`, usado só por `xtask/src/package.rs:72` (`ZipWriter`) para empacotar release — nunca `ZipArchive` para ler pacote, e nada de `sha2` fora de `xtask/src/release.rs` e `xtask/src/branding.rs`. Não há nenhum `.vyntheme` no repositório: a busca por `*.vyntheme` devolve 0 arquivo. |
| Hardware | nenhum: a instalação opera sobre arquivos no disco local e os testes montam os `.vyntheme` em diretório temporário. O que este ambiente não oferece não é hardware, é cobertura de sistema de arquivos: esta máquina é Windows 11 x64, então semântica de symlink POSIX e de permissão de arquivo Linux/macOS só pode ser exercitada aqui por entrada de ZIP com bit de symlink e não pelo `ln -s` real — a limitação fica declarada, não aprovada. | ausência é limitação registrada, não aprovação |

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
