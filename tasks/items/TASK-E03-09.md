# TASK-E03-09 — Entregar letras, bookmarks e histórico

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E03-09` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E03-09` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E03-04`, `TASK-E03-02` |
| Bloqueia | `TASK-E03-10`, `TASK-E08-08`, `TASK-E08-09` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

LRC/plain lyrics, progress/bookmarks, local play history e opt-in providers.

## Boundary

- **Não** entrega os adapters de scrobble Last.fm, ListenBrainz, Libre.fm e Discord; publicar em provedor é TASK-E08-08, e o histórico desta TASK é local e não sai da máquina por padrão.
- **Não** implementa agrupamento de audiobooks, capítulos de livro nem sync de progresso entre dispositivos; isso é TASK-E08-09, e bookmark de faixa musical não é progresso de livro.
- **Não** cria os adapters de leitura de tag que expõem o campo de letra embutida nem a precedência de capa; os adapters são TASK-E03-04, e aqui o parser LRC apenas consome o que eles expõem.
- **Não** persiste a fila, o seek position, o shuffle seed ou o repeat policy; a QueueSession é TASK-E03-02, e guardar posição para recovery é item de fila, não de histórico.
- **Não** desenha o painel de letras, o inspector nem o tema; a UI do Library Workspace é TASK-E02-05, e o contrato de linha ativa sai daqui como dado, não como widget.
- **Não** faz backup, export ou rollback do banco com ratings e bookmarks; a sobrevivência a upgrade é TASK-E03-10, e o AC-2 aqui só prova que o rescan não recria o bookmark.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e03-09-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E03-09-AC-1` | um arquivo com LRC embutido de 40 linhas timestampadas, engine tocando com sink nulo a 48 kHz e clock consumido avançando 1.000 ms por tick; no meio da reprodução o relógio do sistema é adiantado em 2 s | o painel de letras pede a linha a cada tick pela chamada lyrics.line_at(consumed_ms) e o resultado de 100 ticks é comparado | Synced lyrics seguem clock consumido |
| `TASK-E03-09-AC-2` | um MediaItem com 3 bookmarks (00:00:42, 00:12:05, 01:03:11) e rating 4, cujo arquivo é movido de a/track.flac para a/2024/track.flac e reescrito por um editor externo com nova mtime | o rescan incremental do root roda e o item é aberto pela view Músicas | rescan preserva bookmark |
| `TASK-E03-09-AC-3` | uma trilha de 300 s com PlayHistory vazio e a regra de played em 50 por cento ou 240 s, tocada de 0 a 285 s por seek, com 5 s ouvidos antes de um segundo seek para 0 e mais 10 s ouvidos | a sessão de playback é encerrada e os eventos de consumo e conclusão são lidos do PlayHistory | seek não conta como consumo |
| `TASK-E03-09-AC-4` | Windows 11 x86_64 com a interface de rede desabilitada, um provedor remoto de letras configurado com o opt-in desmarcado e uma trilha sem letra embutida e sem arquivo .lrc ao lado | a trilha é tocada, o painel de letras é aberto e o log de sockets é inspecionado | sem rede involuntária. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test -p vynamp-player --test lyrics_history com clock consumido sintético de 1.000 ms por tick e sink nulo, sobre fixtures tests/fixtures/lyrics/synced_40.lrc, tests/fixtures/bookmarks/rename_rescan (3 bookmarks e rating 4), tests/fixtures/history/seek_300s e tests/fixtures/lyrics/remote_only com a interface de rede desabilitada | espera-se a linha ativa igual à de maior timestamp menor ou igual a consumed_ms em 100 de 100 ticks, com a mesma linha antes e depois de adiantar o relógio do sistema em 2 s; os 3 bookmarks em 42 s, 725 s e 3.791 s e o rating 4 continuam no mesmo MediaItem depois do rescan do rename; na trilha de 300 s, 5 s ouvidos após seek para 285 s mais 10 s ouvidos após seek para 0 dão consumo de 15 s e o evento played sai falso; com opt-in desmarcado o log mostra 0 conexões e o painel mostra ausência de letra. O repo vynamp-player tem hoje 1 arquivo .rs (src/main.rs) com println!("vynamp_player 0.1.0"), zero dependências e nenhum tests/, logo não existe parser LRC nem tabela de bookmark. |
| Hardware | nenhum: o clock consumido é injetado por um clock fake e a saída de áudio é um sink nulo; a ausência de rede é obtida desabilitando a interface no Windows 11 x86_64, sem exigir placa de áudio profissional | ausência é limitação registrada, não aprovação |

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
