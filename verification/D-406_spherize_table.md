# D-406: Spherize

B-285, after After Effects' Spherize: each pixel within Radius of the Center of Sphere, rho of the way out, reads the layer at the centre plus its offset times 2 asin(rho) / (pi rho), by document 21's bilinear sample, a half sphere seen from in front; the pixels outside the sphere are left as they are. Every expected pixel is `Fixtures/spherize/expected_spherize.json`, written by `tools/spherize_reference.py` before this code existed and printed in document 25 as FX-SPHERIZE-001 to 015. Tolerance 2e-5.

## FX-SPHERIZE-001 to 015 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SPHERIZE-001 frame 0: The settings as they start: a sphere of radius 100 round the middle, far wider than the 16 by 10 drawing, so every pixel is near its top and the whole drawing is enlarged about pi / 2 times round (8, 5). | largest difference 2.5e-7 | yes |
| FX-SPHERIZE-001 frame 4: The settings as they start: a sphere of radius 100 round the middle, far wider than the 16 by 10 drawing, so every pixel is near its top and the whole drawing is enlarged about pi / 2 times round (8, 5). | largest difference 2.5e-7 | yes |
| FX-SPHERIZE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPHERIZE-002 frame 0: Radius 0: no sphere, the drawing as it is. | largest difference 1.9e-7 | yes |
| FX-SPHERIZE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPHERIZE-003 frame 0: Radius 5 round the middle: the middle swells, the picture crowds toward the rim, and the corners outside the circle are the drawing's own. | largest difference 2.5e-7 | yes |
| FX-SPHERIZE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPHERIZE-004 frame 0: Radius 5 round 25, 50 (4, 5): the sphere on the left, cut by the drawing's edge. | largest difference 2.5e-7 | yes |
| FX-SPHERIZE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPHERIZE-005 frame 0: Radius 2500, the most: rho under 0.004 everywhere, so the drawing is enlarged pi / 2 times round the middle, smoothly. | largest difference 2.5e-7 | yes |
| FX-SPHERIZE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPHERIZE-006 frame 0: Radius 12 round -25, 50 (-4, 5), off the left edge: only the right part of the sphere lies on the drawing, its rim near column 7. | largest difference 2.5e-7 | yes |
| FX-SPHERIZE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPHERIZE-007 frame 0: Radius 3: a small sphere, nine or so pixels round the middle changed. | largest difference 2.5e-7 | yes |
| FX-SPHERIZE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPHERIZE-008 frame 0: Radius keyed from 0 at frame 0 to 8 at frame 4, linear: nothing at frame 0, 4 at frame 2, 8 at frame 4. | largest difference 1.9e-7 | yes |
| FX-SPHERIZE-008 frame 2: Radius keyed from 0 at frame 0 to 8 at frame 4, linear: nothing at frame 0, 4 at frame 2, 8 at frame 4. | largest difference 2.5e-7 | yes |
| FX-SPHERIZE-008 frame 4: Radius keyed from 0 at frame 0 to 8 at frame 4, linear: nothing at frame 0, 4 at frame 2, 8 at frame 4. | largest difference 2.5e-7 | yes |
| FX-SPHERIZE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPHERIZE-009 frame 0: Centre keyed from 25, 50 at frame 0 to 75, 50 at frame 4, radius 4: the sphere slides across. | largest difference 2.5e-7 | yes |
| FX-SPHERIZE-009 frame 2: Centre keyed from 25, 50 at frame 0 to 75, 50 at frame 4, radius 4: the sphere slides across. | largest difference 2.5e-7 | yes |
| FX-SPHERIZE-009 frame 4: Centre keyed from 25, 50 at frame 0 to 75, 50 at frame 4, radius 4: the sphere slides across. | largest difference 2.5e-7 | yes |
| FX-SPHERIZE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPHERIZE-010 frame 0: Radius 5 with the layer moved 3 pixels right: FX-SPHERIZE-003 moved; columns 0 to 2 empty. | largest difference 2.5e-7 | yes |
| FX-SPHERIZE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPHERIZE-011 frame 0: Radius eased from 5 at frame 0 to 0 at frame 4 on a curve that overshoots: at frame 2 it would pass below 0 and is held there, so frames 2 and 4 show the drawing as it is. | largest difference 2.5e-7 | yes |
| FX-SPHERIZE-011 frame 2: Radius eased from 5 at frame 0 to 0 at frame 4 on a curve that overshoots: at frame 2 it would pass below 0 and is held there, so frames 2 and 4 show the drawing as it is. | largest difference 1.9e-7 | yes |
| FX-SPHERIZE-011 frame 4: Radius eased from 5 at frame 0 to 0 at frame 4 on a curve that overshoots: at frame 2 it would pass below 0 and is held there, so frames 2 and 4 show the drawing as it is. | largest difference 1.9e-7 | yes |
| FX-SPHERIZE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPHERIZE-012 frame 0: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPHERIZE-012 frame 4: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPHERIZE-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPHERIZE-013 frame 0: Radius 2501, above 2500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPHERIZE-013 frame 4: Radius 2501, above 2500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPHERIZE-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPHERIZE-014 frame 0: Centre at 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPHERIZE-014 frame 4: Centre at 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPHERIZE-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPHERIZE-015 frame 0: Centre at 50, -1001. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPHERIZE-015 frame 4: Centre at 50, -1001. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPHERIZE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_spherize_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spherize_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spherize_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spherize_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spherize_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spherize_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spherize_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spherize_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spherize_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spherize_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spherize_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spherize_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spherize_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spherize_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spherize_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spherize_004.json is saved with its radius and centre | {"center":[25,50],"radius":5} | yes |
| fx_spherize_012.json is refused in a sentence | Spherize's radius runs from 0 to 2500, and this is -1. | yes |
| fx_spherize_013.json is refused in a sentence | Spherize's radius runs from 0 to 2500, and this is 2501. | yes |
| fx_spherize_014.json is refused in a sentence | Spherize's center runs from -1000 to 1000, and this is 1001. | yes |
| fx_spherize_015.json is refused in a sentence | Spherize's center runs from -1000 to 1000, and this is -1001. | yes |
| a file with a Spherize with no `center` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Spherize with a radius in words is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Spherize with one number for its centre is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| radius -1 is refused with a sentence, and nothing changes | Spherize's radius runs from 0 to 2500, and this is -1. | yes |
| centre 50, 1001 is refused with a sentence, and nothing changes | Spherize's center runs from -1000 to 1000, and this is 1001. | yes |
| radius keyed to 2501 is refused with a sentence, and nothing changes | Spherize's radius runs from 0 to 2500, and this is 2501. | yes |
| radius 6 round 30, 40 is taken | taken | yes |
| radius keyed from 0 to 8 is taken | taken | yes |
| centre keyed from 25, 50 to 75, 50 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_spherize_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spherize_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spherize_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spherize_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spherize_009.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spherize_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_spherize_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spherize_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_spherize_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spherize_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spherize_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spherize_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spherize_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spherize_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_spherize_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spherize_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_spherize_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 4 of 10 frames; the same warnings: true | yes |
| fx_spherize_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_spherize_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_spherize_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_spherize_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Spherize as it starts (radius 100 in the middle): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 31184 pixels changed | yes |
| the reference shot, Spherize as it starts (radius 100 in the middle) on three layers, frame 0, Full | largest difference 1 of 255, 3386 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize as it starts (radius 100 in the middle) on three layers, frame 100, Full | largest difference 1 of 255, 1313 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize as it starts (radius 100 in the middle) on three layers, frame 239, Full | largest difference 1 of 255, 1400 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize as it starts (radius 100 in the middle) on three layers, frame 0, Draft | largest difference 1 of 255, 119 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize as it starts (radius 100 in the middle) on three layers, frame 100, Draft | largest difference 1 of 255, 34 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize as it starts (radius 100 in the middle) on three layers, frame 239, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize radius 500 round 30, 40: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 723989 pixels changed | yes |
| the reference shot, Spherize radius 500 round 30, 40 on three layers, frame 0, Full | largest difference 1 of 255, 2729 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize radius 500 round 30, 40 on three layers, frame 100, Full | largest difference 1 of 255, 1004 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize radius 500 round 30, 40 on three layers, frame 239, Full | largest difference 1 of 255, 796 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize radius 500 round 30, 40 on three layers, frame 0, Draft | largest difference 1 of 255, 123 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize radius 500 round 30, 40 on three layers, frame 100, Draft | largest difference 1 of 255, 70 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize radius 500 round 30, 40 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize radius 2500, the whole layer inside: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2045948 pixels changed | yes |
| the reference shot, Spherize radius 2500, the whole layer inside on three layers, frame 0, Full | largest difference 1 of 255, 1388 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize radius 2500, the whole layer inside on three layers, frame 100, Full | largest difference 1 of 255, 931 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize radius 2500, the whole layer inside on three layers, frame 239, Full | largest difference 1 of 255, 1101 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize radius 2500, the whole layer inside on three layers, frame 0, Draft | largest difference 1 of 255, 71 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize radius 2500, the whole layer inside on three layers, frame 100, Draft | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize radius 2500, the whole layer inside on three layers, frame 239, Draft | largest difference 1 of 255, 65 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize radius 400 round -10, 50, half off the edge: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 102020 pixels changed | yes |
| the reference shot, Spherize radius 400 round -10, 50, half off the edge on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize radius 400 round -10, 50, half off the edge on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize radius 400 round -10, 50, half off the edge on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize radius 400 round -10, 50, half off the edge on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize radius 400 round -10, 50, half off the edge on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spherize radius 400 round -10, 50, half off the edge on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-406 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: a circle of radius 100 in the middle swelling toward you; draws cleanly | [], 24342 pixels changed | yes |
| 3_radius_130.png, radius 130: the sphere nearly the street's height; draws cleanly | [], 41805 pixels changed | yes |
| 4_off_centre.png, radius 80 round 25, 40, over the left houses; draws cleanly | [], 17497 pixels changed | yes |
| 5_whole.png, radius 2500: the whole street inside the sphere, enlarged about pi / 2 times from the middle; draws cleanly | [], 114610 pixels changed | yes |
| 6_edge.png, radius 120 round 0, 50: half a sphere on the left edge; draws cleanly | [], 17127 pixels changed | yes |

## Result

126 of 126 checks pass.
