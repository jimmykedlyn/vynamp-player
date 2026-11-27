//! AC-2: recalcula, pelo oráculo, cada série do gabarito e mede o desvio contra o MVP.
//!
//! A lista esperada (execuções, leituras, analisadores, chaves e contagens) vem do
//! "Protocolo de medição" e de "Estilos de onda" da spec, e do código do MVP no commit
//! `0d9860b9`: `player.js:61` (grafo), `player.js:70` (`frame`), `player.js:243`
//! (`copyLiveSpectrum`, `copyPulseSpectrum`, `copyLiveSamples`), `waveform-styles.js:7-25`
//! (`defaultConfigs`), `:69-72`, `:76`, `:79`, `:80`, `:84-87` e `:89` (`pulseFrame`).

use crate::criterio::json::{campo, inteiro, lista, numero, objeto, texto};
use crate::criterio::svg::{self, Tetos};
use crate::oraculo::analisador::{bins_hifi, blackman};
use crate::oraculo::estilos_criativos::{
    geometria_aurora_weave, geometria_camadas, geometria_constelacao,
};
use crate::oraculo::estilos_espectro::{
    Comando, SpectrumPulse, audivel, geometria_spectrum, geometria_spectrum_pulse, merge,
    spectrum_bands,
};
use crate::oraculo::estilos_temporais::{
    BarsPulse, data_peak, geometria_aurora_neon, geometria_colunas, geometria_pulse,
    geometria_ribbon,
};
use crate::oraculo::grade::{
    amostra_suspensao, quadros_medidor, quadros_pulse_33, quadros_spectrum_pulse, t_k,
};
use crate::oraculo::sinal::{Tipo, sinal};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// SHA do MVP fixado (item 10 do protocolo).
pub const MVP_COMMIT: &str = "0d9860b901510021bcca4c31a25014373ea7c452";

/// `fftSize` de todos os analisadores do `setupAudio` (player.js:61).
pub const FFT: usize = 2048;

/// Nyquist do contexto de 48 kHz (item 2 do protocolo).
const NYQUIST: f64 = 24000.0;

/// Sinais do item 3 do protocolo, na ordem do gerador.
pub const SINAIS: [(&str, Tipo); 3] = [
    ("degrau", Tipo::Degrau),
    ("seno", Tipo::Seno),
    ("rajada", Tipo::Rajada),
];

/// Método do `AnalyserNode` lido.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Metodo {
    /// `getFloatTimeDomainData`.
    Tempo,
    /// `getFloatFrequencyData` (avança o `smoothingTimeConstant`).
    Frequencia,
}

impl Metodo {
    /// Nome do método na Web Audio API, como o gerador o grava.
    pub fn nome(self) -> &'static str {
        match self {
            Metodo::Tempo => "getFloatTimeDomainData",
            Metodo::Frequencia => "getFloatFrequencyData",
        }
    }
}

use Metodo::{Frequencia as F, Tempo as T};

/// `smoothingTimeConstant` por `ordem`: 0 e 1 são `analysers` (0,75), 2 e 3 são
/// `pulseAnalysers` (0), de `setupAudio` (player.js:61).
pub const SUAVIZACAO: [f64; 4] = [0.75, 0.75, 0.0, 0.0];

/// Uma execução-alvo: medidor ou estilo de onda ao vivo.
pub struct Alvo {
    /// Id do medidor ou chave de `defaultConfigs`.
    pub nome: &'static str,
    /// `true` nos medidores (cadência de `frame`), `false` nos estilos (`pulseFrame`).
    pub medidor: bool,
    /// Leituras de analisador de cada quadro lido, em ordem: `(ordem, método)`.
    pub leituras: &'static [(u64, Metodo)],
    /// Chaves das séries, `atributo@elemento/canal` (ou `série/canal` nos medidores).
    pub series: &'static [&'static str],
}

/// `frame` lê, por analisador, o tempo e depois a frequência (player.js:70).
const L_MEDIDOR: &[(u64, Metodo)] = &[(0, T), (0, F), (1, T), (1, F)];
/// `copyLiveSamples(samples)` do canal 0 (waveform-styles.js:89, :85).
const L_TEMPORAL: &[(u64, Metodo)] = &[(0, T)];
/// `spectrumBands(0)`: `copyLiveSpectrum` e depois `copyLiveSamples` (waveform-styles.js:76).
const L_BANDAS: &[(u64, Metodo)] = &[(0, F), (0, T)];
/// `spectrumBands(0)` e `spectrumBands(1)` em `spectrum-stereo` (waveform-styles.js:80).
const L_ESTEREO: &[(u64, Metodo)] = &[(0, F), (0, T), (1, F), (1, T)];
/// `copyPulseSpectrum` dos dois canais (waveform-styles.js:79).
const L_PULSO: &[(u64, Metodo)] = &[(2, F), (3, F)];
/// `copyLiveSamples` dos dois canais em `aurora-weave` (waveform-styles.js:85).
const L_WEAVE: &[(u64, Metodo)] = &[(0, T), (1, T)];

const S_BANDAS_L: &[&str] = &[
    "data-level@wave-spectrum-live/L",
    "data-frequency-left@wave-spectrum-live/L",
];
const S_CAMADAS: &[&str] = &[
    "data-level@wave-creative-live/L",
    "d@wave-creative-outer/L",
    "d@wave-creative-middle/L",
    "d@wave-creative-inner/L",
];

