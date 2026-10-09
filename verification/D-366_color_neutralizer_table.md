# D-366: Color Neutralizer

B-245, after CycoreFX's CC Color Neutralizer: the colours picked as the shadows', midtones' and highlights' cast are each pulled to grey at their own lightness, the corrections faded between black and white, with levels to add, pinning and black and white points. Every expected pixel is `Fixtures/color_neutralizer/expected_color_neutralizer.json`, written by `tools/color_neutralizer_reference.py` before this code existed and printed in document 25 as FX-NEUTRAL-001 to 018. Tolerance 2e-5.

## FX-NEUTRAL-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-NEUTRAL-001 frame 0: The settings as they start, black, grey and white to neutralize and nothing added: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-NEUTRAL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NEUTRAL-002 frame 0: Shadows unbalance the warm shadow #3c2d1e: its cast, turned the other way, is added in full at black and fades out toward the midtones; the warm shadow itself, under a fifth of the way up, loses about two thirds of its cast, its lightness kept; middle grey and up are untouched. | largest difference 1.9e-7 | yes |
| FX-NEUTRAL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NEUTRAL-003 frame 0: Midtones unbalance the warm midtone #968064: the cast taken out most at middle lightness, fading to nothing at black and at white. | largest difference 1.9e-7 | yes |
| FX-NEUTRAL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NEUTRAL-004 frame 0: Highlights unbalance the warm highlight #f0e6c8: the cast taken out of the light colours, white itself turned bluish. | largest difference 1.8e-7 | yes |
| FX-NEUTRAL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NEUTRAL-005 frame 0: All three: the warm shadow, midtone and highlight themselves (row 0) come out near grey, the cast gone, their lightness kept; their darker copies further down, whose cast is smaller than the correction at their lightness, are pushed past grey toward blue. | largest difference 1.7e-7 | yes |
| FX-NEUTRAL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NEUTRAL-006 frame 0: Numbers only: shadows red 20 and blue -20, midtones green 10, highlights red -30 and blue 30, levels added and faded between. | largest difference 1.6e-7 | yes |
| FX-NEUTRAL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NEUTRAL-007 frame 0: All three colours with pinning 50: black and white are pinned, and the correction comes in over the first and last quarter of the way. | largest difference 1.9e-7 | yes |
| FX-NEUTRAL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NEUTRAL-008 frame 0: Pinning 100: only middle lightness gets the whole correction; the rest less, the nearer black or white. | largest difference 2.0e-7 | yes |
| FX-NEUTRAL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NEUTRAL-009 frame 0: Black point 40 and white point 200 with all three colours: everything below 40 counts as shadow and above 200 as highlight. | largest difference 1.7e-7 | yes |
| FX-NEUTRAL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NEUTRAL-010 frame 0: Black point 120 and white point 100, white not above black, with the numbers: lightness below 120 takes the shadows' numbers only, from 120 up the highlights' only. | largest difference 1.5e-7 | yes |
| FX-NEUTRAL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NEUTRAL-011 frame 0: Pinning keyed from 0 at frame 0 to 100 at frame 4, all three colours: frame 0 is FX-NEUTRAL-005, frame 2 pinning 50, FX-NEUTRAL-007, frame 4 FX-NEUTRAL-008. | largest difference 1.7e-7 | yes |
| FX-NEUTRAL-011 frame 2: Pinning keyed from 0 at frame 0 to 100 at frame 4, all three colours: frame 0 is FX-NEUTRAL-005, frame 2 pinning 50, FX-NEUTRAL-007, frame 4 FX-NEUTRAL-008. | largest difference 1.9e-7 | yes |
| FX-NEUTRAL-011 frame 4: Pinning keyed from 0 at frame 0 to 100 at frame 4, all three colours: frame 0 is FX-NEUTRAL-005, frame 2 pinning 50, FX-NEUTRAL-007, frame 4 FX-NEUTRAL-008. | largest difference 2.0e-7 | yes |
| FX-NEUTRAL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NEUTRAL-012 frame 0: FX-NEUTRAL-005 moved three pixels right: the same, moved. | largest difference 1.7e-7 | yes |
| FX-NEUTRAL-012 frame 3: FX-NEUTRAL-005 moved three pixels right: the same, moved. | largest difference 1.7e-7 | yes |
| FX-NEUTRAL-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-NEUTRAL-013 frame 0: Shadows unbalance "#3c2d1", five digits, not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NEUTRAL-013 frame 4: Shadows unbalance "#3c2d1", five digits, not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NEUTRAL-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NEUTRAL-014 frame 0: Midtones two numbers, not three. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NEUTRAL-014 frame 4: Midtones two numbers, not three. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NEUTRAL-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NEUTRAL-015 frame 0: Highlights blue 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NEUTRAL-015 frame 4: Highlights blue 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NEUTRAL-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NEUTRAL-016 frame 0: Pinning 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NEUTRAL-016 frame 4: Pinning 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NEUTRAL-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NEUTRAL-017 frame 0: Black point -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NEUTRAL-017 frame 4: Black point -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NEUTRAL-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-NEUTRAL-018 frame 0: White point 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NEUTRAL-018 frame 4: White point 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-NEUTRAL-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_neutral_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_neutral_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_neutral_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_neutral_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_neutral_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_neutral_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_neutral_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_neutral_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_neutral_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_neutral_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_neutral_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_neutral_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_neutral_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_neutral_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_neutral_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_neutral_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_neutral_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_neutral_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_neutral_006.json is saved with its colours, its three lists of levels, pinning and points | {"black_point":0,"highlights":[-30,0,30],"highlights_unbalance":"#ffffff","midtones":[0,10,0],"midtones_unbalance":"#808080","pinning":0,"shadows":[20,0,-20],"shadows_unbalance":"#000000","white_point":255} | yes |
| fx_neutral_013.json is refused in a sentence naming the shadows unbalance | Color Neutralizer's shadows unbalance is written #rrggbb, and this is "#3c2d1". | yes |
| fx_neutral_014.json is refused in a sentence naming the midtones are three numbers | Color Neutralizer's midtones are three numbers, red, green and blue, and this has 2. | yes |
| a file with a Color Neutralizer whose shadows are a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Color Neutralizer whose midtones unbalance is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| shadows red 255.5 is refused with a sentence, and nothing changes | Color Neutralizer's shadows runs from -255 to 255, and this is 255.5. | yes |
| pinning -0.5 is refused with a sentence, and nothing changes | Color Neutralizer's pinning runs from 0 to 100, and this is -0.5. | yes |
| white point 255.5 is refused with a sentence, and nothing changes | Color Neutralizer's white point runs from 0 to 255, and this is 255.5. | yes |
| highlights unbalance "white" is refused with a sentence, and nothing changes | Color Neutralizer's highlights unbalance is written #rrggbb, and this is "white". | yes |
| black point keyed to 300 is refused with a sentence, and nothing changes | Color Neutralizer's black point runs from 0 to 255, and this is 300. | yes |
| the three warm colours, pinning 50, is taken | taken | yes |
| midtones keyed from 0, 0, 0 to 10, 20, 30 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_neutral_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_neutral_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_neutral_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_neutral_011.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_neutral_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_neutral_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_neutral_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_neutral_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_neutral_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_neutral_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_neutral_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_neutral_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_neutral_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_neutral_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_neutral_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_neutral_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_neutral_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_neutral_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_neutral_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_neutral_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_neutral_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_neutral_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_neutral_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Color Neutralizer with the three warm colours: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Color Neutralizer with the three warm colours on three layers, frame 0, Full | largest difference 1 of 255, 381 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer with the three warm colours on three layers, frame 100, Full | largest difference 1 of 255, 403 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer with the three warm colours on three layers, frame 239, Full | largest difference 1 of 255, 402 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer with the three warm colours on three layers, frame 0, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer with the three warm colours on three layers, frame 100, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer with the three warm colours on three layers, frame 239, Draft | largest difference 1 of 255, 31 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer with levels only, pinning 40, points 30 and 220: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1998866 pixels changed | yes |
| the reference shot, Color Neutralizer with levels only, pinning 40, points 30 and 220 on three layers, frame 0, Full | largest difference 1 of 255, 898 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer with levels only, pinning 40, points 30 and 220 on three layers, frame 100, Full | largest difference 1 of 255, 667 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer with levels only, pinning 40, points 30 and 220 on three layers, frame 239, Full | largest difference 1 of 255, 865 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer with levels only, pinning 40, points 30 and 220 on three layers, frame 0, Draft | largest difference 1 of 255, 40 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer with levels only, pinning 40, points 30 and 220 on three layers, frame 100, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer with levels only, pinning 40, points 30 and 220 on three layers, frame 239, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer with the numbers and white point 100 below black point 120: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2041600 pixels changed | yes |
| the reference shot, Color Neutralizer with the numbers and white point 100 below black point 120 on three layers, frame 0, Full | largest difference 1 of 255, 1962 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer with the numbers and white point 100 below black point 120 on three layers, frame 100, Full | largest difference 1 of 255, 1308 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer with the numbers and white point 100 below black point 120 on three layers, frame 239, Full | largest difference 1 of 255, 1497 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer with the numbers and white point 100 below black point 120 on three layers, frame 0, Draft | largest difference 1 of 255, 1547 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer with the numbers and white point 100 below black point 120 on three layers, frame 100, Draft | largest difference 1 of 255, 1280 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer with the numbers and white point 100 below black point 120 on three layers, frame 239, Draft | largest difference 1 of 255, 1586 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-366 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before_warm.png, the street with a warm cast laid on it (red up 24, green up 8, blue down 24), no effect; draws cleanly | [] | yes |
| 2_neutralized.png, the warm road's colour #54442a picked as the shadows' and midtones' cast and the warm markings' #ffffe2 as the highlights': the road comes out grey again; draws cleanly | [], the road [70, 70, 70] from [84, 68, 42] (spread 0 from 42) | yes |
| 3_numbers_cool_shadows_warm_highlights.png, the plain street with blue 60 added to the shadows and red 40, green 20 to the highlights: the road turns blue; draws cleanly | [], the road [60, 60, 98] | yes |

## Result

123 of 123 checks pass.
