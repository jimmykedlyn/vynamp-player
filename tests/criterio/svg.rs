//! Leitura do `d` que o MVP escreveu e comparação com a geometria do oráculo
//! (item 6 de "Estilos de onda": mesma sequência de comandos, abscissas a até 0,005,
//! ordenadas e `v` no teto da tabela vezes o fator, mais 0,005).

use crate::oraculo::estilos_espectro::Comando;

/// Meia unidade do `toFixed(2)` do MVP: o teto das abscissas e a folga de cada ordenada.
pub const MEIO_TO_FIXED_2: f64 = 0.005;

/// Teto de uma geometria, pela tabela do item 6 de "Estilos de onda".
#[derive(Debug, Clone, Copy)]
pub struct Tetos {
    /// Parte da escala do teto da ordenada (por exemplo, `22·1e-4`), sem o 0,005.
    pub escala: f64,
    /// Fator com que o código multiplica a altura no `v` (2 em `columns-pulse` e
    /// `prisma-neon`, 1 no resto).
    pub fator_v: f64,
}

impl Tetos {
    /// Teto da ordenada absoluta: escala + 0,005.
    pub fn ordenada(&self) -> f64 {
        self.escala + MEIO_TO_FIXED_2
    }

    /// Teto do comprimento relativo `v`: escala · fator + 0,005.
    pub fn v(&self) -> f64 {
        self.escala * self.fator_v + MEIO_TO_FIXED_2
    }
}

/// Lê um `d` com as letras que o MVP escreve (`M L v h H a z Z`). `h` e `H` viram
/// `Comando::H`: o `H` maiúsculo só aparece no caminho vazio (`M0 24H1000`, `M0 46H1000`),
/// em que o oráculo também usa `Comando::H(1000)`.
pub fn ler_d(d: &str) -> Result<Vec<Comando>, String> {
    let mut comandos = Vec::new();
    let mut resto = d.trim();
    while let Some(letra) = resto.chars().next() {
        resto = &resto[letra.len_utf8()..];
        let (numeros, depois) = ler_numeros(resto)?;
        resto = depois;
        let exige = |n: usize| -> Result<(), String> {
            if numeros.len() == n {
                Ok(())
            } else {
                Err(format!(
                    "comando `{letra}` com {} número(s), esperado {n}, em {d:.80}",
                    numeros.len()
                ))
            }
        };
        let c = match letra {
            'M' => {
                exige(2)?;
                Comando::M(numeros[0], numeros[1])
            }
            'L' => {
                exige(2)?;
                Comando::L(numeros[0], numeros[1])
            }
            'v' => {
                exige(1)?;
                Comando::V(numeros[0])
            }
            'h' | 'H' => {
                exige(1)?;
                Comando::H(numeros[0])
            }
            'z' | 'Z' => {
                exige(0)?;
                Comando::Z
            }
            'a' => {
                exige(7)?;
                Comando::A(
                    numeros[0], numeros[1], numeros[2], numeros[3], numeros[4], numeros[5],
                    numeros[6],
                )
            }
            outra => {
                return Err(format!(
                    "letra `{outra}` fora do vocabulário do MVP em {d:.80}"
                ));
            }
        };
        comandos.push(c);
    }
    Ok(comandos)
}

/// Lê os números até a próxima letra de comando.
fn ler_numeros(s: &str) -> Result<(Vec<f64>, &str), String> {
    let fim = s.find(|c: char| c.is_ascii_alphabetic()).unwrap_or(s.len());
    let (trecho, depois) = s.split_at(fim);
    let mut numeros = Vec::new();
    let mut atual = String::new();
    let fecha = |atual: &mut String, numeros: &mut Vec<f64>| -> Result<(), String> {
        if !atual.is_empty() {
            numeros.push(
                atual
                    .parse::<f64>()
                    .map_err(|e| format!("número `{atual}` inválido: {e}"))?,
            );
            atual.clear();
        }
        Ok(())
    };
    for c in trecho.chars() {
        match c {
            ' ' | ',' => fecha(&mut atual, &mut numeros)?,
            '-' if !atual.is_empty() => {
                fecha(&mut atual, &mut numeros)?;
                atual.push(c);
            }
            _ => atual.push(c),
        }
    }
    fecha(&mut atual, &mut numeros)?;
    Ok((numeros, depois))
}

/// Nome curto do comando, para mensagens.
fn letra(c: &Comando) -> &'static str {
    match c {
        Comando::M(..) => "M",
        Comando::L(..) => "L",
        Comando::V(..) => "v",
        Comando::H(..) => "h",
        Comando::Z => "z",
        Comando::A(..) => "a",
    }
}

