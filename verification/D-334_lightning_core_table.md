# B-215: Lightning Bolt's soft core

D-334, from P-26's tutorial 2. Every expected pixel is `Fixtures/lightning_core/expected_lightning_core.json`, written by `tools/lightning_core_reference.py` before this code existed and printed in document 25 as FX-LCORE-001 to 007. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5. D-190's and D-324's own cases are checked again, unchanged, by B-126 and B-203.

## FX-LCORE-001 to 007 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LCORE-001 frame 0: A straight line along the middle of row 4, width 6, no glow, core Soft: brightest on row 4 (1 - 1/12 of the colour), fading row by row to nothing past rows 1 and 7. | largest difference 2.5e-8 | yes |
| FX-LCORE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LCORE-002 frame 0: FX-LCORE-001 with core written "hard": rows 2 to 6 fully lit, rows 1 and 7 half, as D-190's core. | largest difference 5.1e-8 | yes |
| FX-LCORE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LCORE-003 frame 0: FX-BOLT-001's settings with core Soft: the same bolt and glow, the core dimmer toward its edge. | largest difference 5.6e-8 | yes |
| FX-LCORE-003 frame 2: FX-BOLT-001's settings with core Soft: the same bolt and glow, the core dimmer toward its edge. | largest difference 6.6e-8 | yes |
| FX-LCORE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LCORE-004 frame 0: The line at width 1, core Soft: row 4 half lit, as a core half a pixel wide on each side weighs its middle pixel. | largest difference 1.5e-8 | yes |
| FX-LCORE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LCORE-005 frame 0: The line at width 0 with glow 3, core Soft: no core, only D-190's glow. | largest difference 4.5e-8 | yes |
| FX-LCORE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LCORE-006 frame 0: A core "Soft": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LCORE-006 frame 4: A core "Soft": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LCORE-006: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LCORE-007 frame 0: A core "fuzzy", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LCORE-007 frame 4: A core "fuzzy", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LCORE-007: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lcore_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lcore_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lcore_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lcore_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lcore_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lcore_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lcore_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_001.json, a file from before D-334, is saved without the word core | None | yes |
| fx_lcore_007.json's core "fuzzy" is named in a sentence | Lightning Bolt's core edge is "hard" or "soft", and this is "fuzzy". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| core "fuzzy" is refused with a sentence, and nothing changes | Lightning Bolt's core edge is "hard" or "soft", and this is "fuzzy". | yes |
| core hard, is taken | taken | yes |
| core soft, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lcore_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lcore_003.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a night sky, in `verification/D-334 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_hard.png, core Hard, D-190's: the core one flat white ribbon with a hard edge, draws cleanly and changes some pixels | [], 4590 changed | yes |
| 2_soft.png, core Soft: the core white in the middle, fading into the glow at its edge, draws cleanly and changes some pixels | [], 4589 changed | yes |
| the soft core is never brighter than the hard one, and has fewer pure white pixels | pure white: hard 1171, soft 0 | yes |

## Result

35 of 35 checks pass.
