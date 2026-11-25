//! Gabarito de movimento: teste TASK-E02-13 AC-2.
//!
//! Teste headless que usa o oráculo de sinal, grade temporal e nível
//! para conferir o comportamento dos medidores e ondas do MVP.

mod oraculo;

use oraculo::grade::{amostra_suspensao, quadros_medidor, quadros_pulse_33, quadros_spectrum_pulse, t_k};
use oraculo::nivel::nivel_db;
use oraculo::sinal::{Tipo, sinal};

#[test]
fn seno_n12() {
    let v = sinal(Tipo::Seno, 12);
    let esperado = 0.5 * (std::f32::consts::PI * 1000.0 * 12.0 / 48000.0).sin();
    assert!((v - esperado).abs() < 1e-6, "seno n=12: {v} vs {esperado}");
    assert!((v - 0.35355339).abs() < 1e-6);
}

#[test]
fn degrau_n23999_n24000() {
    assert_eq!(0.0, sinal(Tipo::Degrau, 23999));
    assert!((sinal(Tipo::Degrau, 24000) - 0.5 * std::f32::consts::PI.sin()).abs() < 1e-6);
}

#[test]
fn rajada_n2399_n2400() {
    // [0, 50ms) => n=2399 ligado, n=2400 = 50ms desligado
    assert!((sinal(Tipo::Rajada, 2399) - 0.5 * (std::f64::consts::PI * 1000.0 * 2399.0 / 48000.0).sin() as f32).abs() < 1e-6);
    assert_eq!(0.0, sinal(Tipo::Rajada, 2400));
}

#[test]
fn amostra_suspensao_valores() {
    // t_0 = 0 ms => 0; t_1 = 16.666 ms => 16.666*48/128 = 6.25, ceil=7 => 896
    assert_eq!(0, amostra_suspensao(0));
    assert_eq!(896, amostra_suspensao(1));
}

#[test]
fn quadros_medidor_42() {
    let q = quadros_medidor();
    assert_eq!(42, q.len());
}

#[test]
fn quadros_pulse_33_59() {
    let q = quadros_pulse_33();
    assert_eq!(59, q.len());
}

#[test]
fn quadros_spectrum_pulse_120() {
    let q = quadros_spectrum_pulse();
    assert_eq!(120, q.len());
}

#[test]
fn nivel_seno_regime() {
    // Janela [93952..96000): 2048 amostras de seno puro (regime)
    let db = nivel_db(Tipo::Seno, 96000);
    let rms = 0.5 / (2.0f64).sqrt();
    let esperado = 20.0 * rms.max(1e-8).log10();
    assert!(
        (db - esperado).abs() <= 0.02,
        "nivel_db seno regime: {db} vs {esperado}"
    );
    assert!(db > -60.0);
}

#[test]
fn nivel_silencio_chao() {
    let db = nivel_db(Tipo::Degrau, 24000);
    assert_eq!(-60.0, db);
}

#[test]
fn t_k_formula() {
    assert_eq!(0.0, t_k(0));
    assert!((t_k(1) - 1000.0 / 60.0).abs() < 1e-9);
}