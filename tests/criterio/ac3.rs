//! AC-3: Web Audio entra só como comportamento reimplementável.
//!
//! As listas esperadas vêm do código do MVP no commit `0d9860b9`: os 17 estilos de
//! `defaultConfigs` (waveform-styles.js:7-25) e `liveModes` (waveform-styles.js:66), e os
//! 46 usos de Web Audio da varredura dos `.js`, não do próprio JSON.

use crate::criterio::json::{campo, gabarito, lista, objeto, sem_falhas, texto, textos};
use crate::criterio::medicao::ALVOS;
use serde_json::{Value, json};
use std::collections::BTreeSet;

/// Os 17 estilos de `defaultConfigs`, com os parâmetros literais e se estão em
/// `liveModes`.
fn estilos_do_mvp() -> Vec<(&'static str, Value, bool)> {
    vec![
        ("original", json!({"waveColor": "inherit"}), false),
        (
            "dj",
            json!({"hotColor": "orange", "coreColor": "blue"}),
            false,
        ),
        (
            "overview",
            json!({"hotColor": "orange", "coreColor": "orange"}),
            false,
        ),
        ("pulse", json!({"amplitude": 22, "samples": 256}), true),
        (
            "bars-pulse",
            json!({"bars": 128, "gain": 1.0, "smoothing": 0.3}),
            true,
        ),
        (
            "columns-pulse",
            json!({"columns": 128, "gain": 1.4, "height": 21}),
            true,
        ),
        (
            "ribbon-pulse",
            json!({"segments": 64, "gain": 2.0, "height": 21}),
            true,
        ),
        (
            "prisma-neon",
            json!({"columns": 128, "gain": 1.4, "height": 21}),
            true,
        ),
        (
            "aurora-neon",
            json!({"samples": 256, "amplitude": 22}),
            true,
        ),
        (
            "spectrum-aurora",
            json!({"bands": 128, "height": 21, "dbRange": 84}),
            true,
        ),
        (
            "spectrum-ribbon",
            json!({"bands": 128, "height": 21, "dbRange": 84}),
            true,
        ),
        (
            "spectrum-stereo",
            json!({"bands": 128, "height": 21, "dbRange": 84}),
            true,
        ),
        (
            "spectrum-pulse",
            json!({"bars": 64, "height": 42, "peakDecay": 0.02}),
            true,
        ),
        (
            "aurora-weave",
            json!({"samples": 256, "amplitude": 22}),
            true,
        ),
        (
            "neon-crystal",
            json!({"segments": 32, "peakGain": 2.0, "rmsGain": 2.0}),
            true,
        ),
        (
            "aurora-constellation",
            json!({"nodes": 32, "nodeRadius": 1.7, "height": 21}),
            true,
        ),
        (
            "aurora-layers",
            json!({"segments": 64, "peakHeight": 21, "rmsHeight": 16}),
            true,
        ),
    ]
}

/// Envelope do arquivo (waveform-styles.js:93): `peakHeight`/`bodyHeight` padrão.
fn envelope_do_mvp(estilo: &str) -> Value {
    let (pico, corpo) = if estilo == "overview" {
        (15, 7)
    } else {
        (22, 20)
    };
    json!({"peakHeight": pico, "bodyHeight": corpo})
}

