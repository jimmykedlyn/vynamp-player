//! AC-1: cada animação do MVP tem duração, easing e keyframes ou série temporal.
//!
//! As listas esperadas vêm do código do MVP no commit `0d9860b9` (arquivo:linha em cada
//! entrada), não do próprio JSON.

use crate::criterio::json::{campo, gabarito, inteiro, lista, numero, sem_falhas, texto, textos};
use crate::criterio::medicao::ALVOS;
use serde_json::Value;
use std::collections::BTreeSet;

/// Declarações `transition*`/`animation*` e `@keyframes` das folhas do MVP:
/// `(arquivo, linha, coluna, texto)`, na ordem do arquivo.
const CSS: [(&str, u64, u64, &str); 38] = [
    (
        "audio-controls.css",
        3,
        677,
        "transition:transform 100ms var(--ease)",
    ),
    ("audio-controls.css", 8, 68, "transition:none"),
    ("graph-editor.css", 1, 4195, "@keyframes graph-signal"),
    (
        "graph-editor.css",
        1,
        4014,
        "animation:graph-signal 3s linear infinite",
    ),
    ("graph-editor.css", 1, 6061, "transition:width 90ms linear"),
    ("graph-editor.css", 5, 51, "animation:none"),
    ("graph-editor.css", 5, 89, "transition:none"),
    ("graph-editor.css", 7, 44, "animation:none"),
    ("library-workspace.css", 7, 645, "@keyframes media-wait"),
    (
        "library-workspace.css",
        7,
        590,
        "animation:media-wait 1s ease-in-out infinite alternate",
    ),
    ("library-workspace.css", 10, 59, "animation:none!important"),
    ("library-workspace.css", 10, 137, "animation:none!important"),
    (
        "lyrics.css",
        1,
        881,
        "transition:color 200ms var(--ease),opacity 200ms var(--ease)",
    ),
    ("lyrics.css", 3, 117, "transition:none"),
    ("lyrics.css", 3, 184, "transition:none"),
    (
        "player.css",
        2,
        217,
        "transition:background-color 120ms var(--ease),color 120ms var(--ease)",
    ),
    ("reference-layout.css", 12, 664, "transition:none"),
    (
        "refinement28.css",
        15,
        235,
        "transition:background-color 120ms,color 120ms,border-color 120ms",
    ),
    ("refinement28.css", 34, 65, "transition:none"),
    ("refinement39.css", 46, 121, "transition:opacity .12s"),
    ("refinement40.css", 725, 3, "animation:none!important"),
    ("refinement40.css", 726, 3, "transition:none!important"),
    ("restore.css", 15, 60, "transition:none!important"),
    ("restore.css", 15, 86, "animation:none!important"),
    (
        "restore.css",
        24,
        134,
        "transition:border-color .3s,box-shadow .3s",
    ),
    ("restore.css", 35, 196, "transition:background .15s"),
    ("restore.css", 37, 48, "transition:none!important"),
    ("restore.css", 62, 71, "transition:width .2s"),
    ("restore.css", 75, 60, "transition:none!important"),
    ("spec-controls.css", 4, 21, "transition-duration: 80ms"),
    ("spec-controls.css", 72, 38, "transition: none!important"),
    ("spec-controls.css", 72, 66, "animation: none!important"),
    ("usability.css", 26, 125, "animation:none!important"),
    ("usability.css", 26, 150, "transition:none!important"),
    (
        "waveform-styles.css",
        48,
        228,
        "transition:opacity .15s,color .15s",
    ),
    (
        "waveform-styles.css",
        50,
        155,
        "transition:border-color .15s",
    ),
    (
        "waveform-styles.css",
        52,
        195,
        "transition:border-color .15s,box-shadow .15s",
    ),
    ("workspace-layout.css", 4, 55, "transition:none"),
];

/// Chamadas de `requestAnimationFrame(` e `.animate(` nos scripts do MVP:
/// `(arquivo, linha, coluna, api)`.
const JS: [(&str, u64, u64, &str); 18] = [
    ("advanced-eq.js", 109, 541, "requestAnimationFrame"),
    ("eq-spectrum.js", 22, 146, "requestAnimationFrame"),
    ("eq-spectrum.js", 23, 63, "requestAnimationFrame"),
    ("graph-editor.js", 30, 56, "requestAnimationFrame"),
    ("graph-properties.js", 20, 356, "requestAnimationFrame"),
    ("graph-properties.js", 21, 62, "requestAnimationFrame"),
    ("index.html", 6, 50, "requestAnimationFrame"),
    ("lyrics.js", 32, 73, "animate"),
    ("meter-hairline.js", 15, 28, "requestAnimationFrame"),
    ("meter-icons.js", 18, 74, "requestAnimationFrame"),
    ("player.js", 69, 74, "requestAnimationFrame"),
    ("player.js", 70, 928, "requestAnimationFrame"),
    ("refinement21.js", 42, 54, "requestAnimationFrame"),
    ("restore.js", 133, 69, "requestAnimationFrame"),
    ("waveform-styles.js", 56, 259, "requestAnimationFrame"),
    ("waveform-styles.js", 89, 1876, "requestAnimationFrame"),
    ("waveform-styles.js", 90, 178, "requestAnimationFrame"),
    ("waveform-styles.js", 93, 1393, "requestAnimationFrame"),
];

