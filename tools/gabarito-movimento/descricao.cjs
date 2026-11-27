// Descrição do AC-1 (transições, animações, rAF/WAAPI) e do AC-3 (estilos de onda e
// Web Audio) do gabarito de movimento da TASK-E02-13.
//
// A lista de locais vem sempre da varredura do código do MVP; as tabelas abaixo só dão
// o texto de cada local, e um local da varredura sem texto faz o gerador falhar.
'use strict';
const fs = require('fs');
const path = require('path');
const vm = require('vm');

const EASINGS = /^(ease|ease-in|ease-out|ease-in-out|linear|step-start|step-end|cubic-bezier\(.*\)|steps\(.*\)|var\(--ease\))$/;
const TEMPO = /^(\d*\.?\d+)(ms|s)$/;

/** Separa por vírgula fora de parênteses. */
function porVirgula(s) {
  const partes = [];
  let nivel = 0, atual = '';
  for (const c of s) {
    if (c === '(') nivel++;
    if (c === ')') nivel--;
    if (c === ',' && nivel === 0) { partes.push(atual.trim()); atual = ''; } else atual += c;
  }
  if (atual.trim()) partes.push(atual.trim());
  return partes;
}

const ms = t => { const m = t.match(TEMPO); return m[2] === 's' ? Number(m[1]) * 1000 : Number(m[1]); };
const valor = (texto, prop) => texto.replace(new RegExp('^\\s*' + prop + '(-[a-z-]+)?\\s*:\\s*'), '').replace(/\s*!important\s*$/, '');

/** `transition: …` → `[{propriedade, duracaoMs, easing}]`, com `var(--ease)` trocado por `ease`. */
function lerTransicao(texto, ease) {
  return porVirgula(valor(texto, 'transition')).map(item => {
    let propriedade = 'all', duracaoMs = 0, easing = 'ease', viuTempo = false;
    for (const tok of item.split(/\s+/)) {
      if (TEMPO.test(tok)) { if (!viuTempo) { duracaoMs = ms(tok); viuTempo = true; } } else if (EASINGS.test(tok)) easing = tok === 'var(--ease)' ? ease : tok;
      else propriedade = tok;
    }
    return {propriedade, duracaoMs, easing};
  });
}

/** `animation: …` → `{nome, duracaoMs, easing, iteracoes, direcao}`. */
function lerAnimacao(texto) {
  const a = {nome: 'none', duracaoMs: 0, easing: 'ease', iteracoes: '1', direcao: 'normal'};
  let viuTempo = false;
  for (const tok of valor(texto, 'animation').split(/\s+/)) {
    if (TEMPO.test(tok)) { if (!viuTempo) { a.duracaoMs = ms(tok); viuTempo = true; } } else if (EASINGS.test(tok)) a.easing = tok;
    else if (tok === 'infinite' || /^\d*\.?\d+$/.test(tok)) a.iteracoes = tok;
    else if (/^(normal|reverse|alternate|alternate-reverse)$/.test(tok)) a.direcao = tok;
    else a.nome = tok;
  }
  return a;
}

/** `--ease` da folha (player.css:1). */
function lerEase(css) {
  const m = css.match(/--ease\s*:\s*([^;}]+)/);
  if (!m) throw new Error('--ease ausente da folha');
  return m[1].trim();
}

