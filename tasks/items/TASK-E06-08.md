# TASK-E06-08 — Entregar galeria local e recovery visual

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E06-08` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E06-08` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E06-07` |
| Bloqueia | `TASK-E06-09`, `TASK-E06-10`, `TASK-E09-09`, `TASK-E11-08` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Galeria, armazenamento por usuário e safe mode com tema builtin.

## Boundary

- **Não** prepara a publicação nem o upload do tema para o registry online de M4. O dono é `TASK-E06-09`; a galeria aqui é local e listar não publica.
- **Não** entrega o registry de temas e plugins que consome essa galeria. O dono é `TASK-E09-09`, que depende desta TASK; a galeria lê o perfil local e não fala com serviço remoto.
- **Não** valida fidelidade visual contra screenshots de referência. O dono é `TASK-E06-10`; reverter ao builtin prova recuperação, não fidelidade de layout.
- **Não** define a persistência de docking multi-monitor nem a geometria de painéis. O dono é `TASK-E02-08`; aqui só existe o clamp que mantém os controles acessíveis quando o monitor salvo desapareceu.
- **Não** prepara a documentação de lançamento de temas. O dono é `TASK-E11-08`, que depende desta TASK; safe mode é comportamento, e manual é outro artefato.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e06-08-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E06-08-AC-1` | tema `midnight-modern` 1.2.0 instalado e ativo com `anchor: 12` em layouts/classic.json, valor que panica ao montar o nó `panel`, e o builtin `graphite` presente em read-only | o app é iniciado e a primeira frame do tema ativo é montada | Falha no renderer reverte ao builtin |
| `TASK-E06-08-AC-2` | 3 pacotes presentes em `themes/` do perfil do usuário, incluindo `midnight-modern` 1.2.0 marcado como ativo, e o builtin `graphite` disponível | o app é iniciado com a flag `--safe-mode` | safe mode ignora pacotes do usuário |
| `TASK-E06-08-AC-3` | configuração salva com janela em 2560x1440 no monitor de coordenada x=3840 e zoom de UI em 200%, numa máquina com um único monitor 1920x1080 | o app é iniciado e a janela principal é restaurada a partir dessa configuração | monitor/zoom restaurados mantêm controles acessíveis. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | cargo test --test theme_recovery (fixtures: `fixtures/themes/panic-anchor.vyntheme` com `anchor: 12`, perfil de teste com 3 pacotes em `themes/` e `settings.json` com monitor x=3840 e zoom 200%) | hoje nenhum desses mecanismos existe: 1 arquivo `.rs` com `println!("vynamp_player 0.1.0")`, zero dependências e nenhum `tests/`; a evidência exigida é 1 panic capturado e troca para o builtin em até 1 frame com 0 xruns depois da troca, `--safe-mode` carregando 0 de 3 pacotes com os 3 arquivos ainda em disco e SHA256 inalterados, e janela em 1920x1080 com os retângulos de play, mute e stop inteiramente dentro dos limites do monitor. |
| Hardware | nenhum: monitor ausente, falha de renderer e safe mode são simulados por `settings.json` e fixtures de tema; nenhum teste abre segundo monitor físico ou dispositivo de áudio. | ausência é limitação registrada, não aprovação |

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
