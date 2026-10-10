# D-410: Line Boil

B-289: PLUGINS.md's pick #4, merged into `core.turbulent_displace` as New Seed Every (`new_seed_every`, 0 to 100 frames, keyable, its whole part counted). With a whole part N of 1 or more the seed for frame f is the seed plus floor(f / N), P0-23's held step, so the warp holds for N frames and jumps to another. 0, what a file without it means, keeps one seed. Every expected pixel is `Fixtures/line_boil/expected_line_boil.json`, written by `tools/line_boil_reference.py` before this code existed and printed in document 25 as FX-BOIL-001 to 013. Tolerance 2e-5.

## FX-BOIL-001 to 013 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BOIL-001 frame 0: New seed every 2 frames, amount 30, size 8, speed 0 (a push of 2.4 pixels at most): frames 0 and 1 are FX-TURB-AE-007 (seed 0), frames 2 and 3 the same warp with seed 1, frame 4 seed 2: the stripes jump to a new wobble every second frame and hold still between, the hand-drawn boil. | largest difference 2.5e-7 | yes |
| FX-BOIL-001 frame 1: New seed every 2 frames, amount 30, size 8, speed 0 (a push of 2.4 pixels at most): frames 0 and 1 are FX-TURB-AE-007 (seed 0), frames 2 and 3 the same warp with seed 1, frame 4 seed 2: the stripes jump to a new wobble every second frame and hold still between, the hand-drawn boil. | largest difference 2.5e-7 | yes |
| FX-BOIL-001 frame 2: New seed every 2 frames, amount 30, size 8, speed 0 (a push of 2.4 pixels at most): frames 0 and 1 are FX-TURB-AE-007 (seed 0), frames 2 and 3 the same warp with seed 1, frame 4 seed 2: the stripes jump to a new wobble every second frame and hold still between, the hand-drawn boil. | largest difference 2.5e-7 | yes |
| FX-BOIL-001 frame 3: New seed every 2 frames, amount 30, size 8, speed 0 (a push of 2.4 pixels at most): frames 0 and 1 are FX-TURB-AE-007 (seed 0), frames 2 and 3 the same warp with seed 1, frame 4 seed 2: the stripes jump to a new wobble every second frame and hold still between, the hand-drawn boil. | largest difference 2.5e-7 | yes |
| FX-BOIL-001 frame 4: New seed every 2 frames, amount 30, size 8, speed 0 (a push of 2.4 pixels at most): frames 0 and 1 are FX-TURB-AE-007 (seed 0), frames 2 and 3 the same warp with seed 1, frame 4 seed 2: the stripes jump to a new wobble every second frame and hold still between, the hand-drawn boil. | largest difference 2.7e-7 | yes |
| FX-BOIL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOIL-002 frame 0: New seed every 0, written: one seed for ever, so every frame is FX-TURB-AE-007. | largest difference 2.5e-7 | yes |
| FX-BOIL-002 frame 2: New seed every 0, written: one seed for ever, so every frame is FX-TURB-AE-007. | largest difference 2.5e-7 | yes |
| FX-BOIL-002 frame 4: New seed every 0, written: one seed for ever, so every frame is FX-TURB-AE-007. | largest difference 2.5e-7 | yes |
| FX-BOIL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOIL-003 frame 0: New seed every 1: a new seed on every frame, frame f drawn with seed f. | largest difference 2.5e-7 | yes |
| FX-BOIL-003 frame 1: New seed every 1: a new seed on every frame, frame f drawn with seed f. | largest difference 2.5e-7 | yes |
| FX-BOIL-003 frame 2: New seed every 1: a new seed on every frame, frame f drawn with seed f. | largest difference 2.7e-7 | yes |
| FX-BOIL-003 frame 3: New seed every 1: a new seed on every frame, frame f drawn with seed f. | largest difference 2.5e-7 | yes |
| FX-BOIL-003 frame 4: New seed every 1: a new seed on every frame, frame f drawn with seed f. | largest difference 2.5e-7 | yes |
| FX-BOIL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOIL-004 frame 0: New seed every 3 from seed 8: frames 0 to 2 seed 8, frames 3 and 4 seed 9. | largest difference 2.5e-7 | yes |
| FX-BOIL-004 frame 2: New seed every 3 from seed 8: frames 0 to 2 seed 8, frames 3 and 4 seed 9. | largest difference 2.5e-7 | yes |
| FX-BOIL-004 frame 3: New seed every 3 from seed 8: frames 0 to 2 seed 8, frames 3 and 4 seed 9. | largest difference 2.8e-7 | yes |
| FX-BOIL-004 frame 4: New seed every 3 from seed 8: frames 0 to 2 seed 8, frames 3 and 4 seed 9. | largest difference 2.8e-7 | yes |
| FX-BOIL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOIL-005 frame 0: New seed every 2.9: the whole part, 2, is counted, so this is FX-BOIL-001. | largest difference 2.5e-7 | yes |
| FX-BOIL-005 frame 1: New seed every 2.9: the whole part, 2, is counted, so this is FX-BOIL-001. | largest difference 2.5e-7 | yes |
| FX-BOIL-005 frame 2: New seed every 2.9: the whole part, 2, is counted, so this is FX-BOIL-001. | largest difference 2.5e-7 | yes |
| FX-BOIL-005 frame 3: New seed every 2.9: the whole part, 2, is counted, so this is FX-BOIL-001. | largest difference 2.5e-7 | yes |
| FX-BOIL-005 frame 4: New seed every 2.9: the whole part, 2, is counted, so this is FX-BOIL-001. | largest difference 2.7e-7 | yes |
| FX-BOIL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOIL-006 frame 0: New seed every 2 with speed 20: the evolution still moves every frame, so no two frames are alike, and frames 2 and 3 use seed 1. | largest difference 2.5e-7 | yes |
| FX-BOIL-006 frame 1: New seed every 2 with speed 20: the evolution still moves every frame, so no two frames are alike, and frames 2 and 3 use seed 1. | largest difference 2.5e-7 | yes |
| FX-BOIL-006 frame 2: New seed every 2 with speed 20: the evolution still moves every frame, so no two frames are alike, and frames 2 and 3 use seed 1. | largest difference 2.6e-7 | yes |
| FX-BOIL-006 frame 3: New seed every 2 with speed 20: the evolution still moves every frame, so no two frames are alike, and frames 2 and 3 use seed 1. | largest difference 2.5e-7 | yes |
| FX-BOIL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOIL-007 frame 0: New seed every keyed from 1 at frame 0 to 4 at frame 4, linear: frame 0 seed 0; at frame 2 the setting is 2.5, whole part 2, step 1; at frame 4 it is 4, step 1. | largest difference 2.5e-7 | yes |
| FX-BOIL-007 frame 2: New seed every keyed from 1 at frame 0 to 4 at frame 4, linear: frame 0 seed 0; at frame 2 the setting is 2.5, whole part 2, step 1; at frame 4 it is 4, step 1. | largest difference 2.5e-7 | yes |
| FX-BOIL-007 frame 4: New seed every keyed from 1 at frame 0 to 4 at frame 4, linear: frame 0 seed 0; at frame 2 the setting is 2.5, whole part 2, step 1; at frame 4 it is 4, step 1. | largest difference 2.5e-7 | yes |
| FX-BOIL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOIL-008 frame 0: New seed every 2 in a file without units (D-127's classic push), amount 3, size 8, speed 0: FX-TURB-004's warp on frames 0 and 1, seed 1 on frames 2 and 3. | largest difference 2.5e-7 | yes |
| FX-BOIL-008 frame 1: New seed every 2 in a file without units (D-127's classic push), amount 3, size 8, speed 0: FX-TURB-004's warp on frames 0 and 1, seed 1 on frames 2 and 3. | largest difference 2.5e-7 | yes |
| FX-BOIL-008 frame 2: New seed every 2 in a file without units (D-127's classic push), amount 3, size 8, speed 0: FX-TURB-004's warp on frames 0 and 1, seed 1 on frames 2 and 3. | largest difference 2.5e-7 | yes |
| FX-BOIL-008 frame 3: New seed every 2 in a file without units (D-127's classic push), amount 3, size 8, speed 0: FX-TURB-004's warp on frames 0 and 1, seed 1 on frames 2 and 3. | largest difference 2.5e-7 | yes |
| FX-BOIL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOIL-009 frame 0: FX-BOIL-001 moved three pixels right: the layer grows by 3, so the three columns left of the drawing show the stripes pushed into them, and the boil is the same, moved. | largest difference 2.5e-7 | yes |
| FX-BOIL-009 frame 2: FX-BOIL-001 moved three pixels right: the layer grows by 3, so the three columns left of the drawing show the stripes pushed into them, and the boil is the same, moved. | largest difference 2.5e-7 | yes |
| FX-BOIL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOIL-010 frame 0: FX-BOIL-001 with edges repeat: nothing grows, a push past the edge reads the nearest edge pixel; the seed steps as before. | largest difference 2.5e-7 | yes |
| FX-BOIL-010 frame 2: FX-BOIL-001 with edges repeat: nothing grows, a push past the edge reads the nearest edge pixel; the seed steps as before. | largest difference 2.5e-7 | yes |
| FX-BOIL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BOIL-011 frame 0: New seed every -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BOIL-011 frame 4: New seed every -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BOIL-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BOIL-012 frame 0: New seed every 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BOIL-012 frame 4: New seed every 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BOIL-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BOIL-013 frame 0: New seed every keyed from 2 at frame 0 to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BOIL-013 frame 4: New seed every keyed from 2 at frame 0 to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BOIL-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Older projects: one seed, as before

| Check | The build's answer | Matches |
| --- | --- | --- |
| turbulent_displace/fx_turb_001.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_002.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_003.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_004.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_005.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_006.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_007.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_008.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_009.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_010.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_011.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_012.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_013.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_014.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_015.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_016.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_017.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_018.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_019.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_020.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_021.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_022.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_023.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_024.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_025.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_026.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_001.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_002.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_003.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_004.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_005.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_006.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_007.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_008.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_009.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_010.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_011.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_012.json (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0 | saved the same; bit-identical | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_boil_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_boil_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_boil_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_boil_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_boil_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_boil_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_boil_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_boil_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_boil_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_boil_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_boil_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_boil_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_boil_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_boil_004.json is saved with New Seed Every 3 | {"amount":30,"complexity":2,"edges":"transparent","evolution":0,"new_seed_every":3,"seed":8,"size":8,"speed":0,"units":"after_effects"} | yes |
| fx_boil_011.json (New Seed Every -1) is refused in a sentence naming new_seed_every | Turbulent Displace's new seed every runs from 0 to 100, and this is -1. | yes |
| fx_boil_012.json (New Seed Every 101) is refused in a sentence naming new_seed_every | Turbulent Displace's new seed every runs from 0 to 100, and this is 101. | yes |
| a file with a Turbulent Displace whose New Seed Every is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| a half-size draft preview leaves New Seed Every, a count of frames, as it was | TurbulentDisplace { amount: 30.0, size: 4.0, complexity: 2.0, evolution: 0.0, speed: 0.0, seed: 0.0, edges: "transparent", frame: 0, displacement: "turbulent", pinning: "none", units: "after_effects", new_seed_every: 2.0 } | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| new seed every 101 is refused with a sentence, and nothing changes | Turbulent Displace's new seed every runs from 0 to 100, and this is 101. | yes |
| new seed every -1 is refused with a sentence, and nothing changes | Turbulent Displace's new seed every runs from 0 to 100, and this is -1. | yes |
| new seed every keyed to 150 is refused with a sentence, and nothing changes | Turbulent Displace's new seed every runs from 0 to 100, and this is 150. | yes |
| new seed every 3 is taken | taken | yes |
| new seed every keyed from 1 to 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_boil_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_boil_001.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_boil_004.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_boil_006.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_boil_007.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_boil_009.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_boil_010.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_boil_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_boil_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_boil_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_boil_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_boil_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_boil_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_boil_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_boil_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_boil_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_boil_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_boil_011.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_boil_012.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_boil_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Turbulent Displace amount 30, size 40, new seed every 2: the processor's frame 102 differs from the same warp with one seed, so the comparisons below test the new seed | 1676017 pixels changed | yes |
| the reference shot, Turbulent Displace amount 30, size 40, new seed every 2 on three layers, frame 0, Full | largest difference 1 of 255, 2278 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, new seed every 2 on three layers, frame 1, Full | largest difference 1 of 255, 2276 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, new seed every 2 on three layers, frame 100, Full | largest difference 1 of 255, 941 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, new seed every 2 on three layers, frame 102, Full | largest difference 1 of 255, 1079 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, new seed every 2 on three layers, frame 239, Full | largest difference 1 of 255, 923 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, new seed every 2 on three layers, frame 0, Draft | largest difference 1 of 255, 73 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, new seed every 2 on three layers, frame 1, Draft | largest difference 1 of 255, 73 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, new seed every 2 on three layers, frame 100, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, new seed every 2 on three layers, frame 102, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, new seed every 2 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, new seed every 3: the processor's frame 102 differs from the same warp with one seed, so the comparisons below test the new seed | 1637465 pixels changed | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, new seed every 3 on three layers, frame 0, Full | largest difference 1 of 255, 2064 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, new seed every 3 on three layers, frame 1, Full | largest difference 1 of 255, 2071 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, new seed every 3 on three layers, frame 100, Full | largest difference 1 of 255, 1068 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, new seed every 3 on three layers, frame 102, Full | largest difference 1 of 255, 1109 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, new seed every 3 on three layers, frame 239, Full | largest difference 1 of 255, 1065 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, new seed every 3 on three layers, frame 0, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, new seed every 3 on three layers, frame 1, Draft | largest difference 1 of 255, 80 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, new seed every 3 on three layers, frame 100, Draft | largest difference 1 of 255, 73 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, new seed every 3 on three layers, frame 102, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, new seed every 3 on three layers, frame 239, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-410 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_boil_frame_0.png to 6_boil_frame_4.png, amount 6, size 30, a new seed every 2: frames 0 and 1 the same, frames 2 and 3 the same and a different wobble, frame 4 another; each warped from the street; all draw cleanly | pixels differing: 0 to 1 0, 1 to 2 31095, 2 to 3 0, 3 to 4 29457; frame 0 against the street 20687 | yes |
| 7_one_seed_frame_4.png, the same with New Seed Every 0, frame 4: the warp of frame 0, never jumping; draws cleanly | []; 0 pixels differ from 2_boil_frame_0.png | yes |

## Result

163 of 163 checks pass.
