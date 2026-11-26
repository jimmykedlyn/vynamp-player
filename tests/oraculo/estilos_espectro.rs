//! Oráculo para TASK-E02-13: estilos de espectro.
//! Tradução literal do JS do MVP para Rust, sem arredondamento.

/// Um comando de path SVG: letra + valores f64 não arredondados.
#[derive(Debug, Clone, PartialEq)]
pub enum Comando {
    /// `M(x,y)` — move absoluto.
    M(f64, f64),
    /// `L(x,y)` — linha absoluta.
    L(f64, f64),
    /// `v(dy)` — linha vertical relativa.
    V(f64),
    /// `h(dx)` — linha horizontal relativa.
    H(f64),
    /// `Z` — fecha o path.
    Z,
    /// `a(rx ry rotação arco-grande varredura dx dy)` — arco relativo, usado
    /// nos nós de `aurora-constellation` (waveform-styles.js:84).
    A(f64, f64, f64, f64, f64, f64, f64),
}

/// `clamp(v, 0, 1)` — limita ao intervalo [0, 1].
fn clamp01(v: f64) -> f64 {
    0.0f64.max(1.0f64.min(v))
}

/// `envelope(values, height)` — waveform-styles.js linha 64.
/// Gera o path SVG do envelope: M0 24, L forward, L1000 24, L backward, Z.
pub fn envelope(values: &[f64], height: f64) -> Vec<Comando> {
    let n = values.len();
    if n == 0 {
        return vec![Comando::M(0.0, 24.0), Comando::Z];
    }
    let denominator = (n - 1).max(1) as f64;
    // amp(i) = clamp(values[i], 0, 1) * height
    let amp = |i: usize| clamp01(values[i]) * height;

    // M0 24
    let mut comandos = vec![Comando::M(0.0, 24.0)];

    // L forward: L (i*1000/denominator) (24-amp(i))
    for i in 0..n {
        comandos.push(Comando::L(i as f64 * 1000.0 / denominator, 24.0 - amp(i)));
    }
    // L1000 24
    comandos.push(Comando::L(1000.0, 24.0));

    // L backward: L (i*1000/denominator) (24+amp(i))
    for i in (0..n).rev() {
        comandos.push(Comando::L(i as f64 * 1000.0 / denominator, 24.0 + amp(i)));
    }
    // Z
    comandos.push(Comando::Z);

    comandos
}

/// `audivel(bloco)` — waveform-styles.js linha 76 (`some(v => Math.abs(v) > 1e-6)`).
pub fn audivel(bloco: &[f32]) -> bool {
    bloco.iter().any(|v| v.abs() > 1e-6)
}

/// `limites_bandas(nyquist, len)` — waveform-styles.js linha 76.
/// Devolve 128 pares (a, b) para as bandas de frequência.
///
/// Fórmula: max = min(20000, nyquist)
///          low(i) = 40 * (max/40)^(i/128)
///          high(i+1) = 40 * (max/40)^((i+1)/128)
///          a = max(1, floor(low/nyq * len))
///          b = min(len, max(a+1, ceil(high/nyq * len)))
pub fn limites_bandas(nyquist: f64, len: usize) -> Vec<(usize, usize)> {
    let max = 20000.0_f64.min(nyquist);
    let mut bandas = Vec::with_capacity(128);
    for i in 0..128 {
        let low = 40.0 * (max / 40.0).powf(i as f64 / 128.0);
        let high = 40.0 * (max / 40.0).powf((i + 1) as f64 / 128.0);
        let a = (low / nyquist * len as f64).max(1.0).floor() as usize;
        let b = {
            let upper = ((high / nyquist) * len as f64).ceil() as usize;
            len.max(a + 1).min(upper)
        };
        bandas.push((a, b));
    }
    bandas
}

/// `spectrum_bands(db, disponivel, nyquist, db_range)` — waveform-styles.js linha 76.
/// Devolve (128 valores de banda, frequência de pico).
pub fn spectrum_bands(
    db: &[f64],
    disponivel: bool,
    nyquist: f64,
    db_range: f64,
) -> (Vec<f64>, f64) {
    let len = db.len();
    // maxDB e peakBin a partir de j = 1, com > estrito
    let mut max_db = f64::NEG_INFINITY;
    let mut peak_bin = 0usize;
    // O laço indexado é necessário porque precisamos do índice j para peak_bin.
    for (j, &val) in db.iter().enumerate().take(len).skip(1) {
        if val > max_db {
            max_db = val;
            peak_bin = j;
        }
    }

    let freq = if disponivel && max_db > -90.0 {
        (peak_bin as f64 * nyquist / len as f64)
            .max(0.0)
            .min(nyquist)
    } else {
        0.0
    };

    let mut bandas = Vec::with_capacity(128);
    let max_val = 20000.0_f64.min(nyquist);
    for i in 0..128 {
        let low = 40.0 * (max_val / 40.0).powf(i as f64 / 128.0);
        let high = 40.0 * (max_val / 40.0).powf((i + 1) as f64 / 128.0);
        let a = (low / nyquist * len as f64).max(1.0).floor() as usize;
        let b = {
            let upper = ((high / nyquist) * len as f64).ceil() as usize;
            len.max(a + 1).min(upper)
        };
        // máximo dB em [a, b)
        let mut db_val = f64::NEG_INFINITY;
        let db_slice = &db[a..b.min(len)];
        for &val in db_slice.iter() {
            if val > db_val {
                db_val = val;
            }
        }
        let val = if disponivel {
            ((db_val + db_range) / db_range).clamp(0.0, 1.0)
        } else {
            0.0
        };
        bandas.push(val);
    }
    (bandas, freq)
}

