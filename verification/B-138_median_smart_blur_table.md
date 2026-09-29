# B-138: Median and Smart Blur

D-203, accepted on 2026-09-28 with the After Effects picks (B1). Every expected pixel is `Fixtures/median_smart_blur/expected_median_smart_blur.json`, written by `tools/median_smart_blur_reference.py` before this code existed and printed in document 25 as FX-MEDIAN-001 to 011 and FX-SMART-001 to 012. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-MEDIAN-001 to 011 and FX-SMART-001 to 012 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MEDIAN-001 frame 0: Median as it starts, Radius 2 and Operate on Alpha off: the three specks are gone into the skin and the grain is mostly skin, while the line, two pixels wide, is kept, and so is the drawing's outline: the hole stays a hole and the half-covered column stays at half. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MEDIAN-002 frame 0: Radius 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MEDIAN-003 frame 0: Radius 1, a disc of five, the pixel and its four neighbours: the specks are gone and the line is kept. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MEDIAN-004 frame 0: Radius 2 with Operate on Alpha on: the specks are gone and the hole is filled with skin; the drawing's top-left corner, with more of its disc outside the drawing than in, is cut away, and the half-covered column takes its covering from its neighbours. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MEDIAN-005 frame 0: Radius keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing and frame 2 is FX-MEDIAN-001. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-005 frame 2: Radius keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing and frame 2 is FX-MEDIAN-001. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-005 frame 4: Radius keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing and frame 2 is FX-MEDIAN-001. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MEDIAN-006 frame 0: Radius keyed from 0 at frame 0 to 10 at frame 4, eased past its end (about 13 at frame 2): frame 2 is held at 10, the same as frame 4, where every pixel that shows is the skin at its own covering: the line, the specks and the grain are all gone. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-006 frame 2: Radius keyed from 0 at frame 0 to 10 at frame 4, eased past its end (about 13 at frame 2): frame 2 is held at 10, the same as frame 4, where every pixel that shows is the skin at its own covering: the line, the specks and the grain are all gone. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-006 frame 4: Radius keyed from 0 at frame 0 to 10 at frame 4, eased past its end (about 13 at frame 2): frame 2 is held at 10, the same as frame 4, where every pixel that shows is the skin at its own covering: the line, the specks and the grain are all gone. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MEDIAN-007 frame 0: Median as it starts, the layer moved three pixels right: FX-MEDIAN-001 moved with it. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MEDIAN-008 frame 0: Median, Radius 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-008 frame 4: Median, Radius 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-008: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MEDIAN-009 frame 0: Median, Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-009 frame 4: Median, Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-009: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MEDIAN-010 frame 0: Median, Radius keyed to 20 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-010 frame 4: Median, Radius keyed to 20 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-010: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MEDIAN-011 frame 0: Median, Operate on Alpha "sometimes", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-011 frame 4: Median, Operate on Alpha "sometimes", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MEDIAN-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMART-001 frame 0: Smart Blur as it starts, Radius 3 and Threshold 64: the grain, never more than 16 apart, is smoothed toward the skin, while the line and the dark specks, far more than 64 from the skin, are kept sharp, and so is the white speck, 65 from the skin in its blue. The hole stays a hole and the half-covered column, 127 from the skin in its covering, is kept apart. | largest difference 2.0e-7 | yes |
| FX-SMART-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMART-002 frame 0: Radius 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SMART-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMART-003 frame 0: Threshold 0: only the very same colour is mixed, so the drawing is unchanged. | largest difference 1.9e-7 | yes |
| FX-SMART-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMART-004 frame 0: Threshold 8: grain 8 from the skin mixes with the skin but not with grain 8 the other way, so the grain is only partly smoothed. | largest difference 2.0e-7 | yes |
| FX-SMART-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMART-005 frame 0: Threshold 255: every tap that shows counts, a plain disc blur that stays inside the drawing: the line and the specks are blurred into the skin, and the hole, the empty column and the empty row stay empty. | largest difference 2.0e-7 | yes |
| FX-SMART-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMART-006 frame 0: Threshold keyed from 0 at frame 0 to 16 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-SMART-004 at 8, and frame 4 is FX-SMART-001, for nothing in the drawing lies between 17 and 64 apart. | largest difference 1.9e-7 | yes |
| FX-SMART-006 frame 2: Threshold keyed from 0 at frame 0 to 16 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-SMART-004 at 8, and frame 4 is FX-SMART-001, for nothing in the drawing lies between 17 and 64 apart. | largest difference 2.0e-7 | yes |
| FX-SMART-006 frame 4: Threshold keyed from 0 at frame 0 to 16 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-SMART-004 at 8, and frame 4 is FX-SMART-001, for nothing in the drawing lies between 17 and 64 apart. | largest difference 2.0e-7 | yes |
| FX-SMART-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMART-007 frame 0: Radius keyed from 0 at frame 0 to 10 at frame 4, eased past its end (about 13 at frame 2): frame 2 is held at 10, the same as frame 4. | largest difference 1.9e-7 | yes |
| FX-SMART-007 frame 2: Radius keyed from 0 at frame 0 to 10 at frame 4, eased past its end (about 13 at frame 2): frame 2 is held at 10, the same as frame 4. | largest difference 2.0e-7 | yes |
| FX-SMART-007 frame 4: Radius keyed from 0 at frame 0 to 10 at frame 4, eased past its end (about 13 at frame 2): frame 2 is held at 10, the same as frame 4. | largest difference 2.0e-7 | yes |
| FX-SMART-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMART-008 frame 0: Smart Blur as it starts, the layer moved three pixels right: FX-SMART-001 moved with it. | largest difference 2.0e-7 | yes |
| FX-SMART-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SMART-009 frame 0: Smart Blur, Radius 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMART-009 frame 4: Smart Blur, Radius 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMART-009: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMART-010 frame 0: Smart Blur, Threshold 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMART-010 frame 4: Smart Blur, Threshold 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMART-010: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMART-011 frame 0: Smart Blur, Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMART-011 frame 4: Smart Blur, Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMART-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SMART-012 frame 0: Smart Blur, Threshold keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMART-012 frame 4: Smart Blur, Threshold keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SMART-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far they reach

