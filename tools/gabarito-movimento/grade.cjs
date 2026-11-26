// Lógica pura do gerador do gabarito de movimento (TASK-E02-13, "Protocolo de medição").
// Sem Playwright nem DOM: testável com `node --test`.
'use strict';

const nao = nome => () => { throw new Error('não implementado: ' + nome); };

module.exports = {
  MVP_COMMIT: '',
  AMOSTRAS: 96000,
  SINAIS: ['degrau', 'seno', 'rajada'],
  tK: nao('tK'),
  amostraSuspensao: nao('amostraSuspensao'),
  quadros: nao('quadros'),
  sinal: nao('sinal'),
  amostrasSinal: nao('amostrasSinal'),
  wavPcm16Estereo: nao('wavPcm16Estereo'),
};
