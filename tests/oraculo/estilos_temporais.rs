//! Oráculo para TASK-E02-13: estilos de leitura temporal ("Estilos de onda", item 4).
//!
//! Tradução literal de `waveform-styles.js` do MVP (commit 0d9860b9), sem
//! arredondamento: `pulse`, `bars-pulse` (inclusive o ramo de rádio),
//! `columns-pulse`, `prisma-neon`, `ribbon-pulse` e `aurora-neon`.
//!
//! `amostras` é o bloco de `copyLiveSamples` (2048 amostras); `None` é o
//! `available = false` do MVP (`copyLiveSamples` devolveu `false`). Cada
//! amostra `f32` vira `f64` antes da conta, como o `Float32Array` lido pelo JS.

use super::estilos_espectro::{Comando, envelope};

/// `Math.max(-1, Math.min(1, v))`.
fn recorta_unidade(v: f64) -> f64 {
    (-1.0f64).max(1.0f64.min(v))
}

/// Maior `|amostra|` de `amostras[de..ate]`, partindo de 0 (o laço `for j` do MVP).
fn pico(amostras: &[f32], de: usize, ate: usize) -> f64 {
    amostras[de..ate]
        .iter()
        .fold(0.0f64, |acc, &v| acc.max((v as f64).abs()))
}

/// Linha `M/L` de `ns` pontos com `x = i·1000/(ns−1)` e
/// `y = 24 − clamp(s[floor(i·len/ns)], −1, 1)·amp`; sem amostras, `v = 0`.
/// É a construção comum de `pulse` (waveform-styles.js:89) e `aurora-neon`
/// (waveform-styles.js:72).
fn linha_amostrada(amostras: Option<&[f32]>, ns: usize, amp: f64) -> Vec<Comando> {
    (0..ns)
        .map(|i| {
            let v = match amostras {
                Some(s) => recorta_unidade(s[i * s.len() / ns] as f64),
                None => 0.0,
            };
            let x = i as f64 * 1000.0 / (ns as f64 - 1.0);
            let y = 24.0 - v * amp;
            if i == 0 {
                Comando::M(x, y)
            } else {
                Comando::L(x, y)
            }
        })
        .collect()
}

/// `pulse` — waveform-styles.js:89 (ramo `selected==='pulse'` de `pulseFrame`).
/// Sem amostras, o MVP grava `M0 24H1000`.
pub fn geometria_pulse(amostras: Option<&[f32]>, ns: usize, amp: f64) -> Vec<Comando> {
    match amostras {
        Some(_) => linha_amostrada(amostras, ns, amp),
        None => vec![Comando::M(0.0, 24.0), Comando::H(1000.0)],
    }
}

/// `data-peak` de `#wave-live-styles` — waveform-styles.js:69 (`drawAdvanced`).
pub fn data_peak(amostras: Option<&[f32]>) -> f64 {
    match amostras {
        Some(s) => pico(s, 0, s.len()),
        None => 0.0,
    }
}

/// `#wave-live-columns` de `columns-pulse` e `prisma-neon` — waveform-styles.js:70.
pub fn geometria_colunas(amostras: Option<&[f32]>, cols: usize, g: f64, h: f64) -> Vec<Comando> {
    let mut d = Vec::with_capacity(cols * 2);
    for i in 0..cols {
        let valor = match amostras {
            Some(s) => pico(s, i * s.len() / cols, (i + 1) * s.len() / cols),
            None => 0.0,
        };
        let bh = 1.0f64.min(valor * g) * h;
        d.push(Comando::M(
            (i as f64 + 0.5) * 1000.0 / cols as f64,
            24.0 - bh,
        ));
        d.push(Comando::V(bh * 2.0));
    }
    d
}

/// `#wave-live-fill` de `ribbon-pulse` — waveform-styles.js:71.
pub fn geometria_ribbon(amostras: Option<&[f32]>, segs: usize, g: f64, h: f64) -> Vec<Comando> {
    let len = amostras.map_or(2048, <[f32]>::len);
    let step = len / segs;
    let valores: Vec<f64> = (0..segs)
        .map(|i| {
            let soma = match amostras {
                Some(s) => s[i * step..(i + 1) * step]
                    .iter()
                    .map(|&v| v as f64 * v as f64)
                    .sum(),
                None => 0.0,
            };
            1.0f64.min((soma / step as f64).sqrt() * g)
        })
        .collect();
    // `(values[i-1] ?? v) + 2v + (values[i+1] ?? v)) / 4`: fora do vetor vale `v`.
    let suave: Vec<f64> = (0..segs)
        .map(|i| {
            let v = valores[i];
            let antes = if i == 0 { v } else { valores[i - 1] };
            let depois = valores.get(i + 1).copied().unwrap_or(v);
            (antes + 2.0 * v + depois) / 4.0
        })
        .collect();
    envelope(&suave, h)
}

/// `#wave-live-line` e `#wave-live-halo` de `aurora-neon` — waveform-styles.js:72.
/// Devolve `(line, halo)`; o MVP grava o mesmo `d` nos dois.
pub fn geometria_aurora_neon(
    amostras: Option<&[f32]>,
    ns: usize,
    amp: f64,
) -> (Vec<Comando>, Vec<Comando>) {
    let d = linha_amostrada(amostras, ns, amp);
    (d.clone(), d)
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
        BarsPulse { nivel: 1.0 }
    }

    /// Uma leitura de `pulseFrame` em `bars-pulse` — waveform-styles.js:89.
    /// `target = sqrt(min(1, rms·5·g))` (0 sem amostras) e
    /// `nível += (target − nível)·(target > nível ? 1 − sm : sm)`.
    /// Devolve o novo `pulseLevel`.
    pub fn update(&mut self, amostras: Option<&[f32]>, g: f64, sm: f64) -> f64 {
        let alvo = match amostras {
            Some(s) => {
                let soma: f64 = s.iter().map(|&v| v as f64 * v as f64).sum();
                1.0f64.min((soma / s.len() as f64).sqrt() * 5.0 * g).sqrt()
            }
            None => 0.0,
        };
        let fator = if alvo > self.nivel { 1.0 - sm } else { sm };
        self.nivel += (alvo - self.nivel) * fator;
        self.nivel
    }
}

/// Ramo de rádio de `bars-pulse` — waveform-styles.js:89: o `d` único gravado
/// em cada filho de `#wave-reactive-bars`.
pub fn geometria_bars_radio(amostras: Option<&[f32]>, bars: usize, g: f64) -> Vec<Comando> {
    let mut d = Vec::with_capacity(bars * 2);
    for i in 0..bars {
        let p = match amostras {
            Some(s) => pico(s, i * s.len() / bars, (i + 1) * s.len() / bars),
            None => 0.0,
        };
        let h = 1.0f64.min(p * g) * 22.0;
        d.push(Comando::M(i as f64 * 1000.0 / bars as f64, 24.0 - h));
        d.push(Comando::V(h * 2.0));
    }
    d
}
