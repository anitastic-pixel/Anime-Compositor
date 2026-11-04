# D-1049: CC Composite

B-929: `core.cc_composite` lays the layer as it was before its first effect back on what its effects made: in front, behind, or by add, multiply or screen, at an opacity, and with RGB Only the effects' alpha kept. After After Effects' CC Composite (Channel), as lane w12g's long-shadow tutorial (wWtZXibe2QI at 1:55) uses it to put the logo back on top of its shadow. Every expected pixel is `Fixtures/cc_composite/expected_cc_composite.json`, written by `tools/cc_composite_reference.py` before this code existed and printed in document 25 as FX-CCCOMP-001 to 016. Tolerance 2e-5.

## FX-CCCOMP-001 to 016 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CCCOMP-001 frame 0: The settings as they start, opacity 100, in front, RGB only off, as the only effect: the drawing laid over itself: every solid pixel and the empty border as they were, a soft edge thicker (covering a becomes a + a(1 - a)). | largest difference 1.9e-7 | yes |
| FX-CCCOMP-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCCOMP-002 frame 0: After a Solid Composite that leaves only a half-covering blue (#3366cc at opacity 50, source opacity 0): in front, the drawing laid back over the blue: every solid pixel the drawing's, the border the half blue. | largest difference 1.9e-7 | yes |
| FX-CCCOMP-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCCOMP-003 frame 0: The same, behind: the half blue over the drawing: every solid pixel half blue, the border the half blue. | largest difference 1.0e-7 | yes |
| FX-CCCOMP-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCCOMP-004 frame 0: FX-CCCOMP-002 at opacity 50: the drawing at half strength over the half blue. | largest difference 9.6e-8 | yes |
| FX-CCCOMP-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCCOMP-005 frame 0: FX-CCCOMP-002 at opacity 0: nothing laid back: the half blue alone, as the Solid Composite left it. | largest difference 3.9e-8 | yes |
| FX-CCCOMP-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCCOMP-006 frame 0: After a solid blue (#3366cc, source opacity 0), add: every solid pixel the drawing plus the blue, in linear light, held at 1; the border the blue. | largest difference 2.0e-7 | yes |
| FX-CCCOMP-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCCOMP-007 frame 0: After the solid blue, multiply: every solid pixel the drawing times the blue; the border the blue. | largest difference 1.3e-7 | yes |
| FX-CCCOMP-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCCOMP-008 frame 0: After the solid blue, screen: every solid pixel lighter than both; the border the blue. | largest difference 1.9e-7 | yes |
| FX-CCCOMP-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCCOMP-009 frame 0: FX-CCCOMP-002 with RGB only on: the colours of FX-CCCOMP-002 but the Solid Composite's alpha, half covering everywhere. | largest difference 9.3e-8 | yes |
| FX-CCCOMP-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCCOMP-010 frame 0: FX-CCCOMP-001 with RGB only on: the alpha the drawing's own, so a soft edge keeps its covering; every pixel the drawing's. | largest difference 1.9e-7 | yes |
| FX-CCCOMP-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCCOMP-011 frame 0: FX-CCCOMP-002 with opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 FX-CCCOMP-005, frame 2 FX-CCCOMP-004, frame 4 FX-CCCOMP-002. | largest difference 3.9e-8 | yes |
| FX-CCCOMP-011 frame 2: FX-CCCOMP-002 with opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 FX-CCCOMP-005, frame 2 FX-CCCOMP-004, frame 4 FX-CCCOMP-002. | largest difference 9.6e-8 | yes |
| FX-CCCOMP-011 frame 4: FX-CCCOMP-002 with opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 FX-CCCOMP-005, frame 2 FX-CCCOMP-004, frame 4 FX-CCCOMP-002. | largest difference 1.9e-7 | yes |
| FX-CCCOMP-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCCOMP-012 frame 0: FX-CCCOMP-002 moved three pixels right: the same, moved; columns 0 to 2 of the frame stay empty. | largest difference 1.9e-7 | yes |
| FX-CCCOMP-012 frame 3: FX-CCCOMP-002 moved three pixels right: the same, moved; columns 0 to 2 of the frame stay empty. | largest difference 1.9e-7 | yes |
| FX-CCCOMP-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CCCOMP-013 frame 0: Opacity 101: above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCCOMP-013 frame 4: Opacity 101: above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCCOMP-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CCCOMP-014 frame 0: Composite original "stencil": this program's CC Composite lays the original in front, behind, or by add, multiply or screen. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCCOMP-014 frame 4: Composite original "stencil": this program's CC Composite lays the original in front, behind, or by add, multiply or screen. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCCOMP-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CCCOMP-015 frame 0: RGB only "yes": it is "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCCOMP-015 frame 4: RGB only "yes": it is "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCCOMP-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CCCOMP-016 frame 0: Opacity keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCCOMP-016 frame 4: Opacity keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CCCOMP-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cccomp_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cccomp_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cccomp_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cccomp_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cccomp_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cccomp_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cccomp_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cccomp_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cccomp_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cccomp_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cccomp_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cccomp_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cccomp_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cccomp_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cccomp_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cccomp_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cccomp_010.json is saved with its three settings | {"composite_original":"in_front","opacity":100,"rgb_only":"on"} | yes |
| fx_cccomp_013.json is refused in a sentence saying opacity | CC Composite's opacity runs from 0 to 100, and this is 101. | yes |
| fx_cccomp_014.json is refused in a sentence saying composite original is "in_front", "behind", "add", "multiply" or "screen" | CC Composite's composite original is "in_front", "behind", "add", "multiply" or "screen", and this is "stencil". | yes |
| fx_cccomp_015.json is refused in a sentence saying RGB only is "off" or "on" | CC Composite's RGB only is "off" or "on", and this is "yes". | yes |
| a file with a CC Composite whose opacity is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a CC Composite without its RGB only is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| composite original stencil is refused with a sentence, and nothing changes | CC Composite's composite original is "in_front", "behind", "add", "multiply" or "screen", and this is "stencil". | yes |
| RGB only yes is refused with a sentence, and nothing changes | CC Composite's RGB only is "off" or "on", and this is "yes". | yes |
| composite original behind is taken | taken | yes |
| RGB only on is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cccomp_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cccomp_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cccomp_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cccomp_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cccomp_012.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## The card: a CC Composite and every effect above it are drawn by the processor

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CCCOMP-001, fx_cccomp_001.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CCCOMP-002, fx_cccomp_002.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CCCOMP-003, fx_cccomp_003.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CCCOMP-004, fx_cccomp_004.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CCCOMP-005, fx_cccomp_005.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CCCOMP-006, fx_cccomp_006.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CCCOMP-007, fx_cccomp_007.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CCCOMP-008, fx_cccomp_008.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CCCOMP-009, fx_cccomp_009.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CCCOMP-010, fx_cccomp_010.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CCCOMP-011, fx_cccomp_011.json, frames [0, 2, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-CCCOMP-012, fx_cccomp_012.json, frames [0, 3] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CCCOMP-013, fx_cccomp_013.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CCCOMP-014, fx_cccomp_014.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CCCOMP-015, fx_cccomp_015.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CCCOMP-016, fx_cccomp_016.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_cccomp_002.json: the Solid Composite above the CC Composite is not on the card | 0 on the card | yes |
| fx_cccomp_002.json with a second Solid Composite after the CC Composite: that one goes to the card, and the card's frame is the processor's within 1 level | 1 on the card; largest difference 0 of 255; refused: false; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-1049 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_blue.png, a Solid Composite that leaves only #3366cc: every pixel the blue, within 1 level; draws cleanly | [], all blue: true | yes |
| 3_blue_then_in_front.png, the blue then CC Composite in front: the street back on top, byte for byte 1_before.png; draws cleanly | [], 0 pixels differ from 1_before.png | yes |
| 4_blue_then_behind.png, the same behind: the blue covers the street, byte for byte 2_blue.png; draws cleanly | [], 0 pixels differ from 2_blue.png | yes |
| 5_blue_then_in_front_50.png, in front at opacity 50: the street half seen over the blue; every channel between the street's and the blue's, within 1 level; draws cleanly | [], between: true | yes |
| 6_blue_then_multiply.png, multiply: the street tinted blue and darker, no channel lighter than the street's, within 1 level; draws cleanly | [], none lighter: true | yes |

## Result

95 of 95 checks pass.
