//! Sinal determinístico do protocolo de medição (item 3 e 8).

/// Tipo de sinal sintético.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tipo {
    /// Degrau: silêncio até 0,5 s, portadora de 0,5 s a 2,0 s.
    Degrau,
    /// Seno: portadora de 0 a 2,0 s.
    Seno,
    /// Rajada: ligada em [0, 50ms), [500, 550), [1000, 1050), [1500, 1550).
    Rajada,
}

/// Frequência da portadora em Hz. Spec: 1 kHz (REQ-E02-13).
const FREQ: f64 = 1000.0;
/// Taxa de amostragem em Hz.
const FS: f64 = 48000.0;

/// Gera a amostra `n` do sinal do tipo dado, com envelope aplicado.
///
/// `n < 0` (pré-roll) retorna 0.0.
pub fn sinal(tipo: Tipo, n: i64) -> f32 {
    if n < 0 {
        return 0.0;
    }
    let nn = n as f64;
    let theta = std::f64::consts::PI * 2.0 * FREQ * nn / FS;
    let portadora = 0.5f32 * theta.sin() as f32;
    let ligado = match tipo {
        Tipo::Degrau => (24000.0..2.0 * FS).contains(&nn),
        Tipo::Seno => (0.0..2.0 * FS).contains(&nn),
        Tipo::Rajada => {
            // 500 ms per cycle, 50 ms on within each cycle
            let ciclo = 500.0 / 1000.0 * FS; // = 24000 amostras = 500ms
            let dentro_ciclo = (nn % ciclo) < 50.0 / 1000.0 * FS; // 50ms = 2400 amostras
            nn < 2.0 * FS && dentro_ciclo
        }
    };
    if ligado { portadora } else { 0.0 }
}
