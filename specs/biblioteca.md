# Spec — vynamp-player · biblioteca

| Campo | Valor |
|---|---|
| Módulo | `vynamp-player` |
| Crate(s) | `vynamp_player` (módulo `biblioteca`; crate único até uma TASK justificar divisão) |
| Tipo | app |
| Versão da spec | 0.2.0 |
| Data | 2026-11-11 |
| Estado | **rascunho autorado, não aprovado** — §3 a §6, §9 e §10 escritos a partir de `library/01-biblioteca-playlists.md`, `library/02-tags-letras.md` e das TASKs E03, E08-04, E08-09 e E17-06. A aprovação é do mantenedor, por `GOVERNANCE.md` |
| Dono | mantenedor do `vynamp-player` |
| Depende de | `vynamp-engine` (unico crate do projeto) |
| É dependido por | ninguem |

> §1, §2 e §7 foram decididos em `01-arvore-final.md` §2 e valem como norma.
> §3 a §6, §9 e §10 foram autorados em 2026-11-11 a partir de `library/01-biblioteca-playlists.md`, `library/02-tags-letras.md` e das TASKs E03, E08-04, E08-09 e E17-06,
> que continuam sendo a fonte normativa. Estar escrito não é estar aprovado: até o
> mantenedor aprovar, nenhum número aqui é contrato, e toda TASK que dependa dele
> continua `planned`. A coluna `Then` de §5 é o critério aprovado no backlog, copiado
> literal; `Given` e `When` são o enquadramento desta spec.

## 1. Propósito

App desktop e Android: UI, biblioteca, streaming, temas, plugins e integracao com o sistema operacional.

**Não faz:** Nao converte em lote nem restaura audio — delega aos apps proprios.

## 2. Fronteira

| Permitido | Proibido |
|---|---|
| herda a fronteira de `specs/player.md` | herda as proibicoes de `specs/player.md` |

## 3. API pública (esboço)

```rust
pub mod biblioteca {
    pub struct Library { /* índice local, SQLite ou equivalente decidido na TASK */ }
    pub struct TrackId(pub u64);
    pub struct Track { pub id: TrackId, pub path: PathBuf, pub tags: Tags, pub duration_ms: Option<u64> }
    pub struct Tags { pub title: Option<String>, pub artist: Option<String>, pub album: Option<String>, /* … */ }

    impl Library {
        pub fn open(db: &Path) -> Result<Self, LibraryError>;
        /// Varredura incremental; não bloqueia a UI e respeita `cancel`.
        pub fn scan(&mut self, roots: &[PathBuf], cancel: &CancelToken) -> Result<ScanReport, LibraryError>;
        pub fn query(&self, q: &Query) -> Result<Vec<Track>, LibraryError>;
    }

    pub struct Playlist { pub name: String, pub items: Vec<PlaylistItem> }
    impl Playlist {
        /// M3U/M3U8/PLS; caminhos relativos resolvidos contra o arquivo.
        pub fn load(path: &Path) -> Result<Self, LibraryError>;
        pub fn save(&self, path: &Path, fmt: PlaylistFormat) -> Result<(), LibraryError>;
    }

    /// Edição de tags nunca reescreve o áudio; grava em temporário e troca atômica.
    pub fn write_tags(path: &Path, tags: &Tags) -> Result<(), LibraryError>;
}
```

## 4. Requisitos

