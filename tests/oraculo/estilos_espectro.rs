//! Oráculo para TASK-E02-13: estilos de espectro.
//! Tradução literal do JS do MVP para Rust, sem arredondamento.

/// Representação de um comando SVG `d` como letra + sequência de f64 não arredondados.
#[derive(Debug, Clone, PartialEq)]
pub struct Comando {
    pub letra: char,
    pub valores: Vec<f64>,
}

/// `envelope(values, height) -> Vec<Comando>`, idêntico ao JS.
/// Regra: amp(i) = clamp(values[i], 0, 1) * height; denominator = max(1, n-1).
/// Saída: M0 24 -> L forward -> L1000 24 -> L backward -> Z.
pub fn envelope(values: &[f64], height: f64) -> Vec<Comando> {
    let n = values.len();
    if n == 0 {
        return vec![Comando {
            letra: 'M',
            valores: vec![],
        }];
    }
    let denominator = (n - 1).max(1) as f64;
    let amp = |i: usize| clamp(values[i], 0.0, 1.0) * height;
    let mut comandos = Vec::new();
    // M0 24
    comandos.push(Comando {
        letra: 'M',
        valores: vec![0.0, 24.0],
    });
    // L forward: L (i*1000/denominator) (24-amp(i))
    for i in 0..n {
        comandos.push(Comando {
            letra: 'L',
            valores: vec![i as f64 * 1000.0 / denominator, 24.0 - amp(i)],
        });
    }
    // L1000 24
    comandos.push(Comando {
        letra: 'L',
        valores: vec![1000.0, 24.0],
    });
    // L backward: L (i*1000/denominator) (24+amp(i))
    for i in (0..n).rev() {
        comandos.push(Comando {
            letra: 'L',
            valores: vec![i as f64 * 1000.0 / denominator, 24.0 + amp(i)],
        });
    }
    // Z
    comandos.push(Comando {
        letra: 'Z',
        valores: vec![],
    });
    comandos
}

/// Helper clamp f64
fn clamp(v: f64, a: f64, b: f64) -> f64 {
    a.max(b.min(v))
}

/// `spectrum_bands(db: &[f64], audivel: bool, db_range: f64) -> (Vec<f64>, f64)`:
/// as 128 bandas e a frequência de pico. Regra literal da :76.
#[allow(clippy::manual_clamp, dead_code)]
pub fn spectrum_bands(db: &[f64], audivel: bool, db_range: f64) -> (Vec<f64>, f64) {
    if !audivel {
        return (vec![0.0f64; 128], 0.0);
    }

    let len = db.len();
    let nyquist = 24000.0_f64; // 48000/2
    let max_db = db[1..]
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, |a, b| a.max(b));
    let peak_bin = db[1..]
        .iter()
        .enumerate()
        .find(|&(_, &v)| v == max_db)
        .map(|(i, _)| i + 1)
        .unwrap_or(0);

    let freq = if max_db > -90.0 {
        peak_bin as f64 * nyquist / len as f64
    } else {
        0.0
    };

    let max_val = 20000.0_f64.min(nyquist);
    let mut bands = Vec::with_capacity(128);
    for i in 0..128 {
        let low = 40.0 * (max_val / 40.0).powf(i as f64 / 128.0);
        let high = 40.0 * (max_val / 40.0).powf((i + 1) as f64 / 128.0);
        let a = (low / nyquist * len as f64).max(1.0).floor() as usize;
        let b = {
            let upper = ceil_to_usize(high / nyquist * len as f64);
            len.max(a + 1).min(upper)
        };
        let mut db_val = f64::NEG_INFINITY;
        #[allow(clippy::needless_range_loop)]
        for j in a..b {
            if j < len {
                db_val = db_val.max(db[j]);
            }
        }
        let val = if db_val.is_infinite() || db_val.is_nan() {
            0.0
        } else {
            ((db_val + db_range) / db_range).max(0.0).min(1.0)
        };
        bands.push(val);
    }
    (bands, freq)
}

/// Helper ceil f64 para usize
fn ceil_to_usize(x: f64) -> usize {
    let ix = x as usize;
    if x > ix as f64 { ix + 1 } else { ix }
}

/// `SpectrumPulse`, estrutura com barras, picos e último instante.
#[derive(Debug)]
#[allow(dead_code)]
pub struct SpectrumPulse {
    pub bars: Vec<f64>,
    pub peaks: Vec<f64>,
    pub last: u64,
}

impl SpectrumPulse {
    /// Cria novo SpectrumPulse com `count` barras.
    #[allow(dead_code)]
    pub fn new(count: usize) -> Self {
        Self {
            bars: vec![0.0; count],
            peaks: vec![0.0; count],
            last: 0,
        }
    }

