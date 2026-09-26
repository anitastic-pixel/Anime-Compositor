# B-54: curves

D-111, accepted by the owner on 2026-09-26 in the batch of ten. Every expected pixel is `Fixtures/curves/expected_curves.json`, written by `tools/curves_reference.py` before this code existed and printed in document 25 as FX-CURVES-001 to 020. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-CURVES-001 to 020 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CURVES-001 frame 0: All four curves the default, [[0, 0], [255, 255]]: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-CURVES-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURVES-002 frame 0: Master through [[0, 0], [128, 180], [255, 255]]: every colour is lightened, the middle greys most; black and white stay, and so do the empty pixels. | largest difference 1.1e-7 | yes |
| FX-CURVES-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURVES-003 frame 0: Red alone through an S, [[0, 0], [64, 40], [192, 215], [255, 255]]: only the red channel moves, darker below the middle and lighter above; green and blue stay exactly as drawn. | largest difference 1.5e-7 | yes |
| FX-CURVES-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURVES-004 frame 0: Master through [[64, 32], [192, 224]]: flat outside its end points, so the ramp's greys at or below 64 all become 32 and those at or above 192 all become 224; between, the straight line. | largest difference 2.4e-7 | yes |
| FX-CURVES-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURVES-005 frame 0: Master through [[0, 0], [64, 255], [128, 255], [255, 0]]: the spline rises above 255 between 64 and 128 and is held at 255 there. | largest difference 2.3e-7 | yes |
| FX-CURVES-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURVES-006 frame 0: Master through 16 points, the most allowed, alternating 0 and 255 every 17: the ramp's greys land wherever the zigzag puts them; its 16 swings past 255 and its 240 below 0, each held. | largest difference 2.2e-6 | yes |
| FX-CURVES-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURVES-007 frame 0: Red through [[0, 0], [255, 128]], halving it, and master through [[0, 0], [128, 255]], doubling: the channel's own curve runs first, so red comes back as drawn while green and blue double, held at 255. | largest difference 1.9e-7 | yes |
| FX-CURVES-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURVES-008 frame 0: Master through [[0, 255], [255, 0]], inverting: every colour that shows is inverted at its own covering, the soft pixels still soft; the empty pixels stay empty, not white. | largest difference 5.2e-8 | yes |
| FX-CURVES-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURVES-009 frame 0: Green alone through [[0, 0], [255, 128]]: only green is halved. | largest difference 1.9e-7 | yes |
| FX-CURVES-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURVES-010 frame 0: Blue alone through [[0, 64], [255, 255]]: only blue is lifted, black's blue to 64. | largest difference 1.9e-7 | yes |
| FX-CURVES-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURVES-011 frame 0: All four at once: master FX-CURVES-002's lift, red the S, green halved, blue inverted. | largest difference 1.2e-7 | yes |
| FX-CURVES-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURVES-012 frame 0: Master through three points on the straight line, [[0, 0], [128, 128], [255, 255]]: not the default as written, so it is worked, but the spline through them is the line, so the drawing. | largest difference 1.9e-7 | yes |
| FX-CURVES-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURVES-013 frame 0: Master through [[0, 0], [127.5, 200.25], [255, 255]], points written with fractions: a lift a little stronger than FX-CURVES-002's. | largest difference 1.1e-7 | yes |
| FX-CURVES-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURVES-014 frame 0: FX-CURVES-011 moved three pixels right: the same, moved. | largest difference 1.2e-7 | yes |
| FX-CURVES-014 frame 3: FX-CURVES-011 moved three pixels right: the same, moved. | largest difference 1.2e-7 | yes |
| FX-CURVES-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURVES-015 frame 0: Master of one point, [[128, 128]], fewer than two. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURVES-015 frame 4: Master of one point, [[128, 128]], fewer than two. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURVES-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURVES-016 frame 0: Red of 17 points, more than sixteen. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURVES-016 frame 4: Red of 17 points, more than sixteen. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURVES-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURVES-017 frame 0: Green with an out of 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURVES-017 frame 4: Green with an out of 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURVES-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURVES-018 frame 0: Blue with an in of -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURVES-018 frame 4: Blue with an in of -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURVES-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURVES-019 frame 0: Master with two points at in 128, so in does not strictly increase. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURVES-019 frame 4: Master with two points at in 128, so in does not strictly increase. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURVES-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURVES-020 frame 0: Master with a point of three numbers, [128, 128, 128]. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURVES-020 frame 4: Master with a point of three numbers, [128, 128, 128]. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURVES-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: each pixel is regraded where it is | 0 | yes |
| a half-size draft preview changes nothing: a curve is colours, not distances | Curves { master: [[0.0, 0.0], [128.0, 180.0], [255.0, 255.0]], red: [[0.0, 0.0], [255.0, 255.0]], green: [[0.0, 0.0], [255.0, 255.0]], blue: [[0.0, 0.0], [255.0, 255.0]] } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_curves_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curves_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curves_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curves_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curves_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curves_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curves_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curves_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curves_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curves_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curves_013.json's points written with fractions are saved with them | [[0,0],[127.5,200.25],[255,255]] | yes |
| a file with no `blue` curve at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a point's out written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a curve written as one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| a master of one point is refused with a sentence, and nothing changes | Curves' master curve takes 2 to 16 points, and this has 1. | yes |
| a master of 17 points is refused with a sentence, and nothing changes | Curves' master curve takes 2 to 16 points, and this has 17. | yes |
| an out of 256 is refused with a sentence, and nothing changes | Curves' master curve's points run from 0 to 255, and this has 256. | yes |
| an in of -1 is refused with a sentence, and nothing changes | Curves' master curve's points run from 0 to 255, and this has -1. | yes |
| two points at in 128 is refused with a sentence, and nothing changes | Curves' master curve's in goes up from point to point, and 128 is not above 128. | yes |
| a point of three numbers is refused with a sentence, and nothing changes | Each point of Curves' master curve is two numbers, in then out, and this one is [128.0, 128.0, 128.0]. | yes |
| the master curve keyed, which a list of points cannot be is refused with a sentence, and nothing changes | A core.curves has no setting called master. | yes |
| a master of 16 points, the most is taken | taken | yes |
| a master of 2 points at the corners of the range, inverting, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_curves_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_curves_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

75 of 75 checks pass.
