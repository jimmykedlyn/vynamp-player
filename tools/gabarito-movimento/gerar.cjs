// Gerador do gabarito de movimento (TASK-E02-13): mede medidores, estilos de onda e animações
// do MVP fixado, pelo "Protocolo de medição" de tasks/items/TASK-E02-13.md.
//
// Uso: node tools/gabarito-movimento/gerar.cjs <dir-do-mvp> [<raiz-de-saída>]
//   <dir-do-mvp>       diretório com o index.html do MVP, dentro de um clone git em
//                      0d9860b901510021bcca4c31a25014373ea7c452 (recusa outro commit).
//   <raiz-de-saída>    onde gravar assets/gabarito/movimento.json; padrão: a raiz deste repo.
'use strict';
const {chromium} = require('playwright');
const {execFileSync} = require('child_process');
const fs = require('fs');
const http = require('http');
const os = require('os');
const path = require('path');
const g = require('./grade.cjs');

/** Estilos ao vivo (`liveModes` de waveform-styles.js) e as séries do item 6 de "Estilos de onda". */
// Cada série: [atributo, id do elemento, canal]. Canal L = analisador do canal 0, R = do canal 1,
// LR = os dois (o estilo funde ou compara os canais). `grupo` é o analisador lido: `live`
// (copyLiveSamples/copyLiveSpectrum) ou `pulse` (copyPulseSpectrum).
const ESTILOS = {
  'pulse': {grupo: 'live', series: [['d', 'wave-pulse', 'L']]},
  'bars-pulse': {grupo: 'live', series: [['data-level', 'wave-reactive-bars', 'L'], ['transform', 'wave-reactive-bars', 'L']]},
  'columns-pulse': {grupo: 'live', series: [['data-peak', 'wave-live-styles', 'L'], ['d', 'wave-live-columns', 'L']]},
  'ribbon-pulse': {grupo: 'live', series: [['data-peak', 'wave-live-styles', 'L'], ['d', 'wave-live-fill', 'L']]},
  'prisma-neon': {grupo: 'live', series: [['data-peak', 'wave-live-styles', 'L'], ['d', 'wave-live-columns', 'L']]},
  'aurora-neon': {grupo: 'live', series: [['data-peak', 'wave-live-styles', 'L'], ['d', 'wave-live-line', 'L'], ['d', 'wave-live-halo', 'L']]},
  'spectrum-aurora': {grupo: 'live', series: [['data-level', 'wave-spectrum-live', 'L'], ['data-frequency-left', 'wave-spectrum-live', 'L'], ['d', 'wave-spectrum-left', 'L'], ['d', 'wave-spectrum-halo', 'L']]},
  'spectrum-ribbon': {grupo: 'live', series: [['data-level', 'wave-spectrum-live', 'L'], ['data-frequency-left', 'wave-spectrum-live', 'L'], ['d', 'wave-spectrum-fill', 'L']]},
  'spectrum-stereo': {grupo: 'live', series: [['data-level', 'wave-spectrum-live', 'LR'], ['data-frequency-left', 'wave-spectrum-live', 'L'], ['data-frequency-right', 'wave-spectrum-live', 'R'], ['d', 'wave-spectrum-left', 'L'], ['d', 'wave-spectrum-right', 'R']]},
  'spectrum-pulse': {grupo: 'pulse', series: [['data-level', 'wave-spectrum-live', 'LR'], ['d', 'wave-spectrum-fill', 'LR'], ['d', 'wave-spectrum-halo', 'LR']]},
  'aurora-weave': {grupo: 'live', series: [['data-level', 'wave-creative-live', 'LR'], ['d', 'wave-creative-left', 'L'], ['d', 'wave-creative-right', 'R']]},
  'neon-crystal': {grupo: 'live', series: [['data-level', 'wave-creative-live', 'L'], ['d', 'wave-creative-outer', 'L'], ['d', 'wave-creative-middle', 'L'], ['d', 'wave-creative-inner', 'L']]},
  'aurora-constellation': {grupo: 'live', series: [['data-level', 'wave-creative-live', 'L'], ['data-frequency-left', 'wave-spectrum-live', 'L'], ['d', 'wave-creative-nodes', 'L'], ['d', 'wave-creative-mesh', 'L']]},
  'aurora-layers': {grupo: 'live', series: [['data-level', 'wave-creative-live', 'L'], ['d', 'wave-creative-outer', 'L'], ['d', 'wave-creative-middle', 'L'], ['d', 'wave-creative-inner', 'L']]},
};
/** Medidores do AC-2 e as séries do item 7 do protocolo. */
const MEDIDORES = {'06-hifi-spectrum': ['nivel', 'espectro'], '12-osciloscopio-crt': ['nivel', 'onda']};

