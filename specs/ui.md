# Spec — vynamp-player · ui

| Campo | Valor |
|---|---|
| Módulo | `vynamp-player` |
| Crate(s) | `vynamp_player` (módulo `ui`; crate único até uma TASK justificar divisão) |
| Tipo | app |
| Versão da spec | 0.2.0 |
| Data | 2026-11-11 |
| Estado | **rascunho autorado, não aprovado** — §3 a §6, §9 e §10 escritos a partir de `design/02-layouts.md`, `design/05-mobile-tablet.md`, `design/07-pin-mvp.md` e das TASKs E02. A aprovação é do mantenedor, por `GOVERNANCE.md` |
| Dono | mantenedor do `vynamp-player` |
| Depende de | `vynamp-engine` (unico crate do projeto) |
| É dependido por | ninguem |

> §1, §2 e §7 foram decididos em `01-arvore-final.md` §2 e valem como norma.
> §3 a §6, §9 e §10 foram autorados em 2026-11-11 a partir de `design/02-layouts.md`, `design/05-mobile-tablet.md`, `design/07-pin-mvp.md` e das TASKs E02,
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
pub mod ui {
    /// Tokens canônicos de `design/02-layouts.md`; nenhuma cor/medida literal fora daqui.
    pub struct Tokens { /* cor, tipografia, espaçamento, raio, foco */ }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum ControlState { Normal, Hover, Pressed, Focused, Disabled }

    /// Todo controle tem nome acessível e estado; sem nome, não compila o build de UI.
    pub trait Control {
        fn accessible_name(&self) -> &str;
        fn state(&self) -> ControlState;
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Layout { ClassicStack, Compact, Mobile, Tablet }

    /// Ações de transporte; a UI só emite comandos, quem executa é o engine.
    pub enum TransportCmd { Open, Play, Pause, Stop, Seek(f64), Volume(f32), Shuffle(bool), Repeat(RepeatMode) }

