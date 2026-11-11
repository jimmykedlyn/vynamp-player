# Spec — vynamp-player · player

| Campo | Valor |
|---|---|
| Módulo | `vynamp-player` |
| Crate(s) | `vynamp_player` (binário; shell, integração com o SO e plataformas) |
| Tipo | app |
| Versão da spec | 0.2.0 |
| Data | 2026-11-11 |
| Estado | **rascunho autorado, não aprovado** — §3 a §6, §9 e §10 escritos a partir de `platforms/01-matriz-suporte.md` a `05-ios.md`, `design/07-pin-mvp.md`, `delivery/01-roadmap.md` e das TASKs E08, E12, E13, E16 e E20. A aprovação é do mantenedor, por `GOVERNANCE.md` |
| Dono | mantenedor do `vynamp-player` |
| Depende de | `vynamp-engine` (unico crate do projeto) |
| É dependido por | ninguem |

> §1, §2 e §7 foram decididos em `01-arvore-final.md` §2 e valem como norma.
> §3 a §6, §9 e §10 foram autorados em 2026-11-11 a partir de `platforms/01-matriz-suporte.md` a `05-ios.md`, `design/07-pin-mvp.md`, `delivery/01-roadmap.md` e das TASKs E08, E12, E13, E16 e E20,
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
| vynamp-engine (unico crate do projeto); crates de UI e plataforma de terceiros | acessar decode/output/encode/dsp direto; qualquer dependencia de converter ou restore |

## 3. API pública (esboço)

```rust
/// Shell do app: liga UI, biblioteca, streaming, temas e plugins ao `vynamp-engine`.
pub struct App { /* engine, ui, library, … */ }
impl App {
    pub fn run(cfg: AppConfig) -> Result<(), AppError>;
}

/// Integração com o SO, por plataforma (atrás de `cfg(target_os)`).
pub mod sistema {
    pub trait MediaSession {          // SMTC, MPRIS, MPNowPlaying, MediaSession Android
        fn publish(&mut self, now: &NowPlaying) -> Result<(), AppError>;
        fn commands(&mut self) -> Vec<TransportCmd>;
    }
    pub fn register_file_associations(exts: &[&str]) -> Result<(), AppError>;
    pub fn single_instance(args: &[OsString]) -> Result<InstanceRole, AppError>;
}

/// Mobile (Android/iOS) é só player, com DSP adaptado (E20-05):
/// sem converter, sem restore, sem plugins nativos.
#[cfg(any(target_os = "android", target_os = "ios"))]
pub mod mobile { pub struct MobileProfile { pub dsp: AdaptedDsp } }
```

## 4. Requisitos

