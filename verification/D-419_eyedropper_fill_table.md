# D-419: Eyedropper Fill

B-298, after After Effects' Eyedropper Fill: an area about the Sample Point (per cent of the drawing) of every pixel whose centre lies within Sample Radius, row by row (or the one pixel holding the point when none does), pixels past the layer counted as clear; its colour the straight colours' average over the pixels showing (Skip Empty) or over all (All), the premultiplied average (All Premultiplied), or that with the area's covering (Including Alpha); the layer filled with it, times each pixel's alpha with Maintain Original Alpha, then mixed back by Blend With Original. The layer never grows. Every expected pixel is `Fixtures/eyedropper_fill/expected_eyedropper_fill.json`, written by `tools/eyedropper_fill_reference.py` before this code existed and printed in document 25 as FX-EYEFILL-001 to 032. Tolerance 2e-5.

## FX-EYEFILL-001 to 032 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-EYEFILL-001 frame 0: The settings as they start: the point in the middle, (8, 5), radius 0, Skip Empty: the skin under the point fills the whole layer, its line, shadow and empty border alike, all opaque. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-002 frame 0: The point at (2, 5), radius 2, Skip Empty: twelve pixels, two of them empty and left out, the soft edge's line counted at its full colour; a dark colour, mostly the line's, a little skin. | largest difference 3.7e-8 | yes |
| FX-EYEFILL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-003 frame 0: The same area, All: the two empty pixels count as black, so a little darker than FX-EYEFILL-002. | largest difference 3.6e-8 | yes |
| FX-EYEFILL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-004 frame 0: The same area, All Premultiplied: the soft edge counts at half, so darker again, still opaque. | largest difference 2.5e-8 | yes |
| FX-EYEFILL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-005 frame 0: The same area, Including Alpha: the area's covering, about two thirds, becomes the layer's; the colour that of FX-EYEFILL-004 divided by it. | largest difference 2.5e-8 | yes |
| FX-EYEFILL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-006 frame 0: FX-EYEFILL-005 with Maintain Original Alpha: the covering also times each pixel's own, so the border stays empty and the soft edge half again. | largest difference 2.5e-8 | yes |
| FX-EYEFILL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-007 frame 0: FX-EYEFILL-002 with Maintain Original Alpha: the colour opaque inside the cel, half covering on its soft edge, the border empty. | largest difference 3.7e-8 | yes |
| FX-EYEFILL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-008 frame 0: The settings as they start, Blend With Original 50: half way from the skin to the cel, the empty border half covered with skin. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-009 frame 0: Blend With Original 100: the cel as it was. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-010 frame 0: The point on the edge between skin and shadow, (10, 5), radius 1.5: four pixels, two each, an even mix of the two. | largest difference 1.7e-7 | yes |
| FX-EYEFILL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-011 frame 0: Radius 1000, Skip Empty: every pixel that shows, each at its full colour, averaged; the layer's empty pixels and the plane past it left out. | largest difference 8.9e-8 | yes |
| FX-EYEFILL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-012 frame 0: Radius 1000, All: the disc's three million pixels counted, nearly all past the layer and black, so the fill is all but black. | largest difference 3.9e-12 | yes |
| FX-EYEFILL-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-013 frame 0: The point past the layer's left edge, (-50, 50), Skip Empty: nothing shows there, so black, opaque. | largest difference 0.0e0 | yes |
| FX-EYEFILL-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-014 frame 0: The same point, Including Alpha: nothing shows, so the layer is empty. | largest difference 0.0e0 | yes |
| FX-EYEFILL-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-015 frame 0: A radius of 0.4 with the point at (8.5, 5.5), a pixel's centre: that pixel alone, as radius 0, FX-EYEFILL-001's frame. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-016 frame 0: The point keyed from (25, 50) at frame 0 to (75, 50) at frame 4: skin at frames 0 and 2, shadow at frame 4. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-016 frame 2: The point keyed from (25, 50) at frame 0 to (75, 50) at frame 4: skin at frames 0 and 2, shadow at frame 4. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-016 frame 4: The point keyed from (25, 50) at frame 0 to (75, 50) at frame 4: skin at frames 0 and 2, shadow at frame 4. | largest difference 1.5e-7 | yes |
| FX-EYEFILL-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-017 frame 0: The radius keyed from 0 to 4 at the edge point: the colour changes as the area grows. | largest difference 2.2e-9 | yes |
| FX-EYEFILL-017 frame 2: The radius keyed from 0 to 4 at the edge point: the colour changes as the area grows. | largest difference 3.7e-8 | yes |
| FX-EYEFILL-017 frame 4: The radius keyed from 0 to 4 at the edge point: the colour changes as the area grows. | largest difference 8.0e-8 | yes |
| FX-EYEFILL-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-018 frame 0: Blend With Original keyed from 0 to 100: frame 0 FX-EYEFILL-001's, frame 2 FX-EYEFILL-008's, frame 4 the cel. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-018 frame 2: Blend With Original keyed from 0 to 100: frame 0 FX-EYEFILL-001's, frame 2 FX-EYEFILL-008's, frame 4 the cel. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-018 frame 4: Blend With Original keyed from 0 to 100: frame 0 FX-EYEFILL-001's, frame 2 FX-EYEFILL-008's, frame 4 the cel. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-019 frame 0: FX-EYEFILL-001 moved three pixels right: the layer's own pixels move, the three columns it left are empty. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-020 frame 0: After a Motion Tile that grows the layer: the point is the drawing's own, so the frame is FX-EYEFILL-010's. | largest difference 1.7e-7 | yes |
| FX-EYEFILL-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-021 frame 0: Blend With Original keyed from 40 at frame 0 to 100 at frame 4 by an ease that passes its end: held at 100 at frame 2, the cel. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-021 frame 2: Blend With Original keyed from 40 at frame 0 to 100 at frame 4 by an ease that passes its end: held at 100 at frame 2, the cel. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-021 frame 4: Blend With Original keyed from 40 at frame 0 to 100 at frame 4 by an ease that passes its end: held at 100 at frame 2, the cel. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-022 frame 0: Including Alpha, Maintain Original Alpha and Blend With Original 50 together at the edge point, radius 2. | largest difference 1.4e-7 | yes |
| FX-EYEFILL-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-023 frame 0: All Premultiplied, Maintain Original Alpha, radius 0 on the soft edge at (1.5, 5.5): the line at half its colour, laid on each pixel's own covering. | largest difference 3.0e-8 | yes |
| FX-EYEFILL-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EYEFILL-024 frame 0: Sample Point across 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-024 frame 4: Sample Point across 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EYEFILL-025 frame 0: Sample Point down -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-025 frame 4: Sample Point down -1001, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EYEFILL-026 frame 0: Sample Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-026 frame 4: Sample Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EYEFILL-027 frame 0: Sample Radius 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-027 frame 4: Sample Radius 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EYEFILL-028 frame 0: Blend With Original -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-028 frame 4: Blend With Original -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EYEFILL-029 frame 0: Blend With Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-029 frame 4: Blend With Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EYEFILL-030 frame 0: Average Pixel Colors "sum", not one of the four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-030 frame 4: Average Pixel Colors "sum", not one of the four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EYEFILL-031 frame 0: Maintain Original Alpha "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-031 frame 4: Maintain Original Alpha "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EYEFILL-032 frame 0: Blend With Original keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-032 frame 4: Blend With Original keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EYEFILL-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_eyefill_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_eyefill_022.json is saved with its words and numbers as written | {"average_pixel_colors":"including_alpha","blend_with_original":50,"maintain_original_alpha":"on","sample_point":[12.5,50],"sample_radius":2} | yes |
| fx_eyefill_024.json is refused in a sentence | Eyedropper Fill's sample point runs from -1000 to 1000, and this is 1001. | yes |
| fx_eyefill_025.json is refused in a sentence | Eyedropper Fill's sample point runs from -1000 to 1000, and this is -1001. | yes |
| fx_eyefill_026.json is refused in a sentence | Eyedropper Fill's sample radius runs from 0 to 10000, and this is -1. | yes |
| fx_eyefill_027.json is refused in a sentence | Eyedropper Fill's sample radius runs from 0 to 10000, and this is 10001. | yes |
| fx_eyefill_028.json is refused in a sentence | Eyedropper Fill's blend with original runs from 0 to 100, and this is -1. | yes |
| fx_eyefill_029.json is refused in a sentence | Eyedropper Fill's blend with original runs from 0 to 100, and this is 101. | yes |
| fx_eyefill_030.json is refused in a sentence | Eyedropper Fill's average pixel colors is one of skip_empty, all, all_premultiplied, including_alpha, and this is "sum". | yes |
| fx_eyefill_031.json is refused in a sentence | Eyedropper Fill's maintain original alpha is "off" or "on", and this is "yes". | yes |
| a file with an Eyedropper Fill with no `blend_with_original` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Eyedropper Fill whose sample point is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Eyedropper Fill whose maintain original alpha is true is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| sample point 1001, 50 is refused with a sentence, and nothing changes | Eyedropper Fill's sample point runs from -1000 to 1000, and this is 1001. | yes |
| sample radius -1 is refused with a sentence, and nothing changes | Eyedropper Fill's sample radius runs from 0 to 10000, and this is -1. | yes |
| blend with original 101 is refused with a sentence, and nothing changes | Eyedropper Fill's blend with original runs from 0 to 100, and this is 101. | yes |
| average pixel colors "sum" is refused with a sentence, and nothing changes | Eyedropper Fill's average pixel colors is one of skip_empty, all, all_premultiplied, including_alpha, and this is "sum". | yes |
| maintain original alpha "yes" is refused with a sentence, and nothing changes | Eyedropper Fill's maintain original alpha is "off" or "on", and this is "yes". | yes |
| blend with original keyed to 101 is refused with a sentence, and nothing changes | Eyedropper Fill's blend with original runs from 0 to 100, and this is 101. | yes |
| sample point 12.5, 50, radius 2, Including Alpha, alpha kept, blend 30 is taken | taken | yes |
| sample point keyed from 50, 50 to 12.5, 50 is taken | taken | yes |
| sample radius keyed from 0 to 4 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_eyefill_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_eyefill_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_eyefill_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_eyefill_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_eyefill_017.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_eyefill_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_eyefill_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_009.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_eyefill_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_eyefill_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 4 of 10 frames; the same warnings: true | yes |
| fx_eyefill_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_eyefill_024.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_eyefill_025.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_eyefill_026.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_eyefill_027.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_eyefill_028.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_eyefill_029.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_eyefill_030.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_eyefill_031.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_eyefill_032.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Eyedropper Fill as added (the middle pixel): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Eyedropper Fill as added (the middle pixel) on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill as added (the middle pixel) on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill as added (the middle pixel) on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill as added (the middle pixel) on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill as added (the middle pixel) on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill as added (the middle pixel) on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill point 30, 40, radius 50, All: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Eyedropper Fill point 30, 40, radius 50, All on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill point 30, 40, radius 50, All on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill point 30, 40, radius 50, All on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill point 30, 40, radius 50, All on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill point 30, 40, radius 50, All on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill point 30, 40, radius 50, All on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill radius 200, Including Alpha, alpha kept: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073599 pixels changed | yes |
| the reference shot, Eyedropper Fill radius 200, Including Alpha, alpha kept on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill radius 200, Including Alpha, alpha kept on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill radius 200, Including Alpha, alpha kept on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill radius 200, Including Alpha, alpha kept on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill radius 200, Including Alpha, alpha kept on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill radius 200, Including Alpha, alpha kept on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill radius 1000, blend 50: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Eyedropper Fill radius 1000, blend 50 on three layers, frame 0, Full | largest difference 1 of 255, 171 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill radius 1000, blend 50 on three layers, frame 100, Full | largest difference 1 of 255, 554 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill radius 1000, blend 50 on three layers, frame 239, Full | largest difference 1 of 255, 151 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill radius 1000, blend 50 on three layers, frame 0, Draft | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill radius 1000, blend 50 on three layers, frame 100, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill radius 1000, blend 50 on three layers, frame 239, Draft | largest difference 1 of 255, 29 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill point 75, 25, radius 10, All Premultiplied, alpha kept: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073407 pixels changed | yes |
| the reference shot, Eyedropper Fill point 75, 25, radius 10, All Premultiplied, alpha kept on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill point 75, 25, radius 10, All Premultiplied, alpha kept on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill point 75, 25, radius 10, All Premultiplied, alpha kept on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill point 75, 25, radius 10, All Premultiplied, alpha kept on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill point 75, 25, radius 10, All Premultiplied, alpha kept on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Eyedropper Fill point 75, 25, radius 10, All Premultiplied, alpha kept on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-419 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as added: the whole street the colour of its middle pixel; draws cleanly | [], 121140 pixels changed of 129600 | yes |
| 3_radius_60.png, radius 60: the whole street the average colour about its middle; draws cleanly | [], 129600 pixels changed of 129600 | yes |
| 4_upper_left_kept.png, point 20, 20, radius 30, alpha kept, blend 40: the upper left's colour over the street, the street showing through; draws cleanly | [], 127200 pixels changed of 129600 | yes |

## Result

212 of 212 checks pass.