    /// `update(bins_db: Option<&[f64]>, rate, count, now) -> &Self`
    #[allow(
        dead_code,
        unused_variables,
        clippy::manual_clamp,
        clippy::needless_range_loop
    )]
    pub fn update(
        &mut self,
        #[allow(unused_variables)] bins_db: Option<&[f64]>,
        #[allow(unused_variables)] rate: u64,
        count: usize,
        #[allow(unused_variables)] now: u64,
    ) -> &Self {
        let dt = if self.last != 0 {
            f64::min(250.0, (now - self.last) as f64)
        } else {
            16.0
        };
        self.last = now;

        for i in 0..count {
            let mut level = 0.0f64;
            if let Some(bins) = bins_db {
                let from = (i as f64 / count as f64).floor() as usize;
                let to = {
                    let frac = (i + 1) as f64 / count as f64;
                    let ceil_val = ceil_to_usize(frac * count as f64);
                    count.saturating_sub(1).max(i).min(ceil_val)
                };
                let mut db_val = f64::NEG_INFINITY;
                for k in from..to.min(bins.len()) {
                    db_val = db_val.max(bins[k]);
                }
                level = if db_val.is_finite() {
                    // clamp: ((db_val + 90.0) / 80.0).max(0.0).min(1.0)
                    ((db_val + 90.0) / 80.0).clamp(0.0, 1.0)
                } else {
                    0.0
                };
            }
            let decay = (-dt / 70.0).exp();
            self.bars[i] = if level > self.bars[i] {
                level
            } else {
                level + (self.bars[i] - level) * decay
            };
            self.peaks[i] = self.bars[i].max(self.peaks[i] - dt * 0.0008);
        }
        self
    }

    /// Merge máximo bin a bin entre left e right.
    #[allow(dead_code)]
    pub fn merge<'a>(left: &[f64], right: &[f64], target: &'a mut [f64]) -> &'a [f64] {
        for i in 0..target.len() {
            target[i] = left[i].max(right[i]);
        }
        target
    }
}

/// Geometria de spectrum-aurora, spectrum-ribbon e spectrum-stereo.
#[allow(dead_code)]
pub fn geometria_spectrum(
    _estilo: &str,
    _left: &[f64],
    _right: &[f64],
    _bands: usize,
    _h: f64,
) -> Vec<(String, String)> {
    // TODO: implementar conforme :80 da spec
    vec![]
}

/// Geometria de spectrum-pulse.
#[allow(dead_code)]
pub fn geometria_spectrum_pulse(_state: &SpectrumPulse, _h: f64) -> Vec<(String, String)> {
    // TODO: implementar conforme :79 da spec
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envelope_basico() {
        let result = envelope(&[0.5, 1.5], 20.0);
        assert_eq!(result.len(), 7);
        // M0 24
        assert_eq!(result[0].letra, 'M');
        assert_eq!(result[0].valores, vec![0.0, 24.0]);
        // L0 14
        assert_eq!(result[1].letra, 'L');
        assert_eq!(result[1].valores, vec![0.0, 14.0]);
        // L1000 4
        assert_eq!(result[2].letra, 'L');
        assert_eq!(result[2].valores, vec![1000.0, 4.0]);
        // L1000 24
        assert_eq!(result[3].letra, 'L');
        assert_eq!(result[3].valores, vec![1000.0, 24.0]);
        // L1000 44
        assert_eq!(result[4].letra, 'L');
        assert_eq!(result[4].valores, vec![1000.0, 44.0]);
        // L0 34
        assert_eq!(result[5].letra, 'L');
        assert_eq!(result[5].valores, vec![0.0, 34.0]);
        // Z
        assert_eq!(result[6].letra, 'Z');
        assert_eq!(result[6].valores, vec![]);
    }

    #[test]
    fn envelope_vazio() {
        let result = envelope(&[], 20.0);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].letra, 'M');
    }

    #[test]
    fn spectrum_bands_audivel_falso() {
        let (bands, freq) = spectrum_bands(&[0.0f64; 1024], false, 84.0);
        assert_eq!(bands.len(), 128);
        assert!(bands.iter().all(|&v| v == 0.0));
        assert_eq!(freq, 0.0);
    }

    #[test]
    fn spectrum_bands_tudo_zero_audivel() {
        // audivel=true com todos bins = 0: maxDB = 0 > -90, freq computado.
        // peakBin = 1, freq = 1 * 24000 / 1024 = 23.4375
        // Bands: db_val = 0 para cada banda, val = (0 + 84) / 84 = 1.0 após clamp
        let (bands, freq) = spectrum_bands(&[0.0f64; 1024], true, 84.0);
        assert_eq!(bands.len(), 128);
        assert!(
            (freq - 23.4375).abs() < 1e-10,
            "freq = {}, expected ≈23.4375",
            freq
        );
        // Todas as bandas valem 1.0 quando todos os bins são 0 e audivel=true
        assert!(
            bands.iter().all(|&v| (v - 1.0).abs() < 1e-10),
            "bands = {:?}",
            bands
        );
    }
}
