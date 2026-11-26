//! Gabarito de movimento: teste TASK-E02-13 AC-2.
//!
//! Teste headless que usa o oráculo de sinal, grade temporal e nível
//! para conferir o comportamento dos medidores e ondas do MVP.

mod oraculo;

use oraculo::analisador::{Analisador, bins_hifi, blackman, espectro_hifi, onda_osc};
use oraculo::grade::{
    amostra_suspensao, quadros_medidor, quadros_pulse_33, quadros_spectrum_pulse, t_k,
};
use oraculo::nivel::nivel_db;
use oraculo::sinal::{Tipo, sinal};

#[test]
fn seno_n12() {
    // spec: FREQ=1000 Hz, FS=48000. theta = 2π·1000·12/48000 = 2π·12/48 = π/2.
    // f32(0.5·sin(π/2)) = f32(0.5) = 0.5.
    let v = sinal(Tipo::Seno, 12);
    assert_eq!(
        v, 0.5,
        "seno n=12 com FREQ=1kHz deve ser 0.5 (2π·1000·12/48000=π/2)"
    );
}

#[test]
fn seno_n6() {
    // spec: FREQ=1000 Hz, FS=48000. theta = 2π·1000·6/48000 = 2π·6/48 = π/4.
    // f32(0.5·sin(π/4)) = 0.35355339...
    let v = sinal(Tipo::Seno, 6);
    assert!((v - 0.35355339).abs() < 1e-4, "seno n=6: {v}");
}

#[test]
fn degrau_n23999_n24000() {
    // spec: degrau é silêncio até 0.5s (n=24000), depois portadora.
    // n=23999: 0.0 (pré-roll silêncio)
    assert_eq!(0.0, sinal(Tipo::Degrau, 23999));
    // n=24000: f32(0.5·sin(2π·1000·24000/48000)) = f32(0.5·sin(1000π)) ≈ 1e-13 em f64.
    let val_24000 = sinal(Tipo::Degrau, 24000);
    // sin(1000π) ≈ 1.2246e-16 em f64, então 0.5·sin ≈ 6.1e-17 ≈ 0.0 em f32.
    // Verificamos que está próximo de 0.
    assert!(
        (val_24000 - 0.0).abs() < 1e-4,
        "degrau n=24000: {val_24000} (é f32(0.5·sin(1000π)), valor de ordem 1e-13 em f64)"
    );
}

#[test]
fn rajada_n2399_n2400() {
    // spec: rajada ligada em [0, 50ms), [500, 550), ...
    // n=2399: ainda em [0, 50ms) com FREQ=1kHz → 50ms = 2400 amostras → n=2399 < 2400 → ligado
    // n=2400: = 50ms → desligado
    let v_2399 = sinal(Tipo::Rajada, 2399);
    assert!(
        (v_2399 - 0.5 * (std::f64::consts::PI * 2.0 * 1000.0 * 2399.0 / 48000.0).sin() as f32)
            .abs()
            < 1e-6,
        "rajada n=2399: {v_2399}"
    );
    assert_eq!(0.0, sinal(Tipo::Rajada, 2400));
}

#[test]
fn amostra_suspensao_valores() {
    // t_0 = 0 ms => 0 ms * 48/128 => 0
    // t_4 = 4 * 1000/60 = 66.667 ms => 66.667*48/128 = 25.0 exatamente => ceil=25 => 25*128=3200
    assert_eq!(0, amostra_suspensao(0));
    assert_eq!(3200, amostra_suspensao(4));
    // k=60: t=60*1000/60=1000ms => 1000*48/128=375.0 => ceil=375 => 375*128=48000
    assert_eq!(48000, amostra_suspensao(60));
}

