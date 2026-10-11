# D-500: Color Range

B-380, after After Effects' Color Range (Keying): the colours inside a box chosen in Lab, YUV or RGB keyed out, with a fuzziness that softens the box's edge. Adobe describes the effect and names its controls but publishes no formula and no defaults we found, so the rule and the settings when added are ours, stated in `tools/color_range_reference.py`: each pixel's three components on a 0 to 255 scale, how far the farthest lies outside the box, over the fuzziness, is the share of the pixel kept. Every expected pixel is `Fixtures/color_range/expected_color_range.json`, written by the tool before this code existed and printed in document 25 as FX-CRANGE-001 to 018. Tolerance 2e-5.

## FX-CRANGE-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CRANGE-001 frame 0: The settings as they start: Lab, every end of the box 0, fuzziness 0. No colour has a* or b* as low as -128, so nothing is keyed and the layer is as it was. | largest difference 1.2e-7 | yes |
| FX-CRANGE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRANGE-002 frame 0: Lab, a box about the green screen (L 120 to 200, a 40 to 100, b 150 to 220), fuzziness 0: the green screen's top four rows and pure green's middle rows, inside the same box, lose all their covering; everything else keeps all of it. | largest difference 1.2e-7 | yes |
| FX-CRANGE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRANGE-003 frame 0: The same box with fuzziness 30: the screen still all gone; colours just outside the box keep a share of their covering, d / 30. | largest difference 2.7e-7 | yes |
| FX-CRANGE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRANGE-004 frame 0: YUV, a box about the blue screen (Y 20 to 120, U 170 to 230, V 80 to 125), fuzziness 20: the blue screen falls, the half-covered one too; pure blue (U about 255) keeps all its covering and its darker rows fall; the violet keeps a share. | largest difference 2.7e-7 | yes |
| FX-CRANGE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRANGE-005 frame 0: RGB, a box about green (R 0 to 90, G 140 to 255, B 0 to 110), fuzziness 40: pure green and the green screen fall, cyan and yellow, far outside in blue or red, keep all their covering. | largest difference 5.3e-7 | yes |
| FX-CRANGE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRANGE-006 frame 0: RGB, the whole cube (every min 0, every max 255), fuzziness 1: every pixel is inside, so the whole layer is keyed out. | largest difference 0.0e0 | yes |
| FX-CRANGE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRANGE-007 frame 0: Lab with the whole range of L and b but a* only from -8 to 40 (a 120 to 168), fuzziness 2: the greys, black and white (a* 0), skin, the blue screen and the pale blue fall; the greens (a* below -8) and pure red, magenta and pure blue (above 40) keep their covering. | largest difference 2.8e-7 | yes |
| FX-CRANGE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRANGE-008 frame 0: RGB, a minimum above its maximum (red from 200 down to 100): no colour is inside, and with fuzziness 60 a pixel keeps the share of its distance past the nearer end, so mid reds lose some covering. | largest difference 2.4e-7 | yes |
| FX-CRANGE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRANGE-009 frame 0: Fuzziness 255 about the green screen in Lab: the softest edge; colours near green lose much of their covering, the far ones little. | largest difference 7.3e-8 | yes |
| FX-CRANGE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRANGE-010 frame 0: Fuzziness keyed from 0 at frame 0 to 80 at frame 4, linear, about the green screen in Lab. | largest difference 1.2e-7 | yes |
| FX-CRANGE-010 frame 2: Fuzziness keyed from 0 at frame 0 to 80 at frame 4, linear, about the green screen in Lab. | largest difference 2.6e-7 | yes |
| FX-CRANGE-010 frame 4: Fuzziness keyed from 0 at frame 0 to 80 at frame 4, linear, about the green screen in Lab. | largest difference 1.3e-7 | yes |
| FX-CRANGE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRANGE-011 frame 0: RGB, the top of green keyed from 255 at frame 0 to 150 at frame 4, the rest of FX-CRANGE-005: the box shrinks and fewer greens fall. | largest difference 5.3e-7 | yes |
| FX-CRANGE-011 frame 2: RGB, the top of green keyed from 255 at frame 0 to 150 at frame 4, the rest of FX-CRANGE-005: the box shrinks and fewer greens fall. | largest difference 5.3e-7 | yes |
| FX-CRANGE-011 frame 4: RGB, the top of green keyed from 255 at frame 0 to 150 at frame 4, the rest of FX-CRANGE-005: the box shrinks and fewer greens fall. | largest difference 5.3e-7 | yes |
| FX-CRANGE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRANGE-012 frame 0: FX-CRANGE-003 moved three pixels right: the same, moved. | largest difference 2.7e-7 | yes |
| FX-CRANGE-012 frame 3: FX-CRANGE-003 moved three pixels right: the same, moved. | largest difference 2.7e-7 | yes |
| FX-CRANGE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CRANGE-013 frame 0: Fuzziness 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CRANGE-013 frame 4: Fuzziness 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CRANGE-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CRANGE-014 frame 0: Fuzziness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CRANGE-014 frame 4: Fuzziness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CRANGE-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CRANGE-015 frame 0: Min (L, Y, R) 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CRANGE-015 frame 4: Min (L, Y, R) 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CRANGE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CRANGE-016 frame 0: Max (b, V, B) -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CRANGE-016 frame 4: Max (b, V, B) -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CRANGE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CRANGE-017 frame 0: Colour space "Lab": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CRANGE-017 frame 4: Colour space "Lab": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CRANGE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CRANGE-018 frame 0: Colour space "hsl": not one of Lab, YUV and RGB. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CRANGE-018 frame 4: Colour space "hsl": not one of Lab, YUV and RGB. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-CRANGE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_crange_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crange_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crange_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crange_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crange_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crange_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crange_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crange_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crange_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crange_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crange_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crange_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crange_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crange_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crange_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crange_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crange_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crange_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_crange_005.json is saved with its eight settings, colour space rgb, fuzziness 40 and the top of green 255 | {"color_space":"rgb","fuzziness":40,"max_1":90,"max_2":255,"max_3":110,"min_1":0,"min_2":140,"min_3":0} | yes |
| fx_crange_013.json is refused in a sentence naming fuzziness | Color Range's fuzziness runs from 0 to 255, and this is 256. | yes |
| fx_crange_014.json is refused in a sentence naming fuzziness | Color Range's fuzziness runs from 0 to 255, and this is -1. | yes |
| fx_crange_015.json is refused in a sentence naming min 1 | Color Range's min 1 runs from 0 to 255, and this is 256. | yes |
| fx_crange_016.json is refused in a sentence naming max 3 | Color Range's max 3 runs from 0 to 255, and this is -1. | yes |
| fx_crange_017.json is refused in a sentence naming colour space | Color Range's colour space is "lab", "yuv" or "rgb", and this is "Lab". | yes |
| fx_crange_018.json is refused in a sentence naming colour space | Color Range's colour space is "lab", "yuv" or "rgb", and this is "hsl". | yes |
| a file with a Color Range whose fuzziness is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Color Range without its colour space is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Color Range without its max 3 is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| fuzziness 256 is refused with a sentence, and nothing changes | Color Range's fuzziness runs from 0 to 255, and this is 256. | yes |
| colour space "hsl" is refused with a sentence, and nothing changes | Color Range's colour space is "lab", "yuv" or "rgb", and this is "hsl". | yes |
| max 3 keyed to 300 is refused with a sentence, and nothing changes | Color Range's max 3 runs from 0 to 255, and this is 300. | yes |
| colour space yuv with a box about the blue screen is taken | taken | yes |
| fuzziness keyed from 0 to 80 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_crange_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_crange_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_crange_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_crange_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_crange_010.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_crange_012.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_crange_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_crange_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_crange_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_crange_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_crange_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_crange_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_crange_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_crange_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_crange_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_crange_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_crange_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_crange_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_crange_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_crange_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_crange_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_crange_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_crange_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_crange_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Color Range RGB, the whole cube but the darkest reds, fuzziness 60: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Color Range RGB, the whole cube but the darkest reds, fuzziness 60 on three layers, frame 0, Full | largest difference 1 of 255, 419 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Range RGB, the whole cube but the darkest reds, fuzziness 60 on three layers, frame 100, Full | largest difference 1 of 255, 1617 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Range RGB, the whole cube but the darkest reds, fuzziness 60 on three layers, frame 239, Full | largest difference 1 of 255, 625 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Range RGB, the whole cube but the darkest reds, fuzziness 60 on three layers, frame 0, Draft | largest difference 1 of 255, 121 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Range RGB, the whole cube but the darkest reds, fuzziness 60 on three layers, frame 100, Draft | largest difference 1 of 255, 135 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Range RGB, the whole cube but the darkest reds, fuzziness 60 on three layers, frame 239, Draft | largest difference 1 of 255, 131 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Range Lab, the lighter half, a* and b* within 40 of grey, fuzziness 30: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1721826 pixels changed | yes |
| the reference shot, Color Range Lab, the lighter half, a* and b* within 40 of grey, fuzziness 30 on three layers, frame 0, Full | largest difference 1 of 255, 3176 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Range Lab, the lighter half, a* and b* within 40 of grey, fuzziness 30 on three layers, frame 100, Full | largest difference 1 of 255, 938 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Range Lab, the lighter half, a* and b* within 40 of grey, fuzziness 30 on three layers, frame 239, Full | largest difference 1 of 255, 1250 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Range Lab, the lighter half, a* and b* within 40 of grey, fuzziness 30 on three layers, frame 0, Draft | largest difference 1 of 255, 421 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Range Lab, the lighter half, a* and b* within 40 of grey, fuzziness 30 on three layers, frame 100, Draft | largest difference 1 of 255, 287 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Range Lab, the lighter half, a* and b* within 40 of grey, fuzziness 30 on three layers, frame 239, Draft | largest difference 1 of 255, 339 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Range YUV, the darker half, fuzziness 0: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 813817 pixels changed | yes |
| the reference shot, Color Range YUV, the darker half, fuzziness 0 on three layers, frame 0, Full | largest difference 1 of 255, 120 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Range YUV, the darker half, fuzziness 0 on three layers, frame 100, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Range YUV, the darker half, fuzziness 0 on three layers, frame 239, Full | largest difference 1 of 255, 134 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Range YUV, the darker half, fuzziness 0 on three layers, frame 0, Draft | largest difference 1 of 255, 13 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Range YUV, the darker half, fuzziness 0 on three layers, frame 100, Draft | largest difference 1 of 255, 17 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Range YUV, the darker half, fuzziness 0 on three layers, frame 239, Draft | largest difference 1 of 255, 14 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-500 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the green-screen shot with no effect (a face whose edge is spilled green, a blue shirt); draws cleanly | [] | yes |
| as added (Lab, every end 0) nothing is keyed: the shot byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_lab_hard.png, a Lab box about the screen's green, fuzziness 0: the screen see-through, the face, the spilled pixel (a 111, outside the box) and the shirt fully covered; colour kept wherever anything is left; draws cleanly | [], screen 0, face 255, spilled 255, shirt 255, colour kept true | yes |
| 3_lab_soft.png, the same box with fuzziness 40: the screen still see-through, the face and the shirt still whole, the spilled green edge now fading (covering 101 by hand at the spilled pixel); draws cleanly | [], screen 0, face 255, spilled 101, shirt 255 | yes |
| 4_yuv_shirt.png, a YUV box about the blue shirt, fuzziness 20: the shirt see-through, the screen, the face and its edge fully covered; draws cleanly | [], screen 255, face 255, spilled 255, shirt 0 | yes |
| by hand: pure green, in an RGB box about green with fuzziness 30, keeps covering 0 of 255; draws cleanly | [], covering 0 | yes |
| by hand: the sea green (0, 200, 120), in an RGB box about green with fuzziness 30, keeps covering 85 of 255; draws cleanly | [], covering 85 | yes |
| by hand: pure blue, in an RGB box about green with fuzziness 30, keeps covering 255 of 255; draws cleanly | [], covering 255 | yes |

## Result

134 of 134 checks pass.
