# B-64: lens blur iris and highlights

D-121, proposed on 2026-09-26. Every expected pixel is `Fixtures/lens_blur/expected_lens_blur.json`, written by `tools/lens_blur_reference.py` before this code existed and printed in document 25 as FX-LENS-019 to 044. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5. FX-LENS-001 to 018, the round iris of D-116, are walked by B-59 and must not move.

## FX-LENS-019 to 044 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LENS-019 frame 0: Iris triangle, radius 4: the iris holds 25 steps, a triangle standing on its side with its point up; the red pixel spreads into that triangle, each pixel of it 1/25 of the red, its point four pixels above the red pixel and its base two below. | largest difference 1.4e-7 | yes |
| FX-LENS-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-020 frame 0: Iris square, radius 3: the square standing on a side, its corners three pixels from the middle, so the 5 by 5 square of 25 steps. | largest difference 1.6e-7 | yes |
| FX-LENS-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-021 frame 0: Iris hexagon, radius 4: 43 steps, flat along the top and bottom, pointed at the sides. | largest difference 9.0e-8 | yes |
| FX-LENS-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-022 frame 0: Iris decagon, radius 4: 47 steps, nearly FX-LENS-007's circle of 49. | largest difference 8.4e-8 | yes |
| FX-LENS-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-023 frame 0: Iris triangle, radius 4, rotation 180: FX-LENS-019's triangle upside down, its point four pixels below the red pixel. | largest difference 1.4e-7 | yes |
| FX-LENS-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-024 frame 0: Iris square, radius 3, rotation 45: the square turned onto its corner, a diamond of 25 steps reaching three pixels up, down, left and right. | largest difference 1.3e-7 | yes |
| FX-LENS-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-025 frame 0: Iris triangle, radius 4, roundness 100: all round, FX-LENS-007 exactly. | largest difference 8.3e-8 | yes |
| FX-LENS-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-026 frame 0: Iris square, radius 4, roundness 50: halfway between the square, 25 steps at this radius, and FX-LENS-007's circle of 49: 45 steps, the square's sides bowed out to three pixels from the middle, and the circle's four furthest steps, four pixels straight out, not reached. | largest difference 9.6e-8 | yes |
| FX-LENS-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-027 frame 0: Round iris, radius 3, aspect 2: an oval twice as wide as it is tall, 29 steps, reaching four pixels left and right and two up and down. | largest difference 1.3e-7 | yes |
| FX-LENS-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-028 frame 0: Round iris, radius 3, aspect 0.5, rotation 90: an oval twice as tall as wide, turned a quarter, FX-LENS-027 exactly. | largest difference 1.3e-7 | yes |
| FX-LENS-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-029 frame 0: Radius 4, highlight gain 3, threshold 80: the skin, its brightest channel 0.92, is lit to four times before the average, so the block's haze is brighter and near it held at white; the red, at 0.58, is under the threshold, and its patch where the block does not reach is FX-LENS-007's. | largest difference 4.7e-8 | yes |
| FX-LENS-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-030 frame 0: Radius 4, highlight gain 0.5, threshold 50: the red is lit too, to one and a half times, and its patch where the block does not reach is one and a half times FX-LENS-007's in colour, its covering the same. | largest difference 7.7e-8 | yes |
| FX-LENS-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-031 frame 0: Iris triangle, radius 4, rotation keyed from 0 at frame 0 to 60 at frame 4, linear: frame 0 is FX-LENS-019, frame 2 is turned 30, and frame 4, turned 60, stands on its point, FX-LENS-023. | largest difference 1.4e-7 | yes |
| FX-LENS-031 frame 2: Iris triangle, radius 4, rotation keyed from 0 at frame 0 to 60 at frame 4, linear: frame 0 is FX-LENS-019, frame 2 is turned 30, and frame 4, turned 60, stands on its point, FX-LENS-023. | largest difference 1.4e-7 | yes |
| FX-LENS-031 frame 4: Iris triangle, radius 4, rotation keyed from 0 at frame 0 to 60 at frame 4, linear: frame 0 is FX-LENS-019, frame 2 is turned 30, and frame 4, turned 60, stands on its point, FX-LENS-023. | largest difference 1.4e-7 | yes |
| FX-LENS-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-032 frame 0: Radius 4, threshold 80, highlight gain keyed from 0 at frame 0 to 3 at frame 4, linear: frame 0 is FX-LENS-007, frame 4 FX-LENS-029. | largest difference 8.3e-8 | yes |
| FX-LENS-032 frame 2: Radius 4, threshold 80, highlight gain keyed from 0 at frame 0 to 3 at frame 4, linear: frame 0 is FX-LENS-007, frame 4 FX-LENS-029. | largest difference 6.0e-8 | yes |
| FX-LENS-032 frame 4: Radius 4, threshold 80, highlight gain keyed from 0 at frame 0 to 3 at frame 4, linear: frame 0 is FX-LENS-007, frame 4 FX-LENS-029. | largest difference 4.7e-8 | yes |
| FX-LENS-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-033 frame 0: Iris hexagon, radius 2.5, edges repeat, on the picture that fills the layer: every pixel stays fully covered. | largest difference 1.4e-7 | yes |
| FX-LENS-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-034 frame 0: Round iris, radius 1.5, aspect 2, moved three pixels right: the iris is a line of five across with one above and one below the middle; the layer grew three pixels, 1.5 times the square root of 2 rounded up, so the line on the drawing's left edge spreads into the two columns left of it, and the column before those stays empty. | largest difference 1.9e-7 | yes |
| FX-LENS-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-035 frame 0: Every D-121 setting written at its start value: circle, roundness 0, rotation 0, aspect 1, highlight gain 0, threshold 100; FX-LENS-001 exactly. | largest difference 1.3e-8 | yes |
| FX-LENS-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-036 frame 0: Iris decagon, radius 0.9, turned 17 and stretched to aspect 3: under one pixel each pixel takes itself alone, and the drawing is untouched. | largest difference 1.9e-7 | yes |
| FX-LENS-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-037 frame 0: Iris "star", which is not a shape of iris. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-037 frame 4: Iris "star", which is not a shape of iris. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-038 frame 0: Iris "Hexagon": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-038 frame 4: Iris "Hexagon": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-039 frame 0: Roundness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-039 frame 4: Roundness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-040 frame 0: Rotation 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-040 frame 4: Rotation 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-041 frame 0: Aspect 0.05, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-041 frame 4: Aspect 0.05, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-042 frame 0: Aspect keyed to 20 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-042 frame 4: Aspect keyed to 20 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-043 frame 0: Highlight gain -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-043 frame 4: Highlight gain -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-043: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-044 frame 0: Highlight threshold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-044 frame 4: Highlight threshold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-044: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| radius 10, aspect 1, grows the drawing by 10 on every side, as before | 10 | yes |
| radius 1.5, aspect 2, grows it by 3: the radius times the square root of 2, rounded up | 3 | yes |
| radius 1.5, aspect 0.5, grows it by 3 as well | 3 | yes |
| radius 10, aspect 2, with the edge pixels repeated does not grow it | 0 | yes |
| a half-size draft preview halves the radius, and keeps the iris, its turn, its stretch and the highlights | LensBlur { radius: 5.0, edges: "repeat", iris: "hexagon", roundness: 50.0, rotation: 45.0, aspect: 2.0, highlight_gain: 3.0, highlight_threshold: 80.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lens_001.json, from before D-121, is saved without any of the new settings | {"edges":"transparent","radius":10} | yes |
| fx_lens_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with an iris written as a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a rotation written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| iris "star" is refused with a sentence, and nothing changes | Lens Blur's iris is "circle", "triangle", "square", "pentagon", "hexagon", "heptagon", "octagon", "nonagon" or "decagon", and this is "star". | yes |
| iris "Hexagon" is refused with a sentence, and nothing changes | Lens Blur's iris is "circle", "triangle", "square", "pentagon", "hexagon", "heptagon", "octagon", "nonagon" or "decagon", and this is "Hexagon". | yes |
| roundness 101 is refused with a sentence, and nothing changes | Lens Blur's roundness runs from 0 to 100, and this is 101. | yes |
| rotation 3601 is refused with a sentence, and nothing changes | Lens Blur's rotation runs from -3600 to 3600, and this is 3601. | yes |
| aspect 0.05 is refused with a sentence, and nothing changes | Lens Blur's aspect runs from 0.1 to 10, and this is 0.05. | yes |
| highlight gain -1 is refused with a sentence, and nothing changes | Lens Blur's highlight gain runs from 0 to 100, and this is -1. | yes |
| highlight threshold 101 is refused with a sentence, and nothing changes | Lens Blur's highlight threshold runs from 0 to 100, and this is 101. | yes |
| aspect keyed to 20 is refused with a sentence, and nothing changes | Lens Blur's aspect runs from 0.1 to 10, and this is 20. | yes |
| a hexagon, roundness 20, turned 15 degrees, aspect 1.5, gain 3 over 90 is taken | taken | yes |
| the far ends: decagon, roundness 100, rotation -3600, aspect 10, gain 100 over 0 is taken | taken | yes |
| rotation keyed from 0 to 3600 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lens_021.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lens_028.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lens_030.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lens_033.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lens_034.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

101 of 101 checks pass.