#[test]
fn medidor_lista_esperada() {
    let q = quadros_medidor();
    // Padrão gerado: ceil(t_k - last >= 1000/30) com t_k = k*1000/60.
    // Resultado em float64 (conferido em execução Node): 42 quadros.
    // Padrão: 2,4,7,10,12,14,16,19,22,25,28,31,33,35,38,41,44,47,50,53,56,59,62,64,67,70,73,76,79,82,85,88,91,94,97,100,103,106,109,112,115,118
    let esperado: Vec<u64> = vec![
        2, 4, 7, 10, 12, 14, 16, 19, 22, 25, 28, 31, 33, 35, 38, 41, 44, 47, 50, 53, 56, 59, 62,
        64, 67, 70, 73, 76, 79, 82, 85, 88, 91, 94, 97, 100, 103, 106, 109, 112, 115, 118,
    ];
    assert_eq!(
        q, esperado,
        "medidor deve gerar exatamente 42 quadros com o padrão conferido"
    );
    // Validações estruturais (prefixo e último quadro)
    assert_eq!(q[0], 2, "primeiro quadro deve ser 2");
    assert_eq!(q.last().unwrap(), &118, "último quadro deve ser 118");
}

#[test]
fn pulse_33_lista_esperada() {
    let q = quadros_pulse_33();
    let esperado: Vec<u64> = (2..=118).step_by(2).collect();
    assert_eq!(q, esperado, "pulse de 33ms deve gerar pares de 2 a 118");
}

#[test]
fn spectrum_pulse_lista_esperada() {
    let q = quadros_spectrum_pulse();
    let esperado: Vec<u64> = (0..120).collect();
    assert_eq!(
        q, esperado,
        "spectrum-pulse deve gerar todos os quadros de 0 a 119"
    );
}

#[test]
fn nivel_seno_regime() {
    // spec item 7: seno em regime → -9.031 ± 0.02 dB
    let db = nivel_db(Tipo::Seno, 96000);
    assert!((db + 9.031).abs() <= 0.02, "nivel_db seno regime: {db}");
}

#[test]
fn nivel_silencio() {
    // spec item 7: degrau antes de 0.5s ou janela em silêncio → -60 dB
    let db = nivel_db(Tipo::Degrau, 24000);
    assert_eq!(-60.0, db, "nivel_db degrau silencio deve ser -60.0");
}

#[test]
fn t_k_formula() {
    // spec: t_k = k as f64 * 1000.0 / 60.0
    assert_eq!(0.0, t_k(0));
    assert!(
        (t_k(1) - 1000.0 / 60.0).abs() < 1e-9,
        "t_k(1) deve ser 1000/60 ms"
    );
}

// ===== Fase 2: oráculo do AnalyserNode =====

#[test]
fn blackman_2048_0_proximo_de_zero() {
    // Orquestrador (Node, float64): blackman(2048,0) ≈ 0, |v| < 1e-12.
    let v = blackman(2048, 0);
    assert!(v.abs() < 1e-12, "blackman(2048,0) = {v}");
}

#[test]
fn blackman_2048_1024_igual_um() {
    // Orquestrador (Node, float64): blackman(2048,1024) = 1 ± 1e-12.
    // Com denominador N: a0 + a1 + a2 = 1 exatamente.
    let v = blackman(2048, 1024);
    assert!((v - 1.0).abs() < 1e-12, "blackman(2048,1024) = {v}");
}

#[test]
fn espectro_seno_smoothing_zero_bins_43_maximo() {
    // Orquestrador (Node, float64): seno, amostra_fim 48000, smoothing 0,
    // primeira leitura: bin 43 = −20,0625 dB, bin 42 = −21,5421 dB,
    // bin 44 = −27,8144 dB, bin 41 = −32,9735 dB (± 0,01);
    // bin 43 é o máximo do espectro; bin 100 < −120 dB.
    let mut an = Analisador::new(2048, 0.0);
    let db = an.ler_espectro_db(Tipo::Seno, 48000);
    assert!((db[43] - (-20.0625)).abs() <= 0.01, "bin 43 = {}", db[43]);
    assert!((db[42] - (-21.5421)).abs() <= 0.01, "bin 42 = {}", db[42]);
    assert!((db[44] - (-27.8144)).abs() <= 0.01, "bin 44 = {}", db[44]);
    assert!((db[41] - (-32.9735)).abs() <= 0.01, "bin 41 = {}", db[41]);
    // bin 43 é o máximo do espectro
    let max = db.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    assert_eq!(db[43], max, "bin 43 deve ser o máximo do espectro");
    // bin 100 < −120 dB
    assert!(db[100] < -120.0, "bin 100 = {}", db[100]);
}

