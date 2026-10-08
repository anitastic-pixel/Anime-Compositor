# B-219: Lightning Bolt's full-width long forks

D-339, from P-26's tutorial 2. Every expected pixel is `Fixtures/lightning_full_forks/expected_lightning_full_forks.json`, written by `tools/lightning_full_forks_reference.py` before this code existed and printed in document 25 as FX-LFULL-001 to 006. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5. D-338's own cases are checked again, unchanged, by B-218.

## FX-LFULL-001 to 006 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LFULL-001 frame 0: FX-LFORK-001's bolt at width 2 with forks Full: the same seven long forks running to the bottom edge, each starting as wide as the main bolt where it leaves it, where Long starts them at half. | largest difference 6.3e-8 | yes |
| FX-LFULL-001 frame 1: FX-LFORK-001's bolt at width 2 with forks Full: the same seven long forks running to the bottom edge, each starting as wide as the main bolt where it leaves it, where Long starts them at half. | largest difference 5.1e-8 | yes |
| FX-LFULL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LFULL-002 frame 0: FX-LFULL-001 with forks Long, for comparison: the same strands, half as wide. | largest difference 5.9e-8 | yes |
| FX-LFULL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LFULL-003 frame 0: FX-LFORK-004 with forks Full: over the ground drawing, Alpha Obstacle 50, width 1, the strands stop where they reach the ground, and rows 8 and 9 are untouched. | largest difference 4.2e-8 | yes |
| FX-LFULL-003 frame 1: FX-LFORK-004 with forks Full: over the ground drawing, Alpha Obstacle 50, width 1, the strands stop where they reach the ground, and rows 8 and 9 are untouched. | largest difference 4.3e-8 | yes |
| FX-LFULL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LFULL-004 frame 0: FX-BOLT-001's settings with forks Full: the same main bolt as FX-BOLT-001, the first forks long and as wide as the main bolt. | largest difference 5.1e-8 | yes |
| FX-LFULL-004 frame 2: FX-BOLT-001's settings with forks Full: the same main bolt as FX-BOLT-001, the first forks long and as wide as the main bolt. | largest difference 6.3e-8 | yes |
| FX-LFULL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LFULL-005 frame 0: FX-LFULL-001 as Breaking, whose forks already start at the main bolt's weight: Full draws exactly what Long does. | largest difference 6.3e-8 | yes |
| FX-LFULL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LFULL-006 frame 0: Forks "Full": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LFULL-006 frame 4: Forks "Full": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-LFULL-006: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lfull_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lfull_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lfull_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lfull_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lfull_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lfull_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lfull_006.json's forks "Full" is named in a sentence | Lightning Bolt's forks are "short", "long" or "full", and this is "Full". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| forks "Full" is refused with a sentence, and nothing changes | Lightning Bolt's forks are "short", "long" or "full", and this is "Full". | yes |
| forks long, is taken | taken | yes |
| forks full, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lfull_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lfull_003.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lfull_004.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a night sky over the ground, in `verification/D-339 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| the sky over the ground draws cleanly | [] | yes |
| f0_long.png, frame 0, forks Long, D-338's: the strands start at half the main bolt's width | [], 940 white pixels in the sky, 0 pixels lit more than 12 into the ground | yes |
| f0_full.png, frame 0, forks Long, full width: the strands start as wide as the main bolt | [], 1062 white pixels in the sky, 0 pixels lit more than 12 into the ground | yes |
| on frame 0, the full-width strands cover more of the sky in white than the long ones | long 940, full 1062 | yes |
| f7_long.png, frame 7, forks Long, D-338's: the strands start at half the main bolt's width | [], 1094 white pixels in the sky, 0 pixels lit more than 12 into the ground | yes |
| f7_full.png, frame 7, forks Long, full width: the strands start as wide as the main bolt | [], 1457 white pixels in the sky, 0 pixels lit more than 12 into the ground | yes |
| on frame 7, the full-width strands cover more of the sky in white than the long ones | long 1094, full 1457 | yes |

## Result

37 of 37 checks pass.
