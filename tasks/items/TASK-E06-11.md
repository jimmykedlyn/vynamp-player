# TASK-E06-11 — Entregar os estilos de onda do MVP

> Autorada à mão no formato do passo 37, a partir da entrada em `tasks/backlog.json` do
> umbrella. Os campos de estado vêm do backlog e não devem ser editados aqui. A coluna
> `Then` é o requisito aprovado, copiado literal do backlog; `Given` e `When` foram
> autorados junto.

| Campo | Valor |
|---|---|
| ID | `TASK-E06-11` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E06-11` |
| Critérios da spec | `specs/temas.md` §5 (rascunho, não aprovado) |
| Fase | M2 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E06-05`, `TASK-E02-13`, `TASK-E02-16` |
| Bloqueia | — |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–10 dias úteis |
| Risco | alto |

## Entrega

Os 17 estilos de onda do MVP no player, com configuração por estilo persistida.

## Boundary

- **Não** mede nem descreve o MVP. As séries, os tetos e os parâmetros de cada estilo são
  de `TASK-E02-13`, que grava `movimento.json` e o comportamento de `waveform-styles.js`;
  esta TASK consome esses números, e medi-los de novo aqui daria duas referências para o
  mesmo estilo.
- **Não** constrói o canal de observação nem a FFT. O tap, o ring e a análise são de
  `TASK-E06-05`; os estilos leem amostras e espectro desse canal e nunca põem um nó de
  análise no caminho de reprodução.
- **Não** entrega presets de cena, taps limitados nem a certificação do visualizador:
  isso é `TASK-E17-09` e `TASK-E17-10`. A confusão é plausível porque os estilos de onda
  também reagem ao áudio, mas aqui o produto é a onda da barra de busca do MVP, não o
  visualizador em tela cheia.
- **Não** monta o harness de fidelidade visual. A comparação de pixel com o MVP roda no
  harness de `TASK-E02-16`; esta TASK fornece os estilos que ele compara.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e06-11-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E06-11-AC-1` | as 17 chaves de `defaultConfigs` de `waveform-styles.js` do MVP, com os parâmetros que `TASK-E02-13` grava no AC-3 | o player lista os estilos e lê a configuração padrão de cada um | Os 17 estilos existem com os parâmetros de `defaultConfigs` |
| `TASK-E06-11-AC-2` | os 14 estilos ao vivo, o degrau, o seno e a rajada de `assets/gabarito/movimento.json`, e o relógio e a cadência do protocolo de `TASK-E02-13` | cada estilo ao vivo recebe cada sinal e cada leitura gravada é comparada com a do gabarito, atributo e geometria | os 14 ao vivo reproduzem as séries de `movimento.json` dentro dos tetos da TASK-E02-13 |
| `TASK-E06-11-AC-3` | os estilos `original`, `dj` e `overview` e um arquivo local com envelope conhecido | cada um desses estilos é selecionado com o arquivo carregado, com e sem reprodução | os 3 de envelope desenham o envelope do arquivo |
| `TASK-E06-11-AC-4` | um estilo ao vivo tocando com movimento reduzido ligado, pela preferência do sistema ou pela opção do player | os redesenhos da onda são contados sob o relógio virtual | movimento reduzido leva a cadência a 100 ms |
| `TASK-E06-11-AC-5` | um estilo com um parâmetro alterado pelo editor de configuração do estilo | o player é fechado e aberto de novo | configuração por estilo é editável e persiste entre sessões |

## Notas de escopo

Fazem parte desta TASK, como no MVP, e ficam registrados aqui para não se perderem:

1. **Busca por arraste sobre a onda.** No MVP, a onda é a área de busca: arrastar busca, e
   o botão direito ou Shift+F10 abre o menu de estilos.
2. **Ramo de rádio de `bars-pulse`.** Com faixa de rádio, `bars-pulse` desenha barras por
   amostra em vez de escalar as barras do envelope. Esse ramo não tem série em
   `movimento.json`; o comportamento vem do AC-3 de `TASK-E02-13`.
3. **`peakDecay` sem efeito.** `spectrum-pulse` tem `peakDecay` em `defaultConfigs`, mas o
   código do MVP não o lê. A implementação replica esse comportamento ou registra a
   decisão de dar efeito ao parâmetro, com o motivo, antes do código.

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `assets/gabarito/movimento.json` e a descrição dos estilos de `TASK-E02-13`, gerados a partir do MVP no commit `0d9860b901510021bcca4c31a25014373ea7c452`, e não do código desta TASK | cada série dos 14 estilos ao vivo dentro do teto da `TASK-E02-13`; os parâmetros dos 17 estilos iguais aos gravados; 100 ms de cadência com movimento reduzido; a configuração alterada relida depois de reabrir o player. Hoje não há estilo de onda no player nem `movimento.json` no repo |
| Hardware | nenhum: as séries são numéricas e rodam com relógio virtual, sem dispositivo de áudio | ausência é limitação registrada, não aprovação |

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
