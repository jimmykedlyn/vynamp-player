'use strict';
const ni = n => () => { throw new Error('não implementado: ' + n); };
module.exports = {
  lerTransicao: ni('lerTransicao'), lerAnimacao: ni('lerAnimacao'), lerEase: ni('lerEase'),
  apisWebAudio: ni('apisWebAudio'), usosWebAudio: ni('usosWebAudio'), configsDoMvp: ni('configsDoMvp'), exigir: ni('exigir'),
};
