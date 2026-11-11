# TASK-E13-05 — Entregar interface phone e navegação acessível

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E13-05` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E13-05` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M1 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E13-02`, `TASK-E02-02` |
| Bloqueia | `TASK-E13-08`, `TASK-E16-06`, `TASK-E17-10` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 3–8 dias úteis |
| Risco | alto |

## Entrega

Shell Compose com tokens compartilhados, transporte/fila/EQ básico e TalkBack.

## Boundary

- **Não** reusa a geometria do desktop: `platforms/01-matriz-suporte.md` §21 pede
  compartilhar algoritmos e contratos, não copiar o layout.
- **Não** entrega EQ avançado: o transporte e o EQ básico cabem aqui; a cadeia
  completa é `TASK-E04-01`.
- **Não** implementa biblioteca nem tags: a tela lê o que `TASK-E03-01` expõe.
- **Não** valida áudio: a acessibilidade aqui é de navegação e leitura, conforme
  `design/03-acessibilidade.md`, e não de sinal.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e13-05-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E13-05-AC-1` | o shell Compose num telefone com a densidade mais alta suportada | a árvore de layout é medida | Targets de 48 dp |
| `TASK-E13-05-AC-2` | a tela de transporte e fila com o tamanho de fonte do sistema em 200% | a tela é medida e percorrida com TalkBack | fonte 200% faz reflow |
| `TASK-E13-05-AC-3` | a Activity recriada por rotação com o serviço tocando | a fila exibida é comparada com o snapshot do engine | nenhuma fila duplicada |
| `TASK-E13-05-AC-4` | a reprodução correndo durante uma rotação e um back | a posição e o device frame são lidos antes e depois | back/rotation não reiniciam áudio |
| `TASK-E13-05-AC-5` | o gabarito phone do pacote de referência e a tela renderizada | a paleta e a geometria são comparadas | paleta e geometria phone conferidas. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test` no repo do player mais o Accessibility Scanner em viewport de telefone | cada controle é listado com nome e frame dentro do viewport e o TalkBack opera o transporte; hoje a shell Compose não existe |
| Hardware | um aparelho Android real com TalkBack; **nenhum** disponível neste ambiente | ausência é limitação registrada, não aprovação |

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
