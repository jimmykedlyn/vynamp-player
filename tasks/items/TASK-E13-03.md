# TASK-E13-03 — Entregar audio focus e rotas Android

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E13-03` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E13-03` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M1 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E13-02`, `TASK-E01-05` |
| Bloqueia | `TASK-E13-06`, `TASK-E13-07`, `TASK-E17-19` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–8 dias úteis |
| Risco | alto |

## Entrega

AudioManager, foco/noisy/Bluetooth e output clock com formato negociado.

## Boundary

- **Não** escolhe o backend de saída. AAudio veio de `TASK-E13-01`; esta TASK trata
  foco, ruído e rota.
- **Não** implementa a sessão de mídia: `TASK-E13-02`.
- **Não** promete latência de desktop: a meta é medida no aparelho de referência.
- **Não** cobre Bluetooth de áudio de alta qualidade além do que o codec do sistema
  entrega: SBC/AAC são o padrão e LDAC é do aparelho.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e13-03-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E13-03-AC-1` | o engine tocando e um segundo app pedindo foco | cada tipo de perda é solicitado: AUDIOFOCUS_LOSS, AUDIOFOCUS_LOSS_TRANSIENT e AUDIOFOCUS_LOSS_TRANSIENT_CAN_DUCK | Definir loss/transient/duck |
| `TASK-E13-03-AC-2` | a reprodução num fone Bluetooth e o dispositivo desconectando | a rota de saída é consultada imediatamente depois da desconexão | unplug pausa sem tocar speaker |
| `TASK-E13-03-AC-3` | a callback de áudio com a rota invalidada | o backend reabre o dispositivo | desconexão recupera fora da callback |
| `TASK-E13-03-AC-4` | o output Android com um arquivo de 44,1 kHz e um dispositivo negociado em 48 kHz | o painel de saída lê taxa, formato e modo de compartilhamento | taxa e sharing reais visíveis. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test` das transições de foco e uma passada com fone com fio, Bluetooth e chamada telefônica, lendo `AudioManager` no log | cada transição aparece no log com o estado esperado e o clock negociado é o do formato; hoje nenhuma TASK de Android tem código no repo |
| Hardware | aparelho Android real com fone Bluetooth e outro com fio; **nenhum** disponível | ausência é limitação registrada, não aprovação |

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
