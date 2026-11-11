# TASK-E02-14 — Gerar gabarito de comportamento e preferências

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-14` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-14` |
| Critérios da spec | `specs/ui.md` §5 (rascunho, não aprovado) |
| Fase | M0 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E02-11` |
| Bloqueia | `TASK-E02-15` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | médio |

## Entrega

Roteiros de interação por teclado, ponteiro e toque com estado esperado e esquema das preferências persistidas com defaults e migração.

## Boundary

- **Não** implementa preferências de usuário no produto: entrega o esquema com
  defaults e migração; quem grava é `TASK-E02-06`.
- **Não** decide atalhos que `design/03-acessibilidade.md` não lista, e a
  navegação por teclado continua a que o guia fixa em vez de uma inventada aqui.
- **Não** cobre toque no desktop: o roteiro de toque é o de Android e iOS, e os dois
  são `TASK-E13-05` e `TASK-E20-03`.
- **Não** mede acessibilidade: o roteiro descreve o estado esperado; a verificação
  com leitor de tela é `TASK-E02-16` e os gates de plataforma.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e02-14-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E02-14-AC-1` | o inventário de controles do MVP extraído do pacote no SHA do pin | cada controle recebe roteiro de teclado, ponteiro e toque com o estado esperado | Cada controle do MVP tem roteiro com resultado verificável |
| `TASK-E02-14-AC-2` | todas as chaves de localStorage e os campos de preferência lidos ou escritos pelo MVP | o esquema de preferências é escrito com default, tipo e migração e confrontado com o inventário | o esquema cobre todas as chaves lidas ou escritas |
| `TASK-E02-14-AC-3` | o inventário de ações do MVP e os roteiros de AC-1 | a cobertura é conferida ação a ação | nenhuma ação fica sem roteiro. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | o roteiro de interação, executado sobre o MVP de referência por teclado, ponteiro e toque, comparando o estado esperado com o observado | cada passo tem estado esperado e observado registrados; hoje o roteiro não existe no repo do player |
| Hardware | um dispositivo Android para o roteiro de toque; **nenhum** disponível neste ambiente | ausência é limitação registrada, não aprovação |

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
