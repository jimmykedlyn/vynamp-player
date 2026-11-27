//! AC-2: o gabarito commitado tem a forma do protocolo e cada série fica no teto do
//! oráculo; `tolerancia.json` guarda o maior desvio de cada série.

use crate::criterio::json::{CAMINHO_TOLERANCIA, gabarito, ler, numero, objeto, sem_falhas};
use crate::criterio::medicao::{chaves_tolerancia, conferir_estrutura, familia, medir};
use std::collections::{BTreeMap, BTreeSet};

/// Variável que autoriza o teste a regravar `tolerancia.json` (só na geração).
const GRAVAR: &str = "VYNAMP_GRAVAR_TOLERANCIA";

#[test]
fn ac2_gabarito_tem_a_forma_do_protocolo() -> Result<(), String> {
    // Given o gabarito commitado; When comparado ao protocolo (48 execuções, cadência
    // 42/59/120, amostra da suspensão, analisadores, séries, SHA, balística null);
    // Then nenhuma falha.
    let g = gabarito()?;
    sem_falhas(conferir_estrutura(g)?)
}

#[test]
fn ac2_cada_serie_fica_no_teto_do_oraculo() -> Result<(), String> {
    // Given o gabarito; When cada valor é recalculado pelo oráculo a partir da amostra
    // da leitura; Then todo desvio cabe no teto do critério.
    let g = gabarito()?;
    let medidas = medir(g)?;
    let mut por_familia: BTreeMap<&str, (f64, String)> = BTreeMap::new();
    for m in &medidas {
        let e = por_familia
            .entry(familia(&m.chave))
            .or_insert((0.0, String::new()));
        if m.desvio >= e.0 {
            *e = (m.desvio, m.chave.clone());
        }
    }
    for (f, (d, chave)) in &por_familia {
        eprintln!("maior desvio {f}: {d:.6e} em {chave}");
    }
    let chaves: BTreeSet<String> = medidas.iter().map(|m| m.chave.clone()).collect();
    if chaves != chaves_tolerancia() {
        return Err("as séries medidas não são as do protocolo".into());
    }
    sem_falhas(medidas.into_iter().flat_map(|m| m.violacoes).collect())
}

#[test]
fn ac2_tolerancia_registra_o_maior_desvio_de_cada_serie() -> Result<(), String> {
    // Given o gabarito e `tolerancia.json`; When o desvio é recalculado; Then o arquivo
    // tem uma chave por `<execução>/<série>/<canal>` e o valor é o desvio medido.
    let medidas = medir(gabarito()?)?;
    let calculado: BTreeMap<String, f64> = medidas
        .iter()
        .map(|m| (m.chave.clone(), m.desvio))
        .collect();
    if std::env::var(GRAVAR).as_deref() == Ok("1") {
        let mut texto = serde_json::to_string_pretty(&calculado)
            .map_err(|e| format!("serializar tolerancia: {e}"))?;
        texto.push('\n');
        std::fs::write(CAMINHO_TOLERANCIA, texto)
            .map_err(|e| format!("gravar {CAMINHO_TOLERANCIA}: {e}"))?;
    }
    let t = ler(CAMINHO_TOLERANCIA)?;
    let gravado = objeto(&t, "tolerancia")?;
    let chaves: BTreeSet<String> = gravado.keys().cloned().collect();
    let esperadas = chaves_tolerancia();
    let mut falhas = Vec::new();
    if chaves != esperadas {
        falhas.push(format!(
            "chaves faltando {:?}, sobrando {:?}",
            esperadas.difference(&chaves).collect::<Vec<_>>(),
            chaves.difference(&esperadas).collect::<Vec<_>>()
        ));
    }
    for (chave, valor) in gravado {
        let lido = numero(valor, chave)?;
        match calculado.get(chave) {
            Some(&c) if c == lido => {}
            Some(&c) => falhas.push(format!("{chave}: gravado {lido}, medido {c}")),
            None => {}
        }
    }
    sem_falhas(falhas)
}
