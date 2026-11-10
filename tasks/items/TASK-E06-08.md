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

- **Não** instala tema de forma atômica nem faz rollback de versão. Staging, validação completa, commit por versão e restauração da versão anterior são `TASK-E06-02`; aqui o renderer falha depois de instalado e o recovery visual troca para o builtin sem desinstalar nada, o que é outro caminho.
- **Não** entrega o editor nem o preview de temas. É `TASK-E06-07`, dependência declarada desta; a galeria mostra e aplica pacotes prontos, e abrir edição a partir dela faria a galeria depender de undo e exportação que não são dela.
- **Não** implementa o registry online, a assinatura de publisher nem a revogação. É `TASK-E09-09`; aqui a galeria é local, funciona sem conta e sem rede, e o estado de revogação chega como dado do registry, nunca como pacote baixado por esta TASK.
- **Não** implementa docking, snap de 12 DIP nem a persistência da geometria por monitor. `TASK-E02-08` é dona disso e entrega a geometria salva; o AC-3 desta TASK só prova que, ao consumir uma geometria inválida, os controles continuam acessíveis — reimplementar a restauração aqui daria duas políticas para a mesma janela.
- **Não** entrega o safe mode de plugins, com grants, disable e rollback de plugin. É `TASK-E07-07`; a expressão safe mode aparece nos dois aceites, mas lá o que se ignora são plugins e permissões e aqui são pacotes de tema, e misturar os dois faria um flag desligar coisas que o usuário não pediu.
- **Não** escreve o guia de recuperação nem a documentação de lançamento do safe mode. É `TASK-E11-08`, que descreve o fluxo e não o substitui; documentar o recovery sem implementá-lo é exatamente o que aquele boundary dela proíbe.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e06-08-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E06-08-AC-1` | o tema `midnight-modern` 1.2.0 instalado e ativo, com `anchor: 12` no `panel` de layouts/classic.json, valor que faz o renderer entrar em pânico ao montar o nó, o builtin `graphite` presente em read-only e o hash de cada arquivo do perfil anotado antes da abertura | o app abre e monta a primeira frame do tema ativo, o pânico na montagem é capturado e a UI resultante, o tema ativo e os arquivos do perfil são lidos depois da falha | Falha no renderer reverte ao builtin |
| `TASK-E06-08-AC-2` | 4 pacotes de tema de usuário em `themes/` do perfil, entre eles `midnight-modern` 1.2.0 marcado como ativo, o builtin `graphite` disponível e o hash dos 4 arquivos anotado antes da abertura | o app é iniciado com a flag `--safe-mode` e a galeria, o tema ativo e o diretório do perfil são lidos e comparados por hash com a fotografia feita antes da abertura | safe mode ignora pacotes do usuário |
| `TASK-E06-08-AC-3` | um `settings.json` com a janela em 2560x1440 na origem x=3840 e zoom de UI em 200%, gravado quando havia dois monitores, restaurado numa sessão com um único monitor de 1920x1080, e a mesma restauração repetida com a geometria salva inteira fora da área visível | o app restaura a janela nos dois casos e os retângulos de play, mute e stop do Classic Stack são medidos contra a área visível, com alvo, foco e alcance de teclado conferidos | monitor/zoom restaurados mantêm controles acessíveis. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --test theme_recovery` no repo `vynamp-player`, com o fixture `panic-anchor.vyntheme` que traz `anchor: 12`, um perfil de teste com 4 pacotes em `themes/` aberto com a flag `--safe-mode` e um `settings.json` com janela 2560x1440 em x=3840 e zoom 200% restaurado numa tela única de 1920x1080. Hoje nada disso existe: o repo do player tem um único `.rs` de 3 linhas e 54 bytes com `println!`, `[dependencies]` vazio e nenhum `tests/`; o único contrato de tema no disco, `schemas/theme-manifest.schema.json`, descreve o pacote mas não tem campo de tema builtin, de estado de falha nem de geometria salva; e o `schemas/theme-layout.schema.json` não tem campo anchor, então o `anchor: 12` do fixture nem é expressável no schema atual — não há galeria, não há perfil por usuário e safe mode só aparece em prosa de spec. | Esperado: com a falha plantada, o tema ativo cai para o builtin no primeiro frame, os 4 pacotes continuam contados no perfil, 0 arquivos são apagados e o hash de cada um é idêntico ao da fotografia anterior, e o log registra 1 entrada com diagnóstico sem stacktrace na UI; em safe mode, 0 dos 4 pacotes são carregados, a galeria lista só o builtin e o perfil sai com os mesmos hashes, provando que ignorar não é apagar; na restauração, o retângulo fica preso à área de 1920 por 1080 sem coordenada negativa e sem parte fora da tela, 0 controles do Classic Stack de `TASK-E02-03` ficam inalcançáveis, todo alvo essencial mede ao menos 24 DIP pelo `minimum_hit_dip` de `assets/design-tokens.json`, e o caso com a geometria inteira fora da tela termina com a janela centralizada e o foco no controle de play. Nada disso roda hoje: não há galeria, perfil por usuário nem geometria salva em código. |
| Hardware | segundo monitor com escala independente, indisponível: esta máquina reporta 1 monitor de 1920x1080 em Intel UHD Graphics 620, sem segundo monitor e sem sessão RDP, e o monitor que o AC-3 exige faltando é exatamente o cenário testado, então a restauração roda de fato na tela de 1920x1080 disponível e a remoção de um monitor real é simulada só pela geometria forjada em arquivo e fica registrada como limitação. Não há aparelho Android, aparelho iOS nem máquina macOS neste ambiente | ausência é limitação registrada, não aprovação |

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
