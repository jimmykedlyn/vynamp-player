# Estratégia de fontes por plataforma — TASK-E02-12

> **Status:** documento de trabalho preparado para revisão. A escolha final de fonte é
> do mantenedor, conforme `product/05-decisoes.md` (decisões pendentes com dono e
> prazo). Este documento não decide por conta própria; ele confronta as famílias
> observadas no MVP rev.38 com as fontes disponíveis em cada plataforma, com licença
> e métricas comparadas.

## 1. Fontes efetivas observadas no MVP (rev.38)

Fonte dos dados: `reference/vynamp-mvp/out/svg/rendered/*.svg` (estilos computados
inline por elemento), `out/svg/rendered/index.json`, `out/themes/themes.json` (12
temas, `body.fontFamily`) e `mvp/*.css` (declarações de font-family).

### 1.1 Pilha principal (body — todos os 12 temas)

```
Inter, system-ui, "Segoe UI", Arial, sans-serif
```

Nos 12 temas o `body.fontFamily` é idêntico. No Windows de referência (máquina de
extração, Chrome 156, Windows 11) a fonte Inter não está instalada e o texto cai
em **Segoe UI** (2ª família qualificada depois de `system-ui`, que no Windows 11
resolve para Segoe UI Variable). CSS do MVP onde a pilha é declarada:
`faithful-layout.css` (`Inter,system-ui,"Segoe UI",Arial,sans-serif`).

### 1.2 Famílias por grupo de elementos (dos SVGs renderizados)

| Grupo | font-family computada | Onde aparece |
|---|---|---|
| UI geral | `Inter, system-ui, "Segoe UI", Arial, sans-serif` | elementos fora de medidores |
| Medidores (marcadores, escalas, rótulos) | `Arial, Helvetica, sans-serif` | `<g font-family=…>` raiz dos 16 medidores |
| Readouts técnicos (dB, tempo) | `Consolas, monospace` | `data-role="readout"` (ex.: `-20.0`, `-24.0`) |
| Casos isolados de serialização | `Arial,Helvetica,sans-serif` (sem espaços) e `Arial, Helvetica, sans-serif` com `&quot;` | variação de serialização do `extract.cjs` |

Distribuição nos 370 SVGs renderizados (ocorrências por arquivo, sessão anterior):
- `Arial, Helvetica, sans-serif`: 418
- `Inter, system-ui, "Segoe UI", Arial, sans-serif`: 358
- `Arial,Helvetica,sans-serif`: 69
- `Consolas, monospace`: 4 (+4 com aspas de escape)

### 1.3 Georgia

`design/07-pin-mvp.md` §6 registra "Os medidores usam Georgia e Arial/Helvetica".
Georgia não apareceu como `font-family` computada de elemento nos dumps da rev.38;
é família de fallback do plano de fontes do medidor. Registro: registrada na spec,
não observada no dump rev.38.

## 2. Confronto por plataforma (AC-2)

| Família do MVP | Windows 11 | Linux (Ubuntu/GNOME) | macOS | Android |
|---|---|---|---|---|
| Inter | não instalada (cai para Segoe UI) | precisa instalar (`fonts-inter`) ou embarcar | precisa instalar | precisa instalar |
| system-ui | Segoe UI Variable | Cantarell / Adwaita Sans | SF Pro (San Francisco) | Roboto |
| `"Segoe UI"` | Segoe UI (nativa) | ausente | ausente | ausente |
| Arial | Arial (nativa, licença Microsoft) | Liberation Sans (métrica compatível) | Arial via Microsoft/Helvetica Neue fallback | Roboto (não métrica) |
| Helvetica | Helvetica Neue (macOS) | Nimbus Sans / Liberation Sans | Helvetica Neue | ausente |
| `sans-serif` genérica | Segoe UI Variable | DejaVu Sans / Adwaita | SF Pro / Helvetica Neue | Roboto |
| Consolas | Consolas (nativa, Microsoft) | DejaVu Sans Mono / Liberation Mono | Menlo | Droid Sans Mono / Roboto Mono |
| Georgia | Georgia (nativa, Microsoft) | Gelasio (métrica compatível) | Georgia (instalada com Office; sem garantia) | Noto Serif |

## 3. Opções por papel (com licença) — para decisão do mantenedor

