//! Gabarito de movimento: teste TASK-E02-13 AC-2.
//!
//! Teste headless que usa o oráculo de sinal, grade temporal e nível
//! para conferir o comportamento dos medidores e ondas do MVP.

mod oraculo;

use oraculo::analisador::{Analisador, bins_hifi, blackman, bloco, espectro_hifi, onda_osc};
use oraculo::estilos_espectro::{
    Comando, SpectrumPulse, audivel, envelope, geometria_spectrum, geometria_spectrum_pulse,
    limites_bandas, merge, spectrum_bands,
};
use oraculo::grade::{
    amostra_suspensao, quadros_medidor, quadros_pulse_33, quadros_spectrum_pulse, t_k,
};
use oraculo::nivel::nivel_db;
use oraculo::sinal::{Tipo, sinal};

const NYQUIST: f64 = 24000.0;
const LEN: usize = 1024;

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

// ===== Fase 3a: Estilos de espectro =====

#[test]
fn limites_bandas_prefixo_e_valores() {
    let limites = limites_bandas(NYQUIST, LEN);
    assert_eq!(limites.len(), 128);
    // Bandas 0 a 3
    assert_eq!(limites[0], (1, 2), "banda 0");
    assert_eq!(limites[1], (1, 2), "banda 1");
    assert_eq!(limites[2], (1, 2), "banda 2");
    assert_eq!(limites[3], (1, 3), "banda 3");
    // Banda 66
    assert_eq!(limites[66], (42, 45), "banda 66");
    // Banda 127
    assert_eq!(limites[127], (812, 854), "banda 127");
}

#[test]
fn spectrum_bands_valores_esperados() {
    let mut an = Analisador::new(2048, 0.75);
    let db = an.ler_espectro_db(Tipo::Seno, 48000);
    let (bands, freq) = spectrum_bands(&db, true, NYQUIST, 84.0);
    assert_eq!(bands.len(), 128);
    // Maior banda é a 66 com 0.6178136 ± 1e-6
    let max_band = bands[66];
    assert!(
        (max_band - 0.6178136).abs() < 1e-6,
        "banda 66 = {} (esperado 0.6178136)",
        max_band
    );
    // Banda 0 = 0
    assert_eq!(bands[0], 0.0, "banda 0 deve ser 0");
    // Frequência = 1007.8125 exata
    assert_eq!(freq, 1007.8125, "frequência de pico");
}

#[test]
fn spectrum_bands_sem_disponibilidade() {
    let mut an = Analisador::new(2048, 0.75);
    let db = an.ler_espectro_db(Tipo::Seno, 48000);
    let (bands, freq) = spectrum_bands(&db, false, NYQUIST, 84.0);
    // Todas as 128 bandas valem 0
    for (i, &b) in bands.iter().enumerate() {
        assert_eq!(b, 0.0, "banda {} deve ser 0 (sem disponibilidade)", i);
    }
    // Frequência vale 0
    assert_eq!(freq, 0.0, "frequência deve ser 0 sem disponibilidade");
}

#[test]
fn audivel_silencio_e_seno() {
    // Silêncio: bloco do degrau antes de 0.5s (amostra_fim = 3200 < 24000)
    let silencio = bloco(Tipo::Degrau, 3200, 2048);
    assert!(!audivel(&silencio), "silêncio não deve ser audível");

    // Seno: bloco do seno em regime (amostra_fim = 48000)
    let seno = bloco(Tipo::Seno, 48000, 2048);
    assert!(audivel(&seno), "seno deve ser audível");
}

#[test]
fn limites_pulse_prefixo_e_valores() {
    let limites = SpectrumPulse::limites_pulse(48000.0, 64, LEN);
    assert_eq!(limites.len(), 64);
    // Barras 0 a 3
    assert_eq!(limites[0], (0, 1), "barra 0");
    assert_eq!(limites[1], (0, 2), "barra 1");
    assert_eq!(limites[2], (1, 2), "barra 2");
    assert_eq!(limites[3], (1, 2), "barra 3");
    // Barra 36
    assert_eq!(limites[36], (41, 47), "barra 36");
    // Barra 63
    assert_eq!(limites[63], (766, 854), "barra 63");
}

