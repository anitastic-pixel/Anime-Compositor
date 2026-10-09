# D-368: Kernel

B-247, after CycoreFX's CC Kernel: each pixel rebuilt from itself and its eight neighbours, each weighed by a number in a three by three grid the user types, the sum divided by the divider, and made positive if asked. Every expected pixel is `Fixtures/kernel/expected_kernel.json`, written by `tools/kernel_reference.py` before this code existed and printed in document 25 as FX-KERNEL-001 to 018. Tolerance 2e-5.

## FX-KERNEL-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-KERNEL-001 frame 0: The grid as it starts, the middle pixel alone, divider 1: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-KERNEL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KERNEL-002 frame 0: All nine 1, divider 9: each pixel the average of itself and its eight neighbours, a soft blur; the columns' edges soften and the empty column darkens its neighbour. | largest difference 1.0e-7 | yes |
| FX-KERNEL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KERNEL-003 frame 0: 0 -1 0 / -1 5 -1 / 0 -1 0, divider 1: a sharpen; flat areas stay, each column's edge gets a light and a dark rim. | largest difference 6.0e-7 | yes |
| FX-KERNEL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KERNEL-004 frame 0: -1 all round, 8 in the middle, divider 1, absolute values off: edges only; flat areas turn black, and where a pixel is darker than its neighbours the negative result is black too. | largest difference 5.5e-7 | yes |
| FX-KERNEL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KERNEL-005 frame 0: The same edge grid, absolute values on: the negative results turn positive, so both sides of every edge light up. | largest difference 5.5e-7 | yes |
| FX-KERNEL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KERNEL-006 frame 0: -2 -1 0 / -1 1 1 / 0 1 2, divider 1: an emboss, lit from the lower right; it shows the grid is read with line 1 above and each line's first number on the left. | largest difference 3.1e-7 | yes |
| FX-KERNEL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KERNEL-007 frame 0: Only line 2's first number, 1: every pixel takes the colour of the pixel to its left, so the picture moves one pixel right; the first shown column takes the empty column's 0, black. | largest difference 1.9e-7 | yes |
| FX-KERNEL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KERNEL-008 frame 0: Only line 1's middle number, 1: every pixel takes the colour of the pixel above, so the picture moves one pixel down; the top row repeats. | largest difference 1.9e-7 | yes |
| FX-KERNEL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KERNEL-009 frame 0: The middle pixel alone, divider 2: every colour's encoded value halved. | largest difference 4.4e-8 | yes |
| FX-KERNEL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KERNEL-010 frame 0: The middle pixel alone, divider 0.5: every colour's encoded value doubled, held at white. | largest difference 1.5e-7 | yes |
| FX-KERNEL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KERNEL-011 frame 0: Middle number 4, divider keyed from 4 at frame 0 to 1 at frame 4, linear: frame 0 untouched, frame 2 every value times 1.6, frame 4 times 4, held at white. | largest difference 1.9e-7 | yes |
| FX-KERNEL-011 frame 2: Middle number 4, divider keyed from 4 at frame 0 to 1 at frame 4, linear: frame 0 untouched, frame 2 every value times 1.6, frame 4 times 4, held at white. | largest difference 2.6e-7 | yes |
| FX-KERNEL-011 frame 4: Middle number 4, divider keyed from 4 at frame 0 to 1 at frame 4, linear: frame 0 untouched, frame 2 every value times 1.6, frame 4 times 4, held at white. | largest difference 1.5e-7 | yes |
| FX-KERNEL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KERNEL-012 frame 0: FX-KERNEL-003 moved three pixels right: the same, moved. | largest difference 6.0e-7 | yes |
| FX-KERNEL-012 frame 3: FX-KERNEL-003 moved three pixels right: the same, moved. | largest difference 6.0e-7 | yes |
| FX-KERNEL-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KERNEL-013 frame 0: Line 1's first number 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KERNEL-013 frame 4: Line 1's first number 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KERNEL-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KERNEL-014 frame 0: Line 2 with two numbers, not three. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KERNEL-014 frame 4: Line 2 with two numbers, not three. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KERNEL-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KERNEL-015 frame 0: Divider 0, below 0.01. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KERNEL-015 frame 4: Divider 0, below 0.01. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KERNEL-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KERNEL-016 frame 0: Divider 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KERNEL-016 frame 4: Divider 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KERNEL-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KERNEL-017 frame 0: Absolute values "yes", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KERNEL-017 frame 4: Absolute values "yes", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KERNEL-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KERNEL-018 frame 0: Absolute values "ON": the word is exact, so capitals are not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KERNEL-018 frame 4: Absolute values "ON": the word is exact, so capitals are not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KERNEL-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_kernel_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kernel_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kernel_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kernel_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kernel_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kernel_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kernel_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kernel_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kernel_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kernel_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kernel_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kernel_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kernel_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kernel_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kernel_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kernel_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kernel_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kernel_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kernel_006.json is saved with its three lines, divider and absolute values | {"absolute_values":"off","divider":1,"line_1":[-2,-1,0],"line_2":[-1,1,1],"line_3":[0,1,2]} | yes |
| fx_kernel_014.json is refused in a sentence | Kernel's line 2 is three numbers, left, middle and right, and this has 2. | yes |
| fx_kernel_017.json is refused in a sentence | Kernel's absolute values is "off" or "on", and this is "yes". | yes |
| fx_kernel_018.json is refused in a sentence | Kernel's absolute values is "off" or "on", and this is "ON". | yes |
| a file with a Kernel with no `divider` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Kernel whose line 1 is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| line 1 left 1000.5 is refused with a sentence, and nothing changes | Kernel's line 1 runs from -1000 to 1000, and this is 1000.5. | yes |
| divider 0 is refused with a sentence, and nothing changes | Kernel's divider runs from 0.01 to 1000, and this is 0. | yes |
| divider 1000.5 is refused with a sentence, and nothing changes | Kernel's divider runs from 0.01 to 1000, and this is 1000.5. | yes |
| absolute values "On", written with a capital is refused with a sentence, and nothing changes | Kernel's absolute values is "off" or "on", and this is "On". | yes |
| divider keyed to 0 is refused with a sentence, and nothing changes | Kernel's divider runs from 0.01 to 1000, and this is 0. | yes |
| the sharpen grid, absolute values on, is taken | taken | yes |
| divider keyed from 1 to 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_kernel_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_kernel_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_kernel_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_kernel_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_kernel_011.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_kernel_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_kernel_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kernel_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_kernel_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_kernel_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_kernel_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_kernel_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_kernel_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_kernel_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_kernel_009.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_kernel_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_kernel_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_kernel_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_kernel_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kernel_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kernel_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kernel_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kernel_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_kernel_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Kernel sharpen (0 -1 0, -1 5 -1, 0 -1 0): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1545680 pixels changed | yes |
| the reference shot, Kernel sharpen (0 -1 0, -1 5 -1, 0 -1 0) on three layers, frame 0, Full | largest difference 1 of 255, 3215 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel sharpen (0 -1 0, -1 5 -1, 0 -1 0) on three layers, frame 100, Full | largest difference 1 of 255, 821 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel sharpen (0 -1 0, -1 5 -1, 0 -1 0) on three layers, frame 239, Full | largest difference 1 of 255, 1360 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel sharpen (0 -1 0, -1 5 -1, 0 -1 0) on three layers, frame 0, Draft | largest difference 1 of 255, 260 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel sharpen (0 -1 0, -1 5 -1, 0 -1 0) on three layers, frame 100, Draft | largest difference 1 of 255, 216 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel sharpen (0 -1 0, -1 5 -1, 0 -1 0) on three layers, frame 239, Draft | largest difference 1 of 255, 244 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel box blur (all ones, divided by 9): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 997774 pixels changed | yes |
| the reference shot, Kernel box blur (all ones, divided by 9) on three layers, frame 0, Full | largest difference 1 of 255, 1426 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel box blur (all ones, divided by 9) on three layers, frame 100, Full | largest difference 1 of 255, 848 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel box blur (all ones, divided by 9) on three layers, frame 239, Full | largest difference 1 of 255, 705 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel box blur (all ones, divided by 9) on three layers, frame 0, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel box blur (all ones, divided by 9) on three layers, frame 100, Draft | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel box blur (all ones, divided by 9) on three layers, frame 239, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel edges, absolute values on: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073581 pixels changed | yes |
| the reference shot, Kernel edges, absolute values on on three layers, frame 0, Full | largest difference 1 of 255, 1224 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel edges, absolute values on on three layers, frame 100, Full | largest difference 1 of 255, 2148 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel edges, absolute values on on three layers, frame 239, Full | largest difference 1 of 255, 1479 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel edges, absolute values on on three layers, frame 0, Draft | largest difference 1 of 255, 175 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel edges, absolute values on on three layers, frame 100, Draft | largest difference 1 of 255, 184 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel edges, absolute values on on three layers, frame 239, Draft | largest difference 1 of 255, 178 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel emboss (-2 -1 0, -1 1 1, 0 1 2): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1592149 pixels changed | yes |
| the reference shot, Kernel emboss (-2 -1 0, -1 1 1, 0 1 2) on three layers, frame 0, Full | largest difference 1 of 255, 2904 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel emboss (-2 -1 0, -1 1 1, 0 1 2) on three layers, frame 100, Full | largest difference 1 of 255, 1111 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel emboss (-2 -1 0, -1 1 1, 0 1 2) on three layers, frame 239, Full | largest difference 1 of 255, 1436 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel emboss (-2 -1 0, -1 1 1, 0 1 2) on three layers, frame 0, Draft | largest difference 1 of 255, 344 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel emboss (-2 -1 0, -1 1 1, 0 1 2) on three layers, frame 100, Draft | largest difference 1 of 255, 308 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Kernel emboss (-2 -1 0, -1 1 1, 0 1 2) on three layers, frame 239, Draft | largest difference 1 of 255, 316 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-368 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts (the middle pixel alone, divider 1): nothing changes; draws cleanly | [], 0 pixels changed | yes |
| 3_box_blur.png, all ones divided by 9: every edge softened by one pixel; the flat road keeps its colour; draws cleanly | [], 10622 pixels changed, the road [60, 60, 66] from [60, 60, 66] | yes |
| 4_sharpen.png, sharpen: every edge crisper, a light rim on the light side; the flat road keeps its colour; draws cleanly | [], 63530 pixels changed, the road [60, 60, 66] from [60, 60, 66] | yes |
| 6_emboss.png, emboss: edges lit from the lower right, shadowed at the upper left; the flat road keeps its colour; draws cleanly | [], 63524 pixels changed, the road [60, 60, 66] from [60, 60, 66] | yes |
| 5_edges.png, eight round the middle, absolute values on: flat areas go black, only the outlines of houses, windows and markings are left; draws cleanly | [], the road [0, 0, 0], 63936 of 129600 pixels not black | yes |

## Result

135 of 135 checks pass.
