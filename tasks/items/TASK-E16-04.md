# TASK-E16-04 — Entregar MIME, extensões de shell e abertura externa no Linux

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E16-04` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E16-04` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E16-02`, `TASK-E16-09`, `TASK-E14-02` |
| Bloqueia | `TASK-E16-07`, `TASK-E18-01` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Desktop Entry com Desktop Actions, extensões/ServiceMenus (Dolphin, Nautilus, Thunar), MIME hicolor e MPRIS.

## Boundary

- **Não** implementa a interface MPRIS D-Bus nem o controle de playback por org.mpris.MediaPlayer2. O dono é TASK-E12-01, e a confusão é plausível porque a Entrega desta TASK cita MPRIS na mesma frase dos arquivos .desktop: aqui MPRIS é só consumido, o nome de barramento é responsabilidade do alvo Linux nativo.
- **Não** gera o XML de MIME nem as declarações de tipo a partir do build verificado. O dono é TASK-E16-09; esta TASK instala vynamp.xml e os ícones, não deriva a lista de tipos das capabilities.
- **Não** implementa o socket IPC nem a eleição de instância ativa para onde a extensão Nautilus repassa a lista. O dono é TASK-E16-02, e quem executar pode achar que injetar o menu inclui o transporte dos caminhos.
- **Não** valida AppImage, DEB, dependências de runtime nem empacotamento Linux. O dono é TASK-E15-07; esta TASK produz artefatos de integração shell e os instala em ~/.local, o pacote fica com a validação de packaging.
- **Não** escreve as metades Windows ou macOS do menu de contexto. Os donos são TASK-E16-03 e TASK-E16-05; a tentação é unificar os sete verbos numa camada só, mas cada SO tem formato de registro próprio.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e16-04-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E16-04-AC-1` | KDE Plasma 5.27 com Dolphin 23.08, GNOME 45 com Nautilus 45 e XFCE 4.18 com Thunar 4.18, todos com vynamp.desktop em ~/.local/share/applications/, vynamp-servicemenu.desktop em ~/.local/share/kio/servicemenus/, a extensão Nautilus instalada e uca.xml em ~/.config/Thunar/uca.xml já gerado | o usuário seleciona 3 arquivos .flac e .mp3 e abre o menu de contexto do botão direito em cada um dos três gerenciadores | Gerenciadores (Dolphin, Nautilus, Thunar) exibem submenu Vynamp com verbos canônicos |
| `TASK-E16-04-AC-2` | vynamp.desktop com Exec=vynamp --play -- %U e um lote de 4 arquivos cujos nomes contêm espaço, apóstrofo e cifrão, como /mnt/mus/01 Don't $HOME.flac, todos com URI file:// válido | o usuário abre os 4 arquivos por Abrir com Vynamp a partir do Nautilus, disparando a expansão de %U pelo GLib antes do exec | %U repassa lote sem shell |
| `TASK-E16-04-AC-3` | árvore de instalação em ~/.local com os SVGs canônicos das três famílias e as renderizações PNG de 16, 32, 48, 64, 128 e 256 px para os 17 tipos MIME, e a família document ativa em Preferências | o usuário aplica a família signal nas Preferências e o instalador roda update-desktop-database, update-mime-database e gtk-update-icon-cache sobre a árvore de ~/.local | ícones hicolor por família instalados. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | desktop-file-validate em vynamp.desktop, vynamp-servicemenu.desktop e nos Desktop Actions, xmllint --noout em vynamp.xml e uca.xml, mais um helper em Rust que expande Exec=vynamp --play -- %U com 4 URIs contendo espaço e cifrão e imprime argv | Hoje vynamp-player tem só src/main.rs com println!("vynamp_player 0.1.0"), zero dependências e nenhum tests/, e não existe um único .desktop no repo. A evidência exigida é desktop-file-validate sem aviso, 5 Desktop Actions declaradas (NewPlaylist, AddPlaylist, Enqueue, Convert, Restore), 17 entradas mime-type em vynamp.xml, 306 PNGs (17 tipos x 6 tamanhos x 3 famílias) em hicolor, e argv final com 5 elementos, o binário mais 4 caminhos intactos, sem /bin/sh na linha |
| Hardware | ausente para o AC-1: Dolphin 23.08, Nautilus 45 e Thunar 4.18 exigem sessão Linux com X11 ou Wayland, e este ambiente é Windows 11 x86_64 sem nenhum dos três; validação local fica restrita aos arquivos .desktop, ao XML de MIME e ao argv, e a exibição do submenu precisa de um alvo Linux | ausência é limitação registrada, não aprovação |

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
