# Spec — vynamp-player · plugins

| Campo | Valor |
|---|---|
| Módulo | `vynamp-player` |
| Crate(s) | `vynamp_player` (módulo `plugins`; crate único até uma TASK justificar divisão) |
| Tipo | app |
| Versão da spec | 0.2.0 |
| Data | 2026-11-11 |
| Estado | **rascunho autorado, não aprovado** — §3 a §6, §9 e §10 escritos a partir de `extensions/02-plugins.md` e das TASKs E07. A aprovação é do mantenedor, por `GOVERNANCE.md` |
| Dono | mantenedor do `vynamp-player` |
| Depende de | `vynamp-engine` (unico crate do projeto) |
| É dependido por | ninguem |

> §1, §2 e §7 foram decididos em `01-arvore-final.md` §2 e valem como norma.
> §3 a §6, §9 e §10 foram autorados em 2026-11-11 a partir de `extensions/02-plugins.md` e das TASKs E07,
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
pub mod plugins {
    /// Plugin roda isolado (processo ou sandbox decidido em E07-01); nunca no thread de áudio.
    pub struct PluginManifest { pub id: String, pub version: String, pub api: u32, pub permissions: Vec<Permission> }
    #[non_exhaustive]
    pub enum Permission { Network { origins: Vec<String> }, Library, Playback, Ui }