| Check | The build's answer | Matches |
| --- | --- | --- |
| neither ever grows the layer: each declares no growth | [0, 0] | yes |
| a half-size draft preview halves Median's radius, 4 to 2, and keeps the word | Median { radius: 2.0, operate_on_alpha: "on" } | yes |
| a half-size draft preview halves Smart Blur's radius, 3 to 1.5, and keeps the threshold, which is colour, not distance | SmartBlur { radius: 1.5, threshold: 64.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_median_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_median_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_median_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_median_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_median_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_median_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_median_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smart_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smart_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smart_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smart_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smart_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smart_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_smart_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with a Median with no `radius` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Median with no `operate_on_alpha` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Median whose operate on alpha is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Smart Blur with no `threshold` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Smart Blur whose threshold is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Median radius 10.5 is refused with a sentence, and nothing changes | Median's radius runs from 0 to 10, and this is 10.5. | yes |
| Median radius -0.5 is refused with a sentence, and nothing changes | Median's radius runs from 0 to 10, and this is -0.5. | yes |
| operate on alpha "sometimes" is refused with a sentence, and nothing changes | Median's operate on alpha is "off" or "on", and this is "sometimes". | yes |
| operate on alpha "On", written with a capital is refused with a sentence, and nothing changes | Median's operate on alpha is "off" or "on", and this is "On". | yes |
| Median radius keyed to 20 is refused with a sentence, and nothing changes | Median's radius runs from 0 to 10, and this is 20. | yes |
| Median radius 0, is taken | taken | yes |
| operate on alpha on, is taken | taken | yes |
| Median radius keyed from 0 to 10 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |
| Smart Blur radius 11 is refused with a sentence, and nothing changes | Smart Blur's radius runs from 0 to 10, and this is 11. | yes |
| threshold 256 is refused with a sentence, and nothing changes | Smart Blur's threshold runs from 0 to 255, and this is 256. | yes |
| threshold -1 is refused with a sentence, and nothing changes | Smart Blur's threshold runs from 0 to 255, and this is -1. | yes |
| threshold keyed to 300 is refused with a sentence, and nothing changes | Smart Blur's threshold runs from 0 to 255, and this is 300. | yes |
| threshold 0, is taken | taken | yes |
| threshold 255, is taken | taken | yes |
| threshold keyed from 0 to 64 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a scanned face cleaned and smoothed, in `verification/B-138 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the face with no effect: specks off the skin, pinholes, grain; draws cleanly | [], 60 pixels off the skin, 21 pinholes, grain 20.7 | yes |
| median_2.png, Median at its default radius of 2: no white or dark speck left on the skin, the pinholes still clear as the covering is each pixel's own, the line round the face and the eyes kept, at least 19 of their pixels in 20 still line-dark, and the one-pixel hair taken away, as a line thinner than the radius is, wherever it is more than three pixels from the line it runs into; draws cleanly | [], 21 off the skin of which 21 pinholes, line 720 of 720, hair 0 of 34 | yes |
| median_2_alpha_on.png, Median 2 with Operate on Alpha on: the pinholes filled too, nothing off the skin and nothing clear inside the line; nothing spills past the face; draws cleanly | [], 0 off the skin, 0 pinholes, 0 spilt | yes |
| smart_blur_64.png, Smart Blur at its defaults, radius 3 and threshold 64: the grain less than half what it was, and every pixel of the line and the eyes exactly as it was, as the skin is too far from them to be mixed in; draws cleanly | [], grain 2.2 from 20.7, line 720 of 720 unchanged | yes |
| smart_blur_255.png, Smart Blur at threshold 255 mixes everything that shows, a plain blur inside the drawing: the line softened into the skin, fewer than 9 of its pixels in 10 still line-dark, and still nothing past the face's edge, as clear pixels are never mixed in; draws cleanly | [], line 48 of 720 still dark, 0 spilt | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_median_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_median_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_median_005.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_smart_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_smart_007.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

111 of 111 checks pass.
