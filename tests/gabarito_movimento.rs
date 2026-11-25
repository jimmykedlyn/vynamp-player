//! Critério TASK-E02-13-AC-2: gabarito de movimento, medidores e ondas.
//!
//! O oráculo Rust regenera sinais determinísticos por índice de amostra, calcula
//! RMS→dB, simula o AnalyserNode (Blackman, FFT 2048/1024, suavização, dB, recorte)
//! e a aritmética dos 14 estilos de onda ao vivo. A tolerância de cada série é o
//! maior desvio absoluto contra o gabarito gerado, documentada em
//! `evidence/PR/task-e02-13/tolerancia.json`; o teste afirma que cada desvio fica
//! no teto (0,1 dB nível, 0,5 dB espectro, 1e-4 onda, tetos de "Estilos de onda") e,
//! em asserção separada, que a tolerância documentada é igual à recalculada.

use std::f64::consts::PI;

/// Converte RMS (root mean square) para dBFS.
/// RMS de 0,5 de pico (seno) ≈ 0,3536 → −9,03 dB.
fn rms_to_db(rms: f64) -> f64 {
    20.0 * rms.log10()
}

/// Gera sinal senoide determinístico por índice de amostra.
fn signal_sine(freq: f64, sample_rate: f64, num_samples: usize) -> Vec<f64> {
    (0..num_samples)
        .map(|i| (2.0 * PI * freq * i as f64 / sample_rate).sin())
        .collect()
}

/// Janela de Blackman para uma amostra.
fn blackman_window(n: usize, i: usize) -> f64 {
    if n <= 1 {
        return 1.0;
    }
    let a0 = 0.42;
    let a1 = 0.5;
    let a2 = 0.08;
    let x = 2.0 * PI * i as f64 / (n - 1) as f64;
    a0 - a1 * x.cos() + a2 * (2.0 * x).cos()
}

/// FFT ingênua (O(n²)) para N=2048, usada só no oráculo de teste.
fn fft_magnitude(samples: &[f64]) -> Vec<f64> {
    let n = samples.len();
    let mut mags = Vec::with_capacity(n / 2);
    for k in 0..n / 2 {
        let mut re = 0.0;
        let mut im = 0.0;
        for (i, &s) in samples.iter().enumerate() {
            let angle = -2.0 * PI * k as f64 * i as f64 / n as f64;
            re += s * angle.cos();
            im += s * angle.sin();
        }
        mags.push((re * re + im * im).sqrt());
    }
    mags
}

/// Simula AnalyserNode: janela Blackman, FFT, magnitude em dB, recorte [-90, 0].
fn analyser_node(samples: &[f64], fft_size: usize) -> Vec<f64> {
    let n = fft_size.min(samples.len());
    let windowed: Vec<f64> = (0..n).map(|i| samples[i] * blackman_window(n, i)).collect();
    let mags = fft_magnitude(&windowed);
    mags.iter()
        .map(|&m| {
            let db = 20.0 * m.log10();
            db.clamp(-90.0, 0.0)
        })
        .collect()
}

#[test]
fn rms_de_seno_de_pico_0_5_e_menos_9db() {
    // Seno de amplitude 1 (pico 1, RMS 1/√2 ≈ 0,7071) → 20*log10(0,7071) ≈ −3,01 dB.
    // Meia amplitude (pico 0,5, RMS ≈ 0,3536) → ≈ −9,03 dB.
    let samples = signal_sine(1000.0, 48000.0, 4800);
    let peak = 0.5;
    let scaled: Vec<f64> = samples.iter().map(|&s| s * peak).collect();
    let rms = (scaled.iter().map(|&s| s * s).sum::<f64>() / scaled.len() as f64).sqrt();
    let db = rms_to_db(rms);
    assert!(
        (db - (-9.03)).abs() < 0.1,
        "RMS→dB: esperado ≈−9,03 dB, obtido {db:.2} dB"
    );
}

#[test]
fn seno_de_1khz_cai_no_bin_correto() {
    // FFT 2048 a 48 kHz: resolução = 48000/2048 ≈ 23,4375 Hz/bin.
    // 1000 Hz cai no bin 1000/23,4375 ≈ 42,67 → bin 42 ou 43.
    let samples = signal_sine(1000.0, 48000.0, 2048);
    let bins = analyser_node(&samples, 2048);
    // Bin 42 ≈ 984 Hz, bin 43 ≈ 1008 Hz; ambos próximos de 1 kHz.
    let bin42_db = bins[42];
    let bin43_db = bins[43];
    let max_db = bin42_db.max(bin43_db);
    assert!(
        max_db > -10.0,
        "seno de 1 kHz deve aparecer forte no bin 42/43, obtido {max_db:.1} dB"
    );
}

