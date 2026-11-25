//! Grade temporal e cadência de quadros (itens 5 e 6).

/// Instante t_k em ms, com a ordem de operações da spec.
pub fn t_k(k: u64) -> f64 {
    k as f64 * 1000.0 / 60.0
}

/// Instante de suspensão em amostras: ceil(t_k*48/128)*128.
pub fn amostra_suspensao(k: u64) -> u64 {
    0
}

/// Quadros em que o medidor lê (cadência 1000/30 ms), lastMeter começa em 0.
pub fn quadros_medidor() -> Vec<u64> {
    vec![]
}

/// Quadros em que o pulse de 33 ms lê, lastPulse começa em 0.
pub fn quadros_pulse_33() -> Vec<u64> {
    vec![]
}

/// Todos os quadros de 0 a 119 (spectrum-pulse).
pub fn quadros_spectrum_pulse() -> Vec<u64> {
    vec![]
}