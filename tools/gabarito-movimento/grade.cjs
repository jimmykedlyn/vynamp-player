// Lógica pura do gerador do gabarito de movimento (TASK-E02-13, "Protocolo de medição").
// Sem Playwright nem DOM: testável com `node --test`. O mesmo arquivo é injetado na página
// (via `toString` das funções puras), então nada aqui depende de API do Node, exceto o WAV.
'use strict';

/** Commit do MVP fixado em design/07-pin-mvp.md; o gerador recusa qualquer outro (item 10). */
const MVP_COMMIT = '0d9860b901510021bcca4c31a25014373ea7c452';
/** Taxa do contexto offline e da fonte (item 2). */
const TAXA = 48000;
/** Duração do contexto offline em amostras: 2,0 s exatos, sem cauda (item 2). */
const AMOSTRAS = 96000;
/** Quantum de renderização da Web Audio (item 6). */
const QUANTUM = 128;
/** Número de quadros da grade: k de 0 a 119 (item 5). */
const QUADROS = 120;
/** Sinais do item 3, na ordem das execuções. */
const SINAIS = ['degrau', 'seno', 'rajada'];

/** Instante do quadro k em ms: `k*1000/60` em float64, nessa ordem de operações (item 5). */
function tK(k) {
  return k * 1000 / 60;
}

/** Amostra em que o contexto é suspenso no quadro k: `ceil(t_k*48/128)*128` (item 6). */
function amostraSuspensao(k) {
  return Math.ceil(tK(k) * 48 / QUANTUM) * QUANTUM;
}

/** Grade completa: `{k, t, amostra}` para k de 0 a 119. */
function quadros() {
  return Array.from({length: QUADROS}, (_, k) => ({k, t: tK(k), amostra: amostraSuspensao(k)}));
}

/** Envelope do item 3: o sinal está ligado na amostra n? */
function ligado(tipo, n) {
  if (n < 0 || n >= AMOSTRAS) return false;
  if (tipo === 'seno') return true;
  if (tipo === 'degrau') return n >= TAXA / 2;
  if (tipo === 'rajada') return n % (TAXA / 2) < TAXA * 50 / 1000;
  throw new Error('sinal desconhecido: ' + tipo);
}

/** Amostra n do sinal: `f32(0,5·sin(2π·1000·n/48000))` vezes o envelope do item 3. */
function sinal(tipo, n) {
  if (!ligado(tipo, n)) return 0;
  return Math.fround(0.5 * Math.sin(Math.PI * 2 * 1000 * n / TAXA));
}

/** As 96000 amostras do sinal, regeneradas por índice (item 8). */
function amostrasSinal(tipo) {
  const a = new Float32Array(AMOSTRAS);
  for (let n = 0; n < AMOSTRAS; n++) a[n] = sinal(tipo, n);
  return a;
}

/**
 * WAV PCM 16 bits, 48 kHz, estéreo idêntico (item 1). Só existe para o `<audio>` carregar sem
 * `error` e o `localWave` decodificar; o som medido vem da fonte sintética do stub (item 2).
 */
function wavPcm16Estereo(amostras) {
  const dados = amostras.length * 4;
  const b = Buffer.alloc(44 + dados);
  b.write('RIFF', 0, 'ascii');
  b.writeUInt32LE(36 + dados, 4);
  b.write('WAVE', 8, 'ascii');
  b.write('fmt ', 12, 'ascii');
  b.writeUInt32LE(16, 16);
  b.writeUInt16LE(1, 20);
  b.writeUInt16LE(2, 22);
  b.writeUInt32LE(TAXA, 24);
  b.writeUInt32LE(TAXA * 4, 28);
  b.writeUInt16LE(4, 32);
  b.writeUInt16LE(16, 34);
  b.write('data', 36, 'ascii');
  b.writeUInt32LE(dados, 40);
  for (let n = 0; n < amostras.length; n++) {
    const v = Math.round(Math.max(-1, Math.min(1, amostras[n])) * 32767);
    b.writeInt16LE(v, 44 + n * 4);
    b.writeInt16LE(v, 44 + n * 4 + 2);
  }
  return b;
}

module.exports = {
  MVP_COMMIT, TAXA, AMOSTRAS, QUANTUM, QUADROS, SINAIS,
  tK, amostraSuspensao, quadros, ligado, sinal, amostrasSinal, wavPcm16Estereo,
};