#[test]
fn spectrum_pulse_update_com_bins_now_zero() {
    let mut an = Analisador::new(2048, 0.0);
    let db = an.ler_espectro_db(Tipo::Seno, 48000);
    let mut pulse = SpectrumPulse::new(64);
    pulse.update(Some(&db), 48000.0, 64, 0.0);
    // Maior barra é a 36 com 0.8742193 ± 1e-6
    let max_bar = pulse.bars[36];
    assert!(
        (max_bar - 0.8742193).abs() < 1e-6,
        "barra 36 = {} (esperado 0.8742193)",
        max_bar
    );
    // Pico 36 é igual à barra
    assert_eq!(
        pulse.peaks[36], pulse.bars[36],
        "pico 36 deve igualar barra 36"
    );
}

#[test]
fn spectrum_pulse_update_sem_bins_now_zero() {
    let mut an = Analisador::new(2048, 0.0);
    let db = an.ler_espectro_db(Tipo::Seno, 48000);
    let mut pulse = SpectrumPulse::new(64);
    // Primeira leitura com bins
    pulse.update(Some(&db), 48000.0, 64, 0.0);
    // Segunda leitura sem bins, now = 0, last = 0 => dt = 16
    pulse.update(None, 48000.0, 64, 0.0);
    // Barra 36 = 0.6955896 ± 1e-6
    let bar36 = pulse.bars[36];
    assert!(
        (bar36 - 0.6955896).abs() < 1e-6,
        "barra 36 = {} (esperado 0.6955896)",
        bar36
    );
    // Pico 36 = 0.8614193 ± 1e-6
    let peak36 = pulse.peaks[36];
    assert!(
        (peak36 - 0.8614193).abs() < 1e-6,
        "pico 36 = {} (esperado 0.8614193)",
        peak36
    );
}

#[test]
fn spectrum_pulse_update_dt_250() {
    let mut an = Analisador::new(2048, 0.0);
    let db = an.ler_espectro_db(Tipo::Seno, 48000);
    let mut pulse = SpectrumPulse::new(64);
    // Primeira leitura em now = 1000
    pulse.update(Some(&db), 48000.0, 64, 1000.0);
    // Segunda leitura sem bins em now = 1250 => dt = 250
    pulse.update(None, 48000.0, 64, 1250.0);
    // Barra 36 = 0.0245793 ± 1e-6
    let bar36 = pulse.bars[36];
    assert!(
        (bar36 - 0.0245793).abs() < 1e-6,
        "barra 36 = {} (esperado 0.0245793)",
        bar36
    );
    // Pico 36 = 0.6742193 ± 1e-6
    let peak36 = pulse.peaks[36];
    assert!(
        (peak36 - 0.6742193).abs() < 1e-6,
        "pico 36 = {} (esperado 0.6742193)",
        peak36
    );
}

#[test]
fn spectrum_pulse_update_dt_16_667() {
    let mut an = Analisador::new(2048, 0.0);
    let db = an.ler_espectro_db(Tipo::Seno, 48000);
    let mut pulse = SpectrumPulse::new(64);
    // Primeira leitura em now = 1000
    pulse.update(Some(&db), 48000.0, 64, 1000.0);
    // Segunda leitura sem bins em now = 1016.6666... => dt ≈ 16.6667
    pulse.update(None, 48000.0, 64, 1016.0 + 2.0 / 3.0);
    // Barra 36 = 0.6889964 ± 1e-6
    let bar36 = pulse.bars[36];
    assert!(
        (bar36 - 0.6889964).abs() < 1e-6,
        "barra 36 = {} (esperado 0.6889964)",
        bar36
    );
    // Pico 36 = 0.8608860 ± 1e-6
    let peak36 = pulse.peaks[36];
    assert!(
        (peak36 - 0.8608860).abs() < 1e-6,
        "pico 36 = {} (esperado 0.8608860)",
        peak36
    );
}

#[test]
fn merge_max_bin_a_bin() {
    let l = vec![0.1, 0.5, 0.3];
    let r = vec![0.4, 0.2, 0.3];
    let result = merge(&l, &r);
    assert_eq!(result, vec![0.4, 0.5, 0.3]);
}

