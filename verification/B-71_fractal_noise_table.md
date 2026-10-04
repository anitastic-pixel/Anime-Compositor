# B-71: fractal noise

D-128, accepted by the owner on 2026-09-26, the sixth of the second batch of ten. Every expected pixel is `Fixtures/fractal_noise/expected_fractal_noise.json`, written by `tools/fractal_noise_reference.py` before this code existed and printed in document 25 as FX-FRACTAL-001 to 028. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-FRACTAL-001 to 028 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FRACTAL-001 frame 0: The settings as they start: size 100, complexity 4, contrast 100, brightness 0, evolution 0, speed 0, seed 0, black to white, opacity 100, normal. The card is covered by grey clouds so large that the drawing sees a small part of one cell: close greys, near the middle. Every pixel keeps its covering, the empty ones stay empty, and with speed 0 frame 4 is frame 0. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-001 frame 4: The settings as they start: size 100, complexity 4, contrast 100, brightness 0, evolution 0, speed 0, seed 0, black to white, opacity 100, normal. The card is covered by grey clouds so large that the drawing sees a small part of one cell: close greys, near the middle. Every pixel keeps its covering, the empty ones stay empty, and with speed 0 frame 4 is frame 0. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-002 frame 0: Size 4: four pixels a cell, so the greys vary across the card, lighter and darker patches. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-003 frame 0: Size 4, complexity 1.9, which counts as 1: one octave, the smooth value noise itself, softer than FX-FRACTAL-002's four. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-004 frame 0: Size 4, complexity 8: eight octaves, finer detail on top of FX-FRACTAL-002's. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-005 frame 0: Size 4, contrast 0: every shown pixel is the middle of the ramp, encoded exactly 0.5, at its own covering. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-006 frame 0: Size 4, contrast 300: FX-FRACTAL-002's greys three times as far from the middle, held at black and white. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-007 frame 0: Size 4, brightness 30: FX-FRACTAL-002's value lifted by 0.3, held at white. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-008 frame 0: Brightness -100: the value is held at 0 everywhere, so every shown pixel is black at its own covering. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-009 frame 0: Size 4, speed 90 degrees a frame: the clouds change from frame to frame; frame 0 is FX-FRACTAL-002, frame 2 is evolution 180, and frame 4 is evolution 360, FX-FRACTAL-010. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-009 frame 2: Size 4, speed 90 degrees a frame: the clouds change from frame to frame; frame 0 is FX-FRACTAL-002, frame 2 is evolution 180, and frame 4 is evolution 360, FX-FRACTAL-010. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-009 frame 4: Size 4, speed 90 degrees a frame: the clouds change from frame to frame; frame 0 is FX-FRACTAL-002, frame 2 is evolution 180, and frame 4 is evolution 360, FX-FRACTAL-010. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-010 frame 0: Size 4, evolution 360: one full turn moves the field one cell through its third direction, clouds of their own, not FX-FRACTAL-002's. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-011 frame 0: Size 4, evolution keyed from 0 at frame 0 to 720 at frame 4, linear, speed 0: frame 0 is FX-FRACTAL-002 and frame 2, at 360, is FX-FRACTAL-010. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-011 frame 2: Size 4, evolution keyed from 0 at frame 0 to 720 at frame 4, linear, speed 0: frame 0 is FX-FRACTAL-002 and frame 2, at 360, is FX-FRACTAL-010. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-011 frame 4: Size 4, evolution keyed from 0 at frame 0 to 720 at frame 4, linear, speed 0: frame 0 is FX-FRACTAL-002 and frame 2, at 360, is FX-FRACTAL-010. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-012 frame 0: Size 4, seed 7: clouds of their own. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-013 frame 0: Size 4, seed 7.9, which counts as 7: FX-FRACTAL-012. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-014 frame 0: Size 4, dark #1e1a24 and light #fff0b0: the clouds run from the dark violet to the cream, mixed in encoded values. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-015 frame 0: FX-FRACTAL-014 with its colours written in capitals: the same. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-016 frame 0: Size 4, blend multiply: the card darkened by the clouds, its own colours showing through; the black patch stays black. | largest difference 5.3e-8 | yes |
| FX-FRACTAL-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-017 frame 0: Size 4, blend screen, opacity 50: the card lightened by the clouds, none darkened; the white patch stays white. | largest difference 2.1e-7 | yes |
| FX-FRACTAL-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-018 frame 0: Size 4, blend add, opacity 50: the clouds added at half strength, not held, so the white patch goes past its covering. | largest difference 2.3e-7 | yes |
| FX-FRACTAL-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-019 frame 0: Size 4, opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 0 is the drawing; frame 2 is held at 100 and is frame 4, FX-FRACTAL-002. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-019 frame 2: Size 4, opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 0 is the drawing; frame 2 is held at 100 and is frame 4, FX-FRACTAL-002. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-019 frame 4: Size 4, opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 0 is the drawing; frame 2 is held at 100 and is frame 4, FX-FRACTAL-002. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-020 frame 0: FX-FRACTAL-002 moved three pixels right: the clouds are worked in the drawing's own space, so they move with it. | largest difference 1.5e-8 | yes |
| FX-FRACTAL-020 frame 3: FX-FRACTAL-002 moved three pixels right: the clouds are worked in the drawing's own space, so they move with it. | largest difference 1.5e-8 | yes |
| FX-FRACTAL-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-021 frame 0: Size 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-021 frame 4: Size 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-022 frame 0: Complexity 9, above 8. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-022 frame 4: Complexity 9, above 8. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-023 frame 0: Contrast 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-023 frame 4: Contrast 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-024 frame 0: Brightness -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-024 frame 4: Brightness -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-025 frame 0: Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-025 frame 4: Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-026 frame 0: Speed keyed to 400 at frame 4, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-026 frame 4: Speed keyed to 400 at frame 4, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-027 frame 0: Blend "overlay", which is not a blend. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-027 frame 4: Blend "overlay", which is not a blend. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-028 frame 0: A dark colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-028 frame 4: A dark colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview halves the size | FractalNoise { size: 20.0, complexity: 4.0, contrast: 100.0, brightness: 0.0, evolution: 0.0, speed: 0.0, seed: 0.0, dark_color: "#000000", light_color: "#ffffff", opacity: 100.0, blend: "normal", fractal_type: "basic", noise_type: "smooth", invert: "off", offset: [0.0, 0.0], scale_width: 100.0, scale_height: 100.0, cycle: 0.0, frame: 0 } | yes |
| a half-size draft of size 1 holds the size at 1, its range's bottom, rather than leaving the effect out | FractalNoise { size: 1.0, complexity: 4.0, contrast: 100.0, brightness: 0.0, evolution: 0.0, speed: 0.0, seed: 0.0, dark_color: "#000000", light_color: "#ffffff", opacity: 100.0, blend: "normal", fractal_type: "basic", noise_type: "smooth", invert: "off", offset: [0.0, 0.0], scale_width: 100.0, scale_height: 100.0, cycle: 0.0, frame: 0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fractal_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `light_color` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a contrast that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| size 0.5 is refused with a sentence, and nothing changes | Fractal Noise's size runs from 1 to 1000, and this is 0.5. | yes |
| complexity 9 is refused with a sentence, and nothing changes | Fractal Noise's complexity runs from 1 to 8, and this is 9. | yes |
| contrast 1001 is refused with a sentence, and nothing changes | Fractal Noise's contrast runs from 0 to 1000, and this is 1001. | yes |
| brightness -101 is refused with a sentence, and nothing changes | Fractal Noise's brightness runs from -100 to 100, and this is -101. | yes |
| evolution 100001 is refused with a sentence, and nothing changes | Fractal Noise's evolution runs from -100000 to 100000, and this is 100001. | yes |
| speed 361 is refused with a sentence, and nothing changes | Fractal Noise's speed runs from -360 to 360, and this is 361. | yes |
| seed -1 is refused with a sentence, and nothing changes | Fractal Noise's seed runs from 0 to 100000, and this is -1. | yes |
| opacity 101 is refused with a sentence, and nothing changes | Fractal Noise's opacity runs from 0 to 100, and this is 101. | yes |
| dark colour "#12345" is refused with a sentence, and nothing changes | Fractal Noise's dark colour is written #rrggbb, and this is "#12345". | yes |
| light colour "white" is refused with a sentence, and nothing changes | Fractal Noise's light colour is written #rrggbb, and this is "white". | yes |
| blend "overlay" is refused with a sentence, and nothing changes | Fractal Noise's blend is "normal", "multiply", "screen" or "add", and this is "overlay". | yes |
| speed keyed to 400 is refused with a sentence, and nothing changes | Fractal Noise's speed runs from -360 to 360, and this is 400. | yes |
| every number at the top of its range, is taken | taken | yes |
| every number at the bottom of its range, is taken | taken | yes |
| evolution keyed from 0 to 720 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fractal_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fractal_009.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

107 of 107 checks pass.
