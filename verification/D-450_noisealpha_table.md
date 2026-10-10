# D-450: Noise Alpha

B-330, after After Effects' Noise Alpha: a noise of one number a pixel, from -1 to 1, Squared pushing it towards its ends, times Amount, laid on the layer's covering (alpha) where Original Alpha says (Clamp: only fully covered pixels; Add: everywhere; Scale: in proportion to the covering; Edges: only the partly covered ones), a covering pushed out of 0 to 1 held, reflected or wrapped by Overflow. The Random kinds take Random Seed and stay still; the Animation kinds move through the noise a whole field a turn of Noise Phase, coming back every Cycle turns when Cycle Noise is on. The colour is kept; a pixel that had no covering gains black. Every expected pixel is `Fixtures/noisealpha/expected_noisealpha.json`, written by `tools/noisealpha_reference.py` before this code existed and printed in document 25 as FX-NOISEALPHA-001 to 031. Tolerance 2e-5.

## FX-NOISEALPHA-001 to 031 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-NOISEALPHA-001 frame 0: The settings as they start: Uniform Random, amount 20, Clamp, Clip; only the fully covered pixels change, each losing up to 0.2 of its covering or, where the noise would add, staying covered; the empty pixels and the soft edge are left; the colour is kept; the same on every frame. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-001 frame 2: The settings as they start: Uniform Random, amount 20, Clamp, Clip; only the fully covered pixels change, each losing up to 0.2 of its covering or, where the noise would add, staying covered; the empty pixels and the soft edge are left; the colour is kept; the same on every frame. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-002 frame 0: Add: the empty pixels gain up to 0.2 of covering, in black, the soft edge moves up to 0.2 either way keeping its colour, and the covered pixels lose up to 0.2. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-003 frame 0: Scale: the noise in proportion to the covering, so the empty pixels are left and the soft edge moves half as far as the step. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-004 frame 0: Edges: only the soft edge, partly covered, changes. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-005 frame 0: Squared Random, Add: the noise pushed out towards its ends, so the soft edge moves at least as far as in FX-NOISEALPHA-002, the same way. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-006 frame 0: Amount 100, Add, Wrap Back: a covered pixel the noise would push past full is reflected back as far, an empty one pushed below nothing comes back up as far. | largest difference 2.4e-7 | yes |
| FX-NOISEALPHA-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-007 frame 0: Amount 100, Add, Wrap: past full comes round from nothing, below nothing comes round from full. | largest difference 2.4e-7 | yes |
| FX-NOISEALPHA-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-008 frame 0: Amount 100, Add, Clip: held at nothing and full. | largest difference 2.4e-7 | yes |
| FX-NOISEALPHA-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-009 frame 0: Uniform Animation at phase 0, Add: the noise at depth 0 for seed 0, so the frame is FX-NOISEALPHA-002's. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-010 frame 0: Uniform Animation at phase 180, Add: half way between depth 0 and depth 1 in the noise. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-011 frame 0: Uniform Animation, Noise Phase keyed from 0 at frame 0 to 720 at frame 4, Add: a new field each turn, reached smoothly; frame 1 is phase 180, FX-NOISEALPHA-010's. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-011 frame 1: Uniform Animation, Noise Phase keyed from 0 at frame 0 to 720 at frame 4, Add: a new field each turn, reached smoothly; frame 1 is phase 180, FX-NOISEALPHA-010's. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-011 frame 2: Uniform Animation, Noise Phase keyed from 0 at frame 0 to 720 at frame 4, Add: a new field each turn, reached smoothly; frame 1 is phase 180, FX-NOISEALPHA-010's. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-011 frame 4: Uniform Animation, Noise Phase keyed from 0 at frame 0 to 720 at frame 4, Add: a new field each turn, reached smoothly; frame 1 is phase 180, FX-NOISEALPHA-010's. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-012 frame 0: FX-NOISEALPHA-011 with Cycle Noise on and Cycle 2: after two turns the noise is where it began, so frame 4 is frame 0. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-012 frame 2: FX-NOISEALPHA-011 with Cycle Noise on and Cycle 2: after two turns the noise is where it began, so frame 4 is frame 0. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-012 frame 4: FX-NOISEALPHA-011 with Cycle Noise on and Cycle 2: after two turns the noise is where it began, so frame 4 is frame 0. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-013 frame 0: Cycle 1: every whole turn is the start, so phase 360 is phase 0. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-014 frame 0: Squared Animation at phase 90, Add. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-015 frame 0: Random Seed 7, Add: a different noise from FX-NOISEALPHA-002's. | largest difference 2.1e-7 | yes |
| FX-NOISEALPHA-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-016 frame 0: Random Seed 7.6, Add: its whole part counts, so this is FX-NOISEALPHA-015. | largest difference 2.1e-7 | yes |
| FX-NOISEALPHA-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-017 frame 0: Uniform Animation with Random Seed 7: the seed is for the Random kinds only, so this is FX-NOISEALPHA-009. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-018 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-018 frame 2: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-019 frame 0: FX-NOISEALPHA-002 moved three pixels right: the noise is the drawing's own, so it moves with it. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-020 frame 0: After a Motion Tile that grows the layer: the noise is worked in the drawing's own pixels, so the frame is FX-NOISEALPHA-002's. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-021 frame 0: Amount keyed from 0 at frame 0 to 40 at frame 4, Add: frame 0 is the drawing; on the soft edge frame 4's step is twice frame 2's. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-021 frame 2: Amount keyed from 0 at frame 0 to 40 at frame 4, Add: frame 0 is the drawing; on the soft edge frame 4's step is twice frame 2's. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-021 frame 4: Amount keyed from 0 at frame 0 to 40 at frame 4, Add: frame 0 is the drawing; on the soft edge frame 4's step is twice frame 2's. | largest difference 2.4e-7 | yes |
| FX-NOISEALPHA-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-022 frame 0: Squared Animation, amount 60, Scale, Wrap Back, Cycle Noise on with Cycle 3, phase keyed 0 to 400: the controls together. | largest difference 2.0e-7 | yes |
| FX-NOISEALPHA-022 frame 1: Squared Animation, amount 60, Scale, Wrap Back, Cycle Noise on with Cycle 3, phase keyed 0 to 400: the controls together. | largest difference 2.3e-7 | yes |
| FX-NOISEALPHA-022 frame 2: Squared Animation, amount 60, Scale, Wrap Back, Cycle Noise on with Cycle 3, phase keyed 0 to 400: the controls together. | largest difference 2.3e-7 | yes |
| FX-NOISEALPHA-022 frame 3: Squared Animation, amount 60, Scale, Wrap Back, Cycle Noise on with Cycle 3, phase keyed 0 to 400: the controls together. | largest difference 2.3e-7 | yes |
| FX-NOISEALPHA-022 frame 4: Squared Animation, amount 60, Scale, Wrap Back, Cycle Noise on with Cycle 3, phase keyed 0 to 400: the controls together. | largest difference 2.4e-7 | yes |
| FX-NOISEALPHA-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NOISEALPHA-023 frame 0: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-023 frame 4: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEALPHA-024 frame 0: Random Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-024 frame 4: Random Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEALPHA-025 frame 0: Noise Phase 200000, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-025 frame 4: Noise Phase 200000, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEALPHA-026 frame 0: Cycle 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-026 frame 4: Cycle 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEALPHA-027 frame 0: Noise "uniform", not one of its four words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-027 frame 4: Noise "uniform", not one of its four words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEALPHA-028 frame 0: Original Alpha "Add", in capitals, kept as written and not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-028 frame 4: Original Alpha "Add", in capitals, kept as written and not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEALPHA-029 frame 0: Overflow "wrapback", not one of its three words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-029 frame 4: Overflow "wrapback", not one of its three words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEALPHA-030 frame 0: Cycle Noise "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-030 frame 4: Cycle Noise "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NOISEALPHA-031 frame 0: Amount keyed to 150 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-031 frame 4: Amount keyed to 150 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NOISEALPHA-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_noisealpha_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_noisealpha_022.json is saved with its words and numbers as written, and Noise Phase's keys kept | {"amount":60,"cycle":3,"cycle_noise":"on","noise":"squared_animation","noise_phase":{"base":0,"keyframes":[{"frame":0,"interp":"linear","value":0},{"frame":4,"interp":"linear","value":400}]},"original_alpha":"scale","overflow":"wrap_back","random_seed":0} | yes |
| fx_noisealpha_023.json is refused in a sentence | Noise Alpha's amount runs from 0 to 100, and this is 101. | yes |
| fx_noisealpha_024.json is refused in a sentence | Noise Alpha's random seed runs from 0 to 100000, and this is 100001. | yes |
| fx_noisealpha_025.json is refused in a sentence | Noise Alpha's noise phase runs from -100000 to 100000, and this is 200000. | yes |
| fx_noisealpha_026.json is refused in a sentence | Noise Alpha's cycle runs from 1 to 1000, and this is 0.5. | yes |
| fx_noisealpha_027.json is refused in a sentence | Noise Alpha's noise is one of uniform_random, squared_random, uniform_animation, squared_animation, and this is "uniform". | yes |
| fx_noisealpha_028.json is refused in a sentence | Noise Alpha's original alpha is one of clamp, add, scale, edges, and this is "Add". | yes |
| fx_noisealpha_029.json is refused in a sentence | Noise Alpha's overflow is one of clip, wrap_back, wrap, and this is "wrapback". | yes |
| fx_noisealpha_030.json is refused in a sentence | Noise Alpha's cycle noise is "off" or "on", and this is "yes". | yes |
| a file with a Noise Alpha with no `noise` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Noise Alpha whose cycle noise is false is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Noise Alpha whose amount is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amount 101 is refused with a sentence, and nothing changes | Noise Alpha's amount runs from 0 to 100, and this is 101. | yes |
| random seed -1 is refused with a sentence, and nothing changes | Noise Alpha's random seed runs from 0 to 100000, and this is -1. | yes |
| cycle 1001 is refused with a sentence, and nothing changes | Noise Alpha's cycle runs from 1 to 1000, and this is 1001. | yes |
| noise "squared" is refused with a sentence, and nothing changes | Noise Alpha's noise is one of uniform_random, squared_random, uniform_animation, squared_animation, and this is "squared". | yes |
| original alpha "Edges" is refused with a sentence, and nothing changes | Noise Alpha's original alpha is one of clamp, add, scale, edges, and this is "Edges". | yes |
| overflow "wrap back" is refused with a sentence, and nothing changes | Noise Alpha's overflow is one of clip, wrap_back, wrap, and this is "wrap back". | yes |
| cycle noise "On" is refused with a sentence, and nothing changes | Noise Alpha's cycle noise is "off" or "on", and this is "On". | yes |
| amount keyed to 150 is refused with a sentence, and nothing changes | Noise Alpha's amount runs from 0 to 100, and this is 150. | yes |
| amount 45, seed 9, phase 120, cycle 3, Squared Animation, Edges, Wrap Back, cycle noise on is taken | taken | yes |
| amount keyed from 0 to 40 is taken | taken | yes |
| noise phase keyed from 0 to 720 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_noisealpha_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noisealpha_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noisealpha_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noisealpha_011.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noisealpha_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noisealpha_020.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_noisealpha_022.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_noisealpha_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_noisealpha_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Noise Alpha as added (Uniform Random, amount 20, Clamp, Clip): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1024288 pixels changed | yes |
| the reference shot, Noise Alpha as added (Uniform Random, amount 20, Clamp, Clip) on three layers, frame 0, Full | largest difference 1 of 255, 2516 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha as added (Uniform Random, amount 20, Clamp, Clip) on three layers, frame 100, Full | largest difference 1 of 255, 1261 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha as added (Uniform Random, amount 20, Clamp, Clip) on three layers, frame 239, Full | largest difference 1 of 255, 1470 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha as added (Uniform Random, amount 20, Clamp, Clip) on three layers, frame 0, Draft | largest difference 1 of 255, 239 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha as added (Uniform Random, amount 20, Clamp, Clip) on three layers, frame 100, Draft | largest difference 1 of 255, 184 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha as added (Uniform Random, amount 20, Clamp, Clip) on three layers, frame 239, Draft | largest difference 1 of 255, 209 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha Squared Random, amount 60, Add, Wrap Back, seed 42: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2070095 pixels changed | yes |
| the reference shot, Noise Alpha Squared Random, amount 60, Add, Wrap Back, seed 42 on three layers, frame 0, Full | largest difference 1 of 255, 1487 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha Squared Random, amount 60, Add, Wrap Back, seed 42 on three layers, frame 100, Full | largest difference 1 of 255, 1540 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha Squared Random, amount 60, Add, Wrap Back, seed 42 on three layers, frame 239, Full | largest difference 1 of 255, 1512 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha Squared Random, amount 60, Add, Wrap Back, seed 42 on three layers, frame 0, Draft | largest difference 1 of 255, 78 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha Squared Random, amount 60, Add, Wrap Back, seed 42 on three layers, frame 100, Draft | largest difference 1 of 255, 88 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha Squared Random, amount 60, Add, Wrap Back, seed 42 on three layers, frame 239, Draft | largest difference 1 of 255, 90 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha Uniform Animation, amount 80, Scale, Wrap, phase 200: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2069058 pixels changed | yes |
| the reference shot, Noise Alpha Uniform Animation, amount 80, Scale, Wrap, phase 200 on three layers, frame 0, Full | largest difference 1 of 255, 1556 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha Uniform Animation, amount 80, Scale, Wrap, phase 200 on three layers, frame 100, Full | largest difference 1 of 255, 1502 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha Uniform Animation, amount 80, Scale, Wrap, phase 200 on three layers, frame 239, Full | largest difference 1 of 255, 1497 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha Uniform Animation, amount 80, Scale, Wrap, phase 200 on three layers, frame 0, Draft | largest difference 1 of 255, 364 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha Uniform Animation, amount 80, Scale, Wrap, phase 200 on three layers, frame 100, Draft | largest difference 1 of 255, 335 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha Uniform Animation, amount 80, Scale, Wrap, phase 200 on three layers, frame 239, Draft | largest difference 1 of 255, 382 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha Squared Animation, Edges, amount keyed 5 to 100 and phase keyed 0 to 3600, cycle noise on, cycle 4: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 37076 pixels changed | yes |
| the reference shot, Noise Alpha Squared Animation, Edges, amount keyed 5 to 100 and phase keyed 0 to 3600, cycle noise on, cycle 4 on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha Squared Animation, Edges, amount keyed 5 to 100 and phase keyed 0 to 3600, cycle noise on, cycle 4 on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha Squared Animation, Edges, amount keyed 5 to 100 and phase keyed 0 to 3600, cycle noise on, cycle 4 on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha Squared Animation, Edges, amount keyed 5 to 100 and phase keyed 0 to 3600, cycle noise on, cycle 4 on three layers, frame 0, Draft | largest difference 1 of 255, 128 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha Squared Animation, Edges, amount keyed 5 to 100 and phase keyed 0 to 3600, cycle noise on, cycle 4 on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Noise Alpha Squared Animation, Edges, amount keyed 5 to 100 and phase keyed 0 to 3600, cycle noise on, cycle 4 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-450 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, frame 0, as added: the street speckled see-through, keeping at least four fifths of its covering; draws cleanly | [], 64350 pixels changed of 129600, the largest by 51 of 255 | yes |
| 3_squared_60.png, frame 0, Squared Random, amount 60: deeper, more contrasting holes, keeping at least two fifths of its covering; draws cleanly | [], 64867 pixels changed of 129600, the largest by 153 of 255 | yes |
| 4_wrap_100.png, frame 0, amount 100, Wrap: a covering pushed past full comes round from nothing, so some pixels all but vanish; draws cleanly | [], 129338 pixels changed of 129600, the largest by 255 of 255 | yes |
| 5_animation_frame_24.png, frame 24, Uniform Animation, amount 50, phase keyed 0 to 470 over the shot, at frame 24; draws cleanly | [], 64401 pixels changed of 129600, the largest by 127 of 255 | yes |

## Result

210 of 210 checks pass.
