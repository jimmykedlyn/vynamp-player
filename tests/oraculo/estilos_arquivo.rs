//! Oráculo para TASK-E02-13: estilos de envelope do arquivo (AC-3).
//!
//! `original`, `dj` e `overview` desenham o envelope do arquivo em `render`
//! (waveform-styles.js:93, commit 0d9860b9) e não respondem ao áudio ao vivo.
//! O que o código fixa fora de `defaultConfigs`: `peakHeight` 22 e
//! `bodyHeight` 20 por padrão, 15 e 7 em `overview`.

use super::estilos_espectro::{Comando, envelope};

/// Estilo de onda que desenha o envelope do arquivo, sem série ao vivo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EstiloArquivo {
    /// `original`.
    Original,
    /// `dj`.
    Dj,
    /// `overview`.
    Overview,
}

/// `(pH, bH)` de `render` — waveform-styles.js:93:
/// `cfg.peakHeight ?? (overview ? 15 : 22)` e `cfg.bodyHeight ?? (overview ? 7 : 20)`,
/// com `cfg` sem essas chaves em `defaultConfigs`.
pub fn alturas(estilo: EstiloArquivo) -> (f64, f64) {
    match estilo {
        EstiloArquivo::Original | EstiloArquivo::Dj => (22.0, 20.0),
        EstiloArquivo::Overview => (15.0, 7.0),
    }
}

/// `d` de `.wave-peaks` e de `.wave-body` — waveform-styles.js:93:
/// `envelope(data.peaks, pH)` e `envelope(data.rms, bH)`.
pub fn geometria_arquivo(
    estilo: EstiloArquivo,
    picos: &[f64],
    rms: &[f64],
) -> (Vec<Comando>, Vec<Comando>) {
    let (altura_pico, altura_corpo) = alturas(estilo);
    (envelope(picos, altura_pico), envelope(rms, altura_corpo))
}