    pub struct Ui { /* árvore de widgets + estado de layout */ }
    impl Ui {
        pub fn new(tokens: Tokens, layout: Layout) -> Self;
        pub fn set_layout(&mut self, layout: Layout);
        /// Comandos produzidos pela interação desde a última chamada.
        pub fn drain_commands(&mut self) -> Vec<TransportCmd>;
    }
}
```

O toolkit Rust é escolhido por E02-01 (spike de acessibilidade); trocar de toolkit
exige ADR. Esta API é a fronteira que sobrevive à troca.

## 4. Requisitos

| ID | Requisito | REQ de produto | TASK | Teste |
|---|---|---|---|---|
| UI-REQ-01 | Validar toolkit Rust com UI clássica acessível — planned (M0) | REQ-E02-01 | TASK-E02-01 | — (UI-AC-01 planejado) |
| UI-REQ-02 | Implementar tokens e componentes semânticos — planned (M1) | REQ-E02-02 | TASK-E02-02 | — (UI-AC-02 planejado) |
| UI-REQ-03 | Construir Classic Stack e transporte — planned (M1) | REQ-E02-03 | TASK-E02-03 | — (UI-AC-03 planejado) |
| UI-REQ-04 | Entregar playlist UI e keyboard workflow — planned (M1) | REQ-E02-04 | TASK-E02-04 | — (UI-AC-04 planejado) |
| UI-REQ-05 | Construir Library Workspace e inspector — planned (M2) | REQ-E02-05 | TASK-E02-05 | — (UI-AC-05 planejado) |
| UI-REQ-06 | Entregar Mini, preferências e estados de erro — planned (M2) | REQ-E02-06 | TASK-E02-06 | — (UI-AC-06 planejado) |
| UI-REQ-07 | Integrar cadeia de áudio e EQ avançado na UI — planned (M3) | REQ-E02-07 | TASK-E02-07 | — (UI-AC-07 planejado) |
| UI-REQ-08 | Entregar docking e persistência multi-monitor — planned (M3) | REQ-E02-08 | TASK-E02-08 | — (UI-AC-08 planejado) |
| UI-REQ-09 | Completar localização e testes visuais — planned (M3) | REQ-E02-09 | TASK-E02-09 | — (UI-AC-09 planejado) |
| UI-REQ-10 | Validar experiência desktop ponta a ponta — planned (M3) | REQ-E02-10 | TASK-E02-10 | — (UI-AC-10 planejado) |
| UI-REQ-11 | Congelar MVP de referência e extrair SVGs, ícones e temas — planned (M0) | REQ-E02-11 | TASK-E02-11 | — (UI-AC-11 planejado) |
| UI-REQ-12 | Gerar gabarito visual e estratégia de fontes — planned (M0) | REQ-E02-12 | TASK-E02-12 | — (UI-AC-12 planejado) |
| UI-REQ-13 | Gerar gabarito de movimento, medidores e ondas — planned (M0) | REQ-E02-13 | TASK-E02-13 | — (UI-AC-13 planejado) |
| UI-REQ-14 | Gerar gabarito de comportamento e preferências — planned (M0) | REQ-E02-14 | TASK-E02-14 | — (UI-AC-14 planejado) |
| UI-REQ-15 | Mapear lacunas web para nativo por plataforma — planned (M0) | REQ-E02-15 | TASK-E02-15 | — (UI-AC-15 planejado) |
| UI-REQ-16 | Implementar harness de fidelidade com egui_kittest — planned (M1) | REQ-E02-16 | TASK-E02-16 | — (UI-AC-16 planejado) |

Fonte de cada requisito: o documento citado na TASK (`design/02-layouts.md`, `design/07-pin-mvp.md`). Fases presentes: M0, M1, M2, M3.

## 5. Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| UI-AC-01 | a entrega de TASK-E02-01 (Validar toolkit Rust com UI clássica acessível) | o teste de aceite da TASK roda | Narrator/NVDA/Orca/VoiceOver workflow de transporte funciona; DPI/zoom200% sem esconder ações; falhas mudam toolkit com ADR. Android tem gate E13-01/05. |
| UI-AC-02 | a entrega de TASK-E02-02 (Implementar tokens e componentes semânticos) | o teste de aceite da TASK roda | Usar tokens canônicos; focus/hover/pressed/disabled diferem; contraste medido e nome acessível em cada controle. |
| UI-AC-03 | a entrega de TASK-E02-03 (Construir Classic Stack e transporte) | o teste de aceite da TASK roda | Abrir/play/pause/stop/seek/volume/shuffle/repeat operam; resizing mínimo mantém targets e pipeline status. |
| UI-AC-04 | a entrega de TASK-E02-04 (Entregar playlist UI e keyboard workflow) | o teste de aceite da TASK roda | Tocar/agendar/adicionar têm comportamentos distintos; teclado completo e faixa atual separada da seleção. |
| UI-AC-05 | a entrega de TASK-E02-05 (Construir Library Workspace e inspector) | o teste de aceite da TASK roda | 100k rows sem widget por item; inspector vira drawer abaixo1152DIP; search cancelável não altera fila. |
| UI-AC-06 | a entrega de TASK-E02-06 (Entregar Mini, preferências e estados de erro) | o teste de aceite da TASK roda | Trocar modo mantém áudio; Escape/focus return corretos; erros mostram ações úteis sem stacktrace no fluxo. |
| UI-AC-07 | a entrega de TASK-E02-07 (Integrar cadeia de áudio e EQ avançado na UI) | o teste de aceite da TASK roda | Origem/transformações/saída real visíveis; EQ em transparente pede saída do modo; curvas refletem filtros reais. |
| UI-AC-08 | a entrega de TASK-E02-08 (Entregar docking e persistência multi-monitor) | o teste de aceite da TASK roda | Monitor removido não deixa janela inacessível; Alt ignora snap; resize/dock não trava reprodução. |
| UI-AC-09 | a entrega de TASK-E02-09 (Completar localização e testes visuais) | o teste de aceite da TASK roda | Textos longos/CJK/emoji e dados RTL não quebram UI; estilos passam contraste e reduced motion. |
| UI-AC-10 | a entrega de TASK-E02-10 (Validar experiência desktop ponta a ponta) | o teste de aceite da TASK roda | Sem mouse: abrir→tocar→seek→fila→saída→tema; todas telas têm loading/empty/erro/permission state verificável. |
| UI-AC-11 | a entrega de TASK-E02-11 (Congelar MVP de referência e extrair SVGs, ícones e temas) | o teste de aceite da TASK roda | Cada SVG renderiza em resvg sem pixel de geometria divergente do Chrome (delta>48 fora do antialiasing de 1 px; texto registrado e resolvido em E02-12; recurso sem suporte no resvg vira requisito do carregador nativo); cada tema traz resolvidos todos os tokens --* que define em :root, e toda variável referenciada que o tema não define em :root consta de unsetVars com fallback, origem e temas afetados (valor por estado em E02-12); reextração reproduz os mesmos hashes. |
| UI-AC-12 | a entrega de TASK-E02-12 (Gerar gabarito visual e estratégia de fontes) | o teste de aceite da TASK roda | Todo estado alcançável do MVP tem captura e dump (geometria, estilo computado e custom properties --* por elemento, inclusive as de unsetVars), incluindo os estados cujo clique expira na varredura de TASK-E02-11 (`rendered/click-failures.json`: link de salto com foco, fila limpa), acionados por teclado ou script; cada família de fonte usada tem fonte nativa ou substituta licenciada com métricas comparadas; armazenamento dos PNG decidido. |
| UI-AC-13 | a entrega de TASK-E02-13 (Gerar gabarito de movimento, medidores e ondas) | o teste de aceite da TASK roda | Cada animação do MVP tem duração, easing e keyframes ou série temporal; medidores e ondas respondem a degrau, seno e rajada com tolerância documentada; Web Audio entra só como comportamento reimplementável. |
| UI-AC-14 | a entrega de TASK-E02-14 (Gerar gabarito de comportamento e preferências) | o teste de aceite da TASK roda | Cada controle do MVP tem roteiro com resultado verificável; o esquema cobre todas as chaves lidas ou escritas; nenhuma ação fica sem roteiro. |
| UI-AC-15 | a entrega de TASK-E02-15 (Mapear lacunas web para nativo por plataforma) | o teste de aceite da TASK roda | Zero lacunas sem plano; exceções só para APIs intrínsecas do navegador com comportamento reimplementado; toda biblioteca nova tem repositório git próprio e modular. |
| UI-AC-16 | a entrega de TASK-E02-16 (Implementar harness de fidelidade com egui_kittest) | o teste de aceite da TASK roda | O harness falha quando uma tela ou estado excede a tolerância e aponta o elemento divergente; roda apenas localmente, na máquina do mantenedor, com o clone de vynamp-mvp-reference no SHA 0d9860b901510021bcca4c31a25014373ea7c452; o CI Gate o marca como não executado. |

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
| Toolkit não expõe a árvore de acessibilidade na plataforma | `UiError::A11yUnavailable { platform }` |
| Token ausente ou inválido no tema carregado | `UiError::Token { name }` (cai no token padrão) |
| Layout pedido não cabe na janela mínima | `UiError::LayoutTooSmall { min }` |

**Cancelamento é explícito, não timeout.** A UI não tem operações longas: cada comando é síncrono e o cancelamento de reprodução é do engine.

## 7. Fora de escopo

Conversao em lote, restauracao por IA e servidor de streaming (E09): os dois primeiros sao apps separados, o terceiro nao tem repo

## 8. Dependências

Depende de: `vynamp-engine` (unico crate do projeto)

É dependido por: ninguem

## 9. Testes e fixtures

| Fixture | Origem | Uso |
|---|---|---|
| árvore de acessibilidade capturada (UIA, AT-SPI, NSAccessibility) | gerada no teste de cada plataforma | critérios de leitor de tela |
| capturas em DPI 100/150/200 % | geradas no CI de UI | critérios de escala e zoom |
| pares de cor dos tokens | `design/02-layouts.md` | contraste medido |

Teste que depende de hardware, sessão gráfica ou SO específico vive atrás de
`#[ignore]` com o motivo escrito.

```
cargo test --test ui_tokens --locked
cargo test --test ui_a11y --locked -- --ignored  # exige leitor de tela/sessão gráfica
```

## 10. Limitações conhecidas

Limitação declarada não reprova a entrega; limitação omitida reprova.

| Limitação | Efeito | Quem cobre |
|---|---|---|
| `src/` tem só `main.rs` com `println!`, zero dependências e nenhum `tests/` | nenhum critério verificável hoje | as TASKs de §4 |
| Leitor de tela real (Narrator, NVDA, Orca, VoiceOver, TalkBack) só roda em máquina com sessão gráfica | testes de a11y ficam atrás de `#[ignore]` | execução manual registrada em `evidence/` |
| Toolkit ainda não escolhido | a API de §3 é esboço; nomes podem mudar | E02-01 + ADR |
