# TASK-E06-09 — Preparar publicação de temas

> Gerado pelo passo 37 a partir de `tasks/backlog.json`. Os campos de estado vêm do
> backlog e não devem ser editados aqui. A coluna `Then` é o requisito aprovado,
> copiado literal do backlog; `Given` e `When` foram autorados na execução.

| Campo | Valor |
|---|---|
| ID | `TASK-E06-09` |
| Módulo dono | `vynamp-player` |
| Repo | `jimmykedlyn/vynamp-player` |
| REQ coberto | `REQ-E06-09` |
| Critérios da spec | pendente — `specs/player.md` §5 |
| Fase | M4 |
| Prioridade | P1 |
| Estado | planned |
| Depende de | `TASK-E06-08`, `TASK-E09-09` |
| Bloqueia | nenhuma |
| Modelo executor | Opus — áudio/RT, concorrência, segurança ou release |
| Esforço | 8–20 dias úteis |
| Risco | alto |

## Entrega

Metadados assinados, autoria, licença e imagens de preview para registry.

## Boundary

- **Não** implementa o serviço de registry, o admin de publisher, a validação de pacote do lado do servidor nem o endpoint de revogação. Tudo isso é `TASK-E09-09`, que esta TASK consome; a confusão é plausível porque as palavras imutabilidade, assinatura e revogação aparecem nos dois aceites, mas aqui se produz o pacote assinado e se consome o estado publicado, e reimplementar o validador no cliente daria duas cópias para divergir no primeiro caso de borda.
- **Não** define nem altera o schema do manifest, o formato `.vyntheme` ou os erros por JSON Pointer. O schema e o validador são `TASK-E06-01`, e o contrato vigente está em `schemas/theme-manifest.schema.json`, hoje com `additionalProperties` falso e sem campo de assinatura nem de preview; acrescentar esses campos aqui faria o validador medir outro contrato.
- **Não** implementa o staging, o commit atômico por versão nem a restauração do tema anterior. Instalação é `TASK-E06-02`; o AC-3 aqui só prova que esse caminho aceita um arquivo local sem conta e sem rede, e não que ele é reescrito nesta TASK.
- **Não** constrói a galeria local, o histórico de versões nem o safe mode que impede apagar pacote do usuário. É `TASK-E06-08`, dependência declarada desta; a revogação aqui marca estado e oferece rollback, nunca apaga arquivo, porque apagar é do safe mode e não do registry.
- **Não** julga a fidelidade visual das capturas de preview. O relatório de cores, medidas, estados e telas é `TASK-E06-10`; aqui se garante que o PNG foi gerado pela aplicação e viaja no pacote, não que ele está conforme, e é essa separação que impede aprovar acessibilidade por screenshot.
- **Não** entrega o registry e a revogação de plugins, que parecem o mesmo trabalho com outra extensão. É `TASK-E07-09`, dono do `.vynplug`, dos grants e do corte de chamada em curso; esta TASK trata só de tema e dos metadados de autoria, licença e imagem de preview de tema.

## Antes de começar

- [ ] Spec do módulo aprovada e o critério de aceite citado já existe nela
- [ ] Worktree própria criada (`cargo xtask repo worktree task-e06-09-claude-<slug>`)
- [ ] Fixtures/hardware necessários disponíveis ou limitação registrada
- [ ] Nenhuma TASK em execução tocando os mesmos arquivos

## Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| `TASK-E06-09-AC-1` | um `.vyntheme` de `org.vynamp.classic` versão 1.0.0 já publicado e instalado pela galeria de `TASK-E06-08`: manifest com `author`, `license.txt`, os layouts `classic`, `library` e `mini` exigidos por `schemas/theme-manifest.schema.json`, sha256 por asset e assinatura de publisher, mais uma segunda build com o mesmo `id` e a mesma `version` em que só o token `lime` mudou de #B8F35A para #B8F35B, com hashes e assinatura recalculados | a segunda build é empacotada pelo fluxo de publicação do player, que recalcula os sha256, assina e envia o pacote, e em seguida a mesma tela tenta publicar a 1.0.0 por cima da primeira e o manifest publicado é relido | Versão publicada é imutável |
| `TASK-E06-09-AC-2` | o registry com `org.vynamp.classic` 1.0.0 ativo e 0.9.4 como versão anterior, uma lista de revogação assinada revogando 1.0.0 com motivo, data e chave de publisher, e a galeria local com 1.0.0 aplicado como tema ativo e 0.9.4 ainda presente no histórico de versões daquele pacote | o player busca a lista de revogação, exibe nome, autor e versão das duas entradas antes de agir, e o usuário escolhe o rollback para 0.9.4, que é reinstalado com staging, validação completa e commit atômico | revogação e rollback funcionam |
| `TASK-E06-09-AC-3` | um Windows 11 sem conta, sem sessão do registry e com o adaptador de rede desabilitado, e o `.vyntheme` 1.0.0 do mesmo fluxo de publicação copiado à mão de um pendrive para o disco, com `author`, `license.txt` e capturas de preview geradas pela própria aplicação dentro do pacote | o arquivo é arrastado para a janela da galeria e instalado pela tela de temas, com o cliente de registry compilado no binário mas sem sessão e sem rede, e o log de sockets do processo é inspecionado depois | instalação manual continua disponível sem conta ou rede. |

## Verificação

| Item | Comando / fixture | Evidência esperada |
|---|---|---|
| Teste automatizado | `cargo test` no repo `vynamp-player` | verde no CI Gate |
| Oráculo independente | `cargo test --test theme_publish` no repo `vynamp-player`, com o serviço de `TASK-E09-09` em processo, um par Ed25519 efêmero em `tempdir` e três fixtures em `apps/player/tests/fixtures/themes/`: a 1.0.0, a republicação da 1.0.0 com um token trocado e o arquivo copiado para um diretório com o adaptador de rede desabilitado. Hoje não há nada disso para rodar: o repo do player tem um único `.rs`, `apps/player/src/main.rs` com 3 linhas e `println!` de `vynamp_player 0.1.0`, `[dependencies]` vazio no `Cargo.toml` e nenhum diretório `tests/`; e o único schema de tema existente, `schemas/theme-manifest.schema.json`, não tem campo de assinatura nem de preview, então nem o metadado assinado nem a captura têm onde ser declarados ainda | Esperado: a republicação da 1.0.0 cria 0 versões novas e devolve motivo estável de imutabilidade, com o sha256 do zip publicado igual byte a byte antes e depois; a lista de revogação aplica exatamente 1 entrada e o pacote revogado continua no disco, com o tema ativo caindo para 0.9.4 e o relatório declarando as duas versões por nome e autor; o rollback termina com a versão ativa reportada como 0.9.4 e 0 diretórios de staging sobrando; a instalação manual conclui com 1 versão instalada, 0 requisições ao registry e 0 sockets abertos com o adaptador desabilitado; e o preview dentro do pacote é um PNG gerado pela aplicação com 600x760 para o modo Classic, coerente com a viewBox de `assets/classic-layout.svg`. |
| Hardware | nenhum: as três fixtures são arquivos em disco e o fluxo roda em processo local, sem áudio, sem aparelho móvel e sem HSM; a chave de publicação é um par efêmero em `tempdir`, e a ausência de um HSM ou KMS real fica registrada como limitação da prova de assinatura, não como aprovação | ausência é limitação registrada, não aprovação |

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