/// `--ease` de `player.css:1`, que as transições com `var(--ease)` resolvem.
const EASE: &str = "cubic-bezier(.2,.8,.2,1)";

/// Uma transição ou animação CSS observável, resolvida: `var(--ease)` substituído e a
/// sobreposição de `spec-controls.css:4` (`transition-duration: 80ms` em
/// `button, a, select`, folha carregada depois de `player.css`) aplicada.
struct Transicao {
    origem: &'static str,
    tipo: &'static str,
    /// `(propriedade, duração em ms, easing)`.
    propriedades: &'static [(&'static str, f64, &'static str)],
}

/// As 14 transições e animações que não são `none`.
const TRANSICOES: [Transicao; 14] = [
    Transicao {
        origem: "audio-controls.css:3",
        tipo: "transition",
        propriedades: &[("transform", 100.0, EASE)],
    },
    Transicao {
        origem: "graph-editor.css:1:4014",
        tipo: "animation",
        propriedades: &[("stroke-dashoffset", 3000.0, "linear")],
    },
    Transicao {
        origem: "graph-editor.css:1:6061",
        tipo: "transition",
        propriedades: &[("width", 90.0, "linear")],
    },
    Transicao {
        origem: "library-workspace.css:7",
        tipo: "animation",
        propriedades: &[("opacity", 1000.0, "ease-in-out")],
    },
    Transicao {
        origem: "lyrics.css:1",
        tipo: "transition",
        propriedades: &[("color", 200.0, EASE), ("opacity", 200.0, EASE)],
    },
    Transicao {
        origem: "player.css:2",
        tipo: "transition",
        propriedades: &[("background-color", 80.0, EASE), ("color", 80.0, EASE)],
    },
    Transicao {
        origem: "refinement28.css:15",
        tipo: "transition",
        propriedades: &[
            ("background-color", 120.0, "ease"),
            ("color", 120.0, "ease"),
            ("border-color", 120.0, "ease"),
        ],
    },
    Transicao {
        origem: "refinement39.css:46",
        tipo: "transition",
        propriedades: &[("opacity", 120.0, "ease")],
    },
    Transicao {
        origem: "restore.css:24",
        tipo: "transition",
        propriedades: &[
            ("border-color", 300.0, "ease"),
            ("box-shadow", 300.0, "ease"),
        ],
    },
    Transicao {
        origem: "restore.css:35",
        tipo: "transition",
        propriedades: &[("background", 150.0, "ease")],
    },
    Transicao {
        origem: "restore.css:62",
        tipo: "transition",
        propriedades: &[("width", 200.0, "ease")],
    },
    Transicao {
        origem: "waveform-styles.css:48",
        tipo: "transition",
        propriedades: &[("opacity", 150.0, "ease"), ("color", 150.0, "ease")],
    },
    Transicao {
        origem: "waveform-styles.css:50",
        tipo: "transition",
        propriedades: &[("border-color", 150.0, "ease")],
    },
    Transicao {
        origem: "waveform-styles.css:52",
        tipo: "transition",
        propriedades: &[
            ("border-color", 150.0, "ease"),
            ("box-shadow", 150.0, "ease"),
        ],
    },
];

/// Cadência dos laços de `requestAnimationFrame`, literal do código do MVP.
const CADENCIAS: [(&str, &str); 6] = [
    (
        "player.js:70",
        "t-lastMeter>=1000/meterFPS; meterFPS=motionReduced()?10:[10,30,60].includes(state.fps)?state.fps:30",
    ),
    (
        "waveform-styles.js:89",
        "t-lastPulse>=(low?100:selected==='spectrum-pulse'?0:33)",
    ),
    (
        "eq-spectrum.js:22",
        "t-last>=(motion.matches||api.state.reducedMotion?250:0)",
    ),
    (
        "graph-properties.js:20",
        "time-lastFrame>=(motion.matches||api.state.reducedMotion?250:mode==='spectrum'?0:50)",
    ),
    ("player.js:69", "agenda o laço de player.js:70"),
    (
        "waveform-styles.js:56",
        "agenda o laço de waveform-styles.js:89",
    ),
];

fn animacoes() -> Result<&'static Value, String> {
    campo(gabarito()?, "animacoes", "gabarito")
}

