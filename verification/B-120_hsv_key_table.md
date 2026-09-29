# B-120: HSV key

D-184, accepted by the owner on 2026-09-28. Every expected pixel is `Fixtures/hsv_key/expected_hsv_key.json`, written by `tools/hsv_key_reference.py` before this code existed and printed in document 25 as FX-HSV-001 to 016. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-HSV-001 to 016 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-HSV-001 frame 0: As it is added, a green key: hue 120 within 40, saturation 60 within 40, value 60 within 40. The screen, its half-covering right edge, the shadow and the green spill down the figure's left side go; the skin, the line and the grey button stay. | largest difference 1.9e-7 | yes |
| FX-HSV-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HSV-002 frame 0: Hue 142 within 5, saturation 100 within 10, value 50 within 50: the screen, its edge and the shadow go; the spill, 13.4 degrees of hue away, stays. | largest difference 1.9e-7 | yes |
| FX-HSV-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HSV-003 frame 0: FX-HSV-002 with value 70 within 5: the screen, value 69.4, goes; the shadow, value 39.2, stays. | largest difference 1.9e-7 | yes |
| FX-HSV-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HSV-004 frame 0: FX-HSV-002 inverted: the screen, its edge and the shadow stay, and everything else that shows goes, the figure and the spill. | largest difference 6.7e-8 | yes |
| FX-HSV-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HSV-005 frame 0: Hue 350 within 40, saturation 55 within 50, value 50 within 50: the skin, hue 25.7, is 35.7 degrees away the short way round, past 360, and goes. The grey button's hue counts as 0, but its saturation 0 is 55 from 55; it stays, and so does everything else. | largest difference 9.8e-8 | yes |
| FX-HSV-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HSV-006 frame 0: Hue 0 within 0, saturation 0 within 0, value 50 within 5: only the grey button, value 50.2, goes. | largest difference 1.9e-7 | yes |
| FX-HSV-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HSV-007 frame 0: Hue within 180, saturation 50 within 50, value 50 within 50: every colour is inside, so the frame is empty. | largest difference 0.0e0 | yes |
| FX-HSV-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HSV-008 frame 0: OpenToonz's own starting values, everything 0: only pure black would go, and the drawing has none that shows, so it is untouched. | largest difference 1.9e-7 | yes |
| FX-HSV-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HSV-009 frame 0: Hue 142, saturation 70 within 30, value 50 within 50, hue range keyed from 0 at frame 0 to 20 at frame 4, linear: frame 0 takes nothing, the screen being 0.31 degrees off; frame 2, range 10, takes the screen, its edge and the shadow; frame 4, range 20, the spill too. | largest difference 1.9e-7 | yes |
| FX-HSV-009 frame 2: Hue 142, saturation 70 within 30, value 50 within 50, hue range keyed from 0 at frame 0 to 20 at frame 4, linear: frame 0 takes nothing, the screen being 0.31 degrees off; frame 2, range 10, takes the screen, its edge and the shadow; frame 4, range 20, the spill too. | largest difference 1.9e-7 | yes |
| FX-HSV-009 frame 4: Hue 142, saturation 70 within 30, value 50 within 50, hue range keyed from 0 at frame 0 to 20 at frame 4, linear: frame 0 takes nothing, the screen being 0.31 degrees off; frame 2, range 10, takes the screen, its edge and the shadow; frame 4, range 20, the spill too. | largest difference 1.9e-7 | yes |
| FX-HSV-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HSV-010 frame 0: FX-HSV-002 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-HSV-010 frame 3: FX-HSV-002 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-HSV-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HSV-011 frame 0: Hue 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HSV-011 frame 4: Hue 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HSV-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HSV-012 frame 0: Hue range 181, above 180. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HSV-012 frame 4: Hue range 181, above 180. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HSV-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HSV-013 frame 0: Saturation 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HSV-013 frame 4: Saturation 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HSV-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HSV-014 frame 0: Value range -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HSV-014 frame 4: Value range -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HSV-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HSV-015 frame 0: Invert "yes", which is not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HSV-015 frame 4: Invert "yes", which is not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HSV-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HSV-016 frame 0: Hue range keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HSV-016 frame 4: Hue range keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HSV-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview keeps every setting, none being a distance | HsvKey { hue: 120.0, saturation: 60.0, value: 60.0, hue_range: 40.0, saturation_range: 40.0, value_range: 40.0, invert: "on" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_hsv_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hsv_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hsv_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hsv_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hsv_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hsv_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `invert` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a hue that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| hue 361 is refused with a sentence, and nothing changes | HSV Key's hue runs from 0 to 360, and this is 361. | yes |
| hue range 181 is refused with a sentence, and nothing changes | HSV Key's hue range runs from 0 to 180, and this is 181. | yes |
| saturation 101 is refused with a sentence, and nothing changes | HSV Key's saturation runs from 0 to 100, and this is 101. | yes |
| value range -1 is refused with a sentence, and nothing changes | HSV Key's value range runs from 0 to 100, and this is -1. | yes |
| invert "yes" is refused with a sentence, and nothing changes | HSV Key's invert is "off" or "on", and this is "yes". | yes |
| hue range keyed to 200 is refused with a sentence, and nothing changes | HSV Key's hue range runs from 0 to 180, and this is 200. | yes |
| every setting at the top of its range, is taken | taken | yes |
| every setting at the bottom, is taken | taken | yes |
| hue keyed from 0 to 360 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a figure in front of a green screen, in `verification/B-120 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the plate with no effect, draws cleanly | [] | yes |
| as_added.png, as it is added, draws cleanly, takes out [(10, 10), (10, 90), (61, 70)] and keeps [(80, 30), (90, 75)] whole | [], covering [0, 0, 0] and [255, 255] | yes |
| narrow.png, hue 142 within 5, saturation 100 within 10, value 50 within 50, draws cleanly, takes out [(10, 10), (10, 90)] and keeps [(61, 70), (80, 30), (90, 75)] whole | [], covering [0, 0] and [255, 255, 255] | yes |
| inverted.png, the same inverted, draws cleanly, takes out [(61, 70), (80, 30), (90, 75)] and keeps [(10, 10), (10, 90)] whole | [], covering [0, 0, 0] and [255, 255] | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_hsv_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_hsv_010.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

67 of 67 checks pass.