| ID | Requisito | REQ de produto | TASK | Teste |
|---|---|---|---|---|
| BIB-REQ-01 | Implementar schema SQLite e migrations seguras — planned (M1) | REQ-E03-01 | TASK-E03-01 | — (BIB-AC-01 planejado) |
| BIB-REQ-02 | Implementar queue independente e shuffle — planned (M1) | REQ-E03-02 | TASK-E03-02 | — (BIB-AC-02 planejado) |
| BIB-REQ-03 | Implementar import incremental e roots offline — planned (M2) | REQ-E03-03 | TASK-E03-03 | — (BIB-AC-03 planejado) |
| BIB-REQ-04 | Implementar leitura de tags e capas segura — planned (M2) | REQ-E03-04 | TASK-E03-04 | — (BIB-AC-04 planejado) |
| BIB-REQ-05 | Entregar FTS, filtros e views de biblioteca — planned (M2) | REQ-E03-05 | TASK-E03-05 | — (BIB-AC-05 planejado) |
| BIB-REQ-06 | Entregar playlists import/export e CUE — planned (M2) | REQ-E03-06 | TASK-E03-06 | — (BIB-AC-06 planejado) |
| BIB-REQ-07 | Implementar smart playlists e templates — planned (M3) | REQ-E03-07 | TASK-E03-07 | — (BIB-AC-07 planejado) |
| BIB-REQ-08 | Implementar escrita de tags e capítulos em lote — planned (M3) | REQ-E03-08 | TASK-E03-08 | — (BIB-AC-08 planejado) |
| BIB-REQ-09 | Entregar letras, bookmarks e histórico — planned (M3) | REQ-E03-09 | TASK-E03-09 | — (BIB-AC-09 planejado) |
| BIB-REQ-10 | Completar recovery, export e large-library gate — planned (M3) | REQ-E03-10 | TASK-E03-10 | — (BIB-AC-10 planejado) |
| BIB-REQ-11 | Entregar organização de arquivos e dedupe — planned (M3) | REQ-E08-04 | TASK-E08-04 | — (BIB-AC-11 planejado) |
| BIB-REQ-12 | Entregar audiobooks e progresso offline — planned (M4) | REQ-E08-09 | TASK-E08-09 | — (BIB-AC-12 planejado) |
| BIB-REQ-13 | Certificar tags técnicas, capítulos e sidecars — planned (M3) | REQ-E17-06 | TASK-E17-06 | — (BIB-AC-13 planejado) |

Fonte de cada requisito: o documento citado na TASK (`library/01-biblioteca-playlists.md`, `library/02-tags-letras.md`, `reference/audio/06-codec-capability-registry.md`). Fases presentes: M1, M2, M3, M4.

## 5. Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| BIB-AC-01 | a entrega de TASK-E03-01 (Implementar schema SQLite e migrations seguras) | o teste de aceite da TASK roda | Kill no migrate/save preserva DB ou backup; FTS reconstruível; schema unknown não é apagado. |
| BIB-AC-02 | a entrega de TASK-E03-02 (Implementar queue independente e shuffle) | o teste de aceite da TASK roda | Reorder playlist não muda queue; shuffle sem reposição, Previous usa histórico; explicit next vence repeat. |
| BIB-AC-03 | a entrega de TASK-E03-03 (Implementar import incremental e roots offline) | o teste de aceite da TASK roda | NAS indisponível não deleta item; symlink loop limitado; cancel mantém itens já indexados; move preserva ratings. |
| BIB-AC-04 | a entrega de TASK-E03-04 (Implementar leitura de tags e capas segura) | o teste de aceite da TASK roda | Unknown fields preservados para futuro write; image bomb limitada; metadata Unicode mantém representação original. |
| BIB-AC-05 | a entrega de TASK-E03-05 (Entregar FTS, filtros e views de biblioteca) | o teste de aceite da TASK roda | Search100k warmp95≤100ms; filtros combinados não executam SQL do usuário; ordering estável. |
| BIB-AC-06 | a entrega de TASK-E03-06 (Entregar playlists import/export e CUE) | o teste de aceite da TASK roda | Loop de playlists bloqueado; CUE75fps preserva offsets; export sem token; undo/redo100 ops da sessão. |
| BIB-AC-07 | a entrega de TASK-E03-07 (Implementar smart playlists e templates) | o teste de aceite da TASK roda | Query invalid rejeitada; atualização cancelável; playlist lock não bloqueia playback; snapshots consistentes. |
| BIB-AC-08 | a entrega de TASK-E03-08 (Implementar escrita de tags e capítulos em lote) | o teste de aceite da TASK roda | Fields mistos não são apagados sem escolha; kill/locked file mantém mídia; capítulo inválido não é gravado. |
| BIB-AC-09 | a entrega de TASK-E03-09 (Entregar letras, bookmarks e histórico) | o teste de aceite da TASK roda | Synced lyrics seguem clock consumido; rescan preserva bookmark; seek não conta como consumo; sem rede involuntária. |
| BIB-AC-10 | a entrega de TASK-E03-10 (Completar recovery, export e large-library gate) | o teste de aceite da TASK roda | Fila restaura sem autoplay; missing files têm erro agrupado; ratings/bookmarks sobrevivem upgrade e rollback seguro. |
| BIB-AC-11 | a entrega de TASK-E08-04 (Entregar organização de arquivos e dedupe) | o teste de aceite da TASK roda | Bloquear escape e nomes reservados; não sobrescrever por padrão; mover preserva identidade/bookmark; duplicatas não são apagadas automaticamente. |
| BIB-AC-12 | a entrega de TASK-E08-09 (Entregar audiobooks e progresso offline) | o teste de aceite da TASK roda | Velocidade por categoria; retomada explícita; conflito de retrocesso não usa máximo sempre; sem rede continua reproduzindo. |
| BIB-AC-13 | a entrega de TASK-E17-06 (Certificar tags técnicas, capítulos e sidecars) | o teste de aceite da TASK roda | Roundtrip preserva desconhecidos; capítulos/ratings e WVC não somem no rename; grant de sidecar não amplia acesso nem baixa URLs. |

