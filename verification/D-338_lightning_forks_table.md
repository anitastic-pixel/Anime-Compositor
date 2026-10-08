# B-218: Lightning Bolt's long forks

D-338, from P-26's tutorial 2. Every expected pixel is `Fixtures/lightning_forks/expected_lightning_forks.json`, written by `tools/lightning_forks_reference.py` before this code existed and printed in document 25 as FX-LFORK-001 to 008. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5. D-190's, D-324's, D-329's and D-334's own cases are checked again, unchanged, by B-126, B-203, B-211 and B-215.

## FX-LFORK-001 to 008 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LFORK-001 frame 0: A jagged bolt from the middle of the top edge to the middle of the bottom, width 1, no glow, branches 100, forks Long: the seven forks of the first three halvings each run on to the bottom edge beside the main bolt, turned 10 to 30 degrees from straight down. | largest difference 4.6e-8 | yes |
| FX-LFORK-001 frame 1: A jagged bolt from the middle of the top edge to the middle of the bottom, width 1, no glow, branches 100, forks Long: the seven forks of the first three halvings each run on to the bottom edge beside the main bolt, turned 10 to 30 degrees from straight down. | largest difference 2.9e-8 | yes |
| FX-LFORK-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LFORK-002 frame 0: FX-LFORK-001 with forks written "short": D-324's forks, exactly FX-LIGHTX-005's bolt with branches 100. | largest difference 4.6e-8 | yes |
| FX-LFORK-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LFORK-003 frame 0: FX-LFORK-001 with decay 50: each long fork goes half of what is left of the way down and thins to half its starting weight. | largest difference 3.2e-8 | yes |
| FX-LFORK-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LFORK-004 frame 0: FX-LFORK-001 over a drawing whose rows 7 to 9 are ground, Alpha Obstacle 50: every strand stops where it reaches the ground, several of them on row 7, and row 9 is untouched. Tutorial 2's strands touching down. | largest difference 4.2e-8 | yes |
| FX-LFORK-004 frame 1: FX-LFORK-001 over a drawing whose rows 7 to 9 are ground, Alpha Obstacle 50: every strand stops where it reaches the ground, several of them on row 7, and row 9 is untouched. Tutorial 2's strands touching down. | largest difference 2.5e-8 | yes |
| FX-LFORK-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LFORK-005 frame 0: FX-LFORK-001 as Strike: the long forks leave along the main bolt's own way too, so they are FX-LFORK-001's; only the short ones turn from the way to the end point. | largest difference 4.6e-8 | yes |
| FX-LFORK-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LFORK-006 frame 0: FX-BOLT-001's settings (branches 30) with forks Long: the same main bolt, the forks of its first three halvings longer. | largest difference 5.1e-8 | yes |
| FX-LFORK-006 frame 2: FX-BOLT-001's settings (branches 30) with forks Long: the same main bolt, the forks of its first three halvings longer. | largest difference 6.3e-8 | yes |
| FX-LFORK-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LFORK-007 frame 0: Forks "Long": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LFORK-007 frame 4: Forks "Long": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LFORK-007: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LFORK-008 frame 0: Forks "many", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LFORK-008 frame 4: Forks "many", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LFORK-008: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lfork_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lfork_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lfork_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lfork_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lfork_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lfork_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lfork_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lfork_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bolt_001.json, a file from before D-338, is saved without the word forks | None | yes |
| fx_lfork_008.json's forks "many" is named in a sentence | Lightning Bolt's forks are "short", "long" or "full", and this is "many". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| forks "many" is refused with a sentence, and nothing changes | Lightning Bolt's forks are "short", "long" or "full", and this is "many". | yes |
| forks short, is taken | taken | yes |
| forks long, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lfork_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lfork_004.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lfork_006.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a night sky over the ground, in `verification/D-338 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the sky over the ground, draws cleanly | [] | yes |
| f0_short.png, frame 0, forks Short, D-190's: the forks are short twigs off the main bolt | [], 1 strands touch the ground, 0 pixels lit more than 12 into it | yes |
| f0_long.png, frame 0, forks Long: the first forks run on down beside the main bolt to the ground | [], 2 strands touch the ground, 0 pixels lit more than 12 into it | yes |
| on frame 0, more strands reach the ground with long forks than with short | short 1, long 2 | yes |
| f7_short.png, frame 7, forks Short, D-190's: the forks are short twigs off the main bolt | [], 2 strands touch the ground, 0 pixels lit more than 12 into it | yes |
| f7_long.png, frame 7, forks Long: the first forks run on down beside the main bolt to the ground | [], 4 strands touch the ground, 0 pixels lit more than 12 into it | yes |
| on frame 7, more strands reach the ground with long forks than with short | short 2, long 4 | yes |

## Result

45 of 45 checks pass.
