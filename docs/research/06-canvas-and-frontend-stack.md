---
status: DRAFT
---

# 06 — Canvas Rendering & Frontend Stack Research

Decision input for `architecture/06-tech-stack.md`: how should Smith render its infinite
pan/zoom UML canvas, and what frontend stack should wrap it?

**Verdict up front:** Ship **Tauri 2 + React + TypeScript + Konva.js** (HTML canvas, 2D
context). Fallback: **Tauri 2 + JointJS core** if the build-it-ourselves interaction layer
proves too slow. Pure-Rust (egui) is the contingency only if the system-webview dependency
is ever rejected outright. Everything below justifies this against the decision matrix.

Research date: 2026-08-09. All versions and licenses verified against npm registry and
project sites on that date; claims that could not be verified are marked `[INFERENCE]`.

---

## 1. What the canvas must do (requirements recap)

From the vision/design docs, the canvas needs:

1. **Infinite pan/zoom** at 60 fps with *hundreds* of elements (class boxes with
   compartments, lifelines, activity nodes), plus connectors with user-editable bend points.
2. **Crisp text at every zoom level** — UML is mostly text (names, signatures,
   stereotypes, multiplicity labels). This is the harshest requirement and kills several
   approaches.
3. **Hit-testing** for selection, ports (connection points), handles, and the
   context-sensitive gizmo.
4. **Connection routing** — orthogonal connectors that avoid obstacles, re-routed on
   demand, with user bends persisted as presentation data (design principle P1).
5. **Beautiful, restrained chrome** (P7) — themable, high-DPI, no visual noise.
6. **Single binary** (P10) and **embedded MCP server in the same Rust process** (P3/P10).

Scale note: Smith's diagrams hold *hundreds* of elements, not tens of thousands (a class
diagram with 200 classes is already huge). This matters: it removes the "must have WebGL"
constraint and puts text quality and interaction ergonomics first.

---

## 2. Architecture-level decision matrix (options a / b / c)

Score: ● strong · ◐ partial · ○ weak.

| Criterion | (a) Tauri 2 + web canvas | (b) Pure native Rust | (c) Tauri 2 + WebGPU/WASM |
|---|---|---|---|
| Infinite-canvas render perf (hundreds of elements) | ● trivially; 2D canvas suffices | ● egui/Vello are fast, but | ● same as (a), GPU backend |
| Text crispness at arbitrary zoom | ● browser text stack, re-rasterized per zoom | ◐ egui glyph atlas: good, but bespoke control & IME work | ◐ WebGL text needs SDF/mipmaps care |
| Hit-testing | ● built into canvas libs | ○ manual in immediate mode | ● via lib event systems |
| Connection routing ecosystem | ● JointJS/maxGraph routers, ELK | ○ port or reimplement | ● same as (a) |
| Auto-layout ecosystem (ELK.js, Graphviz WASM) | ● runs natively in the webview | ○ no maintained Rust ports; need WASM bridge | ● |
| Bespoke polished UI (P5, P7) | ● CSS/design-system maturity, huge component ecosystem | ◐ egui theming limited; Slint good but licensing | ● |
| Single-binary packaging (P10) | ● assets embedded in Rust binary; OS supplies webview runtime | ●● no webview at all | ● same as (a) |
| Rust integration / MCP in-process (P3) | ● Tauri commands; MCP server is a task in the host process, independent of UI | ● direct calls | ● same as (a) |
| Maintenance signal for years | ● Tauri 2.9.x stable; web libs active | ◐ fragmented; Xilem experimental | ● |
| Effort to "beautiful" | ● low — web design tooling | ○ high — custom widget work | ● |

