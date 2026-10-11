# D-460: Burn Film

B-340: `core.burn_film` takes After Effects' CC Burn Film controls (Burn, Center, Random Seed). Every expected pixel is `Fixtures/burn_film/expected_burn_film.json`, written by `tools/burn_film_reference.py` before this code existed and printed in document 25 as FX-BURN-001 to 020. Tolerance 2e-5.

## FX-BURN-001 to 020 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BURN-001 frame 0: The settings as they start: Burn 0, nothing burned, the drawing as it is. | largest difference 1.9e-7 | yes |
| FX-BURN-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURN-002 frame 0: Burn 100: everything burned away, the layer empty. | largest difference 0.0e0 | yes |
| FX-BURN-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURN-003 frame 0: Burn 30: a hole has opened round the middle, ringed by black char and a scorched edge; the corners untouched. | largest difference 1.9e-7 | yes |
| FX-BURN-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURN-004 frame 0: Burn 50: the hole has spread over most of the middle. | largest difference 2.1e-7 | yes |
| FX-BURN-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURN-005 frame 0: Burn 75: nearly all of the layer gone, scorched edges round what is left. | largest difference 1.2e-8 | yes |
| FX-BURN-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURN-006 frame 0: Center 0, 0, Burn 25: the burning starts at the top left corner. | largest difference 1.9e-7 | yes |
| FX-BURN-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURN-007 frame 0: Center 100, 50, the middle of the right edge, Burn 25: the burning starts at the right, and the middle is untouched. | largest difference 1.9e-7 | yes |
| FX-BURN-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURN-008 frame 0: Random Seed 7, Burn 50: the other burns start elsewhere, and the edges are ragged differently, from FX-BURN-004. | largest difference 1.9e-7 | yes |
| FX-BURN-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURN-009 frame 0: Random Seed 7.9: its whole part, FX-BURN-008. | largest difference 1.9e-7 | yes |
| FX-BURN-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURN-010 frame 0: Burn keyed from 0 at frame 0 to 100 at frame 4, linear: the drawing, then Burn 25, 50 and 75, then empty. | largest difference 1.9e-7 | yes |
| FX-BURN-010 frame 1: Burn keyed from 0 at frame 0 to 100 at frame 4, linear: the drawing, then Burn 25, 50 and 75, then empty. | largest difference 1.9e-7 | yes |
| FX-BURN-010 frame 2: Burn keyed from 0 at frame 0 to 100 at frame 4, linear: the drawing, then Burn 25, 50 and 75, then empty. | largest difference 2.1e-7 | yes |
| FX-BURN-010 frame 3: Burn keyed from 0 at frame 0 to 100 at frame 4, linear: the drawing, then Burn 25, 50 and 75, then empty. | largest difference 1.2e-8 | yes |
| FX-BURN-010 frame 4: Burn keyed from 0 at frame 0 to 100 at frame 4, linear: the drawing, then Burn 25, 50 and 75, then empty. | largest difference 0.0e0 | yes |
| FX-BURN-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURN-011 frame 0: Center keyed from 25, 50 at frame 0 to 75, 50 at frame 4, Burn 20: the first hole moves across. | largest difference 1.9e-7 | yes |
| FX-BURN-011 frame 2: Center keyed from 25, 50 at frame 0 to 75, 50 at frame 4, Burn 20: the first hole moves across. | largest difference 1.9e-7 | yes |
| FX-BURN-011 frame 4: Center keyed from 25, 50 at frame 0 to 75, 50 at frame 4, Burn 20: the first hole moves across. | largest difference 1.9e-7 | yes |
| FX-BURN-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURN-012 frame 0: Burn keyed from 30 at frame 0 to 0 at frame 4, eased past its end: below 0 it is held at 0, the drawing as it is. | largest difference 1.9e-7 | yes |
| FX-BURN-012 frame 2: Burn keyed from 30 at frame 0 to 0 at frame 4, eased past its end: below 0 it is held at 0, the drawing as it is. | largest difference 1.9e-7 | yes |
| FX-BURN-012 frame 4: Burn keyed from 30 at frame 0 to 0 at frame 4, eased past its end: below 0 it is held at 0, the drawing as it is. | largest difference 1.9e-7 | yes |
| FX-BURN-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURN-013 frame 0: FX-BURN-004 moved three pixels right: the burn belongs to the layer, so it moves with it. | largest difference 2.1e-7 | yes |
| FX-BURN-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURN-014 frame 0: After a directional blur (direction 90, length 4) that grows the layer two pixels on every side, Burn 50: the burn is laid over the grown layer, 20 by 14, its middle and size. | largest difference 3.7e-8 | yes |
| FX-BURN-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURN-015 frame 0: Burn 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BURN-015 frame 4: Burn 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BURN-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BURN-016 frame 0: Burn -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BURN-016 frame 4: Burn -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BURN-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BURN-017 frame 0: Center at 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BURN-017 frame 4: Center at 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BURN-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BURN-018 frame 0: Random Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BURN-018 frame 4: Random Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BURN-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BURN-019 frame 0: Random Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BURN-019 frame 4: Random Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BURN-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BURN-020 frame 0: Burn keyed to 150 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BURN-020 frame 4: Burn keyed to 150 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BURN-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_burn_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burn_008.json is saved with its three settings | {"burn":50,"center":[50,50],"seed":7} | yes |
| fx_burn_015.json is refused in a sentence naming Burn Film and its burn | Burn Film's burn runs from 0 to 100, and this is 101. | yes |
| fx_burn_016.json is refused in a sentence naming Burn Film and its burn | Burn Film's burn runs from 0 to 100, and this is -1. | yes |
| fx_burn_017.json is refused in a sentence naming Burn Film and its center | Burn Film's center runs from -1000 to 1000, and this is 1001. | yes |
| fx_burn_018.json is refused in a sentence naming Burn Film and its seed | Burn Film's seed runs from 0 to 100000, and this is -1. | yes |
| fx_burn_019.json is refused in a sentence naming Burn Film and its seed | Burn Film's seed runs from 0 to 100000, and this is 100001. | yes |
| a file with a Burn Film whose burn is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Burn Film without its seed is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Burn Film whose center is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| burn 101 is refused with a sentence, and nothing changes | Burn Film's burn runs from 0 to 100, and this is 101. | yes |
| center 50, -1001 is refused with a sentence, and nothing changes | Burn Film's center runs from -1000 to 1000, and this is -1001. | yes |
| burn keyed to 150 is refused with a sentence, and nothing changes | Burn Film's burn runs from 0 to 100, and this is 150. | yes |
| burn 40, center 20, 30, seed 12 is taken | taken | yes |
| center keyed from 0, 0 to 100, 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_burn_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_burn_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_burn_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_burn_011.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_burn_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_burn_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_burn_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 4 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_burn_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Burn Film burn 30: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 349457 pixels changed | yes |
| the reference shot, Burn Film burn 30 on three layers, frame 0, Full | largest difference 1 of 255, 1086 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Burn Film burn 30 on three layers, frame 100, Full | largest difference 1 of 255, 1112 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Burn Film burn 30 on three layers, frame 239, Full | largest difference 1 of 255, 1011 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Burn Film burn 30 on three layers, frame 0, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Burn Film burn 30 on three layers, frame 100, Draft | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Burn Film burn 30 on three layers, frame 239, Draft | largest difference 1 of 255, 66 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Burn Film burn 60, center 20, 30, seed 5: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1717933 pixels changed | yes |
| the reference shot, Burn Film burn 60, center 20, 30, seed 5 on three layers, frame 0, Full | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Burn Film burn 60, center 20, 30, seed 5 on three layers, frame 100, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Burn Film burn 60, center 20, 30, seed 5 on three layers, frame 239, Full | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Burn Film burn 60, center 20, 30, seed 5 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Burn Film burn 60, center 20, 30, seed 5 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Burn Film burn 60, center 20, 30, seed 5 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-460 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| as added (Burn 0): the street byte for byte; draws cleanly | [], the same: true | yes |
| 2_burn_30.png, Burn 30: a hole through the middle (the middle pixel clear), ringed by a black char line and a scorched edge, the four corners untouched; draws cleanly | [], middle cover 0, char pixels 3023, scorched pixels 14944, corners untouched true | yes |
| 3_burn_60.png, Burn 60: more burned away than at 30; draws cleanly | [], pixels with any cover: before 129600, burn 30 125733, burn 60 56252 | yes |
| 4_from_the_corner.png, Center 0, 0, Burn 30: the burn starts at the top left (that corner clear), and the picture is not 2_burn_30.png; draws cleanly | [], top-left cover 0, different: true | yes |
| 5_seed_7.png, Random Seed 7, Burn 30: ragged differently from 2_burn_30.png; draws cleanly | [], different: true | yes |
| Burn 100: every pixel clear; draws cleanly | [], pixels with any cover 0 | yes |

## Result

136 of 136 checks pass.