/// Os 2 medidores e os 14 estilos de `liveModes` (waveform-styles.js:66), na ordem de
/// `defaultConfigs` (waveform-styles.js:7-25). As séries são as do item 6 de
/// "Estilos de onda".
pub const ALVOS: [Alvo; 16] = [
    Alvo {
        nome: "06-hifi-spectrum",
        medidor: true,
        leituras: L_MEDIDOR,
        series: &["nivel/L", "nivel/R", "espectro/L", "espectro/R"],
    },
    Alvo {
        nome: "12-osciloscopio-crt",
        medidor: true,
        leituras: L_MEDIDOR,
        series: &["nivel/L", "nivel/R", "onda/L", "onda/R"],
    },
    Alvo {
        nome: "pulse",
        medidor: false,
        leituras: L_TEMPORAL,
        series: &["d@wave-pulse/L"],
    },
    Alvo {
        nome: "bars-pulse",
        medidor: false,
        leituras: L_TEMPORAL,
        series: &[
            "data-level@wave-reactive-bars/L",
            "transform@wave-reactive-bars/L",
        ],
    },
    Alvo {
        nome: "columns-pulse",
        medidor: false,
        leituras: L_TEMPORAL,
        series: &["data-peak@wave-live-styles/L", "d@wave-live-columns/L"],
    },
    Alvo {
        nome: "ribbon-pulse",
        medidor: false,
        leituras: L_TEMPORAL,
        series: &["data-peak@wave-live-styles/L", "d@wave-live-fill/L"],
    },
    Alvo {
        nome: "prisma-neon",
        medidor: false,
        leituras: L_TEMPORAL,
        series: &["data-peak@wave-live-styles/L", "d@wave-live-columns/L"],
    },
    Alvo {
        nome: "aurora-neon",
        medidor: false,
        leituras: L_TEMPORAL,
        series: &[
            "data-peak@wave-live-styles/L",
            "d@wave-live-line/L",
            "d@wave-live-halo/L",
        ],
    },
    Alvo {
        nome: "spectrum-aurora",
        medidor: false,
        leituras: L_BANDAS,
        series: &[
            S_BANDAS_L[0],
            S_BANDAS_L[1],
            "d@wave-spectrum-left/L",
            "d@wave-spectrum-halo/L",
        ],
    },
    Alvo {
        nome: "spectrum-ribbon",
        medidor: false,
        leituras: L_BANDAS,
        series: &[S_BANDAS_L[0], S_BANDAS_L[1], "d@wave-spectrum-fill/L"],
    },
    Alvo {
        nome: "spectrum-stereo",
        medidor: false,
        leituras: L_ESTEREO,
        series: &[
            "data-level@wave-spectrum-live/LR",
            "data-frequency-left@wave-spectrum-live/L",
            "data-frequency-right@wave-spectrum-live/R",
            "d@wave-spectrum-left/L",
            "d@wave-spectrum-right/R",
        ],
    },
    Alvo {
        nome: "spectrum-pulse",
        medidor: false,
        leituras: L_PULSO,
        series: &[
            "data-level@wave-spectrum-live/LR",
            "d@wave-spectrum-fill/LR",
            "d@wave-spectrum-halo/LR",
        ],
    },
    Alvo {
        nome: "aurora-weave",
        medidor: false,
        leituras: L_WEAVE,
        series: &[
            "data-level@wave-creative-live/LR",
            "d@wave-creative-left/L",
            "d@wave-creative-right/R",
        ],
    },
    Alvo {
        nome: "neon-crystal",
        medidor: false,
        leituras: L_TEMPORAL,
        series: S_CAMADAS,
    },
    Alvo {
        nome: "aurora-constellation",
        medidor: false,
        leituras: L_BANDAS,
        series: &[
            "data-level@wave-creative-live/L",
            "data-frequency-left@wave-spectrum-live/L",
            "d@wave-creative-nodes/L",
            "d@wave-creative-mesh/L",
        ],
    },
    Alvo {
        nome: "aurora-layers",
        medidor: false,
        leituras: L_TEMPORAL,
        series: S_CAMADAS,
    },
];

impl Alvo {
    /// Quadros lidos: cadência de `frame` (item 5 do protocolo) ou de `pulseFrame`
    /// (item 5 de "Estilos de onda").
    pub fn quadros(&self) -> Vec<u64> {
        if self.medidor {
            quadros_medidor()
        } else if self.nome == "spectrum-pulse" {
            quadros_spectrum_pulse()
        } else {
            quadros_pulse_33()
        }
    }

    /// Contagem literal do item 5 do protocolo: 42, 120 ou 59.
    pub fn contagem(&self) -> usize {
        if self.medidor {
            42
        } else if self.nome == "spectrum-pulse" {
            120
        } else {
            59
        }
    }

    /// Analisadores que alimentam a série, pelo canal do sufixo (`L`, `R` ou `LR`):
    /// `(canal, smoothingTimeConstant)`.
    pub fn analisadores_da_serie(&self, chave: &str) -> Vec<(&'static str, f64)> {
        let suavizacao = if self.nome == "spectrum-pulse" {
            SUAVIZACAO[2]
        } else {
            SUAVIZACAO[0]
        };
        let canais: &[&'static str] = match chave.rsplit('/').next() {
            Some("LR") => &["L", "R"],
            Some("R") => &["R"],
            _ => &["L"],
        };
        canais.iter().map(|c| (*c, suavizacao)).collect()
    }
}