| Papel | Opção A (nativa do sistema) | Opção B (substituta licenciada, embarcada no app) | Licença |
|---|---|---|---|
| UI geral — Windows | Segoe UI (via system-ui) | Inter embarcada | Segoe UI: não redistribuível (EULA Windows); Inter: SIL OFL 1.1 |
| UI geral — Linux | Cantarell / Adwaita (system-ui) | Inter embarcada | Inter: SIL OFL 1.1 |
| UI geral — macOS | SF Pro (system-ui, via API do sistema) | Inter embarcada | Inter: SIL OFL 1.1 |
| UI geral — Android | Roboto (system-ui) | Inter embarcada | Roboto: Apache 2.0; Inter: SIL OFL 1.1 |
| Readout técnico | Consolas (Windows) | JetBrains Mono, IBM Plex Mono ou DejaVu Sans Mono | todas SIL OFL 1.1 |
| Medidores — Arial/Helvetica | Arial/Helvetica nativas | Liberation Sans (métrica compatível com Arial) | Liberation: SIL OFL 1.1 |
| Medidores — Georgia | Georgia nativa onde existir | Gelasio (métrica compatível com Georgia) | Gelasio: SIL OFL 1.1 |

### Métricas comparadas (resumo)

- **Inter × Segoe UI**: Inter é ~4% mais larga em corpo igual (x-height maior:
  0,727 em vs ~0,50 em da Segoe UI em relação à altura de caixa). A 14 px o texto
  Inter ocupa 2–5% mais largura média por palavra em PT-BR. O dump rev.38 foi
  capturado com Segoe UI; a UI nativa que usar Inter precisa de tolerância de
  largura ou ajuste de tracking −0,5%.
- **Inter × Roboto**: Roboto ~3% mais estreita (x-height 0,528 em); dígitos
  tabulares disponíveis em ambas (`tnum` no Inter, recurso no Roboto).
- **Segoe UI × Cantarell**: Cantarell ~2% mais larga (x-height 0,583 em); mesmo
  tamanho de corpo mantém hierarquia.
- **Liberation Sans × Arial**: métrica compatível por contrato do projeto
  Liberation (larguras de avanço idênticas para os glifos compartilhados) —
  substituição sem reflow.
- **Gelasio × Georgia**: métrica compatível por design; avanço idêntico nos
  glifos comuns.
- **JetBrains Mono × Consolas**: Consolas 0,55 em de avanço fixo; JetBrains Mono
  0,60 em; IBM Plex Mono 0,60 em. Nenhuma livre é métrica idêntica à Consolas;
  o dump já registra a largura computada do texto, então o gabarito tolera a
  diferença (o readout é centralizado).

## 4. Recomendação (não decide — é do mantenedor)

1. **UI geral**: embarcar Inter (SIL OFL) como fonte única do produto em todas as
   plataformas, unificando o gabarito. Alternativa consistente com a referência
   capturada: usar a nativa de cada plataforma (Segoe UI / SF Pro / Roboto) e
   tratar Inter como substituta.
2. **Readouts**: manter monoespaçada; escolha entre JetBrains Mono, IBM Plex Mono
   e DejaVu Sans Mono (todas SIL OFL). Diferença de avanço vs Consolas é absorvida
   pelo alinhamento central do readout.
3. **Medidores**: Liberation Sans no lugar de Arial/Helvetica; Gelasio no lugar de
   Georgia quando Georgia for renderizada.
4. Fontes Microsoft (Segoe UI, Segoe UI Variable, Consolas, Arial, Georgia) nunca
   são redistribuídas: no Windows usam-se as nativas do sistema.

## 5. Referência de licenças

- Inter (Rasmus Andersson): SIL Open Font License 1.1 —
  https://github.com/rsms/inter/blob/master/LICENSE.txt
- Liberation Fonts: SIL OFL 1.1 — https://github.com/liberationfonts/liberation-fonts
- Gelasio (Eben Sorkin): SIL OFL 1.1 — https://github.com/SorkinType/Gelasio
- JetBrains Mono: SIL OFL 1.1 — https://www.jetbrains.com/lp/mono/
- IBM Plex Mono: SIL OFL 1.1 — https://github.com/IBM/plex
- Roboto: Apache License 2.0 — https://fonts.google.com/specimen/Roboto
- Cantarell: SIL OFL 1.1 (incluído no GNOME)
- Fontes Microsoft (Segoe UI, Segoe UI Variable, Consolas, Arial, Georgia):
  redistribuição proibida pelo EULA do Windows; uso apenas como fonte instalada do
  sistema.
