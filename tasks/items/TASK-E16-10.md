# TASK-E16-10 — Entregar deep links e callbacks do provider

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E16-10` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E16-10` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M4 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E16-01`, `TASK-E09-03` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Rotas próprias e HTTPS verificadas, OAuth state/PKCE e preview de ação.

## Boundary

- **Não** implementa identidade, troca de código por token nem autorização do provider: o fluxo OAuth completo é TASK-E09-03, e aqui se verifica só o state e o PKCE do lado do app antes de qualquer troca de código.
- **Não** define a semântica de Open e Enqueue nem o formato do envelope de ativação: esse contrato é TASK-E16-01, e o deep link apenas produz um OpenRequest que aquele broker valida.
- **Não** registra ProgID, menu de contexto nem associação de arquivo no Windows: o registro de shell é TASK-E16-03 e a remoção dele é TASK-E16-08, então esta TASK só declara a rota e nunca escreve em HKCU/Software/Classes.
- **Não** implementa MediaStore, SAF nem o modelo de permissão do Android: os grants de filesystem são TASK-E13-04, e o AC-3 apenas prova que uma intent concedida não os amplia.
- **Não** gera o manifesto OS nem as declarações de handler por alvo: isso é TASK-E16-09, e uma rota validada aqui não substitui a declaração assinada que aquele build emite.
- **Não** implementa a redaction de log nem a retenção que impedem a URL privada de aparecer em diagnóstico: esse mecanismo é TASK-E11-05, e aqui só se exige que a query não seja logada.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e16-10-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E16-10-AC-1` | build desktop com o scheme vynamp registrado e as rotas allowlisted v1/open e v1/auth/callback, mais o arquivo tests/fixtures/links-hostis.txt com 9 URIs, entre elas vynamp://open?locator=file:///C:/Windows/System32/cmd.exe, vynamp://open?autoplay=1, vynamp://plugin/install?src=http://evil.example/p.dll, vynamp://open?shell=1 e vynamp://auth/callback?code=abc&state=forjado | cada uma das 9 URIs é entregue ao handler isoladamente, uma vez como argv do processo e uma vez por ativação do shell, sem outra janela do app aberta | Links rejeitam path/shell/plugin/autoplay e callbacks falsos |
| `TASK-E16-10-AC-2` | Windows 11 x86_64 sem HKCU/Software/Classes/http/shell/open/command e sem HKLM/Software/Classes/http/shell/open/command apontando para o Vynamp, e sem nenhuma entrada em Software/RegisteredApplications que declare URLAssociations para http ou https | o instalador 1.0.0 roda, o app é aberto, um link https://vynamp.example/v1/open?id=x é clicado no navegador padrão e depois colado em outro aplicativo, com HKCU, HKLM e RegisteredApplications exportados antes e depois | não registram HTTP global |
| `TASK-E16-10-AC-3` | envelope OpenRequest v1 com request_id UUID fixo, origin deeplink e locator file:///D:/Music/a.flac, mais um bloco permissions=rwkx que o cliente anexa ao corpo embora o schema v1 tenha additionalProperties=false e não declare permissions, com o broker conhecendo o peer e um grant real do perfil limitado a D:/Music | o broker valida o peer e o grant real do perfil contra o envelope recebido, e o app tenta ler D:/Music/a.flac, que está dentro do grant, e D:/Music/../Secret/b.flac, que resolve para fora dele, na mesma sessão | intent concedida não amplia filesystem grants. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | harness novo tests/e16_10_links.rs, que não existe hoje, lendo tests/fixtures/links-hostis.txt e entregando cada URI por argv e por ativação do shell, mais o hash de `MSYS_NO_PATHCONV=1 reg export HKCU\Software\Classes`, de HKLM\Software\Classes e dos bytes de packaging/macos/Info.plist antes e depois de rodar o instalador; roda como `cargo test --test e16_10_links -- --nocapture` | esperado 9 de 9 URIs recusadas com LinkRejected e 0 efeitos, sendo 0 leituras de C:/Windows/System32/cmd.exe, 0 plugins instalados e 0 reproduções iniciadas, 0 diferenças nos dumps de HKCU, HKLM e Info.plist para http e https, e 1 leitura autorizada em D:/Music/a.flac contra 1 negada em D:/Music/../Secret/b.flac; hoje o repo vynamp-player tem 1 arquivo .rs, src/main.rs, com println!("vynamp_player 0.1.0"), zero dependências e nenhum tests/, e `cargo test` mede 0 testes, então o alvo e16_10_links responde 'no test target named'; não existe AppxManifest.xml em nenhum ponto do repo, packaging/macos/Info.plist tem 13 linhas e 0 CFBundleURLTypes, packaging/linux/vynamp.desktop não declara x-scheme-handler, e não há assetlinks.json nem apple-app-site-association, de modo que o app link HTTPS do AC-2 não tem domínio verificado para apontar |
| Hardware | parcial: as rotas, o state e o PKCE rodam neste Windows 11 Pro build 26300 x86_64, onde reg export e a leitura de HKCU, HKLM e RegisteredApplications funcionam, e hoje HKCU tem 965 chaves em Software/Classes com 0 delas de vynamp e nenhuma entrada de HKLM ou HKCU em RegisteredApplications declara URLAssociations para http ou https, o que dá o estado de partida do AC-2; HKLM aponta http e https para o msedge.exe e HKCU declara http e https só com URL Protocol vazio, e esse estado de terceiro deve permanecer intocado. Faltam aparelho Android (adb não está instalado) e máquina macOS, e o app link HTTPS exige domínio verificado com assetlinks.json e apple-app-site-association, inexistentes no repo, então o AC-2 mede a ausência de registro HTTP global no Windows e nada além | ausência é limitação registrada, não aprovação |

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
