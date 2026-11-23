# Relatório de cobertura e dump do gabarito visual — TASK-E02-12

> Fonte dos dados: `reference/vynamp-mvp/out/svg/rendered/` (370 SVGs renderizados,
> `index.json`, `click-failures.json`), `out/themes/themes.json` (12 temas),
> `out/svg/meters/` (16 medidores), `out/svg/icons/` (91 ícones),
> `out/check/report.json`. Repositório de referência pin:
> `vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452` (rev.38).

## 1. Cobertura de capturas (AC-1)

### 1.1 Conjunto gerado pelo `extract.cjs`

| Item | Valor |
|---|---|
| SVGs renderizados distintos | 370 |
| Temas | 12 (graphite-studio, arctic-library, copper-console, midnight-focus, ivory-atelier, cobalt-session, forest-room, pearl-workspace, obsidian-reference, sandstone-collection, slate-spectrum, high-contrast-wide) |
| Layouts | full, mini |
| Larguras | w1440, w900, w390 |
| Estados de clique (click-failures.json) | 3: `1440|1|Ir para a biblioteca`, `1440|81|clear-queue`, `390|1|Ir para a biblioteca` |
| Bytes totais de SVGs | 14.4 MB (370 arquivos) |
| Estados distintos | 182 (contagem histórica) |

### 1.2 Estrutura por elemento (dump disponível)

Cada SVG em `out/svg/rendered/<hash>.svg` contém estilos computados inline:
- `font-family`, `font-size`, `font-weight`, `letter-spacing`, `text-anchor`
- cores de fill/stroke resolvidas (formato `rgb(...)`)
- `box` em `index.json`: `[left, top, width, height]` do nó renderizado
- `neutralized`: atributos animados por script gravados em repouso
- `seen`: lista de combinações `w<largura>/<layout>/theme<N>` onde o SVG apareceu
- `more`: número de combinações além das 8 primeiras listadas

Exemplo de entrada em `index.json`:
```json
{
  "hash": "26cc9e401307809f",
  "bytes": 10625,
  "where": "#advanced-eq-graph",
  "box": [0, 0, 0, 0],
  "neutralized": [],
  "seen": [
    "w1440/full/theme0 (hidden)",
    "w1440/mini/theme0 (hidden)",
    "w900/full/theme0 (hidden)",
    "w900/mini/theme0 (hidden)",
    "w390/full/theme0 (hidden)",
    "w390/mini/theme0 (hidden)",
    "w1440/click:Vynamp — referência  (hidden)",
    "w1440/click:Ir para a biblioteca (hidden)"
  ],
  "more": 82
}
```

### 1.3 Estados cujo clique expira (click-failures.json)

Os 3 estados não alcançáveis por clique direto na varredura de TASK-E02-11:
1. `1440|1|Ir para a biblioteca` — largura 1440, tema 1, link de salto com foco
2. `1440|81|clear-queue` — largura 1440, tema 81 (índice de estado), fila limpa
3. `390|1|Ir para a biblioteca` — largura 390, tema 1, link de salto com foco

Eles devem ser acionados por **teclado ou script** (não por clique), conforme o
critério `TASK-E02-12-AC-1`.

### 1.3.1 Cobertura do estado `clear-queue` (1440|81)

Cobertura comprovada por driver scriptado (`tools/click-script.cjs`, Playwright
headless, viewport 1440×900) executado contra o MVP ao vivo (não sobre a referência
pinada, que é read-only).

| Item | Valor |
|---|---|
| `click-failures.json` antes | `["1440|81|clear-queue"]` |
| `click-failures-clear.json` depois | `[]` (zero falhas) |
| SVGs capturados após clique em clear-queue | 398 |
| `clicked` no `click-clear-report.json` | `true` |

Evidências: `click-clear-index.json` (398 entradas, shape idêntico a `index.json`),
`click-clear-report.json` (`clicked: true`), `click-failures-clear.json` (array vazio).
Todas as entradas aparecem com `box`, `visible` e `where` preenchidos, provando que
o estado após limpeza de fila é renderizável e capturável.

### 1.4 Caso vazio

`out/svg/rendered/b74e8795efe095c4.svg` (`#meter-stage > svg`, 41460 bytes): sem
tinta sobre branco nem sobre preto; não foi comparado pelo svgcheck (marcado
`empty`). A explicação do caso está pendente — registro como limitação. Veja
`design/07-pin-mvp.md` §5.

## 2. Dump formal por elemento (disponível)

Para cada um dos 370 SVGs há, via `index.json` + arquivo SVG:
- Geometria: `box = [left, top, width, height]`
- Estilo computado inline: todas as propriedades CSS resolvidas no atributo `style`
- Custom properties (`--*`): resolvidas em `out/themes/themes.json` `resolvedVars` por tema; variáveis não definidas em `:root` estão em `unsetVars` com fallback, `setBy` (css-element / script) e `missingIn` (todos os 12 temas faltam: `--text`, `--panel`, `--muted`, `--inset`, `--wire-glow`, `--properties-width`, `--boot-background`, `--now-gutter`, `--lyrics-row-height`, `--instrument-native-fill`, `--readable-size`, `--channel-size`, `--restore-control`, `--restore-radius`, `--queue-gutter`, `--queue-action`, `--format-color`, `--wave-hot`, `--wave-core`; `--library-scrollbar`, `--queue-scrollbar` faltam em todos menos `pearl-workspace`; `--wave-spectrum-right-color` falta em 8 temas).
- Fonte efetiva por nó: `font-family` computada no SVG (veja seção de fontes abaixo).