| ID | Requisito | REQ de produto | TASK | Teste |
|---|---|---|---|---|
| PLY-REQ-01 | Entregar análise de loudness, ReplayGain e BPM — planned (M3) | REQ-E08-03 | TASK-E08-03 | — (PLY-AC-01 planejado) |
| PLY-REQ-02 | Entregar sleep timer e scheduler — planned (M3) | REQ-E08-05 | TASK-E08-05 | — (PLY-AC-02 planejado) |
| PLY-REQ-03 | Integrar controles do Windows — planned (M2) | REQ-E08-06 | TASK-E08-06 | — (PLY-AC-03 planejado) |
| PLY-REQ-04 | Entregar CLI e IPC de instância única — planned (M3) | REQ-E08-07 | TASK-E08-07 | — (PLY-AC-04 planejado) |
| PLY-REQ-05 | Entregar scrobblers e presence — planned (M3) | REQ-E08-08 | TASK-E08-08 | — (PLY-AC-05 planejado) |
| PLY-REQ-06 | Entregar controle remoto autenticado — planned (M4) | REQ-E08-10 | TASK-E08-10 | — (PLY-AC-06 planejado) |
| PLY-REQ-07 | Entregar Linux nativo e MPRIS na 1.0 — planned (M3) | REQ-E12-01 | TASK-E12-01 | — (PLY-AC-07 planejado) |
| PLY-REQ-08 | Entregar macOS Intel e Apple Silicon nativos na 1.0 — planned (M3) | REQ-E12-02 | TASK-E12-02 | — (PLY-AC-08 planejado) |
| PLY-REQ-09 | Entregar casting e companion — planned (M5) | REQ-E12-09 | TASK-E12-09 | — (PLY-AC-09 planejado) |
| PLY-REQ-10 | Provar Android arm64, bridge Rust e output — planned (M0) | REQ-E13-01 | TASK-E13-01 | — (PLY-AC-10 planejado) |
| PLY-REQ-11 | Entregar serviço e MediaSession com engine Rust — planned (M1) | REQ-E13-02 | TASK-E13-02 | — (PLY-AC-11 planejado) |
| PLY-REQ-12 | Entregar audio focus e rotas Android — planned (M1) | REQ-E13-03 | TASK-E13-03 | — (PLY-AC-12 planejado) |
| PLY-REQ-13 | Entregar MediaStore, SAF e permissões — planned (M2) | REQ-E13-04 | TASK-E13-04 | — (PLY-AC-13 planejado) |
| PLY-REQ-14 | Entregar interface phone e navegação acessível — planned (M1) | REQ-E13-05 | TASK-E13-05 | — (PLY-AC-14 planejado) |
| PLY-REQ-15 | Integrar streaming, podcasts e downloads Android — planned (M2) | REQ-E13-06 | TASK-E13-06 | — (PLY-AC-15 planejado) |
| PLY-REQ-16 | Entregar recovery de processo e controle de energia — planned (M2) | REQ-E13-07 | TASK-E13-07 | — (PLY-AC-16 planejado) |
| PLY-REQ-17 | Entregar temas e plugins mobile — planned (M3) | REQ-E13-08 | TASK-E13-08 | — (PLY-AC-17 planejado) |
| PLY-REQ-18 | Entregar tablet/foldable e distribuição Android — planned (M3) | REQ-E13-09 | TASK-E13-09 | — (PLY-AC-18 planejado) |
| PLY-REQ-19 | Fechar aceite Android 1.0 em hardware — planned (M3) | REQ-E13-10 | TASK-E13-10 | — (PLY-AC-19 planejado) |
| PLY-REQ-20 | Provar envelope de ativação e protocolo comum — planned (M0) | REQ-E16-01 | TASK-E16-01 | — (PLY-AC-20 planejado) |
| PLY-REQ-21 | Implementar owner único por perfil e IPC autorizado — planned (M1) | REQ-E16-02 | TASK-E16-02 | — (PLY-AC-21 planejado) |
| PLY-REQ-22 | Entregar associação, menu de contexto e pacotes de ícones no Windows — planned (M2) | REQ-E16-03 | TASK-E16-03 | — (PLY-AC-22 planejado) |
| PLY-REQ-23 | Entregar MIME, extensões de shell e abertura externa no Linux — planned (M2) | REQ-E16-04 | TASK-E16-04 | — (PLY-AC-23 planejado) |
| PLY-REQ-24 | Entregar document types, Quick Actions e Finder nos dois Macs — planned (M2) | REQ-E16-05 | TASK-E16-05 | — (PLY-AC-24 planejado) |
| PLY-REQ-25 | Entregar VIEW e Share seguros no Android — planned (M2) | REQ-E16-06 | TASK-E16-06 | — (PLY-AC-25 planejado) |
| PLY-REQ-26 | Certificar ativação e associações em cinco alvos — planned (M3) | REQ-E16-07 | TASK-E16-07 | — (PLY-AC-26 planejado) |
| PLY-REQ-27 | Validar portable, backup e limpeza de integração — planned (M3) | REQ-E16-08 | TASK-E16-08 | — (PLY-AC-27 planejado) |
| PLY-REQ-28 | Gerar declarações OS a partir do build verificado — planned (M2) | REQ-E16-09 | TASK-E16-09 | — (PLY-AC-28 planejado) |
| PLY-REQ-29 | Entregar deep links e callbacks do provider — planned (M4) | REQ-E16-10 | TASK-E16-10 | — (PLY-AC-29 planejado) |
| PLY-REQ-30 | Provar iOS arm64, bridge Rust e CoreAudio — planned (F6) | REQ-E20-01 | TASK-E20-01 | — (PLY-AC-30 planejado) |
| PLY-REQ-31 | Entregar AVAudioSession e playback em background — planned (F6) | REQ-E20-02 | TASK-E20-02 | — (PLY-AC-31 planejado) |
| PLY-REQ-32 | Entregar shell SwiftUI touch-first e acessibilidade — planned (F6) | REQ-E20-03 | TASK-E20-03 | — (PLY-AC-32 planejado) |
| PLY-REQ-33 | Entregar acesso a arquivos e biblioteca sob sandbox — planned (F6) | REQ-E20-04 | TASK-E20-04 | — (PLY-AC-33 planejado) |
| PLY-REQ-34 | Adaptar DSP e equalização ao layout e ao orçamento iOS — planned (F6) | REQ-E20-05 | TASK-E20-05 | — (PLY-AC-34 planejado) |
| PLY-REQ-35 | Fechar assinatura, TestFlight e aceite iOS — planned (F6) | REQ-E20-06 | TASK-E20-06 | — (PLY-AC-35 planejado) |

