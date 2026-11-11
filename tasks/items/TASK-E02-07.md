# TASK-E02-07 — Integrar cadeia de áudio e EQ avançado na UI

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-07` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-07` |
| Critérios da spec | `specs/ui.md` §5 (rascunho, não aprovado) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E02-06`, `TASK-E04-04`, `TASK-E04-06` |
| Bloqueia | `TASK-E02-09` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

Audio-chain inspector, editor de curva/bandas e device profiles UI.

## Boundary

- **Não** implementa o EQ paramétrico, os tipos de filtro, o Q por canal nem a precedência de preset. `TASK-E04-04` é a dona dos coeficientes e do schema; esta TASK desenha a curva a partir dos coeficientes que o nó declara, e calcular coeficientes aqui criaria uma segunda resposta para o mesmo filtro.
- **Não** decide nem implementa o modo transparente, o caminho inteiro de inteiros ou as condições ativas. `TASK-E04-06` é a dona; aqui só se mostra o que ele reporta e se pede a saída do modo quando o usuário liga o EQ, que é a condição de entrada descrita na spec do painel DSP §5.
- **Não** abre stream exclusivo, negocia taxa nem enumera dispositivos. `TASK-E04-05` é a dona da saída exclusiva, dos IDs estáveis e das capabilities; a UI desta TASK exibe a negociação que ele já fez e não tenta renegociar por conta própria.
- **Não** constrói o grafo DSP, a interpolação de parâmetros nem o crossfade da troca de modo. `TASK-E04-03` é a dona; o requisito de 50 ms de resposta vem de lá, e o editor desta TASK só emite a intenção e lê o estado efetivo.
- **Não** implementa os 64 bandas nem o PEQ de 64 filtros. A spec de layouts manda mostrar 64 como indisponível nesta versão até `TASK-E12-10`; exibir 64 sliders reais aqui seria entregar um marco de M5 em M3.
- **Não** importa AutoEQ nem cria perfis por dispositivo. `TASK-E04-09` é a dona dos perfis por dispositivo e do import com origem e licença; `TASK-E04-05`, apesar do título «saída exclusiva e device profiles», entrega só WASAPI exclusivo, IDs estáveis e capabilities, sem o perfil em si. O seletor desta TASK lista e aplica perfis que já existem, sem ler arquivo de fora.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e02-07-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E02-07-AC-1` | um FLAC de 96 kHz e 24 bits aberto e tocando em saída exclusiva negociada a 96 kHz, com a cadeia composta por decode, trim, ReplayGain, EQ-31 ativo, resampler final em passagem e o dispositivo escolhido, e o painel de cadeia aberto em 960×600 DIP | o áudio-chain inspector é aberto e cada nó é lido contra o `PathProperties` e a config de stream reais devolvidos pela engine e pela saída, incluindo taxa de origem, taxa entregue ao dispositivo, banda passante e se cada nó está ativo, bypass ou incompatível | Origem/transformações/saída real visíveis |
| `TASK-E02-07-AC-2` | a reprodução em modo transparente ativo, com o selo visível, o EQ plano e desligado, e o arquivo PCM de 24 bits sendo entregue sem conversão de precisão, com a lista de condições ativas iguais à de `TASK-E04-06` | o usuário move uma banda do editor de curva do EQ e depois liga o EQ pelo toggle, primeiro sem confirmar o diálogo e depois confirmando-o, lendo em cada passo o selo, o estado do EQ e as condições ativas reportadas pelo caminho de saída | EQ em transparente pede saída do modo |
| `TASK-E02-07-AC-3` | o editor avançado em 960×600 DIP com os cinco modos no seletor, o EQ paramétrico com 32 filtros de tipos diferentes, Q e canal por filtro, e o nó de EQ reportando coeficientes e curva calculada pelo próprio módulo de `TASK-E04-04` | a curva prevista desenhada é amostrada em 512 pontos logarítmicos de 10 Hz a min(40 kHz, 0,45 × Fs) e cada ponto é comparado com a resposta calculada fora do código de UI a partir dos mesmos coeficientes que o nó declara estar usando | curvas refletem filtros reais. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test -p vynamp-player-ui --test audio_chain --locked` com um `PathProperties` e uma `StreamConfig` de fixture injetados (FLAC 96/24 em exclusive, EQ-31 ativo), mais `cargo test -p vynamp-player-ui --test eq_curve --locked`, que monta o editor com o preset de 32 filtros, amostra a curva exibida em 512 pontos de 10 Hz a min(40 kHz, 0,45 × Fs) e compara com a cascata RBJ calculada em Python puro fora do código de UI, no mesmo molde do `tools/eq_ref.py` que `TASK-E04-04` especifica e que ainda não existe — `libs/engine` não tem diretório `tools/` | Esperado: cada nó do inspector tem rótulo, estado (ativo, bypass, incompatível) e latência declarada batendo com o `PathProperties` injetado, com taxa de origem 96 000 Hz e taxa entregue 96 000 Hz visíveis e nenhum nó inventado; com EQ ligado em transparente o diálogo aparece com o texto da spec do painel DSP §5, cancelar deixa o selo e o EQ intactos e confirmar remove o selo e acrescenta o EQ à lista de condições ativas; a curva exibida fica dentro de 0,1 dB da cascata de referência nos 512 pontos. Hoje nada disso existe: `apps/player` tem 1 arquivo `.rs` de 3 linhas, `Cargo.lock` com 1 pacote e nenhuma dependência de UI; em `libs/engine` há 1 arquivo `.rs` de 6 linhas só com `pub const VERSION` e 0 testes; em `libs/output` o `NullHost` existe e o dispositivo virtual dele reporta `os_mixer_bypassed` com default `Yes`, enquanto `PathProperties::unknown()` devolve `Tri::Unknown`, e não há uma ocorrência sequer de `transparent`, `bit_perfect` ou badge em `crates/`. Das metas, 32 filtros vêm de `design/02-layouts.md` («até 32 filtros»); 512 pontos e 0,1 dB são a convenção que o oráculo de `TASK-E04-04` fixa, e nenhum dos dois foi medido aqui. |
| Hardware | precisa de DAC USB com saída exclusiva e, para a parte de caminho inteiro do AC-2, de interface de captura digital a jusante; não disponível aqui — esta máquina tem só 'Conexant ISST Audio' e 'Áudio Intel(R) para telas', dispositivos internos sem prova de exclusivo nem de bit-perfect. O que é provável sem esse hardware é a árvore do inspector e a curva do EQ, que rodam sobre fixtures injetadas | ausência é limitação registrada, não aprovação |

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
