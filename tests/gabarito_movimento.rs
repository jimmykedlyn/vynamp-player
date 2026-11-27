//! Gabarito de movimento: TASK-E02-13 AC-2 (medidor, onda, geometria dos
//! estilos de onda) e AC-3 (comportamento Web Audio da varredura do MVP,
//! transformado no formato consumível pelo Rust).

mod criterio;
mod oraculo;

use oraculo::analisador::{Analisador, bins_hifi, blackman, bloco, espectro_hifi, onda_osc};
use oraculo::estilos_arquivo::{EstiloArquivo, alturas, geometria_arquivo};
use oraculo::estilos_criativos::{
    geometria_aurora_weave, geometria_camadas, geometria_constelacao,
};
use oraculo::estilos_espectro::{
    Comando, SpectrumPulse, audivel, envelope, geometria_spectrum, geometria_spectrum_pulse,
    limites_bandas, merge, spectrum_bands,
};
use oraculo::estilos_temporais::{
    BarsPulse, data_peak, geometria_aurora_neon, geometria_bars_radio, geometria_colunas,
    geometria_pulse, geometria_ribbon,
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
        (v_2399 - 0.5 * libm::sin(std::f64::consts::PI * 2.0 * 1000.0 * 2399.0 / 48000.0) as f32)
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
    assert_eq!(
        q.last()
            .expect("lista de quadros nao vazia; ultimo quadro deve ser 118"),
        &118,
        "último quadro deve ser 118"
    );
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
            (0.5f64 * libm::sin(std::f64::consts::PI * 2.0 * 1000.0 * n as f64 / 48000.0)) as f32;
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
        .expect("geometria_spectrum deve gerar wave-spectrum-left");
    let right_path = paths
        .iter()
        .find(|(k, _)| *k == "wave-spectrum-right")
        .expect("geometria_spectrum deve gerar wave-spectrum-right");

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
        .expect("geometria_spectrum deve gerar wave-spectrum-fill");

    // Suavização 1-2-1: [(0+0+1)/4, (0+2+0)/4, (1+0+0)/4] = [0.25, 0.5, 0.25]
    let smoothed = vec![0.25, 0.5, 0.25];
    let expected_fill = envelope(&smoothed, 21.0);
    assert_eq!(
        fill_path.1, expected_fill,
        "wave-spectrum-fill deve ser envelope da suavização"
    );
}

#[test]
fn geometria_spectrum_aurora_left_e_halo_top_mais_bottom() {
    // Given: bandas de spectrum-aurora; o MVP usa right = left fora de spectrum-stereo.
    let left = vec![0.5, 0.0];
    let right = left.clone();
    // When
    let (paths, level) = geometria_spectrum("spectrum-aurora", &left, &right, 21.0);
    // Then: left e halo são top+bottom (waveform-styles.js linha 80, drawSpectrum).
    let expected = vec![
        Comando::M(250.0, 24.0),
        Comando::V(-10.5),
        Comando::M(750.0, 24.0),
        Comando::V(-0.0),
        Comando::M(250.0, 24.0),
        Comando::V(10.5),
        Comando::M(750.0, 24.0),
        Comando::V(0.0),
    ];
    let left_path = paths
        .iter()
        .find(|(k, _)| *k == "wave-spectrum-left")
        .map(|(_, d)| d.clone());
    let halo_path = paths
        .iter()
        .find(|(k, _)| *k == "wave-spectrum-halo")
        .map(|(_, d)| d.clone());
    assert_eq!(left_path, Some(expected.clone()), "wave-spectrum-left");
    assert_eq!(halo_path, Some(expected), "wave-spectrum-halo");
    assert_eq!(level, 0.5, "data_level");
}

// ---------------------------------------------------------------------------
// Fase 3b — estilos de leitura temporal ("Estilos de onda", item 4).
//
// Os literais esperados vêm do código do MVP (waveform-styles.js, commit
// 0d9860b9) rodado por um script Node de scratch, fora do repo, com
// `Number.prototype.toFixed` neutralizado para devolver o valor cheio:
// o oráculo compara sem arredondar.
// ---------------------------------------------------------------------------

/// Tolerância de igualdade entre o oráculo e o MVP sem `toFixed`: só a
/// diferença de último ulp entre `Math.sin` do V8 e `libm::sin`.
const EPS_TEMPORAL: f64 = 1e-9;

/// Bloco determinístico de 2048 amostras que passa de ±1 (exercita o recorte):
/// `f32(((j·37 mod 101) − 50)·j / (40·2048))`, o mesmo do script Node.
fn bloco_padrao() -> Vec<f32> {
    (0..2048u64)
        .map(|j| ((((j * 37) % 101) as f64 - 50.0) * j as f64 / (40.0 * 2048.0)) as f32)
        .collect()
}