#[test]
fn ac1_varredura_estatica_acha_cada_declaracao_e_chamada() -> Result<(), String> {
    // Given as folhas e os scripts do MVP; When o gerador varre transition/animation,
    // @keyframes, requestAnimationFrame e .animate; Then acha exatamente as entradas do
    // código, nem mais nem menos.
    let est = campo(animacoes()?, "estatica", "animacoes")?;
    let mut css = BTreeSet::new();
    for f in lista(campo(est, "css", "estatica")?, "estatica.css")? {
        let arq = texto(campo(f, "arquivo", "css")?, "css.arquivo")?;
        for grupo in ["keyframes", "declaracoes"] {
            for d in lista(campo(f, grupo, arq)?, arq)? {
                css.insert((
                    arq.to_string(),
                    inteiro(campo(d, "linha", arq)?, arq)?,
                    inteiro(campo(d, "coluna", arq)?, arq)?,
                    texto(campo(d, "texto", arq)?, arq)?.to_string(),
                ));
            }
        }
    }
    let css_esperado: BTreeSet<_> = CSS
        .iter()
        .map(|(a, l, c, t)| (a.to_string(), *l, *c, t.to_string()))
        .collect();
    let mut js = BTreeSet::new();
    for f in lista(campo(est, "js", "estatica")?, "estatica.js")? {
        let arq = texto(campo(f, "arquivo", "js")?, "js.arquivo")?;
        for api in ["requestAnimationFrame", "animate"] {
            for d in lista(campo(f, api, arq)?, arq)? {
                js.insert((
                    arq.to_string(),
                    inteiro(campo(d, "linha", arq)?, arq)?,
                    inteiro(campo(d, "coluna", arq)?, arq)?,
                    api.to_string(),
                ));
            }
        }
    }
    let js_esperado: BTreeSet<_> = JS
        .iter()
        .map(|(a, l, c, t)| (a.to_string(), *l, *c, t.to_string()))
        .collect();
    let mut f = Vec::new();
    if css != css_esperado {
        f.push(format!(
            "CSS faltando {:?}, sobrando {:?}",
            css_esperado.difference(&css).collect::<Vec<_>>(),
            css.difference(&css_esperado).collect::<Vec<_>>()
        ));
    }
    if js != js_esperado {
        f.push(format!(
            "JS faltando {:?}, sobrando {:?}",
            js_esperado.difference(&js).collect::<Vec<_>>(),
            js.difference(&js_esperado).collect::<Vec<_>>()
        ));
    }
    sem_falhas(f)
}

#[test]
fn ac1_cada_transicao_tem_duracao_easing_e_keyframes_ou_estados() -> Result<(), String> {
    // Given as 14 transições e animações do MVP que não são `none`; When o gabarito as
    // descreve; Then cada uma tem a duração e o easing resolvidos, keyframes (animação)
    // ou estado de partida, de chegada e gatilho (transição), e a lista de reduções
    // (`none`) que a desligam.
    let a = animacoes()?;
    let lidas = lista(campo(a, "transicoes", "animacoes")?, "transicoes")?;
    let mut f = Vec::new();
    let origens: Vec<&str> = lidas
        .iter()
        .map(|t| t.get("origem").and_then(Value::as_str).unwrap_or("?"))
        .collect();
    let esperadas: Vec<&str> = TRANSICOES.iter().map(|t| t.origem).collect();
    if origens != esperadas {
        f.push(format!("origens {origens:?}, esperadas {esperadas:?}"));
        return sem_falhas(f);
    }
    let reducoes_validas: BTreeSet<String> = CSS
        .iter()
        .filter(|(_, _, _, t)| t.contains("none"))
        .map(|(a, l, c, _)| format!("{a}:{l}:{c}"))
        .collect();
    for (t, e) in lidas.iter().zip(&TRANSICOES) {
        let ctx = e.origem;
        if texto(campo(t, "tipo", ctx)?, ctx)? != e.tipo {
            f.push(format!("{ctx}: tipo diferente de {}", e.tipo));
        }
        if texto(campo(t, "seletor", ctx)?, ctx)?.is_empty() {
            f.push(format!("{ctx}: seletor vazio"));
        }
        let props = lista(campo(t, "propriedades", ctx)?, ctx)?;
        let mut lidas_props = Vec::new();
        for p in props {
            lidas_props.push((
                texto(campo(p, "propriedade", ctx)?, ctx)?,
                numero(campo(p, "duracaoMs", ctx)?, ctx)?,
                texto(campo(p, "easing", ctx)?, ctx)?,
            ));
        }
        let esperadas_props: Vec<(&str, f64, &str)> = e.propriedades.to_vec();
        if lidas_props != esperadas_props {
            f.push(format!(
                "{ctx}: propriedades {lidas_props:?}, esperadas {esperadas_props:?}"
            ));
        }
        if e.tipo == "animation" {
            if lista(campo(t, "keyframes", ctx)?, ctx)?.len() < 2 {
                f.push(format!("{ctx}: animação sem os dois keyframes (de e para)"));
            }
            if texto(campo(t, "iteracoes", ctx)?, ctx)? != "infinite" {
                f.push(format!("{ctx}: iterações diferentes de infinite"));
            }
        } else {
            for c in ["de", "para", "gatilho"] {
                if texto(campo(t, c, ctx)?, ctx)?.is_empty() {
                    f.push(format!("{ctx}: `{c}` vazio"));
                }
            }
        }
        let reducao = textos(campo(t, "reducao", ctx)?, ctx)?;
        if reducao.is_empty() {
            f.push(format!("{ctx}: sem a regra `none` que a desliga"));
        }
        for r in reducao {
            if !reducoes_validas.contains(r) {
                f.push(format!("{ctx}: redução {r} não é declaração `none` do MVP"));
            }
        }
    }
    sem_falhas(f)
}