/// Compara a geometria do MVP com a do oráculo. Devolve o maior desvio absoluto entre
/// todos os números e as violações (sequência diferente ou valor fora do teto).
pub fn comparar(mvp: &[Comando], oraculo: &[Comando], tetos: Tetos) -> (f64, Vec<String>) {
    let mut desvio = 0.0f64;
    let mut violacoes = Vec::new();
    let mut estrutura_igual = mvp.len() == oraculo.len();
    if !estrutura_igual {
        violacoes.push(format!(
            "sequência com {} comandos no MVP e {} no oráculo",
            mvp.len(),
            oraculo.len()
        ));
    }
    for (i, (a, b)) in mvp.iter().zip(oraculo).enumerate() {
        // (valor do MVP, valor do oráculo, teto, papel)
        let pares: Vec<(f64, f64, f64, &str)> = match (a, b) {
            (Comando::M(x, y), Comando::M(x2, y2)) | (Comando::L(x, y), Comando::L(x2, y2)) => {
                vec![
                    (*x, *x2, MEIO_TO_FIXED_2, "abscissa"),
                    (*y, *y2, tetos.ordenada(), "ordenada"),
                ]
            }
            (Comando::V(v), Comando::V(v2)) => vec![(*v, *v2, tetos.v(), "v")],
            (Comando::H(h), Comando::H(h2)) => vec![(*h, *h2, MEIO_TO_FIXED_2, "h")],
            (Comando::Z, Comando::Z) => Vec::new(),
            (Comando::A(a0, a1, a2, a3, a4, a5, a6), Comando::A(b0, b1, b2, b3, b4, b5, b6)) => [
                (a0, b0),
                (a1, b1),
                (a2, b2),
                (a3, b3),
                (a4, b4),
                (a5, b5),
                (a6, b6),
            ]
            .iter()
            .map(|(p, q)| (**p, **q, MEIO_TO_FIXED_2, "arco"))
            .collect(),
            _ => {
                estrutura_igual = false;
                violacoes.push(format!(
                    "comando {i}: `{}` no MVP, `{}` no oráculo",
                    letra(a),
                    letra(b)
                ));
                break;
            }
        };
        for (p, q, teto, papel) in pares {
            let d = (p - q).abs();
            desvio = desvio.max(d);
            if d > teto || d.is_nan() {
                violacoes.push(format!(
                    "comando {i} `{}`: {papel} MVP {p}, oráculo {q}, desvio {d:.6e} > teto {teto:.6e}",
                    letra(a)
                ));
            }
        }
    }
    if !estrutura_igual {
        desvio = f64::INFINITY;
    }
    (desvio, violacoes)
}

#[test]
fn ler_d_de_cada_forma_do_mvp() -> Result<(), String> {
    // Formas copiadas de waveform-styles.js:79, :80 e :84 (strings do gabarito).
    assert_eq!(
        ler_d("M532.75 46v-2.69h12.63v2.69z")?,
        vec![
            Comando::M(532.75, 46.0),
            Comando::V(-2.69),
            Comando::H(12.63),
            Comando::V(2.69),
            Comando::Z
        ]
    );
    assert_eq!(
        ler_d("M1 2.5h3v1.6h-3z")?,
        vec![
            Comando::M(1.0, 2.5),
            Comando::H(3.0),
            Comando::V(1.6),
            Comando::H(-3.0),
            Comando::Z
        ]
    );
    assert_eq!(
        ler_d("M0 46H1000")?,
        vec![Comando::M(0.0, 46.0), Comando::H(1000.0)]
    );
    assert_eq!(
        ler_d("M482.68 23.24a1.7 1.7 0 1 0 3.4 0a1.7 1.7 0 1 0 -3.4 0")?,
        vec![
            Comando::M(482.68, 23.24),
            Comando::A(1.7, 1.7, 0.0, 1.0, 0.0, 3.4, 0.0),
            Comando::A(1.7, 1.7, 0.0, 1.0, 0.0, -3.4, 0.0)
        ]
    );
    assert_eq!(
        ler_d("M0 24L500 23.5L1000 24Z")?,
        vec![
            Comando::M(0.0, 24.0),
            Comando::L(500.0, 23.5),
            Comando::L(1000.0, 24.0),
            Comando::Z
        ]
    );
    assert_eq!(ler_d("")?, Vec::<Comando>::new());
    assert!(
        ler_d("M0 24Q1 2").is_err(),
        "letra fora do vocabulário é erro"
    );
    assert!(ler_d("M0").is_err(), "M com um número é erro");
    Ok(())
}

#[test]
fn comparar_aplica_teto_por_papel() {
    let t = Tetos {
        escala: 1e-3,
        fator_v: 2.0,
    };
    // Abscissa dentro de 0,005, ordenada dentro de 0,006, v dentro de 0,007.
    let (d, v) = comparar(
        &[Comando::M(1.004, 2.0059), Comando::V(3.0069)],
        &[Comando::M(1.0, 2.0), Comando::V(3.0)],
        t,
    );
    assert!(v.is_empty(), "{v:?}");
    assert!((d - 0.0069).abs() < 1e-12);
    // Ordenada 0,0061 passa do teto de 0,006.
    let (_, v) = comparar(&[Comando::M(1.0, 2.0061)], &[Comando::M(1.0, 2.0)], t);
    assert_eq!(v.len(), 1, "{v:?}");
    // Sequência diferente é violação, com desvio infinito.
    let (d, v) = comparar(&[Comando::M(0.0, 0.0)], &[], t);
    assert!(!v.is_empty() && d.is_infinite());
    let (d, v) = comparar(&[Comando::Z], &[Comando::V(0.0)], t);
    assert!(!v.is_empty() && d.is_infinite());
}
