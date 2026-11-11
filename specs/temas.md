# Spec — vynamp-player · temas

| Campo | Valor |
|---|---|
| Módulo | `vynamp-player` |
| Crate(s) | `vynamp_player` (módulo `temas`; crate único até uma TASK justificar divisão) |
| Tipo | app |
| Versão da spec | 0.2.0 |
| Data | 2026-11-11 |
| Estado | **rascunho autorado, não aprovado** — §3 a §6, §9 e §10 escritos a partir de `extensions/01-temas.md`, `extensions/05-visualizador.md` e das TASKs E06, E17-09 e E17-10. A aprovação é do mantenedor, por `GOVERNANCE.md` |
| Dono | mantenedor do `vynamp-player` |
| Depende de | `vynamp-engine` (unico crate do projeto) |
| É dependido por | ninguem |

> §1, §2 e §7 foram decididos em `01-arvore-final.md` §2 e valem como norma.
> §3 a §6, §9 e §10 foram autorados em 2026-11-11 a partir de `extensions/01-temas.md`, `extensions/05-visualizador.md` e das TASKs E06, E17-09 e E17-10,
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
pub mod temas {
    /// Pacote de tema: manifesto + tokens + recursos; nunca código executável.
    pub struct ThemePackage { pub manifest: ThemeManifest, pub tokens: ThemeTokens, pub assets: Vec<AssetRef> }
    pub struct ThemeManifest { pub id: String, pub version: String, pub min_player: String, pub license: String }

