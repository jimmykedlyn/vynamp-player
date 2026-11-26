//! Oráculo para TASK-E02-13: estilos criativos ("Estilos de onda", itens 2 e 4).
//!
//! Tradução literal de `drawCreative` de `waveform-styles.js` do MVP
//! (commit 0d9860b9, linhas 84 a 87), sem arredondamento: `aurora-weave`,
//! `neon-crystal`, `aurora-layers` e `aurora-constellation`.
//!
//! `None` é o `copyLiveSamples` que devolveu `false`. Cada amostra `f32` vira
//! `f64` antes da conta, como o `Float32Array` lido pelo JS.

use super::estilos_espectro::Comando;

/// O que `aurora-weave` publica: `data-level` e os `d` dos dois canais.
pub struct Weave {
    /// `data-level` de `#wave-creative-live`.
    pub nivel: f64,
    /// `d` de `#wave-creative-left`.
    pub esquerda: Vec<Comando>,
    /// `d` de `#wave-creative-right`.
    pub direita: Vec<Comando>,
}

/// `aurora-weave` — waveform-styles.js:85 e 86.
pub fn geometria_aurora_weave(
    esquerda: Option<&[f32]>,
    direita: Option<&[f32]>,
    ns: usize,
    amp: f64,
) -> Weave {
    unimplemented!(
        "fase 3c: aurora-weave ({}, {}, ns {ns}, amp {amp})",
        esquerda.is_some(),
        direita.is_some()
    )
}

/// O que `neon-crystal` e `aurora-layers` publicam.
pub struct Camadas {
    /// `data-level` de `#wave-creative-live`.
    pub nivel: f64,
    /// `d` de `#wave-creative-outer` (pico por segmento).
    pub externa: Vec<Comando>,
    /// `d` de `#wave-creative-middle` (RMS por segmento).
    pub media: Vec<Comando>,
    /// `d` de `#wave-creative-inner` (RMS, metade da altura arredondada).
    pub interna: Vec<Comando>,
}

/// `neon-crystal` e `aurora-layers` — waveform-styles.js:85 e 87.
pub fn geometria_camadas(
    amostras: Option<&[f32]>,
    segmentos: usize,
    ganho_pico: f64,
    ganho_rms: f64,
    altura_pico: f64,
    altura_rms: f64,
) -> Camadas {
    unimplemented!(
        "fase 3c: camadas ({}, {segmentos}, {ganho_pico}, {ganho_rms}, {altura_pico}, {altura_rms})",
        amostras.is_some()
    )
}

/// O que `aurora-constellation` publica em `#wave-creative-live`.
pub struct Constelacao {
    /// `data-level`.
    pub nivel: f64,
    /// `d` de `#wave-creative-nodes`.
    pub nos: Vec<Comando>,
    /// `d` de `#wave-creative-mesh`.
    pub malha: Vec<Comando>,
}

/// `aurora-constellation` — waveform-styles.js:84, sobre as 128 bandas de
/// `spectrumBands(0, dbRange)`.
pub fn geometria_constelacao(bandas: &[f64], nos: usize, raio: f64, h: f64) -> Constelacao {
    unimplemented!(
        "fase 3c: aurora-constellation ({} bandas, {nos} nós, raio {raio}, h {h})",
        bandas.len()
    )
}
