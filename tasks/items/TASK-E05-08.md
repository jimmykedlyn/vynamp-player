# TASK-E05-08 — Entregar gravação de rádios

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E05-08` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E05-08` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E05-03`, `TASK-E08-02` |
| Bloqueia | `TASK-E05-09` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Gravação passthrough ou encoding escolhido, divisão de arquivos/CUE e monitor de espaço.

## Boundary

- **Não** implementa o parser ICY, a contagem de metaint em bytes nem as estações favoritas: é TASK-E05-03, de quem esta TASK depende, e a cláusula título ICY não é fronteira exata convida a mexer no parser em vez de na gravação.
- **Não** implementa o registry de encoder nem a conversão para FLAC, WAV, Opus, Vorbis e MP3: é TASK-E08-02, e encoding escolhido na entrega parece pedido para construir o encoder dentro do player.
- **Não** define o cache LRU de 2 GiB nem o isolamento cache/downloads: é TASK-E05-04, e o monitor de espaço livre é facilmente confundido com a quota do cache em disco.
- **Não** decide direitos de download offline nem renovação de signed URL: é TASK-E05-10, e a proibição explícita do provider parece o mesmo rights descriptor que autoriza download offline.
- **Não** entrega o painel de saúde de stream, a contagem de rebuffer ou o bitrate estimado: é TASK-E05-09, e o meter aproximado da rádio fica lado a lado do indicador de gravação na mesma barra da UI.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e05-08-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E05-08-AC-1` | estação Icecast servida por fixture local cujo SourceDescriptor traz restrictions com recording = forbidden e rights com offline_download = false | o usuário aciona Gravar com a estação tocando e confirma a gravação na UI | Proibição explícita do provider bloqueia gravação |
| `TASK-E05-08-AC-2` | partição de fixture de 64 MiB em target/tmp/quota com 61 MiB ocupados e gravação WAV 16 bits 44,1 kHz estéreo a 176 400 B/s de um stream ICY contínuo | o monitor de espaço com intervalo de 1 s enxerga menos de 8 MiB livres e o writer continua até fechar o bloco de 4 096 bytes corrente | disco cheio encerra arquivo válido |
| `TASK-E05-08-AC-3` | stream ICY com metaint 8 192 bytes e blocos StreamTitle mudando de A para B e de B para C sempre em fronteiras de bloco, gravando 120,0 s em WAV 44,1 kHz | a gravação é encerrada e o divisor escreve o CUE com uma faixa por título, comparando cada marca com o offset de bytes do metadado que a originou | título ICY não é tratado como fronteira exata de samples. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test -p vynamp-player --test radio_record com tests/fixtures/ices/icy_metaint8192.raw, um descriptor com restrictions.recording = forbidden e uma partição de fixture de 64 MiB em target/tmp/quota com 61 MiB ocupados | Hoje o repo vynamp-player tem 1 arquivo .rs e nenhum tests/, então o comando falha com error: no test target named radio_record. Depois: 3 testes; a gravação proibida retorna RecordingForbidden e deixa 0 bytes criados em disco; o arquivo final abre com header RIFF íntegro e 3 087 000 bytes, isto é 17,5 s a 176 400 B/s e múltiplo exato do block align 4, sem cauda truncada; o CUE tem 3 faixas com INDEX 01 em múltiplos de 8 192 frames e REM ICY_TITLE_APPROX declarando incerteza de 8 192 frames, 185,8 ms a 44,1 kHz. |
| Hardware | nenhum: a fonte é uma fixture ICY em arquivo local e a quota de disco é uma partição de fixture; a gravação escreve em arquivo e não abre dispositivo de áudio de saída. | ausência é limitação registrada, não aprovação |

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
