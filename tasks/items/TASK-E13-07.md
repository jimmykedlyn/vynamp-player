# TASK-E13-07 — Entregar recovery de processo e controle de energia

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E13-07` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E13-07` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E13-03`, `TASK-E03-02` |
| Bloqueia | `TASK-E13-09`, `TASK-E14-04` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | alto |

## Entrega

Checkpoint, reconexão ao serviço, WorkManager, quotas de cache e wake locks limitados.

## Boundary

- **Não** implementa o serviço em foreground, a notificação de mídia nem o MediaSession que a rotação precisa preservar. O serviço e o MediaSession com engine Rust são TASK-E13-02; aqui se reconecta a uma sessão que aquele serviço mantém, e criar um segundo owner do process lifecycle quebraria a sessão única.
- **Não** define a fila, a posição ou o shuffle que o checkpoint serializa. Fila independente e shuffle são TASK-E03-02; esta TASK só persiste e restaura o estado que ela expõe, e reimplementar a fila aqui faria o restore divergir do modelo de queue.
- **Não** entrega o download em si nem a política de metered e quota do provider. Streaming, podcasts e downloads Android são TASK-E13-06; aqui o WorkManager aparece apenas como consumidor do lifecycle, e a quota de cache é limite desta TASK, não transporte.
- **Não** cobre reprodução por dias sob qualquer fabricante nem autostart após force-stop. `platforms/02-android.md` diz que swiping a Activity não deve matar música ativa e que, após force-stop, não se promete continuidade nem autostart; quem mede o comportamento por fabricante e por versão de OS em aparelho real é `TASK-E13-10`. Aqui o checkpoint sobrevive ao processo morto pelo próprio sistema e o usuário retoma; prometer sobrevivência sob force-stop viraria uma garantia que nenhum AC desta TASK mede.
- **Não** implementa o sleep timer nem alarme/wake por política de OS. Sleep timer, scheduler e o fechamento de playback por tempo são `TASK-E08-05`; esta TASK entrega o checkpoint e as quotas de trabalho em segundo plano, e duplicar a política de tempo aqui daria dois donos ao mesmo despertador.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e13-07-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E13-07-AC-1` | sessão Rust tocando uma faixa em 44,1 kHz estéreo com posição 97 300 ms, fila de 42 itens e ganho 0,8, em aparelho Android 14 com a opção de desenvolvedor Don't keep activities ativada, e nenhum checkpoint persistido ainda existe no espaço privado do app | o processo do app é morto com adb shell am kill e depois a Activity é reaberta pelo usuário a partir do launcher, sem serviço declarado com START_STICKY e sem receiver de BOOT_COMPLETED | Process death permite restore sem autostart proibido |
| `TASK-E13-07-AC-2` | sessão em reprodução da faixa 3 de 42, posição 61 500 ms, engine Rust com handle nativo aberto e a Activity em portrait com o MediaSession ativo | a tela é girada para landscape e de volta para portrait três vezes, com a configuração android:configChanges conferida no manifesto | rotation mantém sessão |
| `TASK-E13-07-AC-3` | uma playlist de 2 h de FLAC 44,1 kHz alimentando o output AAudio do serviço, aparelho carregando com tela apagada, wake lock de áudio parcial adquirido e visualizer ativo antes do screen-off | o teste de 2 h roda com o app em segundo plano e a tela apagada, medindo underruns por contador do callback de áudio do AAudio | screen-off 2 h sem underrun causado pelo app |
| `TASK-E13-07-AC-4` | sessão tocando com 1 wake lock parcial e o visualizer consumindo o tap do buffer a 60 Hz, mais um download de 120 MiB agendado em WorkManager | o usuário pausa a reprodução e o app vai para background pela tecla Home, e depois o serviço é encerrado por stopSelf | wake locks liberados e visualizer desativado no background. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | Harness Rust novo em apps/player/tests/android_recovery.rs (inexistente: o repo tem só src/main.rs com println!("vynamp_player 0.1.0"), zero dependências e nenhum tests/): escreve um checkpoint em diretório temporário com posição 97 300 ms, fila de 42 itens e ganho 0,8, encerra e reinicia o processo do harness e restaura o estado; o vetor de rotação é uma sequência de 3 eventos onSaveInstanceState/onRestoreInstanceState com contagem de criação de engine; o teste de 2 h fica em roteiro manual separado, com contador de underrun. | A evidência esperada é o checkpoint relido com posição 97 300 ms, 42 itens na fila e ganho 0,8 iguais ao serializado, e 0 recriações de engine e 0 perdas de posição nas 3 rotações; no roteiro de 2 h, 0 underrun atribuído ao app e 0 wake locks retidos em dumpsys power após a pausa, com 1 wake lock adquirido durante o playback e 0 com o visualizer ativo em background. Nada disso está medível hoje: não há checkpoint, teste, nem dependência além de std. |
| Hardware | ausente: exige aparelho Android real para tela apagada por 2 h, contador de underrun do AAudio e dumpsys power de wake locks, além de Android 14 para Don't keep activities; este ambiente é Windows 11 x86_64 sem aparelho Android, sem adb e sem AVD. | ausência é limitação registrada, não aprovação |

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
