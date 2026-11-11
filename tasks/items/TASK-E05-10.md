# TASK-E05-10 — Integrar provider API, offline e direitos

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E05-10` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E05-10` |
| Critérios da spec | `specs/streaming.md` §5 (rascunho, não aprovado) |
| Fase | M4 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E05-09`, `TASK-E09-02` |
| Bloqueia | `TASK-E08-09`, `TASK-E12-08` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 8–20 dias úteis |
| Risco | alto |

## Entrega

Adapter da plataforma, renovação de URLs, restrições e progresso de conteúdo longo.

## Boundary

- **Não** materializa o OpenAPI v1 nem escreve os contract tests do provider. É TASK-E09-02, a dependência: aqui é o adapter que consome o contrato, e redefinir schema aqui é exatamente o que faria mock e real divergirem, que é o que o contract test existe para impedir.
- **Não** implementa o HLS VOD assinado nem a CDN do lado servidor. É TASK-E09-06: o token e a expiração nascem lá; esta TASK só sabe renovar e reagir ao 401, e implementar o emissor aqui daria dois donos da mesma assinatura.
- **Não** entrega a política de SSRF, DNS e redirect nem a sanitização de log. É TASK-E05-09, a dependência direta: toda URL filha que este adapter resolver passa por ela, e duplicar a checagem aqui faria a URL renovada escapar da revalidação pós-rebind.
- **Não** faz agrupamento de audiobooks, capítulos, bookmarks nem velocidade por categoria. É TASK-E08-09, que esta TASK bloqueia: ela consome o progresso offline daqui, mas a semântica de retomada e de conflito de retrocesso pertence a ela.
- **Não** implementa DASH, WebDAV ou nuvens comerciais. É TASK-E12-08, também bloqueada por esta TASK: o adapter de provider daqui é o ponto de extensão, não o provider desses transportes.
- **Não** implementa o adapter Android, o serviço em background nem as condições de job do sistema. É TASK-E13-06: esta TASK entrega o core sem plataforma, e misturar permissão de rede móvel com direito de conteúdo confunde duas restrições diferentes.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e05-10-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E05-10-AC-1` | um HLS VOD de 40 segmentos de 3 s servido por mock provider com URL assinada válida por 60 s, onde o segmento 12 responde 401 com corpo token_expired na segunda tentativa e o mock emite um token novo a cada renovação | a reprodução chega ao segmento 12 com o token expirado há 2 s e o player emite o GET do segmento 12 sem nenhuma ação do usuário | Renovar token sem reiniciar indevidamente |
| `TASK-E05-10-AC-2` | dois rights descriptors do mesmo mock provider, faixa A com download=false e faixa B com download=true, com cache de disco e diretório de downloads separados e LRU de 2 GiB configurado | o usuário aciona Baixar na faixa A e em seguida Baixar na faixa B, e o gerenciador de downloads processa os dois pedidos até o fim da fila offline | download só se permitido |
| `TASK-E05-10-AC-3` | 3 arquivos baixados de 180 s cada no disco local, conta deslogada com o cache privado já purgado e interface de rede desabilitada no sistema | o usuário abre a biblioteca local, dá play no arquivo 2 e em seguida faz logout outra vez, ainda com a rede desligada | arquivos locais continuam após logout ou ausência de rede. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test --test provider_rights no repo vynamp-player com mock provider em loopback gerado a partir do OpenAPI de TASK-E09-02, configurado com token de 60 s, faixa A com download=false, faixa B com download=true e 3 arquivos locais de 180 s abertos com a rede desabilitada e a conta deslogada | esperado: 1 renovação por expiração e 0 reinícios do decoder, com o segmento 12 tocando 500 ms ou menos depois do 401; 0 byte gravado para a faixa A e 1 arquivo íntegro de 180 s para a B; 3 de 3 arquivos locais tocando os 180 s com a rede desligada e depois do logout. Hoje não existe: vynamp-player tem 1 arquivo .rs (src/main.rs) que só imprime vynamp_player 0.1.0, zero dependências e nenhum tests/, então nenhuma destas medidas é produzida; o OpenAPI de TASK-E09-02, de onde o mock deveria ser gerado, também ainda não existe, então até ele o mock é escrito à mão e o contract test que o prende ao schema está pendente |
| Hardware | nenhum: provider, rights e download rodam contra mock em loopback e arquivos em disco local, com a reprodução verificada por posição de stream e sem saída de áudio real. A ausência de rede do AC-3 é emulada negando connect e DNS na camada de socket do próprio cliente, não desligando a interface física do sistema: derrubar a NIC é ação de máquina e não é reproduzível em teste automatizado neste ambiente Windows | ausência é limitação registrada, não aprovação |

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