/// Ids das 48 execuções, na ordem do gerador: medidores e depois estilos, cada um
/// pelos três sinais (item 4 do protocolo).
pub fn execucoes_esperadas() -> Vec<(String, &'static Alvo, Tipo)> {
    ALVOS
        .iter()
        .flat_map(|a| {
            SINAIS
                .iter()
                .map(move |(nome, tipo)| (format!("{}:{nome}", a.nome), a, *tipo))
        })
        .collect()
}

/// Chave de `tolerancia.json`: `<execução>/<série>/<canal>` (item 10 do protocolo).
pub fn chave_tolerancia(execucao: &str, serie: &str) -> String {
    format!("{execucao}/{serie}")
}

/// Todas as chaves esperadas de `tolerancia.json`, derivadas do protocolo.
pub fn chaves_tolerancia() -> BTreeSet<String> {
    execucoes_esperadas()
        .iter()
        .flat_map(|(id, alvo, _)| alvo.series.iter().map(|s| chave_tolerancia(id, s)))
        .collect()
}

/// Bloco de 2048 amostras que termina em `fim` (exclusivo), com pré-roll zero antes da
/// amostra 0 (item 6 do protocolo): o `AnalyserNode` lê sempre `fftSize` amostras.
pub fn bloco_com_preroll(tipo: Tipo, fim: u64) -> Vec<f32> {
    let fim = fim as i64;
    (fim - FFT as i64..fim).map(|n| sinal(tipo, n)).collect()
}

/// Nível do medidor: RMS das 2048 amostras → `20·log10(max(1e-8, rms))`, recortado em
/// [−60, 0] dB (player.js:70).
pub fn nivel_preroll(bloco: &[f32]) -> f64 {
    let soma: f64 = bloco.iter().map(|&v| v as f64 * v as f64).sum();
    let rms = (soma / bloco.len() as f64).sqrt();
    (20.0 * libm::log10(rms.max(1e-8))).clamp(-60.0, 0.0)
}

/// `|X[k]|/N`, `k` em `0..N/2`, com a janela Blackman do oráculo, por FFT radix-2.
/// Mesma conta de `oraculo::analisador::magnitudes`, em O(N log N).
pub fn magnitudes_fft(bloco: &[f32]) -> Vec<f64> {
    let n = bloco.len();
    assert!(
        n.is_power_of_two(),
        "magnitudes_fft: {n} não é potência de 2; o fftSize do MVP é 2048"
    );
    let mut re: Vec<f64> = bloco
        .iter()
        .enumerate()
        .map(|(i, &s)| s as f64 * blackman(n, i))
        .collect();
    let mut im = vec![0.0f64; n];
    let bits = n.trailing_zeros();
    for i in 0..n {
        let j = i.reverse_bits() >> (usize::BITS - bits);
        if j > i {
            re.swap(i, j);
            im.swap(i, j);
        }
    }
    let mut tamanho = 2;
    while tamanho <= n {
        let passo = -2.0 * std::f64::consts::PI / tamanho as f64;
        for inicio in (0..n).step_by(tamanho) {
            for j in 0..tamanho / 2 {
                let (s, c) = libm::sincos(passo * j as f64);
                let a = inicio + j;
                let b = a + tamanho / 2;
                let tr = re[b] * c - im[b] * s;
                let ti = re[b] * s + im[b] * c;
                re[b] = re[a] - tr;
                im[b] = im[a] - ti;
                re[a] += tr;
                im[a] += ti;
            }
        }
        tamanho <<= 1;
    }
    (0..n / 2)
        .map(|k| (re[k] * re[k] + im[k] * im[k]).sqrt() / n as f64)
        .collect()
}

/// Valor esperado de uma série numa leitura, com o teto que o critério admite.
enum Esperado {
    /// Número (nível, `data-level`, `data-peak`).
    Escalar { valor: f64, teto: f64 },
    /// Vetor de números (espectro, onda).
    Vetor { valores: Vec<f64>, teto: f64 },
    /// `d` de um caminho SVG.
    Geometria {
        comandos: Vec<Comando>,
        tetos: Tetos,
    },
    /// `scale` do `transform` de `bars-pulse`.
    Escala { valor: f64, teto: f64 },
    /// `data-frequency-*`: espectro suavizado em dB do canal e o gate de áudio.
    Frequencia { db: Vec<f64>, disponivel: bool },
}

/// Prefixo e sufixo do `transform` de `scaleBars` (waveform-styles.js:65).
const TRANSFORM_ANTES: &str = "translate(0 24) scale(1 ";
const TRANSFORM_DEPOIS: &str = ") translate(0 -24)";

/// Número gravado como número ou como texto (`toFixed`, `String`).
fn numero_ou_texto(v: &Value, ctx: &str) -> Result<f64, String> {
    match v {
        Value::String(s) => s
            .parse::<f64>()
            .map_err(|e| format!("{ctx}: `{s}` não é número: {e}")),
        _ => numero(v, ctx),
    }
}

