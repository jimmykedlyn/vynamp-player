//! AnalyserNode reimplementação da Web Audio API.
//! Registro em tests/oraculo/mod.rs: pub mod analisador;

use crate::oraculo::sinal::{Tipo, sinal};

/// Bloco de `n` amostras terminando em `amostra_fim` (exclusivo).
/// Índices `[amostra_fim - n, amostra_fim)`; índice negativo = 0 (pré-roll).
pub fn bloco(tipo: Tipo, amostra_fim: u64, n: usize) -> Vec<f32> {
    let start = amostra_fim.saturating_sub(n as u64);
    (start..amostra_fim)
        .map(|idx| sinal(tipo, idx as i64))
        .collect()
}

/// Janela Blackman da especificação Web Audio (AnalyserNode):
/// w(n) = a0 − a1·cos(2πn/N) + a2·cos(4πn/N), com denominador N, não N−1.
/// Com N, blackman(2048,1024) = a0 + a1 + a2 = 1 exatamente.
// α = 0,16, a0 = (1−α)/2 = 0.42, a1 = 0.5, a2 = α/2 = 0.08.
pub fn blackman(n: usize, i: usize) -> f64 {
    let alpha = 0.16f64;
    let a0 = (1.0 - alpha) / 2.0;
    let a1 = 0.5;
    let a2 = alpha / 2.0;
    // Denominador N (não N−1) conforme spec Web Audio AnalyserNode.
    // Isso garante blackman(2048, 1024) = a0 + a1 + a2 = 1.0 exatamente.
    let n_f64 = n as f64;
    let i_f64 = i as f64;
    a0 - a1 * (2.0 * std::f64::consts::PI * i_f64 / n_f64).cos()
        + a2 * (4.0 * std::f64::consts::PI * i_f64 / n_f64).cos()
}

/// Magnitudes da janela Blackman: |X[k]|/N para k em 0..N/2 (N=2048).
/// DFT direta sobre o bloco aplicando a janela Blackman.
/// O índice `k` é usado para gravar no vetor `mags[k]`; o loop por `for k in 0..N/2`
/// precisa do índice de destino, não só de um elemento, por isso o range loop é intencional.
#[allow(clippy::needless_range_loop)]
pub fn magnitudes(bloco: &[f32]) -> Vec<f64> {
    let n = bloco.len();
    let mut mags = vec![0.0f64; n / 2];
    for k in 0..(n / 2) {
        let mut re = 0.0f64;
        let mut im = 0.0f64;
        for j in 0..n {
            let w = blackman(n, j);
            let angle = 2.0 * std::f64::consts::PI * k as f64 * j as f64 / n as f64;
            let s = bloco[j] as f64;
            re += s * w * angle.cos();
            im -= s * w * angle.sin();
        }
        mags[k] = (re * re + im * im).sqrt() / n as f64;
    }
    mags
}

/// Analisador com suavização exponencial (Web Audio AnalyserNode).
/// Estado inicial: zero.
pub struct Analisador {
    fft_size: usize,
    smoothing: f64,
    estado: Vec<f64>,
}

impl Analisador {
    /// Cria analisador com fft_size e smoothing_time_constant.
    /// Estado inicial: zero.
    pub fn new(fft_size: usize, smoothing: f64) -> Self {
        Analisador {
            fft_size,
            smoothing,
            estado: vec![0.0f64; fft_size / 2],
        }
    }

    /// Lê o espectro em dB: X̂ = τ·X̂_prev + (1−τ)·|X|; devolve 20·log10(X̂).
    /// Pode dar -inf; o recorte é de quem consome.
    pub fn ler_espectro_db(&mut self, tipo: Tipo, amostra_fim: u64) -> Vec<f64> {
        let blk = bloco(tipo, amostra_fim, self.fft_size);
        let mags = magnitudes(&blk);
        // Suavização exponencial: X̂_new[k] = τ·X̂_prev[k] + (1−τ)·|X[k]|
        for (est, &m) in self.estado.iter_mut().zip(mags.iter()) {
            *est = self.smoothing * *est + (1.0 - self.smoothing) * m;
        }
        self.estado.iter().map(|x| 20.0 * x.log10()).collect()
    }
}

/// Frequências dos bins do medidor hi-fi spectrum (frequenciesHz do medidor 06-hifi-spectrum)
/// em assets.js do MVP (pin D:\Projects\vynamp-mvp-pin-e02-13\mvp\assets.js).
pub const FREQUENCIAS_HIFI: [f64; 24] = [
    20.0, 27.01, 36.47, 49.24, 66.49, 89.79, 121.24, 163.71, 221.06, 298.5, 403.07, 544.27, 734.93,
    992.39, 1340.04, 1809.47, 2443.35, 3299.3, 4455.09, 6015.77, 8123.17, 10968.83, 14811.37,
    20000.0,
];

/// Calcula os bins do espectro hi-fi para um dado tamanho de FFT.
/// Regra literal do MVP (player.js:44): para cada hz em FREQUENCIAS_HIFI,
/// min(bins_len - 1, max(1, round(hz / (48000/2) * (bins_len - 1)))).
/// Math.round do JS arredonda .5 para +∞.
fn js_round(x: f64) -> f64 {
    (x + 0.5).floor()
}

pub fn bins_hifi(bins_len: usize) -> Vec<usize> {
    let nyquist = 48000.0 / 2.0;
    let max_bin = bins_len - 1;
    FREQUENCIAS_HIFI
        .iter()
        .map(|&hz| {
            let val = js_round(hz / nyquist * max_bin as f64);
            let clamped = val.max(1.0).min(max_bin as f64);
            clamped as usize
        })
        .collect()
}

/// Lê os bins hi-fi do espectro, recortados em [−60, 0] dB.
pub fn espectro_hifi(an: &mut Analisador, tipo: Tipo, amostra_fim: u64) -> Vec<f64> {
    let db = an.ler_espectro_db(tipo, amostra_fim);
    let bins = bins_hifi(db.len());
    bins.iter().map(|&b| db[b].clamp(-60.0, 0.0)).collect()
}

/// Onda do osciloscópio: amostra [i*4] do bloco de 2048, para i em 0..=180.
pub fn onda_osc(tipo: Tipo, amostra_fim: u64) -> Vec<f32> {
    let blk = bloco(tipo, amostra_fim, 2048);
    (0..=180).map(|i| blk[i * 4]).collect()
}

// Gabarito matemático dos valores esperados pelo orquestrador (Node, float64):
// - blackman(2048,0) ≈ 0 (|v| < 1e-12)
// - blackman(2048,1024) = 1 ± 1e-12 (denominador N na Blackman Web Audio)
// - seno regime: bin 43 ≈ −20,0625 dB (smoothing 0), bin 43 ≈ −32,1037 dB (smoothing 0.75 1ª leitura)
// - seno smoothing 0.75 2ª leitura: −27,1414 dB (estado persiste)
// - silêncio: todos bins = −60 dB
// - bins_hifi(1024) começa 1,1,2,2,3,4,5,7,9,13,17,23,31,42,57
// - onda_osc(Seno, 48000): 181 valores; i=0,90,180 vs fórmula analítica f32(0.5·sin(2π·1000·n/48000))
