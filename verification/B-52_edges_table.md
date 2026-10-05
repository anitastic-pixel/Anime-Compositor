# B-52: Repeat Edge Pixels

D-109, proposed on 2026-09-26 and built at the owner's "proceed, add to other blurs if needed". Every expected pixel is `Fixtures/edges/expected_edges.json`, written by `tools/edges_reference.py` before this code existed and printed in document 25 as FX-EDGES-001 to 012. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-EDGES-001 to 012 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-EDGES-001 frame 0: Gaussian Blur, sigma 1, edges repeat, on a picture that fills the layer: every pixel stays fully covered, the edges as solid as the middle, and the middle is blurred exactly as without it. | largest difference 1.2e-7 | yes |
| FX-EDGES-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EDGES-002 frame 0: Gaussian Blur, sigma 1, edges written "transparent": the rule as it always was, the edges fading, the same as a file without the setting. | largest difference 1.7e-7 | yes |
| FX-EDGES-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EDGES-003 frame 0: FX-EDGES-001 moved three pixels right: the layer does not grow, so the three columns left of it stay empty. | largest difference 1.2e-7 | yes |
| FX-EDGES-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EDGES-004 frame 0: Directional Blur, direction 90, length 6, edges repeat, on the picture: streaked left and right, every pixel still fully covered. | largest difference 1.1e-7 | yes |
| FX-EDGES-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EDGES-005 frame 0: Directional Blur, direction 30, length 8, edges repeat: a slanting streak whose samples leave the layer across both edges, every pixel still fully covered. | largest difference 1.5e-7 | yes |
| FX-EDGES-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EDGES-006 frame 0: Directional Blur, direction 90, length 6, edges repeat, on the bars moved three pixels right: the line on the drawing's left edge carries on past it, so its column keeps more of the line than FX-DIRBLUR-009's, and nothing is drawn left of the drawing. | largest difference 1.5e-7 | yes |
| FX-EDGES-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EDGES-007 frame 0: Radial Blur, spin 30 about the middle, edges repeat, on the picture: every pixel still fully covered, the corners too. | largest difference 1.5e-7 | yes |
| FX-EDGES-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EDGES-008 frame 0: Radial Blur, zoom 40 about the top left corner, edges repeat: every pixel still fully covered. | largest difference 1.1e-7 | yes |
| FX-EDGES-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EDGES-009 frame 0: A Directional Blur, direction 90 and length 4, edges transparent, then FX-EDGES-007's spin: the spin repeats the edge of its own input, the layer the Directional Blur grew two pixels on every side, whose outer columns have faded. | largest difference 1.4e-7 | yes |
| FX-EDGES-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EDGES-010 frame 0: Gaussian Blur with edges "wrap", which is not a way of treating edges. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-EDGES-010 frame 4: Gaussian Blur with edges "wrap", which is not a way of treating edges. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-EDGES-010: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EDGES-011 frame 0: Directional Blur with edges "Repeat": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-EDGES-011 frame 4: Directional Blur with edges "Repeat": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-EDGES-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EDGES-012 frame 0: Radial Blur with edges "", empty. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-EDGES-012 frame 4: Radial Blur with edges "", empty. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-EDGES-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| a Gaussian Blur of sigma 4 repeating its edges does not grow | 0 | yes |
| one leaving them transparent grows by its radius, as before | 12 | yes |
| a Directional Blur 20 long repeating its edges does not grow | 0 | yes |
| one leaving them transparent grows by half its length, as before | 10 | yes |
| a Radial Blur repeating its edges does not grow, as it never did | 0 | yes |
| a half-size draft preview halves the length and keeps the edges | DirectionalBlur { direction: 30.0, length: 10.0, edges: "repeat" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_edges_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_edges_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_edges_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_edges_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_edges_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_edges_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_edges_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_edges_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_edges_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_edges_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_edges_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_edges_002.json's written "transparent" is not written back, as a file from before D-109 has it | {"sigma_px":1} | yes |
| fx_edges_009.json's first blur, with no edges, is saved with none | {"direction":90,"length":4} | yes |
| a file with edges written as a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with edges written as keys is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_edges_001.json: GaussianBlur { sigma_px: 1.0, edges: "wrap", dimensions: "both", units: "sigma" } is refused with a sentence, and nothing changes | Gaussian Blur's edges are "transparent" or "repeat", and this is "wrap". | yes |
| fx_edges_001.json: GaussianBlur { sigma_px: 1.0, edges: "transparent", dimensions: "both", units: "sigma" } is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |
| fx_edges_004.json: DirectionalBlur { direction: 90.0, length: 6.0, edges: "Repeat" } is refused with a sentence, and nothing changes | Directional Blur's edges are "transparent" or "repeat", and this is "Repeat". | yes |
| fx_edges_004.json: DirectionalBlur { direction: 90.0, length: 6.0, edges: "transparent" } is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |
| fx_edges_007.json: RadialBlur { kind: "spin", amount: 30.0, center: [50.0, 50.0], edges: "" } is refused with a sentence, and nothing changes | Radial Blur's edges are "transparent" or "repeat", and this is "". | yes |
| fx_edges_007.json: RadialBlur { kind: "spin", amount: 30.0, center: [50.0, 50.0], edges: "transparent" } is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_edges_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_edges_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_edges_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

60 of 60 checks pass.