    /// Valida tamanho, caminhos (sem `..`), formatos de imagem e contraste mínimo.
    pub fn load(path: &Path) -> Result<ThemePackage, ThemeError>;
    /// Troca a quente; em erro, mantém o tema anterior.
    pub fn apply(ui: &mut Ui, theme: &ThemePackage) -> Result<(), ThemeError>;
    pub fn export(theme: &ThemePackage, dst: &Path) -> Result<(), ThemeError>;
}
```

## 4. Requisitos

| ID | Requisito | REQ de produto | TASK | Teste |
|---|---|---|---|---|
| TEM-REQ-01 | Definir schema e validador de temas — planned (M2) | REQ-E06-01 | TASK-E06-01 | — (TEM-AC-01 planejado) |
| TEM-REQ-02 | Entregar instalação atômica de temas — planned (M2) | REQ-E06-02 | TASK-E06-02 | — (TEM-AC-02 planejado) |
| TEM-REQ-03 | Implementar layouts declarativos e bindings — planned (M2) | REQ-E06-03 | TASK-E06-03 | — (TEM-AC-03 planejado) |
| TEM-REQ-04 | Produzir três temas originais — planned (M2) | REQ-E06-04 | TASK-E06-04 | — (TEM-AC-04 planejado) |
| TEM-REQ-05 | Entregar meters e espectro — planned (M2) | REQ-E06-05 | TASK-E06-05 | — (TEM-AC-05 planejado) |
| TEM-REQ-06 | Entregar visualizações avançadas — planned (M3) | REQ-E06-06 | TASK-E06-06 | — (TEM-AC-06 planejado) |
| TEM-REQ-07 | Entregar editor e preview de temas — planned (M3) | REQ-E06-07 | TASK-E06-07 | — (TEM-AC-07 planejado) |
| TEM-REQ-08 | Entregar galeria local e recovery visual — planned (M3) | REQ-E06-08 | TASK-E06-08 | — (TEM-AC-08 planejado) |
| TEM-REQ-09 | Preparar publicação de temas — planned (M4) | REQ-E06-09 | TASK-E06-09 | — (TEM-AC-09 planejado) |
| TEM-REQ-10 | Validar fidelidade visual dos temas — planned (M3) | REQ-E06-10 | TASK-E06-10 | — (TEM-AC-10 planejado) |
| TEM-REQ-11 | Entregar presets e taps limitados de visualização — planned (M3) | REQ-E17-09 | TASK-E17-09 | — (TEM-AC-11 planejado) |
| TEM-REQ-12 | Certificar visualizers, fallback e energia mobile — planned (M3) | REQ-E17-10 | TASK-E17-10 | — (TEM-AC-12 planejado) |

Fonte de cada requisito: o documento citado na TASK (`extensions/01-temas.md`, `extensions/05-visualizador.md`). Fases presentes: M2, M3, M4.

## 5. Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| TEM-AC-01 | a entrega de TASK-E06-01 (Definir schema e validador de temas) | o teste de aceite da TASK roda | Rejeitar schema major desconhecido; avisar campos opcionais desconhecidos; detectar ciclos e ações essenciais ausentes. |
| TEM-AC-02 | a entrega de TASK-E06-02 (Entregar instalação atômica de temas) | o teste de aceite da TASK roda | Rejeitar traversal, paths absolutos, symlinks e zip bombs; update inválido mantém tema atual; limite de 20 MiB extraídos. |
| TEM-AC-03 | a entrega de TASK-E06-03 (Implementar layouts declarativos e bindings) | o teste de aceite da TASK roda | Nenhum código ou acesso de rede arbitrário; controles essenciais e semântica acessível permanecem; constraints de geometria são aplicadas. |
| TEM-AC-04 | a entrega de TASK-E06-04 (Produzir três temas originais) | o teste de aceite da TASK roda | Os três oferecem Classic/Library/Mini; tokens, cores e estados conferidos em snapshots; nenhum asset copiado. |
| TEM-AC-05 | a entrega de TASK-E06-05 (Entregar meters e espectro) | o teste de aceite da TASK roda | Renderer lento não causa underrun; escalas de FFT correspondem à taxa; minimizar e reduzir movimento diminuem processamento. |
| TEM-AC-06 | a entrega de TASK-E06-06 (Entregar visualizações avançadas) | o teste de aceite da TASK roda | GPU indisponível não interrompe áudio; Escape sai do fullscreen; presets não executam shaders arbitrários. |
| TEM-AC-07 | a entrega de TASK-E06-07 (Entregar editor e preview de temas) | o teste de aceite da TASK roda | Exportar e importar preserva configuração; avisar problemas de contraste/constraints; preview não altera tema ativo nem áudio. |
| TEM-AC-08 | a entrega de TASK-E06-08 (Entregar galeria local e recovery visual) | o teste de aceite da TASK roda | Falha no renderer reverte ao builtin; safe mode ignora pacotes do usuário; monitor/zoom restaurados mantêm controles acessíveis. |
| TEM-AC-09 | a entrega de TASK-E06-09 (Preparar publicação de temas) | o teste de aceite da TASK roda | Versão publicada é imutável; revogação e rollback funcionam; instalação manual continua disponível sem conta ou rede. |
| TEM-AC-10 | a entrega de TASK-E06-10 (Validar fidelidade visual dos temas) | o teste de aceite da TASK roda | Conferir todos os modos/DPI e hierarchy clássica; falhas de acessibilidade não são ocultadas por aprovação de screenshots. |
| TEM-AC-11 | a entrega de TASK-E17-09 (Entregar presets e taps limitados de visualização) | o teste de aceite da TASK roda | Preset inválido/código arbitrário rejeitado; orçamento agregado e overflow descartam visuais; nunca bloqueiam callback/clock. |
| TEM-AC-12 | a entrega de TASK-E17-10 (Certificar visualizers, fallback e energia mobile) | o teste de aceite da TASK roda | GPU perdida mantém áudio; zero rendering Android screen-off/background; reduced motion, foco e FPS/budgets medidos. |

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
| Manifesto ausente ou inválido | `ThemeError::Manifest { reason }` |
| Caminho escapa do pacote ou recurso acima do limite | `ThemeError::UnsafeAsset { path }` |
| Contraste abaixo do mínimo dos tokens | `ThemeError::Contrast { pair }` (tema recusado) |
| Versão mínima do player maior que a atual | `ThemeError::Incompatible { min }` |

**Cancelamento é explícito, não timeout.** Carregar tema é síncrono e limitado por tamanho; não há o que cancelar.

## 7. Fora de escopo

Conversao em lote, restauracao por IA e servidor de streaming (E09): os dois primeiros sao apps separados, o terceiro nao tem repo

## 8. Dependências

Depende de: `vynamp-engine` (unico crate do projeto)

É dependido por: ninguem

## 9. Testes e fixtures

| Fixture | Origem | Uso |
|---|---|---|
| pacotes válidos e maliciosos (zip-slip, imagem gigante, contraste baixo) | `tests/fixtures/temas/` | validação |
| tema padrão | gerado dos tokens de `ui.md` | troca a quente e fallback |

Teste que depende de hardware, sessão gráfica ou SO específico vive atrás de
`#[ignore]` com o motivo escrito.

```
cargo test --test temas_validacao --locked
cargo test --test temas_apply --locked
```

## 10. Limitações conhecidas

Limitação declarada não reprova a entrega; limitação omitida reprova.

| Limitação | Efeito | Quem cobre |
|---|---|---|
| `src/` tem só `main.rs` com `println!`, zero dependências e nenhum `tests/` | nenhum critério verificável hoje | as TASKs de §4 |
| Visualizador depende de render da UI | critérios visuais exigem sessão gráfica | `#[ignore]` + evidência manual |
