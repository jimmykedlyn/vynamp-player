# TASK-E08-03 — Entregar análise de loudness, ReplayGain e BPM

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E08-03` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E08-03` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E08-01`, `TASK-E04-01` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

Jobs com padrão/versão de análise, grouping de álbum e escrita opcional de tags.

## Boundary

- **Não** implementa a escrita em lote das tags calculadas (temporário no mesmo volume, sync, rename, backup e relatório). O dono é `TASK-E03-08`; aqui o job só produz o valor e a proposta de write, e trazer o write para cá faria dois donos do mesmo arquivo em disco — a confusão é plausível porque o AC-5 fala em write.
- **Não** aplica o ganho de ReplayGain na reprodução nem mexe no EQ10. O consumo do valor no playback é de `TASK-E04-01`, que é dependência desta TASK; aqui a análise é offline sobre arquivo, e misturar as duas coisas acoplaria o scan à thread de áudio.
- **Não** implementa a fila de jobs persistida, a retomada após crash nem o estado `queued/running/done` do job. Isso é de `TASK-E08-01`, também dependência; o scan registra seu relatório numa fila que já existe, e reimplementá-la aqui criaria um segundo formato de persistência.
- **Não** implementa o conversor nem o registry de encoders que o lote de ferramentas M3 também expõe. O dono é `TASK-E08-02`; a confusão é plausível porque os dois são jobs de lote sobre muitos arquivos com barra de progresso e cancelamento, mas aqui não se reescreve amostra nenhuma.
- **Não** implementa smart playlists nem filtros que usem o BPM ou o gênero calculados. O dono é `TASK-E03-07`; esta TASK só entrega o número com confiança, e transformar BPM em regra de playlist aqui faria a análise depender de schema de biblioteca.
- **Não** implementa o envio desses campos a scrobblers ou presence. O dono é `TASK-E08-08`; o resultado da análise é local e a confusão é plausível porque BPM e loudness são metadados que serviços externos também recebem.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e08-03-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E08-03-AC-1` | Um corpus de fixtures em `fixtures/loudness/` com três arquivos: um WAV PCM 16 bits 48 kHz estéreo de 20 s com senoide de 997 Hz a -20 dBFS, um FLAC 24 bits 96 kHz estéreo de 180 s e um FLAC 16 bits 44,1 kHz mono de 12 s, cada um com integral de loudness e true peak de referência BS.1770-4 anotados em manifesto JSON. | O job de análise roda o padrão `ebur128` na versão `BS.1770-4` sobre as três fixtures e grava loudness integrado e true peak no relatório do job. | Loudness conferido contra fixtures |
| `TASK-E08-03-AC-2` | Uma árvore com seis FLAC de 30 s em `fixtures/grouping/`: cinco com `album` = `Grouping Test` e `album artist` = `Fixture Ensemble` (a faixa 3 com `artist` diferente, e a faixa 5 com `album artist` vazio) e um sexto com o mesmo `album` mas `album artist` = `Outro Ensemble`, todos com `musicbrainz_albumid` distintos. | O job roda em modo de grouping por álbum e grava o ganho de ReplayGain de álbum por grupo no relatório do job. | agrupamento correto |
| `TASK-E08-03-AC-3` | Duas fixtures de 60 s a 44,1 kHz 16 bits: uma com batida sintética a cada 500 ms (120 BPM exatos) e outra com a mesma batida mais ruído branco a -30 dBFS e swing de 8 por cento. | O analisador experimental de BPM roda sobre as duas fixtures e o resultado é gravado no relatório do job. | BPM inclui confiança |
| `TASK-E08-03-AC-4` | Engine tocando um FLAC 24 bits 96 kHz de 10 min com callback de 256 frames a 48 kHz (5,333 ms de orçamento por callback) e deadline de 10 ms, com o contador de overruns em zero. | Um scan de loudness de 500 arquivos é iniciado em thread de background enquanto a reprodução segue e o p95 do tempo de callback é medido por 60 s. | scan não interfere callback |
| `TASK-E08-03-AC-5` | Um job de análise concluído com ReplayGain track e album mais BPM calculados para 12 FLAC, nenhuma tag escrita e o plano de preview ainda não aceito pelo usuário. | O comando de aplicar do job é invocado diretamente, sem passar pela etapa de preview e confirmação. | write requer preview. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | Corpus `fixtures/loudness/` com manifesto JSON de referência BS.1770-4 e `cargo test --test loudness_fixtures` no repo `vynamp-player`, mais `git status --porcelain` sobre `fixtures/` depois de rodar o caso de write sem preview. | Loudness integrado de cada fixture dentro de mais ou menos 0,1 LU do manifesto e true peak dentro de mais ou menos 0,1 dBTP em 3 de 3 arquivos; grouping resolve em 2 grupos (5 faixas e 1 faixa) e 0 grupos; BPM da fixture limpa em 120 mais ou menos 1 com campo de confiança numérico em [0,1] e confiança menor que a da fixture limpa no caso com ruído; 0 overruns de callback e p95 abaixo de 5,333 ms com o scan ativo; 0 arquivos com mtime alterado após a tentativa de write sem preview. Hoje nada disso roda: o repo `vynamp-player` tem 1 arquivo `.rs` (`src/main.rs`) com um `println!` do texto `vynamp_player 0.1.0`, zero dependências e nenhum `tests/`, então não existe runner, fixture nem comando de análise para executar. |
| Hardware | nenhum: a análise é numérica sobre arquivo decodificado e o critério de callback é medido com sink nulo de 256 frames em 48 kHz; este ambiente Windows 11 x86_64 não tem placa de áudio profissional nem saída exclusiva, e nenhum critério desta TASK exige uma. | ausência é limitação registrada, não aprovação |

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
