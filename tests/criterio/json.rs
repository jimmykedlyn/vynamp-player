//! Leitura do gabarito e acesso tipado aos campos, com erro como dado.

use serde_json::{Map, Value};
use std::sync::OnceLock;

/// Caminho do gabarito gerado (item 10 do "Protocolo de medição").
pub const CAMINHO_GABARITO: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/gabarito/movimento.json"
);

/// Caminho da tolerância documentada (item 10 do "Protocolo de medição").
pub const CAMINHO_TOLERANCIA: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/evidence/PR/task-e02-13/tolerancia.json"
);

/// Lê e interpreta um arquivo JSON.
pub fn ler(caminho: &str) -> Result<Value, String> {
    let texto = std::fs::read_to_string(caminho).map_err(|e| format!("{caminho}: {e}"))?;
    serde_json::from_str(&texto).map_err(|e| format!("{caminho}: JSON inválido: {e}"))
}

/// O gabarito, lido uma vez por processo de teste (são cerca de 9 MB).
pub fn gabarito() -> Result<&'static Value, String> {
    static GABARITO: OnceLock<Result<Value, String>> = OnceLock::new();
    GABARITO
        .get_or_init(|| ler(CAMINHO_GABARITO))
        .as_ref()
        .map_err(Clone::clone)
}

/// Campo `chave` de um objeto; `ctx` diz onde, para a mensagem de erro.
pub fn campo<'a>(v: &'a Value, chave: &str, ctx: &str) -> Result<&'a Value, String> {
    v.get(chave)
        .ok_or_else(|| format!("{ctx}: campo `{chave}` ausente"))
}

/// O valor como texto.
pub fn texto<'a>(v: &'a Value, ctx: &str) -> Result<&'a str, String> {
    v.as_str()
        .ok_or_else(|| format!("{ctx}: esperado texto, veio {v}"))
}

/// O valor como número `f64`.
pub fn numero(v: &Value, ctx: &str) -> Result<f64, String> {
    v.as_f64()
        .ok_or_else(|| format!("{ctx}: esperado número, veio {v}"))
}

/// O valor como inteiro sem sinal.
pub fn inteiro(v: &Value, ctx: &str) -> Result<u64, String> {
    v.as_u64()
        .ok_or_else(|| format!("{ctx}: esperado inteiro sem sinal, veio {v}"))
}

/// O valor como lista.
pub fn lista<'a>(v: &'a Value, ctx: &str) -> Result<&'a Vec<Value>, String> {
    v.as_array()
        .ok_or_else(|| format!("{ctx}: esperado lista, veio {}", resumo(v)))
}

/// O valor como objeto.
pub fn objeto<'a>(v: &'a Value, ctx: &str) -> Result<&'a Map<String, Value>, String> {
    v.as_object()
        .ok_or_else(|| format!("{ctx}: esperado objeto, veio {}", resumo(v)))
}

/// Lista de textos.
pub fn textos<'a>(v: &'a Value, ctx: &str) -> Result<Vec<&'a str>, String> {
    lista(v, ctx)?
        .iter()
        .enumerate()
        .map(|(i, x)| texto(x, &format!("{ctx}[{i}]")))
        .collect()
}

/// Começo do valor, para mensagens curtas.
fn resumo(v: &Value) -> String {
    let s = v.to_string();
    s.chars().take(80).collect()
}

/// Junta as falhas numa mensagem só; `Ok` se não houver nenhuma.
pub fn sem_falhas(falhas: Vec<String>) -> Result<(), String> {
    if falhas.is_empty() {
        Ok(())
    } else {
        Err(format!("{} falha(s):\n{}", falhas.len(), falhas.join("\n")))
    }
}
