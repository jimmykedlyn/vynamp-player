//! Oráculo para TASK-E02-13: estilos de leitura temporal ("Estilos de onda", item 4).
//!
//! Tradução literal de `waveform-styles.js` do MVP (commit 0d9860b9), sem
//! arredondamento: `pulse`, `bars-pulse` (inclusive o ramo de rádio),
//! `columns-pulse`, `prisma-neon`, `ribbon-pulse` e `aurora-neon`.
//!
//! `amostras` é o bloco de `copyLiveSamples` (2048 amostras); `None` é o
//! `available = false` do MVP (`copyLiveSamples` devolveu `false`).

use super::estilos_espectro::Comando;

/// `pulse` — waveform-styles.js:89 (ramo `selected==='pulse'` de `pulseFrame`).
/// Sem amostras, o MVP grava `M0 24H1000`.
pub fn geometria_pulse(amostras: Option<&[f32]>, ns: usize, amp: f64) -> Vec<Comando> {
    let _ = (amostras, ns, amp);
    unimplemented!("geometria_pulse ainda não implementada (fase 3b, vermelho)")
}

/// `data-peak` de `#wave-live-styles` — waveform-styles.js:69 (`drawAdvanced`).
pub fn data_peak(amostras: Option<&[f32]>) -> f64 {
    let _ = amostras;
    unimplemented!("data_peak ainda não implementado (fase 3b, vermelho)")
}

/// `#wave-live-columns` de `columns-pulse` e `prisma-neon` — waveform-styles.js:70.
pub fn geometria_colunas(amostras: Option<&[f32]>, cols: usize, g: f64, h: f64) -> Vec<Comando> {
    let _ = (amostras, cols, g, h);
    unimplemented!("geometria_colunas ainda não implementada (fase 3b, vermelho)")
}

/// `#wave-live-fill` de `ribbon-pulse` — waveform-styles.js:71.
pub fn geometria_ribbon(amostras: Option<&[f32]>, segs: usize, g: f64, h: f64) -> Vec<Comando> {
    let _ = (amostras, segs, g, h);
    unimplemented!("geometria_ribbon ainda não implementada (fase 3b, vermelho)")
}

/// `#wave-live-line` e `#wave-live-halo` de `aurora-neon` — waveform-styles.js:72.
/// Devolve `(line, halo)`; o MVP grava o mesmo `d` nos dois.
pub fn geometria_aurora_neon(
    amostras: Option<&[f32]>,
    ns: usize,
    amp: f64,
) -> (Vec<Comando>, Vec<Comando>) {
    let _ = (amostras, ns, amp);
    unimplemented!("geometria_aurora_neon ainda não implementada (fase 3b, vermelho)")
}

/// Estado de `bars-pulse`: o `pulseLevel` de waveform-styles.js:65, que começa
/// em 1 (`scaleBars(1)`).
pub struct BarsPulse {
    /// `pulseLevel`: publicado em `data-level` (`toFixed(3)`) e no `scale` do
    /// `transform` (`toFixed(4)`) de `#wave-reactive-bars`.
    pub nivel: f64,
}

impl BarsPulse {
    /// Estado inicial do MVP: `pulseLevel = 1`.
    pub fn new() -> Self {
        unimplemented!("BarsPulse::new ainda não implementado (fase 3b, vermelho)")
    }

    /// Uma leitura de `pulseFrame` em `bars-pulse` — waveform-styles.js:89.
    /// Devolve o novo `pulseLevel`.
    pub fn update(&mut self, amostras: Option<&[f32]>, g: f64, sm: f64) -> f64 {
        let _ = (amostras, g, sm);
        unimplemented!("BarsPulse::update ainda não implementado (fase 3b, vermelho)")
    }
}

/// Ramo de rádio de `bars-pulse` — waveform-styles.js:89: o `d` único gravado
/// em cada filho de `#wave-reactive-bars`.
pub fn geometria_bars_radio(amostras: Option<&[f32]>, bars: usize, g: f64) -> Vec<Comando> {
    let _ = (amostras, bars, g);
    unimplemented!("geometria_bars_radio ainda não implementada (fase 3b, vermelho)")
}
