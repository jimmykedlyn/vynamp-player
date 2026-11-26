// Testes da lógica isolada do gerador (TASK-E02-13, "Protocolo de medição").
// Rodar: node --test tools/gabarito-movimento/
'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const g = require('./grade.cjs');

test('SHA do MVP é o literal do pin', () => {
  assert.equal(g.MVP_COMMIT, '0d9860b901510021bcca4c31a25014373ea7c452');
});

test('t_k é k*1000/60 em float64 nessa ordem (item 5)', () => {
  assert.equal(g.tK(0), 0);
  assert.equal(g.tK(1), 1000 / 60);
  assert.equal(g.tK(7), 7 * 1000 / 60);
  assert.equal(g.tK(119), 119 * 1000 / 60);
});

test('amostra de suspensão é ceil(t_k*48/128)*128 (item 6)', () => {
  assert.equal(g.amostraSuspensao(0), 0);
  // t_1 = 16,67 ms → 6,25 quanta → 7 quanta.
  assert.equal(g.amostraSuspensao(1), 896);
  // t_4 = 66,67 ms → 25 quanta exatos: fica na fronteira.
  assert.equal(g.amostraSuspensao(4), 3200);
  assert.equal(g.amostraSuspensao(60), 48000);
  assert.equal(g.amostraSuspensao(119), 95232);
});

test('grade tem 120 quadros distintos, em ordem, múltiplos de 128 e dentro de 96000', () => {
  const q = g.quadros();
  assert.equal(q.length, 120);
  q.forEach((x, k) => {
    assert.equal(x.k, k);
    assert.equal(x.t, g.tK(k));
    assert.equal(x.amostra, g.amostraSuspensao(k));
    assert.equal(x.amostra % 128, 0);
    assert.ok(x.amostra < g.AMOSTRAS);
    if (k) assert.ok(x.amostra > q[k - 1].amostra);
  });
});

test('sinal: portadora de 1 kHz, amplitude 0,5, fase 0, em f32 (item 3)', () => {
  // 2π·1000·12/48000 = π/2.
  assert.equal(g.sinal('seno', 12), 0.5);
  assert.equal(g.sinal('seno', 0), 0);
  assert.equal(g.sinal('seno', 6), Math.fround(0.5 * Math.sin(Math.PI * 2 * 1000 * 6 / 48000)));
  assert.equal(g.sinal('seno', 95999), Math.fround(0.5 * Math.sin(Math.PI * 2 * 1000 * 95999 / 48000)));
});

test('degrau: silêncio até 0,5 s e portadora depois', () => {
  assert.equal(g.sinal('degrau', 12), 0);
  assert.equal(g.sinal('degrau', 23999), 0);
  assert.equal(g.sinal('degrau', 24012), g.sinal('seno', 24012));
  assert.notEqual(g.sinal('degrau', 24012), 0);
});

test('rajada: 50 ms ligada e 450 ms desligada desde t = 0', () => {
  assert.equal(g.sinal('rajada', 12), 0.5);
  assert.equal(g.sinal('rajada', 2399), g.sinal('seno', 2399));
  assert.equal(g.sinal('rajada', 2400 + 12), 0);
  assert.equal(g.sinal('rajada', 24000 + 12), g.sinal('seno', 24000 + 12));
  assert.equal(g.sinal('rajada', 72000 + 2412), 0);
  assert.notEqual(g.sinal('rajada', 72000 + 12), 0);
});

test('amostrasSinal devolve 96000 amostras iguais a sinal(tipo, n)', () => {
  for (const tipo of g.SINAIS) {
    const a = g.amostrasSinal(tipo);
    assert.ok(a instanceof Float32Array);
    assert.equal(a.length, 96000);
    for (const n of [0, 12, 2399, 2412, 23999, 24012, 50000, 95999]) assert.equal(a[n], g.sinal(tipo, n));
  }
});

test('WAV é PCM 16 bits, 48 kHz, estéreo, com as 96000 amostras nos dois canais', () => {
  const a = g.amostrasSinal('seno');
  const w = g.wavPcm16Estereo(a);
  assert.equal(w.length, 44 + 96000 * 4);
  assert.equal(w.toString('ascii', 0, 4), 'RIFF');
  assert.equal(w.readUInt32LE(4), w.length - 8);
  assert.equal(w.toString('ascii', 8, 12), 'WAVE');
  assert.equal(w.toString('ascii', 12, 16), 'fmt ');
  assert.equal(w.readUInt32LE(16), 16);
  assert.equal(w.readUInt16LE(20), 1);
  assert.equal(w.readUInt16LE(22), 2);
  assert.equal(w.readUInt32LE(24), 48000);
  assert.equal(w.readUInt32LE(28), 48000 * 4);
  assert.equal(w.readUInt16LE(32), 4);
  assert.equal(w.readUInt16LE(34), 16);
  assert.equal(w.toString('ascii', 36, 40), 'data');
  assert.equal(w.readUInt32LE(40), 96000 * 4);
  // n = 12: 0,5 → round(0,5·32767) = 16384 nos dois canais.
  assert.equal(w.readInt16LE(44 + 12 * 4), 16384);
  assert.equal(w.readInt16LE(44 + 12 * 4 + 2), 16384);
});