/// Compara duas sequências de comandos: mesma letra, mesma contagem e cada
/// número a até `EPS_TEMPORAL`.
fn assert_comandos(obtido: &[Comando], esperado: &[Comando], rotulo: &str) {
    assert_eq!(
        obtido.len(),
        esperado.len(),
        "{rotulo}: contagem de comandos (obtido {obtido:?})"
    );
    for (i, (a, b)) in obtido.iter().zip(esperado).enumerate() {
        let perto = |x: f64, y: f64| (x - y).abs() <= EPS_TEMPORAL;
        let igual = match (a, b) {
            (Comando::M(x0, y0), Comando::M(x1, y1)) | (Comando::L(x0, y0), Comando::L(x1, y1)) => {
                perto(*x0, *x1) && perto(*y0, *y1)
            }
            (Comando::V(d0), Comando::V(d1)) | (Comando::H(d0), Comando::H(d1)) => perto(*d0, *d1),
            (Comando::Z, Comando::Z) => true,
            (Comando::A(a0, b0, c0, d0, e0, f0, g0), Comando::A(a1, b1, c1, d1, e1, f1, g1)) => [
                (a0, a1),
                (b0, b1),
                (c0, c1),
                (d0, d1),
                (e0, e1),
                (f0, f1),
                (g0, g1),
            ]
            .iter()
            .all(|(x, y)| perto(**x, **y)),
            _ => false,
        };
        assert!(igual, "{rotulo}: comando {i} obtido {a:?}, esperado {b:?}");
    }
}

#[test]
fn pulse_padrao_16_amostras_com_recorte() {
    // Given: bloco padrão (valores fora de ±1), samples 16, amplitude 22.
    let amostras = bloco_padrao();
    // When
    let d = geometria_pulse(Some(&amostras), 16, 22.0);
    // Then: M/L com x = i·1000/15 e y = 24 − clamp(s[floor(i·2048/16)], −1, 1)·22
    // (waveform-styles.js:89).
    let esperado = vec![
        Comando::M(0.0, 24.0),
        Comando::L(66.66666666666667, 22.625),
        Comando::L(133.33333333333334, 22.00624993443489),
        Comando::L(200.0, 22.143749967217445),
        Comando::L(266.6666666666667, 23.037500016391277),
        Comando::L(333.3333333333333, 24.6875),
        Comando::L(400.0, 27.09375),
        Comando::L(466.6666666666667, 30.256250262260437),
        Comando::L(533.3333333333334, 34.17500013113022),
        Comando::L(600.0, 38.85000026226044),
        Comando::L(666.6666666666666, 9.5625),
        Comando::L(733.3333333333334, 12.278125524520874),
        Comando::L(800.0, 15.75),
        Comando::L(866.6666666666666, 19.97812506556511),
        Comando::L(933.3333333333334, 24.962499983608723),
        Comando::L(1000.0, 30.703125),
    ];
    assert_comandos(&d, &esperado, "pulse ns16");
}

#[test]
fn pulse_padrao_256_amostras_recorta_em_mais_e_menos_um() {
    // Given: bloco padrão; s[1848] = 1.1279… e s[1960] = −1.1484… passam de ±1.
    let amostras = bloco_padrao();
    // When: samples 256, amplitude 22 (defaultConfigs, waveform-styles.js:11).
    let d = geometria_pulse(Some(&amostras), 256, 22.0);
    // Then: 256 comandos; os índices 231 e 245 recortam em 24 ∓ 22.
    assert_eq!(d.len(), 256, "pulse ns256: contagem");
    for (i, esperado) in [
        (0, Comando::M(0.0, 24.0)),
        (1, Comando::L(3.9215686274509802, 23.90546874795109)),
        (230, Comando::L(901.9607843137255, 45.7421875)),
        (231, Comando::L(905.8823529411765, 2.0)),
        (245, Comando::L(960.7843137254902, 46.0)),
        (246, Comando::L(964.7058823529412, 2.0)),
        (255, Comando::L(1000.0, 33.3134765625)),
    ] {
        assert_comandos(&d[i..=i], &[esperado], &format!("pulse ns256 [{i}]"));
    }
}

#[test]
fn pulse_seno_quadro_em_26048() {
    // Given: seno, bloco que termina na amostra 26048 (exclusivo).
    let amostras = bloco(Tipo::Seno, 26048, 2048);
    // When
    let d = geometria_pulse(Some(&amostras), 256, 22.0);
    // Then
    assert_eq!(d.len(), 256, "pulse seno: contagem");
    for (i, esperado) in [
        (0, Comando::M(0.0, 23.999999999998533)),
        (1, Comando::L(3.9215686274509802, 14.473720729351044)),
        (64, Comando::L(250.98039215686273, 33.526279270648956)),
        (255, Comando::L(1000.0, 23.99999999999514)),
    ] {
        assert_comandos(&d[i..=i], &[esperado], &format!("pulse seno [{i}]"));
    }
}

#[test]
fn pulse_sem_amostras_e_linha_central() {
    // Given: copyLiveSamples devolveu false. When
    let d = geometria_pulse(None, 256, 22.0);
    // Then: o MVP grava `M0 24H1000`.
    assert_eq!(d, vec![Comando::M(0.0, 24.0), Comando::H(1000.0)]);
}