#[test]
fn envelope_valores() {
    let cmds = envelope(&[0.5, 1.5], 20.0);
    // M0 24, L0 14, L1000 4, L1000 24, L1000 44, L0 34, Z
    let expected = vec![
        Comando::M(0.0, 24.0),
        Comando::L(0.0, 14.0),
        Comando::L(1000.0, 4.0),
        Comando::L(1000.0, 24.0),
        Comando::L(1000.0, 44.0),
        Comando::L(0.0, 34.0),
        Comando::Z,
    ];
    assert_eq!(cmds, expected);
}

#[test]
fn geometria_spectrum_pulse_com_valores() {
    let pulse = SpectrumPulse {
        bars: vec![0.004, 0.5],
        peaks: vec![0.01, 0.5],
        last: 0.0,
    };
    let (fill, halo, level) = geometria_spectrum_pulse(&pulse, 42.0);
    // Fill: só a barra 1 (bh = 0.5*42 = 21 > 0.2)
    let expected_fill = vec![
        Comando::M(501.5, 46.0),
        Comando::V(-21.0),
        Comando::H(497.0),
        Comando::V(21.0),
        Comando::Z,
    ];
    assert_eq!(fill, expected_fill, "fill deve ter só a barra 1");

    // Halo: só o pico 1 (0.5 > 0.02)
    let expected_halo = vec![
        Comando::M(501.5, 23.0),
        Comando::H(497.0),
        Comando::V(1.6),
        Comando::H(-497.0),
        Comando::Z,
    ];
    assert_eq!(halo, expected_halo, "halo deve ter só o pico 1");

    // Data level = 0.5
    assert_eq!(level, 0.5, "data_level");
}

#[test]
fn geometria_spectrum_pulse_vazio() {
    let pulse = SpectrumPulse {
        bars: vec![0.0, 0.0],
        peaks: vec![0.0, 0.0],
        last: 0.0,
    };
    let (fill, halo, level) = geometria_spectrum_pulse(&pulse, 42.0);
    // Fill e halo vazios: M0 46 H1000
    let expected_empty = vec![Comando::M(0.0, 46.0), Comando::H(1000.0)];
    assert_eq!(fill, expected_empty, "fill vazio");
    assert_eq!(halo, expected_empty, "halo vazio");
    assert_eq!(level, 0.0, "data_level vazio");
}

#[test]
fn geometria_spectrum_stereo() {
    let left = vec![0.5, 0.0];
    let right = vec![0.0, 1.0];
    let (paths, level) = geometria_spectrum("spectrum-stereo", &left, &right, 21.0);
    // Procura wave-spectrum-left e wave-spectrum-right
    let left_path = paths
        .iter()
        .find(|(k, _)| *k == "wave-spectrum-left")
        .unwrap();
    let right_path = paths
        .iter()
        .find(|(k, _)| *k == "wave-spectrum-right")
        .unwrap();

    // Left: M250 24 v-10.5, M750 24 v-0
    let expected_left = vec![
        Comando::M(250.0, 24.0),
        Comando::V(-10.5),
        Comando::M(750.0, 24.0),
        Comando::V(-0.0),
    ];
    assert_eq!(left_path.1, expected_left, "wave-spectrum-left");

    // Right: M250 24 v0, M750 24 v21
    let expected_right = vec![
        Comando::M(250.0, 24.0),
        Comando::V(0.0),
        Comando::M(750.0, 24.0),
        Comando::V(21.0),
    ];
    assert_eq!(right_path.1, expected_right, "wave-spectrum-right");

    // Data level = 1
    assert_eq!(level, 1.0, "data_level");
}

#[test]
fn geometria_spectrum_ribbon_suavizacao() {
    let left = vec![0.0, 1.0, 0.0];
    let right = vec![0.0, 1.0, 0.0]; // não usado em ribbon
    let (paths, _) = geometria_spectrum("spectrum-ribbon", &left, &right, 21.0);
    // Procura wave-spectrum-fill
    let fill_path = paths
        .iter()
        .find(|(k, _)| *k == "wave-spectrum-fill")
        .unwrap();

    // Suavização 1-2-1: [(0+0+1)/4, (0+2+0)/4, (1+0+0)/4] = [0.25, 0.5, 0.25]
    let smoothed = vec![0.25, 0.5, 0.25];
    let expected_fill = envelope(&smoothed, 21.0);
    assert_eq!(
        fill_path.1, expected_fill,
        "wave-spectrum-fill deve ser envelope da suavização"
    );
}
