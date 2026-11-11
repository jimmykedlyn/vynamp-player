# Spec — vynamp-player · streaming

| Campo | Valor |
|---|---|
| Módulo | `vynamp-player` |
| Crate(s) | `vynamp_player` (módulo `streaming`; crate único até uma TASK justificar divisão) |
| Tipo | app |
| Versão da spec | 0.2.0 |
| Data | 2026-11-11 |
| Estado | **rascunho autorado, não aprovado** — §3 a §6, §9 e §10 escritos a partir de `streaming/01-cliente.md`, `streaming/04-rede-proxy.md` e das TASKs E05, E12-08 e E17-11. A aprovação é do mantenedor, por `GOVERNANCE.md` |
| Dono | mantenedor do `vynamp-player` |
| Depende de | `vynamp-engine` (unico crate do projeto) |
| É dependido por | ninguem |

> §1, §2 e §7 foram decididos em `01-arvore-final.md` §2 e valem como norma.
> §3 a §6, §9 e §10 foram autorados em 2026-11-11 a partir de `streaming/01-cliente.md`, `streaming/04-rede-proxy.md` e das TASKs E05, E12-08 e E17-11,
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
pub mod streaming {
    /// Fonte remota entregue ao engine como bytes; o decode é do engine.
    pub struct HttpSource { /* cliente, URL corrente, política de redirect */ }
    impl HttpSource {
        pub fn open(url: &Url, opts: &HttpOptions, cancel: &CancelToken) -> Result<Self, StreamError>;
        /// Range: 206 → leitura posicionada; 200 → recomeço; 416 → seek indisponível.
        pub fn read_at(&mut self, offset: u64, buf: &mut [u8]) -> Result<RangeOutcome, StreamError>;
    }
    pub enum RangeOutcome { Partial { read: usize }, Restarted { read: usize }, SeekUnavailable }

    pub struct HttpOptions {
        pub max_redirects: u8,
        /// Authorization nunca segue para outra origem nem para http.
        pub auth: Option<AuthHeader>,
        pub proxy: ProxyConfig,
    }