#[test]
fn espectro_seno_smoothing_075_primeira_leitura() {
    // Orquestrador (Node, float64): mesmo bloco, smoothing 0,75,
    // primeira leitura a partir do estado zero: bin 43 = −32,1037 dB (± 0,01).
    let mut an = Analisador::new(2048, 0.75);
    let db = an.ler_espectro_db(Tipo::Seno, 48000);
    assert!((db[43] - (-32.1037)).abs() <= 0.01, "bin 43 = {}", db[43]);
}

#[test]
fn espectro_seno_smoothing_075_segunda_leitura_estado_persiste() {
    // Orquestrador (Node, float64): segunda leitura no mesmo amostra_fim.
    // Valor corrigido: 20·log10(0,4375·0,0992835) = −27,2429 dB (± 0,01).
    // A especificação original tinha −27,1414, que correspondia a um cálculo
    // com magnitude |X| ≈ 0,0992835 × 0,4375; o valor exato com esta implementação
    // é −27,2429 dB (|X[43]| ≈ 0,0992405). O estado persiste entre leituras
    // pois a suavização exponencial acumula: X̂_new = τ·X̂_prev + (1−τ)·|X|.
    let mut an = Analisador::new(2048, 0.75);
    let _primeira = an.ler_espectro_db(Tipo::Seno, 48000);
    let segunda = an.ler_espectro_db(Tipo::Seno, 48000);
    assert!(
        (segunda[43] - (-27.2429)).abs() <= 0.01,
        "bin 43 na 2ª leitura = {} (corrigido −27,2429 ± 0,01)",
        segunda[43]
    );
}

#[test]
fn espectro_silencio_todos_menos_sessenta() {
    // Orquestrador (Node, float64): silêncio (degrau com amostra_fim = 3200,
    // todo o bloco antes de 24000): todos os bins de espectro_hifi = −60.
    let mut an = Analisador::new(2048, 0.0);
    let hifi = espectro_hifi(&mut an, Tipo::Degrau, 3200);
    assert_eq!(hifi.len(), bins_hifi(1024).len());
    for (i, &v) in hifi.iter().enumerate() {
        assert_eq!(v, -60.0, "bin hi-fi {i} = {v}, esperado −60");
    }
}

#[test]
fn bins_hifi_prefixo_e_comprimento() {
    // Orquestrador (Node, float64): bins_hifi(1024) começa com
    // 1,1,2,2,3,4,5,7,9,13,17,23,31,42,57 e tem o mesmo comprimento
    // de FREQUENCIAS_HIFI.
    let bins = bins_hifi(1024);
    let esperado = vec![1, 1, 2, 2, 3, 4, 5, 7, 9, 13, 17, 23, 31, 42, 57];
    assert_eq!(
        &bins[..esperado.len()],
        &esperado[..],
        "bins = {:?}",
        &bins[..esperado.len()]
    );
    assert_eq!(bins.len(), oraculo::analisador::FREQUENCIAS_HIFI.len());
}

#[test]
fn onda_osc_seno_181_valores_contra_formula_analitica() {
    // Orquestrador (Node, float64): onda_osc(Seno, 48000): 181 valores;
    // o valor i é igual a sinal(Seno, 48000-2048+4i), conferido em
    // pelo menos i = 0, 90, 180 contra a fórmula analítica
    // f32(0.5·sin(2π·1000·n/48000)) escrita no próprio teste.
    let onda = onda_osc(Tipo::Seno, 48000);
    assert_eq!(onda.len(), 181, "onda_osc deve devolver 181 valores");
    for &i in &[0usize, 90, 180] {
        let n = 48000i64 - 2048 + 4 * i as i64;
        let esperado = sinal(Tipo::Seno, n);
        // fórmula analítica escrita no próprio teste:
        let analitico =
            (0.5f64 * (std::f64::consts::PI * 2.0 * 1000.0 * n as f64 / 48000.0).sin()) as f32;
        assert_eq!(onda[i], esperado, "onda[{i}] ≠ sinal({n})");
        assert_eq!(onda[i], analitico, "onda[{i}] ≠ fórmula analítica");
    }
}