/// Compara um valor do MVP com o esperado: (desvio, violações).
fn comparar(esperado: &Esperado, mvp: &Value, ctx: &str) -> (f64, Vec<String>) {
    let falha = |e: String| (f64::INFINITY, vec![e]);
    let escalar = |valor: f64, lido: f64, teto: f64| {
        let d = (lido - valor).abs();
        if d > teto || d.is_nan() {
            (
                d,
                vec![format!(
                    "{ctx}: MVP {lido}, oráculo {valor}, desvio {d:.6e} > teto {teto:.6e}"
                )],
            )
        } else {
            (d, Vec::new())
        }
    };
    match esperado {
        Esperado::Escalar { valor, teto } => match numero_ou_texto(mvp, ctx) {
            Ok(lido) => escalar(*valor, lido, *teto),
            Err(e) => falha(e),
        },
        Esperado::Escala { valor, teto } => {
            let s = match texto(mvp, ctx) {
                Ok(s) => s,
                Err(e) => return falha(e),
            };
            match s
                .strip_prefix(TRANSFORM_ANTES)
                .and_then(|r| r.strip_suffix(TRANSFORM_DEPOIS))
                .map(str::parse::<f64>)
            {
                Some(Ok(lido)) => escalar(*valor, lido, *teto),
                _ => falha(format!(
                    "{ctx}: transform fora do formato de scaleBars: {s}"
                )),
            }
        }
        Esperado::Vetor { valores, teto } => {
            let lidos = match lista(mvp, ctx) {
                Ok(l) => l,
                Err(e) => return falha(e),
            };
            if lidos.len() != valores.len() {
                return falha(format!(
                    "{ctx}: {} valores no MVP, {} no oráculo",
                    lidos.len(),
                    valores.len()
                ));
            }
            let mut desvio = 0.0f64;
            let mut violacoes = Vec::new();
            for (i, (l, v)) in lidos.iter().zip(valores).enumerate() {
                match numero(l, &format!("{ctx}[{i}]")) {
                    Ok(lido) => {
                        let (d, mut e) = escalar(*v, lido, *teto);
                        desvio = desvio.max(d);
                        violacoes.extend(e.drain(..).map(|x| format!("[{i}] {x}")));
                    }
                    Err(e) => return falha(e),
                }
            }
            (desvio, violacoes)
        }
        Esperado::Geometria { comandos, tetos } => {
            let d = match texto(mvp, ctx).and_then(svg::ler_d) {
                Ok(d) => d,
                Err(e) => return falha(format!("{ctx}: {e}")),
            };
            let (desvio, violacoes) = svg::comparar(&d, comandos, *tetos);
            (
                desvio,
                violacoes
                    .into_iter()
                    .map(|v| format!("{ctx}: {v}"))
                    .collect(),
            )
        }
        Esperado::Frequencia { db, disponivel } => {
            let lido = match numero_ou_texto(mvp, ctx) {
                Ok(f) => f,
                Err(e) => return falha(e),
            };
            comparar_frequencia(db, *disponivel, lido, ctx)
        }
    }
}

/// `data-frequency-*` (item 6 de "Estilos de onda"): o bin do MVP é o de pico do
/// oráculo ou um bin a até 0,5 dB do pico; 0 quando não há amostra audível ou o maior
/// nível não passa de −90 dB. O desvio é a distância, em dB, até o pico.
fn comparar_frequencia(db: &[f64], disponivel: bool, lido: f64, ctx: &str) -> (f64, Vec<String>) {
    let mut max_db = f64::NEG_INFINITY;
    let mut pico = 0usize;
    for (j, &v) in db.iter().enumerate().skip(1) {
        if v > max_db {
            max_db = v;
            pico = j;
        }
    }
    let tem = disponivel && max_db > -90.0;
    match (tem, lido == 0.0) {
        (false, true) => (0.0, Vec::new()),
        (false, false) => (
            f64::INFINITY,
            vec![format!(
                "{ctx}: MVP {lido} Hz, oráculo 0 (audível {disponivel}, máximo {max_db} dB)"
            )],
        ),
        (true, true) => (
            f64::INFINITY,
            vec![format!(
                "{ctx}: MVP 0, oráculo pico no bin {pico} ({max_db} dB)"
            )],
        ),
        (true, false) => {
            let bin = lido * db.len() as f64 / NYQUIST;
            let inteiro = bin.round();
            if (bin - inteiro).abs() > 1e-9 || inteiro < 1.0 || inteiro as usize >= db.len() {
                return (
                    f64::INFINITY,
                    vec![format!("{ctx}: {lido} Hz não cai num bin ({bin})")],
                );
            }
            let gap = max_db - db[inteiro as usize];
            if gap > 0.5 || gap.is_nan() {
                (
                    gap,
                    vec![format!(
                        "{ctx}: bin do MVP {inteiro} a {gap:.6} dB do pico {pico}, teto 0,5 dB"
                    )],
                )
            } else {
                (gap, Vec::new())
            }
        }
    }
}

/// O que cada analisador devolveu num quadro, por `ordem`.
#[derive(Default)]
struct Lido {
    tempo: [Option<Vec<f32>>; 4],
    db: [Option<Vec<f64>>; 4],
}

/// Estado do oráculo ao longo de uma execução (um contexto novo por execução).
struct Estado {
    suavizado: [Vec<f64>; 4],
    barras: BarsPulse,
    pulso: SpectrumPulse,
}

impl Estado {
    fn novo() -> Self {
        Estado {
            suavizado: std::array::from_fn(|_| vec![0.0; FFT / 2]),
            barras: BarsPulse::new(),
            // `VynampSpectrumPulse.create(64)` (waveform-styles.js:78).
            pulso: SpectrumPulse::new(64),
        }
    }
}

/// Magnitudes por `(sinal, amostra)`: o sinal é o mesmo nos dois canais e em todas as
/// execuções do mesmo sinal, então a FFT é calculada uma vez.
type CacheFft = HashMap<(usize, u64), Vec<f64>>;

fn indice(tipo: Tipo) -> usize {
    match tipo {
        Tipo::Degrau => 0,
        Tipo::Seno => 1,
        Tipo::Rajada => 2,
    }
}

