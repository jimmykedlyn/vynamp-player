# TASK-E17-06 — Certificar tags técnicas, capítulos e sidecars

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E17-06` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E17-06` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M3 |
| Prioridade | P0 |
| Estado | planned |
| Depende de | `TASK-E03-08` |
| Bloqueia | `TASK-E17-20` |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 2–5 dias úteis |
| Risco | alto |

## Entrega

Matriz read/write de ID3/MP4/Matroska/CAF/AIFF/BWF e LRC/WebVTT.

## Boundary

- **Não** implementa a escrita de tags em lote na biblioteca (varredura, transação, undo, barra de progresso e rollback por item). Essa é TASK-E03-08, que já é a dependência desta TASK; aqui só se certifica que o formato sobrevive ao roundtrip, e misturar a transação de lote com a matriz read/write daria dois donos ao mesmo ponto de commit.
- **Não** implementa a leitura de tags e capas do import (modelo de exibição, extração de capa, dedupe por hash). É TASK-E03-04; 'tags técnicas' parece o mesmo assunto, mas lá a pergunta é o que a biblioteca guarda, e aqui é o que cada container aceita ler e gravar sem perder campo desconhecido.
- **Não** implementa a política de proxy e rede isolada do app. É TASK-E17-11; o critério de AC-3 fala em não baixar URL, o que sugere que o bloqueio de rede nasce aqui, mas esta TASK só prova que o parser de sidecar não amplia o grant nem emite requisição, usando a política que a outra TASK define.
- **Não** entrega letras, bookmarks e histórico do usuário (sincronização de letra por linha, estado de bookmark, timeline de reprodução). É TASK-E03-09; LRC aparece tanto aqui como lá, e a execução tentadora é começar a renderizar a letra sincronizada em vez de só certificar o parser do sidecar.
- **Não** entrega import/export de playlists nem arquivos CUE. É TASK-E03-06; a spec cita cue junto de chapters na mesma frase, mas o item certificado aqui é capítulo e rating do próprio arquivo, não a playlist posicional que o CUE descreve.
- **Não** fecha o gate combinado de codecs, DSP e visualizers. É TASK-E17-20, que esta TASK bloqueia; a matriz gerada aqui é insumo aprovado para esse gate, e promovê-la a gate faria a assinatura de release acontecer duas vezes.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e17-06-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E17-06-AC-1` | oito fixtures de tag com campos que o Vynamp não modela, cada uma com SHA-256 registrado em manifest e com o payload de áudio separado: MP3 com ID3v2.4 e frame desconhecido XYZW de 12 bytes, MP3 com ID3v2.3 e TXXX de proprietário, M4A com átomo ---- freeform de 31 bytes, MKA com SimpleTag em Target 50 e elemento Unknown, CAF com chunk info não mapeado, AIFF com bloco ID3 aninhado, BWF com iXML contendo campo não reconhecido e WAV com LIST/INFO de chave arbitrária | o writer recebe os metadados lidos sem alteração, grava a mesma tupla de volta em uma cópia de cada fixture e o Reader relê o arquivo gravado | Roundtrip preserva desconhecidos |
| `TASK-E17-06-AC-2` | um WavPack híbrido com sidecar .wvc de mesmo basename, 7 capítulos CHAP com títulos distintos e POPM de rating 196, ao lado de um MKA com 3 capítulos e tag RATING=80, todos com SHA-256 por arquivo registrado antes do rename | a ação Rename da biblioteca troca o basename dos dois itens, incluindo o sidecar .wvc, e o scan incremental relê os dois arquivos renomeados | capítulos/ratings e WVC não somem no rename |
| `TASK-E17-06-AC-3` | um LRC cujo corpo contém uma diretiva apontando para http://127.0.0.1:9/lrc e um caminho ../../../../etc/passwd, e um WebVTT com uma linha NOTE referenciando //exemplo.invalid/cue, com grant concedido apenas para a pasta D:/fixtures/sidecars e nenhum grant de rede | o parser de sidecar lê os dois arquivos sob o grant e o serviço de metadados tenta resolver cada referência encontrada no conteúdo | grant de sidecar não amplia acesso nem baixa URLs. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | fixture em tests/fixtures/tags/ (diretório a criar) com os oito arquivos de AC-1, os dois de AC-2 e os dois sidecars hostis de AC-3, mais manifest de SHA-256 por arquivo, executada por cargo test --test tag_roundtrip e por um oráculo independente que compara bytes de campo desconhecido e SHA-256 do payload de áudio antes e depois do write-back | esperado: os 8 fixtures voltam com os mesmos bytes de campo desconhecido byte a byte, o SHA-256 do payload de áudio idêntico ao do manifest, os 7 capítulos CHAP e o POPM 196 legíveis depois do rename com o .wvc renomeado junto e o hash do sidecar ainda válido, e zero requisição de rede e zero open fora de D:/fixtures/sidecars no log do parser. Nada disso roda hoje: o repo vynamp-player tem 1 arquivo .rs (src/main.rs) que só executa println!("vynamp_player 0.1.0"), zero dependências e nenhum diretório tests/, então o comando acima falha com 'no test target named tag_roundtrip' até a matriz existir |
| Hardware | nenhum: todo o corpus é arquivo em disco (ID3v2.3/v2.4, MP4, Matroska, CAF, AIFF, BWF, LRC, WebVTT) e nenhum critério abre dispositivo de áudio, rota Bluetooth, aparelho Android ou iOS | ausência é limitação registrada, não aprovação |

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
