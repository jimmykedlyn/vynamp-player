// Testes das funções puras da descrição do gabarito (TASK-E02-13, AC-1 e AC-3).
// Rodar: node --test tools/gabarito-movimento/
'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const d = require('./descricao.cjs');

test('transição: propriedade, duração em ms e easing, com var(--ease) resolvido', () => {
  assert.deepEqual(
    d.lerTransicao('transition:color 200ms var(--ease),opacity .2s', 'cubic-bezier(.2,.8,.2,1)'),
    [
      {propriedade: 'color', duracaoMs: 200, easing: 'cubic-bezier(.2,.8,.2,1)'},
      {propriedade: 'opacity', duracaoMs: 200, easing: 'ease'},
    ],
  );
  assert.deepEqual(d.lerTransicao('transition:width 90ms linear', 'x'), [{propriedade: 'width', duracaoMs: 90, easing: 'linear'}]);
});

test('animação: nome, duração, easing, iterações e direção', () => {
  assert.deepEqual(d.lerAnimacao('animation:media-wait 1s ease-in-out infinite alternate'), {
    nome: 'media-wait', duracaoMs: 1000, easing: 'ease-in-out', iteracoes: 'infinite', direcao: 'alternate',
  });
  assert.deepEqual(d.lerAnimacao('animation:graph-signal 3s linear infinite'), {
    nome: 'graph-signal', duracaoMs: 3000, easing: 'linear', iteracoes: 'infinite', direcao: 'normal',
  });
});

test('--ease é lido da folha', () => {
  assert.equal(d.lerEase(':root{--a:1;--ease:cubic-bezier(.2,.8,.2,1);--b:2}'), 'cubic-bezier(.2,.8,.2,1)');
  assert.throws(() => d.lerEase(':root{}'));
});

test('APIs de Web Audio de uma linha, na ordem e sem repetir', () => {
  assert.deepEqual(
    d.apisWebAudio('ctx=new AudioContext();g=ctx.createGain();h=ctx.createGain();a.getFloatFrequencyData(b)'),
    ['new AudioContext', '.createGain(', 'getFloatFrequencyData'],
  );
  assert.deepEqual(d.apisWebAudio('nada aqui'), []);
});

test('usos de Web Audio pulam linhas de comentário', () => {
  const usos = d.usosWebAudio({'a.js': '// ctx.createGain()\nx=ctx.createGain();\n * decodeAudioData\n'});
  assert.deepEqual(usos, [{arquivo: 'a.js', linha: 2, apis: ['.createGain(']}]);
});

test('defaultConfigs e liveModes saem do código do MVP', () => {
  const fonte = "const defaultConfigs={'a':{x:1},'b':{gain:1.0}};\n" +
    "const creativeModes=new Set(['b']),spectrumModes=new Set([]),advancedModes=new Set([]),liveModes=new Set([...creativeModes]);";
  assert.deepEqual(d.configsDoMvp(fonte), {configs: {a: {x: 1}, b: {gain: 1}}, aoVivo: ['b']});
});

test('descrição ausente para um uso da varredura falha', () => {
  assert.throws(() => d.exigir({}, 'x.js:1', 'Web Audio'), /x\.js:1/);
  assert.equal(d.exigir({'x.js:1': 'ok'}, 'x.js:1', 'Web Audio'), 'ok');
});
