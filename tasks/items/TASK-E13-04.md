# TASK-E13-04 — Entregar MediaStore, SAF e permissões

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E13-04` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E13-04` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M2 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E13-02`, `TASK-E03-03` |
| Bloqueia | `TASK-E13-06`, `TASK-E16-06` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 5–12 dias úteis |
| Risco | alto |

## Entrega

Adapters de document URI/FD, grants persistidos, indexação e capabilities de escrita de tags.

## Boundary

- **Não** indexa e faz curadoria do catálogo bibliográfico local. A importação incremental e os roots offline são TASK-E03-03; aqui é só o adapter que lê MediaStore e SAF e entrega o que ela indexa, e trazer a varredura de roots para cá criaria dois donos do mesmo índice.
- **Não** monta a tela de browse nem a navegação acessível do telefone. A interface phone é TASK-E13-05; esta TASK entrega os adapters e as capabilities, e a confusão é plausível porque a permissão negada só é observável na UI.
- **Não** abre a Activity de viewer nem registra o share de arquivos. VIEW e Share seguros no Android são TASK-E16-06; aqui o content:// é apenas fonte de reprodução, e um grant de leitura não autoriza exportar o arquivo.
- **Não** implementa o serviço de playback nem o MediaSession que consomem esses document URIs. O serviço e o MediaSession com engine Rust são TASK-E13-02; esta TASK só declara como o item é aberto, e misturar os dois faria o ciclo de vida do handle depender da Activity.
- **Não** escreve o streaming ICY nem o download de episódios que também produzem itens de biblioteca. Rádio Icecast é TASK-E05-03 e feeds de podcast são TASK-E05-05; ambos entregam o SourceDescriptor de rede, e android-content é exclusivo de grants locais.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e13-04-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E13-04-AC-1` | uma biblioteca indexada por MediaStore com 1 200 faixas e 8 estações Icecast salvas, em aparelho com target SDK 34 onde READ_MEDIA_AUDIO foi concedida e depois revogada em Settings, mais um árvore SAF de 40 arquivos com grant persistida revogada pelo provider aprovada por FLAG_GRANT_PERSISTABLE_URI_PERMISSION | o adapter de biblioteca reabre o índice e a tela de rádio é aberta depois da revogação, sem nova concessão de permissão | Deny/revoke preserva biblioteca e rádio |
| `TASK-E13-04-AC-2` | manifesto do APK debug em target SDK 34 declarando apenas READ_MEDIA_AUDIO, READ_MEDIA_VISUAL_USER_SELECTED e grants SAF por document picker, sem MANAGE_EXTERNAL_STORAGE, instalado em aparelho com 300 arquivos de áudio em shared storage e READ_MEDIA_AUDIO negada | aapt2 dump badging e adb shell appops get do pacote são lidos e as telas de importação MediaStore, de pasta SAF e de rádio são percorridas nesse aparelho | não exigir acesso irrestrito a arquivos |
| `TASK-E13-04-AC-3` | um item de origem content://com.android.providers.media.documents/document/audio%3A4711 e um item content://com.android.externalstorage.documents/tree/primary%3AMusic/document/primary%3AMusic%2Fa.flac, ambos com grant lida por ContentResolver e fd obtido por openFileDescriptor | o SourceDescriptor android-content é resolvido e o item é reproduzido até o fim, com o caminho reportado em log de diagnóstico e no arquivo de estado do item | Content URI não vira path fictício |
| `TASK-E13-04-AC-4` | um FLAC com tags de 2,1 KiB em content:// de provider que somente expõe openDocument com modo r e w, sem replace atômico, e uma cópia prévia do arquivo guardada em cache privado | o usuário pede rename e escrita de tags em lote de 3 arquivos pelo fluxo de edição e o provider recusa a escrita no segundo arquivo, informando ErrnoException EACCES | backup/limites de escrita explícitos. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | Teste de integração Rust novo em apps/player/tests/android_content.rs (o repo hoje tem só src/main.rs com um println!("vynamp_player 0.1.0"), zero dependências e nenhum tests/, então esse arquivo ainda não existe): fixture de manifesto com os atributos de permissão lidos de um arquivo de texto, mais vetores de entrada para resolver 2 URIs content:// e 40 references %3A decodificadas, e um vetor de revogação que marca 8 estações e 1 200 faixas. | A saída tem que mostrar 0 ocorrências de MANAGE_EXTERNAL_STORAGE no manifesto serializado, 0 de EXTERNAL_STORAGE, e as 2 URIs resolvidas com display name e MIME type reais e sem nenhum /storage/armazenado no struct de resolução; após o vetor de revogação, 1 200 faixas e 8 estações ainda presentes com available=false e 0 linhas removidas. Hoje nenhum desses números pode ser produzido: não existe adapter, manifesto Android, testes nem dependências além de std. |
| Hardware | ausente: exige aparelho Android com target SDK 34 e providers MediaStore/SAF reais para revogar grant e ver openFileDescriptor; este ambiente é Windows 11 x86_64 sem aparelho Android, então só o vetor de dados offline é executável. | ausência é limitação registrada, não aprovação |

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
