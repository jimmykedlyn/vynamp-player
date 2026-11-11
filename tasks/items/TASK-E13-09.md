# TASK-E13-09 — Entregar tablet/foldable e distribuição Android

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E13-09` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E13-09` |
| Critérios da spec | `specs/player.md` §5 (rascunho, não aprovado) |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E13-06`, `TASK-E13-07`, `TASK-E13-08`, `TASK-E11-04` |
| Bloqueia | `TASK-E13-10`, `TASK-E14-05`, `TASK-E15-09` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

UI adaptativa, APK/AAB assinado, ABIs/NDK/page size e requisitos de publicação.

## Boundary

- **Não** constrói a shell phone nem a navegação acessível do telefone: bottom navigation de 64 dp, transporte, fila, EQ phone e TalkBack são `TASK-E13-05`. Aqui o que muda é a classe de largura a partir de 600 dp, a postura e a escala de fonte; refazer o layout phone aqui daria dois donos ao mesmo shell Compose.
- **Não** fixa NDK, compile SDK, target SDK nem build-tools: os quatro campos de Android em `config/toolchains.json` são `null` e o pin por SHA é de `TASK-E11-04`. Esta TASK constrói com o valor pinado e confronta o manifest com ele; preencher o pin para destravar o build inverteria a ordem que a AC-3 exige.
- **Não** prova a correspondência entre o signer do APK/AAB e o keystore nem o alinhamento de 16 KB page size: `apksigner verify --print-certs`, `jarsigner -verify`, `keytool` e a leitura de `lib/arm64-v8a` são de `TASK-E15-09`. O artefato sai assinado daqui, mas quem confere o certificado é ela, e conferir nos dois lugares daria dois pareceres sobre o mesmo SHA-256.
- **Não** faz o item-a-item de grants e permissões de sistema depois do update nem instala em máquina limpa: esse confronto é `TASK-E14-05`. A AC-2 só relê o grant que o próprio app persistiu, junto com biblioteca, playlists e checkpoint; o inventário de MediaStore, permissões do OS e instalação em máquina limpa não entra aqui.
- **Não** fecha o aceite Android 1.0 em hardware: lifecycle, audio focus, rota Bluetooth, TalkBack, energia e as 2 h de tela apagada são `TASK-E13-10`, inclusive o perfil de aparelho exigido. Um APK instalado e um layout aprovado não são esse aceite.
- **Não** autoriza publicação nem declara a política vigente da Play como cumprida: a AC-3 é uma revisão datada com fonte, e a aprovação de publicação, o draft privado e a custódia dos secrets são de `TASK-E15-10`. Ler a revisão como aval de release é a confusão mais cara desta TASK.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e13-09-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E13-09-AC-1` | o módulo `:app` do app Android em buildType release, com o layout adaptativo da spec, medido em seis perfis fixados por qualifier do Robolectric: compacto 360×800 dp, compacto em landscape 800×360 dp com capa reduzida e duas colunas, médio 600×839 dp com navigation rail mais player e fila, expandido 840×1280 dp de tablet com biblioteca e inspector, foldable vertical com hinge de 24 dp em State.HALF_OPENED ocluindo o centro, e o compacto com fontScale 2.0 | os quatro perfis de largura e orientação rodam como `:app:testReleaseUnitTest` com `@Config(qualifiers="w360dp-h800dp")`, `w800dp-h360dp-land`, `w600dp-h839dp` e `w840dp-h1280dp`, o hinge é injetado por `TestWindowLayoutInfoPublisherRule` publicando `FoldingFeature(Orientation.VERTICAL, State.HALF_OPENED, occluding 24 dp)` mais um `createFoldingFeatureActivityRule` num AVD de foldable, e o sexto perfil é o qualifier `w360dp-h800dp` somado a `fontScale = 2.0`, com cada árvore capturada e afirmada | Phone/tablet/landscape/hinge/fonte passam |
| `TASK-E13-09-AC-2` | um aparelho arm64 onde a 1.0.0, instalada a partir do AAB desta entrega, indexou 120 faixas de uma pasta SAF, tem 3 playlists no banco privado e gravou no checkpoint a faixa 47 na posição 1:23,4 com a fila aberta, e cuja fotografia pré-update de contagem de faixas, ids de playlist, checkpoint e grant de pasta foi salva em `evidence/` | o APK 1.0.1 do mesmo pipeline de release é instalado por cima da 1.0.0 com `adb install -r` sem `adb uninstall` e sem `pm clear`, o app é aberto uma vez, fechado, e contagem de faixas, as 3 playlists, o checkpoint e o grant são lidos de volta e confrontados linha a linha com a fotografia | update preserva dados |
| `TASK-E13-09-AC-3` | o AAB candidato a 1.0.0, com `android_target_sdk` e `android_compile_sdk` lidos de `config/toolchains.json`, e as quatro páginas do Google Play sobre target API level, Data safety, suporte a 16 KB page size e permissões sensíveis abertas na data do release | `bundletool dump manifest --bundle` roda no AAB e `apkanalyzer manifest target-sdk` roda no APK derivado do mesmo bundle, e cada requisito das quatro páginas é confrontado com o manifest, com o formulário de Data safety preenchido e com a lista de permissões declaradas, gravando por requisito o link da fonte e a data da leitura | target SDK e políticas revisados |
| `TASK-E13-09-AC-4` | um aparelho Android arm64 físico de médio porte com 6 GiB e Android 14 ou 15, mais um AVD com a mesma imagem de API, ambos com o APK de release 1.0.0 instalado e o mesmo conjunto de fixtures de QA-020 e QA-021 | a mesma suíte de QA-020 e QA-021 roda nos dois alvos no mesmo SHA, e em cada alvo são coletados `adb shell getprop ro.product.cpu.abi`, `ro.build.version.sdk` e `ro.kernel.qemu`, com o resultado registrado por aparelho e o AVD rotulado como auxiliar de automação | aparelho arm64 real além do emulador. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `apps/android/gradlew.bat -p apps/android :app:testReleaseUnitTest` com os qualifiers `w360dp-h800dp`, `w800dp-h360dp-land`, `w600dp-h839dp`, `w840dp-h1280dp`, o foldable e `fontScale=2.0`, mais `TestWindowLayoutInfoPublisherRule` publicando o `FoldingFeature` HALF_OPENED de 24 dp; em seguida `cargo xtask release build --version 1.0.0 --platform android-arm64` para gerar APK e AAB, `bundletool dump manifest --bundle` e `apkanalyzer manifest target-sdk` sobre o artefato, e no aparelho `adb install -r` de 1.0.0 para 1.0.1 mais `adb shell getprop ro.product.cpu.abi` e `ro.kernel.qemu` no aparelho e no AVD | esperado: os seis perfis renderizam sem clipping, com os controles de transporte visíveis em w800dp-h360dp-land e a árvore refluída em fontScale 2.0; o manifest do AAB traz target SDK igual ao valor de `android_target_sdk` em `config/toolchains.json` e nenhuma permissão além das justificadas no Data safety; o APK derivado do mesmo bundle responde o mesmo target SDK no `apkanalyzer`; após o `adb install -r` continuam 120 faixas, as 3 playlists com os mesmos ids, o checkpoint da faixa 47 na posição 1:23,4 e o grant de pasta; no aparelho sai `ro.product.cpu.abi=arm64-v8a` com `ro.kernel.qemu` vazio contra `1` no AVD. O que ainda não existe hoje, medido: `apps/player/src/main.rs` tem 3 linhas e só o `println!("vynamp_player 0.1.0")`, o `Cargo.toml` do player tem a seção `[dependencies]` vazia e não há diretório `tests/`, então a suíte é 0 teste; `apps/android/` não existe, logo `cargo xtask ci detect` devolve `android=false` e `cargo xtask ci android` aborta com 'Actual Android app absent' em `xtask/src/ci.rs:204` antes de qualquer Gradle; os quatro campos de Android de `config/toolchains.json` são `null` e `ci::toolchains` aborta com 'Pin android_ndk in M0 before Android builds'; `cargo xtask release build` nem chega lá, porque `config/project.json` tem `release_enabled` e `application_ready` em `false` e `release::preflight` aborta com 'Release disabled: actual player/build/hardware/signing acceptance missing' (e `apps/desktop` também não existe, então `core` já é `false`); `xtask/src/release.rs` lista `android-arm64.apk` e `android-arm64.aab` em `SUFFIXES` e o subcomando `package` aceita `--platform android-arm64`, mas `package::android` chama `ci::gradle` num `apps/android/gradlew.bat` inexistente, então não há arquivo produzido; `packaging/android/signing.gradle.kts` é o único arquivo Gradle do repo e exige `ANDROID_KEYSTORE_PATH`, `ANDROID_STORE_PASSWORD`, `ANDROID_KEY_ALIAS` e `ANDROID_KEY_PASSWORD` do ambiente; `specs/player.md` §5 e `specs/ui.md` §5 seguem sem critério escrito, então o critério de UI tablet ainda não tem texto na spec do módulo; e `assets/android-layout.svg` é só o preview do phone (viewBox 400×880), sem nenhum caso de tablet, hinge ou 200%. |
| Hardware | aparelho Android arm64 físico com 6 GiB e Android 14 ou 15, mais um foldable ou tablet com hinge real para o perfil de postura: ausente neste ambiente, que é Windows 11 x64 sem Android SDK, sem `ANDROID_HOME` e sem `ANDROID_SDK_ROOT` definidos, sem `adb`, sem `bundletool` e sem `apkanalyzer` no PATH, sem aparelho conectado e sem AVD de foldable; só há JDK 20 fora do pin de `java: 17` em `config/toolchains.json`; o HALF_OPENED pode ser simulado por `androidx.window.testing`, mas a AC-4 exige arm64 físico e `quality/03-matriz-multiplataforma.md` reserva o emulador para automação de UI, então AC-1 hinge e AC-4 ficam declarados como bloqueados por hardware, nunca aprovados por emulador | ausência é limitação registrada, não aprovação |

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