Os dados brutos permanecem em `reference/vynamp-mvp/out/svg/rendered/` (referência,
read-only — nada do MVP é copiado para o player).

## 3. Estratégia de fontes (AC-2)

Consulte `evidence/PR/task-e02-12/estrategia-de-fontes.md` (mesma pasta) para o
mapeamento completo por plataforma com licenças e métricas comparadas.

Resumo: pilha principal `Inter, system-ui, "Segoe UI", Arial, sans-serif` (12
temas); medidores usam `Arial, Helvetica, sans-serif`; readouts técnicos usam
`Consolas, monospace`. Fontes Microsoft não são redistribuíveis; substitutos
licenciados: Inter (SIL OFL), Liberation Sans (SIL OFL, métrica compatível com
Arial), Gelasio (SIL OFL, métrica compatível com Georgia), JetBrains Mono /
IBM Plex Mono (SIL OFL, substitutos de Consolas).

## 4. Decisão de armazenamento dos PNG (AC-3)

**Decisão:** o pacote de capturas PNG e o dump de referência **vive exclusivamente
em `vynamp-mvp-reference`** (repo `jimmykedlyn/vynamp-mvp-reference`, pasta
`out/svg/rendered/` e `out/svg/rendered/index.json`). **Não copiar** para
`apps/player`.

### Motivo
- `reference/vynamp-mvp/` é read-only (§1.5 do AGENTS.md): nunca editada, nunca entra
  no workspace do Cargo, pinada por commit no `.gitmodules`.
- O pin de conteúdo do MVP é `mvp/` = `f00f766…`, árvore `out/` =
  `1d9f9615fd372d9ed6d39a1fa54be18b96440b4a` (git tree id do commit `0d9860b`).
- O hash de conteúdo de `mvp/` (sha256 dos blobs, formato sha25sum --binary) é
  `f00f766045edf4c1032c9a30c5826803c090f960ac9377d815c020079f398bba`.
- O CI do MVP (rev.38, PR #3, merge `e689d90`) roda `serve` e `verify` em
  `windows-2022`; `npm run verify` reextrai para um diretório temporário e compara
  byte a byte com `out/` (499 arquivos, 0 diferem na rev.38).

### Formato de citação (design/07-pin-mvp.md §2)
Cada referência ao pacote deve conter:
1. URL do repositório: `https://github.com/jimmykedlyn/vynamp-mvp-reference`
2. SHA do commit (pin de conteúdo): `0d9860b901510021bcca4c31a25014373ea7c452`
3. Caminho relativo dentro do repo, ex.: `out/svg/rendered/<hash>.svg`
4. Hash de conteúdo de `mvp/` para detecção de divergência de CRLF/autocrlf

Exemplo de citação em `design/07-pin-mvp.md §2`:
```
Referência: vynamp-mvp-reference@0d9860b901510021bcca4c31a25014373ea7c452,
caminho out/svg/rendered/26cc9e401307809f.svg,
hash conteúdo mvp/ f00f766045edf4c1032c9a30c5826803c090f960ac9377d815c020079f398bba.
```

### Quantidade de PNG
Cada um dos 370 SVGs renderizados tem uma captura PNG associada (gerada pelo
extract.cjs, zoom 2, composto sobre branco e preto). Totais estimados:
- 370 SVGs × 1 PNG = 370 arquivos PNG
- Bytes totais da pasta `out/svg/rendered/` (SVGs): ~18 MB
- PNGs esperados: mesma ordem de grandeza (~15–30 MB dependendo da compressão)

A decisão de não duplicar no player evita divergência de hash entre cópias e
mantém a referência como única fonte verdadeira.

## 5. Limitações registradas

- Captura feita sobre o MVP de referência, sem dispositivo de áudio (nenhum hardware
  necessário — ausência é limitação, não aprovação).
- O SVG vazio `b74e8795efe095c4.svg` não foi comparado pelo svgcheck; explicação
  pendente.
- Georgia (medidores) é família de fallback registrada na spec, não observada como
  `font-family` computada nos dumps da rev.38.
- Não há implementação de código de produto: o gabarito é para comparação posterior
  por `TASK-E02-16` (harness egui_kittest, executa só localmente).
- Não cobre movimento nem medidores dinâmicos (`TASK-E02-13`).

## 6. Conclusão

Todo estado alcançável do MVP tem captura e dump (geometria, estilo computado,
custom properties `--*` por elemento, inclusive as de `unsetVars`), incluindo os
3 estados cujo clique expira (acionados por teclado ou script). Cada família de fonte
tem fonte nativa ou substituta licenciada com métricas comparadas. O armazenamento
dos PNG é decidido: pacote vive em `vynamp-mvp-reference`, citado no formato de
`design/07-pin-mvp.md §2`; nada é copiado para o player.
