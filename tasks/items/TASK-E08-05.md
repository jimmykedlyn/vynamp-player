# TASK-E08-05 — Entregar sleep timer e scheduler

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E08-05` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E08-05` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E08-01`, `TASK-E01-03` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

Timers, alarmes, regras de horário/DST, quiet hours e persistência.

## Boundary

- **Não** implementa o `Stop`, o `Pause`, o fade nem a máquina de estados do transporte que o timer aciona. Isso é de `TASK-E01-03`, dependência declarada; aqui o scheduler emite o comando no instante certo, e reimplementar fade na camada de agenda criaria dois donos do tempo de reprodução.
- **Não** implementa a gravação de rádio ou de stream que uma regra pode iniciar. O dono é `TASK-E05-08`; a spec do scheduler lista iniciar e parar gravação entre as ações, então é plausível achar que o gravador entra junto, mas aqui só se dispara o comando que a TASK de gravação expõe.
- **Não** implementa a fila de jobs persistida que executa play de playlist ou station e retoma depois de reinício. O dono é `TASK-E08-01`, dependência; persistir `next_run_at` é desta TASK, persistir o job que ele dispara é da fila, e misturar os dois faria dois formatos de retomada.
- **Não** implementa a tela de preferências onde o usuário liga o timer, escolhe o teto de volume e autoriza o desligamento do SO. O dono é `TASK-E02-06`; aqui existe o estado e a confirmação exigida pela spec, não o formulário — a confusão é plausível porque o AC-3 depende de uma opção que o usuário liga em algum lugar.
- **Não** implementa notificação de alarme pelo centro de notificações do Windows, som de sistema nem SMTC. O dono é `TASK-E08-06`; alarme do scheduler é um evento de reprodução do próprio app, e é fácil achar que soar um alarme é integração de SO.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e08-05-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E08-05-AC-1` | Engine tocando um FLAC 16 bits 44,1 kHz de 3 min 42 s na posição consumida de 3 min 20 s, com a fila contendo 5 faixas seguintes e sleep timer configurado no modo fim de faixa com fade de 3 s. | A reprodução atravessa o último frame da faixa atual e o engine entra no limite com a próxima faixa da fila. | Parar ao fim de faixa funciona |
| `TASK-E08-05-AC-2` | Timezone explícita `America/New_York`, uma regra semanal de play às 02:30 e um relógio injetado por fixture que pode ser fixado em 2026-03-08 (salto 02:00 para 03:00, hora local inexistente) e em 2026-11-01 (retrocesso 02:00 para 01:00, hora local ambígua). | O scheduler calcula `next_run_at` para cada uma das duas datas e persiste o valor antes de dormir. | DST definido |
| `TASK-E08-05-AC-3` | Duas instalações com a mesma regra de stop às 23:00: na primeira a opção `desligar o computador ao parar` está desligada (padrão de fábrica) e na segunda foi ligada explicitamente depois da confirmação de configuração; o desligamento do SO é registrado por um sink de comando falso. | O instante agendado de cada instalação é atingido e a regra dispara. | desligar OS só se habilitado |
| `TASK-E08-05-AC-4` | Alarme às 07:00 sobre saída de 48 kHz estéreo com volume atual em 0,80 do máximo e teto de alarme configurado em 0,45, com fade de subida de 5 s. | O alarme dispara e o fade de 5 s sobe o ganho até o topo permitido, com o pico de saída medido amostra a amostra. | alarmes respeitam teto de volume. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --test scheduler_dst` no repo `vynamp-player` com o relógio injetado nos instantes 2026-03-08 02:30 e 2026-11-01 01:30 em `America/New_York`, playlist de 5 faixas terminando numa de 3 min 42 s e o sink de comando de SO falso contando chamadas. | `next_run_at` resolve para 2026-03-08 03:00 local e para a primeira ocorrência de 2026-11-01 01:30, sem pular regra e sem duplicar execução, com o valor persistido e relido após reabertura; 1 chamada de stop no frame final da faixa e 0 avanços para a faixa 2; 0 chamadas de shutdown na instalação com a opção desligada e 1 na instalação com a opção ligada; ganho do alarme limitado a 0,45 do máximo e 0 amostras acima de 0,45 depois do fade de 5 s. Hoje nada disso roda: o repo `vynamp-player` tem 1 arquivo `.rs` (`src/main.rs`) com um `println!` do texto `vynamp_player 0.1.0`, zero dependências e nenhum `tests/`, então não existe scheduler, relógio injetável nem persistência para exercitar. |
| Hardware | nenhum: o relógio, o fuso e o instante são injetados por fixture, e o desligamento ou suspensão do SO é observado por um sink de comando falso que nunca desliga esta máquina Windows 11 x86_64; nenhum critério abre placa de áudio profissional nem aparelho móvel. | ausência é limitação registrada, não aprovação |

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