const falha = msg => { throw new Error(msg); };

/** `git` no clone do MVP, sem herdar `GIT_*` do ambiente (hooks exportam GIT_DIR). */
function git(dir, args) {
  const env = Object.fromEntries(Object.entries(process.env).filter(([k]) => !k.startsWith('GIT_')));
  return execFileSync('git', ['-C', dir, ...args], {env, encoding: 'utf8'}).trim();
}

/** Recusa rodar se o clone não estiver exatamente no commit fixado (item 10). */
function conferePin(mvp) {
  const head = git(mvp, ['rev-parse', 'HEAD']);
  if (head !== g.MVP_COMMIT) falha(`MVP em ${head}, esperado ${g.MVP_COMMIT}: o gerador recusa outro commit.`);
  const sujo = git(mvp, ['status', '--porcelain', '--untracked-files=no', '--', '.']);
  if (sujo) falha(`MVP com arquivos rastreados alterados, não está no commit ${g.MVP_COMMIT}:\n${sujo}`);
  return head;
}

/** Servidor estático só em loopback, sem cache, preso ao diretório do MVP. */
function servir(dir) {
  const raiz = path.resolve(dir);
  const tipos = {'.html': 'text/html; charset=utf-8', '.js': 'text/javascript', '.css': 'text/css', '.svg': 'image/svg+xml', '.json': 'application/json', '.woff2': 'font/woff2'};
  const srv = http.createServer((req, res) => {
    let f;
    try { f = path.join(raiz, decodeURIComponent(new URL(req.url, 'http://x').pathname)); } catch { res.writeHead(404); return res.end(); }
    if (!f.startsWith(raiz + path.sep) || !fs.existsSync(f) || !fs.statSync(f).isFile()) { res.writeHead(404); return res.end(); }
    res.writeHead(200, {'Content-Type': tipos[path.extname(f)] || 'application/octet-stream', 'Cache-Control': 'no-store'});
    fs.createReadStream(f).pipe(res);
  });
  return new Promise(ok => srv.listen(0, '127.0.0.1', () => ok(srv)));
}

/**
 * Roda na página antes de qualquer script dela (item 2): troca AudioContext por um
 * OfflineAudioContext de 48 kHz e 96000 amostras, fornece createMediaElementSource com o sinal
 * sintético em estéreo idêntico, marca os AnalyserNode criados nele e registra cada leitura, e
 * substitui requestAnimationFrame e performance.now por um relógio virtual com ids a partir de 1.
 */
