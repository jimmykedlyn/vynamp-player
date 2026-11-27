//! Testes dos critérios de aceite da TASK-E02-13 sobre o gabarito commitado.
//!
//! `assets/gabarito/movimento.json` é a saída do gerador (`tools/gabarito-movimento/`);
//! `evidence/PR/task-e02-13/tolerancia.json` é a tolerância documentada. As listas esperadas
//! vêm da spec (`tasks/items/TASK-E02-13.md`) e do código do MVP fixado em
//! `0d9860b901510021bcca4c31a25014373ea7c452`, citado por arquivo e linha, nunca do JSON.

mod ac1;
mod ac2;
mod ac3;
pub mod json;
pub mod medicao;
pub mod svg;
