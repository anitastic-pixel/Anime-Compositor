# D-319: Float working depth

Written by `tests/b200_float_depth.rs`. A composition's working depth is Display (0 to 1, every file before D-319) or Float (past white), as After Effects' 8 and 32 bpc. In Float, Add is not held to 1, Screen keeps the brighter where both are past white (Nuke's rule), and Fractal Noise goes past white. A composition inside another draws in the outermost one's depth. Every expected pixel is `Fixtures/float_depth/expected_float_depth.json`, written by `tools/float_depth_reference.py` before the build had Float. Tolerance 2e-5.

## The fixtures

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BLEND-ADDF-001 frame 0: Float. The card, and above it in Add the card two stops up (four times): white plus four times white is five, not held to 1. | largest difference 8.7e-7 | yes |
| FX-BLEND-ADDF-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLEND-ADDF-002 frame 0: The same file in Display (no `float_depth`): Add held to 1, as before D-319. | largest difference 3.5e-7 | yes |
| FX-BLEND-ADDF-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLEND-ADDF-003 frame 0: Float. The card in Add on the card, nothing brightened: white plus white is 2. | largest difference 3.7e-7 | yes |
| FX-BLEND-ADDF-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLEND-SCRF-001 frame 0: Float. Both cards two stops up, the top in Screen: where both are past white the brighter is kept, Nuke's rule. | largest difference 7.7e-7 | yes |
| FX-BLEND-SCRF-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLEND-SCRF-002 frame 0: Float. The top card two stops up in Screen on the plain card: one of the two is at most 1 everywhere, so `cs + cd - cs cd` throughout. | largest difference 4.1e-7 | yes |
| FX-BLEND-SCRF-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLEND-SCRF-003 frame 0: FX-BLEND-SCRF-001 in Display: `cs + cd - cs cd` as before D-319, which turns down past white. | largest difference 3.6e-6 | yes |
| FX-BLEND-SCRF-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FNOISE-HDR-001 frame 0: Float. Fractal Noise, size 4, contrast 300, brightness 50, on the card: the brightest clouds go past white, the darkest still stop at black. | largest difference 1.2e-7 | yes |
| FX-FNOISE-HDR-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FNOISE-HDR-002 frame 0: FX-FNOISE-HDR-001 in Display: held to 0..1, as before D-319. | largest difference 3.4e-8 | yes |
| FX-FNOISE-HDR-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FNOISE-HDR-003 frame 0: Float. The card two stops up, then FX-FNOISE-HDR-001's noise in Screen: where both are past white the brighter is kept. | largest difference 8.4e-7 | yes |
| FX-FNOISE-HDR-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| fx_blend_addf_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blend_addf_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |

## Saved, read back and undone

| Check | The build's answer | Matches |
| --- | --- | --- |
| A file without the setting is Display and saves no float_depth line | no line | yes |
| Set to Float, it is written as float_depth: true | Some(Bool(true)) | yes |
| And read back as Float | Float | yes |
| FX-BLEND-ADDF-002 set to Float through the command draws FX-BLEND-ADDF-001's frame | largest difference 8.7e-7 | yes |
| Undo puts it back to Display, and nothing is written | Display, no line | yes |
| And the frame is FX-BLEND-ADDF-002's again | largest difference 3.5e-7 | yes |
| Set back to Display, no float_depth line is written | no line | yes |
| A file with float_depth "yes" is refused, not guessed | Some("PROJECT_SCHEMA_INVALID") | yes |
| A file with float_depth 1 is refused, not guessed | Some("PROJECT_SCHEMA_INVALID") | yes |
| A file with float_depth null is refused, not guessed | Some("PROJECT_SCHEMA_INVALID") | yes |

## A composition inside another follows the outermost one

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BLEND-ADDF's layers inside a composition: outer Float, inner Display, draws FX-BLEND-ADDF-001's frame | largest difference 8.7e-7 | yes |
| FX-BLEND-ADDF's layers inside a composition: outer Display, inner Float, draws FX-BLEND-ADDF-002's frame | largest difference 3.5e-7 | yes |
| FX-BLEND-ADDF's layers inside a composition: outer Float, inner Float, draws FX-BLEND-ADDF-001's frame | largest difference 8.7e-7 | yes |

## The preview card

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_blend_addf_001.json: Float, drawn by the CPU, the CPU's picture exactly, and said as GPU_PREVIEW_ON_CPU | CPU, largest difference 0 of 255 | yes |
| fx_blend_addf_002.json: Display, drawn on the card as before, within 1 level of 255 | card, largest difference 0 of 255 | yes |
| fx_fnoise_hdr_001.json: Float, drawn by the CPU, the CPU's picture exactly, and said as GPU_PREVIEW_ON_CPU | CPU, largest difference 0 of 255 | yes |

## Files from before D-319

| Check | The build's answer | Matches |
| --- | --- | --- |
| The reference shot is Display in every composition and saves no float_depth line | Display, no line | yes |

## Result

37 of 37 checks pass.