**(c) is not a distinct architecture** — WebGPU is just PixiJS v8's rendering backend
inside the same webview as (a), and WASM runs in the same webview too. The real decision is
**(a) vs (b)**. (a) wins on every row except the one thing (b) owns: zero webview runtime
dependency. On Windows the webview runtime is WebView2 (Evergreen, preinstalled on
Win 11 and Win 10 since 2020); on macOS it's WKWebView (OS component); on Linux it's
WebKitGTK (distro package). The *app binary itself* is single-file: Tauri embeds the built
frontend assets into the Rust binary at compile time ([Tauri config](https://v2.tauri.app/reference/config/),
[resources](https://v2.tauri.app/zh-cn/develop/resources/)). P10 holds.

The MCP server constraint is orthogonal to this choice: with rmcp it runs as a task in the
Tauri host process bound to 127.0.0.1, whether the UI is web or native. It does not favor
(b).

---

## 3. Web canvas / diagramming libraries (stack (a) internals)

Versions/licenses verified 2026-08-09 via npm registry.

| Library | Ver | License | Renderer | Fit for Smith |
|---|---|---|---|---|
| [Konva.js](https://konvajs.org/) | 10.3.0 | MIT | Canvas 2D | **Best fit.** Scene graph, groups/transforms, built-in hit layer (`stage.getIntersection`), text via `fillText` re-rasterized at current zoom (crisp at all scales), declarative bindings `react-konva`/`vue-konva`/`svelte-konva`. No diagram semantics — Smith builds UML shapes itself, which matches P1 (Rust owns the model; canvas is pure presentation). |
| [PixiJS](https://pixijs.com/) | 8.19.0 | MIT | WebGL2/WebGPU | Fastest by far (thousands–tens of thousands of sprites), but text crispness under continuous zoom needs SDF/mipmap care, and hit-testing is lower-level (`eventMode` + `hitArea`). Right tool at 10k+ elements; overkill at hundreds. Escape hatch if Konva ever hits a ceiling. |
| [Fabric.js](https://fabricjs.com/) | 7.4.0 | MIT | Canvas 2D | Object model + SVG import/export, aimed at image/design editing, not connector-heavy diagramming. Weaker fit than Konva for ports/edges. |
| [JointJS core](https://github.com/clientio/joint) | 3.7.7 (4.0.0-alpha.4) | **MPL-2.0** (core) | SVG | Batteries-included diagramming: elements, links, ports, vertices, tool handles, `manhattan`/`orthogonal` routers in core. Cost: its own graph model duplicates the Rust model (P1 friction — must be used as a synced view only), SVG perf ceiling (~thousands of elements; users report strain at 10k+), and v4 (drops jQuery/Backbone/Lodash) is still alpha on npm. Advanced UI (stencils, inspectors) is the commercial **JointJS+**. |
| [GoJS](https://gojs.net/) | n/a | **Proprietary, $6,990/team** | Canvas | Excellent orthogonal routing, but not open source and paid — excluded on license/cost. |
| [mxGraph](https://github.com/jgraph/mxgraph) / [maxGraph](https://github.com/maxGraph/maxGraph) | mxGraph 4.2.2 **archived 2020**; maxGraph 0.24.0 | Apache-2.0 | SVG/HTML | mxGraph (drawio's engine) is battle-tested for exactly this problem but archived; [maxGraph](https://maxgraph.github.io/maxGraph/docs/intro/) is the maintained TS successor. Its value to Smith today is as a **reference implementation** (orthogonal edge routing, folding, swimlanes) rather than a dependency: 0.x API churn, thin community. |
| [React Flow (@xyflow/react)](https://reactflow.dev/) | 12.11.2 | MIT | DOM/SVG | Great node-graph editor; UML compartments, ports, and orthogonal routing are all DIY. Wrong altitude for full UML. |
| [tldraw SDK](https://tldraw.dev/) | 4.x | **Non-commercial free; commercial requires paid license key (~$6k/yr/team)** | Canvas | Technically superb infinite-canvas SDK, but since SDK 4.0 (Sep 2025) production use needs a license key ([license](https://tldraw.dev/community/license), [announcement](https://tldraw.dev/blog/tldraw-sdk-4-0)). Excluded for a commercial product on license grounds; fine to study. |
| [Excalidraw](https://github.com/excalidraw/excalidraw) | 0.18.1 | MIT | Canvas | Hand-drawn whiteboard aesthetic — the opposite of precise UML notation (P7). Not a fit. |
| [Litegraph.js](https://github.com/jagenjo/litegraph.js) | 0.x | MIT | Canvas 2D | Node/patch-cable editor; no UML affordances; low maintenance. Not a fit. |

**Key architectural observation.** Design principle P1 makes the Rust graph DB the single
source of truth; diagrams store only view references + presentation. Diagramming *frameworks*
(JointJS, GoJS, React Flow) ship their own model layer — adopting one means double
book-keeping and event-synchronizing two graphs. Renderer-level libraries (Konva, PixiJS)
match Smith's architecture: Rust owns facts, TypeScript renders views. This is why the
shortlist is renderer + interaction layer, not an all-in-one diagramming framework.

---

## 4. Native Rust GUI options (stack (b))

| Framework | License | Model | Canvas/2D capability | Fit for Smith |
|---|---|---|---|---|
| [egui](https://github.com/emilk/egui) | MIT OR Apache-2.0 | Immediate mode | Pan/zoom scenes supported (zoomable scene since 0.31, with crispness improvements); custom drawing via `Painter` | Most mature pure-Rust option; used in Rerun, Bevy tooling. But: text is glyph-atlas based (good, yet you own shaping edge cases/IME), all UML widgets/handles are DIY in immediate mode, and "beautiful bespoke chrome" is egui's weakest axis. |
| [iced](https://github.com/iced-rs/iced) | MIT | Elm-like retained, wgpu renderer | `Canvas` widget exists; pan/zoom DIY | Clean DX, but diagramming-scale canvas and rich text layouts are less proven. |
| [Slint](https://slint.dev/) | **GPLv3 OR royalty-free OR commercial** ([LICENSE.md](https://github.com/slint-ui/slint/blob/master/LICENSE.md), [pricing](https://slint.dev/pricing)) | Declarative `.slint` DSL | Custom drawing via canvas elements, limited vector scene | Excellent forms/tooling, but no first-class infinite vector canvas, and licensing (GPLv3 copyleft vs paid commercial) adds friction for a closed-source product. |
| [Dioxus](https://github.com/DioxusLabs/dioxus) | MIT/Apache | React-like | Desktop renderer = **system webview** (same as Tauri); native wgpu renderer "Blitz" is experimental | If you pick Dioxus desktop you get Tauri's webview trade without Tauri's maturity; Blitz (native GPU rendering) shipped experimentally in 0.7 but is not production-proven. |
| [Xilem + Vello (Linebender)](https://github.com/linebender) | MIT/Apache | Experimental retained widgets + GPU compute vector renderer | [Vello](https://github.com/linebender/vello) is the most promising Rust 2D vector renderer (large scenes, GPU compute); Parley/Fontique text stack | The long-term future of native Rust 2D, but explicitly experimental (Xilem 0.x, [Aug 2025 status](https://linebender.org/blog/tmil-20/)). **Watch, don't build on.** Candidate migration target in 2–3 years. |
| [floem](https://github.com/lapce/floem) | MIT | Reactive, Vello/wgpu | Lapce editor uses it; canvas story young | Younger than egui/iced; editor-centric lineage. Not the pick. |

Summary: no native Rust stack today delivers *beautiful, bespoke, text-heavy, interactive
vector diagrams* with less total effort than the web stack. The gap is not raw rendering
speed — it's the surrounding ecosystem (text layout, accessibility, design tooling,
layout engines) and the person-years of custom widget work.

---

## 5. Frontend framework (given stack (a))

The canvas is rendered by the canvas library, not the DOM, so the framework's job is the
chrome: menu bar, model explorer tree, inspector, issue panel, RTM views (all DOM forms —
the framework's strength).

| Framework | Bundle/DX | Diagramming ecosystem | Verdict |
|---|---|---|---|
| **React** | ~45 kB min runtime; hooks model fine since heavy per-frame work stays out of React (canvas imperative) | Largest: `react-konva` is the best-maintained canvas binding; every diagram lib ships a React adapter | **Pick.** Ecosystem gravity + hiring pool. |
| Svelte 5 | Smallest bundles (compiler-first, [comparisons](https://strapi.io/blog/svelte-vs-react-comparison)) | `svelte-konva` exists (v1.0.1) but thin; Svelte Flow exists | Fine second choice; less third-party muscle. |
| SolidJS | Tiny, fine-grained reactivity | Minimal canvas bindings | Interesting reactivity model, ecosystem too thin. |
| Vue | Mature | `vue-konva` exists | Fine, but no advantage over React here. |

Recommendation: **React 19 + TypeScript (strict)**, with canvas state kept in Konva/imperative
stores and React owning panels. (If bundle purism ever matters, Svelte is the drop-in
alternative; the renderer choice is unaffected.)

---

## 6. Top-3 ranked recommendations for Smith

### #1 — Tauri 2 + React + TypeScript + Konva.js  ✅ primary
- **Why:** crisp text at all zooms (Canvas 2D re-rasterizes on zoom), built-in hit layer,
  MIT, active (v10.3.0), declarative `react-konva`, renderer-only model matches P1.
  Hundreds of elements at 60 fps is comfortable for Canvas 2D; GPU not needed.
- **Tradeoffs:** no diagram semantics for free — compartments, ports, bend-point handles,
  marquee select, and the gizmo are built in Smith's TS interaction layer (~months of work,
  but that work defines Smith's novel UX per P5 anyway); SVG/PDF export must be implemented
  (walk the scene to SVG — straightforward); single-threaded JS (mitigate: layout/routing
  in Web Workers, heavy graph work already in Rust).

### #2 — Tauri 2 + JointJS core (MPL-2.0)
- **Why:** fastest route to working UML-ish diagrams: elements/links/ports/vertices and
  `manhattan`/`orthogonal` routers ship in the open-source core.
- **Tradeoffs:** SVG performance ceiling; must use JointJS purely as a synced view of the
  Rust model (P1 discipline); v4 is alpha on npm; MPL-2.0 file-level copyleft is compatible
  with a proprietary app but requires publishing modifications to JointJS files; the
  polished UX parts (stencils/inspectors) sit in paid JointJS+.

### #3 — Pure Rust: Tauri-less egui/eframe app
- **Why:** true zero-webview single binary; one language end-to-end; egui is the most
  production-proven Rust GUI (zoomable scenes, used by real products).
- **Tradeoffs:** the "beautiful, ergonomic, eye-pleasing UI" requirement is hardest to meet
  here — every widget, animation, text-layout edge case, and accessibility item is DIY;
  no ELK.js/Graphviz without WASM bridges; largest schedule risk by a wide margin.
  [INFERENCE] effort estimate relative to #1: roughly 2–3× for equivalent polish.

**Rejected:** GoJS and tldraw SDK (paid/commercial-key licensing); Excalidraw (wrong
aesthetic); mxGraph proper (archived; use maxGraph only as reference); Litegraph/React Flow
(wrong altitude); Slint (licensing + no vector canvas); Dioxus-desktop (webview anyway,
less mature than Tauri); Xilem/Vello/floem (experimental — revisit in ~2 years).

---

## 7. Connection routing libraries (orthogonal connectors)

Smith connectors: orthogonal, object-avoiding, re-routable on demand, with user-editable
bend points persisted as presentation (P1). Algorithms are referenced, not re-derived (P6) —
see [Orthogonal Connector Routing (Wybrow et al.)](https://researchgate.net/publication/43433430_Orthogonal_Connector_Routing).

> Note: the `routing-orthogonal-connector` npm package named in the brief **does not exist**
> (npm registry returns 404, verified 2026-08-09). The options below are the real ones.

| Option | License | Notes |
|---|---|---|
| [JointJS core routers](https://docs.jointjs.com/) (`manhattan`, `orthogonal`) | MPL-2.0 | Routers + connectors usable even if you render elsewhere; battle-tested in production diagram tools. |
| [maxGraph edge routing](https://maxgraph.github.io/maxGraph/docs/intro/) (mxGraph successor) | Apache-2.0 | drawio's orthogonal/manhattan edge handlers — the most battle-tested OSS orthogonal routing in existence; treat as reference implementation to port/study. |
| [libavoid / adaptagrams](https://github.com/mjwybrow/adaptagrams) (Monash) | LGPL (adaptagrams) | Academic-grade incremental object-avoiding orthogonal + polyline routing (C++). Best algorithmic pedigree. |
| [obstacle-router](https://www.npmjs.com/package/obstacle-router) | LGPL-2.1 | TypeScript port of libavoid, v0.1.2 (2026) — young, single maintainer; usable, but vendor carefully. |
| [Bukk94/OrthogonalConnectorRouting](https://github.com/Bukk94/OrthogonalConnectorRouting) | OSS (small) | Reference implementation of the grid/A*-style orthogonal routing from the [well-known article](https://medium.com/swlh/routing-orthogonal-diagram-connectors-in-javascript-191dc2c5ff70); port to TS/Rust is small. |
| ELK edge routing via elkjs | EPL-2.0 OR GPL-3.0+ | ELK's layered pipeline includes orthogonal edge routing; usable standalone for whole-diagram re-route. |
| GoJS / yFiles routers | Proprietary | Noted for completeness; excluded on license. |

**Plan:** interactive single-connector re-route = lightweight A*/visibility approach in
Smith's TS (or Rust, exposed via Tauri command) following the Wybrow lineage; whole-diagram
orthogonal routing = ELK (also gives coordinates for auto-layout). Bend points the user
dragged are stored and win over auto-routing (P1).

---

## 8. Auto-layout libraries

All compute coordinates only — renderer-agnostic, run in a Web Worker, usable with any
option-(a) stack. Layered/hierarchical layout follows Sugiyama et al.; reference, don't
derive (P6).

| Library | Ver (verified) | License | Algorithm family | Notes |
|---|---|---|---|---|
| [elkjs](https://github.com/kieler/elkjs) | 0.12.0 (Jul 2026) | EPL-2.0 OR GPL-3.0+ | Layered (Sugiyama), force, stress, radial; ports; edge routing | The Eclipse Layout Kernel in JS. Best quality for UML-style directed graphs with ports/compartments. Worker-based. EPL-2.0 is weak file-level copyleft — consuming the unmodified npm package is compatible with a proprietary app. |
| [@dagrejs/dagre](https://github.com/dagrejs/dagre) | 3.1.1 | MIT | Layered/Sugiyama | Maintained successor of dagre (original 0.8.5 frozen ~2020). Simpler API, fewer knobs than ELK; good default for quick "tidy diagram". |
| [@hpcc-js/wasm-graphviz](https://github.com/hpcc-systems/hpcc-js-wasm) | 1.28.0 | Apache-2.0 | Graphviz (dot/neato/fdp/sfdp/circo/twopi) | The classic `dot` engine as WASM. Mature, superb for dense dependency graphs; DOT in, coordinates/SVG out. |
| [d3-force](https://github.com/d3/d3-force) | 3.0.0 | ISC | Force-directed | Organic layouts for dependency/traceability exploration views — not for layered UML. |
| [d3-dag](https://github.com/erikbrinkman/d3-dag) | 1.x | MIT | Sugiyama in TypeScript | Pure-TS alternative to dagre; worth knowing, not primary. |

For stack (b) (pure Rust) none of these has a maintained Rust port — layout would require
embedding their WASM in-process or reimplementing — a concrete, quantified cost of choosing
(b). [INFERENCE] no actively maintained `elk` Rust binding was found during this research.

---

## 9. Recommendation

**Primary stack:** **Tauri 2.x (Rust host) + React 19 + TypeScript (strict) + Konva.js 10
(Canvas 2D)**, ELK.js for auto-layout, Wybrow-style orthogonal routing for connectors.

Rationale vs the matrix: wins every row except "zero webview dependency", which P10 does
not actually require (single *binary* ≠ no system webview; Tauri embeds all frontend assets
into the binary, and WKWebView/WebView2/WebKitGTK are OS-supplied everywhere Smith ships).
It maximizes the two scarce resources: text/interaction quality (browser text stack is the
best 2D text engine available) and years of sustainable maintenance (every component is MIT
and actively released). The renderer-only choice respects P1: the Rust model API is the
single source of truth; the webview is a view, driven through Tauri commands/events, exactly
like the MCP server is a peer client (P3).

**Fallback stack:** **Tauri 2 + JointJS core (MPL-2.0)** — switch only if the custom
interaction layer on Konva proves too slow to build; it trades performance ceiling and
model-duplication discipline for free ports/links/routers.

**Contingency:** pure-Rust **egui** build, triggered only by an explicit rejection of any
webview dependency; expect ~2–3× effort for equivalent visual polish.

**Watchlist:** Linebender Xilem/Vello (native Rust 2D done right, ~2 yrs out); Dioxus Blitz
(native rendering for React-style Rust); maxGraph (keep studying drawio's routing).

**Risks & mitigations:**
- *Webview fragmentation* (WKWebView vs WebView2 vs WebKitGTK): pin to Canvas 2D + standard
  APIs, CI smoke tests on all three; avoid WebGPU/bleeding APIs in v1.
- *JS single-threadedness*: ELK/routing in Web Workers; all graph algorithms in Rust anyway.
- *Konva perf ceiling*: [INFERENCE] ~thousands of interactive shapes at 60 fps is the
  practical envelope — beyond Smith's needs; PixiJS is the documented escape hatch.
- *Supply chain*: pin exact versions, `pnpm audit`, per repo rules.

---

## Sources

### Canvas & diagramming libraries
- Konva.js — https://konvajs.org/ (v10.3.0 via npm registry, 2026-08-09)
- Konva FAQ / library comparison — https://konvajs.org/docs/faq.html, https://konvajs.org/docs/guides/best-canvas-library.html
- PixiJS — https://pixijs.com/ (v8.19.0 via npm registry, 2026-08-09)
- Fabric.js — https://fabricjs.com/ (v7.4.0 via npm registry)
- JointJS licensing — https://www.jointjs.com/license (MPL-2.0 core)
- JointJS 4.0 dependency-free release — https://www.jointjs.com/blog/introducing-version-4
- GoJS pricing/licensing — https://gojs.net/latest/pricing, https://gojs.net/latest/learn/deployment
- mxGraph (archived) / drawio — https://github.com/jgraph/mxgraph; diagrams.net — https://en.wikipedia.org/wiki/Diagrams.net
- maxGraph (mxGraph successor) — https://github.com/maxGraph/maxGraph, https://maxgraph.github.io/maxGraph/docs/intro/
- React Flow — https://reactflow.dev/ (MIT, v12.11.2 via npm registry); xyflow open source — https://xyflow.com/open-source
- tldraw license — https://tldraw.dev/community/license; tldraw SDK 4.0 announcement — https://tldraw.dev/blog/tldraw-sdk-4-0; LICENSE.md — https://github.com/tldraw/tldraw/blob/main/LICENSE.md
- Excalidraw — https://github.com/excalidraw/excalidraw (v0.18.1 via npm registry)

### Native Rust GUI
- A 2025 Survey of Rust GUI Libraries — https://www.boringcactus.com/2025/04/13/2025-survey-of-rust-gui-libraries.html
- The State of Rust GUI — https://weeklyrust.substack.com/p/the-state-of-rust-gui-the-good-and
- egui zoomable scene (0.31) — https://www.reddit.com/r/rust/comments/1ihliwx/introducing_egui_031_with_zoomable_scene/
- Slint LICENSE.md (GPLv3 / royalty-free / commercial) — https://github.com/slint-ui/slint/blob/master/LICENSE.md; pricing — https://slint.dev/pricing; royalty-free blog — https://slint.dev/blog/slint-1.1-released
- Dioxus desktop (webview) & Blitz — https://dioxuslabs.com/learn/0.7/guides/platforms/desktop/, https://dioxuslabs.com/learn/0.7/beyond/project_structure/
- Linebender status (Aug 2025) — https://linebender.org/blog/tmil-20/; Vello — https://github.com/linebender/vello; Xilem — https://github.com/linebender/xilem

### Routing
- Routing Orthogonal Diagram Connectors in JavaScript — https://medium.com/swlh/routing-orthogonal-diagram-connectors-in-javascript-191dc2c5ff70
- Bukk94/OrthogonalConnectorRouting — https://github.com/Bukk94/OrthogonalConnectorRouting
- adaptagrams / libavoid — https://github.com/mjwybrow/adaptagrams
- obstacle-router (libavoid TS port) — https://www.npmjs.com/package/obstacle-router
- Orthogonal Connector Routing paper — https://www.researchgate.net/publication/43433430_Orthogonal_Connector_Routing

### Auto-layout
- elkjs — https://github.com/kieler/elkjs (v0.12.0, EPL-2.0 OR GPL-3.0+, verified via npm registry)
- Eclipse Layout Kernel project — https://projects.eclipse.org/projects/modeling.elk
- @dagrejs/dagre — https://github.com/dagrejs/dagre (v3.1.1 via npm registry); original dagre — https://github.com/dagre/dagre
- @hpcc-js/wasm (Graphviz WASM) — https://github.com/hpcc-systems/hpcc-js-wasm, https://hpcc-systems.github.io/hpcc-js-wasm/
- d3-force — https://github.com/d3/d3-force
- d3-dag — https://github.com/erikbrinkman/d3-dag

### Framework & packaging
- Tauri 2 config/asset embedding — https://v2.tauri.app/reference/config/; resources — https://v2.tauri.app/zh-cn/develop/resources/
- Svelte vs React 2026 comparison — https://strapi.io/blog/svelte-vs-react-comparison
- Tauri 2 impressions (practitioner) — https://www.reddit.com/r/rust/comments/1nvvoee/built_a_desktop_app_with_tauri_20_impressions/

*Versions and licenses verified against the npm registry and project license files on 2026-08-09.*