/// Usos de Web Audio nos `.js` do MVP, por arquivo e linha, com as APIs da linha.
const WEB_AUDIO: [(&str, u64, &[&str]); 46] = [
    (
        "advanced-dsp.js",
        251,
        &["AudioWorkletProcessor", "registerProcessor"],
    ),
    ("advanced-eq.js", 19, &["new OfflineAudioContext"]),
    ("advanced-eq.js", 77, &[".createBiquadFilter("]),
    ("advanced-eq.js", 80, &["getFrequencyResponse"]),
    ("dsp-ir-assets.js", 7, &["decodeAudioData"]),
    (
        "dsp-workspace.js",
        114,
        &["new OfflineAudioContext", "decodeAudioData"],
    ),
    (
        "dynamics-dsp.js",
        8,
        &["AudioWorkletProcessor", "registerProcessor"],
    ),
    ("graph-audio.js", 7, &["audioWorklet"]),
    ("graph-audio.js", 14, &["audioWorklet"]),
    ("graph-audio.js", 35, &[".createGain("]),
    (
        "graph-audio.js",
        40,
        &["setValueAtTime", "linearRampToValueAtTime"],
    ),
    ("graph-audio.js", 42, &[".createBiquadFilter("]),
    (
        "graph-audio.js",
        43,
        &[".createChannelSplitter(", ".createChannelMerger("],
    ),
    (
        "graph-audio.js",
        44,
        &[".createChannelSplitter(", ".createAnalyser("],
    ),
    ("graph-audio.js", 47, &[".createOscillator("]),
    ("graph-audio.js", 49, &[".createDynamicsCompressor("]),
    ("graph-audio.js", 51, &["AudioWorkletNode"]),
    (
        "graph-audio.js",
        53,
        &[".createChannelSplitter(", ".createChannelMerger("],
    ),
    ("graph-audio.js", 56, &[".createStereoPanner("]),
    (
        "graph-audio.js",
        57,
        &[
            ".createChannelSplitter(",
            ".createChannelMerger(",
            ".createDelay(",
        ],
    ),
    (
        "graph-audio.js",
        58,
        &[
            ".createChannelSplitter(",
            ".createChannelMerger(",
            ".createDelay(",
        ],
    ),
    ("graph-audio.js", 59, &[".createDelay("]),
    (
        "graph-audio.js",
        60,
        &[
            ".createDelay(",
            ".createChannelSplitter(",
            ".createChannelMerger(",
            ".createConvolver(",
            ".createBuffer(",
        ],
    ),
    ("graph-audio.js", 61, &["AudioWorkletNode"]),
    ("graph-audio.js", 62, &["AudioWorkletNode"]),
    (
        "graph-audio.js",
        63,
        &[".createDelay(", ".createOscillator("],
    ),
    (
        "graph-audio.js",
        64,
        &[".createDelay(", ".createConvolver("],
    ),
    ("graph-audio.js", 67, &[".createWaveShaper("]),
    ("graph-audio.js", 69, &[".createWaveShaper("]),
    ("graph-audio.js", 75, &[".createDelay("]),
    (
        "graph-audio.js",
        78,
        &["getFloatTimeDomainData", "getByteFrequencyData"],
    ),
    ("graph-audio.js", 82, &[".createGain(", ".createDelay("]),
    (
        "graph-audio.js",
        83,
        &["setValueAtTime", "linearRampToValueAtTime"],
    ),
    ("graph-audio.js", 86, &[".createGain("]),
    ("graph-properties.js", 18, &["getFloatTimeDomainData"]),
    ("graph-properties.js", 19, &["getFloatFrequencyData"]),
    (
        "player.js",
        61,
        &[
            "new AudioContext",
            ".createGain(",
            ".createMediaElementSource(",
            ".createChannelSplitter(",
            ".createAnalyser(",
        ],
    ),
    (
        "player.js",
        70,
        &["getFloatTimeDomainData", "getFloatFrequencyData"],
    ),
    ("player.js", 73, &["decodeAudioData"]),
    (
        "player.js",
        205,
        &["new OfflineAudioContext", ".createBiquadFilter("],
    ),
    ("player.js", 210, &["getFrequencyResponse"]),
    (
        "player.js",
        243,
        &["getFloatFrequencyData", "getFloatTimeDomainData"],
    ),
    (
        "restore-dsp.js",
        391,
        &["AudioWorkletProcessor", "registerProcessor"],
    ),
    (
        "restore-engine.js",
        5,
        &[
            "audioWorklet",
            "AudioWorkletNode",
            ".createGain(",
            "setValueAtTime",
            "linearRampToValueAtTime",
        ],
    ),
    (
        "restore.js",
        145,
        &[
            "new OfflineAudioContext",
            ".createBuffer(",
            ".createBufferSource(",
            "startRendering",
        ],
    ),
    ("restore.js", 275, &["decodeAudioData"]),
];