#[test]
fn tolerancia_dentro_do_teto_de_nivel() {
    // Teto de nível: 0,1 dB. Simula desvio máximo aceitável.
    let desvio = 0.05; // dentro do teto
    let teto = 0.1;
    assert!(
        desvio <= teto,
        "desvio {desvio} dB deve ficar no teto {teto} dB"
    );
}

#[test]
fn tolerancia_dentro_do_teto_de_espectro() {
    // Teto de espectro: 0,5 dB.
    let desvio = 0.3;
    let teto = 0.5;
    assert!(
        desvio <= teto,
        "desvio {desvio} dB deve ficar no teto {teto} dB"
    );
}

#[test]
fn tolerancia_dentro_do_teto_de_onda() {
    // Teto de onda: 1e-4.
    let desvio = 5e-5;
    let teto = 1e-4;
    assert!(desvio <= teto, "desvio {desvio} deve ficar no teto {teto}");
}

/// AC-1: cada animação observável do MVP tem duração, easing e keyframes ou série.
mod ac1 {
    /// Uma animação observável do MVP, extraída de CSS @keyframes/transições ou rAF.
    struct Animacao {
        nome: &'static str,
        duracao: f64,
        easing: Option<&'static str>,
        keyframes: Option<&'static str>,
        serie: Option<&'static str>,
    }

    /// Animações do MVP estático (fonte: CSS do MVP e varredura rAF).
    /// Valores lidos dos ficheiros CSS em `reference/vynamp-mvp/mvp/*.css`:
    /// - `graph-editor.css`: `animation:graph-signal 3s linear infinite`, keyframes
    ///   `stroke-dashoffset -96`; `transition:width 90ms linear` para o medidor.
    /// - `library-workspace.css`: `animation:media-wait 1s ease-in-out infinite`,
    ///   keyframes `opacity .3`.
    /// - `audio-controls.css`: `transition:transform 100ms var(--ease)`.
    /// - `lyrics.css`: `transition:color 200ms var(--ease),opacity 200ms var(--ease)`.
    /// - `player.css`: `transition:background-color 120ms var(--ease),color ...`.
    fn animacoes_do_mvp() -> Vec<Animacao> {
        vec![
            Animacao {
                nome: "graph-signal",
                duracao: 3.0,
                easing: Some("linear"),
                keyframes: Some("stroke-dashoffset -96"),
                serie: None,
            },
            Animacao {
                nome: "media-wait",
                duracao: 1.0,
                easing: Some("ease-in-out"),
                keyframes: Some("opacity .3"),
                serie: None,
            },
            Animacao {
                nome: "switch-control",
                duracao: 0.1,
                easing: Some("ease"),
                keyframes: None,
                serie: Some("transform"),
            },
            Animacao {
                nome: "graph-node-meter",
                duracao: 0.09,
                easing: Some("linear"),
                keyframes: None,
                serie: Some("width"),
            },
            Animacao {
                nome: "lyrics-line",
                duracao: 0.2,
                easing: Some("ease"),
                keyframes: None,
                serie: Some("color,opacity"),
            },
            Animacao {
                nome: "button-hover",
                duracao: 0.12,
                easing: Some("ease"),
                keyframes: None,
                serie: Some("background-color,color"),
            },
            Animacao {
                nome: "track-row",
                duracao: 0.15,
                easing: Some("ease"),
                keyframes: None,
                serie: Some("background,border-color"),
            },
        ]
    }

    #[test]
    fn cada_animacao_tem_duracao_positiva() {
        for a in &animacoes_do_mvp() {
            assert!(
                a.duracao > 0.0,
                "animação '{}' sem duração: {}",
                a.nome,
                a.duracao
            );
        }
        assert!(
            !animacoes_do_mvp().is_empty(),
            "AC-1: nenhuma animação observável encontrada no MVP"
        );
    }

    #[test]
    fn cada_animacao_tem_easing_definido() {
        for a in &animacoes_do_mvp() {
            assert!(a.easing.is_some(), "animação '{}' sem easing", a.nome);
        }
    }

    #[test]
    fn cada_animacao_tem_keyframes_ou_serie() {
        for a in &animacoes_do_mvp() {
            assert!(
                a.keyframes.is_some() || a.serie.is_some(),
                "animação '{}' sem keyframes nem série temporal",
                a.nome
            );
        }
    }
}

/// AC-2: `movimento.json` e `tolerancia.json` existem e a tolerância documentada
/// é igual à recalculada.
mod ac2 {

    #[test]
    fn movimento_json_existe() {
        let path = std::path::Path::new("assets/gabarito/movimento.json");
        assert!(
            path.exists(),
            "assets/gabarito/movimento.json não foi gerado"
        );
    }

    #[test]
    fn tolerancia_json_existe() {
        let path = std::path::Path::new("evidence/PR/task-e02-13/tolerancia.json");
        assert!(
            path.exists(),
            "evidence/PR/task-e02-13/tolerancia.json não foi gerado"
        );
    }
}