/// Executa as leituras de um quadro, na ordem gravada, e avança a suavização só nas
/// leituras de frequência (item 4 do protocolo).
fn ler_quadro(
    tipo: Tipo,
    amostra: u64,
    leituras: &[(u64, Metodo)],
    estado: &mut Estado,
    cache: &mut CacheFft,
) -> Lido {
    let mut lido = Lido::default();
    for &(ordem, metodo) in leituras {
        let o = ordem as usize;
        match metodo {
            Metodo::Tempo => lido.tempo[o] = Some(bloco_com_preroll(tipo, amostra)),
            Metodo::Frequencia => {
                let mags = cache
                    .entry((indice(tipo), amostra))
                    .or_insert_with(|| magnitudes_fft(&bloco_com_preroll(tipo, amostra)));
                let tau = SUAVIZACAO[o];
                for (s, &m) in estado.suavizado[o].iter_mut().zip(mags.iter()) {
                    *s = tau * *s + (1.0 - tau) * m;
                }
                lido.db[o] = Some(
                    estado.suavizado[o]
                        .iter()
                        .map(|x| 20.0 * libm::log10(*x))
                        .collect(),
                );
            }
        }
    }
    lido
}

/// Teto dos estilos de bandas e de `aurora-constellation`: `21·0,5/84`.
const ESCALA_BANDAS: f64 = 21.0 * 0.5 / 84.0;
/// Teto do `data-level` das bandas: `0,5/84 + 5e-5`.
const TETO_NIVEL_BANDAS: f64 = 0.5 / 84.0 + 5e-5;
/// Teto de `data-peak` e do `data-level` de weave, crystal e layers: `1e-4 + 5e-5`.
const TETO_PICO: f64 = 1e-4 + 5e-5;

