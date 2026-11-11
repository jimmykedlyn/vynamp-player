# TASK-E20-04 — Entregar acesso a arquivos e biblioteca sob sandbox

> Gerado do `tasks/backlog.json`, que é a fonte de status. A coluna `Then` é o
> requisito aprovado, copiado literal do backlog; `Given` e `When` foram autorados
> na criação da TASK.

| Campo | Valor |
|---|---|
| ID | `TASK-E20-04` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E20-04` |
| Critérios da spec | `platforms/05-ios.md` §2 |
| Fase | F6 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E20-02`, `TASK-E02-05` |
| Bloqueia | `TASK-E20-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–6 dias úteis |
| Risco | médio |

## Entrega

Importação por document picker, Core File Provider e MediaPlayer, com biblioteca persistida dentro do container do app.

## Boundary

- **Não** cria loja de mídia própria nem scanner do sistema: o acesso é por
  document picker, Core File Provider e MediaPlayer, como a spec determina. O scan de
  roots é `TASK-E03-03`, que já é dono do import incremental, e duplicá-lo aqui criaria
  dois donos do mesmo índice.
- **Não** guarda cópia fora do container do app: a sandbox é o limite, não uma
  conveniência a contornar. O limite e a prova de isolamento são `TASK-E11-02`, e
  afrouxá-lo aqui seria política de segurança decidida na tela.
- **Não** implementa sincronização nem nuvem: isso não está em FASE 6, e o sync é `TASK-E09-08`.
- **Não** trata credenciais de serviço de streaming, que pertencem a `TASK-E05-10`.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e20-04-claude-<slug>`)
- [ ] Hardware Apple, Xcode e assinatura disponíveis, ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E20-04-AC-1` | o app sem permissão de arquivo concedida | o usuário escolhe um arquivo pelo document picker e o app é reaberto | arquivo escolhido pelo usuário persiste no container |
| `TASK-E20-04-AC-2` | um arquivo já importado e a permissão revogada | o app tenta abrir o arquivo de novo | permissão revogada vira erro tipado, não crash |
| `TASK-E20-04-AC-3` | a biblioteca com 10 mil arquivos importados | a listagem é aberta | lista de 10 mil entradas rola sem carregar todas de uma vez |
| `TASK-E20-04-AC-4` | o container do app e um caminho fora dele | o código tenta abrir o caminho externo | nenhuma leitura fora do container. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `xcrun simctl get_app_container` apontando para o arquivo importado | o arquivo está dentro do container retornado pelo próprio comando |
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
