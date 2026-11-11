# TASK-E16-09 — Gerar declarações OS a partir do build verificado

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E16-09` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E16-09` |
| Critérios da spec | `specs/player.md` §5 (rascunho, não aprovado) |
| Fase | M2 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E17-01`, `TASK-E16-01` |
| Bloqueia | `TASK-E16-03`, `TASK-E16-04`, `TASK-E16-05`, `TASK-E16-06`, `TASK-E17-12` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Gerador de handlers por container/parser e capabilities reais de cada artefato.

## Boundary

- **Não** gera o manifest de capability por artefato nem decide o que cada build entrega: esse manifest é `TASK-E17-12`, e esta TASK o consome como entrada. A confusão é plausível porque ambos escrevem JSON por plataforma, mas se este gerador também produzisse capability haveria dois donos do mesmo hash de artefato.
- **Não** registra a associação no sistema operacional: `HKCU\Software\Classes`, pacote esparso MSIX e `IExplorerCommand` são `TASK-E16-03`, MIME e ServiceMenus do XDG são `TASK-E16-04`, UTTypes e NSServices são `TASK-E16-05` e VIEW/SEND no Android é `TASK-E16-06`. Aqui se emite a declaração a partir do build; escrevê-la no shell sem o registro dono faria o gerador parecer instalador.
- **Não** certifica a ativação resultante nos cinco alvos: aceitar lote, cold/warm, defaults e preservação de terceiros é `TASK-E16-07`. Um handler gerado e coerente com o manifest não prova que o Explorer, o Nautilus ou o Finder o abrem, e declarar isso aqui seria aprovar sem aparelho.
- **Não** fixa a fase, a política nem a viabilidade de cada formato: `TASK-E17-01` é a dona do catálogo por binding. O gerador não pode promover uma entrada `planned` a `available` para produzir handler, e a guarda de `xtask/src/catalog.rs:91` já reprova `policy: required` fora de M0–M3.
- **Não** assina o `manifest.json` com minisign, não abre draft privado e não publica: `TASK-E15-10` é a dona do source digest, do manifesto assinado e da chave pública pinada. Comparar hash é o teto desta TASK; assinar sobre o mesmo arquivo daria duas autoridades ao mesmo conteúdo.
- **Não** instala nem roda o pacote dos cinco alvos para conferir: `TASK-E15-06`, `TASK-E15-07`, `TASK-E15-08` e `TASK-E15-09` fazem o smoke de NSIS/portable, AppImage/DEB, DMG/ZIP e APK/AAB. O artefato aqui é lido por hash, não executado.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e16-09-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E16-09-AC-1` | o catálogo `config/format-catalog.json` rev 0.7.0 com 68 entradas e `state: planned` nas 68 — 32 `required`, 19 `conditional`, 16 `optional` e 1 `unsupported` — mais quatro fixtures de capability: uma `planned` com backend e licença preenchidos, uma `experimental` com `backend` nulo, uma `verified` com `licensed: false` e uma `verified` com `backend`, `backend_version`, `license_ids` e `evidence` não vazios | o gerador é executado uma vez por alvo sobre as 68 entradas do catálogo e sobre as quatro fixtures, e o diretório de declarações emitido para cada alvo é listado arquivo a arquivo | planned/experimental/sem backend ou licença não gera handler |
| `TASK-E16-09-AC-2` | as entradas `bt-classic` e `bt-le` (`kind: bluetooth-transport`, `association_allowed: false`, zero extensões e zero MIMEs), `legacy-video-audio` (`avi`, `flv`, `mpg`, `association_allowed: false`), `pcm-raw` (`raw`, `pcm`, `association_allowed: false`), `sacd-iso` (`iso`, `association_allowed: false`) e `encrypted-drm` (`kind: excluded`), mais uma fixture de capability com `mime_types` iguais a `audio/*` e `application/octet-stream` | o gerador emite as listas de extensões e de MIMEs de associação das cinco plataformas e cada uma das seis entradas excluídas é procurada nessas listas e no relatório de motivos por entrada | MIME genérico/video/BT não é associado |
| `TASK-E16-09-AC-3` | os oito nomes de `SUFFIXES` de `xtask/src/release.rs:14` presentes em `dist/release/` — entre eles `vynamp-1.0.0-windows-x64-setup.exe` e `vynamp-1.0.0-windows-x64-portable.zip` — com `manifest.json` e `SHA256SUMS` escritos por `artifact_manifest`, mais as declarações de handler geradas que citam `artifact_digest` do artefato `windows-x64` | um script relê cada arquivo de `dist/release/` do disco, recalcula o sha256 e confronta cada `artifact_digest` das capabilities e das declarações de handler com o valor do `manifest.json` e com a linha correspondente de `SHA256SUMS` | manifest e pacote têm mesmos hashes. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --manifest-path apps/player/Cargo.toml handler_declarations` sobre `apps/player/tests/fixtures/capabilities/` — quatro capabilities mais as seis entradas excluídas do catálogo — junto de um script que relê `dist/release/` e recomputa o sha256 (`Get-FileHash -Algorithm SHA256` no Windows, `sha256sum` no Linux) contra `manifest.json` e `SHA256SUMS`, e do par `cargo test --manifest-path xtask/Cargo.toml release::tests::manifest_requires_exact_nonempty_set`, que já fixa hoje o conjunto exato de oito nomes em `xtask/src/release.rs:342` | esperado: das 68 entradas do catálogo rev 0.7.0, todas `state: planned`, saem 0 handlers — 0 das 58 com `association_allowed: true` e 0 das 10 com `false`; das quatro fixtures sai exatamente 1 handler, o `verified` com `licensed: true` e backend não vazio, e `planned`, `experimental` com `backend` nulo e `verified` sem licença saem com motivo escrito; `avi`, `flv`, `mpg`, `raw`, `pcm` e `iso` não aparecem em nenhuma lista de associação, `bt-classic` e `bt-le` não geram ProgID, UTI nem entrada de `MimeType`, e `audio/*` e `application/octet-stream` são reprovados como MIME associável — o catálogo tem 24 MIMEs distintos e nenhum deles genérico; o sha256 recalculado dos dois arquivos de `windows-x64` bate byte a byte com `manifest.json` e com `SHA256SUMS`. Nada disso existe hoje: `apps/player/src/main.rs` tem 3 linhas e só o `println!("vynamp_player 0.1.0")`, o `Cargo.toml` do player tem `[dependencies]` vazio e não há `apps/player/tests/`, então a suíte é 0 teste; `tests/fixtures/` do umbrella tem apenas `README.md` e nenhum `capabilities/`; `packaging/linux/vynamp.desktop` declara `MimeType=audio/flac;audio/mpeg;audio/ogg;audio/x-wav;` escrito à mão, sem gerador; `artifact_manifest` em `xtask/src/release.rs:143` escreve apenas `product_version` e `artifacts[file,bytes,sha256]` dos oito artefatos e recusa conjunto parcial, sem nenhuma linha de capability; `dist/release/` e `evidence/release/` não existem e `application_ready`, `release_enabled` e `branding_exports_ready` valem `false` em `config/project.json` |
| Hardware | nenhum: o gerador lê JSON de capability e recomputa sha256 sobre arquivos, e nenhum teste abre dispositivo de áudio, registra associação no shell ou roda em telefone. O artefato de AC-3 também não é produzível nesta máquina: `release::preflight` reprova com `release_enabled` e `application_ready` em `false` e `product_version` `0.1.0-dev`, faltam `THIRD_PARTY_NOTICES`, `packaging/update.pub`, `apps/android/gradlew` e `packaging/icons/export-manifest.json`, e não há `makensis` instalado nem máquina macOS Intel, macOS Silicon ou aparelho Android arm64, então os cinco alvos ficam declarados incompletos, nunca aprovados | ausência é limitação registrada, não aprovação |

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