/// Esperados de um quadro, por chave de série.
fn esperados(
    alvo: &Alvo,
    k: u64,
    lido: &Lido,
    estado: &mut Estado,
) -> Result<HashMap<String, Esperado>, String> {
    let tempo = |o: usize| {
        lido.tempo[o]
            .as_deref()
            .ok_or_else(|| format!("{}: sem leitura de tempo na ordem {o}", alvo.nome))
    };
    let db = |o: usize| {
        lido.db[o]
            .as_deref()
            .ok_or_else(|| format!("{}: sem leitura de frequência na ordem {o}", alvo.nome))
    };
    let geo = |comandos: Vec<Comando>, escala: f64, fator_v: f64| Esperado::Geometria {
        comandos,
        tetos: Tetos { escala, fator_v },
    };
    // `spectrumBands(canal, 84)` (waveform-styles.js:76): bandas e o gate de áudio.
    let bandas = |o: usize| -> Result<(Vec<f64>, Vec<f64>, bool), String> {
        let d = db(o)?;
        let disponivel = audivel(tempo(o)?);
        let (b, _) = spectrum_bands(d, disponivel, NYQUIST, 84.0);
        Ok((b, d.to_vec(), disponivel))
    };
    let mut e: HashMap<String, Esperado> = HashMap::new();
    let mut put = |k: &str, v: Esperado| {
        e.insert(k.to_string(), v);
    };
    match alvo.nome {
        "06-hifi-spectrum" | "12-osciloscopio-crt" => {
            for (o, canal) in [(0usize, "L"), (1, "R")] {
                let b = tempo(o)?;
                put(
                    &format!("nivel/{canal}"),
                    Esperado::Escalar {
                        valor: nivel_preroll(b),
                        teto: 0.1,
                    },
                );
                if alvo.nome == "06-hifi-spectrum" {
                    let d = db(o)?;
                    let valores = bins_hifi(d.len())
                        .iter()
                        .map(|&i| d[i].clamp(-60.0, 0.0))
                        .collect();
                    put(
                        &format!("espectro/{canal}"),
                        Esperado::Vetor { valores, teto: 0.5 },
                    );
                } else {
                    let valores = (0..=180).map(|i| b[i * 4] as f64).collect();
                    put(
                        &format!("onda/{canal}"),
                        Esperado::Vetor {
                            valores,
                            teto: 1e-4,
                        },
                    );
                }
            }
        }
        "pulse" => put(
            "d@wave-pulse/L",
            geo(geometria_pulse(Some(tempo(0)?), 256, 22.0), 22e-4, 1.0),
        ),
        "bars-pulse" => {
            let nivel = estado.barras.update(Some(tempo(0)?), 1.0, 0.3);
            put(
                "data-level@wave-reactive-bars/L",
                Esperado::Escalar {
                    valor: nivel,
                    teto: 1e-4 + 5e-4,
                },
            );
            put(
                "transform@wave-reactive-bars/L",
                Esperado::Escala {
                    valor: nivel,
                    teto: 1e-4 + 5e-5,
                },
            );
        }
        "columns-pulse" | "prisma-neon" => {
            let s = tempo(0)?;
            put(
                "data-peak@wave-live-styles/L",
                Esperado::Escalar {
                    valor: data_peak(Some(s)),
                    teto: TETO_PICO,
                },
            );
            put(
                "d@wave-live-columns/L",
                geo(geometria_colunas(Some(s), 128, 1.4, 21.0), 29.4e-4, 2.0),
            );
        }
        "ribbon-pulse" => {
            let s = tempo(0)?;
            put(
                "data-peak@wave-live-styles/L",
                Esperado::Escalar {
                    valor: data_peak(Some(s)),
                    teto: TETO_PICO,
                },
            );
            put(
                "d@wave-live-fill/L",
                geo(geometria_ribbon(Some(s), 64, 2.0, 21.0), 42e-4, 1.0),
            );
        }
        "aurora-neon" => {
            let s = tempo(0)?;
            put(
                "data-peak@wave-live-styles/L",
                Esperado::Escalar {
                    valor: data_peak(Some(s)),
                    teto: TETO_PICO,
                },
            );
            let (linha, halo) = geometria_aurora_neon(Some(s), 256, 22.0);
            put("d@wave-live-line/L", geo(linha, 22e-4, 1.0));
            put("d@wave-live-halo/L", geo(halo, 22e-4, 1.0));
        }
        "aurora-weave" => {
            let w = geometria_aurora_weave(Some(tempo(0)?), Some(tempo(1)?), 256, 22.0);
            put(
                "data-level@wave-creative-live/LR",
                Esperado::Escalar {
                    valor: w.nivel,
                    teto: TETO_PICO,
                },
            );
            put("d@wave-creative-left/L", geo(w.esquerda, 22e-4, 1.0));
            put("d@wave-creative-right/R", geo(w.direita, 22e-4, 1.0));
        }
        "neon-crystal" | "aurora-layers" => {
            let segmentos = if alvo.nome == "neon-crystal" { 32 } else { 64 };
            let c = geometria_camadas(Some(tempo(0)?), segmentos, 2.0, 2.0, 21.0, 16.0);
            put(
                "data-level@wave-creative-live/L",
                Esperado::Escalar {
                    valor: c.nivel,
                    teto: TETO_PICO,
                },
            );
            put("d@wave-creative-outer/L", geo(c.externa, 42e-4, 1.0));
            put("d@wave-creative-middle/L", geo(c.media, 42e-4, 1.0));
            put("d@wave-creative-inner/L", geo(c.interna, 42e-4, 1.0));
        }
        "spectrum-aurora" | "spectrum-ribbon" | "spectrum-stereo" => {
            let (esq, db_esq, disp_esq) = bandas(0)?;
            let dir = if alvo.nome == "spectrum-stereo" {
                let (dir, db_dir, disp_dir) = bandas(1)?;
                put(
                    "data-frequency-right@wave-spectrum-live/R",
                    Esperado::Frequencia {
                        db: db_dir,
                        disponivel: disp_dir,
                    },
                );
                dir
            } else {
                esq.clone()
            };
            let (caminhos, nivel) = geometria_spectrum(alvo.nome, &esq, &dir, 21.0);
            let canal_nivel = if alvo.nome == "spectrum-stereo" {
                "LR"
            } else {
                "L"
            };
            put(
                &format!("data-level@wave-spectrum-live/{canal_nivel}"),
                Esperado::Escalar {
                    valor: nivel,
                    teto: TETO_NIVEL_BANDAS,
                },
            );
            put(
                "data-frequency-left@wave-spectrum-live/L",
                Esperado::Frequencia {
                    db: db_esq,
                    disponivel: disp_esq,
                },
            );
            for (id, comandos) in caminhos {
                let canal = if id == "wave-spectrum-right" {
                    "R"
                } else {
                    "L"
                };
                put(
                    &format!("d@{id}/{canal}"),
                    geo(comandos, ESCALA_BANDAS, 1.0),
                );
            }
        }
        "aurora-constellation" => {
            let (b, d, disponivel) = bandas(0)?;
            let c = geometria_constelacao(&b, 32, 1.7, 21.0);
            put(
                "data-level@wave-creative-live/L",
                Esperado::Escalar {
                    valor: c.nivel,
                    teto: TETO_NIVEL_BANDAS,
                },
            );
            put(
                "data-frequency-left@wave-spectrum-live/L",
                Esperado::Frequencia { db: d, disponivel },
            );
            put("d@wave-creative-nodes/L", geo(c.nos, ESCALA_BANDAS, 1.0));
            put("d@wave-creative-mesh/L", geo(c.malha, ESCALA_BANDAS, 1.0));
        }
        "spectrum-pulse" => {
            let m = merge(db(2)?, db(3)?);
            estado.pulso.update(Some(&m), 48000.0, 64, t_k(k));
            let (fill, halo, nivel) = geometria_spectrum_pulse(&estado.pulso, 42.0);
            let escala = 42.0 * 0.5 / 80.0;
            put(
                "data-level@wave-spectrum-live/LR",
                Esperado::Escalar {
                    valor: nivel,
                    teto: 0.5 / 80.0 + 5e-5,
                },
            );
            put("d@wave-spectrum-fill/LR", geo(fill, escala, 1.0));
            put("d@wave-spectrum-halo/LR", geo(halo, escala, 1.0));
        }
        outro => return Err(format!("alvo sem oráculo: {outro}")),
    }
    Ok(e)
}

/// Resultado da medição de uma série inteira.
pub struct Medida {
    /// Chave `<execução>/<série>/<canal>`.
    pub chave: String,
    /// Maior desvio absoluto contra o oráculo na série.
    pub desvio: f64,
    /// Leituras fora do teto ou com estrutura divergente.
    pub violacoes: Vec<String>,
}

