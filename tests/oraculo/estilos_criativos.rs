//! Oráculo para TASK-E02-13: estilos criativos ("Estilos de onda", itens 2 e 4).
//!
//! Tradução literal de `drawCreative` de `waveform-styles.js` do MVP
//! (commit 0d9860b9, linhas 84 a 87), sem arredondamento: `aurora-weave`,
//! `neon-crystal`, `aurora-layers` e `aurora-constellation`.
//!
//! `None` é o `copyLiveSamples` que devolveu `false`. Cada amostra `f32` vira
//! `f64` antes da conta, como o `Float32Array` lido pelo JS.

use super::estilos_espectro::{Comando, envelope};
use super::estilos_temporais::linha_amostrada;

/// Maior `|amostra|` do bloco lido, partindo de 0; 0 se `copyLiveSamples`
/// devolveu `false` (o `if(read)for(const v of ...)` de waveform-styles.js:85).
fn pico(amostras: Option<&[f32]>) -> f64 {
    amostras.map_or(0.0, |s| {
        s.iter().fold(0.0f64, |acc, &v| acc.max((v as f64).abs()))
    })
}

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
    Weave {
        nivel: pico(esquerda).max(pico(direita)),
        esquerda: linha_amostrada(esquerda, ns, amp),
        direita: linha_amostrada(direita, ns, amp),
    }
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
///
/// `step = 2048/count` sobre o bloco de 2048 amostras; o oráculo só cobre
/// `count` divisor de 2048 (os padrões 32 e 64), em que `step` é inteiro.
pub fn geometria_camadas(
    amostras: Option<&[f32]>,
    segmentos: usize,
    ganho_pico: f64,
    ganho_rms: f64,
    altura_pico: f64,
    altura_rms: f64,
) -> Camadas {
    assert!(
        segmentos > 0 && 2048 % segmentos == 0,
        "geometria_camadas: {segmentos} não divide 2048 (o MVP leria índice \
         fracionário e daria NaN); use um divisor de 2048, como 32 ou 64"
    );
    let step = 2048 / segmentos;
    let mut picos = Vec::with_capacity(segmentos);
    let mut rms = Vec::with_capacity(segmentos);
    for i in 0..segmentos {
        let (max, soma) = match amostras {
            Some(s) => {
                s[i * step..(i + 1) * step]
                    .iter()
                    .fold((0.0f64, 0.0f64), |(max, soma), &v| {
                        let v = v as f64;
                        (max.max(v.abs()), soma + v * v)
                    })
            }
            None => (0.0, 0.0),
        };
        picos.push(1.0f64.min(max * ganho_pico));
        rms.push(1.0f64.min((soma / step as f64).sqrt() * ganho_rms));
    }
    // `Math.round(rH/2)`: arredonda meio para cima, como `floor(x + 0.5)`.
    let altura_interna = (altura_rms / 2.0 + 0.5).floor();
    Camadas {
        nivel: pico(amostras),
        externa: envelope(&picos, altura_pico),
        media: envelope(&rms, altura_rms),
        interna: envelope(&rms, altura_interna),
    }
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
    let n = bandas.len();
    // values[i] = Math.max(...bands.slice(floor(i·n/nós), floor((i+1)·n/nós))).
    let valores: Vec<f64> = (0..nos)
        .map(|i| {
            bandas[i * n / nos..(i + 1) * n / nos]
                .iter()
                .fold(f64::NEG_INFINITY, |a, &b| a.max(b))
        })
        .collect();
    let nivel = valores.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let mut circulos = Vec::new();
    let mut superior = Vec::with_capacity(nos);
    let mut inferior = Vec::with_capacity(nos);
    for (i, &v) in valores.iter().enumerate() {
        let x = (i as f64 + 0.5) * 1000.0 / nos as f64;
        let hh = v * h;
        let ponto = |y: f64| {
            if i == 0 {
                Comando::M(x, y)
            } else {
                Comando::L(x, y)
            }
        };
        superior.push(ponto(24.0 - hh));
        inferior.push(ponto(24.0 + hh));
        // Só nó acima de 0,01 ganha os dois arcos que desenham o círculo.
        if v > 0.01 {
            for y in [24.0 - hh, 24.0 + hh] {
                circulos.push(Comando::M(x - raio, y));
                circulos.push(Comando::A(raio, raio, 0.0, 1.0, 0.0, raio * 2.0, 0.0));
                circulos.push(Comando::A(raio, raio, 0.0, 1.0, 0.0, -(raio * 2.0), 0.0));
            }
        }
    }
    superior.extend(inferior);
    Constelacao {
        nivel,
        nos: circulos,
        malha: superior,
    }
}