/// `SpectrumPulse` — spectrum-pulse.js.
/// Estrutura de estado do pulso espectral.
pub struct SpectrumPulse {
    /// Valores atuais das barras (0..1).
    pub bars: Vec<f64>,
    /// Valores de pico das barras (0..1).
    pub peaks: Vec<f64>,
    /// Timestamp da última atualização (ms).
    pub last: f64,
}

impl SpectrumPulse {
    /// Cria um novo `SpectrumPulse` com `count` barras, tudo zero.
    pub fn new(count: usize) -> Self {
        SpectrumPulse {
            bars: vec![0.0f64; count],
            peaks: vec![0.0f64; count],
            last: 0.0,
        }
    }

    /// `limites_pulse(rate, count, len)` — spectrum-pulse.js linha 11.
    /// Devolve `count` pares (from, to) para as bandas log.
    ///
    /// Fórmula: low = 20, top = min(20000, rate/2)
    ///          from = floor(low * (top/low)^(i/count) / (rate/2) * len), sem max(1)
    ///          to = max(from+1, ceil(low * (top/low)^((i+1)/count) / (rate/2) * len))
    pub fn limites_pulse(rate: f64, count: usize, len: usize) -> Vec<(usize, usize)> {
        let top = 20000.0_f64.min(rate / 2.0);
        let mut faixas = Vec::with_capacity(count);
        for i in 0..count {
            let from = (20.0 * (top / 20.0).powf(i as f64 / count as f64) / (rate / 2.0)
                * len as f64)
                .floor() as usize;
            let to = {
                let ceil_val = (20.0 * (top / 20.0).powf((i + 1) as f64 / count as f64)
                    / (rate / 2.0)
                    * len as f64)
                    .ceil() as usize;
                std::cmp::max(from + 1, ceil_val)
            };
            faixas.push((from, to));
        }
        faixas
    }

    /// `update(bins, rate, count, now)` — spectrum-pulse.js linha 11–14.
    /// Atualiza o estado do pulso espectral.
    pub fn update(&mut self, bins: Option<&[f64]>, rate: f64, count: usize, now: f64) {
        let count = std::cmp::min(count, self.bars.len());
        let dt = if self.last != 0.0 {
            f64::min(250.0, now - self.last)
        } else {
            16.0
        };
        self.last = now;
        let keep = (-dt / 70.0).exp();
        let drop = dt * 0.0008;

        let top = 20000.0_f64.min(rate / 2.0);
        let bins_len = bins.map(|b| b.len()).unwrap_or(0);
        for i in 0..count {
            let from = (20.0 * (top / 20.0).powf(i as f64 / count as f64) / (rate / 2.0)
                * bins_len as f64)
                .floor() as usize;
            let to = {
                let ceil_val = (20.0 * (top / 20.0).powf((i + 1) as f64 / count as f64)
                    / (rate / 2.0)
                    * bins_len as f64)
                    .ceil() as usize;
                std::cmp::max(from + 1, ceil_val)
            };

            let mut db = f64::NEG_INFINITY;
            if let Some(bins_slice) = bins {
                let start = from.min(bins_slice.len());
                let end = to.min(bins_slice.len());
                for &val in &bins_slice[start..end] {
                    if val > db {
                        db = val;
                    }
                }
            }

            let level = if db.is_finite() {
                ((db + 90.0) / 80.0).clamp(0.0, 1.0)
            } else {
                0.0
            };

            // bars[i] = if level > bars { level } else { level + (bars - level)*keep }
            if level > self.bars[i] {
                self.bars[i] = level;
            } else {
                self.bars[i] = level + (self.bars[i] - level) * keep;
            }

            // peaks[i] = max(bars, peaks - drop)
            if self.bars[i] > self.peaks[i] - drop {
                self.peaks[i] = self.bars[i];
            } else {
                self.peaks[i] -= drop;
            }
        }
    }
}

