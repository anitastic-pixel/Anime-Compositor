# B-211: Lightning Bolt round shapes and a negative Alpha Obstacle

D-329, from P-26's tutorial 2. Every expected pixel is `Fixtures/lightning_around/expected_lightning_around.json`, written by `tools/lightning_around_reference.py` before this code existed and printed in document 25 as FX-LIGHTA-001 to 014. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5. D-324's FX-LIGHTX-001 to 024 are checked again, unchanged, by B-203, and D-190's FX-BOLT-001 to 032 by B-126.

## FX-LIGHTA-001 to 014 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LIGHTA-001 frame 0: FX-LIGHTX-004 with the new word written at its starting value (path split): exactly FX-LIGHTX-004. | largest difference 2.9e-8 | yes |
| FX-LIGHTA-001 frame 1: FX-LIGHTX-004 with the new word written at its starting value (path split): exactly FX-LIGHTX-004. | largest difference 2.4e-8 | yes |
| FX-LIGHTA-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTA-002 frame 0: FX-LIGHTX-005 (no obstacle) with path around: exactly the same, a bolt with nothing to go round is not changed. | largest difference 3.3e-8 | yes |
| FX-LIGHTA-002 frame 1: FX-LIGHTX-005 (no obstacle) with path around: exactly the same, a bolt with nothing to go round is not changed. | largest difference 4.2e-8 | yes |
| FX-LIGHTA-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTA-003 frame 0: The jagged bolt from the top edge's middle to the bottom's, width 1, no glow, over a block of rock across columns 5 to 10, rows 3 to 6, Alpha Obstacle 50, path split: it stops at the block's top, rows 7 to 9 untouched. | largest difference 2.9e-8 | yes |
| FX-LIGHTA-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTA-004 frame 0: FX-LIGHTA-003 with path around: the bolt goes round the block and reaches the bottom edge; the block's middle is untouched. | largest difference 2.7e-8 | yes |
| FX-LIGHTA-004 frame 1: FX-LIGHTA-003 with path around: the bolt goes round the block and reaches the bottom edge; the block's middle is untouched. | largest difference 2.8e-8 | yes |
| FX-LIGHTA-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTA-005 frame 0: FX-LIGHTA-004 with branches 100: the main bolt goes round, its forks stop at the block. | largest difference 2.7e-8 | yes |
| FX-LIGHTA-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTA-006 frame 0: FX-LIGHTA-004 with the end point inside the block, (50, 50) per cent: there is no way in, so the bolt goes round and ends at the free pixel nearest it, just under the block. | largest difference 2.7e-8 | yes |
| FX-LIGHTA-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTA-007 frame 0: FX-LIGHTX-004's ground with path around: no way round the ground, so the bolt ends at the free pixel nearest the end point, and row 9 is untouched. | largest difference 1.7e-8 | yes |
| FX-LIGHTA-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTA-008 frame 0: A straight line from the block's middle to the right, from (50, 45) to (110, 45) per cent, width 1, no glow, Alpha Obstacle -50, path split: it stays inside the block and stops at its right edge, columns 11 to 15 untouched. | largest difference 5.1e-8 | yes |
| FX-LIGHTA-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTA-009 frame 0: A jagged bolt inside a U of rock, from the left arm, (20, 25) per cent, to the right arm, (75, 25), width 1, no glow, Alpha Obstacle -50, path split: it stops where it leaves the left arm. | largest difference 2.2e-8 | yes |
| FX-LIGHTA-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTA-010 frame 0: FX-LIGHTA-009 with path around: the bolt runs down the left arm, along the bottom and up the right arm to the end point; the empty sky between the arms, columns 6 to 9 above row 6, is untouched. | largest difference 5.1e-8 | yes |
| FX-LIGHTA-010 frame 1: FX-LIGHTA-009 with path around: the bolt runs down the left arm, along the bottom and up the right arm to the end point; the empty sky between the arms, columns 6 to 9 above row 6, is untouched. | largest difference 7.2e-8 | yes |
| FX-LIGHTA-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTA-011 frame 0: A jagged bolt along row 8 of half-covering ground, from (5, 85) to (95, 85) per cent, Alpha Obstacle -40, path around: half covering is enough to keep it, and the bolt stays inside rows 7 to 9. | largest difference 4.5e-8 | yes |
| FX-LIGHTA-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTA-012 frame 0: FX-LIGHTA-011 at Alpha Obstacle -60: half covering is now too thin, the start blocks, and no bolt is drawn. | largest difference 3.0e-8 | yes |
| FX-LIGHTA-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LIGHTA-013 frame 0: A path "sideways". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LIGHTA-013 frame 4: A path "sideways". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LIGHTA-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIGHTA-014 frame 0: Alpha Obstacle -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LIGHTA-014 frame 4: Alpha Obstacle -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LIGHTA-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lighta_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lighta_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lighta_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lighta_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lighta_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lighta_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lighta_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lighta_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lighta_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lighta_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lighta_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lighta_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lighta_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lighta_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lightx_004.json, a file from before D-329, is saved without `path` | None | yes |
| fx_lighta_013.json's path "sideways" is named in a sentence | Lightning Bolt's path at an obstacle is "split" or "around", and this is "sideways". | yes |
| a file with a path that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| a half-size draft preview halves the width and the glow, and neither new setting | LightningBolt { start: [50.0, 0.0], end: [50.0, 100.0], jagged: 40.0, detail: 6.0, branches: 0.0, width: 0.5, glow: 0.0, opacity: 100.0, hold: 1.0, seed: 0.0, color: "#ffffff", glow_color: "#6e8cff", composite: "on", kind: "direction", turbulence: 0.0, decay: 0.0, conductivity: 0.0, obstacle: -50.0, path: "around", frame: 0 } | yes |
| going round, it never grows the drawing's bounds | 0 | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| path "sideways" is refused with a sentence, and nothing changes | Lightning Bolt's path at an obstacle is "split" or "around", and this is "sideways". | yes |
| Alpha Obstacle -101 is refused with a sentence, and nothing changes | Lightning Bolt's obstacle runs from -100 to 100, and this is -101. | yes |
| Alpha Obstacle keyed to -150 is refused with a sentence, and nothing changes | Lightning Bolt's obstacle runs from -100 to 100, and this is -150. | yes |
| path around, is taken | taken | yes |
| Alpha Obstacle -100, is taken | taken | yes |
| Alpha Obstacle keyed from 100 to -100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lighta_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lighta_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lighta_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a night sky with rock over it, in `verification/D-329 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| disc_before.png, the sky with the round rock, draws cleanly | [] | yes |
| 1_disc_stop.png, the bolt from the top edge to the bottom, Alpha Obstacle 50, Stop: it ends on the rock | [], 0 of the bottom row lit, 0 lit more than 10 pixels inside the rock | yes |
| 2_disc_go_round.png, the same with Go Round: the bolt goes round the rock and on to the bottom edge | [], 18 of the bottom row lit, 0 lit more than 10 pixels inside the rock | yes |
| ring_before.png, the sky with the ring of rock, draws cleanly | [] | yes |
| 3_ring_inside_stop.png, a bolt from the ring's left side to its right, Alpha Obstacle -50, Stop: it ends where it leaves the rock | [], 0 lit more than 10 pixels into the hole, 0 more than 10 outside the ring, right side reached: false | yes |
| 4_ring_inside_go_round.png, the same with Go Round: the bolt follows the ring round to its right side, never crossing the hole | [], 0 lit more than 10 pixels into the hole, 0 more than 10 outside the ring, right side reached: true | yes |
| Stop and Go Round draw different pictures, on the disc and on the ring | 4 pictures | yes |

## Result

70 of 70 checks pass.