#[test]
fn ac3_cada_estilo_tem_parametros_literais_e_serie_ou_motivo() -> Result<(), String> {
    // Given os 17 estilos de defaultConfigs; When o gabarito os descreve; Then cada um
    // tem os parâmetros literais, os 14 de liveModes apontam a execução medida e os 3 de
    // arquivo dizem por que não têm série, com o envelope que o código fixa.
    let lidos = lista(campo(gabarito()?, "estilos", "gabarito")?, "estilos")?;
    let esperados = estilos_do_mvp();
    let mut f = Vec::new();
    let nomes: Vec<&str> = lidos
        .iter()
        .map(|e| e.get("estilo").and_then(Value::as_str).unwrap_or("?"))
        .collect();
    let nomes_esperados: Vec<&str> = esperados.iter().map(|e| e.0).collect();
    if nomes != nomes_esperados {
        f.push(format!("estilos {nomes:?}, esperados {nomes_esperados:?}"));
        return sem_falhas(f);
    }
    let medidos: BTreeSet<&str> = ALVOS
        .iter()
        .filter(|a| !a.medidor)
        .map(|a| a.nome)
        .collect();
    for (e, (nome, parametros, ao_vivo)) in lidos.iter().zip(&esperados) {
        if campo(e, "parametros", nome)? != parametros {
            f.push(format!(
                "{nome}: parâmetros {}, esperados {parametros}",
                e["parametros"]
            ));
        }
        if campo(e, "aoVivo", nome)?.as_bool() != Some(*ao_vivo) {
            f.push(format!("{nome}: aoVivo diferente de {ao_vivo}"));
        }
        if *ao_vivo {
            if !medidos.contains(nome) {
                f.push(format!("{nome}: ao vivo e sem execução no protocolo"));
            }
            if texto(campo(e, "execucao", nome)?, nome)? != *nome {
                f.push(format!("{nome}: execução diferente de {nome}"));
            }
        } else {
            if texto(campo(e, "motivo", nome)?, nome)?.is_empty() {
                f.push(format!("{nome}: estilo de arquivo sem motivo"));
            }
            if *campo(e, "envelope", nome)? != envelope_do_mvp(nome) {
                f.push(format!(
                    "{nome}: envelope diferente de {}",
                    envelope_do_mvp(nome)
                ));
            }
        }
    }
    let pulso = &lidos[12];
    if textos(
        campo(pulso, "parametrosSemEfeito", "spectrum-pulse")?,
        "spectrum-pulse",
    )? != ["peakDecay"]
    {
        f.push("spectrum-pulse: peakDecay não está como parâmetro sem efeito".into());
    }
    let barras = &lidos[4];
    let radio = objeto(
        campo(barras, "ramoRadio", "bars-pulse")?,
        "bars-pulse.ramoRadio",
    )?;
    if radio
        .get("motivo")
        .and_then(Value::as_str)
        .unwrap_or("")
        .is_empty()
        || radio.get("parametros") != Some(&json!({"bars": 128, "gain": 1.0, "altura": 22}))
    {
        f.push(format!(
            "bars-pulse: ramo de rádio sem motivo ou sem bars 128, gain 1, altura 22: {}",
            barras["ramoRadio"]
        ));
    }
    sem_falhas(f)
}

#[test]
fn ac3_cada_uso_de_web_audio_vira_comportamento() -> Result<(), String> {
    // Given os 46 usos de Web Audio da varredura; When o gabarito os descreve; Then cada
    // um tem as APIs da linha e um comportamento escrito, sem nó do navegador como dado.
    let lidos = lista(campo(gabarito()?, "webAudio", "gabarito")?, "webAudio")?;
    let mut f = Vec::new();
    let origens: Vec<String> = lidos
        .iter()
        .map(|u| {
            u.get("origem")
                .and_then(Value::as_str)
                .unwrap_or("?")
                .to_string()
        })
        .collect();
    let esperadas: Vec<String> = WEB_AUDIO
        .iter()
        .map(|(a, l, _)| format!("{a}:{l}"))
        .collect();
    if origens != esperadas {
        f.push(format!("origens {origens:?}, esperadas {esperadas:?}"));
        return sem_falhas(f);
    }
    for (u, (_, _, apis)) in lidos.iter().zip(&WEB_AUDIO) {
        let ctx = u.get("origem").and_then(Value::as_str).unwrap_or("?");
        if textos(campo(u, "apis", ctx)?, ctx)? != *apis {
            f.push(format!("{ctx}: apis {}, esperadas {apis:?}", u["apis"]));
        }
        if texto(campo(u, "comportamento", ctx)?, ctx)?
            .trim()
            .is_empty()
        {
            f.push(format!("{ctx}: comportamento vazio"));
        }
    }
    sem_falhas(f)
}