#[test]
fn data_peak_bloco_padrao_seno_e_sem_amostras() {
    // Given/When/Then: maior |amostra| do bloco (waveform-styles.js:69).
    let padrao = bloco_padrao();
    assert!((data_peak(Some(&padrao)) - 1.23291015625).abs() <= EPS_TEMPORAL);
    let seno = bloco(Tipo::Seno, 26048, 2048);
    assert!((data_peak(Some(&seno)) - 0.5).abs() <= EPS_TEMPORAL);
    assert_eq!(data_peak(None), 0.0);
}

#[test]
fn colunas_quatro_colunas_bloco_padrao() {
    // Given: columns 4, gain 1.4, height 21.
    let amostras = bloco_padrao();
    // When
    let d = geometria_colunas(Some(&amostras), 4, 1.4, 21.0);
    // Then: M x (24−bh) v (2bh), bh = min(1, pico·1.4)·21, x = (i+.5)·1000/4.
    let esperado = vec![
        Comando::M(125.0, 14.9381103515625),
        Comando::V(18.123779296875),
        Comando::M(375.0, 5.876220703125),
        Comando::V(36.24755859375),
        Comando::M(625.0, 3.0),
        Comando::V(42.0),
        Comando::M(875.0, 3.0),
        Comando::V(42.0),
    ];
    assert_comandos(&d, &esperado, "columns 4");
}

#[test]
fn colunas_padrao_128_bloco_padrao_e_seno() {
    // Given: defaultConfigs de columns-pulse e prisma-neon (128, 1.4, 21).
    let padrao = bloco_padrao();
    let d = geometria_colunas(Some(&padrao), 128, 1.4, 21.0);
    assert_eq!(d.len(), 256, "columns 128: contagem");
    for (i, esperado) in [
        (0, Comando::M(3.90625, 23.81409668242559)),
        (1, Comando::V(0.37180663514882323)),
        (2, Comando::M(11.71875, 23.461669921875)),
        (3, Comando::V(1.07666015625)),
        (246, Comando::M(964.84375, 3.0)),
        (247, Comando::V(42.0)),
    ] {
        assert_comandos(&d[i..=i], &[esperado], &format!("columns padrão [{i}]"));
    }
    let seno = bloco(Tipo::Seno, 26048, 2048);
    let d = geometria_colunas(Some(&seno), 128, 1.4, 21.0);
    assert_eq!(d.len(), 256, "columns seno: contagem");
    for (i, esperado) in [
        (0, Comando::M(3.90625, 9.3)),
        (1, Comando::V(29.4)),
        (254, Comando::M(996.09375, 11.269426792860031)),
        (255, Comando::V(25.461146414279938)),
    ] {
        assert_comandos(&d[i..=i], &[esperado], &format!("columns seno [{i}]"));
    }
}

#[test]
fn colunas_sem_amostras_tem_altura_zero() {
    let d = geometria_colunas(None, 2, 1.4, 21.0);
    let esperado = vec![
        Comando::M(250.0, 24.0),
        Comando::V(0.0),
        Comando::M(750.0, 24.0),
        Comando::V(0.0),
    ];
    assert_comandos(&d, &esperado, "columns sem amostras");
}

#[test]
fn ribbon_quatro_segmentos_bloco_padrao() {
    // Given: segments 4, gain 2.0, height 21.
    let amostras = bloco_padrao();
    // When
    let d = geometria_ribbon(Some(&amostras), 4, 2.0, 21.0);
    // Then: envelope do RMS por segmento recortado em 1 e suavizado 1-2-1
    // (waveform-styles.js:71).
    let esperado = vec![
        Comando::M(0.0, 24.0),
        Comando::L(0.0, 17.769313738665023),
        Comando::L(333.3333333333333, 12.242811452432393),
        Comando::L(666.6666666666666, 6.202751161582711),
        Comando::L(1000.0, 3.4363764556116294),
        Comando::L(1000.0, 24.0),
        Comando::L(1000.0, 44.56362354438837),
        Comando::L(666.6666666666666, 41.79724883841729),
        Comando::L(333.3333333333333, 35.75718854756761),
        Comando::L(0.0, 30.230686261334977),
        Comando::Z,
    ];
    assert_comandos(&d, &esperado, "ribbon 4");
}

#[test]
fn ribbon_padrao_degrau_na_borda_do_ataque() {
    // Given: degrau; o bloco que termina em 24576 tem silêncio até 24000.
    let amostras = bloco(Tipo::Degrau, 24576, 2048);
    // When: defaultConfigs de ribbon-pulse (64, 2.0, 21).
    let d = geometria_ribbon(Some(&amostras), 64, 2.0, 21.0);
    // Then: 64·2 + 3 comandos; a suavização espalha o ataque para o segmento 45.
    assert_eq!(d.len(), 131, "ribbon degrau: contagem");
    for (i, esperado) in [
        (0, Comando::M(0.0, 24.0)),
        (1, Comando::L(0.0, 24.0)),
        (32, Comando::L(492.06349206349205, 24.0)),
        (45, Comando::L(698.4126984126984, 24.0)),
        (46, Comando::L(714.2857142857143, 20.52987426419936)),
        (47, Comando::L(730.1587301587301, 12.983695548144151)),
        (48, Comando::L(746.031746031746, 8.81577887461734)),
        (49, Comando::L(761.9047619047619, 9.32984242579903)),
        (64, Comando::L(1000.0, 9.237978732526784)),
        (65, Comando::L(1000.0, 24.0)),
        (66, Comando::L(1000.0, 38.762021267473216)),
        (130, Comando::Z),
    ] {
        assert_comandos(&d[i..=i], &[esperado], &format!("ribbon degrau [{i}]"));
    }
}