/// Mede todas as séries do gabarito. Erro quando o gabarito não tem a forma do protocolo.
pub fn medir(g: &Value) -> Result<Vec<Medida>, String> {
    let execucoes = lista(campo(g, "execucoes", "gabarito")?, "execucoes")?;
    let esperadas = execucoes_esperadas();
    if execucoes.len() != esperadas.len() {
        return Err(format!(
            "{} execuções no gabarito, {} no protocolo",
            execucoes.len(),
            esperadas.len()
        ));
    }
    let mut cache = CacheFft::new();
    let mut medidas = Vec::new();
    for (ex, (id, alvo, tipo)) in execucoes.iter().zip(&esperadas) {
        let ctx = id.as_str();
        let leituras = lista(campo(ex, "leituras", ctx)?, ctx)?;
        let series = objeto(campo(ex, "series", ctx)?, ctx)?;
        let mut por_serie: BTreeMap<&str, (f64, Vec<String>)> = alvo
            .series
            .iter()
            .map(|s| (*s, (0.0, Vec::new())))
            .collect();
        let mut estado = Estado::novo();
        for (i, l) in leituras.iter().enumerate() {
            let k = inteiro(campo(l, "k", ctx)?, ctx)?;
            let amostra = inteiro(campo(l, "amostra", ctx)?, ctx)?;
            let lido = ler_quadro(*tipo, amostra, alvo.leituras, &mut estado, &mut cache);
            let esp = esperados(alvo, k, &lido, &mut estado)?;
            for serie in alvo.series {
                let rotulo = format!("{id}/{serie} k={k}");
                let entrada = por_serie
                    .get_mut(serie)
                    .ok_or_else(|| format!("{rotulo}: série fora da lista"))?;
                let esperado = esp
                    .get(*serie)
                    .ok_or_else(|| format!("{rotulo}: oráculo sem valor para a série"))?;
                let valor = series
                    .get(*serie)
                    .and_then(|s| s.get("valores"))
                    .and_then(|v| v.get(i))
                    .ok_or_else(|| format!("{rotulo}: gabarito sem o valor {i}"))?;
                let (d, v) = comparar(esperado, valor, &rotulo);
                entrada.0 = entrada.0.max(d);
                entrada.1.extend(v);
            }
        }
        for (serie, (desvio, violacoes)) in por_serie {
            medidas.push(Medida {
                chave: chave_tolerancia(id, serie),
                desvio,
                violacoes,
            });
        }
    }
    Ok(medidas)
}

/// Família de teto de uma série, para o relatório do maior desvio por família.
pub fn familia(chave: &str) -> &'static str {
    let serie = chave.split_once('/').map_or(chave, |(_, s)| s);
    if serie.starts_with("nivel/") {
        "nível (0,1 dB)"
    } else if serie.starts_with("espectro/") {
        "espectro (0,5 dB)"
    } else if serie.starts_with("onda/") {
        "onda (1e-4)"
    } else if serie.starts_with("data-frequency") {
        "data-frequency (0,5 dB)"
    } else if serie.starts_with("d@") || serie.starts_with("transform@") {
        "geometria"
    } else {
        "atributo numérico"
    }
}

