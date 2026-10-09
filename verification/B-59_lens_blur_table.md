# B-59: lens blur

D-116, accepted by the owner on 2026-09-26 in the batch of ten. Every expected pixel is `Fixtures/lens_blur/expected_lens_blur.json`, written by `tools/lens_blur_reference.py` before this code existed and printed in document 25 as FX-LENS-001 to 018. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-LENS-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LENS-001 frame 0: The settings as they start, radius 10, edges transparent: every pixel is the mean of the 317 within ten pixels of it, so the line, the block and the red pixel spread into one faint even haze over the whole frame. | largest difference 1.3e-8 | yes |
| FX-LENS-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-002 frame 0: Radius 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-LENS-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-003 frame 0: Radius 0.9, under one pixel: each pixel takes itself alone, and the drawing is untouched. | largest difference 1.9e-7 | yes |
| FX-LENS-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-004 frame 0: Radius 1: each pixel is the mean of five, itself and the four beside, above and below it, a plus: the block's edge pixels lose a fifth of themselves for each empty neighbour, its middle stays skin, and the pixel diagonally off its corner stays empty. | largest difference 1.9e-7 | yes |
| FX-LENS-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-005 frame 0: Radius 1.5: the mean of the nine pixels of the 3 by 3 square about each pixel, so a diagonal neighbour of the block now takes a ninth of it. | largest difference 1.9e-7 | yes |
| FX-LENS-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-006 frame 0: Radius 2.5: the mean of 21 pixels, the 5 by 5 square without its four corners. | largest difference 1.8e-7 | yes |
| FX-LENS-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-007 frame 0: Radius 4: the mean of 49 pixels; the red pixel at half covering spreads into an even round patch nine pixels across, each pixel of it 1/49 of it, and the block into a rounded haze. | largest difference 8.3e-8 | yes |
| FX-LENS-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-008 frame 0: Radius 2.5, edges repeat, on a picture that fills the layer: every pixel stays fully covered, the edges as solid as the middle. | largest difference 1.1e-7 | yes |
| FX-LENS-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-009 frame 0: The same with edges transparent: the edges fade, the corners most, and the middle, more than 2.5 pixels from every edge, is FX-LENS-008's. | largest difference 1.1e-7 | yes |
| FX-LENS-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-010 frame 0: Radius keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 untouched, frame 2 at radius 2, the mean of 13, and frame 4 FX-LENS-007. | largest difference 1.9e-7 | yes |
| FX-LENS-010 frame 2: Radius keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 untouched, frame 2 at radius 2, the mean of 13, and frame 4 FX-LENS-007. | largest difference 2.0e-7 | yes |
| FX-LENS-010 frame 4: Radius keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 untouched, frame 2 at radius 2, the mean of 13, and frame 4 FX-LENS-007. | largest difference 8.3e-8 | yes |
| FX-LENS-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-011 frame 0: Radius eased from 1 at frame 0 to 0 at frame 4 on a curve that overshoots: at frame 2 the radius would be below 0, is held at 0, and the drawing is untouched; frame 0 is FX-LENS-004. | largest difference 1.9e-7 | yes |
| FX-LENS-011 frame 2: Radius eased from 1 at frame 0 to 0 at frame 4 on a curve that overshoots: at frame 2 the radius would be below 0, is held at 0, and the drawing is untouched; frame 0 is FX-LENS-004. | largest difference 1.9e-7 | yes |
| FX-LENS-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-012 frame 0: Radius 1.5, moved three pixels right: the layer grew two pixels on every side, so the line on the drawing's left edge spreads into the column left of it, which shows; nothing reaches the two columns before that. | largest difference 1.9e-7 | yes |
| FX-LENS-012 frame 3: Radius 1.5, moved three pixels right: the layer grew two pixels on every side, so the line on the drawing's left edge spreads into the column left of it, which shows; nothing reaches the two columns before that. | largest difference 1.9e-7 | yes |
| FX-LENS-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-013 frame 0: Radius 1.5, edges repeat, on the bars moved three pixels right: the layer does not grow, so the three columns left of it stay empty, and the line on its edge keeps more of itself than FX-LENS-012's. | largest difference 1.9e-7 | yes |
| FX-LENS-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-014 frame 0: Radius 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-014 frame 4: Radius 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-015 frame 0: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-015 frame 4: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-016 frame 0: Radius keyed to 250 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-016 frame 4: Radius keyed to 250 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-017 frame 0: Edges "wrap", which is not a way of treating edges. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-017 frame 4: Edges "wrap", which is not a way of treating edges. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-018 frame 0: Edges "Repeat": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-018 frame 4: Edges "Repeat": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| radius 10 grows the drawing by 10 on every side | 10 | yes |
| radius 2.5 grows it by 3, the radius rounded up | 3 | yes |
| radius 0.9 grows it by 1, though it changes no pixel | 1 | yes |
| radius 0 does not grow it | 0 | yes |
| radius 10 with the edge pixels repeated does not grow it | 0 | yes |
| a half-size draft preview halves the radius, and keeps the edges | LensBlur { radius: 5.0, edges: "repeat", iris: "circle", roundness: 0.0, rotation: 0.0, aspect: 1.0, highlight_gain: 0.0, highlight_threshold: 100.0, layer: String(""), fit: "center", channel: "luminance", focal_distance: 0.0, invert: "off", map: None } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lens_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `edges` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a radius written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| radius 201 is refused with a sentence, and nothing changes | Lens Blur's radius runs from 0 to 200, and this is 201. | yes |
| radius -1 is refused with a sentence, and nothing changes | Lens Blur's radius runs from 0 to 200, and this is -1. | yes |
| edges "wrap" is refused with a sentence, and nothing changes | Lens Blur's edges are "transparent" or "repeat", and this is "wrap". | yes |
| edges "Repeat" is refused with a sentence, and nothing changes | Lens Blur's edges are "transparent" or "repeat", and this is "Repeat". | yes |
| radius keyed to 250 is refused with a sentence, and nothing changes | Lens Blur's radius runs from 0 to 200, and this is 250. | yes |
| radius 200, the top, is taken | taken | yes |
| radius 0, edges repeat is taken | taken | yes |
| radius keyed from 0 to 4 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lens_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lens_010.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

74 of 74 checks pass.