#[test]
fn aurora_neon_line_e_halo_iguais_e_recorte() {
    // Given: bloco padrão, samples 4, amplitude 22.
    let amostras = bloco_padrao();
    // When
    let (line, halo) = geometria_aurora_neon(Some(&amostras), 4, 22.0);
    // Then: mesma aritmética de pulse (waveform-styles.js:72), mesmo d nos dois.
    let esperado = vec![
        Comando::M(0.0, 24.0),
        Comando::L(333.3333333333333, 23.037500016391277),
        Comando::L(666.6666666666666, 34.17500013113022),
        Comando::L(1000.0, 15.75),
    ];
    assert_comandos(&line, &esperado, "aurora-neon line");
    assert_comandos(&halo, &esperado, "aurora-neon halo");
    // Recorte em ±1 com o padrão de 256 amostras.
    let (line, _) = geometria_aurora_neon(Some(&amostras), 256, 22.0);
    assert_comandos(
        &line[231..=231],
        &[Comando::L(905.8823529411765, 2.0)],
        "aurora-neon [231]",
    );
    assert_comandos(
        &line[245..=245],
        &[Comando::L(960.7843137254902, 46.0)],
        "aurora-neon [245]",
    );
}

#[test]
fn aurora_neon_sem_amostras_desenha_linha_em_24() {
    // Diferente de pulse: sem amostras, aurora-neon desenha os ns pontos em 24.
    let (line, halo) = geometria_aurora_neon(None, 4, 22.0);
    let esperado = vec![
        Comando::M(0.0, 24.0),
        Comando::L(333.3333333333333, 24.0),
        Comando::L(666.6666666666666, 24.0),
        Comando::L(1000.0, 24.0),
    ];
    assert_comandos(&line, &esperado, "aurora-neon line vazia");
    assert_comandos(&halo, &esperado, "aurora-neon halo vazia");
}

#[test]
fn bars_pulse_sequencia_de_niveis() {
    // Given: pulseLevel começa em 1 (scaleBars(1), waveform-styles.js:65);
    // gain 1.0, smoothing 0.3 (defaultConfigs).
    let mut estado = BarsPulse::new();
    assert_eq!(estado.nivel, 1.0, "nível inicial");
    // When/Then: zeros dão alvo 0 e o nível desce com fator sm: 1 + (0−1)·0.3 = 0.7.
    let zeros = vec![0.0f32; 2048];
    let n = estado.update(Some(&zeros), 1.0, 0.3);
    assert!((n - 0.7).abs() <= EPS_TEMPORAL, "zeros: {n}");
    // Degrau parcial (64 amostras ligadas): alvo abaixo, desce.
    let n = estado.update(Some(&bloco(Tipo::Degrau, 24064, 2048)), 1.0, 0.3);
    assert!(
        (n - 0.6593442411092251).abs() <= EPS_TEMPORAL,
        "degrau 24064: {n}"
    );
    // Degrau pleno: alvo acima, sobe com fator 1−sm.
    let n = estado.update(Some(&bloco(Tipo::Degrau, 26048, 2048)), 1.0, 0.3);
    assert!(
        (n - 0.8978032723327675).abs() <= EPS_TEMPORAL,
        "degrau 26048: {n}"
    );
    // Sem amostras: alvo 0.
    let n = estado.update(None, 1.0, 0.3);
    assert!(
        (n - 0.6284622906329372).abs() <= EPS_TEMPORAL,
        "sem amostras: {n}"
    );
    assert_eq!(estado.nivel, n, "o estado guarda o último nível");
}

#[test]
fn bars_pulse_alvo_recortado_em_um() {
    // Given: bloco padrão, gain 0.5: sqrt(min(1, rms·5·0.5)) = 1, nível segue 1.
    let mut estado = BarsPulse::new();
    let n = estado.update(Some(&bloco_padrao()), 0.5, 0.6);
    assert_eq!(n, 1.0);
}

#[test]
fn bars_pulse_radio_quatro_barras() {
    // Given: ramo de rádio, bars 4, gain 1.0, bloco padrão.
    let amostras = bloco_padrao();
    // When
    let d = geometria_bars_radio(Some(&amostras), 4, 1.0);
    // Then: M x (24−h) v (2h), h = min(1, pico·g)·22, x = i·1000/bars.
    let esperado = vec![
        Comando::M(0.0, 17.218994140625),
        Comando::V(13.56201171875),
        Comando::M(250.0, 10.43798828125),
        Comando::V(27.1240234375),
        Comando::M(500.0, 3.656982421875),
        Comando::V(40.68603515625),
        Comando::M(750.0, 2.0),
        Comando::V(44.0),
    ];
    assert_comandos(&d, &esperado, "bars rádio 4");
}

