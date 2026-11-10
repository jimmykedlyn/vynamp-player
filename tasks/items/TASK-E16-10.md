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

- **Não** implementa identidade, troca de código por token nem autorização do provider; isso é TASK-E09-03, e aqui se verifica apenas state e PKCE do lado do app antes de qualquer troca de código.
- **Não** registra ProgID, menu de contexto nem associação de arquivo no Windows; o registro de shell é de TASK-E16-03 e a remoção dele é de TASK-E16-08, então esta TASK só declara a rota.
- **Não** define a semântica de Open e Enqueue nem o formato do envelope de ativação; isso é TASK-E16-01, e o deep link apenas produz um OpenRequest que aquele broker valida.
- **Não** implementa MediaStore, SAF nem o modelo de permissão do Android; grants de filesystem no Android são de TASK-E13-04, e aqui só se prova que a intent não os amplia.
- **Não** gera o manifesto OS nem as declarações de handler por alvo; isso é TASK-E16-09, e uma rota validada aqui não substitui a declaração assinada do build.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e16-10-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E16-10-AC-1` | build desktop com o scheme vynamp registrado e as rotas allowlisted v1/open e v1/auth/callback, mais um arquivo com 9 links hostis, entre eles vynamp://open?locator=file:///C:/Windows/System32/cmd.exe, vynamp://open?autoplay=1, vynamp://plugin/install?src=http://evil.example/p.dll, vynamp://open?shell=1 e vynamp://auth/callback?code=abc&state=forjado | cada um dos 9 links é entregue ao handler isoladamente, uma vez por argv do processo e uma vez por ativação do shell | Links rejeitam path/shell/plugin/autoplay e callbacks falsos |
| `TASK-E16-10-AC-2` | Windows 11 x86_64 limpo, sem HKCU/Software/Classes/http/shell/open/command e sem HKLM/Software/Classes/http/shell/open/command apontando para o Vynamp, e sem chave em Software/RegisteredApplications declarando handler http ou https | o instalador 1.0.0 roda, o app é aberto e um link https://vynamp.example/v1/open?id=x é clicado no navegador padrão e depois colado em outro aplicativo | não registram HTTP global |
| `TASK-E16-10-AC-3` | envelope OpenRequest v1 com request_id fixo UUID e locator file:///D:/Music/a.flac, origin declarada como site, e um bloco permissions=rwkx com grant em D:/Music fornecido pelo próprio cliente | o broker valida o peer e o grant real do perfil e o app tenta ler D:/Music/a.flac e D:/Music/sub/b.flac na mesma sessão | intent concedida não amplia filesystem grants. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | teste em tests/e16_10_links.rs lendo tests/fixtures/links-hostis.txt com 9 URIs e comparando dumps de HKCU/Software/Classes, de HKLM/Software/Classes e de Info.plist antes e depois do install, rodando como cargo test --test e16_10_links -- --nocapture | esperado 9 de 9 links rejeitados com LinkRejected e 0 efeitos colaterais, sendo 0 arquivos lidos, 0 plugins instalados e 0 reproduções iniciadas, 0 diferenças nos três dumps para http e https, e 1 leitura autorizada contra 1 negada para D:/Music/sub/b.flac; hoje o repo vynamp-player tem apenas src/main.rs com um println da versão 0.1.0, zero dependências e nenhum tests/, então o harness não existe nem roda |
| Hardware | nenhum: as rotas e o OAuth state e PKCE são exercitados por processo e por dump de registro neste Windows 11 x86_64; a verificação de app links HTTPS reais em Android e macOS exige aparelhos ausentes neste ambiente e fica declarada como pendência | ausência é limitação registrada, não aprovação |

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
