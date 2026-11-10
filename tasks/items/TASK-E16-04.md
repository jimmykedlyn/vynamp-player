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

- **Não** implementa a interface MPRIS D-Bus nem o controle de playback por org.mpris.MediaPlayer2. O dono é TASK-E12-01; a Entrega desta TASK cita MPRIS na mesma frase dos .desktop, mas aqui o nome de barramento é apenas consumido do alvo Linux nativo.
- **Não** gera o XML de MIME nem as declarações de tipo a partir do build verificado. O dono é TASK-E16-09; esta TASK instala vynamp.xml e os ícones, não deriva a lista de tipos das capabilities.
- **Não** implementa o socket IPC nem a eleição da instância ativa que recebe a lista repassada pela extensão Nautilus. O dono é TASK-E16-02; injetar o menu não inclui transportar os caminhos.
- **Não** valida AppImage, DEB, dependências de runtime nem empacotamento Linux. O dono é TASK-E15-07; aqui se produzem artefatos de integração shell instalados em ~/.local, e o pacote fica com a validação de packaging.
- **Não** escreve as metades Windows ou macOS do menu de contexto. Os donos são TASK-E16-03 e TASK-E16-05; a tentação é unificar os sete verbos numa camada só, mas cada SO tem formato de registro próprio.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e16-04-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E16-04-AC-1` | KDE Plasma 5.27 com Dolphin 23.08, GNOME 45 com Nautilus 45 e XFCE 4.18 com Thunar 4.18, todos com vynamp.desktop em ~/.local/share/applications, vynamp-servicemenu.desktop em ~/.local/share/kio/servicemenus, a extensão Nautilus instalada e uca.xml em ~/.config/Thunar/uca.xml gerado, com update-desktop-database e update-mime-database já executados | o usuário seleciona 3 arquivos .flac e .mp3 e abre o menu de contexto do botão direito em cada um dos três gerenciadores | Gerenciadores (Dolphin, Nautilus, Thunar) exibem submenu Vynamp com verbos canônicos |
| `TASK-E16-04-AC-2` | vynamp.desktop com Exec=vynamp --play -- %U e um lote de 4 arquivos cujos nomes contêm espaço, apóstrofo e cifrão, como /mnt/mus/01 Don't $HOME.flac, todos com URI file:// válido e nenhum com aspas no nome | o usuário abre os 4 arquivos por Abrir com Vynamp a partir do Nautilus, disparando a expansão de %U pelo GLib antes do exec | %U repassa lote sem shell |
| `TASK-E16-04-AC-3` | árvore de instalação em ~/.local com os SVGs canônicos das três famílias e as renderizações PNG de 16, 32, 48, 64, 128 e 256 px para os 17 tipos MIME, com a família document ativa em Preferências e o icon cache do hicolor ainda apontando para ela | o usuário aplica a família signal nas Preferências e o instalador roda update-desktop-database, update-mime-database e gtk-update-icon-cache sobre ~/.local/share/icons/hicolor | ícones hicolor por família instalados. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | WSL Ubuntu 22.04.5 já presente nesta máquina, com update-mime-database e gtk-update-icon-cache instalados e python3 disponível, roda xmllint via python3 xml.etree e a checagem de chaves do .desktop; desktop-file-validate e desktop-file-utils ainda não estão instalados no WSL, e o helper Rust novo em apps/player/tests/exec_percent_u.rs não existe: o repo vynamp-player tem só src/main.rs, zero dependências e nenhum tests/. O único .desktop do umbrella é packaging/linux/vynamp.desktop, com Exec=vynamp %U, sem --play, sem -- e sem nenhuma Desktop Action, então a fixture o reescreve antes de medir. | A evidência exigida é 0 avisos do desktop-file-validate em vynamp.desktop e em vynamp-servicemenu.desktop, 5 Desktop Actions declaradas (NewPlaylist, AddPlaylist, Enqueue, Convert, Restore), nenhum Exec passando por /bin/sh -c, xmllint limpo em vynamp.xml com 17 entradas mime-type, 306 PNGs (17 tipos x 6 tamanhos x 3 famílias) mais os SVGs escaláveis em hicolor — hoje assets/icons/families tem 4 SVGs por família, só audio, flac, mp3 e wav — e argv final com 5 elementos, o binário, --play, -- e os 4 caminhos com Don't e $HOME intactos. Nada disso existe no repo, e desktop-file-validate prova formato de arquivo, não que os três gerenciadores exibam o submenu. |
| Hardware | ausente para o AC-1: Dolphin 23.08, Nautilus 45 e Thunar 4.18 exigem sessão Linux com X11 ou Wayland; este host é Windows 11 x86_64 e o WSL Ubuntu 22.04.5 disponível não tem sessão gráfica, não tem os três gerenciadores e nem sequer tem desktop-file-utils ou rustc instalados, então serve só para validar XML e chaves do .desktop, e a exibição do submenu Vynamp fica sem medição | ausência é limitação registrada, não aprovação |

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