#[test]
fn bars_pulse_radio_sem_amostras() {
    let d = geometria_bars_radio(None, 2, 1.0);
    let esperado = vec![
        Comando::M(0.0, 24.0),
        Comando::V(0.0),
        Comando::M(500.0, 24.0),
        Comando::V(0.0),
    ];
    assert_comandos(&d, &esperado, "bars rádio vazio");
}

// ---------------------------------------------------------------------------
// Fase 3c — estilos criativos e envelope do arquivo ("Estilos de onda",
// itens 1, 2 e 4; AC-3).
//
// Os literais esperados vêm do código do MVP (waveform-styles.js, commit
// 0d9860b9, linhas 7, 64, 66, 76, 84 a 87 e 93) rodado por um script Node de
// scratch, fora do repo, que recusa o MVP em outro commit, com
// `Number.prototype.toFixed` neutralizado para devolver o valor cheio.
// ---------------------------------------------------------------------------

/// Espectro sintético de 1024 bins em dB, o mesmo do script Node:
/// −83,5 nos bins 0 a 39 e `−100 + (j·53 mod 97)` nos demais (todos exatos em f32).
fn db_sintetico() -> Vec<f64> {
    (0..1024u64)
        .map(|j| {
            if j < 40 {
                -83.5
            } else {
                -100.0 + ((j * 53) % 97) as f64
            }
        })
        .collect()
}

#[test]
fn aurora_weave_quatro_amostras_dois_canais() {
    // Given: esquerda bloco padrão, direita seno até 26048; samples 4, amplitude 22.
    let padrao = bloco_padrao();
    let seno = bloco(Tipo::Seno, 26048, 2048);
    // When
    let w = geometria_aurora_weave(Some(&padrao), Some(&seno), 4, 22.0);
    // Then: data-level = maior |amostra| dos dois canais (waveform-styles.js:85).
    assert!(
        (w.nivel - 1.23291015625).abs() <= EPS_TEMPORAL,
        "nível {}",
        w.nivel
    );
    assert_comandos(
        &w.esquerda,
        &[
            Comando::M(0.0, 24.0),
            Comando::L(333.3333333333333, 23.037500016391277),
            Comando::L(666.6666666666666, 34.17500013113022),
            Comando::L(1000.0, 15.75),
        ],
        "weave esquerda",
    );
    assert_comandos(
        &w.direita,
        &[
            Comando::M(0.0, 23.999999999998533),
            Comando::L(333.3333333333333, 33.526279270648956),
            Comando::L(666.6666666666666, 14.473720729351044),
            Comando::L(1000.0, 24.00000000000612),
        ],
        "weave direita",
    );
}

#[test]
fn aurora_weave_canal_ausente_desenha_linha_em_24_e_sai_do_nivel() {
    // Given: só a esquerda (seno). When
    let seno = bloco(Tipo::Seno, 26048, 2048);
    let w = geometria_aurora_weave(Some(&seno), None, 4, 22.0);
    // Then: o nível ignora a direita e a direita fica em 24.
    assert!((w.nivel - 0.5).abs() <= EPS_TEMPORAL, "nível {}", w.nivel);
    let em_24 = [
        Comando::M(0.0, 24.0),
        Comando::L(333.3333333333333, 24.0),
        Comando::L(666.6666666666666, 24.0),
        Comando::L(1000.0, 24.0),
    ];
    assert_comandos(&w.direita, &em_24, "weave sem direita");
    // Given: só a direita (bloco padrão).
    let padrao = bloco_padrao();
    let w = geometria_aurora_weave(None, Some(&padrao), 4, 22.0);
    assert!(
        (w.nivel - 1.23291015625).abs() <= EPS_TEMPORAL,
        "nível {}",
        w.nivel
    );
    assert_comandos(&w.esquerda, &em_24, "weave sem esquerda");
}

#[test]
fn aurora_weave_padrao_256_recorta_em_mais_e_menos_um() {
    // Given: defaultConfigs de aurora-weave (samples 256, amplitude 22).
    let padrao = bloco_padrao();
    let seno = bloco(Tipo::Seno, 26048, 2048);
    // When
    let w = geometria_aurora_weave(Some(&padrao), Some(&seno), 256, 22.0);
    // Then
    assert_eq!(w.esquerda.len(), 256, "weave esquerda: contagem");
    assert_eq!(w.direita.len(), 256, "weave direita: contagem");
    for (i, esperado) in [
        (0, Comando::M(0.0, 24.0)),
        (1, Comando::L(3.9215686274509802, 23.90546874795109)),
        (231, Comando::L(905.8823529411765, 2.0)),
        (245, Comando::L(960.7843137254902, 46.0)),
        (255, Comando::L(1000.0, 33.3134765625)),
    ] {
        assert_comandos(
            &w.esquerda[i..=i],
            &[esperado],
            &format!("weave esquerda [{i}]"),
        );
    }
    for (i, esperado) in [
        (0, Comando::M(0.0, 23.999999999998533)),
        (1, Comando::L(3.9215686274509802, 14.473720729351044)),
        (64, Comando::L(250.98039215686273, 33.526279270648956)),
        (255, Comando::L(1000.0, 23.99999999999514)),
    ] {
        assert_comandos(
            &w.direita[i..=i],
            &[esperado],
            &format!("weave direita [{i}]"),
        );
    }
}