function stub({sinalB64, taxa, amostras}) {
  const bytes = Uint8Array.from(atob(sinalB64), c => c.charCodeAt(0));
  const sinal = new Float32Array(bytes.buffer);
  const G = window.__gm = {ctx: null, criados: 0, analisadores: [], leituras: [], quadro: null, erros: [], pend: new Map(), proximoId: 1, virtual: false, agora: 0};
  try { localStorage.clear(); } catch { G.erros.push('localStorage indisponível'); }
  const Offline = window.OfflineAudioContext;
  window.AudioContext = function AudioContext() {
    G.criados++;
    if (G.ctx) throw new Error('segundo AudioContext: o protocolo prevê um só contexto por execução');
    const ctx = new Offline({numberOfChannels: 2, length: amostras, sampleRate: taxa});
    const criaAnalisador = ctx.createAnalyser.bind(ctx);
    ctx.createAnalyser = () => { const a = criaAnalisador(); a.__gmOrdem = G.analisadores.length; G.analisadores.push(a); return a; };
    ctx.createMediaElementSource = () => {
      const buf = ctx.createBuffer(2, amostras, taxa);
      buf.copyToChannel(sinal, 0);
      buf.copyToChannel(sinal, 1);
      const fonte = ctx.createBufferSource();
      fonte.buffer = buf;
      fonte.start(0);
      return fonte;
    };
    G.ctx = ctx;
    return ctx;
  };
  for (const metodo of ['getFloatTimeDomainData', 'getFloatFrequencyData', 'getByteTimeDomainData', 'getByteFrequencyData']) {
    const original = AnalyserNode.prototype[metodo];
    AnalyserNode.prototype[metodo] = function (destino) {
      const r = original.call(this, destino);
      if (this.__gmOrdem !== undefined) G.leituras.push({quadro: G.quadro, ordem: this.__gmOrdem, metodo, fftSize: this.fftSize, smoothingTimeConstant: this.smoothingTimeConstant, dados: Float32Array.from(destino)});
      return r;
    };
  }
  const rafNativo = window.requestAnimationFrame.bind(window);
  const agoraNativo = performance.now.bind(performance);
  window.requestAnimationFrame = cb => { const id = G.proximoId++; G.pend.set(id, cb); return id; };
  window.cancelAnimationFrame = id => { G.pend.delete(id); };
  Object.defineProperty(performance, 'now', {configurable: true, value: () => (G.virtual ? G.agora : agoraNativo())});
  // Chama os callbacks pendentes no início do quadro; os agendados durante a chamada ficam para o próximo.
  G.flush = t => {
    for (const id of [...G.pend.keys()]) {
      const cb = G.pend.get(id);
      if (!cb) continue;
      G.pend.delete(id);
      try { cb(t); } catch (e) { G.erros.push(String((e && e.stack) || e)); }
    }
  };
  // Antes da medição, o carregamento da página anda em tempo real; para quando o relógio vira virtual.
  const bomba = () => { if (G.virtual) return; G.flush(agoraNativo()); rafNativo(bomba); };
  rafNativo(bomba);
}

/** Prepara o estado do item 1 e toca pelo caminho da spec até `playTrack`. */
async function preparar({alvo, medidor, wavB64}) {
  const R = window.VynampReference, $ = id => document.getElementById(id), G = window.__gm;
  const exige = (cond, msg) => { if (!cond) throw new Error('estado do protocolo: ' + msg); };
  exige(document.visibilityState === 'visible', 'aba visível');
  exige(!matchMedia('(prefers-reduced-motion: reduce)').matches, 'prefers-reduced-motion: no-preference');
  exige(R.state.reducedMotion === false && R.state.fps === 30 && R.state.muted === false && R.state.eqEnabled === false, 'reducedMotion, fps 30, sem mudo, EQ desligado');
  exige(window.VynampWaveformStyles.snapshot().mode === 'original', 'nenhum estilo ao vivo selecionado');
  exige(G.ctx === null, 'nenhum AudioContext antes do stub');
  if (medidor !== null) {
    document.querySelector(`#meter-menu [data-meter="${medidor}"]`).click();
    exige(R.state.meter === medidor && R.state.meterOff === false, 'medidor ' + alvo + ' ligado');
  } else {
    if (!R.state.meterOff) $('meter-off').click();
    exige(R.state.meterOff === true, 'medidor desligado');
  }
  $('volume').value = '100';
  $('volume').dispatchEvent(new Event('input', {bubbles: true}));
  exige(R.state.volume === 1, 'volume 1,0');
  const wav = Uint8Array.from(atob(wavB64), c => c.charCodeAt(0));
  const id = R.registerLocalFile(new File([wav], 'sinal.wav', {type: 'audio/wav'}));
  exige(R.getTracksSnapshot().find(t => t.id === id).demo === false, 'faixa local com demo: false');
  await R.playTrack(id, false);
}

