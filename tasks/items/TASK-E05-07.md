# TASK-E05-07 — Entregar HLS live, ABR e timeshift

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E05-07` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E05-07` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E05-06` |
| Bloqueia | `TASK-E05-09` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Janela live, escolha de qualidade por throughput e buffer DVR limitado.

## Boundary

- **Não** implementa o parser master/media de VOD, o tratamento de discontinuity nem byte ranges: é TASK-E05-06, de quem esta TASK depende, e a confusão é plausível porque a playlist de live é o mesmo parser rodando com janela que desliza.
- **Não** redefine a eviction do cache em disco de 2 GiB nem o isolamento por conta: a política LRU é TASK-E05-04, e o ring DVR de 30 min e 512 MiB parece um segundo cache a ser unificado com aquele.
- **Não** constrói o painel de saúde, o corpus de falhas de rede nem a sanitização de URLs nas métricas: é TASK-E05-09, e live latency exposta como estimativa soa como métrica de saúde desta TASK.
- **Não** trata renovação de auth por provider nem restrictions de conteúdo: são TASK-E05-10, e um 401 repetido numa variant ao vivo parece renovação de token a ser feita aqui.
- **Não** valida o codec das variants (AAC-LC ou HE-AAC em TS e fMP4): é TASK-E01-07, e escolher variant só por throughput faz a ABR trocar para um perfil que o backend não decodifica.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e05-07-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E05-07-AC-1` | tests/fixtures/hls/live_slide/ com media playlist que muda EXT-X-MEDIA-SEQUENCE de 900 para 903 entre requisições, segmentos de 2,0 s e ring DVR de 30 min com 90,0 s já gravados | o poller rebusca a playlist a cada target duration por 180,0 s e a saída é capturada em destino PCM nulo a 48 kHz | Mudança de sequência não duplica áudio |
| `TASK-E05-07-AC-2` | master com variants de 64, 128 e 256 kbit/s e usuário que escolheu 128 kbit/s no seletor de qualidade, com throughput EWMA de 5 Mbit/s no início e queda para 300 kbit/s na metade | 40 segmentos de 2,0 s são baixados em sequência sem erro de segmento e o log de decisão de ABR é coletado | seleção manual de qualidade é respeitada |
| `TASK-E05-07-AC-3` | janela live de 60,0 s (30 segmentos de 2,0 s) com ring DVR de 30 min e 512 MiB habilitado e live edge em 60,0 s | o usuário arrasta a barra para 120,0 s atrás do live edge e depois para 45,0 s atrás dele | seek só na janela disponível |
| `TASK-E05-07-AC-4` | reprodução pausada 45,0 s atrás do live edge em janela de 60,0 s, com 45,0 s de timeline no ring DVR e o edge avançando 4,0 s durante a pausa | o usuário aciona GoLive e o áudio volta a tocar | voltar ao live edge funciona. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test -p vynamp-player --test hls_live contra tests/fixtures/hls/live_slide/ (sequência 900 para 903, janela de 30 segmentos de 2,0 s, 3 variants de 64, 128 e 256 kbit/s) capturando em destino PCM nulo a 48 kHz | Hoje o repo vynamp-player tem 1 arquivo .rs e nenhum tests/, então o comando falha com error: no test target named hls_live. Depois: 8 640 000 amostras, exatamente 180,0 s para 180,0 s de timeline, com 180 blocos de 1 s de MD5 todos distintos (0 repetição na troca de sequência 900 para 903); a variant de 128 kbit/s em 40 de 40 segmentos e 0 trocas enquanto a seleção manual estava ativa; o seek de 120,0 s atrás do edge é limitado a 60,0 s de janela com 1 rebuffer e nenhum segmento fora do intervalo; GoLive salta 45,0 s e a captura mostra 45,0 s de timeline sem amostras repetidas. |
| Hardware | nenhum: a janela live é uma playlist local que desliza por fixture e a captura vai para destino PCM nulo; não há placa de áudio, aparelho Android, aparelho iOS ou máquina macOS envolvido. | ausência é limitação registrada, não aprovação |

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
