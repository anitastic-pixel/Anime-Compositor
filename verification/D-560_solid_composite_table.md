# D-560: Solid Composite, its fixture folder

B-440: `core.solid_composite` (D-312, built in B-195 from P-26's Advanced Electric tutorial) lays a layer, at its own opacity, on a solid colour at another, blended normal, add, screen or multiply, after After Effects' Solid Composite (Channel). B-195 pinned it on hand-worked solids; this unit gives it a fixture folder. Every expected pixel is `Fixtures/solid_composite/expected_solid_composite.json`, written by `tools/solid_composite_reference.py` before this code existed and printed in document 25 as FX-SOLIDC-001 to 019. No code changed. Tolerance 2e-5.

## FX-SOLIDC-001 to 019 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SOLIDC-001 frame 0: The settings as they start, source opacity 100, colour #ffffff, opacity 100, normal: the drawing over white: the empty border turns white, the soft edges the line and the skin half over white, every solid pixel as it was. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLIDC-002 frame 0: Colour #3366cc: the drawing over a blue: the border turns #3366cc, every solid pixel as it was. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLIDC-003 frame 0: Colour #3366cc at opacity 50: the border is the blue at half covering, a soft edge covers three quarters, every solid pixel as it was. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLIDC-004 frame 0: Opacity 0: no solid at all: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLIDC-005 frame 0: Source opacity 50 over the blue: the drawing at half strength over a solid blue, everything opaque. | largest difference 1.1e-7 | yes |
| FX-SOLIDC-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLIDC-006 frame 0: Source opacity 0: the blue alone, every pixel #3366cc. | largest difference 7.8e-8 | yes |
| FX-SOLIDC-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLIDC-007 frame 0: Add, colour #404040: every solid pixel raised by the grey (in linear light, held at the top): the black turns #404040, the white stays white; the border is the grey. | largest difference 2.0e-7 | yes |
| FX-SOLIDC-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLIDC-008 frame 0: Screen, colour #808080: every solid pixel lightened, the black turns #808080, the white stays white; the border is the grey. | largest difference 1.7e-7 | yes |
| FX-SOLIDC-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLIDC-009 frame 0: Multiply, colour #ff8040: every solid pixel darkened by the orange: the white turns #ff8040, the black stays black; the border is the orange. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLIDC-010 frame 0: Multiply, colour #ff8040 at opacity 60, source opacity 70: the drawing at 70 per cent multiplied with the orange at 60, where both show. | largest difference 1.6e-7 | yes |
| FX-SOLIDC-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLIDC-011 frame 0: Colour #3366cc, opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing untouched, frame 2 FX-SOLIDC-003 (opacity 50), frame 4 FX-SOLIDC-002. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-011 frame 2: Colour #3366cc, opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing untouched, frame 2 FX-SOLIDC-003 (opacity 50), frame 4 FX-SOLIDC-002. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-011 frame 4: Colour #3366cc, opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing untouched, frame 2 FX-SOLIDC-003 (opacity 50), frame 4 FX-SOLIDC-002. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLIDC-012 frame 0: Colour #3366cc, source opacity eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: frame 0 FX-SOLIDC-006; at frame 2 it has gone past 100 and is held there, so frames 2 and 4 are both FX-SOLIDC-002. | largest difference 7.8e-8 | yes |
| FX-SOLIDC-012 frame 2: Colour #3366cc, source opacity eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: frame 0 FX-SOLIDC-006; at frame 2 it has gone past 100 and is held there, so frames 2 and 4 are both FX-SOLIDC-002. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-012 frame 4: Colour #3366cc, source opacity eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: frame 0 FX-SOLIDC-006; at frame 2 it has gone past 100 and is held there, so frames 2 and 4 are both FX-SOLIDC-002. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLIDC-013 frame 0: FX-SOLIDC-007 moved three pixels right: the same, moved; the grey fills only the layer, so columns 0 to 2 of the frame stay empty. | largest difference 2.0e-7 | yes |
| FX-SOLIDC-013 frame 3: FX-SOLIDC-007 moved three pixels right: the same, moved; the grey fills only the layer, so columns 0 to 2 of the frame stay empty. | largest difference 2.0e-7 | yes |
| FX-SOLIDC-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SOLIDC-014 frame 0: Opacity 101: above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-014 frame 4: Opacity 101: above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SOLIDC-015 frame 0: Source opacity -1: below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-015 frame 4: Source opacity -1: below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SOLIDC-016 frame 0: Blend "difference": this program's Solid Composite blends normal, add, screen or multiply. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-016 frame 4: Blend "difference": this program's Solid Composite blends normal, add, screen or multiply. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SOLIDC-017 frame 0: Colour "white": a colour is written #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-017 frame 4: Colour "white": a colour is written #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SOLIDC-018 frame 0: Colour "#12345": five digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-018 frame 4: Colour "#12345": five digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SOLIDC-019 frame 0: Opacity keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-019 frame 4: Opacity keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SOLIDC-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_solidc_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_solidc_010.json is saved with its four settings | {"blend":"multiply","color":"#ff8040","opacity":60,"source_opacity":70} | yes |
| fx_solidc_014.json is refused in a sentence saying opacity | Solid Composite's opacity runs from 0 to 100, and this is 101. | yes |
| fx_solidc_015.json is refused in a sentence saying source_opacity | Solid Composite's source opacity runs from 0 to 100, and this is -1. | yes |
| fx_solidc_016.json is refused in a sentence saying blend is "normal", "add", "screen" or "multiply" | Solid Composite's blend is "normal", "add", "screen" or "multiply", and this is "difference". | yes |
| fx_solidc_017.json is refused in a sentence saying colour is written #rrggbb | Solid Composite's colour is written #rrggbb, and this is "white". | yes |
| fx_solidc_018.json is refused in a sentence saying colour is written #rrggbb | Solid Composite's colour is written #rrggbb, and this is "#12345". | yes |
| a file with a Solid Composite whose opacity is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Solid Composite without its blend is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| blend overlay is refused with a sentence, and nothing changes | Solid Composite's blend is "normal", "add", "screen" or "multiply", and this is "overlay". | yes |
| colour red is refused with a sentence, and nothing changes | Solid Composite's colour is written #rrggbb, and this is "red". | yes |
| blend screen is taken | taken | yes |
| colour #3366cc is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_solidc_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_solidc_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_solidc_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_solidc_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_solidc_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_solidc_013.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_solidc_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_solidc_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_solidc_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_solidc_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_solidc_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_solidc_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_solidc_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_solidc_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_solidc_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_solidc_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_solidc_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_solidc_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_solidc_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_solidc_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_solidc_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_solidc_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_solidc_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_solidc_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_solidc_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Solid Composite colour #3366cc, source opacity 50: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Solid Composite colour #3366cc, source opacity 50 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Solid Composite colour #3366cc, source opacity 50 on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Solid Composite colour #3366cc, source opacity 50 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Solid Composite colour #3366cc, source opacity 50 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Solid Composite colour #3366cc, source opacity 50 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Solid Composite colour #3366cc, source opacity 50 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Solid Composite multiply, colour #ff8040: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Solid Composite multiply, colour #ff8040 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Solid Composite multiply, colour #ff8040 on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Solid Composite multiply, colour #ff8040 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Solid Composite multiply, colour #ff8040 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Solid Composite multiply, colour #ff8040 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Solid Composite multiply, colour #ff8040 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Solid Composite add, colour #404040 at opacity 60: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073464 pixels changed | yes |
| the reference shot, Solid Composite add, colour #404040 at opacity 60 on three layers, frame 0, Full | largest difference 1 of 255, 606 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Solid Composite add, colour #404040 at opacity 60 on three layers, frame 100, Full | largest difference 1 of 255, 153 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Solid Composite add, colour #404040 at opacity 60 on three layers, frame 239, Full | largest difference 1 of 255, 473 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Solid Composite add, colour #404040 at opacity 60 on three layers, frame 0, Draft | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Solid Composite add, colour #404040 at opacity 60 on three layers, frame 100, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Solid Composite add, colour #404040 at opacity 60 on three layers, frame 239, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-560 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| the settings as they start (white, normal, both at 100) change nothing on the solid street: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_half_over_blue.png, source opacity 50 over #3366cc: the street half seen through a blue; every channel between the street's and the blue's, within 1 level; draws cleanly | [], between: true | yes |
| 3_multiply_orange.png, multiply with #ff8040: the street warmer and darker, no channel lighter, the reds kept (the orange's red is full), within 1 level; draws cleanly | [], none lighter: true, reds kept: true | yes |
| 4_screen_grey.png, screen with #808080: the street paler, every channel at least the street's and at least 128, within 1 level; draws cleanly | [], lighter: true | yes |

## Result

132 of 132 checks pass.