/** Roda a grade do item 5 sobre o contexto offline e devolve as leituras por quadro. */
async function medir({estilo, quadros, taxa, frequenciasHz}) {
  const R = window.VynampReference, G = window.__gm;
  const fora = G.leituras.length;
  G.virtual = true;
  G.agora = 0;
  const host = R.getAudioGraphHost();
  if (host.context !== G.ctx) throw new Error('o grafo não está sobre o contexto do stub');
  if (estilo) {
    document.querySelector(`[data-wave-choice="${estilo}"]`).click();
    if (window.VynampWaveformStyles.snapshot().mode !== estilo) throw new Error('seleção do estilo ' + estilo + ' falhou');
  }
  R.setProceduralTransport(true, 'playing');
  const ctx = G.ctx, saida = [];
  const pendentes = quadros.map(q => ctx.suspend(q.amostra / taxa).then(() => {
    G.agora = q.t;
    G.quadro = q.k;
    const amostra = Math.round(ctx.currentTime * taxa);
    G.flush(q.t);
    G.quadro = null;
    const todas = G.leituras.splice(0);
    const lidas = todas.filter(l => l.quadro === q.k);
    if (todas.length !== lidas.length) G.erros.push(`quadro ${q.k}: ${todas.length - lidas.length} leituras de analisador fora do quadro`);
    if (amostra !== q.amostra) G.erros.push(`quadro ${q.k}: amostra real ${amostra}, fórmula ${q.amostra}`);
    if (lidas.length) saida.push(estilo ? leituraEstilo(q, amostra, lidas) : leituraMedidor(q, amostra, lidas));
    return ctx.resume();
  }));
  function base(q, amostra, lidas) {
    return {k: q.k, t: q.t, amostra, analisadores: lidas.map(l => ({ordem: l.ordem, metodo: l.metodo, fftSize: l.fftSize, smoothingTimeConstant: l.smoothingTimeConstant}))};
  }
  // Pós-processamento literal do `frame` e do `paintMeter` (player.js:59 e :43 a :55).
  function leituraMedidor(q, amostra, lidas) {
    const r = base(q, amostra, lidas);
    r.nivel = [];
    r.espectro = [];
    r.onda = [];
    for (const ch of [0, 1]) {
      const td = lidas.find(l => l.ordem === ch && l.metodo === 'getFloatTimeDomainData');
      const fd = lidas.find(l => l.ordem === ch && l.metodo === 'getFloatFrequencyData');
      if (!td || !fd) throw new Error(`quadro ${q.k}: o frame não leu o canal ${ch}`);
      const rms = Math.sqrt(td.dados.reduce((s, v) => s + v * v, 0) / td.dados.length);
      r.nivel.push(Math.max(-60, Math.min(0, 20 * Math.log10(Math.max(1e-8, rms)))));
      const bins = fd.dados;
      r.espectro.push(frequenciasHz.map(hz => {
        const bin = Math.min(bins.length - 1, Math.max(1, Math.round(hz / (taxa / 2) * (bins.length - 1))));
        return Math.max(-60, Math.min(0, bins[bin]));
      }));
      r.onda.push(Array.from({length: 181}, (_, i) => td.dados[i * 4]));
    }
    return r;
  }
  function leituraEstilo(q, amostra, lidas) {
    const r = base(q, amostra, lidas);
    r.atributos = {};
    for (const el of ['wave-pulse', 'wave-reactive-bars', 'wave-live-styles', 'wave-live-columns', 'wave-live-fill', 'wave-live-line', 'wave-live-halo', 'wave-spectrum-live', 'wave-spectrum-left', 'wave-spectrum-right', 'wave-spectrum-fill', 'wave-spectrum-halo', 'wave-creative-live', 'wave-creative-left', 'wave-creative-right', 'wave-creative-outer', 'wave-creative-middle', 'wave-creative-inner', 'wave-creative-nodes', 'wave-creative-mesh']) {
      const n = document.getElementById(el);
      for (const a of ['d', 'transform', 'data-level', 'data-peak', 'data-frequency-left', 'data-frequency-right']) if (n && n.hasAttribute(a)) r.atributos[a + '@' + el] = n.getAttribute(a);
    }
    return r;
  }
  await ctx.startRendering();
  await Promise.all(pendentes);
  return {saida, leiturasAntes: fora, leiturasDepois: G.leituras.length, erros: G.erros, criados: G.criados, ordens: G.analisadores.map(a => ({fftSize: a.fftSize, smoothingTimeConstant: a.smoothingTimeConstant}))};
}