    /// Rádio (Icecast/Shoutcast) com metadados ICY.
    pub struct RadioStream { /* … */ }
    /// Cache em disco com limites declarados (E05-04).
    pub struct StreamCache { pub max_encoded_bytes: u64, pub max_decoded_bytes: u64 }
}
```

## 4. Requisitos

| ID | Requisito | REQ de produto | TASK | Teste |
|---|---|---|---|---|
| STR-REQ-01 | Implementar SourceDescriptor e provider simulado — planned (M2) | REQ-E05-01 | TASK-E05-01 | — (STR-AC-01 planejado) |
| STR-REQ-02 | Entregar HTTP Range e reprodução progressiva — planned (M2) | REQ-E05-02 | TASK-E05-02 | — (STR-AC-02 planejado) |
| STR-REQ-03 | Entregar rádio Icecast/SHOUTcast e ICY — planned (M2) | REQ-E05-03 | TASK-E05-03 | — (STR-AC-03 planejado) |
| STR-REQ-04 | Entregar buffers, cache e downloads — planned (M2) | REQ-E05-04 | TASK-E05-04 | — (STR-AC-04 planejado) |
| STR-REQ-05 | Entregar feeds e episódios de podcasts — planned (M2) | REQ-E05-05 | TASK-E05-05 | — (STR-AC-05 planejado) |
| STR-REQ-06 | Entregar HLS sob demanda — planned (M3) | REQ-E05-06 | TASK-E05-06 | — (STR-AC-06 planejado) |
| STR-REQ-07 | Entregar HLS live, ABR e timeshift — planned (M3) | REQ-E05-07 | TASK-E05-07 | — (STR-AC-07 planejado) |
| STR-REQ-08 | Entregar gravação de rádios — planned (M3) | REQ-E05-08 | TASK-E05-08 | — (STR-AC-08 planejado) |
| STR-REQ-09 | Endurecer rede e diagnóstico de streaming — planned (M3) | REQ-E05-09 | TASK-E05-09 | — (STR-AC-09 planejado) |
| STR-REQ-10 | Integrar provider API, offline e direitos — planned (M4) | REQ-E05-10 | TASK-E05-10 | — (STR-AC-10 planejado) |
| STR-REQ-11 | Entregar DASH, WebDAV e clouds — planned (M5) | REQ-E12-08 | TASK-E12-08 | — (STR-AC-11 planejado) |
| STR-REQ-12 | Implementar política de proxy e rede isolada — planned (M2) | REQ-E17-11 | TASK-E17-11 | — (STR-AC-12 planejado) |

Fonte de cada requisito: o documento citado na TASK (`delivery/01-roadmap.md`, `streaming/01-cliente.md`, `streaming/04-rede-proxy.md`). Fases presentes: M2, M3, M4, M5.

## 5. Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| STR-AC-01 | a entrega de TASK-E05-01 (Implementar SourceDescriptor e provider simulado) | o teste de aceite da TASK roda | Distinguir capacidades de arquivo, live e VOD; nenhum token em playlist/log; transporte desconhecido retorna erro de suporte. |
| STR-AC-02 | a entrega de TASK-E05-02 (Entregar HTTP Range e reprodução progressiva) | o teste de aceite da TASK roda | Testar respostas 206/200/416; não encaminhar auth para outra origem; Stop cancela rede e decode e descarta buffers antigos. |
| STR-AC-03 | a entrega de TASK-E05-03 (Entregar rádio Icecast/SHOUTcast e ICY) | o teste de aceite da TASK roda | Separar metadados fragmentados dos bytes de áudio; limitar tags inválidas; reconexão limitada e cancelável; não inventar duração. |
| STR-AC-04 | a entrega de TASK-E05-04 (Entregar buffers, cache e downloads) | o teste de aceite da TASK roda | Rede lenta pode recuperar sem repetir amostras; respeitar limites de 32/16 MiB; logout limpa cache privado sem apagar mídia local. |
| STR-AC-05 | a entrega de TASK-E05-05 (Entregar feeds e episódios de podcasts) | o teste de aceite da TASK roda | Desabilitar entidades XML externas; GUID por feed; cancelar download preserva parcial válido; eventos de progresso são idempotentes. |
| STR-AC-06 | a entrega de TASK-E05-06 (Entregar HLS sob demanda) | o teste de aceite da TASK roda | Limitar manifests inválidos/aninhados; tratar segmentos ausentes; seek preserva timeline; não registrar chaves ou URLs secretas. |
| STR-AC-07 | a entrega de TASK-E05-07 (Entregar HLS live, ABR e timeshift) | o teste de aceite da TASK roda | Mudança de sequência não duplica áudio; seleção manual de qualidade é respeitada; seek só na janela disponível; voltar ao live edge funciona. |
| STR-AC-08 | a entrega de TASK-E05-08 (Entregar gravação de rádios) | o teste de aceite da TASK roda | Proibição explícita do provider bloqueia gravação; disco cheio encerra arquivo válido; título ICY não é tratado como fronteira exata de samples. |
| STR-AC-09 | a entrega de TASK-E05-09 (Endurecer rede e diagnóstico de streaming) | o teste de aceite da TASK roda | Revalidar SSRF/DNS/redirects conforme origem; esconder segredos; reconexão, cancelamento e stall não travam a interface. |
| STR-AC-10 | a entrega de TASK-E05-10 (Integrar provider API, offline e direitos) | o teste de aceite da TASK roda | Renovar token sem reiniciar indevidamente; download só se permitido; arquivos locais continuam após logout ou ausência de rede. |
| STR-AC-11 | a entrega de TASK-E12-08 (Entregar DASH, WebDAV e clouds) | o teste de aceite da TASK roda | Negociar auth/seek/codec/qualidade; DRM desconhecido gera erro explícito; roots privadas só com permissão. |
| STR-AC-12 | a entrega de TASK-E17-11 (Implementar política de proxy e rede isolada) | o teste de aceite da TASK roda | Proxy failure não cai direto; credenciais origin/proxy/provider separadas; redirects/filhos respeitam política; cancel/reconnect bounded. |

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
| Falha de rede ou DNS | `StreamError::Network { reason }` |
| Status HTTP inesperado | `StreamError::Http { status }` |
| Redirect para outra origem com credencial ou downgrade para http | `StreamError::UnsafeRedirect { to }` (credencial não é enviada) |
| Limite de redirect ou de cache excedido | `StreamError::Limit { what }` |
| Cancelado (Stop) | `StreamError::Cancelled` |

**Cancelamento é explícito, não timeout.** Stop dispara o `CancelToken`; o worker de rede fecha o socket, o decoder para e os buffers antigos são descartados, com join em até 2 s (E05-02).

## 7. Fora de escopo

Conversao em lote, restauracao por IA e servidor de streaming (E09): os dois primeiros sao apps separados, o terceiro nao tem repo

## 8. Dependências

Depende de: `vynamp-engine` (unico crate do projeto)

É dependido por: ninguem

## 9. Testes e fixtures

| Fixture | Origem | Uso |
|---|---|---|
| servidor HTTP em `TcpListener` efêmero com rotas 206/200/416/302 | em processo, no teste | Range, redirect e Stop |
| MP3 CBR 128 kbps de 600 000 bytes | gerado por script versionado | Range e seek |
| servidor ICY simulado | em processo, no teste | rádio e metadados |

Teste que depende de hardware, sessão gráfica ou SO específico vive atrás de
`#[ignore]` com o motivo escrito.

```
cargo test --test http_range --locked
cargo test --test radio_icy --locked
```

## 10. Limitações conhecidas

Limitação declarada não reprova a entrega; limitação omitida reprova.

| Limitação | Efeito | Quem cobre |
|---|---|---|
| `src/` tem só `main.rs` com `println!`, zero dependências e nenhum `tests/` | nenhum critério verificável hoje | as TASKs de §4 |
| Nenhum teste toca rede externa | CDNs e servidores reais não são exercitados | smoke manual registrado em `evidence/` |
| Servidor de streaming (E09) não tem repo | só o cliente é especificado | decisão de produto |