/// `merge(l, r)` — spectrum-pulse.js linha 22.
/// Devolve o máximo bin a bin entre `l` e `r`.
pub fn merge(l: &[f64], r: &[f64]) -> Vec<f64> {
    let n = std::cmp::max(l.len(), r.len());
    let mut result = vec![0.0f64; n];
    for (i, item) in result.iter_mut().enumerate().take(n) {
        let lv = l.get(i).copied().unwrap_or(0.0);
        let rv = r.get(i).copied().unwrap_or(0.0);
        *item = lv.max(rv);
    }
    result
}

/// `geometria_spectrum(estilo, left, right, h)` — waveform-styles.js linha 80.
/// Devolve os paths SVG por estilo e o data_level.
pub fn geometria_spectrum(
    estilo: &str,
    left: &[f64],
    right: &[f64],
    h: f64,
) -> (Vec<(&'static str, Vec<Comando>)>, f64) {
    // `Math.max(...left, ...right)`, sem valor absoluto, como no MVP.
    let data_level = left
        .iter()
        .chain(right.iter())
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let mut paths = Vec::new();

    match estilo {
        "spectrum-stereo" | "spectrum-aurora" => {
            // top: M (i+.5)*1000/bands 24, v-left*h; bottom: M (i+.5)*1000/bands 24, v right*h
            let bands = left.len();
            let mut top = Vec::new();
            let mut bottom = Vec::new();

            for (i, &v) in left.iter().enumerate() {
                let x = (i as f64 + 0.5) * 1000.0 / bands as f64;
                top.push(Comando::M(x, 24.0));
                top.push(Comando::V(-v * h));
            }
            for (i, &v) in right.iter().enumerate() {
                let x = (i as f64 + 0.5) * 1000.0 / bands as f64;
                bottom.push(Comando::M(x, 24.0));
                bottom.push(Comando::V(v * h));
            }

            if estilo == "spectrum-stereo" {
                paths.push(("wave-spectrum-left", top));
                paths.push(("wave-spectrum-right", bottom));
            } else {
                // spectrum-aurora: left e halo recebem top+bottom.
                let mut top_bottom = top;
                top_bottom.extend(bottom);
                paths.push(("wave-spectrum-left", top_bottom.clone()));
                paths.push(("wave-spectrum-halo", top_bottom));
            }
        }
        "spectrum-ribbon" => {
            // ribbon: envelope da suavização 1-2-1 de left (borders: left[i-1] ?? v)
            // JS: smooth[i] = (left[i-1] ?? v + 2*v + left[i+1] ?? v) / 4
            let mut smoothed = Vec::new();
            let n = left.len();
            for (i, &v) in left.iter().enumerate() {
                let l = if i == 0 { v } else { left[i - 1] };
                let r = if i == n - 1 { v } else { left[i + 1] };
                let s = (l + 2.0 * v + r) / 4.0;
                smoothed.push(s);
            }
            let envelope_cmds = envelope(&smoothed, h);
            paths.push(("wave-spectrum-fill", envelope_cmds));
        }
        // O drawSpectrum do MVP só desenha estes três estilos (spectrum-pulse tem
        // geometria_spectrum_pulse); outro estilo aqui é erro do chamador no teste.
        outro => unreachable!(
            "geometria_spectrum recebeu o estilo {outro:?}; só spectrum-stereo, \
             spectrum-aurora e spectrum-ribbon passam por drawSpectrum"
        ),
    }

    (paths, data_level)
}

/// `geometria_spectrum_pulse(&SpectrumPulse, h)` — waveform-styles.js linha 79.
/// Devolve (fill, halo, data_level).
pub fn geometria_spectrum_pulse(
    pulse: &SpectrumPulse,
    h: f64,
) -> (Vec<Comando>, Vec<Comando>, f64) {
    let count = pulse.bars.len();
    let step = 1000.0 / count as f64;
    let w = step - 3.0;

    let mut fill = Vec::new();
    let mut halo = Vec::new();

    for i in 0..count {
        let x = i as f64 * step + 1.5;
        let bh = pulse.bars[i] * h;

        // barra só se bh = bars*h > 0.2
        if bh > 0.2 {
            fill.push(Comando::M(x, 46.0));
            fill.push(Comando::V(-bh));
            fill.push(Comando::H(w));
            fill.push(Comando::V(bh));
            fill.push(Comando::Z);
        }

        // pico só se peaks > 0.02
        if pulse.peaks[i] > 0.02 {
            halo.push(Comando::M(x, 44.0 - pulse.peaks[i] * h));
            halo.push(Comando::H(w));
            halo.push(Comando::V(1.6));
            halo.push(Comando::H(-w));
            halo.push(Comando::Z);
        }
    }

    // vazio: M0 46 H1000
    if fill.is_empty() {
        fill = vec![Comando::M(0.0, 46.0), Comando::H(1000.0)];
    }
    if halo.is_empty() {
        halo = vec![Comando::M(0.0, 46.0), Comando::H(1000.0)];
    }

    let data_level = pulse
        .bars
        .iter()
        .cloned()
        .fold(0.0f64, |a, b| a.max(b.abs()));

    (fill, halo, data_level)
}