/** Monta as séries `<série>/<canal>` de uma execução a partir das leituras. */
function series(alvo, tipo, m) {
  const ordemDe = (grupo, canal) => (grupo === 'pulse' ? 2 : 0) + (canal === 'R' ? 1 : 0);
  const analisador = (grupo, canal) => (canal === 'LR' ? ['L', 'R'] : [canal]).map(c => {
    const o = m.ordens[ordemDe(grupo, c)];
    return {canal: c, fftSize: o.fftSize, smoothingTimeConstant: o.smoothingTimeConstant};
  });
  const out = {};
  if (tipo === 'medidor') {
    for (const s of MEDIDORES[alvo]) for (const [ch, canal] of [[0, 'L'], [1, 'R']]) {
      out[`${s}/${canal}`] = {analisador: analisador('live', canal), valores: m.saida.map(l => l[s][ch])};
    }
  } else {
    const def = ESTILOS[alvo];
    for (const [attr, el, canal] of def.series) {
      const chave = `${attr}@${el}`;
      out[`${chave}/${canal}`] = {analisador: analisador(def.grupo, canal), valores: m.saida.map(l => {
        if (!(chave in l.atributos)) falha(`${alvo}: quadro ${l.k} sem ${chave}`);
        return l.atributos[chave];
      })};
    }
  }
  return out;
}

/** Uma execução do item 4: página e contexto novos, um sinal, um medidor ou estilo. */
async function executar(browser, base, {alvo, tipo, sinal, medidor, frequenciasHz}) {
  const amostras = g.amostrasSinal(sinal);
  const ctxNav = await browser.newContext({viewport: {width: 1440, height: 900}, reducedMotion: 'no-preference'});
  try {
    const page = await ctxNav.newPage();
    const erros = [];
    page.on('pageerror', e => erros.push(String(e)));
    page.on('dialog', d => d.dismiss().catch(() => {}));
    await page.addInitScript(stub, {sinalB64: Buffer.from(amostras.buffer).toString('base64'), taxa: g.TAXA, amostras: g.AMOSTRAS});
    await page.goto(base + 'index.html');
    await page.waitForFunction(() => window.VynampReference && window.VynampWaveformStyles && document.readyState === 'complete', null, {timeout: 30000});
    await page.evaluate(preparar, {alvo, medidor, wavB64: g.wavPcm16Estereo(amostras).toString('base64')});
    const fonte = await page.waitForFunction(() => {
      const s = document.getElementById('wave').dataset.source;
      return ['decoded', 'unavailable'].includes(s) && s;
    }, null, {timeout: 30000}).then(h => h.jsonValue());
    if (fonte !== 'decoded') falha(`${alvo}:${sinal}: o decode do WAV falhou (#wave data-source="${fonte}"); nenhuma série gravada.`);
    const m = await page.evaluate(medir, {estilo: tipo === 'estilo' ? alvo : null, quadros: g.quadros(), taxa: g.TAXA, frequenciasHz});
    const problemas = [...erros, ...m.erros];
    if (m.criados !== 1) problemas.push(`AudioContext criado ${m.criados} vezes`);
    if (m.leiturasAntes || m.leiturasDepois) problemas.push(`leituras de analisador fora da grade: ${m.leiturasAntes} antes, ${m.leiturasDepois} depois`);
    if (m.ordens.length !== 4) problemas.push(`esperados 4 AnalyserNode do setupAudio, criados ${m.ordens.length}`);
    if (problemas.length) falha(`${alvo}:${sinal}:\n` + problemas.join('\n'));
    return {
      execucao: `${alvo}:${sinal}`, alvo, tipo, sinal,
      leituras: m.saida.map(l => ({k: l.k, t: l.t, amostra: l.amostra, analisadores: l.analisadores})),
      series: series(alvo, tipo, m),
    };
  } finally {
    await ctxNav.close();
  }
}

