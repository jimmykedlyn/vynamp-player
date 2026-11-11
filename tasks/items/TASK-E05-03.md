# TASK-E05-03 — Entregar rádio Icecast/SHOUTcast e ICY

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E05-03` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E05-03` |
| Critérios da spec | `specs/streaming.md` §5 (rascunho, não aprovado) |
| Fase | M2 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E05-02`, `TASK-E02-06` |
| Bloqueia | `TASK-E05-08`, `TASK-E10-04`, `TASK-E13-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | alto |

## Entrega

Parser ICY, metadados, estações favoritas, entrada de URL e indicador LIVE.

## Boundary

- **Não** grava a rádio em arquivo nem faz split por faixa; é TASK-E05-08, e o parser aqui entrega áudio ao decoder em tempo real, sem sink de disco.
- **Não** implementa a tela de Mini com preferências e estados de erro, nem o aviso inline de URL sem TLS; é TASK-E02-06, e aqui só se emite o evento que aquela tela consome.
- **Não** faz feeds de podcast nem RSS/Atom; é TASK-E05-05, e o formato lá é XML com enclosure, não stream contínuo com metaint.
- **Não** implementa HLS nem playlist adaptativa; é TASK-E05-06, e rádio ICY é uma conexão única sem segmentos nem master playlist.
- **Não** implementa a integração de streaming e downloads no app Android; é TASK-E13-06, e aqui o alvo é o cliente desktop sem aparelho móvel.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e05-03-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E05-03-AC-1` | resposta ICY com icy-metaint 8192 cujo bloco de metadados de 256 bytes chega partido em três escritas TCP de 100, 100 e 56 bytes, seguido de 8192 bytes de áudio MP3 de 44,1 kHz | o parser ICY consome o stream pelo reader em pedaços de 1024 bytes e cada bloco de áudio é contado e enviado ao sink do teste | Separar metadados fragmentados dos bytes de áudio |
| `TASK-E05-03-AC-2` | bloco de metadados ICY declarando comprimento de 65280 bytes, com markup script e StreamTitle sem terminador nulo, servido pelo fixture e com teto de tag configurado em 4096 bytes | o parser processa o bloco e aplica o teto de tamanho e a sanitização antes de expor a tag ao modelo de metadados | limitar tags inválidas |
| `TASK-E05-03-AC-3` | estação ICY de fixture que aceita a conexão, envia 4 KB e fecha o socket, com retry em 1, 2, 4, 8 e 16 s, teto de 30 s e máximo de 5 tentativas consecutivas | a conexão cai, a primeira reconexão falha, e Stop é chamado durante a espera de backoff de 4 s | reconexão limitada e cancelável |
| `TASK-E05-03-AC-4` | descritor de rádio ao vivo ICY sem Content-Length e com icy-br 128 declarado mas não confiável, reproduzindo por 45 s no fixture | a UI consulta duração, posição e o indicador LIVE depois de 45 s de reprodução | não inventar duração. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | fixture binário tests/fixtures/icy/fragmented.bin com header icy-metaint 8192 e bloco de 256 bytes partido em três escritas, mais o caso de comprimento 65280, servido por TcpListener efêmero e exercitado por cargo test --test icy | o separador entrega exatamente 8192 bytes de áudio por ciclo sem nenhum byte de tag no sink, o bloco de 65280 é truncado no teto de 4096 e rejeitado, o backoff registra tentativas em 1, 2 e 4 s e para em 5 sem chegar ao teto de 30 s, e duration permanece ausente em vez de zero; hoje o repo vynamp-player tem 1 arquivo .rs com println e nenhum tests/, então nada disso existe. |
| Hardware | nenhum: as estações são fixtures servidos por TcpListener local, nenhuma estação Icecast pública é contatada e a saída de áudio é descartada | ausência é limitação registrada, não aprovação |

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
