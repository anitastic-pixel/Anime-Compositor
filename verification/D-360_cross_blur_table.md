# D-360: Cross Blur

B-239, after CycoreFX's CC Cross Blur: the layer blurred across by one box of Fast Box Blur and, apart, down by another, the two laid together by a transfer mode, so a bright point becomes a soft cross. Every expected pixel is `Fixtures/cross_blur/expected_cross_blur.json`, written by `tools/cross_blur_reference.py` before this code existed and printed in document 25 as FX-CROSS-001 to 018. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-CROSS-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CROSS-001 frame 0: Radius X 4, Radius Y 4, mode blend (the default): every edge streaks across and down, four pixels, at half strength, and not diagonally, so the half-covered red pixel becomes a plus sign. | largest difference 1.2e-7 | yes |
| FX-CROSS-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CROSS-002 frame 0: Radius X 6, Radius Y 0, blend: the across blur laid half and half on the drawing itself, a soft streak across over a sharp copy. | largest difference 1.6e-7 | yes |
| FX-CROSS-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CROSS-003 frame 0: Mode add: the two blurs added, held at 1, so where they cross they are brighter than either. | largest difference 1.6e-7 | yes |
| FX-CROSS-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CROSS-004 frame 0: Mode screen. | largest difference 1.7e-7 | yes |
| FX-CROSS-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CROSS-005 frame 0: Mode multiply: only where both blurs reach does colour stay as it is; the plus sign's arms keep the colour of the one blur there. | largest difference 2.7e-7 | yes |
| FX-CROSS-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CROSS-006 frame 0: Mode lighten: the lighter colour of the two blurs. | largest difference 2.2e-7 | yes |
| FX-CROSS-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CROSS-007 frame 0: Mode darken: the darker colour of the two blurs. | largest difference 2.2e-7 | yes |
| FX-CROSS-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CROSS-008 frame 0: Edges repeat: past the edge the edge pixel is read, so the line down the left edge keeps its strength there, and the layer does not grow. | largest difference 1.2e-7 | yes |
| FX-CROSS-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CROSS-009 frame 0: Radius X 2.5, Radius Y 1: the box of 5 and half of the next pixel each side, across; 3 down. | largest difference 2.1e-7 | yes |
| FX-CROSS-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CROSS-010 frame 0: Radius X 0, Radius Y 0: the drawing untouched. | largest difference 1.9e-7 | yes |
| FX-CROSS-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CROSS-011 frame 0: Radius X keyed from 0 at frame 0 to 8 at frame 4, linear, Radius Y 2: frame 0 blurs down only, half and half with the drawing, frame 2 is Radius X 4. | largest difference 1.7e-7 | yes |
| FX-CROSS-011 frame 2: Radius X keyed from 0 at frame 0 to 8 at frame 4, linear, Radius Y 2: frame 0 blurs down only, half and half with the drawing, frame 2 is Radius X 4. | largest difference 1.5e-7 | yes |
| FX-CROSS-011 frame 4: Radius X keyed from 0 at frame 0 to 8 at frame 4, linear, Radius Y 2: frame 0 blurs down only, half and half with the drawing, frame 2 is Radius X 4. | largest difference 1.2e-7 | yes |
| FX-CROSS-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CROSS-012 frame 0: Moved three pixels right, edges transparent: the layer grows, so the line's streak reaches the three columns left of the drawing. | largest difference 1.2e-7 | yes |
| FX-CROSS-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CROSS-013 frame 0: Moved three pixels right, edges repeat: nothing left of the drawing. | largest difference 1.2e-7 | yes |
| FX-CROSS-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CROSS-014 frame 0: Radius X 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CROSS-014 frame 4: Radius X 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CROSS-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CROSS-015 frame 0: Radius Y -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CROSS-015 frame 4: Radius Y -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CROSS-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CROSS-016 frame 0: Mode "overlay", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CROSS-016 frame 4: Mode "overlay", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CROSS-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CROSS-017 frame 0: Mode "Add": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CROSS-017 frame 4: Mode "Add": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CROSS-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CROSS-018 frame 0: Edges "Repeat": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CROSS-018 frame 4: Edges "Repeat": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CROSS-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| CrossBlur { radius_x: 4.0, radius_y: 4.0, mode: "blend", edges: "transparent" } grows the layer by the larger reach, none with edges repeat: 4 | 4 | yes |
| CrossBlur { radius_x: 2.5, radius_y: 1.0, mode: "blend", edges: "transparent" } grows the layer by the larger reach, none with edges repeat: 3 | 3 | yes |
| CrossBlur { radius_x: 0.0, radius_y: 9.0, mode: "add", edges: "transparent" } grows the layer by the larger reach, none with edges repeat: 9 | 9 | yes |
| CrossBlur { radius_x: 40.0, radius_y: 4.0, mode: "blend", edges: "repeat" } grows the layer by the larger reach, none with edges repeat: 0 | 0 | yes |
| a half-size draft preview halves both radii, 20 and 6 to 10 and 3, and keeps the words | CrossBlur { radius_x: 10.0, radius_y: 3.0, mode: "add", edges: "repeat" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cross_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cross_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cross_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cross_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cross_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cross_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cross_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cross_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cross_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cross_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cross_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cross_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cross_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cross_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cross_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cross_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cross_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cross_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cross_001.json, which leaves out the mode and the edges, is saved without them: blend and transparent are the defaults | {"radius_x":4,"radius_y":4} | yes |
| fx_cross_016.json is refused in a sentence | Cross Blur's mode is "blend", "add", "screen", "multiply", "lighten" or "darken", and this is "overlay". | yes |
| a file with a Cross Blur with no `radius_y` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Cross Blur whose mode is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| radius X 500.5 is refused with a sentence, and nothing changes | Cross Blur's radius x runs from 0 to 500, and this is 500.5. | yes |
| radius Y -0.5 is refused with a sentence, and nothing changes | Cross Blur's radius y runs from 0 to 500, and this is -0.5. | yes |
| mode "Screen", written with a capital is refused with a sentence, and nothing changes | Cross Blur's mode is "blend", "add", "screen", "multiply", "lighten" or "darken", and this is "Screen". | yes |
| edges "clamp" is refused with a sentence, and nothing changes | Cross Blur's edges are "transparent" or "repeat", and this is "clamp". | yes |
| radius X keyed to 600 is refused with a sentence, and nothing changes | Cross Blur's radius x runs from 0 to 500, and this is 600. | yes |
| radius X 20, radius Y 6, mode add, edges repeat, is taken | taken | yes |
| radius Y keyed from 0 to 8 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cross_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cross_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cross_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cross_011.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cross_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cross_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_cross_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_cross_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_cross_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_cross_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_cross_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_cross_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_cross_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_cross_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_cross_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_cross_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_cross_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_cross_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_cross_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_cross_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_cross_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_cross_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_cross_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Cross Blur as added (Radius X 10, Radius Y 10, blend): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1536597 pixels changed | yes |
| the reference shot, Cross Blur as added (Radius X 10, Radius Y 10, blend) on three layers, frame 0, Full | largest difference 1 of 255, 1038 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Blur as added (Radius X 10, Radius Y 10, blend) on three layers, frame 100, Full | largest difference 1 of 255, 1060 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Blur as added (Radius X 10, Radius Y 10, blend) on three layers, frame 239, Full | largest difference 1 of 255, 900 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Blur as added (Radius X 10, Radius Y 10, blend) on three layers, frame 0, Draft | largest difference 1 of 255, 70 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Blur as added (Radius X 10, Radius Y 10, blend) on three layers, frame 100, Draft | largest difference 1 of 255, 75 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Blur as added (Radius X 10, Radius Y 10, blend) on three layers, frame 239, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Blur Radius X 40, Radius Y 4, add: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073592 pixels changed | yes |
| the reference shot, Cross Blur Radius X 40, Radius Y 4, add on three layers, frame 0, Full | largest difference 1 of 255, 512 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Blur Radius X 40, Radius Y 4, add on three layers, frame 100, Full | largest difference 1 of 255, 500 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Blur Radius X 40, Radius Y 4, add on three layers, frame 239, Full | largest difference 1 of 255, 349 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Blur Radius X 40, Radius Y 4, add on three layers, frame 0, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Blur Radius X 40, Radius Y 4, add on three layers, frame 100, Draft | largest difference 1 of 255, 17 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Blur Radius X 40, Radius Y 4, add on three layers, frame 239, Draft | largest difference 1 of 255, 28 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Blur Radius X 6, Radius Y 30, darken, edges repeat: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1613198 pixels changed | yes |
| the reference shot, Cross Blur Radius X 6, Radius Y 30, darken, edges repeat on three layers, frame 0, Full | largest difference 1 of 255, 839 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Blur Radius X 6, Radius Y 30, darken, edges repeat on three layers, frame 100, Full | largest difference 1 of 255, 1085 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Blur Radius X 6, Radius Y 30, darken, edges repeat on three layers, frame 239, Full | largest difference 1 of 255, 745 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Blur Radius X 6, Radius Y 30, darken, edges repeat on three layers, frame 0, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Blur Radius X 6, Radius Y 30, darken, edges repeat on three layers, frame 100, Draft | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cross Blur Radius X 6, Radius Y 30, darken, edges repeat on three layers, frame 239, Draft | largest difference 1 of 255, 45 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: a night street crossed, in `verification/D-360 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the street with no effect; draws cleanly | [] | yes |
| cross_as_added.png, Cross Blur as it starts, Radius X 10, Radius Y 10, blend: each star a soft plus, 6 pixels right of the middle star lit above the sky, 6 right and 6 down the sky as it was; draws cleanly | [], red 6 right 78, 6 right and 6 down 16, the sky 16 | yes |
| cross_streak_add.png, Radius X 40, Radius Y 2, add: a long flare across each star and window, 30 pixels right of the middle star lit well above the pixel 6 higher, where only the sky is added to itself and so is a little lighter than it was; draws cleanly | [], red 30 right 61, 30 right and 6 up 26, the sky before 16 | yes |
| cross_darken.png, Radius 8 both ways, darken: only where both blurs reach do the lights keep any strength, so each star shrinks to a faint dot with no arms, 6 pixels right of the middle star the sky as it was; draws cleanly | [], red at the star 118, 6 right 16, the sky 16 | yes |

## Result

126 of 126 checks pass.
