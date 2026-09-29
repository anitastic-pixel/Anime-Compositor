# B-121: paraffin

D-185, accepted by the owner on 2026-09-28. Every expected pixel is `Fixtures/paraffin/expected_paraffin.json`, written by `tools/paraffin_reference.py` before this code existed and printed in document 25 as FX-PARA-001 to 023. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-PARA-001 to 023 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PARA-001 frame 0: The settings as they start: #6450a0 from below (direction 180), spread 70, opacity 50, multiply. The block's bottom row, row 8, is darkened toward violet the most, each row above less, and rows 1 and 2, more than 70 per cent of the figure's eight rows up, not at all; the dot at (14, 4) is darkened as row 4 is; the empty pixels stay empty. | largest difference 1.9e-7 | yes |
| FX-PARA-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PARA-002 frame 0: From below, spread 100, opacity 100, normal: row 8 wholly violet at each pixel's own covering, and each row above less violet on the smooth curve, row 1 by 1 - s s (3 - 2 s) at s = 7/8, about 0.043. | largest difference 2.0e-7 | yes |
| FX-PARA-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PARA-003 frame 0: Direction 0, from above, the rest as FX-PARA-002: turned upside down, row 1 wholly violet and row 8 the least. | largest difference 2.0e-7 | yes |
| FX-PARA-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PARA-004 frame 0: Direction 90, from the right, the rest as FX-PARA-002: the figure reaches from column 0 to the dot at column 14, so the dot is wholly violet and the block less violet toward the left; the quarter-covered column 11, not part of the figure, is shaded as its place says. | largest difference 1.9e-7 | yes |
| FX-PARA-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PARA-005 frame 0: Spread 50, the rest as FX-PARA-002: the wash stops half way up the figure; rows 1 to 4 and the dot are untouched. | largest difference 1.9e-7 | yes |
| FX-PARA-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PARA-006 frame 0: Direction 45, from the upper right, the rest as FX-PARA-002: the dot and the block's top right corner the most violet, the bottom left the least. | largest difference 1.9e-7 | yes |
| FX-PARA-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PARA-007 frame 0: Overlay, spread 100, opacity 100: the skin, light, is lifted toward the violet's screen and its shadow, darker, pushed toward its multiply; the most at row 8. | largest difference 2.4e-7 | yes |
| FX-PARA-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PARA-008 frame 0: Soft light, spread 100, opacity 100: gentler than overlay; each channel whose violet is below one half is darkened, blue is lifted. | largest difference 2.3e-7 | yes |
| FX-PARA-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PARA-009 frame 0: Multiply, spread 100, opacity 100: darkened toward violet, the most at row 8. | largest difference 1.5e-7 | yes |
| FX-PARA-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PARA-010 frame 0: Screen, spread 100, opacity 100: lightened, none darkened. | largest difference 1.9e-7 | yes |
| FX-PARA-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PARA-011 frame 0: Add, spread 100, opacity 100: the violet added and held at 1, so the skin's red, already near 1, stops at 1. | largest difference 2.2e-7 | yes |
| FX-PARA-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PARA-012 frame 0: Spread 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-PARA-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PARA-013 frame 0: Opacity 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-PARA-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PARA-014 frame 0: FX-PARA-001 with the colour written in capitals, #6450A0: the same. | largest difference 1.9e-7 | yes |
| FX-PARA-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PARA-015 frame 0: Direction keyed from 0 at frame 0 to 180 at frame 4, linear, the rest as FX-PARA-002: frame 0 is FX-PARA-003, frame 2 is FX-PARA-004 and frame 4 is FX-PARA-002: the wash swings round the figure. | largest difference 2.0e-7 | yes |
| FX-PARA-015 frame 2: Direction keyed from 0 at frame 0 to 180 at frame 4, linear, the rest as FX-PARA-002: frame 0 is FX-PARA-003, frame 2 is FX-PARA-004 and frame 4 is FX-PARA-002: the wash swings round the figure. | largest difference 1.9e-7 | yes |
| FX-PARA-015 frame 4: Direction keyed from 0 at frame 0 to 180 at frame 4, linear, the rest as FX-PARA-002: frame 0 is FX-PARA-003, frame 2 is FX-PARA-004 and frame 4 is FX-PARA-002: the wash swings round the figure. | largest difference 2.0e-7 | yes |
| FX-PARA-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PARA-016 frame 0: Spread eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots, opacity 100, normal: at frame 2 it would pass 100, is held at 100, and is FX-PARA-002; frame 0 is the drawing. | largest difference 1.9e-7 | yes |
| FX-PARA-016 frame 2: Spread eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots, opacity 100, normal: at frame 2 it would pass 100, is held at 100, and is FX-PARA-002; frame 0 is the drawing. | largest difference 2.0e-7 | yes |
| FX-PARA-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PARA-017 frame 0: FX-PARA-001 moved three pixels right: the same, moved; the wash is worked on the drawing before it moves. | largest difference 1.9e-7 | yes |
| FX-PARA-017 frame 3: FX-PARA-001 moved three pixels right: the same, moved; the wash is worked on the drawing before it moves. | largest difference 1.9e-7 | yes |
| FX-PARA-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PARA-018 frame 0: Direction 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PARA-018 frame 4: Direction 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PARA-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PARA-019 frame 0: Spread 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PARA-019 frame 4: Spread 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PARA-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PARA-020 frame 0: Opacity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PARA-020 frame 4: Opacity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PARA-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PARA-021 frame 0: Spread keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PARA-021 frame 4: Spread keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PARA-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PARA-022 frame 0: Blend "color_burn", which is not a blend. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PARA-022 frame 4: Blend "color_burn", which is not a blend. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PARA-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PARA-023 frame 0: Colour "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PARA-023 frame 4: Colour "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PARA-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: the wash stays on the covering | 0 | yes |
| a half-size draft preview keeps every setting, none being a distance | Paraffin { color: "#6450a0", direction: 180.0, spread: 70.0, opacity: 50.0, blend: "multiply" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_para_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_para_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_para_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_para_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_para_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_para_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_para_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_para_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_para_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_para_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_para_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `spread` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a direction that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| direction 361 is refused with a sentence, and nothing changes | Paraffin's direction runs from 0 to 360, and this is 361. | yes |
| direction -1 is refused with a sentence, and nothing changes | Paraffin's direction runs from 0 to 360, and this is -1. | yes |
| spread 101 is refused with a sentence, and nothing changes | Paraffin's spread runs from 0 to 100, and this is 101. | yes |
| opacity 101 is refused with a sentence, and nothing changes | Paraffin's opacity runs from 0 to 100, and this is 101. | yes |
| blend "color_burn" is refused with a sentence, and nothing changes | Paraffin's blend is "normal", "multiply", "screen", "add", "overlay" or "soft_light", and this is "color_burn". | yes |
| colour "#12345" is refused with a sentence, and nothing changes | Paraffin's colour is written #rrggbb, and this is "#12345". | yes |
| spread keyed to 150 is refused with a sentence, and nothing changes | Paraffin's spread runs from 0 to 100, and this is 150. | yes |
| direction 360, spread 100 and opacity 100, the tops, soft light, is taken | taken | yes |
| direction 0, spread 0, opacity 0, overlay, is taken | taken | yes |
| direction keyed from 0 to 360 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a figure on a clear cel, in `verification/B-121 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the cel with no effect, draws cleanly | [] | yes |
| as_added.png, as it is added, violet multiplied up from below, draws cleanly, changes [(50, 58), (43, 110)] and leaves [(50, 9), (50, 27), (5, 5)] exactly | [], changed [true, true], left [true, true, true] | yes |
| warm_from_above.png, a warm light screened down from above, spread 50, opacity 70, draws cleanly, changes [(50, 9), (50, 27)] and leaves [(43, 110), (5, 5)] exactly | [], changed [true, true], left [true, true] | yes |
| blue_from_right.png, a blue shade multiplied in from the right, spread 60, opacity 60, draws cleanly, changes [(62, 58)] and leaves [(37, 85), (43, 110), (5, 5)] exactly | [], changed [true], left [true, true, true] | yes |
| soft_light_from_below.png, violet in soft light from below, spread 100, opacity 100, draws cleanly, changes [(43, 110), (50, 27)] and leaves [(5, 5)] exactly | [], changed [true, true], left [true] | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_para_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_para_017.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

89 of 89 checks pass.