#[test]
fn neon_crystal_quatro_segmentos_bloco_padrao() {
    // Given: segments 4, peakGain 2, rmsGain 2, alturas 21 e 16 (padrões do código).
    let padrao = bloco_padrao();
    // When
    let c = geometria_camadas(Some(&padrao), 4, 2.0, 2.0, 21.0, 16.0);
    // Then: waveform-styles.js:87; inner com round(16/2) = 8.
    assert!(
        (c.nivel - 1.23291015625).abs() <= EPS_TEMPORAL,
        "nível {}",
        c.nivel
    );
    assert_comandos(
        &c.externa,
        &[
            Comando::M(0.0, 24.0),
            Comando::L(0.0, 11.054443359375),
            Comando::L(333.3333333333333, 3.0),
            Comando::L(666.6666666666666, 3.0),
            Comando::L(1000.0, 3.0),
            Comando::L(1000.0, 24.0),
            Comando::L(1000.0, 45.0),
            Comando::L(666.6666666666666, 45.0),
            Comando::L(333.3333333333333, 45.0),
            Comando::L(0.0, 36.945556640625),
            Comando::Z,
        ],
        "crystal outer",
    );
    assert_comandos(
        &c.media,
        &[
            Comando::M(0.0, 24.0),
            Comando::L(0.0, 20.636764940500896),
            Comando::L(333.3333333333333, 15.100947048714525),
            Comando::L(666.6666666666666, 9.32990919805449),
            Comando::L(1000.0, 8.0),
            Comando::L(1000.0, 24.0),
            Comando::L(1000.0, 40.0),
            Comando::L(666.6666666666666, 38.670090801945506),
            Comando::L(333.3333333333333, 32.899052951285476),
            Comando::L(0.0, 27.363235059499104),
            Comando::Z,
        ],
        "crystal middle",
    );
    assert_comandos(
        &c.interna,
        &[
            Comando::M(0.0, 24.0),
            Comando::L(0.0, 22.318382470250448),
            Comando::L(333.3333333333333, 19.550473524357262),
            Comando::L(666.6666666666666, 16.664954599027247),
            Comando::L(1000.0, 16.0),
            Comando::L(1000.0, 24.0),
            Comando::L(1000.0, 32.0),
            Comando::L(666.6666666666666, 31.335045400972753),
            Comando::L(333.3333333333333, 28.449526475642738),
            Comando::L(0.0, 25.681617529749552),
            Comando::Z,
        ],
        "crystal inner",
    );
}

#[test]
fn neon_crystal_padrao_degrau_na_borda_do_ataque() {
    // Given: degrau, bloco que termina em 24576; defaultConfigs de neon-crystal (32).
    let degrau = bloco(Tipo::Degrau, 24576, 2048);
    // When
    let c = geometria_camadas(Some(&degrau), 32, 2.0, 2.0, 21.0, 16.0);
    // Then: 32·2 + 3 comandos por camada; silêncio até o segmento 22.
    assert!((c.nivel - 0.5).abs() <= EPS_TEMPORAL, "nível {}", c.nivel);
    for (rotulo, d, esperado) in [
        ("outer", &c.externa, [3.0, 3.0, 3.0, 45.0, 45.0]),
        (
            "middle",
            &c.media,
            [
                12.464051247635014,
                13.283481649198663,
                12.334747601980407,
                35.66525239801959,
                35.66525239801959,
            ],
        ),
        (
            "inner",
            &c.interna,
            [
                18.232025623817506,
                18.64174082459933,
                18.167373800990205,
                29.832626199009795,
                29.832626199009795,
            ],
        ),
    ] {
        assert_eq!(d.len(), 67, "crystal degrau {rotulo}: contagem");
        for (i, cmd) in [
            (0, Comando::M(0.0, 24.0)),
            (1, Comando::L(0.0, 24.0)),
            (23, Comando::L(709.6774193548387, 24.0)),
            (24, Comando::L(741.9354838709677, esperado[0])),
            (25, Comando::L(774.1935483870968, esperado[1])),
            (32, Comando::L(1000.0, esperado[2])),
            (33, Comando::L(1000.0, 24.0)),
            (34, Comando::L(1000.0, esperado[3])),
            (40, Comando::L(806.4516129032259, esperado[4])),
            (66, Comando::Z),
        ] {
            assert_comandos(&d[i..=i], &[cmd], &format!("crystal degrau {rotulo} [{i}]"));
        }
    }
}