Fonte de cada requisito: o documento citado na TASK (`delivery/01-roadmap.md`, `delivery/04-gates.md`, `design/05-mobile-tablet.md`, `library/02-tags-letras.md`, `platforms/01-matriz-suporte.md`, `platforms/02-android.md`, `platforms/03-integracao-sistema.md`, `platforms/04-ativacao-ownership.md`, `platforms/05-ios.md`, `quality/03-matriz-multiplataforma.md`). Fases presentes: F6, M0, M1, M2, M3, M4, M5.

## 5. Critérios de aceite

| ID | Given | When | Then |
|---|---|---|---|
| PLY-AC-01 | a entrega de TASK-E08-03 (Entregar análise de loudness, ReplayGain e BPM) | o teste de aceite da TASK roda | Loudness conferido contra fixtures; agrupamento correto; BPM inclui confiança; scan não interfere callback; write requer preview. |
| PLY-AC-02 | a entrega de TASK-E08-05 (Entregar sleep timer e scheduler) | o teste de aceite da TASK roda | Parar ao fim de faixa funciona; DST definido; desligar OS só se habilitado; alarmes respeitam teto de volume. |
| PLY-AC-03 | a entrega de TASK-E08-06 (Integrar controles do Windows) | o teste de aceite da TASK roda | Controlar sessão correta; close-to-tray explícito; sem admin/associação involuntária; detectar conflitos de hotkeys. |
| PLY-AC-04 | a entrega de TASK-E08-07 (Entregar CLI e IPC de instância única) | o teste de aceite da TASK roda | Segundo launch não executa comando arbitrário; Unicode preservado; CLI retorna exit codes; IPC não expõe filesystem geral. |
| PLY-AC-05 | a entrega de TASK-E08-08 (Entregar scrobblers e presence) | o teste de aceite da TASK roda | Sem requests antes de opt-in; tokens no keychain; retries idempotentes; revoke impede novos envios. |
| PLY-AC-06 | a entrega de TASK-E08-10 (Entregar controle remoto autenticado) | o teste de aceite da TASK roda | Cliente sem pairing não controla; nenhum endpoint de shell/filesystem; rotação e revogação de token funcionam. |
| PLY-AC-07 | a entrega de TASK-E12-01 (Entregar Linux nativo e MPRIS na 1.0) | o teste de aceite da TASK roda | Passar corpus comum e gates M1–M3 no Linux nativo; Wayland/X11 e portas/saída testados; não usar Wine como substituto; instalar pacote real. |
| PLY-AC-08 | a entrega de TASK-E12-02 (Entregar macOS Intel e Apple Silicon nativos na 1.0) | o teste de aceite da TASK roda | Fixtures, VoiceOver, device/rate changes e instalação passam em Mac Intel e Silicon reais; Universal2/helpers contêm ambas slices nativas nos DMG/ZIP Universal2 obrigatórios; sem exigir Rosetta. |
| PLY-AC-09 | a entrega de TASK-E12-09 (Entregar casting e companion) | o teste de aceite da TASK roda | Capacidade real do destino; limites de clock/transferência visíveis; negar cliente não pareado; sem claims hi-res não verificados. |
| PLY-AC-10 | a entrega de TASK-E13-01 (Provar Android arm64, bridge Rust e output) | o teste de aceite da TASK roda | Tocar WAV/FLAC em aparelho arm64 real; callback sem JNI/alloc; native build compatível com requisitos de páginas; comparar lifecycle e runtime antes congelar. |
| PLY-AC-11 | a entrega de TASK-E13-02 (Entregar serviço e MediaSession com engine Rust) | o teste de aceite da TASK roda | Activity não é owner do áudio; lockscreen/play/pause usam fila única Rust; cleanup idempotente; serviço segue restrições do SDK. |
| PLY-AC-12 | a entrega de TASK-E13-03 (Entregar audio focus e rotas Android) | o teste de aceite da TASK roda | Definir loss/transient/duck; unplug pausa sem tocar speaker; desconexão recupera fora da callback; taxa e sharing reais visíveis. |
| PLY-AC-13 | a entrega de TASK-E13-04 (Entregar MediaStore, SAF e permissões) | o teste de aceite da TASK roda | Deny/revoke preserva biblioteca e rádio; não exigir acesso irrestrito a arquivos; Content URI não vira path fictício; backup/limites de escrita explícitos. |
| PLY-AC-14 | a entrega de TASK-E13-05 (Entregar interface phone e navegação acessível) | o teste de aceite da TASK roda | Targets de 48 dp; fonte 200% faz reflow; nenhuma fila duplicada; back/rotation não reiniciam áudio; paleta e geometria phone conferidas. |
| PLY-AC-15 | a entrega de TASK-E13-06 (Integrar streaming, podcasts e downloads Android) | o teste de aceite da TASK roda | Cancel/retry e rádio screen-off funcionam; permissões, rede e jobs respeitados; não iniciar dois decoders produzindo áudio simultaneamente. |
| PLY-AC-16 | a entrega de TASK-E13-07 (Entregar recovery de processo e controle de energia) | o teste de aceite da TASK roda | Process death permite restore sem autostart proibido; rotation mantém sessão; screen-off 2 h sem underrun causado pelo app; wake locks liberados e visualizer desativado no background. |
| PLY-AC-17 | a entrega de TASK-E13-08 (Entregar temas e plugins mobile) | o teste de aceite da TASK roda | Fallback acessível; budgets e permissões funcionam; nenhum helper nativo externo; runtime e distribuição compatíveis documentados, sem promessa de JIT não provado. |
| PLY-AC-18 | a entrega de TASK-E13-09 (Entregar tablet/foldable e distribuição Android) | o teste de aceite da TASK roda | Phone/tablet/landscape/hinge/fonte passam; update preserva dados; target SDK e políticas revisados; aparelho arm64 real além do emulador. |
| PLY-AC-19 | a entrega de TASK-E13-10 (Fechar aceite Android 1.0 em hardware) | o teste de aceite da TASK roda | Gates comuns e Android aprovados no aparelho real; saída hi-res apenas quando negociada; flag AAudio não é prova de bit-perfect. |
| PLY-AC-20 | a entrega de TASK-E16-01 (Provar envelope de ativação e protocolo comum) | o teste de aceite da TASK roda | Cold/warm e lotes 1/2/20/100 preservam ordem; nomes especiais são dados; limites e ACK/generation definidos. |
| PLY-AC-21 | a entrega de TASK-E16-02 (Implementar owner único por perfil e IPC autorizado) | o teste de aceite da TASK roda | Vinte launches elegem um owner; installed/portable não escrevem o mesmo perfil simultaneamente; stale PID não libera lock; ACK e Conflict testados. |
| PLY-AC-22 | a entrega de TASK-E16-03 (Entregar associação, menu de contexto e pacotes de ícones no Windows) | o teste de aceite da TASK roda | Explorer moderno (Win11) e clássico (Win10) abrem lote ordenado por IShellItemArray; pacote esparso registrado via PackageManager; remoção limpa; suporte às 3 famílias de ícones. |
| PLY-AC-23 | a entrega de TASK-E16-04 (Entregar MIME, extensões de shell e abertura externa no Linux) | o teste de aceite da TASK roda | Gerenciadores (Dolphin, Nautilus, Thunar) exibem submenu Vynamp com verbos canônicos; %U repassa lote sem shell; ícones hicolor por família instalados. |
| PLY-AC-24 | a entrega de TASK-E16-05 (Entregar document types, Quick Actions e Finder nos dois Macs) | o teste de aceite da TASK roda | Intel e Silicon recebem cold/warm batches via application(_:openFiles:); Quick Actions no Finder ativas; pacotes de ícones aplicados na UI interna; defaults preservados. |
| PLY-AC-25 | a entrega de TASK-E16-06 (Entregar VIEW e Share seguros no Android) | o teste de aceite da TASK roda | VIEW/SEND/MULTIPLE e onNewIntent preservam grants/ordem; rotation não duplica; Share pede ação; sem persistência de grant não oferecido. |
| PLY-AC-26 | a entrega de TASK-E16-07 (Certificar ativação e associações em cinco alvos) | o teste de aceite da TASK roda | QA-022/023/024 passam nos cenários aplicáveis M1–M3 dos cinco alvos; deep links/OAuth só em M4; terceiros preservados; ausência de hardware não vira aprovação. |
| PLY-AC-27 | a entrega de TASK-E16-08 (Validar portable, backup e limpeza de integração) | o teste de aceite da TASK roda | Restore por staging e rollback preserva mídia; desinstalação remove pacote esparso e HKCU sem sobras; portable permite ativar/desativar integração com confirmação; update respeita owner. |
| PLY-AC-28 | a entrega de TASK-E16-09 (Gerar declarações OS a partir do build verificado) | o teste de aceite da TASK roda | planned/experimental/sem backend ou licença não gera handler; MIME genérico/video/BT não é associado; manifest e pacote têm mesmos hashes. |
| PLY-AC-29 | a entrega de TASK-E16-10 (Entregar deep links e callbacks do provider) | o teste de aceite da TASK roda | Links rejeitam path/shell/plugin/autoplay e callbacks falsos; não registram HTTP global; intent concedida não amplia filesystem grants. |
| PLY-AC-30 | a entrega de TASK-E20-01 (Provar iOS arm64, bridge Rust e CoreAudio) | o teste de aceite da TASK roda | artefato de simulador e de device são distinguidos no build; callback sem alocação nem travessia para o runtime Swift; WAV e FLAC tocam em aparelho arm64 real; comparar lifecycle e sessão de áudio antes de congelar a decisão. |
| PLY-AC-31 | a entrega de TASK-E20-02 (Entregar AVAudioSession e playback em background) | o teste de aceite da TASK roda | a sessão de áudio mantém o playback sem interrupção; interrupção pausa e retoma na posição em que estava; comandos remotos operam a fila única em Rust; a sessão não vaza ao terminar a reprodução. |
| PLY-AC-32 | a entrega de TASK-E20-03 (Entregar shell SwiftUI touch-first e acessibilidade) | o teste de aceite da TASK roda | controles de DSP e playlist acessíveis sem scroll horizontal; VoiceOver nomeia e opera transporte e DSP; Dynamic Type em 200% não esconde ação; rotação não reinicia áudio nem perde seleção. |
| PLY-AC-33 | a entrega de TASK-E20-04 (Entregar acesso a arquivos e biblioteca sob sandbox) | o teste de aceite da TASK roda | arquivo escolhido pelo usuário persiste no container; permissão revogada vira erro tipado, não crash; lista de 10 mil entradas rola sem carregar todas de uma vez; nenhuma leitura fora do container. |
| PLY-AC-34 | a entrega de TASK-E20-05 (Adaptar DSP e equalização ao layout e ao orçamento iOS) | o teste de aceite da TASK roda | presets respondem dentro de ±0.2 dB do desktop; nenhuma alocação na callback; cadeia completa não causa underrun no aparelho de referência; valor aplicado é o que a curva exibe. |
| PLY-AC-35 | a entrega de TASK-E20-06 (Fechar assinatura, TestFlight e aceite iOS) | o teste de aceite da TASK roda | build assinado instala por TestFlight em aparelho real; gates comuns aprovados no iOS; hardware ausente bloqueia o aceite em vez de aprová-lo; abre sem violação de sandbox. |

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
| Engine falhou ao abrir dispositivo de saída | `AppError::Output { reason }` (UI mostra, não fecha) |
| Integração do SO indisponível (sem MPRIS, SMTC negado) | `AppError::Integration { what }` (degrada sem quebrar) |
| Configuração corrompida | `AppError::Config { path }` (recria padrão e preserva cópia) |
| Ativação/licença inválida | `AppError::Activation { reason }` (ver `platforms/04-ativacao-ownership.md`) |