/** AC-1, na página: regras de transição e animação do CSSOM, @keyframes e `getAnimations()`. */
async function animacoesNaPagina() {
  const condicao = r => (r.conditionText !== undefined ? r.constructor.name + ' ' + r.conditionText : r.constructor.name);
  const regras = [], keyframes = [];
  const anda = (lista, folha, cond) => {
    for (const r of lista) {
      if (r instanceof CSSKeyframesRule) {
        keyframes.push({folha, condicao: cond, nome: r.name, quadros: [...r.cssRules].map(k => ({chave: k.keyText, estilo: k.style.cssText}))});
        continue;
      }
      if (r instanceof CSSStyleRule) {
        const s = r.style, v = p => s.getPropertyValue(p);
        const t = {propriedade: v('transition-property'), duracao: v('transition-duration'), easing: v('transition-timing-function'), atraso: v('transition-delay')};
        const a = {nome: v('animation-name'), duracao: v('animation-duration'), easing: v('animation-timing-function'), atraso: v('animation-delay'), iteracoes: v('animation-iteration-count'), direcao: v('animation-direction'), preenchimento: v('animation-fill-mode')};
        const tem = o => Object.values(o).some(x => x !== '');
        if (tem(t) || tem(a)) regras.push({folha, condicao: cond, seletor: r.selectorText, transicao: tem(t) ? t : null, animacao: tem(a) ? a : null});
      }
      if (r.cssRules) anda(r.cssRules, folha, r instanceof CSSStyleRule ? cond : [...cond, condicao(r)]);
    }
  };
  for (const sh of document.styleSheets) {
    const folha = sh.href ? new URL(sh.href).pathname.replace(/^\//, '') : 'inline';
    anda(sh.cssRules, folha, []);
  }
  // Espera as animações finitas terminarem, para o retrato não depender do instante.
  const finitas = document.getAnimations().filter(a => a.effect && a.effect.getComputedTiming().endTime !== Infinity);
  await Promise.race([Promise.all(finitas.map(a => a.finished.catch(() => null))), new Promise(r => setTimeout(r, 10000))]);
  const alvo = el => {
    if (!el) return null;
    const partes = [];
    for (let n = el; n && n !== document.documentElement && partes.length < 6; n = n.parentElement) {
      if (n.id) { partes.unshift('#' + n.id); break; }
      partes.unshift(n.tagName.toLowerCase() + (n.classList[0] ? '.' + n.classList[0] : ''));
    }
    return partes.join(' > ');
  };
  const num = x => (Number.isFinite(x) ? x : String(x));
  const vivas = document.getAnimations().map(a => {
    const tm = a.effect.getTiming();
    return {
      tipo: a.constructor.name, nome: a.animationName ?? a.transitionProperty ?? null,
      alvo: alvo(a.effect.target), pseudo: a.effect.pseudoElement ?? null,
      duracao: num(tm.duration), easing: tm.easing, atraso: tm.delay, iteracoes: num(tm.iterations), direcao: tm.direction, preenchimento: tm.fill,
      keyframes: a.effect.getKeyframes().map(k => Object.fromEntries(Object.entries(k).map(([c, v]) => [c, typeof v === 'number' ? num(v) : v]))),
    };
  });
  return {regras, keyframes, getAnimations: vivas};
}

/** AC-1, estático: @keyframes, transition/animation nas folhas e rAF/WAAPI nos scripts. */
function varreduraEstatica(mvp) {
  const linhaCol = (s, i) => { const antes = s.slice(0, i).split('\n'); return [antes.length, antes.at(-1).length + 1]; };
  const trecho = (s, i) => s.slice(Math.max(0, i - 80), i + 120).replace(/\s+/g, ' ');
  const achados = (s, re) => [...s.matchAll(re)].map(m => { const [linha, coluna] = linhaCol(s, m.index); return {linha, coluna, texto: m[0], trecho: trecho(s, m.index)}; });
  const arquivos = fs.readdirSync(mvp).sort();
  const css = arquivos.filter(f => f.endsWith('.css')).map(f => {
    const s = fs.readFileSync(path.join(mvp, f), 'utf8');
    return {arquivo: f, keyframes: achados(s, /@keyframes\s+[\w-]+/g), declaracoes: achados(s, /(?<![\w-])(transition|animation)(-[a-z-]+)?\s*:[^;}]*/g)};
  }).filter(x => x.keyframes.length || x.declaracoes.length);
  const js = arquivos.filter(f => f.endsWith('.js') || f.endsWith('.html')).map(f => {
    const s = fs.readFileSync(path.join(mvp, f), 'utf8');
    return {arquivo: f, requestAnimationFrame: achados(s, /requestAnimationFrame\s*\(/g), animate: achados(s, /\.animate\s*\(/g)};
  }).filter(x => x.requestAnimationFrame.length || x.animate.length);
  return {css, js};
}

async function animacoes(browser, base, mvp) {
  const ctxNav = await browser.newContext({viewport: {width: 1440, height: 900}, reducedMotion: 'no-preference'});
  try {
    const page = await ctxNav.newPage();
    await page.addInitScript(() => { try { localStorage.clear(); } catch {} });
    await page.goto(base + 'index.html');
    await page.waitForFunction(() => window.VynampReference && document.readyState === 'complete' && document.documentElement.dataset.ready === 'true', null, {timeout: 30000});
    await page.evaluate(() => document.fonts.ready);
    return {...await page.evaluate(animacoesNaPagina), estatica: varreduraEstatica(mvp)};
  } finally {
    await ctxNav.close();
  }
}

/** Lê os medidores do MVP (assets.js) para achar o índice e as frequências de cada um. */
function medidoresDoMvp(mvp) {
  const vm = require('vm'), w = {};
  vm.runInNewContext(fs.readFileSync(path.join(mvp, 'assets.js'), 'utf8'), {window: w});
  return Object.fromEntries(Object.keys(MEDIDORES).map(alvo => {
    const i = w.VYNAMP_ASSETS.meters.findIndex(m => m.file === alvo + '.svg');
    if (i < 0) falha(`medidor ${alvo} ausente de assets.js`);
    return [alvo, {indice: i, frequenciasHz: w.VYNAMP_ASSETS.meters[i].frequenciesHz || []}];
  }));
}

async function main() {
  const [mvpArg, saidaArg] = process.argv.slice(2);
  if (!mvpArg) falha('uso: node gerar.cjs <dir-do-mvp> [<raiz-de-saída>]');
  const mvp = path.resolve(mvpArg);
  const raiz = path.resolve(saidaArg || path.join(__dirname, '..', '..'));
  const commit = conferePin(mvp);
  const medidores = medidoresDoMvp(mvp);
  const srv = await servir(mvp);
  const base = `http://127.0.0.1:${srv.address().port}/`;
  const browser = await chromium.launch({headless: true});
  try {
    const ambiente = {
      chrome: browser.version(),
      playwright: require('playwright/package.json').version,
      node: process.version,
      so: {tipo: os.type(), versao: os.version(), release: os.release(), arquitetura: os.arch()},
    };
    const execucoes = [];
    const plano = [
      ...Object.keys(MEDIDORES).flatMap(alvo => g.SINAIS.map(sinal => ({alvo, tipo: 'medidor', sinal, medidor: medidores[alvo].indice, frequenciasHz: medidores[alvo].frequenciasHz}))),
      ...Object.keys(ESTILOS).flatMap(alvo => g.SINAIS.map(sinal => ({alvo, tipo: 'estilo', sinal, medidor: null, frequenciasHz: []}))),
    ];
    for (const p of plano) {
      const e = await executar(browser, base, p);
      console.log(`${e.execucao}: ${e.leituras.length} leituras, ${Object.keys(e.series).length} séries`);
      execucoes.push(e);
    }
    const gabarito = {
      tarefa: 'TASK-E02-13',
      mvp: {commit},
      ambiente,
      protocolo: {taxa: g.TAXA, amostras: g.AMOSTRAS, quadros: g.QUADROS, quantum: g.QUANTUM, sinais: g.SINAIS},
      balistica: {valor: null, motivo: 'paintMeter mapeia o dB direto; a única suavização é o smoothingTimeConstant do analisador'},
      animacoes: await animacoes(browser, base, mvp),
      execucoes,
    };
    const destino = path.join(raiz, 'assets', 'gabarito', 'movimento.json');
    fs.mkdirSync(path.dirname(destino), {recursive: true});
    fs.writeFileSync(destino, JSON.stringify(gabarito) + '\n');
    console.log(`MVP ${commit}; Chrome ${ambiente.chrome}; Node ${ambiente.node}; ${ambiente.so.versao}`);
    console.log(`gravado ${destino}: ${execucoes.length} execuções`);
  } finally {
    await browser.close();
    srv.close();
  }
}

main().catch(e => { console.error(e.message || e); process.exit(1); });