#[test]
fn aurora_layers_padrao_seno_e_sem_amostras() {
    // Given: seno; defaultConfigs de aurora-layers (64, peakHeight 21, rmsHeight 16).
    let seno = bloco(Tipo::Seno, 26048, 2048);
    // When
    let c = geometria_camadas(Some(&seno), 64, 2.0, 2.0, 21.0, 16.0);
    // Then
    assert!((c.nivel - 0.5).abs() <= EPS_TEMPORAL, "nível {}", c.nivel);
    for (rotulo, d, cima, baixo) in [
        ("outer", &c.externa, 3.0, 45.0),
        ("middle", &c.media, 13.424378709940907, 34.57562129005909),
        ("inner", &c.interna, 18.712189354970455, 29.287810645029545),
    ] {
        assert_eq!(d.len(), 131, "layers seno {rotulo}: contagem");
        for (i, cmd) in [
            (0, Comando::M(0.0, 24.0)),
            (1, Comando::L(0.0, cima)),
            (64, Comando::L(1000.0, cima)),
            (65, Comando::L(1000.0, 24.0)),
            (66, Comando::L(1000.0, baixo)),
            (130, Comando::Z),
        ] {
            assert_comandos(&d[i..=i], &[cmd], &format!("layers seno {rotulo} [{i}]"));
        }
    }
    // Sem amostras: nível 0 e envelope achatado em 24.
    let c = geometria_camadas(None, 64, 2.0, 2.0, 21.0, 16.0);
    assert_eq!(c.nivel, 0.0, "layers sem amostras: nível");
    assert_eq!(c.externa.len(), 131, "layers sem amostras: contagem");
    for (i, x) in [(1, 0.0), (64, 1000.0), (66, 1000.0)] {
        assert_comandos(
            &c.interna[i..=i],
            &[Comando::L(x, 24.0)],
            &format!("layers sem amostras inner [{i}]"),
        );
    }
}

#[test]
fn aurora_constellation_quatro_nos() {
    // Given: bandas de spectrumBands(0, 84) sobre o espectro sintético, com
    // amostra audível; nodes 4, nodeRadius 1.7, height 21.
    let (bandas, freq) = spectrum_bands(&db_sintetico(), true, NYQUIST, 84.0);
    assert!((freq - 2015.625).abs() <= EPS_TEMPORAL, "frequência {freq}");
    // When
    let c = geometria_constelacao(&bandas, 4, 1.7, 21.0);
    // Then: waveform-styles.js:84; só nós acima de 0,01 têm círculo.
    assert!(
        (c.nivel - 0.9523809523809523).abs() <= EPS_TEMPORAL,
        "nível {}",
        c.nivel
    );
    let ida = Comando::A(1.7, 1.7, 0.0, 1.0, 0.0, 3.4, 0.0);
    let volta = Comando::A(1.7, 1.7, 0.0, 1.0, 0.0, -3.4, 0.0);
    assert_comandos(
        &c.nos,
        &[
            Comando::M(623.3, 4.0),
            ida.clone(),
            volta.clone(),
            Comando::M(623.3, 44.0),
            ida.clone(),
            volta.clone(),
            Comando::M(873.3, 4.0),
            ida.clone(),
            volta.clone(),
            Comando::M(873.3, 44.0),
            ida,
            volta,
        ],
        "constellation nós",
    );
    assert_comandos(
        &c.malha,
        &[
            Comando::M(125.0, 23.875),
            Comando::L(375.0, 23.875),
            Comando::L(625.0, 4.0),
            Comando::L(875.0, 4.0),
            Comando::M(125.0, 24.125),
            Comando::L(375.0, 24.125),
            Comando::L(625.0, 44.0),
            Comando::L(875.0, 44.0),
        ],
        "constellation malha",
    );
}

#[test]
fn aurora_constellation_padrao_32_nos() {
    // Given: defaultConfigs de aurora-constellation (nodes 32, nodeRadius 1.7, height 21).
    let (bandas, _) = spectrum_bands(&db_sintetico(), true, NYQUIST, 84.0);
    // When
    let c = geometria_constelacao(&bandas, 32, 1.7, 21.0);
    // Then: 16 nós acima do limiar, 3 comandos por círculo, 2 círculos por nó.
    assert!(
        (c.nivel - 0.9523809523809523).abs() <= EPS_TEMPORAL,
        "nível {}",
        c.nivel
    );
    assert_eq!(c.nos.len(), 96, "constellation nós: contagem");
    let volta = Comando::A(1.7, 1.7, 0.0, 1.0, 0.0, -3.4, 0.0);
    for (i, esperado) in [
        (0, Comando::M(513.925, 5.0)),
        (1, Comando::A(1.7, 1.7, 0.0, 1.0, 0.0, 3.4, 0.0)),
        (2, volta.clone()),
        (3, Comando::M(513.925, 43.0)),
        (95, volta),
    ] {
        assert_comandos(&c.nos[i..=i], &[esperado], &format!("nós [{i}]"));
    }
    assert_eq!(c.malha.len(), 64, "constellation malha: contagem");
    for (i, esperado) in [
        (0, Comando::M(15.625, 23.875)),
        (15, Comando::L(484.375, 23.875)),
        (16, Comando::L(515.625, 5.0)),
        (17, Comando::L(546.875, 4.75)),
        (31, Comando::L(984.375, 4.0)),
        (32, Comando::M(15.625, 24.125)),
        (47, Comando::L(484.375, 24.125)),
        (48, Comando::L(515.625, 43.0)),
        (63, Comando::L(984.375, 44.0)),
    ] {
        assert_comandos(&c.malha[i..=i], &[esperado], &format!("malha [{i}]"));
    }
}