**Cancelamento é explícito, não timeout.** Fechar o app pede Stop ao engine e espera os workers terminarem com prazo; integração do SO é desfeita no encerramento.

## 7. Fora de escopo

Conversao em lote, restauracao por IA e servidor de streaming (E09): os dois primeiros sao apps separados, o terceiro nao tem repo

## 8. Dependências

Depende de: `vynamp-engine` (unico crate do projeto)

É dependido por: ninguem

## 9. Testes e fixtures

| Fixture | Origem | Uso |
|---|---|---|
| sessão D-Bus privada | subida pelo teste | MPRIS no Linux |
| emulador Android e simulador iOS | CI de plataforma | critérios mobile |
| matriz de `quality/03-matriz-multiplataforma.md` | umbrella | cobertura por SO |

Teste que depende de hardware, sessão gráfica ou SO específico vive atrás de
`#[ignore]` com o motivo escrito.

```
cargo test --test shell --locked
cargo test --test integracao_so --locked -- --ignored  # exige sessão do SO
```

## 10. Limitações conhecidas

Limitação declarada não reprova a entrega; limitação omitida reprova.

| Limitação | Efeito | Quem cobre |
|---|---|---|
| `src/` tem só `main.rs` com `println!`, zero dependências e nenhum `tests/` | nenhum critério verificável hoje | as TASKs de §4 |
| Critérios de Android, iOS, macOS e Windows exigem hardware ou VM da plataforma | atrás de `#[ignore]` fora do SO alvo | CI de plataforma + `evidence/` |
| iOS exige conta Apple, assinatura e TestFlight | E20-06 não fecha sem conta | mantenedor |
| Mobile é só player com DSP adaptado | converter, restore e plugins não existem em mobile | decisão E20-05 |
