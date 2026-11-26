//! Oráculo para TASK-E02-13: sinal, grade temporal e nível.
//!
//! Código de teste, nunca de produção. Regenera o sinal analiticamente
//! a partir do índice de amostra e reimplementa a cadência de quadros
//! e o cálculo de RMS → dB do protocolo de medição.

pub mod analisador;
pub mod estilos_espectro;
pub mod estilos_temporais;
pub mod grade;
pub mod nivel;
pub mod sinal;
