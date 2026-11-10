# TASK-E20-02 — Entregar AVAudioSession e playback em background

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E20-02` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E20-02` |
| Critérios da spec | `platforms/05-ios.md` §2 |
| Fase | F6 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E20-01`, `TASK-E01-03` |
| Bloqueia | `TASK-E20-04`, `TASK-E20-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–8 dias úteis |
| Risco | alto |

## Entrega

Sessão de áudio configurada por categoria, Now Playing com MPRemoteCommandCenter, e reprodução que sobrevive ao app ir para segundo plano.

## Boundary

- **Não** constrói o shell de interface; o Now Playing é superfície do sistema,
  não tela do app, e a tela é `TASK-E20-03`.
- **Não** decide a saída de áudio por conta própria: o backend CoreAudio vem de
  `TASK-E20-01`.
- **Não** toca em biblioteca nem em permissões de arquivo: isso é `TASK-E20-04`.
- **Não** trata `apps/converter` nem `apps/restore`. No iOS os dois **não existem**,
  por decisão de `platforms/05-ios.md` §1.2, e não há build a produzir para eles.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e20-02-claude-<slug>`)
- [ ] Hardware Apple, Xcode e assinatura disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E20-02-AC-1` | a reprodução ativa em primeiro plano | o app é enviado para segundo plano | a sessão de áudio mantém o playback sem interrupção |
| `TASK-E20-02-AC-2` | a reprodução em andamento | uma chamada telefônica interrompe a sessão e depois termina | interrupção pausa e retoma na posição em que estava |
| `TASK-E20-02-AC-3` | a MediaSession publicada e a fila em Rust como fonte única | play, pause e next são acionados pelo lockscreen e pelo fone | comandos remotos operam a fila única em Rust |
| `TASK-E20-02-AC-4` | a reprodução encerrada pelo usuário | a sessão de áudio é inspecionada depois do encerramento | a sessão não vaza ao terminar a reprodução. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `xcrun simctl` para background e `AVAudioSession` lida no log do dispositivo | a reprodução continua em background e a sessão aparece desativada ao fim — lido do log, não "parece certo" |
| Hardware | aparelho Apple real com Xcode e assinatura; **nenhum** disponível neste ambiente (Windows) | ausência é limitação registrada, não aprovação |

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