#[test]
fn aurora_constellation_silencio_sem_nos_e_malha_em_24() {
    // Given: nenhuma amostra audível: spectrumBands zera as bandas.
    let (bandas, freq) = spectrum_bands(&db_sintetico(), false, NYQUIST, 84.0);
    assert_eq!(freq, 0.0, "frequência no silêncio");
    // When
    let c = geometria_constelacao(&bandas, 32, 1.7, 21.0);
    // Then: d de nós vazio e malha achatada em 24.
    assert_eq!(c.nivel, 0.0, "nível no silêncio");
    assert!(c.nos.is_empty(), "nós no silêncio: {:?}", c.nos);
    assert_eq!(c.malha.len(), 64, "malha no silêncio: contagem");
    for (i, esperado) in [
        (0, Comando::M(15.625, 24.0)),
        (31, Comando::L(984.375, 24.0)),
        (32, Comando::M(15.625, 24.0)),
        (63, Comando::L(984.375, 24.0)),
    ] {
        assert_comandos(
            &c.malha[i..=i],
            &[esperado],
            &format!("malha silêncio [{i}]"),
        );
    }
}

#[test]
fn estilos_de_arquivo_alturas_do_envelope() {
    // Given/When/Then: render (waveform-styles.js:93) sem peakHeight/bodyHeight
    // em defaultConfigs: 22 e 20, e 15 e 7 em overview.
    assert_eq!(alturas(EstiloArquivo::Original), (22.0, 20.0));
    assert_eq!(alturas(EstiloArquivo::Dj), (22.0, 20.0));
    assert_eq!(alturas(EstiloArquivo::Overview), (15.0, 7.0));
}

/// Envelope de três valores (x = 0, 500, 1000) com as ordenadas de cima
/// `c0, c1, c2` e as de baixo `b0, b1, b2`.
fn envelope_de_tres(c0: f64, c1: f64, c2: f64, b0: f64, b1: f64, b2: f64) -> Vec<Comando> {
    vec![
        Comando::M(0.0, 24.0),
        Comando::L(0.0, c0),
        Comando::L(500.0, c1),
        Comando::L(1000.0, c2),
        Comando::L(1000.0, 24.0),
        Comando::L(1000.0, b2),
        Comando::L(500.0, b1),
        Comando::L(0.0, b0),
        Comando::Z,
    ]
}

#[test]
fn estilos_de_arquivo_geometria_de_picos_e_corpo() {
    // Given: picos [0.25, 1.5, −0.5] e rms [0.1, 0.6, 0.3] (recorte em [0, 1]).
    let picos = [0.25, 1.5, -0.5];
    let rms = [0.1, 0.6, 0.3];
    for (estilo, pico, corpo) in [
        (
            EstiloArquivo::Original,
            envelope_de_tres(18.5, 2.0, 24.0, 29.5, 46.0, 24.0),
            envelope_de_tres(22.0, 12.0, 18.0, 26.0, 36.0, 30.0),
        ),
        (
            EstiloArquivo::Dj,
            envelope_de_tres(18.5, 2.0, 24.0, 29.5, 46.0, 24.0),
            envelope_de_tres(22.0, 12.0, 18.0, 26.0, 36.0, 30.0),
        ),
        (
            EstiloArquivo::Overview,
            envelope_de_tres(20.25, 9.0, 24.0, 27.75, 39.0, 24.0),
            envelope_de_tres(23.3, 19.8, 21.9, 24.7, 28.2, 26.1),
        ),
    ] {
        // When
        let (p, b) = geometria_arquivo(estilo, &picos, &rms);
        // Then
        assert_comandos(&p, &pico, &format!("{estilo:?} picos"));
        assert_comandos(&b, &corpo, &format!("{estilo:?} corpo"));
    }
}

#[test]
fn envelope_vazio_nao_tem_comandos() {
    // Given: values vazio. When/Then: `if(!values.length)return ''`
    // (waveform-styles.js:64): nenhum comando.
    assert_eq!(envelope(&[], 22.0), Vec::<Comando>::new());
}

#[test]
fn envelope_nan_vale_zero() {
    // Given: valor NaN. When/Then: `Number(values[i])||0` o troca por 0
    // (waveform-styles.js:64), então a ordenada fica em 24.
    let d = envelope(&[f64::NAN], 10.0);
    assert_comandos(
        &d,
        &[
            Comando::M(0.0, 24.0),
            Comando::L(0.0, 24.0),
            Comando::L(1000.0, 24.0),
            Comando::L(0.0, 24.0),
            Comando::Z,
        ],
        "envelope NaN",
    );
}
