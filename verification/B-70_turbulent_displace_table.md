# B-70: turbulent displace

D-127, accepted by the owner on 2026-09-26, the fifth of the second batch of ten. Every expected pixel is `Fixtures/turbulent_displace/expected_turbulent_displace.json`, written by `tools/turbulent_displace_reference.py` before this code existed and printed in document 25 as FX-TURB-001 to 026. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-TURB-001 to 026 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TURB-001 frame 0: The settings as they start: amount 10, size 60, complexity 2, evolution 0, speed 20, seed 0, edges transparent. A wave is wider than the drawing, so each pixel reads from two to four pixels left of it and two to five below, not quite alike everywhere: the drawing is carried right and up and its stripes lean and bend gently; the warp moves a little from frame 0 to 2 to 4. The first three columns read from past the drawing's left edge and are empty on every frame. | largest difference 2.1e-7 | yes |
| FX-TURB-001 frame 2: The settings as they start: amount 10, size 60, complexity 2, evolution 0, speed 20, seed 0, edges transparent. A wave is wider than the drawing, so each pixel reads from two to four pixels left of it and two to five below, not quite alike everywhere: the drawing is carried right and up and its stripes lean and bend gently; the warp moves a little from frame 0 to 2 to 4. The first three columns read from past the drawing's left edge and are empty on every frame. | largest difference 2.5e-7 | yes |
| FX-TURB-001 frame 4: The settings as they start: amount 10, size 60, complexity 2, evolution 0, speed 20, seed 0, edges transparent. A wave is wider than the drawing, so each pixel reads from two to four pixels left of it and two to five below, not quite alike everywhere: the drawing is carried right and up and its stripes lean and bend gently; the warp moves a little from frame 0 to 2 to 4. The first three columns read from past the drawing's left edge and are empty on every frame. | largest difference 2.5e-7 | yes |
| FX-TURB-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-002 frame 0: Amount 0: the drawing, untouched, and nothing grows. | largest difference 1.9e-7 | yes |
| FX-TURB-002 frame 2: Amount 0: the drawing, untouched, and nothing grows. | largest difference 1.9e-7 | yes |
| FX-TURB-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-003 frame 0: Amount 3, size 8: a wave every eight pixels, so the stripes bend into waves and the blue band wobbles; frame 4 is warped differently from frame 0. | largest difference 2.5e-7 | yes |
| FX-TURB-003 frame 4: Amount 3, size 8: a wave every eight pixels, so the stripes bend into waves and the blue band wobbles; frame 4 is warped differently from frame 0. | largest difference 2.5e-7 | yes |
| FX-TURB-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-004 frame 0: Amount 3, size 8, speed 0: the warp holds still, the same on frames 0, 2 and 4, and is FX-TURB-003's frame 0. | largest difference 2.5e-7 | yes |
| FX-TURB-004 frame 2: Amount 3, size 8, speed 0: the warp holds still, the same on frames 0, 2 and 4, and is FX-TURB-003's frame 0. | largest difference 2.5e-7 | yes |
| FX-TURB-004 frame 4: Amount 3, size 8, speed 0: the warp holds still, the same on frames 0, 2 and 4, and is FX-TURB-003's frame 0. | largest difference 2.5e-7 | yes |
| FX-TURB-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-005 frame 0: FX-TURB-004 with edges repeat: the layer does not grow, and a push past the drawing's edge reads the nearest edge pixel, so where FX-TURB-004 reads emptiness at the left edge this reads the stripe there. | largest difference 2.5e-7 | yes |
| FX-TURB-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-006 frame 0: Complexity 1, amount 3, size 8, speed 0: one octave, a smoother warp than FX-TURB-004's two. | largest difference 2.5e-7 | yes |
| FX-TURB-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-007 frame 0: Complexity 8, amount 3, size 8, speed 0: eight octaves, a rougher warp. | largest difference 2.5e-7 | yes |
| FX-TURB-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-008 frame 0: Size 1, amount 3, speed 0: a wave every pixel, so the drawing breaks up into a jumble. | largest difference 2.5e-7 | yes |
| FX-TURB-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-009 frame 0: Size 1000, amount 3, speed 0: the wave is so wide that every pixel is pushed alike, within a hundredth of a pixel: the drawing slides whole. | largest difference 2.5e-7 | yes |
| FX-TURB-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-010 frame 0: Amount 3, size 8, speed 90: frame 0 is FX-TURB-004; frame 2 is evolution 180, and frame 4 is evolution 360, FX-TURB-011. | largest difference 2.5e-7 | yes |
| FX-TURB-010 frame 2: Amount 3, size 8, speed 90: frame 0 is FX-TURB-004; frame 2 is evolution 180, and frame 4 is evolution 360, FX-TURB-011. | largest difference 2.5e-7 | yes |
| FX-TURB-010 frame 4: Amount 3, size 8, speed 90: frame 0 is FX-TURB-004; frame 2 is evolution 180, and frame 4 is evolution 360, FX-TURB-011. | largest difference 2.5e-7 | yes |
| FX-TURB-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-011 frame 0: Amount 3, size 8, evolution 360, speed 0: one full turn moves the field one cell through its third direction, a warp of its own. | largest difference 2.5e-7 | yes |
| FX-TURB-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-012 frame 0: Amount 3, size 8, speed 0, seed 8: a warp of its own. | largest difference 2.5e-7 | yes |
| FX-TURB-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-013 frame 0: Seed 8.5, which counts as 8: FX-TURB-012. | largest difference 2.5e-7 | yes |
| FX-TURB-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-014 frame 0: Complexity 2.7, which counts as 2: FX-TURB-004. | largest difference 2.5e-7 | yes |
| FX-TURB-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-015 frame 0: Size 8, speed 0, amount keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 2 is amount 2, and frame 4 amount 4. | largest difference 1.9e-7 | yes |
| FX-TURB-015 frame 2: Size 8, speed 0, amount keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 2 is amount 2, and frame 4 amount 4. | largest difference 2.5e-7 | yes |
| FX-TURB-015 frame 4: Size 8, speed 0, amount keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 2 is amount 2, and frame 4 amount 4. | largest difference 2.5e-7 | yes |
| FX-TURB-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-016 frame 0: Amount 3, size 8, speed keyed from 0 at frame 0 to 360 at frame 4, eased past its end (477 at frame 2): frame 0 is FX-TURB-004, and frame 2 is held at 360, so it is speed 360's frame 2. | largest difference 2.5e-7 | yes |
| FX-TURB-016 frame 2: Amount 3, size 8, speed keyed from 0 at frame 0 to 360 at frame 4, eased past its end (477 at frame 2): frame 0 is FX-TURB-004, and frame 2 is held at 360, so it is speed 360's frame 2. | largest difference 2.5e-7 | yes |
| FX-TURB-016 frame 4: Amount 3, size 8, speed keyed from 0 at frame 0 to 360 at frame 4, eased past its end (477 at frame 2): frame 0 is FX-TURB-004, and frame 2 is held at 360, so it is speed 360's frame 2. | largest difference 2.5e-7 | yes |
| FX-TURB-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-017 frame 0: FX-TURB-012 moved three pixels right: the warp is worked in the drawing's own space, so it moves with it, and the three columns left of the drawing show the grown pixels, into which the stripes are pushed. | largest difference 2.5e-7 | yes |
| FX-TURB-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-018 frame 0: FX-TURB-005, edges repeat, moved three pixels right: nothing grows, so the three columns left of the drawing stay empty. | largest difference 2.5e-7 | yes |
| FX-TURB-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-019 frame 0: Amount 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-019 frame 4: Amount 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURB-020 frame 0: Size 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-020 frame 4: Size 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURB-021 frame 0: Complexity 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-021 frame 4: Complexity 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURB-022 frame 0: Speed -361, below -360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-022 frame 4: Speed -361, below -360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURB-023 frame 0: Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-023 frame 4: Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURB-024 frame 0: Amount keyed to 1500 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-024 frame 4: Amount keyed to 1500 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURB-025 frame 0: Edges "wrap", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-025 frame 4: Edges "wrap", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURB-026 frame 0: Edges "Repeat": the word is exact, so a capital is not the choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-026 frame 4: Edges "Repeat": the word is exact, so a capital is not the choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| with transparent edges, amount 2.5 grows the drawing's bounds by 3 pixels | 3 | yes |
| with repeat edges it grows them by nothing | 0 | yes |
| at amount 0 it grows them by nothing | 0 | yes |
| a half-size draft preview halves the amount and the size | TurbulentDisplace { amount: 10.0, size: 30.0, complexity: 2.0, evolution: 0.0, speed: 20.0, seed: 0.0, edges: "transparent", frame: 0, displacement: "turbulent", pinning: "none" } | yes |
| a half-size draft of size 1 holds the size at 1, its range's bottom, rather than leaving the effect out | TurbulentDisplace { amount: 5.0, size: 1.0, complexity: 2.0, evolution: 0.0, speed: 20.0, seed: 0.0, edges: "transparent", frame: 0, displacement: "turbulent", pinning: "none" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_turb_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `edges` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a speed that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amount 1001 is refused with a sentence, and nothing changes | Turbulent Displace's amount runs from 0 to 1000, and this is 1001. | yes |
| size 0.5 is refused with a sentence, and nothing changes | Turbulent Displace's size runs from 1 to 1000, and this is 0.5. | yes |
| complexity 9 is refused with a sentence, and nothing changes | Turbulent Displace's complexity runs from 1 to 8, and this is 9. | yes |
| evolution 100001 is refused with a sentence, and nothing changes | Turbulent Displace's evolution runs from -100000 to 100000, and this is 100001. | yes |
| speed -361 is refused with a sentence, and nothing changes | Turbulent Displace's speed runs from -360 to 360, and this is -361. | yes |
| seed 100001 is refused with a sentence, and nothing changes | Turbulent Displace's seed runs from 0 to 100000, and this is 100001. | yes |
| edges "wrap" is refused with a sentence, and nothing changes | Turbulent Displace's edges are "transparent" or "repeat", and this is "wrap". | yes |
| amount keyed to 1500 is refused with a sentence, and nothing changes | Turbulent Displace's amount runs from 0 to 1000, and this is 1500. | yes |
| amount 1000, size 1000, complexity 8, evolution 100000, speed 360 and seed 100000, the tops, is taken | taken | yes |
| amount 0, size 1, complexity 1, evolution -100000, speed -360 and seed 0, the bottoms, is taken | taken | yes |
| speed keyed from 20 to -20 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_turb_003.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turb_018.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

106 of 106 checks pass.
