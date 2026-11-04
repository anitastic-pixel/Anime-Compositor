# B-931: Lightning Bolt's Core Opacity and Glow Opacity

D-1051, from wave 12's tutorial uGxruFKgt6U. Every expected pixel is `Fixtures/lightning_opacity/expected_lightning_opacity.json`, written by `tools/lightning_opacity_reference.py` before this code existed and printed in document 25 as FX-LOPAC-001 to 010. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5. D-190's, D-324's and D-334's own cases are checked again, unchanged, by B-126, B-203 and B-215.

## FX-LOPAC-001 to 010 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LOPAC-001 frame 0: D-334's wide line (width 6, no glow), core opacity 50: rows 2 to 6 half lit and rows 1 and 7 a quarter, half of D-190's core. | largest difference 1.5e-8 | yes |
| FX-LOPAC-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LOPAC-002 frame 0: The line at width 2 with glow 3, glow opacity 50: the core as D-190's, the glow round it half as strong. | largest difference 5.1e-8 | yes |
| FX-LOPAC-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LOPAC-003 frame 0: The same line, core opacity 0: no core, the glow full on the line where the core was. | largest difference 4.5e-8 | yes |
| FX-LOPAC-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LOPAC-004 frame 0: The same line, glow opacity 0: only the core, as with glow 0. | largest difference 5.1e-8 | yes |
| FX-LOPAC-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LOPAC-005 frame 0: FX-BOLT-001's settings with both written at 100: exactly FX-BOLT-001. | largest difference 6.6e-8 | yes |
| FX-LOPAC-005 frame 2: FX-BOLT-001's settings with both written at 100: exactly FX-BOLT-001. | largest difference 6.3e-8 | yes |
| FX-LOPAC-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LOPAC-006 frame 0: FX-BOLT-001's settings at the tutorial's core 75 and glow 50: the same bolt, its core and glow both dimmer. | largest difference 3.6e-8 | yes |
| FX-LOPAC-006 frame 2: FX-BOLT-001's settings at the tutorial's core 75 and glow 50: the same bolt, its core and glow both dimmer. | largest difference 3.4e-8 | yes |
| FX-LOPAC-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LOPAC-007 frame 0: The wide line with the soft core (D-334), core opacity 50: half of FX-LCORE-001's covering, row by row. | largest difference 2.5e-8 | yes |
| FX-LOPAC-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LOPAC-008 frame 0: The glowing line with core opacity keyed from 0 at frame 0 to 100 at frame 4: frame 0 is FX-LOPAC-003, frame 4 is the plain line, frame 2 between. | largest difference 4.5e-8 | yes |
| FX-LOPAC-008 frame 2: The glowing line with core opacity keyed from 0 at frame 0 to 100 at frame 4: frame 0 is FX-LOPAC-003, frame 4 is the plain line, frame 2 between. | largest difference 4.5e-8 | yes |
| FX-LOPAC-008 frame 4: The glowing line with core opacity keyed from 0 at frame 0 to 100 at frame 4: frame 0 is FX-LOPAC-003, frame 4 is the plain line, frame 2 between. | largest difference 5.1e-8 | yes |
| FX-LOPAC-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LOPAC-009 frame 0: A core opacity of 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LOPAC-009 frame 4: A core opacity of 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LOPAC-009: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LOPAC-010 frame 0: A glow opacity of -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LOPAC-010 frame 4: A glow opacity of -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LOPAC-010: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lopac_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lopac_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lopac_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lopac_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lopac_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lopac_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lopac_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lopac_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lopac_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lopac_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_001.json, a file from before D-1051, is saved without either opacity | None None | yes |
| fx_lopac_009.json's core opacity of 101 is named in a sentence | Lightning Bolt's core opacity runs from 0 to 100, and this is 101. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| core opacity 101 is refused with a sentence, and nothing changes | Lightning Bolt's core opacity runs from 0 to 100, and this is 101. | yes |
| glow opacity -1 is refused with a sentence, and nothing changes | Lightning Bolt's glow opacity runs from 0 to 100, and this is -1. | yes |
| core 75 and glow 50, is taken | taken | yes |
| core opacity keyed 0 to 100, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The graphics card

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lopac_005.json: both at 100, handed to the card as before | handed: true | yes |
| fx_lopac_006.json: an opacity below 100, kept on the processor | handed: false | yes |
| fx_lopac_008.json: an opacity below 100, kept on the processor | handed: false | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lopac_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lopac_008.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: the tutorial's retro bolt, in `verification/D-1051 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_full.png, core and glow at 100, the bolt before D-1051: a wide, strong pink glow, draws cleanly and changes some pixels | [], 36605 changed | yes |
| 2_tutorial.png, the tutorial's core 75 and glow 50: the same bolt, the glow faint, the core a little see-through, draws cleanly and changes some pixels | [], 36298 changed | yes |
| 3_glow_only.png, core 0, glow 100: no white core, the pink glow alone, draws cleanly and changes some pixels | [], 36605 changed | yes |
| the tutorial's settings are never brighter than full, and core 0 leaves no pure white | pure white: full 1590, tutorial 0, glow only 0 | yes |

## Result

52 of 52 checks pass.
