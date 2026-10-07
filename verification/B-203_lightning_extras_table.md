# B-203: Lightning Bolt's Advanced Lightning extras

D-324, from D-308, accepted by the owner on 2026-10-04. Every expected pixel is `Fixtures/lightning_extras/expected_lightning_extras.json`, written by `tools/lightning_extras_reference.py` before this code existed and printed in document 25 as FX-LIGHTX-001 to 024. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5. D-190's own FX-BOLT-001 to 032 are checked again, unchanged, by B-126.

## FX-LIGHTX-001 to 024 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LIGHTX-001 frame 0: FX-BOLT-001's settings with the five new ones written at their starting values (type direction, turbulence 0, decay 0, conductivity 0, obstacle 0): exactly FX-BOLT-001. | largest difference 6.6e-8 | yes |
| FX-LIGHTX-001 frame 1: FX-BOLT-001's settings with the five new ones written at their starting values (type direction, turbulence 0, decay 0, conductivity 0, obstacle 0): exactly FX-BOLT-001. | largest difference 6.6e-8 | yes |
| FX-LIGHTX-001 frame 2: FX-BOLT-001's settings with the five new ones written at their starting values (type direction, turbulence 0, decay 0, conductivity 0, obstacle 0): exactly FX-BOLT-001. | largest difference 6.3e-8 | yes |
| FX-LIGHTX-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-002 frame 0: A straight line across row 4 from right to left, from (110, 45) to (-10, 45) per cent, width 1, no glow, with Alpha Obstacle 50 over the night sky's left half: it stops where it meets the sky. Columns 8 to 15 of row 4 lit, columns 0 to 6 untouched. | largest difference 1.7e-8 | yes |
| FX-LIGHTX-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-003 frame 0: FX-LIGHTX-002 with Alpha Obstacle 0: the whole row lit, as before. | largest difference 5.1e-8 | yes |
| FX-LIGHTX-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-004 frame 0: A jagged bolt from the middle of the top edge to the middle of the bottom, width 1, no glow, over a drawing whose rows 7 to 9 are ground, with Alpha Obstacle 50: it ends where it reaches the ground, and row 9 is untouched. Tutorial 2's bolt ending on the ground. | largest difference 2.9e-8 | yes |
| FX-LIGHTX-004 frame 1: A jagged bolt from the middle of the top edge to the middle of the bottom, width 1, no glow, over a drawing whose rows 7 to 9 are ground, with Alpha Obstacle 50: it ends where it reaches the ground, and row 9 is untouched. Tutorial 2's bolt ending on the ground. | largest difference 2.4e-8 | yes |
| FX-LIGHTX-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-005 frame 0: FX-LIGHTX-004 with Alpha Obstacle 0: the bolt runs on through the ground to the bottom edge. | largest difference 3.3e-8 | yes |
| FX-LIGHTX-005 frame 1: FX-LIGHTX-004 with Alpha Obstacle 0: the bolt runs on through the ground to the bottom edge. | largest difference 4.2e-8 | yes |
| FX-LIGHTX-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-006 frame 0: FX-LIGHTX-004 with branches 100: the forks stop at the ground too. | largest difference 2.9e-8 | yes |
| FX-LIGHTX-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-007 frame 0: FX-LIGHTX-004 over ground that half covers, Alpha Obstacle 40: half covering is not enough to block at 40, so the bolt runs to the bottom, as FX-LIGHTX-005 does on that drawing. | largest difference 4.1e-8 | yes |
| FX-LIGHTX-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-008 frame 0: FX-LIGHTX-007 at Alpha Obstacle 60: now half covering blocks, and the bolt ends at the ground. | largest difference 3.2e-8 | yes |
| FX-LIGHTX-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-009 frame 0: FX-BOLT-010's bolt (branches 100) as Strike: the main bolt the same, its forks turned from the way to the end point instead. | largest difference 6.2e-8 | yes |
| FX-LIGHTX-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-010 frame 0: FX-BOLT-010's bolt as Breaking: the main bolt and the forks' paths the same, each fork starting as bright and as wide as the bolt where it leaves it. | largest difference 6.2e-8 | yes |
| FX-LIGHTX-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-011 frame 0: FX-BOLT-009's bolt as Bouncy: three bolts between the two points, there, back and there again, each its own shape. | largest difference 6.2e-8 | yes |
| FX-LIGHTX-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-012 frame 0: Omni from the middle, (50, 50), with the end point 3 pixels above it: six bolts 3 pixels long, 60 degrees apart, the first straight up. | largest difference 5.1e-8 | yes |
| FX-LIGHTX-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-013 frame 0: Anywhere from the middle, the end point 3 pixels above it, hold 1: one bolt to somewhere within 3 pixels, a new place each frame. | largest difference 3.1e-8 | yes |
| FX-LIGHTX-013 frame 1: Anywhere from the middle, the end point 3 pixels above it, hold 1: one bolt to somewhere within 3 pixels, a new place each frame. | largest difference 2.5e-8 | yes |
| FX-LIGHTX-013 frame 2: Anywhere from the middle, the end point 3 pixels above it, hold 1: one bolt to somewhere within 3 pixels, a new place each frame. | largest difference 1.9e-8 | yes |
| FX-LIGHTX-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-014 frame 0: Vertical from (25, 0), the end point at (90, 30): a bolt from column 4 of the top edge straight down to the bottom edge; the end point is not used. | largest difference 4.6e-8 | yes |
| FX-LIGHTX-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-015 frame 0: FX-LIGHTX-014 with the end point at (10, 80): the same. | largest difference 4.6e-8 | yes |
| FX-LIGHTX-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-016 frame 0: Two-Way Striking from the top edge's middle to the bottom's: two bolts, one down from the top and one up from the bottom, meeting in the middle. | largest difference 5.2e-8 | yes |
| FX-LIGHTX-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-017 frame 0: FX-BOLT-001 with turbulence 100: a rougher bolt with more forks. | largest difference 6.1e-8 | yes |
| FX-LIGHTX-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-018 frame 0: FX-BOLT-006's line (rows 4 and 5, width 2, left to right) with decay 100: thick at the left, thinning to nothing at the right. | largest difference 2.8e-8 | yes |
| FX-LIGHTX-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-019 frame 0: FX-BOLT-009's bolt held for 100 frames, its conductivity keyed from 0 at frame 0 to 2 at frame 4: frame 0 is FX-BOLT-009's frame 0; the bolt changes shape smoothly through frames 1 to 4 without being redrawn. | largest difference 6.2e-8 | yes |
| FX-LIGHTX-019 frame 1: FX-BOLT-009's bolt held for 100 frames, its conductivity keyed from 0 at frame 0 to 2 at frame 4: frame 0 is FX-BOLT-009's frame 0; the bolt changes shape smoothly through frames 1 to 4 without being redrawn. | largest difference 5.8e-8 | yes |
| FX-LIGHTX-019 frame 2: FX-BOLT-009's bolt held for 100 frames, its conductivity keyed from 0 at frame 0 to 2 at frame 4: frame 0 is FX-BOLT-009's frame 0; the bolt changes shape smoothly through frames 1 to 4 without being redrawn. | largest difference 4.9e-8 | yes |
| FX-LIGHTX-019 frame 4: FX-BOLT-009's bolt held for 100 frames, its conductivity keyed from 0 at frame 0 to 2 at frame 4: frame 0 is FX-BOLT-009's frame 0; the bolt changes shape smoothly through frames 1 to 4 without being redrawn. | largest difference 4.8e-8 | yes |
| FX-LIGHTX-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTX-020 frame 0: A lightning type "sideways". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LIGHTX-020 frame 4: A lightning type "sideways". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LIGHTX-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIGHTX-021 frame 0: Turbulence 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LIGHTX-021 frame 4: Turbulence 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LIGHTX-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIGHTX-022 frame 0: Decay -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LIGHTX-022 frame 4: Decay -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LIGHTX-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIGHTX-023 frame 0: Conductivity 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LIGHTX-023 frame 4: Conductivity 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LIGHTX-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIGHTX-024 frame 0: Alpha Obstacle 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LIGHTX-024 frame 4: Alpha Obstacle 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LIGHTX-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lightx_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_001.json, a file from before D-324, is saved with none of the five new settings | [] | yes |
| fx_lightx_020.json's lightning type "sideways" is named in a sentence | Lightning Bolt's lightning type is "direction", "strike", "breaking", "bouncy", "omni", "anywhere", "vertical" or "two_way", and this is "sideways". | yes |
| a file with a turbulence that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a lightning type that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| a half-size draft preview halves the width and the glow, and none of the five new settings | LightningBolt { start: [40.0, 0.0], end: [60.0, 100.0], jagged: 40.0, detail: 6.0, branches: 30.0, width: 1.5, glow: 12.0, opacity: 100.0, hold: 2.0, seed: 0.0, color: "#ffffff", glow_color: "#6e8cff", composite: "on", kind: "strike", turbulence: 50.0, decay: 50.0, conductivity: 3.0, obstacle: 50.0, path: "split", core: "hard", frame: 0 } | yes |
| every new setting at its top, it never grows the drawing's bounds | 0 | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| lightning type "sideways" is refused with a sentence, and nothing changes | Lightning Bolt's lightning type is "direction", "strike", "breaking", "bouncy", "omni", "anywhere", "vertical" or "two_way", and this is "sideways". | yes |
| turbulence 101 is refused with a sentence, and nothing changes | Lightning Bolt's turbulence runs from 0 to 100, and this is 101. | yes |
| decay -1 is refused with a sentence, and nothing changes | Lightning Bolt's decay runs from 0 to 100, and this is -1. | yes |
| conductivity 10001 is refused with a sentence, and nothing changes | Lightning Bolt's conductivity runs from 0 to 10000, and this is 10001. | yes |
| Alpha Obstacle -101 is refused with a sentence, and nothing changes | Lightning Bolt's obstacle runs from -100 to 100, and this is -101. | yes |
| Alpha Obstacle keyed to 150 is refused with a sentence, and nothing changes | Lightning Bolt's obstacle runs from -100 to 100, and this is 150. | yes |
| a lightning type, is taken | taken | yes |
| a lightning type, is taken | taken | yes |
| a lightning type, is taken | taken | yes |
| a lightning type, is taken | taken | yes |
| a lightning type, is taken | taken | yes |
| a lightning type, is taken | taken | yes |
| a lightning type, is taken | taken | yes |
| every new number at its top, is taken | taken | yes |
| conductivity keyed from 0 to 3 is taken | taken | yes |
| Alpha Obstacle keyed from 0 to 100 is taken | taken | yes |
| undo 10 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lightx_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lightx_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lightx_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lightx_019.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: B-126's night scene, its hill a drawing of its own, in `verification/D-324 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the sky with the hill over it, draws cleanly, the sky at the top and the hill at the bottom | [], top [58, 58, 78, 255], bottom [14, 16, 28, 255] | yes |
| 1_no_obstacle.png, type Direction, Alpha Obstacle 0: the bolt runs on through the hill to the bottom edge, draws cleanly and changes some pixels | [], 3548 changed, 14 of the bottom row | yes |
| 2_obstacle_50.png, Alpha Obstacle 50: the bolt ends where it reaches the hill, draws cleanly and changes some pixels | [], 2491 changed, 0 of the bottom row | yes |
| 3_strike.png, Strike: the forks reach on toward the end point, draws cleanly and changes some pixels | [], 3497 changed, 15 of the bottom row | yes |
| 4_breaking.png, Breaking: the forks as bright as the bolt where they leave it, draws cleanly and changes some pixels | [], 3727 changed, 14 of the bottom row | yes |
| 5_bouncy.png, Bouncy: three bolts between the two points, draws cleanly and changes some pixels | [], 9013 changed, 26 of the bottom row | yes |
| 6_omni.png, Omni: six bolts from the start, 60 degrees apart, draws cleanly and changes some pixels | [], 7830 changed, 0 of the bottom row | yes |
| 7_anywhere.png, Anywhere: one bolt from the start to somewhere within reach, draws cleanly and changes some pixels | [], 1291 changed, 0 of the bottom row | yes |
| 8_vertical.png, Vertical: straight down to the bottom edge, the end point unused, draws cleanly and changes some pixels | [], 3515 changed, 13 of the bottom row | yes |
| 9_two_way.png, Two-Way Striking: one bolt from each end, meeting in the middle, draws cleanly and changes some pixels | [], 3292 changed, 13 of the bottom row | yes |
| 10_turbulence_100.png, turbulence 100: rougher, with more forks, draws cleanly and changes some pixels | [], 6423 changed, 21 of the bottom row | yes |
| 11_decay_100.png, decay 100: thinning to nothing at its end, draws cleanly and changes some pixels | [], 1928 changed, 1 of the bottom row | yes |
| 12_conductivity_0_5.png, conductivity 0.5: picture 1's bolt half way to another shape, draws cleanly and changes some pixels | [], 3147 changed, 14 of the bottom row | yes |
| with no obstacle the bolt reaches the bottom row; with Alpha Obstacle 50 its lowest lit pixel is within its glow (6 pixels) of the hill's top, and the bottom row is untouched | bottom row 14 and 0, lowest lit row 209, the hill at most row 222 | yes |
| each type draws a picture of its own: no two of pictures 1 and 3 to 12 are the same | 12 pictures | yes |

## Result

128 of 128 checks pass.
