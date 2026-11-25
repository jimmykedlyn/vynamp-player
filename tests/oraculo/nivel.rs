//! Cálculo de nível RMS → dB (item 7).

use super::sinal::{Tipo, sinal};

/// Limite inferior de recorte.
const DB_FLOOR: f64 = -60.0;

/// RMS das 2048 amostras que terminam em `amostra_fim` (exclusivo),
/// depois 20*log10(max(1e-8, rms)) recortado em [−60, 0].
pub fn nivel_db(tipo: Tipo, amostra_fim: u64) -> f64 {
    let inicio = amostra_fim.saturating_sub(2048);
    let mut soma: f64 = 0.0;
    let mut cont = 0u64;
    for n in inicio..amostra_fim {
        let s = sinal(tipo, n as i64) as f64;
        soma += s * s;
        cont += 1;
    }
    if cont == 0 {
        return DB_FLOOR;
    }
    let rms = (soma / cont as f64).sqrt();
    let db = 20.0 * rms.max(1e-8).log10();
    db.clamp(DB_FLOOR, 0.0)
}