**Nenhum destes testes existe ainda.** O nome é o compromisso; a TASK citada em §4 é
quem os escreve, e o teste nasce vermelho antes da implementação. O detalhamento
Given/When de cada critério está no arquivo da TASK (`tasks/items/TASK-Exx-nn.md`).

### Critérios que esta spec **não** fixa

- Escolha de crate de terceiros (toolkit, HTTP, banco, sandbox): é da TASK, com ADR quando a fronteira mudar.
- Números de desempenho além dos citados nos critérios: saem da medição na TASK.

## 6. Erros e cancelamento

**Erro é dado, não `panic!`.**

| Situação | Erro |
|---|---|
| Banco corrompido ou de versão futura | `LibraryError::Db { reason }` (abre só leitura) |
| Arquivo sumiu entre varredura e reprodução | `LibraryError::Missing { path }` |
| Tag inválida ou contêiner sem suporte a escrita | `LibraryError::TagWrite { path, reason }` |
| Playlist com linha ilegível | `LibraryError::Playlist { line }` (as demais linhas carregam) |
| Cancelado | `LibraryError::Cancelled` |

**Cancelamento é explícito, não timeout.** `CancelToken` é verificado entre arquivos na varredura; cancelar mantém o que já foi indexado e não deixa transação aberta.

## 7. Fora de escopo

Conversao em lote, restauracao por IA e servidor de streaming (E09): os dois primeiros sao apps separados, o terceiro nao tem repo

## 8. Dependências

Depende de: `vynamp-engine` (unico crate do projeto)

É dependido por: ninguem

## 9. Testes e fixtures

| Fixture | Origem | Uso |
|---|---|---|
| árvore de pastas sintética com 10 000 arquivos vazios e tags | gerada no teste | varredura e consulta |
| playlists M3U/M3U8/PLS com caminhos relativos, absolutos e quebrados | literais em `tests/fixtures/` | parsing |
| arquivos pequenos MP3/FLAC/OGG com tags conhecidas | `tests/fixtures/`, gerados por script versionado | leitura e escrita de tags |

Teste que depende de hardware, sessão gráfica ou SO específico vive atrás de
`#[ignore]` com o motivo escrito.

```
cargo test --test biblioteca_scan --locked
cargo test --test playlists --locked
cargo test --test tags_roundtrip --locked
```

## 10. Limitações conhecidas

Limitação declarada não reprova a entrega; limitação omitida reprova.

| Limitação | Efeito | Quem cobre |
|---|---|---|
| `src/` tem só `main.rs` com `println!`, zero dependências e nenhum `tests/` | nenhum critério verificável hoje | as TASKs de §4 |
| Motor de banco não escolhido | esquema e migração ficam para a TASK E03 que criar o índice | E03 |
| Letras sincronizadas dependem de fonte externa | só arquivos locais `.lrc` são cobertos | E08-04/E08-09 |
