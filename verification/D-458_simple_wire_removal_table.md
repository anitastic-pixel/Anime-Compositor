# D-458: CC Simple Wire Removal

B-338, after CycoreFX's CC Simple Wire Removal (Keying): a thin wire or rig between two points painted out. The Cycore FX manual names the controls but publishes no formula, so the rule is ours, stated in `tools/simple_wire_removal_reference.py`: a band Thickness wide along the wire, its edge softened by Slope, becomes clear (Fade), the layer at another frame (Frame Offset), or the picture either side of the band mixed across it with Mirror Blend of its reflection (Displace; Displace Horizontal samples along the rows). Every expected pixel is `Fixtures/simple_wire_removal/expected_simple_wire_removal.json`, written by the tool before this code existed and printed in document 25 as FX-WIRE-001 to 030. Tolerance 2e-5.

## FX-WIRE-001 to 030 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-WIRE-001 frame 0: The settings as added: displace, from 25, 50 to 75, 50, thickness 5, slope 50, mirror blend 25. The remover reaches 5 pixels each side of the line between columns 4 and 12, so the whole height there is smoothed, most at the middle. | largest difference 1.5e-7 | yes |
| FX-WIRE-001 frame 1: The settings as added: displace, from 25, 50 to 75, 50, thickness 5, slope 50, mirror blend 25. The remover reaches 5 pixels each side of the line between columns 4 and 12, so the whole height there is smoothed, most at the middle. | largest difference 1.3e-7 | yes |
| FX-WIRE-001: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-002 frame 0: Fade along the wire, thickness 3: the wire and a soft edge each side made clear, the ramp showing through the edges; past the ends untouched. | largest difference 1.5e-7 | yes |
| FX-WIRE-002: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-003 frame 0: Frame offset 1 along the wire: frames 0 and 2 take the band from the clean, redder plate after them, so the wire is gone; frame 3 would read frame 4, where the layer does not show, so it is untouched. | largest difference 1.5e-7 | yes |
| FX-WIRE-003 frame 2: Frame offset 1 along the wire: frames 0 and 2 take the band from the clean, redder plate after them, so the wire is gone; frame 3 would read frame 4, where the layer does not show, so it is untouched. | largest difference 1.5e-7 | yes |
| FX-WIRE-003 frame 3: Frame offset 1 along the wire: frames 0 and 2 take the band from the clean, redder plate after them, so the wire is gone; frame 3 would read frame 4, where the layer does not show, so it is untouched. | largest difference 1.3e-7 | yes |
| FX-WIRE-003: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-004 frame 0: Frame offset -1: frame 0 would read frame -1, so it is untouched; frame 2 reads frame 1, the clean plate. | largest difference 1.5e-7 | yes |
| FX-WIRE-004 frame 2: Frame offset -1: frame 0 would read frame -1, so it is untouched; frame 2 reads frame 1, the clean plate. | largest difference 1.5e-7 | yes |
| FX-WIRE-004: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-005 frame 0: Frame offset 1.7, taken down to 1: the same as FX-WIRE-003. | largest difference 1.5e-7 | yes |
| FX-WIRE-005: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-006 frame 0: Displace along the wire, thickness 3: the wire replaced by the ramp above and below it, mixed across the band, a quarter mirrored. | largest difference 1.5e-7 | yes |
| FX-WIRE-006: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-007 frame 0: Mirror blend 0: the band is only the mix of the two edges. | largest difference 1.5e-7 | yes |
| FX-WIRE-007: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-008 frame 0: Mirror blend 100: the band is only the picture reflected in its edges. | largest difference 1.5e-7 | yes |
| FX-WIRE-008: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-009 frame 0: Slope 0: the edge fades over a whole thickness each side. | largest difference 1.5e-7 | yes |
| FX-WIRE-009: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-010 frame 0: Slope 100, thickness 4: a hard edge, every pixel within 2 of the line replaced whole and none past it touched. | largest difference 1.5e-7 | yes |
| FX-WIRE-010: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-011 frame 0: Thickness 0: the drawing, untouched. | largest difference 1.5e-7 | yes |
| FX-WIRE-011: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-012 frame 0: Both points at 50, 54: the drawing, untouched. | largest difference 1.5e-7 | yes |
| FX-WIRE-012: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-013 frame 0: Displace horizontal along the flat wire: no row crosses it, so the drawing is untouched. | largest difference 1.5e-7 | yes |
| FX-WIRE-013: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-014 frame 0: Displace on a wire straight down the middle, from 50, 0 to 50, 100. | largest difference 1.5e-7 | yes |
| FX-WIRE-014: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-015 frame 0: Displace horizontal on the same upright wire: across it is along a row, so the same as FX-WIRE-014. | largest difference 1.5e-7 | yes |
| FX-WIRE-015: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-016 frame 0: Displace horizontal on a slanting wire from 20, 10 to 80, 90: each sample taken along its own row. | largest difference 1.6e-7 | yes |
| FX-WIRE-016: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-017 frame 0: Displace on the same slanting wire: the samples taken across the wire, so not the same as FX-WIRE-016. | largest difference 1.5e-7 | yes |
| FX-WIRE-017: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-018 frame 0: Thickness keyed from 2 at frame 0 to 6 at frame 2, linear: 2, 4 and 6. | largest difference 1.5e-7 | yes |
| FX-WIRE-018 frame 1: Thickness keyed from 2 at frame 0 to 6 at frame 2, linear: 2, 4 and 6. | largest difference 1.3e-7 | yes |
| FX-WIRE-018 frame 2: Thickness keyed from 2 at frame 0 to 6 at frame 2, linear: 2, 4 and 6. | largest difference 1.5e-7 | yes |
| FX-WIRE-018: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-019 frame 0: FX-WIRE-006 with the layer moved three pixels right: the same, moved; nothing grows. | largest difference 1.5e-7 | yes |
| FX-WIRE-019: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-020 frame 0: A wire along the top edge, from 10, 4 to 90, 4, thickness 4: the samples above the picture are held to its top row. | largest difference 1.5e-7 | yes |
| FX-WIRE-020: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-021 frame 0: Frame offset on an adjustment layer above the holder: the frames beneath at other times are not to hand, so each frame as it is, with a warning each frame. | largest difference 1.5e-7 | yes |
| FX-WIRE-021 frame 2: Frame offset on an adjustment layer above the holder: the frames beneath at other times are not to hand, so each frame as it is, with a warning each frame. | largest difference 1.5e-7 | yes |
| FX-WIRE-021: what opening it warns of, and what frame 3 warns of | [] and ["TEMPORAL_SMOOTHING_SKIPPED"] | yes |
| FX-WIRE-022 frame 0: Displace on an adjustment layer above the holder: the frame beneath is the holder alone, so the same as FX-WIRE-006. | largest difference 1.5e-7 | yes |
| FX-WIRE-022: what opening it warns of, and what frame 3 warns of | [] and [] | yes |
| FX-WIRE-023 frame 0: Removal style "clone", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WIRE-023 frame 2: Removal style "clone", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WIRE-023: what opening it warns of, and what frame 3 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WIRE-024 frame 0: Removal style "Displace": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WIRE-024 frame 2: Removal style "Displace": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WIRE-024: what opening it warns of, and what frame 3 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WIRE-025 frame 0: Thickness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WIRE-025 frame 2: Thickness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WIRE-025: what opening it warns of, and what frame 3 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WIRE-026 frame 0: Slope -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WIRE-026 frame 2: Slope -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WIRE-026: what opening it warns of, and what frame 3 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WIRE-027 frame 0: Mirror blend 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WIRE-027 frame 2: Mirror blend 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WIRE-027: what opening it warns of, and what frame 3 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WIRE-028 frame 0: Frame offset 1001, past 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WIRE-028 frame 2: Frame offset 1001, past 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WIRE-028: what opening it warns of, and what frame 3 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WIRE-029 frame 0: Point A at 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WIRE-029 frame 2: Point A at 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WIRE-029: what opening it warns of, and what frame 3 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WIRE-030 frame 0: Thickness keyed to 150 at frame 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WIRE-030 frame 2: Thickness keyed to 150 at frame 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WIRE-030: what opening it warns of, and what frame 3 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_wire_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wire_003.json is saved with its seven settings, Frame Offset style and offset 1, and no picture | {"frame_offset":1,"mirror_blend":25,"point_a":[10,54],"point_b":[90,54],"removal_style":"frame_offset","slope":50,"thickness":3} | yes |
| fx_wire_023.json is refused in a sentence naming removal style | CC Simple Wire Removal's removal style is "fade", "frame_offset", "displace" or "displace_horizontal", and this is "clone". | yes |
| fx_wire_024.json is refused in a sentence naming removal style | CC Simple Wire Removal's removal style is "fade", "frame_offset", "displace" or "displace_horizontal", and this is "Displace". | yes |
| fx_wire_025.json is refused in a sentence naming thickness | CC Simple Wire Removal's thickness runs from 0 to 100, and this is 101. | yes |
| fx_wire_026.json is refused in a sentence naming slope | CC Simple Wire Removal's slope runs from 0 to 100, and this is -1. | yes |
| fx_wire_027.json is refused in a sentence naming mirror blend | CC Simple Wire Removal's mirror blend runs from 0 to 100, and this is 101. | yes |
| fx_wire_028.json is refused in a sentence naming frame offset | CC Simple Wire Removal's frame offset runs from -1000 to 1000, and this is 1001. | yes |
| fx_wire_029.json is refused in a sentence naming point a | CC Simple Wire Removal's point a runs from -1000 to 1000, and this is 1001. | yes |
| a file with a CC Simple Wire Removal whose thickness is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a CC Simple Wire Removal without its removal style is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a CC Simple Wire Removal whose point A is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| thickness 101 is refused with a sentence, and nothing changes | CC Simple Wire Removal's thickness runs from 0 to 100, and this is 101. | yes |
| removal style "clone" is refused with a sentence, and nothing changes | CC Simple Wire Removal's removal style is "fade", "frame_offset", "displace" or "displace_horizontal", and this is "clone". | yes |
| slope keyed to 150 is refused with a sentence, and nothing changes | CC Simple Wire Removal's slope runs from 0 to 100, and this is 150. | yes |
| Fade is taken | taken | yes |
| thickness keyed from 0 to 10 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_wire_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_wire_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_wire_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_wire_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_wire_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_wire_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_wire_020.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_wire_001.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_002.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_003.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 6 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_004.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 6 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_005.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 6 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_006.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_007.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_008.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_009.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_010.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_011.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 0 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_012.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 0 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_013.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 0 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_014.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_015.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_016.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_017.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_018.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_019.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_020.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_021.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 0 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_022.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 0 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_023.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 0 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_024.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 0 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_025.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 0 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_026.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 0 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_027.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 0 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_028.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 0 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_029.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 0 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_wire_030.json, frames 0 to 3 at Full and Draft | largest difference 0 of 255; on the card in 0 of 8 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, CC Simple Wire Removal Displace as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 7450 pixels changed | yes |
| the reference shot, CC Simple Wire Removal Displace as added on three layers, frame 0, Full | largest difference 1 of 255, 3724 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Displace as added on three layers, frame 100, Full | largest difference 1 of 255, 1407 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Displace as added on three layers, frame 239, Full | largest difference 1 of 255, 1760 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Displace as added on three layers, frame 0, Draft | largest difference 1 of 255, 130 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Displace as added on three layers, frame 100, Draft | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Displace as added on three layers, frame 239, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Fade, a slanting wire, thickness 12, slope 0: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 58538 pixels changed | yes |
| the reference shot, CC Simple Wire Removal Fade, a slanting wire, thickness 12, slope 0 on three layers, frame 0, Full | largest difference 1 of 255, 3301 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Fade, a slanting wire, thickness 12, slope 0 on three layers, frame 100, Full | largest difference 1 of 255, 1278 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Fade, a slanting wire, thickness 12, slope 0 on three layers, frame 239, Full | largest difference 1 of 255, 1505 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Fade, a slanting wire, thickness 12, slope 0 on three layers, frame 0, Draft | largest difference 1 of 255, 119 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Fade, a slanting wire, thickness 12, slope 0 on three layers, frame 100, Draft | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Fade, a slanting wire, thickness 12, slope 0 on three layers, frame 239, Draft | largest difference 1 of 255, 40 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Displace Horizontal, a slanting wire, thickness 9, mirror blend 70: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 13060 pixels changed | yes |
| the reference shot, CC Simple Wire Removal Displace Horizontal, a slanting wire, thickness 9, mirror blend 70 on three layers, frame 0, Full | largest difference 1 of 255, 3770 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Displace Horizontal, a slanting wire, thickness 9, mirror blend 70 on three layers, frame 100, Full | largest difference 1 of 255, 1406 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Displace Horizontal, a slanting wire, thickness 9, mirror blend 70 on three layers, frame 239, Full | largest difference 1 of 255, 1759 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Displace Horizontal, a slanting wire, thickness 9, mirror blend 70 on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Displace Horizontal, a slanting wire, thickness 9, mirror blend 70 on three layers, frame 100, Draft | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Displace Horizontal, a slanting wire, thickness 9, mirror blend 70 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Frame Offset -1, corner to corner, thickness 60 (frame 0 has no frame before it): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 88881 pixels changed | yes |
| the reference shot, CC Simple Wire Removal Frame Offset -1, corner to corner, thickness 60 (frame 0 has no frame before it) on three layers, frame 0, Full | largest difference 1 of 255, 4566 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Frame Offset -1, corner to corner, thickness 60 (frame 0 has no frame before it) on three layers, frame 100, Full | largest difference 1 of 255, 1417 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Frame Offset -1, corner to corner, thickness 60 (frame 0 has no frame before it) on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Frame Offset -1, corner to corner, thickness 60 (frame 0 has no frame before it) on three layers, frame 0, Draft | largest difference 1 of 255, 16989 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Frame Offset -1, corner to corner, thickness 60 (frame 0 has no frame before it) on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Simple Wire Removal Frame Offset -1, corner to corner, thickness 60 (frame 0 has no frame before it) on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-458 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with a dark wire drawn across it from (60, 50) to (420, 140); draws cleanly | [], the wire is 148.9 levels from the clean street on average | yes |
| 2_displace.png, Displace along the wire, thickness 4, slope 50: the wire painted over with the street either side of it, under a quarter of its difference from the clean street left; nothing more than 4 pixels (h + r) from it touched; draws cleanly | [], 4.7 levels left of 148.9, far pixels untouched: true | yes |
| 3_fade.png, Fade: the wire made clear (a see-through line where it was), with a soft edge; every wire pixel between its ends clear; nothing more than 4 pixels from it touched; draws cleanly | [], every wire pixel clear: true, far pixels untouched: true | yes |
| 4_displace_horizontal.png, Displace Horizontal: the same wire painted over from the street left and right of it along each row; under a quarter of its difference left; draws cleanly | [], 8.8 levels left of 148.9 | yes |
| 5_mirror_100.png, Displace with mirror blend 100: the band filled with the street reflected across it; the wire still gone; draws cleanly | [], 7.2 levels left of 148.9 | yes |
| thickness 0 changes nothing: the wired street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| by hand: a white row through flat grey 100, a level wire along it, thickness 2, slope 100, Displace: the row becomes 100 | [], row 4 red: [100, 100, 100, 100, 100, 100, 100, 100] | yes |

## Result

194 of 194 checks pass.
