# D-405: Magnify

B-284, after After Effects' Magnify: a circle or square of radius Size round Center, the layer read in it at the centre plus the offset over the magnification (standard the pixel holding the place, soft document 21's bilinear sample, scatter the place nudged by Noise's hash), faded over Feather inside its edge and by Opacity, then laid over the layer by the blending mode (document 21's layer blend), or alone for None. Resize Layer grows the layer to hold the area unless Size is linked. Every expected pixel is `Fixtures/magnify/expected_magnify.json`, written by `tools/magnify_reference.py` before this code existed and printed in document 25 as FX-MAGNIFY-001 to 034. Tolerance 2e-5.

## FX-MAGNIFY-001 to 034 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MAGNIFY-001 frame 0: The settings as they start: a circle of radius 100 round the middle, magnification 200, standard scaling, Normal: the whole picture enlarged twice round (8, 5), blocks of 2 by 2 pixels, laid over the drawing, which shows through only where the enlargement is clear or soft (After Effects' doubling). | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-001 frame 4: The settings as they start: a circle of radius 100 round the middle, magnification 200, standard scaling, Normal: the whole picture enlarged twice round (8, 5), blocks of 2 by 2 pixels, laid over the drawing, which shows through only where the enlargement is clear or soft (After Effects' doubling). | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-002 frame 0: Magnification 100 with blending mode None: the area reads each pixel itself, so the drawing comes back untouched. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-003 frame 0: Size 4: a circle of radius 4 round (8, 5) enlarged twice, the drawing round it. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-004 frame 0: Size 4, square: the area a square 8 by 8 round (8, 5). | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-005 frame 0: Size 4, feather 2: the circle fades over the last 2 pixels inside its edge. | largest difference 2.5e-7 | yes |
| FX-MAGNIFY-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-006 frame 0: Link size to magnification, size 2: the radius 200 per cent of 2, so FX-MAGNIFY-003 exactly. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-007 frame 0: Link size and feather, size 2, feather 1: radius 4, feather 2, so FX-MAGNIFY-005 exactly. | largest difference 2.5e-7 | yes |
| FX-MAGNIFY-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-008 frame 0: Size 4, opacity 50: the area at half its covering, over the drawing. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-009 frame 0: Soft scaling: as FX-MAGNIFY-001 with each place read by document 21's bilinear sample, smooth instead of blocky. | largest difference 2.5e-7 | yes |
| FX-MAGNIFY-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-010 frame 0: Scatter scaling: as FX-MAGNIFY-001 with each place nudged by up to a half a picture pixel each way (j = 1/2 at 200 per cent) by Noise's hash, so the blocks' edges break up. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-011 frame 0: Size 4, blending mode None: the circle alone, clear round it. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-012 frame 0: Size 5, Multiply over the drawing. | largest difference 3.2e-7 | yes |
| FX-MAGNIFY-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-013 frame 0: Size 5, Screen. | largest difference 2.2e-7 | yes |
| FX-MAGNIFY-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-014 frame 0: Size 5, Add, each straight colour held to 1. | largest difference 2.1e-7 | yes |
| FX-MAGNIFY-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-015 frame 0: Size 5, Overlay, on the encoded colours. | largest difference 2.7e-7 | yes |
| FX-MAGNIFY-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-016 frame 0: Size 5, Soft Light, on the encoded colours. | largest difference 2.6e-7 | yes |
| FX-MAGNIFY-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-017 frame 0: Resize Layer on, size 4, centre at 90, 50 (14.4, 5), the layer moved 3 pixels left: the circle reaches 2.4 pixels past the right edge, so the layer grows 3 pixels on every side and the area shows past the drawing's edge, in columns 13 to 15. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-018 frame 0: FX-MAGNIFY-017 with the link at size (size 2, so radius 4): After Effects turns Resize Layer off when linked, so nothing grows and the circle is cut at the drawing's edge. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-019 frame 0: Magnification keyed from 100 at frame 0 to 400 at frame 4, size 4, linear: 175 at frame 1, 400 at frame 4. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-019 frame 1: Magnification keyed from 100 at frame 0 to 400 at frame 4, size 4, linear: 175 at frame 1, 400 at frame 4. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-019 frame 4: Magnification keyed from 100 at frame 0 to 400 at frame 4, size 4, linear: 175 at frame 1, 400 at frame 4. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-020 frame 0: Centre keyed from 25, 50 at frame 0 to 75, 50 at frame 4, size 3: the lens slides across. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-020 frame 2: Centre keyed from 25, 50 at frame 0 to 75, 50 at frame 4, size 3: the lens slides across. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-020 frame 4: Centre keyed from 25, 50 at frame 0 to 75, 50 at frame 4, size 3: the lens slides across. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-021 frame 0: Size 4 with the layer moved 3 pixels right: the same area, moved; columns 0 to 2 empty. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-022 frame 0: Magnification 300, size 100: blocks of 3 by 3 pixels. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-023 frame 0: Size eased from 4 at frame 0 to 0 at frame 4 on a curve that overshoots: at frame 2 it would pass below 0 and is held there, so frames 2 and 4 show no area, the drawing as it is. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-023 frame 2: Size eased from 4 at frame 0 to 0 at frame 4 on a curve that overshoots: at frame 2 it would pass below 0 and is held there, so frames 2 and 4 show no area, the drawing as it is. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-023 frame 4: Size eased from 4 at frame 0 to 0 at frame 4 on a curve that overshoots: at frame 2 it would pass below 0 and is held there, so frames 2 and 4 show no area, the drawing as it is. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MAGNIFY-024 frame 0: Magnification 99, below 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-024 frame 4: Magnification 99, below 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MAGNIFY-025 frame 0: Magnification 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-025 frame 4: Magnification 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MAGNIFY-026 frame 0: Size -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-026 frame 4: Size -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MAGNIFY-027 frame 0: Feather 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-027 frame 4: Feather 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MAGNIFY-028 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-028 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MAGNIFY-029 frame 0: Centre at 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-029 frame 4: Centre at 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MAGNIFY-030 frame 0: Shape "oval". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-030 frame 4: Shape "oval". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MAGNIFY-031 frame 0: Link "feather". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-031 frame 4: Link "feather". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MAGNIFY-032 frame 0: Scaling "bicubic". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-032 frame 4: Scaling "bicubic". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MAGNIFY-033 frame 0: Blending mode "difference", which this build does not offer. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-033 frame 4: Blending mode "difference", which this build does not offer. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MAGNIFY-034 frame 0: Resize layer "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-034 frame 4: Resize layer "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MAGNIFY-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_magnify_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_magnify_017.json is saved with its words and numbers | {"blending_mode":"normal","center":[90,50],"feather":0,"link":"none","magnification":200,"opacity":100,"resize_layer":"on","scaling":"standard","shape":"circle","size":4} | yes |
| fx_magnify_024.json is refused in a sentence | Magnify's magnification runs from 100 to 1000, and this is 99. | yes |
| fx_magnify_025.json is refused in a sentence | Magnify's magnification runs from 100 to 1000, and this is 1001. | yes |
| fx_magnify_026.json is refused in a sentence | Magnify's size runs from 0 to 1000, and this is -1. | yes |
| fx_magnify_027.json is refused in a sentence | Magnify's feather runs from 0 to 1000, and this is 1001. | yes |
| fx_magnify_028.json is refused in a sentence | Magnify's opacity runs from 0 to 100, and this is 101. | yes |
| fx_magnify_029.json is refused in a sentence | Magnify's center runs from -1000 to 1000, and this is 1001. | yes |
| fx_magnify_030.json is refused in a sentence | Magnify's shape is "circle" or "square", and this is "oval". | yes |
| fx_magnify_031.json is refused in a sentence | Magnify's link is "none", "size" or "size_feather", and this is "feather". | yes |
| fx_magnify_032.json is refused in a sentence | Magnify's scaling is "standard", "soft" or "scatter", and this is "bicubic". | yes |
| fx_magnify_033.json is refused in a sentence | Magnify's blending mode is "none", "normal", "add", "multiply", "screen", "overlay" or "soft_light", and this is "difference". | yes |
| fx_magnify_034.json is refused in a sentence | Magnify's resize layer is "off" or "on", and this is "yes". | yes |
| a file with a Magnify with no `scaling` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Magnify with a magnification in words is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| magnification 99 is refused with a sentence, and nothing changes | Magnify's magnification runs from 100 to 1000, and this is 99. | yes |
| shape "oval" is refused with a sentence, and nothing changes | Magnify's shape is "circle" or "square", and this is "oval". | yes |
| blending mode "difference" is refused with a sentence, and nothing changes | Magnify's blending mode is "none", "normal", "add", "multiply", "screen", "overlay" or "soft_light", and this is "difference". | yes |
| size keyed to -1 is refused with a sentence, and nothing changes | Magnify's size runs from 0 to 1000, and this is -1. | yes |
| a square, size 3, feather 1, opacity 60, soft, Screen, resized is taken | taken | yes |
| magnification keyed from 100 to 400 is taken | taken | yes |
| centre keyed from 25, 50 to 75, 50 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_magnify_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_magnify_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_magnify_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_magnify_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_magnify_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_magnify_020.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_magnify_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_magnify_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_magnify_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_magnify_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_magnify_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_magnify_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_magnify_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_magnify_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_magnify_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_magnify_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_magnify_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_magnify_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Magnify as it starts (a circle of radius 100, magnification 200, Normal): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 31415 pixels changed | yes |
| the reference shot, Magnify as it starts (a circle of radius 100, magnification 200, Normal) on three layers, frame 0, Full | largest difference 1 of 255, 3555 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify as it starts (a circle of radius 100, magnification 200, Normal) on three layers, frame 100, Full | largest difference 1 of 255, 1368 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify as it starts (a circle of radius 100, magnification 200, Normal) on three layers, frame 239, Full | largest difference 1 of 255, 1557 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify as it starts (a circle of radius 100, magnification 200, Normal) on three layers, frame 0, Draft | largest difference 1 of 255, 121 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify as it starts (a circle of radius 100, magnification 200, Normal) on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify as it starts (a circle of radius 100, magnification 200, Normal) on three layers, frame 239, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify a square of radius 200, feather 30, magnification 300, soft, Overlay: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 157892 pixels changed | yes |
| the reference shot, Magnify a square of radius 200, feather 30, magnification 300, soft, Overlay on three layers, frame 0, Full | largest difference 1 of 255, 870 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify a square of radius 200, feather 30, magnification 300, soft, Overlay on three layers, frame 100, Full | largest difference 1 of 255, 1087 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify a square of radius 200, feather 30, magnification 300, soft, Overlay on three layers, frame 239, Full | largest difference 1 of 255, 859 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify a square of radius 200, feather 30, magnification 300, soft, Overlay on three layers, frame 0, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify a square of radius 200, feather 30, magnification 300, soft, Overlay on three layers, frame 100, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify a square of radius 200, feather 30, magnification 300, soft, Overlay on three layers, frame 239, Draft | largest difference 1 of 255, 41 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify radius 300 round 30, 40, scatter, Soft Light, opacity 70, Resize Layer on: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 259295 pixels changed | yes |
| the reference shot, Magnify radius 300 round 30, 40, scatter, Soft Light, opacity 70, Resize Layer on on three layers, frame 0, Full | largest difference 1 of 255, 3457 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify radius 300 round 30, 40, scatter, Soft Light, opacity 70, Resize Layer on on three layers, frame 100, Full | largest difference 1 of 255, 1074 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify radius 300 round 30, 40, scatter, Soft Light, opacity 70, Resize Layer on on three layers, frame 239, Full | largest difference 1 of 255, 1368 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify radius 300 round 30, 40, scatter, Soft Light, opacity 70, Resize Layer on on three layers, frame 0, Draft | largest difference 1 of 255, 128 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify radius 300 round 30, 40, scatter, Soft Light, opacity 70, Resize Layer on on three layers, frame 100, Draft | largest difference 1 of 255, 43 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify radius 300 round 30, 40, scatter, Soft Light, opacity 70, Resize Layer on on three layers, frame 239, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify None, size and feather linked (100 and 10 at 250 per cent): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073424 pixels changed | yes |
| the reference shot, Magnify None, size and feather linked (100 and 10 at 250 per cent) on three layers, frame 0, Full | largest difference 1 of 255, 2166 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify None, size and feather linked (100 and 10 at 250 per cent) on three layers, frame 100, Full | largest difference 1 of 255, 781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify None, size and feather linked (100 and 10 at 250 per cent) on three layers, frame 239, Full | largest difference 1 of 255, 1441 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify None, size and feather linked (100 and 10 at 250 per cent) on three layers, frame 0, Draft | largest difference 1 of 255, 88 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify None, size and feather linked (100 and 10 at 250 per cent) on three layers, frame 100, Draft | largest difference 1 of 255, 28 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify None, size and feather linked (100 and 10 at 250 per cent) on three layers, frame 239, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify Add, radius 250: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 196364 pixels changed | yes |
| the reference shot, Magnify Add, radius 250 on three layers, frame 0, Full | largest difference 1 of 255, 393 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify Add, radius 250 on three layers, frame 100, Full | largest difference 1 of 255, 803 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify Add, radius 250 on three layers, frame 239, Full | largest difference 1 of 255, 412 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify Add, radius 250 on three layers, frame 0, Draft | largest difference 1 of 255, 26 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify Add, radius 250 on three layers, frame 100, Draft | largest difference 1 of 255, 26 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify Add, radius 250 on three layers, frame 239, Draft | largest difference 1 of 255, 20 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify Multiply, radius 250, soft: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 196364 pixels changed | yes |
| the reference shot, Magnify Multiply, radius 250, soft on three layers, frame 0, Full | largest difference 1 of 255, 969 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify Multiply, radius 250, soft on three layers, frame 100, Full | largest difference 1 of 255, 1107 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify Multiply, radius 250, soft on three layers, frame 239, Full | largest difference 1 of 255, 896 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify Multiply, radius 250, soft on three layers, frame 0, Draft | largest difference 1 of 255, 79 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify Multiply, radius 250, soft on three layers, frame 100, Draft | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify Multiply, radius 250, soft on three layers, frame 239, Draft | largest difference 1 of 255, 70 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify Screen, radius 250, scatter at 600 per cent: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 196364 pixels changed | yes |
| the reference shot, Magnify Screen, radius 250, scatter at 600 per cent on three layers, frame 0, Full | largest difference 1 of 255, 710 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify Screen, radius 250, scatter at 600 per cent on three layers, frame 100, Full | largest difference 1 of 255, 934 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify Screen, radius 250, scatter at 600 per cent on three layers, frame 239, Full | largest difference 1 of 255, 594 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify Screen, radius 250, scatter at 600 per cent on three layers, frame 0, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify Screen, radius 250, scatter at 600 per cent on three layers, frame 100, Draft | largest difference 1 of 255, 39 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Magnify Screen, radius 250, scatter at 600 per cent on three layers, frame 239, Draft | largest difference 1 of 255, 39 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-405 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: a circle of radius 100 in the middle, the street in it twice as big, in 2 by 2 blocks; draws cleanly | [], 27215 pixels changed | yes |
| 3_square_300.png, a square 120 across, three times as big; draws cleanly | [], 11448 pixels changed | yes |
| 4_soft_feather.png, soft scaling, feather 30: smooth, the edge fading into the street; draws cleanly | [], 16174 pixels changed | yes |
| 5_scatter.png, scatter at 400 per cent: the blocks' edges broken up; draws cleanly | [], 16913 pixels changed | yes |
| 6_none.png, blending mode None: the lens alone, clear round it; draws cleanly | [], 125564 pixels changed | yes |
| 7_screen.png, Screen: the enlarged street lightening the street under it; draws cleanly | [], 20108 pixels changed | yes |

## Result

238 of 238 checks pass.
