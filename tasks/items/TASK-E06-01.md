# TASK-E06-01 — Definir schema e validador de temas

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E06-01` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E06-01` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E02-02` |
| Bloqueia | `TASK-E06-02`, `TASK-E06-03`, `TASK-E14-03` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | alto |

## Entrega

Schemas de manifest/tokens/layout, parser e erros por JSON Pointer.

## Boundary

- **Não** abre, extrai, mede nem instala o ZIP do tema: recusa de traversal, paths absolutos, symlinks, zip bomb e o limite de 20 MiB são `TASK-E06-02`. Aqui o parser recebe um diretório ou bytes já confiáveis; trazer a inspeção do pacote para cá daria dois donos à decisão de aceitar o `.vyntheme`.
- **Não** renderiza nó algum nem avalia binding em tempo de execução: montar as telas e manter Classic/Library/Mini funcionando é `TASK-E06-03`. O validador só responde se os nomes de binding e de ação estão na allowlist; desenhá-los aqui faria o validador depender da UI que ele deveria guardar.
- **Não** produz nem pinta os três temas: os pacotes Classic Graphite, Midnight Modern e Copper com assets e licenças são `TASK-E06-04`. Esta TASK entrega o schema que os reprova ou aceita, e aprovar o próprio produto seria o validador auditando a si mesmo.
- **Não** define limites de recurso, defaults de segurança nem prova de sandbox: threat model, budgets e regras de pacote são `TASK-E11-02`. Os números de 256 KiB de manifest, 200 arquivos e 4096 px por dimensão vêm de lá e da spec `extensions/01-temas.md`; fixá-los aqui sem essa TASK criaria dois conjuntos de limites que divergem na primeira revisão.
- **Não** emenda nem migra schema: quando o layout precisar de `anchor`, `constraints` e `fallback`, quem muda o contrato de perfil desktop/phone/tablet é `TASK-E14-03`. O schema atual em `schemas/theme-layout.schema.json` não tem esses campos, e inventá-los nesta TASK seria a mesma violação que ela existe para detectar.
- **Não** exercita o parser contra entrada hostil aleatória: fuzz de manifest, layout e ZIP com corpus adversarial é `TASK-E10-05`, que já declara o ZIP de proporção 1000:1 e o caminho `../`. Um teste de fixture escrito à mão não substitui o alvo de fuzz e não pode ser contado como cobertura dele.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e06-01-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E06-01-AC-1` | duas fixtures de manifest em `fixtures/themes/manifest/`: `major-2.json`, idêntica a `schemas/examples/theme-manifest.json` exceto por `schema_version: 2`, e `major-1.json`, idêntica ao exemplo com `schema_version: 1` | o parser de tema lê as duas fixtures e cada leitura é registrada como aceita ou recusada, com o motivo e o JSON Pointer do campo culpado | Rejeitar schema major desconhecido |
| `TASK-E06-01-AC-2` | uma fixture `optional-unknown.json` derivada de `schemas/examples/theme-manifest.json` com dois campos opcionais que o schema não declara — `homepage` na raiz e `preview` dentro de `layouts` — sem violar nenhum dos 9 `required` nem os `pattern` de `id` e `version` | o parser roda no modo forward-compatible, os avisos são coletados por JSON Pointer e a mesma fixture é submetida em paralelo ao schema estrito, que mantém `additionalProperties: false` | avisar campos opcionais desconhecidos |
| `TASK-E06-01-AC-3` | três fixtures de layout em `fixtures/themes/layout/`: `cycle.json`, com âncoras `a -> b` e `b -> a`; `missing-essential.json`, em `mode: mini`, sem nó de `action: stop` e sem nó de `action: mute`; e `complete.json`, com os dois nós e âncoras acíclicas | o validador resolve o grafo de âncoras e confere as ações essenciais por modo, devolvendo a lista de erros com o JSON Pointer de cada nó culpado | detectar ciclos e ações essenciais ausentes. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --manifest-path apps/player/Cargo.toml theme_schema` sobre `fixtures/themes/manifest/` (cinco manifests: o exemplo do repo, `major-1`, `major-2`, `optional-unknown` e um com `id` fora do `pattern`) e sobre `fixtures/themes/layout/` (três layouts), comparando cada veredito com uma tabela de veredito esperado escrita na fixture | esperado: `schema_version: 2` sai recusado com o Pointer `/schema_version` e motivo de major desconhecido, e `schema_version: 1` sai aceito com zero erro; `optional-unknown.json` sai aceito com exatamente 2 avisos, `/homepage` e `/layouts/preview`, e o schema estrito (`schemas/theme-manifest.schema.json`, `additionalProperties: false`) sai recusado no mesmo arquivo; `cycle.json` sai recusado nomeando os dois nós do ciclo — e vale registrar que `anchor` não existe em `schemas/theme-layout.schema.json`, que só declara `id`, `type`, `x`, `y`, `width`, `height`, `binding`, `action`, `accessible_name` e `color_token` com `additionalProperties: false`, então o grafo de AC-3 se prova contra `extensions/01-temas.md` linha 9 e o contrato que acrescenta `anchor`, `constraints` e `gap` é `TASK-E14-03` —, `missing-essential.json` sai recusado nomeando `stop` e `mute` como ações essenciais ausentes e `complete.json` sai aceito. Hoje nada disso roda: `apps/player/src/main.rs` tem 3 linhas e só `println!("vynamp_player 0.1.0")`, o `Cargo.toml` do player não tem uma dependência sequer e não existe diretório `tests/`, então a suíte do player é 0 teste. E o pacote é bin-only — o `Cargo.toml` do player declara `[[bin]]` e nenhum `[lib]` —, então um `tests/theme_schema.rs` novo não importaria o parser: o teste teria que morar em módulo `#[cfg(test)]` ou o pacote precisaria ganhar alvo de biblioteca antes do primeiro teste. O único validador que existe é `xtask/src/validate.rs::schema_example` (linha 167), que roda `jsonschema` draft 2020-12 sobre `schemas/examples/theme-manifest.json` e `theme-layout.json` — 2 arquivos, ambos válidos por construção — e não tem modo forward-compatible, não emite aviso por Pointer e não conhece ciclos nem ações essenciais. `VALIDATION.json` reporta `schemas_and_examples: 10`, `errors: []`. |
| Hardware | nenhum: o trabalho é parser puro sobre JSON e os testes leem fixtures de texto. Nenhum teste abre dispositivo de áudio, janela ou aparelho, e o formato não depende de plataforma — os perfis phone/tablet do mesmo schema ficam para `TASK-E14-03`. A única coisa ausente deste ambiente é validação em alvo móvel, que este escopo não pede. | ausência é limitação registrada, não aprovação |

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