/** Mesma expressão da varredura de Web Audio do AC-3. */
const RE_WEB_AUDIO = /new \((window\.)?(Offline|webkit)?AudioContext(\|\|(window\.)?(Offline|webkit)?AudioContext)*\)|new (window\.)?(Offline|webkit)?AudioContext|AudioWorkletNode|AudioWorkletProcessor|registerProcessor|audioWorklet|\.create(Analyser|Gain|BiquadFilter|Delay|Convolver|DynamicsCompressor|WaveShaper|StereoPanner|ChannelSplitter|ChannelMerger|Oscillator|BufferSource|Buffer|MediaElementSource|ConstantSource|IIRFilter|Panner)\(|decodeAudioData|startRendering|getFloat(TimeDomain|Frequency)Data|getByte(TimeDomain|Frequency)Data|getFrequencyResponse|setValueAtTime|linearRampToValueAtTime|setTargetAtTime|exponentialRampToValueAtTime/g;

/** APIs de Web Audio de uma linha, na ordem da primeira ocorrência. */
function apisWebAudio(linha) {
  return [...new Set([...linha.matchAll(RE_WEB_AUDIO)].map(m => m[0]))];
}

/** `{arquivo: fonte}` → usos por linha, arquivos em ordem, pulando linhas de comentário. */
function usosWebAudio(fontes) {
  const usos = [];
  for (const arquivo of Object.keys(fontes).sort()) {
    fontes[arquivo].split('\n').forEach((l, i) => {
      if (/^\s*(\/\/|\*|\/\*)/.test(l)) return;
      const apis = apisWebAudio(l);
      if (apis.length) usos.push({arquivo, linha: i + 1, apis});
    });
  }
  return usos;
}

/** `defaultConfigs` e `liveModes` avaliados do próprio `waveform-styles.js`. */
function configsDoMvp(fonte) {
  const dc = fonte.match(/const defaultConfigs=(\{[\s\S]*?\});/);
  const modos = fonte.match(/const creativeModes=[^;]*liveModes=new Set\([^;]*\);/);
  if (!dc || !modos) throw new Error('defaultConfigs ou liveModes ausente de waveform-styles.js');
  const ctx = {};
  vm.runInNewContext('configs=' + dc[1] + ';' + modos[0] + 'aoVivo=[...liveModes];', ctx);
  return JSON.parse(JSON.stringify({configs: ctx.configs, aoVivo: ctx.aoVivo}));
}

/** Texto da tabela para um local da varredura; falha se faltar. */
function exigir(tabela, chave, rotulo) {
  if (!Object.prototype.hasOwnProperty.call(tabela, chave)) throw new Error(`${rotulo}: local ${chave} da varredura sem descrição`);
  return tabela[chave];
}

// ---------------------------------------------------------------------------------
// AC-1: transições e animações CSS que não são `none`.

const RM = 'refinement40.css:726:3', RM_ANIM = 'refinement40.css:725:3', HTML_RM = 'spec-controls.css:72:38';

/** Seletor, estados e gatilho de cada transição/animação, por `arquivo:linha:coluna`. */
const TRANSICOES = {
  'audio-controls.css:3:677': {seletor: '.switch-control input[type=checkbox]::after', de: 'transform: none (left 4px)', para: 'transform: translateX(18px); translateX(26px) com html[data-touch=true] (audio-controls.css:11)', gatilho: 'input:checked', reducao: ['audio-controls.css:8:68', RM]},
  'graph-editor.css:1:4014': {seletor: '.graph-flow', de: 'stroke-dashoffset: 0 (implícito, a regra não o define)', para: 'stroke-dashoffset: -96 (@keyframes graph-signal, graph-editor.css:1)', gatilho: 'contínua enquanto o fio .graph-flow está no DOM', reducao: ['graph-editor.css:5:51', 'graph-editor.css:7:44', RM_ANIM, 'spec-controls.css:72:66']},
  'graph-editor.css:1:6061': {seletor: '.graph-node-meter span', de: 'width anterior (0 no início)', para: 'width nova que graph-editor.js:85 escreve a cada leitura do medidor do nó', gatilho: 'escrita de style.width pelo laço do editor de grafo', reducao: ['graph-editor.css:5:89', 'workspace-layout.css:4:55', RM, HTML_RM]},
  'library-workspace.css:7:590': {seletor: '#media-state[data-state=loading]::before, #media-state[data-state=buffering]::before', de: 'opacity: 1 (implícito)', para: 'opacity: .3 (@keyframes media-wait, library-workspace.css:7), ida e volta', gatilho: 'data-state loading ou buffering, escrito por playback-status.js:3', reducao: ['library-workspace.css:10:59', 'library-workspace.css:10:137', RM_ANIM]},
  'lyrics.css:1:881': {seletor: '.lyrics-line', de: 'color var(--text-muted); opacity 1 (lyrics.css:4)', para: '.current: color var(--text-primary), opacity 1; .outgoing: opacity 0 (lyrics.css:4)', gatilho: 'troca das classes current/outgoing na mudança de linha da letra', reducao: ['lyrics.css:3:117', 'lyrics.css:3:184', RM, HTML_RM]},
  'player.css:2:217': {seletor: 'button, a, select', de: 'cores de repouso do controle', para: 'cores de :hover, :focus-visible ou estado pressionado do controle', gatilho: 'hover, foco ou mudança de estado; duração sobreposta para 80ms por spec-controls.css:4', reducao: [HTML_RM, RM, 'usability.css:26:150']},
  'refinement28.css:15:235': {seletor: '#restore-workspace button:not(.icon-button)', de: 'cores de repouso', para: 'cores de :hover, :focus-visible ou estado do botão', gatilho: 'hover, foco ou mudança de estado (especificidade vence o 80ms de spec-controls.css:4)', reducao: ['refinement28.css:34:65', 'restore.css:15:60', RM]},
  'refinement39.css:46:121': {seletor: ':root[data-faithful] #queue-list .queue-item :is([data-remove], .queue-order)', de: 'opacity 0 fora de hover e foco (refinement39.css:49)', para: 'opacity 1 em hover ou focus-within (refinement40.css:968)', gatilho: 'hover ou focus-within no item da fila', reducao: [RM, HTML_RM, 'usability.css:26:150']},
  'restore.css:24:134': {seletor: ':root[data-restore-processing=true] #restore-workspace', de: 'sem borda esquerda nem sombra', para: 'border-left 2px solid var(--lime); box-shadow inset 2px 0 8px -2px var(--lime)', gatilho: 'data-restore-processing=true; só a entrada transiciona, a saída é imediata porque a regra deixa de valer', reducao: [RM, HTML_RM]},
  'restore.css:35:196': {seletor: '.diag-ts', de: 'background de repouso', para: 'background var(--text-secondary) em :hover (restore.css:36)', gatilho: 'hover no carimbo de tempo do diagnóstico', reducao: ['restore.css:37:48', 'restore.css:15:60', RM]},
  'restore.css:62:71': {seletor: '.restore-batch-bar i', de: 'width anterior do progresso do lote', para: 'width nova escrita pelo restaurador', gatilho: 'avanço do lote de restauração', reducao: ['restore.css:75:60', 'restore.css:15:60', RM]},
  'waveform-styles.css:48:228': {seletor: '.visualizer-gear', de: 'color var(--text-muted) e a opacidade de repouso', para: 'color var(--cyan); opacity 1 em :hover ou :focus-visible', gatilho: 'hover ou foco no botão de engrenagem', reducao: [RM, HTML_RM]},
  'waveform-styles.css:50:155': {seletor: '.wave-color-swatch', de: 'border-color var(--line)', para: 'border-color var(--cyan) em :hover ou :focus-visible', gatilho: 'hover ou foco na amostra de cor', reducao: [RM, HTML_RM]},
  'waveform-styles.css:52:195': {seletor: '.wave-neon-swatch', de: 'border-color var(--line), sem sombra', para: 'border-color var(--cyan) em :hover/:focus-visible; box-shadow 0 0 0 2px var(--cyan) com aria-pressed=true', gatilho: 'hover, foco ou seleção da cor neon', reducao: [RM, HTML_RM]},
};

/** `@keyframes nome{…}` de todas as folhas → quadros `{chave, estilo}`. */
function keyframesDasFolhas(folhas) {
  const k = {};
  for (const s of Object.values(folhas)) {
    for (const m of s.matchAll(/@keyframes\s+([\w-]+)\s*\{((?:[^{}]*\{[^{}]*\})*)\s*\}/g)) {
      k[m[1]] = [...m[2].matchAll(/([^{}]+)\{([^{}]*)\}/g)].map(q => ({chave: q[1].trim(), estilo: q[2].trim()}));
    }
  }
  return k;
}

/** Transições e animações resolvidas, na ordem da varredura estática. */
function descreverTransicoes(mvp, estatica) {
  const folhas = Object.fromEntries(estatica.css.map(f => [f.arquivo, fs.readFileSync(path.join(mvp, f.arquivo), 'utf8')]));
  const ease = lerEase(fs.readFileSync(path.join(mvp, 'player.css'), 'utf8'));
  const quadros = keyframesDasFolhas(folhas);
  // spec-controls.css:4 (`button, a, select{transition-duration:80ms}`) vem depois de
  // player.css e tem a mesma especificidade: sobrepõe a duração de player.css:2.
  const sobreposicao = {'player.css:2:217': 80};
  const lista = [];
  for (const f of estatica.css) {
    for (const d of f.declaracoes) {
      if (/:\s*none/.test(d.texto) || /^transition-duration/.test(d.texto)) continue;
      const origem = `${f.arquivo}:${d.linha}:${d.coluna}`;
      const t = exigir(TRANSICOES, origem, 'transição');
      if (/^animation/.test(d.texto)) {
        const a = lerAnimacao(d.texto);
        const q = quadros[a.nome];
        if (!q) throw new Error(`${origem}: @keyframes ${a.nome} ausente`);
        const prop = q.at(-1).estilo.split(':')[0].trim();
        lista.push({origem, tipo: 'animation', seletor: t.seletor, nome: a.nome,
          propriedades: [{propriedade: prop, duracaoMs: a.duracaoMs, easing: a.easing}],
          iteracoes: a.iteracoes, direcao: a.direcao,
          keyframes: [{chave: 'from', estilo: t.de}, ...q.map(x => ({chave: x.chave, estilo: x.estilo}))],
          gatilho: t.gatilho, reducao: t.reducao});
      } else {
        const props = lerTransicao(d.texto, ease).map(p => (sobreposicao[origem] ? {...p, duracaoMs: sobreposicao[origem]} : p));
        lista.push({origem, tipo: 'transition', seletor: t.seletor, propriedades: props, de: t.de, para: t.para, gatilho: t.gatilho, reducao: t.reducao});
      }
    }
  }
  return lista;
}

// ---------------------------------------------------------------------------------
// AC-1: requestAnimationFrame e WAAPI.

const LACO_MEDIDOR = 'agenda o laço de player.js:70';
const LACO_ONDA = 'agenda o laço de waveform-styles.js:89';

/** Comportamento de cada chamada de rAF ou `.animate`, por `arquivo:linha`. */
const COMPORTAMENTOS = {
  'advanced-eq.js:109': {tipo: 'rAF único coalescido', cadencia: 'um quadro por rajada de arrasto', motivo: 'redesenha as bandas do EQ avançado durante o arrasto; depende do ponteiro, não do áudio'},
  'eq-spectrum.js:22': {tipo: 'laço rAF', cadencia: 't-last>=(motion.matches||api.state.reducedMotion?250:0)', motivo: 'espectro do diálogo de EQ (VynampSpectrumPulse.update com 10 dB e .45·taxa); não é o medidor nem a onda do UI-AC-13, fora do AC-2'},
  'eq-spectrum.js:23': {tipo: 'início do laço rAF', cadencia: 'agenda o laço de eq-spectrum.js:22', motivo: 'idem eq-spectrum.js:22'},
  'graph-editor.js:30': {tipo: 'rAF único coalescido', cadencia: 'um quadro por pedido de redesenho', motivo: 'redesenha os fios do editor de grafo; depende da edição, não do áudio'},
  'graph-properties.js:20': {tipo: 'laço rAF', cadencia: "time-lastFrame>=(motion.matches||api.state.reducedMotion?250:mode==='spectrum'?0:50)", motivo: "inspetor do nó do grafo (no modo espectro zera o smoothing e chama pulse.update com max(16,min(96,floor(width/7))) barras); fora do AC-2"},
  'graph-properties.js:21': {tipo: 'início do laço rAF', cadencia: 'agenda o laço de graph-properties.js:20', motivo: 'idem graph-properties.js:20'},
  'index.html:6': {tipo: 'rAF único', cadencia: 'uma vez no DOMContentLoaded (setTimeout de 4000 ms como reserva)', motivo: 'marca a página como pronta no boot; não anima'},
  'lyrics.js:32': {tipo: 'WAAPI', cadencia: 'uma animação por troca de linha da letra', duracaoMs: 260, easing: 'cubic-bezier(.2,.7,.2,1)', preenchimento: 'forwards', keyframes: [{transform: 'translateY(0)'}, {transform: 'translateY(-${height}px)'}], motivo: 'rola a faixa da letra; altura medida no DOM, sem dependência do áudio'},
  'meter-hairline.js:15': {tipo: 'rAF único por mutação', cadencia: 'um quadro por mudança de atributo do documento', motivo: 'recalcula a espessura do traço do medidor; não anima'},
  'meter-icons.js:18': {tipo: 'rAF único por evento', cadencia: 'um quadro por troca do acabamento do medidor', motivo: 'redecora os ícones do medidor; não anima'},
  'player.js:69': {tipo: 'início do laço rAF', cadencia: LACO_MEDIDOR, serie: 'medidores'},
  'player.js:70': {tipo: 'laço rAF', cadencia: 't-lastMeter>=1000/meterFPS; meterFPS=motionReduced()?10:[10,30,60].includes(state.fps)?state.fps:30', serie: 'medidores'},
  'refinement21.js:42': {tipo: 'rAF único coalescido', cadencia: 'um quadro por pedido de preparo', motivo: 'prepara os rótulos do medidor; não anima'},
  'restore.js:133': {tipo: 'rAF único coalescido', cadencia: 'um quadro por pedido de redesenho', motivo: 'redesenha o painel de restauração; processo separado, fora do player'},
  'waveform-styles.js:56': {tipo: 'reinício do laço rAF', cadencia: LACO_ONDA, serie: 'estilos'},
  'waveform-styles.js:89': {tipo: 'laço rAF', cadencia: "t-lastPulse>=(low?100:selected==='spectrum-pulse'?0:33)", serie: 'estilos'},
  'waveform-styles.js:90': {tipo: 'início do laço rAF', cadencia: LACO_ONDA, serie: 'estilos'},
  'waveform-styles.js:93': {tipo: 'reinício do laço rAF', cadencia: LACO_ONDA, serie: 'estilos'},
};

/** Comportamento de cada rAF/WAAPI da varredura; `serie` lista as execuções medidas. */
function descreverComportamentos(estatica, medidores, estilos) {
  const lista = [];
  for (const f of estatica.js) {
    const locais = [...f.requestAnimationFrame.map(x => ['requestAnimationFrame', x]), ...f.animate.map(x => ['animate', x])].sort((a, b) => a[1].linha - b[1].linha || a[1].coluna - b[1].coluna);
    for (const [api, x] of locais) {
      const origem = `${f.arquivo}:${x.linha}`;
      const c = exigir(COMPORTAMENTOS, origem, 'rAF/WAAPI');
      const e = {origem, api, ...c};
      if (c.serie) e.serie = c.serie === 'medidores' ? medidores : estilos;
      lista.push(e);
    }
  }
  return lista;
}

// ---------------------------------------------------------------------------------
// AC-3: estilos de onda e Web Audio.

const MOTIVO_ARQUIVO = 'desenha o envelope do arquivo decodificado (waveform-styles.js:93) e não responde ao áudio ao vivo; sem série';

/** Os 17 estilos de `defaultConfigs`, com parâmetros literais e série ou motivo. */
function descreverEstilos(mvp) {
  const {configs, aoVivo} = configsDoMvp(fs.readFileSync(path.join(mvp, 'waveform-styles.js'), 'utf8'));
  return Object.entries(configs).map(([estilo, parametros]) => {
    const e = {estilo, parametros, aoVivo: aoVivo.includes(estilo)};
    if (e.aoVivo) e.execucao = estilo;
    else {
      e.motivo = MOTIVO_ARQUIVO;
      // `cfg.peakHeight??(selected==='overview'?15:22)` e `bodyHeight` 7/20 (waveform-styles.js:93).
      e.envelope = estilo === 'overview' ? {peakHeight: 15, bodyHeight: 7} : {peakHeight: 22, bodyHeight: 20};
      e.geometria = 'envelope(values,height): abscissa i*1000/(n-1), ordenadas 24 ± valor·altura, toFixed(2) (waveform-styles.js:64)';
    }
    if (estilo === 'spectrum-pulse') e.parametrosSemEfeito = ['peakDecay'];
    if (estilo === 'bars-pulse') {
      e.ramoRadio = {
        parametros: {bars: 128, gain: 1.0, altura: 22},
        formula: 'para i em 0..bars-1: peak = max |amostra| em [floor(i·n/bars), floor((i+1)·n/bars)) (0 sem amostras); h = min(1, peak·gain)·22; M (i·1000/bars) (24-h) v (2h), toFixed(2), concatenado num d gravado em cada filho de #wave-reactive-bars; depois scaleBars como no ramo local (waveform-styles.js:89)',
        motivo: 'só roda com faixa de rádio (getTrackSnapshot().radio); o protocolo usa faixa local, então não há série',
      };
    }
    return e;
  });
}

/** Comportamento reimplementável de cada uso de Web Audio, por `arquivo:linha`. */
const WEB_AUDIO = {
  'advanced-dsp.js:251': 'gera o código do AudioWorkletProcessor do DSP avançado (vynamp-advanced37) a partir das funções puras clamp/db/level e dos coeficientes de true peak',
  'advanced-eq.js:19': 'contexto offline 2 canais, 1 quadro, na taxa da sessão, usado só para calcular respostas de filtro',
  'advanced-eq.js:77': 'um BiquadFilter por banda, em cache, para a curva do EQ avançado',
  'advanced-eq.js:80': 'resposta em frequência da banda; lowpass/highpass repetem a magnitude slope/12 vezes',
  'dsp-ir-assets.js:7': 'decodifica a IR guardada; aceita 1, 2 ou 4 canais e até 10 s',
  'dsp-workspace.js:114': 'decodifica bytes importados num contexto offline de 48 kHz, 2 canais',
  'dynamics-dsp.js:8': 'gera o código do AudioWorkletProcessor de dinâmica (gate/expander) a partir do kernel puro',
  'graph-audio.js:7': 'carrega o módulo do worklet por data URL; falha vira estado de indisponível',
  'graph-audio.js:14': 'prepara os worklets avançado e de dinâmica por contexto; sem audioWorklet registra o motivo e desliga o tipo',
  'graph-audio.js:35': 'ganho linear como bloco de construção (entrada, saída, mistura dry/wet)',
  'graph-audio.js:40': 'mudança de parâmetro com rampa linear a partir do valor atual; imediata em contexto offline',
  'graph-audio.js:42': 'biquad do tipo pedido com parâmetros em k-rate',
  'graph-audio.js:43': 'matriz estéreo 2x2 por splitter, quatro ganhos e merger',
  'graph-audio.js:44': 'medidor por nó: splitter em dois analisadores, um por canal',
  'graph-audio.js:47': 'LFO senoidal com ganho de profundidade como saída de controle',
  'graph-audio.js:49': 'compressor/limitador nativo seguido de ganho de makeup',
  'graph-audio.js:51': 'bandas de EQ em taxas altas rodam no worklet avançado em vez de 64 biquads nativos',
  'graph-audio.js:53': 'EQ paramétrico por canal: pré-ganho, splitter, cadeia de biquads por canal e merger',
  'graph-audio.js:56': 'pan estéreo pela matriz e StereoPanner',
  'graph-audio.js:57': 'fase: atraso independente por canal entre splitter e merger',
  'graph-audio.js:58': 'crossfeed: direto e cruzado por canal com atraso e filtro',
  'graph-audio.js:59': 'eco: dry, wet, atraso de até 2 s e realimentação',
  'graph-audio.js:60': 'reverb FDN de 8 linhas (29,7 a 61,7 ms) com lowpass e realimentação por linha, para taxas acima de 96 kHz',
  'graph-audio.js:61': 'gate/expander no worklet de dinâmica vynamp-dynamics36',
  'graph-audio.js:62': 'tipos do DSP avançado no worklet vynamp-advanced37',
  'graph-audio.js:63': 'chorus/flanger: atraso de até 100 ms modulado por oscilador, com realimentação',
  'graph-audio.js:64': 'convolução: pré-atraso de até 500 ms, convolver com a IR e mistura dry/wet',
  'graph-audio.js:67': 'bass/exciter: filtro lowpass/highpass, drive e waveshaper misturados ao sinal seco',
  'graph-audio.js:69': 'saturação: waveshaper com curva de 2049 pontos, drive e mistura',
  'graph-audio.js:75': 'alinhamento de latência: atraso do déficit de quadros entre ramos que se juntam',
  'graph-audio.js:78': 'medição do nó: pico e soma de quadrados das amostras dos dois canais e espectro em bytes com máximo por bin',
  'graph-audio.js:82': 'motor do grafo: caminho seco com atraso de compensação de até 2 s',
  'graph-audio.js:83': 'rampa linear de parâmetro a partir do valor atual, cancelando o agendado',
  'graph-audio.js:86': 'troca de cadeia com ganho wet começando em 0 e rampa de entrada',
  'graph-properties.js:18': 'osciloscópio do inspetor: amostras do analisador do nó',
  'graph-properties.js:19': 'espectro do inspetor: dB do analisador do nó, smoothing 0 no modo espectro',
  'player.js:61': 'grafo do player: elemento de áudio → ganho → splitter em 2 analisadores ao vivo (fftSize 2048, smoothing .75) e 2 de pulso (smoothing 0)',
  'player.js:70': 'medidor: por canal, RMS de 2048 amostras em dB recortado em [-60,0] e espectro em dB (medido no AC-2)',
  'player.js:73': 'decodifica o arquivo local para o envelope da onda de arquivo',
  'player.js:205': 'contexto offline para a prévia do EQ, um biquad por banda, na taxa da sessão',
  'player.js:210': 'curva do EQ: soma em dB das respostas de cada filtro, magnitude mínima 1e-8',
  'player.js:243': 'copyPulseSpectrum, copyLiveSpectrum e copyLiveSamples: leitura dos analisadores pelos estilos de onda; -Infinity sem áudio (medido no AC-2)',
  'restore-dsp.js:391': 'gera o código do AudioWorkletProcessor de restauração a partir do kernel puro',
  'restore-engine.js:5': 'restauração ao vivo: carrega o worklet vynamp-restore, nó estéreo e ganhos com rampa linear na troca',
  'restore.js:145': 'exportação: renderiza a cadeia DSP num contexto offline 2 canais com frames+latência quadros a partir de um buffer de entrada',
  'restore.js:274': 'decodifica em lote: cria AudioContext (ou webkitAudioContext) para decodeAudioData',
  'restore.js:275': 'decodifica o arquivo aberto no restaurador',
};

/** Cada uso de Web Audio da varredura dos `.js`, com APIs e comportamento. */
function descreverWebAudio(mvp) {
  const fontes = Object.fromEntries(fs.readdirSync(mvp).filter(f => f.endsWith('.js')).map(f => [f, fs.readFileSync(path.join(mvp, f), 'utf8')]));
  return usosWebAudio(fontes).map(u => {
    const origem = `${u.arquivo}:${u.linha}`;
    return {origem, apis: u.apis, comportamento: exigir(WEB_AUDIO, origem, 'Web Audio')};
  });
}

module.exports = {
  lerTransicao, lerAnimacao, lerEase, apisWebAudio, usosWebAudio, configsDoMvp, exigir,
  descreverTransicoes, descreverComportamentos, descreverEstilos, descreverWebAudio,
};
