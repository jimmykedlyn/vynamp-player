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

/// Gera a amostra `n` do sinal do tipo dado, com envelope aplicado.
///
/// `n < 0` (pré-roll) retorna 0.0.
pub fn sinal(_tipo: Tipo, _n: i64) -> f32 {
    0.0
}