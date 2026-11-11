# TASK-E13-08 — Entregar temas e plugins mobile

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E13-08` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E13-08` |
| Critérios da spec | `specs/player.md` §5 (rascunho, não aprovado) |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E13-05`, `TASK-E06-04`, `TASK-E07-08` |
| Bloqueia | `TASK-E13-09` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–15 dias úteis |
| Risco | alto |

## Entrega

Perfis phone/tablet, manager mobile e runtime de extensões validado para Android.

## Boundary

- **Não** produz nem redesenha os três temas originais do produto. Produzir três temas originais é TASK-E06-04; aqui se entrega o perfil phone/tablet e o fallback builtin que consome esses temas, e desenhar tema novo aqui faria o pack compartilhado ter dois autores.
- **Não** entrega o SDK de plugin nem o harness de documentação para extensões. Documentação e harness do SDK são TASK-E07-08; esta TASK valida o runtime no Android e documenta compatibilidade de distribuição, e o harness de exemplo pertence ao SDK.
- **Não** implementa a navegação acessível nem a interface phone que o fallback precisa cobrir. A interface phone e a navegação acessível são TASK-E13-05; o fallback daqui é de tema e de escala, e confundir os dois coloca duas fontes de verdade de acessibilidade.
- **Não** faz o layout de tablet, foldable e a distribuição Android do pacote final. Tablet/foldable e distribuição Android são TASK-E13-09; aqui o perfil chega até phone e tablet em declaração de tema, e o empacotamento de loja é daquela TASK.
- **Não** implementa listagem, browse ou controle do MediaSession por Android Auto. `platforms/02-android.md` trata Android Auto browsing e controls como expansão de integração prevista depois da base de MediaLibraryService, e essa base é `TASK-E13-02`; antecipá-la aqui prometeria uma integração que a spec colocou fora do 1.0.
- **Não** define nem valida o schema do manifest do plugin ou do tema, nem o broker de permissões genérico. Schema e validador de temas são TASK-E06-01 e o broker de permissões é TASK-E07-03; aqui só se mostra que o runtime mobile respeita o budget e as capabilities que aquele broker entrega, e revalidar o schema aqui faria duas autoridades sobre o mesmo contrato.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e13-08-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E13-08-AC-1` | aparelho Android com densidade 420 dpi e largura de 411 dp, e um pacote .vyntheme cujo manifest declara em assets o caminho assets/fonts/silkscreen.ttf com sha256, mas o ZIP não contém esse arquivo, mais as preferências de acessibilidade com tamanho de fonte em 200% e tema de alto contraste do sistema | o app inicia com esse tema selecionado e o tema é trocado para o fallback builtin pela tela de temas, com o TalkBack ativo | Fallback acessível |
| `TASK-E13-08-AC-2` | um plugin WASM de metadata no perfil Android1.0 com budget declarado de 16 MiB de memória linear e 8 ms de CPU por chamada, e capabilities plugin.metadata.read sem permissão de rede e sem acesso a content:// | o host mobile instancia o módulo e o plugin tenta alocar 32 MiB, chamar o provider de metadata por 200 ms e abrir uma URL HTTPS | budgets e permissões funcionam |
| `TASK-E13-08-AC-3` | APK em arm64-v8a gerado pelo Gradle com o diretório plugins/ contendo 2 arquivos .wasm, nenhuma biblioteca .so de terceiro e 1 plugin que declara dependência de helper nativo externo | o APK é instalado em aparelho arm64, a tela de plugins mobile é aberta e o runtime tenta resolver a import env dlopen do plugin, e o conteúdo nativo é listado com unzip -l | nenhum helper nativo externo |
| `TASK-E13-08-AC-4` | matriz de suporte do runtime WASM com as linhas de AOT, interpreter e JIT para arm64-v8a, e o documento de distribuição com budgets, licença e regras de loja para o perfil Android1.0 | a matriz e o documento são publicados na spec platforms/02-android.md e a decision record M0 é citada, e o manager mobile lista as diferenças de capability por dispositivo | runtime e distribuição compatíveis documentados, sem promessa de JIT não provado. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | Teste Rust novo em apps/player/tests/android_plugins.rs (inexistente: o repo tem só src/main.rs com println!("vynamp_player 0.1.0"), zero dependências e nenhum tests/): roda um .wasm de fixture de 4 KiB que declara budget de 16 MiB e 8 ms e tenta alocar 32 MiB, mais um .wasm com import env dlopen, mais uma varredura do manifest do tema cujo layout phone aponta para um asset de fonte ausente do pacote; a matriz de runtime e o documento de distribuição entram como arquivos versionados e verificáveis. | A saída tem que mostrar 1 trap de out-of-memory para a alocação de 32 MiB com a memória máxima do módulo em 16 MiB, 1 recusa de resolução do import env dlopen com 0 símbolos nativos carregados, 0 arquivos .so de terceiro listados pelo unzip -l do APK, e a resolução do tema inválido caindo no builtin com contraste e escala de fonte declarados; a matriz tem que ter as 3 linhas AOT, interpreter e JIT e a mesma decisão M0 citada no documento. Hoje nada disso existe: sem teste, sem runtime WASM, sem fixtures e sem Gradle em apps/player; e o schema vigente em schemas/theme-layout.schema.json não tem campo de fonte, então o fixture falha pelo asset declarado no manifest e ausente do ZIP. |
| Hardware | ausente: exige aparelho Android arm64 com arm64-v8a para instalar o APK, TalkBack e o tema de alto contraste do sistema para o critério de fallback acessível; este ambiente é Windows 11 x86_64 sem aparelho Android e sem NDK/adb, então só o fixture .wasm e a varredura de manifest são exercitáveis aqui. | ausência é limitação registrada, não aprovação |

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