    pub struct PluginHost { /* broker de permissões + supervisor */ }
    impl PluginHost {
        pub fn install(&mut self, pkg: &Path) -> Result<PluginId, PluginError>;
        /// Permissão pedida é mostrada e concedida pelo usuário; negar não derruba o player.
        pub fn grant(&mut self, id: &PluginId, perm: Permission) -> Result<(), PluginError>;
        pub fn disable(&mut self, id: &PluginId) -> Result<(), PluginError>;
    }
}
```

## 4. Requisitos

| ID | Requisito | REQ de produto | TASK | Teste |
|---|---|---|---|---|
| PLG-REQ-01 | Provar sandbox WASM e capabilities — planned (M0) | REQ-E07-01 | TASK-E07-01 | — (PLG-AC-01 planejado) |
| PLG-REQ-02 | Definir pacote e SDK de plugins v1 — planned (M3) | REQ-E07-02 | TASK-E07-02 | — (PLG-AC-02 planejado) |
| PLG-REQ-03 | Implementar broker de permissões — planned (M3) | REQ-E07-03 | TASK-E07-03 | — (PLG-AC-03 planejado) |
| PLG-REQ-04 | Implementar lifecycle e circuit breaker — planned (M3) | REQ-E07-04 | TASK-E07-04 | — (PLG-AC-04 planejado) |
| PLG-REQ-05 | Implementar helper nativo isolado — planned (M3) | REQ-E07-05 | TASK-E07-05 | — (PLG-AC-05 planejado) |
| PLG-REQ-06 | Entregar templates e plugins de exemplo — planned (M3) | REQ-E07-06 | TASK-E07-06 | — (PLG-AC-06 planejado) |
| PLG-REQ-07 | Entregar gerenciador de plugins — planned (M3) | REQ-E07-07 | TASK-E07-07 | — (PLG-AC-07 planejado) |
| PLG-REQ-08 | Entregar documentação e harness do SDK — planned (M3) | REQ-E07-08 | TASK-E07-08 | — (PLG-AC-08 planejado) |
| PLG-REQ-09 | Preparar registry e revogação de plugins — planned (M4) | REQ-E07-09 | TASK-E07-09 | — (PLG-AC-09 planejado) |
| PLG-REQ-10 | Auditar isolamento das extensões — planned (M3) | REQ-E07-10 | TASK-E07-10 | — (PLG-AC-10 planejado) |

Fonte de cada requisito: o documento citado na TASK (`extensions/02-plugins.md`). Fases presentes: M0, M3, M4.

## 5. Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| PLG-AC-01 | a entrega de TASK-E07-01 (Provar sandbox WASM e capabilities) | o teste de aceite da TASK roda | Interromper loops e crescimento de memória; nenhum acesso ambiente a arquivos/rede; medir consumo com runtime não carregado. |
| PLG-AC-02 | a entrega de TASK-E07-02 (Definir pacote e SDK de plugins v1) | o teste de aceite da TASK roda | Validar tipo, plataformas e permissões; major incompatível não carrega; assinatura não permite ignorar políticas de acesso. |
| PLG-AC-03 | a entrega de TASK-E07-03 (Implementar broker de permissões) | o teste de aceite da TASK roda | Negar e revogar bloqueia requests/handles; redirect revalida domínio; plugin não recebe shell, env vars ou filesystem completo. |
| PLG-AC-04 | a entrega de TASK-E07-04 (Implementar lifecycle e circuit breaker) | o teste de aceite da TASK roda | Três falhas em 60 s isolam plugin; cleanup é idempotente; floods são limitados; playback local continua. |
| PLG-AC-05 | a entrega de TASK-E07-05 (Implementar helper nativo isolado) | o teste de aceite da TASK roda | Crash ou RPC inválido do child não derruba main; handles mínimos; sandbox testado; nenhuma DLL arbitrária carregada no main. |
| PLG-AC-06 | a entrega de TASK-E07-06 (Entregar templates e plugins de exemplo) | o teste de aceite da TASK roda | Exemplos funcionam no harness offline com permissões mínimas; documentação de lifecycle acompanha a versão do SDK. |
| PLG-AC-07 | a entrega de TASK-E07-07 (Entregar gerenciador de plugins) | o teste de aceite da TASK roda | Novas permissões exigem intenção; rollback restaura estado compatível; falha mostra ação segura e ID de diagnóstico. |
| PLG-AC-08 | a entrega de TASK-E07-08 (Entregar documentação e harness do SDK) | o teste de aceite da TASK roda | Host simulado cobre erros e grants; corpus adversarial aprovado; versão minor aditiva não quebra exemplos. |
| PLG-AC-09 | a entrega de TASK-E07-09 (Preparar registry e revogação de plugins) | o teste de aceite da TASK roda | Build revogado não instala/atualiza; pacote manual ainda exige consentimento; hashes de download são verificados. |
| PLG-AC-10 | a entrega de TASK-E07-10 (Auditar isolamento das extensões) | o teste de aceite da TASK roda | Traps, loops, crash de helper e revoke durante I/O não interrompem áudio local; permissões negadas impedem exfiltração; críticos resolvidos. |

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
| Manifesto inválido ou API incompatível | `PluginError::Manifest { reason }` |
| Chamada fora das permissões concedidas | `PluginError::Denied { perm }` |
| Plugin travou, excedeu tempo ou memória | `PluginError::Crashed { id }` (plugin desativado, player segue) |
| Assinatura ou hash não confere | `PluginError::Integrity { id }` |

**Cancelamento é explícito, não timeout.** Desativar um plugin encerra seu processo/sandbox; chamadas pendentes recebem `Cancelled`.

## 7. Fora de escopo

Conversao em lote, restauracao por IA e servidor de streaming (E09): os dois primeiros sao apps separados, o terceiro nao tem repo

## 8. Dependências

Depende de: `vynamp-engine` (unico crate do projeto)

É dependido por: ninguem

## 9. Testes e fixtures

| Fixture | Origem | Uso |
|---|---|---|
| plugins de teste: válido, sem permissão, que trava, que vaza memória | compilados no teste | isolamento e broker |
| manifestos inválidos | literais em `tests/fixtures/plugins/` | validação |

Teste que depende de hardware, sessão gráfica ou SO específico vive atrás de
`#[ignore]` com o motivo escrito.

```
cargo test --test plugins_broker --locked
cargo test --test plugins_isolamento --locked
```

## 10. Limitações conhecidas

Limitação declarada não reprova a entrega; limitação omitida reprova.

| Limitação | Efeito | Quem cobre |
|---|---|---|
| `src/` tem só `main.rs` com `println!`, zero dependências e nenhum `tests/` | nenhum critério verificável hoje | as TASKs de §4 |
| Mecanismo de isolamento não escolhido | API de §3 é esboço | E07-01 + ADR |
| Marketplace e distribuição de plugins | fora desta spec | decisão futura |