/// Confere a estrutura do gabarito contra o protocolo (asserções contra o vazio, item 7
/// de "Estilos de onda", e itens 4 a 7 e 10 do protocolo). Devolve as falhas.
pub fn conferir_estrutura(g: &Value) -> Result<Vec<String>, String> {
    let mut f = Vec::new();
    let commit = texto(
        campo(campo(g, "mvp", "gabarito")?, "commit", "mvp")?,
        "mvp.commit",
    )?;
    if commit != MVP_COMMIT {
        f.push(format!("mvp.commit {commit}, esperado {MVP_COMMIT}"));
    }
    let balistica = campo(g, "balistica", "gabarito")?;
    if !campo(balistica, "valor", "balistica")?.is_null() {
        f.push("balistica.valor deveria ser null (item 7 do protocolo)".into());
    }
    if texto(campo(balistica, "motivo", "balistica")?, "balistica.motivo")?.is_empty() {
        f.push("balistica.motivo vazio".into());
    }
    let protocolo = campo(g, "protocolo", "gabarito")?;
    let esperado_protocolo = serde_json::json!({
        "taxa": 48000, "amostras": 96000, "quadros": 120, "quantum": 128,
        "sinais": ["degrau", "seno", "rajada"]
    });
    if *protocolo != esperado_protocolo {
        f.push(format!(
            "protocolo {protocolo}, esperado {esperado_protocolo}"
        ));
    }
    let execucoes = lista(campo(g, "execucoes", "gabarito")?, "execucoes")?;
    let esperadas = execucoes_esperadas();
    let ids: Vec<&str> = execucoes
        .iter()
        .map(|e| e.get("execucao").and_then(Value::as_str).unwrap_or("?"))
        .collect();
    let ids_esperados: Vec<&str> = esperadas.iter().map(|(id, _, _)| id.as_str()).collect();
    if ids != ids_esperados {
        f.push(format!(
            "execuções {ids:?}, esperadas {ids_esperados:?} (item 4 do protocolo)"
        ));
        return Ok(f);
    }
    for (ex, (id, alvo, _)) in execucoes.iter().zip(&esperadas) {
        let ctx = id.as_str();
        let (alvo_nome, sinal_nome) = id.split_once(':').unwrap_or((id, ""));
        let tipo_esperado = if alvo.medidor { "medidor" } else { "estilo" };
        for (campo_nome, esperado) in [
            ("alvo", alvo_nome),
            ("sinal", sinal_nome),
            ("tipo", tipo_esperado),
        ] {
            let lido = texto(campo(ex, campo_nome, ctx)?, ctx)?;
            if lido != esperado {
                f.push(format!("{ctx}: {campo_nome} {lido}, esperado {esperado}"));
            }
        }
        let leituras = lista(campo(ex, "leituras", ctx)?, ctx)?;
        let quadros = alvo.quadros();
        if quadros.len() != alvo.contagem() || leituras.len() != alvo.contagem() {
            f.push(format!(
                "{ctx}: {} leituras no gabarito, {} na grade, contagem do protocolo {}",
                leituras.len(),
                quadros.len(),
                alvo.contagem()
            ));
        }
        for (l, &k) in leituras.iter().zip(&quadros) {
            let lk = inteiro(campo(l, "k", ctx)?, ctx)?;
            let t = numero(campo(l, "t", ctx)?, ctx)?;
            let amostra = inteiro(campo(l, "amostra", ctx)?, ctx)?;
            if lk != k || t != t_k(k) || amostra != amostra_suspensao(k) {
                f.push(format!(
                    "{ctx}: leitura k={lk} t={t} amostra={amostra}, esperado k={k} t={} amostra={}",
                    t_k(k),
                    amostra_suspensao(k)
                ));
            }
            let analisadores = lista(campo(l, "analisadores", ctx)?, ctx)?;
            let lidos: Vec<(u64, &str, u64, f64)> = analisadores
                .iter()
                .map(|a| {
                    (
                        a.get("ordem").and_then(Value::as_u64).unwrap_or(u64::MAX),
                        a.get("metodo").and_then(Value::as_str).unwrap_or("?"),
                        a.get("fftSize").and_then(Value::as_u64).unwrap_or(0),
                        a.get("smoothingTimeConstant")
                            .and_then(Value::as_f64)
                            .unwrap_or(f64::NAN),
                    )
                })
                .collect();
            let esperados: Vec<(u64, &str, u64, f64)> = alvo
                .leituras
                .iter()
                .map(|&(o, m)| (o, m.nome(), FFT as u64, SUAVIZACAO[o as usize]))
                .collect();
            if lidos != esperados {
                f.push(format!(
                    "{ctx} k={lk}: analisadores {lidos:?}, esperados {esperados:?}"
                ));
            }
        }
        let series = objeto(campo(ex, "series", ctx)?, ctx)?;
        let chaves: BTreeSet<&str> = series.keys().map(String::as_str).collect();
        let chaves_esperadas: BTreeSet<&str> = alvo.series.iter().copied().collect();
        if chaves != chaves_esperadas {
            f.push(format!(
                "{ctx}: séries {chaves:?}, esperadas {chaves_esperadas:?}"
            ));
            continue;
        }
        for serie in alvo.series {
            let s = &series[*serie];
            let rotulo = format!("{ctx}/{serie}");
            let analisador: Vec<(&str, u64, f64)> =
                lista(campo(s, "analisador", &rotulo)?, &rotulo)?
                    .iter()
                    .map(|a| {
                        (
                            a.get("canal").and_then(Value::as_str).unwrap_or("?"),
                            a.get("fftSize").and_then(Value::as_u64).unwrap_or(0),
                            a.get("smoothingTimeConstant")
                                .and_then(Value::as_f64)
                                .unwrap_or(f64::NAN),
                        )
                    })
                    .collect();
            let analisador_esperado: Vec<(&str, u64, f64)> = alvo
                .analisadores_da_serie(serie)
                .into_iter()
                .map(|(c, sm)| (c, FFT as u64, sm))
                .collect();
            if analisador != analisador_esperado {
                f.push(format!(
                    "{rotulo}: analisador {analisador:?}, esperado {analisador_esperado:?}"
                ));
            }
            let valores = lista(campo(s, "valores", &rotulo)?, &rotulo)?;
            if valores.len() != alvo.contagem() {
                f.push(format!(
                    "{rotulo}: {} valores, esperados {}",
                    valores.len(),
                    alvo.contagem()
                ));
            }
            if sinal_nome == "degrau" {
                let distintos: BTreeSet<String> = valores.iter().map(Value::to_string).collect();
                if distintos.len() < 2 {
                    f.push(format!(
                        "{rotulo}: degrau com {} valor distinto; a série não responde",
                        distintos.len()
                    ));
                }
            }
        }
    }
    Ok(f)
}

#[test]
fn fft_rapida_bate_com_a_dft_do_oraculo() {
    // A FFT do teste tem de ser a mesma conta da DFT direta do oráculo
    // (oraculo::analisador::magnitudes), num bloco com pré-roll e portadora.
    let bloco = bloco_com_preroll(Tipo::Degrau, 24000 + 1000);
    let rapida = magnitudes_fft(&bloco);
    let direta = crate::oraculo::analisador::magnitudes(&bloco);
    assert_eq!(rapida.len(), direta.len());
    let pior = rapida
        .iter()
        .zip(&direta)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f64, f64::max);
    assert!(pior < 1e-12, "FFT difere da DFT em {pior:e}");
    assert!(
        rapida[43] > 1e-3,
        "a portadora de 1 kHz tem de aparecer no bin 43"
    );
}

#[test]
fn preroll_e_zero_antes_da_amostra_zero() {
    // Item 6 do protocolo: o bloco tem sempre 2048 amostras; antes de 0, zero.
    let b = bloco_com_preroll(Tipo::Seno, 1664);
    assert_eq!(b.len(), FFT);
    assert!(b[..FFT - 1664].iter().all(|&v| v == 0.0));
    assert_eq!(b[FFT - 1664 + 12], sinal(Tipo::Seno, 12));
    // RMS sobre 2048, não sobre as 1664 amostras com sinal: 1664 de 2048 com RMS
    // 0,5/√2 dão 20·log10(0,3536·√(1664/2048)) ≈ −9,94 dB.
    let n = nivel_preroll(&b);
    assert!((n - (-9.943)).abs() < 0.01, "nível {n}");
}