#[test]
fn ac1_cada_raf_e_waapi_tem_serie_ou_motivo() -> Result<(), String> {
    // Given as 17 chamadas de requestAnimationFrame e a de .animate; When o gabarito as
    // descreve; Then cada uma tem tipo e cadência, e aponta a série temporal medida ou
    // diz por que não tem; a WAAPI de lyrics.js:32 tem duração, easing e keyframes.
    let a = animacoes()?;
    let lidos = lista(campo(a, "comportamentos", "animacoes")?, "comportamentos")?;
    let mut f = Vec::new();
    let origens: Vec<&str> = lidos
        .iter()
        .map(|c| c.get("origem").and_then(Value::as_str).unwrap_or("?"))
        .collect();
    let esperadas: Vec<String> = JS.iter().map(|(a, l, _, _)| format!("{a}:{l}")).collect();
    if origens != esperadas {
        f.push(format!("origens {origens:?}, esperadas {esperadas:?}"));
        return sem_falhas(f);
    }
    let medidores: Vec<&str> = ALVOS.iter().filter(|a| a.medidor).map(|a| a.nome).collect();
    let estilos: Vec<&str> = ALVOS
        .iter()
        .filter(|a| !a.medidor)
        .map(|a| a.nome)
        .collect();
    for ((c, ctx), (arq, _, _, api)) in lidos.iter().zip(&origens).zip(&JS) {
        let ctx = *ctx;
        if texto(campo(c, "api", ctx)?, ctx)? != *api {
            f.push(format!("{ctx}: api diferente de {api}"));
        }
        if texto(campo(c, "tipo", ctx)?, ctx)?.is_empty() {
            f.push(format!("{ctx}: tipo vazio"));
        }
        let cadencia = texto(campo(c, "cadencia", ctx)?, ctx)?;
        if let Some((_, esperada)) = CADENCIAS.iter().find(|(o, _)| *o == ctx)
            && cadencia != *esperada
        {
            f.push(format!(
                "{ctx}: cadência `{cadencia}`, esperada `{esperada}`"
            ));
        }
        let serie = c.get("serie").map(|s| textos(s, ctx)).transpose()?;
        let motivo = c.get("motivo").and_then(Value::as_str).unwrap_or("");
        let serie_esperada: Option<&[&str]> = match *arq {
            "player.js" => Some(&medidores),
            "waveform-styles.js" => Some(&estilos),
            _ => None,
        };
        match (serie_esperada, serie) {
            (Some(e), Some(s)) if s == e => {}
            (Some(e), s) => f.push(format!("{ctx}: série {s:?}, esperada {e:?}")),
            (None, Some(s)) => f.push(format!("{ctx}: série {s:?} sem execução medida")),
            (None, None) if motivo.is_empty() => {
                f.push(format!("{ctx}: sem série e sem motivo"));
            }
            (None, None) => {}
        }
    }
    let waapi = &lidos[7];
    let ctx = "lyrics.js:32";
    let keyframes: Vec<String> = lista(campo(waapi, "keyframes", ctx)?, ctx)?
        .iter()
        .map(|k| {
            k.get("transform")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string()
        })
        .collect();
    if numero(campo(waapi, "duracaoMs", ctx)?, ctx)? != 260.0
        || texto(campo(waapi, "easing", ctx)?, ctx)? != "cubic-bezier(.2,.7,.2,1)"
        || texto(campo(waapi, "preenchimento", ctx)?, ctx)? != "forwards"
        || keyframes != ["translateY(0)", "translateY(-${height}px)"]
    {
        f.push(format!(
            "{ctx}: esperado 260 ms, cubic-bezier(.2,.7,.2,1), forwards e keyframes \
             translateY(0) → translateY(-${{height}}px); veio {waapi}"
        ));
    }
    sem_falhas(f)
}
