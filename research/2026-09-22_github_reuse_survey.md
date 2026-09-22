# GitHub reuse survey: what already exists that we could branch from or take in

Research date: 2026-09-22. Method: three web searches run side by side, reading each project's GitHub page, LICENSE file and crates.io entry. This is research. It authorises nothing: every item below would still need an owner decision in document 14, its fixtures written first, and a dependency record under document 10.

Licence rule used throughout: this project is MIT OR Apache-2.0. Code under a permissive licence (MIT, Apache-2.0, BSD) may be copied or ported, keeping its copyright notice. Code under GPL, LGPL or MPL can be studied, but not copied in.

## 1. Forking a whole app: no

The earlier view (document 30, D-15) is confirmed. Every mature layer-based animation tool on GitHub is GPL. That applies to Friction (the successor to Enve), Synfig and Glaxnimate. Natron, Blender and Olive are also GPL, and they are node-based. Rebasing onto any of them would make this project GPL and throw away the Rust core, the fixtures and the ADRs.

The permissive apps do not fit either:
- OpenToonz and Tahoma2D are BSD, but they are C++ exposure-sheet and node-graph applications.
- Graphite (Apache-2.0, Rust) is node-first, and its keyframe animation is still on its roadmap.

No newer Rust layer-based compositor exists. Two repositories carrying After Effects names ("ImageBeautician/adobe-after-effects-software", "FastMakerString/after-effects-pro-setup") look like malware bait. Avoid them.

Friction and Natron remain useful as references for how things should behave.

## 2. Anime-specific effects (roadmap G3a, G3b): a lot is available

| Our planned effect | Code that exists | Licence |
|---|---|---|
| Line smoothing (like OLM Smoother) | OpenToonz `toonz/sources/common/trop/tantialias.cpp`, about 400 lines. It uses the same method OLM Smoother uses (MLAA). | BSD-3 |
| Line smoothing, a second method | loilo-inc/smooth (the algorithm is separate from the After Effects glue), and cr-market/anime-smoother-ofx (an OpenFX port with tests) | Apache-2.0 |
| Main-line repaint, selective colour blur, colour key and switch, rim fill, line extraction, thin, edge line | bryful/F-s-PluginsProjects (about 40 effects). cr-market/FsPluginsOFX ports 8 of them in cleaner form. akahito-ot/Fs-Plugins-Fusion-Ports has short readable versions. | MIT |
| Directional, radial, rotate and motion blur; line blur; erode and dilate; glow, bloom and glare; RGB and HSV key; gradients | OpenToonz `stdfx/`: `igs_*` and `ino_*`, `iwa_directionalblurfx`, `iwa_bloomfx`, `glowfx`, `erodilatefx`, `rgbkeyfx`, `hsvkeyfx`. The `igs_*` cores are plain C++ with no OpenToonz types. | BSD-3 |
| Toon Dilate, distance gradation | No source exists. OLM publishes only built plugins, although they are Apache-2.0. OLM's page describes the method: fill outward with the nearest opaque colour. That is a nearest-seed distance transform, small to write ourselves. | — |
| KiraKira, paraffin gradients | Nothing found. OpenToonz glare and body-highlight are the closest. | — |

## 3. Timesheet import (roadmap G2b): a spec and several readers

- The official XDTS specification is a public PDF from CELSYS. It is the thing to write our parser from: vd.clipstudio.net/clipcontent/paint/app/ToeiAnimation/XDTSFileFormat_en.pdf
- Readers to test against:
  - OpenToonz `toonz/xdtsio.cpp` (BSD-3);
  - opentoonz/xdts_viewer (BSD, including the "uext" extension);
  - lunafuse/vdts-editor (MIT, one HTML file; it also reads **TDTS**, and is the only TDTS reader found);
  - stechdrive/xsheet-remap (MIT, TypeScript).
- CSP CSV: only ChenxingM/TimeSheetReader (MIT, an After Effects script, not checked in detail).
- There is no Rust XDTS crate.

## 4. Rust libraries for other planned work

| Planned item | Best candidate | Licence | Recommendation |
|---|---|---|---|
| Shape strokes with miter and bevel joins, caps, trim paths | kurbo (Linebender; 46M downloads; released 2026-05) | MIT/Apache | Take it when D-78's excluded joins and trim paths are asked for |
| Merge paths (boolean ops) | i_overlay (very active) | MIT/Apache | Take it if merge paths are asked for |
| Gradients, repeaters | none needed | — | Write ourselves; each is a few lines |
| Text layers | harfrust (shaping) plus skrifa (glyph outlines), drawn by our own shape rasteriser; parley if wrapping and fallback fonts are needed | MIT/Apache | Best fit, because text then matches shapes exactly |
| LUT (.cube) | all LUT crates are weak or stale | — | Write ourselves (about 200 lines) |
| OCIO / ACES | ocio-rs (young; builds C++ OpenColorIO) | BSD-3 | Only when a real OCIO config must be honoured |
| OpenFX host (roadmap G3e) | ASWF openfx headers and its C++ HostSupport library | BSD-3 | No Rust host exists. It is a large job, written by us with HostSupport as a reference. |
| GPU path (gated by ADR-006) | wgpu | MIT/Apache | Only if the stopwatch trigger fires |
| Lottie | velato (reader; pulls in the vello GPU renderer) | MIT/Apache | Use as a model; write serde_json structs ourselves |
| Motion blur, frame blending | nothing suitable | — | Write ourselves from our own transform data |
| Audio waveform | none needed (symphonia is MPL-2.0; avoid) | — | We already read WAV |

## 5. Most worth doing, in order of value to anime work

1. **XDTS timesheet import**, written from the CELSYS PDF and checked against OpenToonz and vdts-editor. Nothing in Rust exists, and every reference is permissive.
2. **Line smoothing**, ported from OpenToonz `tantialias.cpp` (BSD), with loilo `smooth` (Apache) as a second opinion.
3. **Main-line repaint, selective colour blur and colour key**, ported from F's Plugins (MIT) and FsPluginsOFX.
4. **Glow and directional blur**, from OpenToonz `stdfx` (BSD).

Each would follow the usual order: a decision in document 14, fixtures first, then the build. Every port records its origin and keeps its notice (document 10).
