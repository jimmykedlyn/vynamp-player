# TASK-E02-08 — Entregar docking e persistência multi-monitor

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E02-08` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E02-08` |
| Critérios da spec | `specs/ui.md` §5 (rascunho, não aprovado) |
| Fase | M3 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E02-06` |
| Bloqueia | `TASK-E02-09` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | médio |

## Entrega

Painéis destacáveis, snap12DIP, saved geometry e retorno ao monitor disponível.

## Boundary

- **Não** entrega o Mini nem a navegação de Preferências que originam as janelas destacáveis. `TASK-E02-06` é a dona do Mini 420×112 e das Preferências; aqui o que se destaca é painel do layout já existente, e nascer uma segunda janela de Mini faria duas telas donas do mesmo modo.
- **Não** implementa a cadeia de áudio que corre durante o resize. O AC-3 mede que o áudio sobrevive ao docking; quem garante o contrato RT, a thread de callback e o ring é `TASK-E01-02` em `vynamp-output`, e quem compila o grafo é `TASK-E04-03`. Esta TASK não otimiza a callback nem reduz alocação nela.
- **Não** persiste o esquema de preferências como um todo nem decide migração de chaves. `TASK-E02-14` define o esquema com defaults e migração e `TASK-E11-05` grava config; aqui se acrescenta só a geometria por monitor, dentro do esquema que eles já definiram.
- **Não** reposiciona nem recupera janelas no Android. `TASK-E13-05` é a dona da interface phone e `TASK-E13-09` do tablet/foldable, e a spec de mobile diz explicitamente que em telefone não há window docking; reusar este código lá contradiria a spec.
- **Não** constrói o canvas do grafo DSP nem a geometria de nós dentro do editor. O editor e a cadeia nativa são `TASK-E02-07`, o grafo compilado é `TASK-E04-03` e o snap de nó não é snap de janela; esta TASK move só a janela que contém o painel e não toca em coordenada de nó. A confusão é plausível porque a spec do painel DSP §9 descreve nós arrastáveis com zoom e pan, gesto que se parece com arrastar e encaixar janela.
- **Não** valida o retorno ao monitor no Linux, macOS ou iOS. `TASK-E12-01`, `TASK-E12-02` e `TASK-E20-01` cobrem essas plataformas; a persistência por monitor desta TASK é medida no Windows, e a especificação exige registrar a plataforma como limitação em vez de supor equivalência.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e02-08-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E02-08-AC-1` | duas geometrias salvas em disco, uma com a janela principal em 1920×1080 no monitor primário e o painel do EQ destacado em (2400, 120) num segundo monitor de 2560×1440, e a sessão reiniciada num ambiente que só expõe o primário | a aplicação inicia com essas geometrias persistidas e a área visível é enumerada; depois o segundo monitor é reconectado em outra posição de origem e a aplicação é reiniciada de novo | Monitor removido não deixa janela inacessível |
| `TASK-E02-08-AC-2` | a janela principal arrastada pelo título para perto da borda de outra janela e de outra tela, com o snap a 12 DIP habilitado e o ponteiro a 7 DIP do ponto de encaixe | o arraste é repetido duas vezes sobre a mesma trajetória: uma soltando direto e outra com a tecla Alt mantida até o fim do gesto, lendo em cada caso a posição final arredondada da janela | Alt ignora snap |
| `TASK-E02-08-AC-3` | uma faixa tocando em saída exclusiva a 96 kHz com o buffer negociado, os contadores de underrun e o p99.9 da callback coletados fora da thread de áudio, e a janela em execução | o painel do EQ é destacado e reencaixado, a janela é redimensionada de 600×760 para 1920×1080 e de volta, e o conjunto é repetido 30 vezes, lendo ao fim o contador de underrun, o p99.9 da callback e a continuidade da posição de reprodução | resize/dock não trava reprodução. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test -p vynamp-player-shell --test geometry_restore --locked` com geometrias de fixture serializadas em JSON (1920×1080 no primário, painel do EQ em 2400,120 num segundo de 2560×1440) e um enumerador de monitores falso que expõe um, dois e dois em origem deslocada; `cargo test -p vynamp-player-shell --test snap_modifier --locked` para o arraste com e sem Alt; e um teste de caracterização de resize com 30 ciclos lendo contador de underrun e p99.9 por uma fixture de output em vez de dispositivo real. | Esperado: com um único monitor a janela restaurada tem o retângulo inteiro dentro da área visível e o painel do EQ reancorado nela, nunca em coordenada negativa ou fora da tela; com dois monitores na origem original o painel volta para (2400, 120) e com a origem deslocada ele é corrigido para a nova área visível sem sumir; soltar sem Alt arredonda a posição para múltiplo de 12 DIP e soltar com Alt mantém a posição livre do ponteiro; nos 30 ciclos de resize e dock o contador de underrun atribuível ao app fica em 0 e a posição de reprodução não reinicia. Hoje nada disso existe: `apps/player` tem 1 arquivo `.rs` de 3 linhas só com `println!`, `[dependencies]` vazia, nenhum `tests/` e nenhuma dependência de janela; não há enumerador de monitores, nem serialização de geometria, nem contador de underrun a ser lido. Esta máquina expõe uma única tela de 1920×1080, então o caso de dois monitores só é exercido com enumerador falso. |
| Hardware | precisa de um segundo monitor físico para a parte de multi-monitor (aqui há uma única tela de 1920×1080, então o caso de dois monitores é exercido por enumerador falso, o que não prova a plataforma) e de um DAC com saída exclusiva para o AC-3 medir underrun real; nenhum dos dois está disponível, e esta máquina só tem 'Conexant ISST Audio' e 'Áudio Intel(R) para telas' | ausência é limitação registrada, não aprovação |

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
