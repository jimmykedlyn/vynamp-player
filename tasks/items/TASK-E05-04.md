# TASK-E05-04 — Entregar buffers, cache e downloads

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E05-04` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E05-04` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E05-02` |
| Bloqueia | `TASK-E05-05`, `TASK-E05-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | alto |

## Entrega

Buffers com limites de tempo/bytes, cache LRU em disco e isolamento por conta.

## Boundary

- **Não** implementa o Range HTTP nem fala com o servidor; é TASK-E05-02, e o buffer consome bytes que aquele adapter já entregou, incluindo o tratamento de 206 e 416.
- **Não** implementa feeds nem episódios de podcast; é TASK-E05-05, mesmo que também precise de cache e de download para tocar offline.
- **Não** implementa HLS nem a janela de segmentos com timeshift; é TASK-E05-06, e lá o descarte obedece a sequence e discontinuity, não a byte count.
- **Não** autoriza nem nega a purga por conta via broker de permissões; é TASK-E07-03, e aqui a purga recebe o identificador de conta já autorizado.
- **Não** purga o banco SQLite nem a biblioteca local; é TASK-E03-01, e a mídia local é justamente o que o logout não pode tocar.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e05-04-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E05-04-AC-1` | VOD MP3 de 128 kbps reproduzindo com buffer alvo de 3 s, num fixture que entrega 40 kbps por 5 s até esvaziar o buffer e depois volta a 128 kbps | a rede se recupera e o player retoma após o underrun, e o teste compara a sequência de índices de amostra PCM entregue ao output antes e depois da queda | Rede lenta pode recuperar sem repetir amostras |
| `TASK-E05-04-AC-2` | stream Opus de 256 kbps alimentado por fixture rápido com caps configurados em 32 MiB codificados e 16 MiB decodificados, e rate PCM de 48 kHz estéreo | o teste empurra 40 MiB codificados e mede o residente de cada buffer a cada 64 KiB, até o produtor ficar bloqueado | respeitar limites de 32/16 MiB |
| `TASK-E05-04-AC-3` | duas contas A e B com cache privado em disco contendo 12 MiB de A e 30 MiB de B, mais 3 arquivos de mídia local de 8 MiB cada fora de qualquer diretório de cache | logout da conta A executa a purga do cache privado e o teste conta arquivos e bytes do cache e da pasta de mídia antes e depois | logout limpa cache privado sem apagar mídia local. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | tests/buffers_cache.rs exercitando três cenários com cargo test --test buffers_cache: fixture lento de 40 kbps por 5 s seguido de 128 kbps, saturação com 40 MiB codificados e cache em diretório temporário com duas contas | a sequência após o underrun continua no sample de índice N sem repetir nem pular, o pico medido fica em 32 MiB codificados e 16 MiB decodificados sem nenhum byte acima do cap, e o logout de A remove 12 MiB do cache privado de A, preserva os 30 MiB de B e não toca os 3 arquivos de mídia local de 8 MiB; hoje o repo vynamp-player tem 1 arquivo .rs com println e nenhum tests/, então nada disso existe. |
| Hardware | nenhum: os streams são gerados pelo próprio teste, o cache fica em diretório temporário e nenhuma placa de áudio é aberta | ausência é limitação registrada, não aprovação |

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
