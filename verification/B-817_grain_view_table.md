# B-817: Add Grain's Viewing Mode

D-937, accepted on 2026-10-27 under the owner's request for the tutorial follow-alongs. Every expected pixel is `Fixtures/addgrain/expected_addgrain_view.json`, written by `tools/addgrain_view_reference.py` before this code existed and printed in document 25 as FX-ADDGRAIN-042 to 047. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance.

## FX-ADDGRAIN-042 to 047 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-ADDGRAIN-042 frame 0: Preview on a strip 220 by 12, wider than the box: columns 10 to 209 are the box, its edge (columns 10 and 209, rows 0 and 11) opaque white and the grain inside it FX-ADDGRAIN-043's exactly; columns 0 to 9 and 210 to 219 are the strip untouched. | largest difference 1.5e-7 | yes |
| FX-ADDGRAIN-042 frame 2: Preview on a strip 220 by 12, wider than the box: columns 10 to 209 are the box, its edge (columns 10 and 209, rows 0 and 11) opaque white and the grain inside it FX-ADDGRAIN-043's exactly; columns 0 to 9 and 210 to 219 are the strip untouched. | largest difference 1.5e-7 | yes |
| FX-ADDGRAIN-042: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-043 frame 0: Final Output written on the strip: the grain everywhere, D-443 exactly. | largest difference 1.6e-7 | yes |
| FX-ADDGRAIN-043 frame 2: Final Output written on the strip: the grain everywhere, D-443 exactly. | largest difference 1.6e-7 | yes |
| FX-ADDGRAIN-043: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-044 frame 0: Preview at intensity 0 on the strip: the box is drawn and nothing else changes. | largest difference 1.4e-7 | yes |
| FX-ADDGRAIN-044: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-045 frame 0: Preview on the 16 by 10 card, smaller than the box: the box is the card's own edge, its border pixels white (the empty ones too), the grain inside FX-ADDGRAIN-001's frame 0. | largest difference 2.6e-7 | yes |
| FX-ADDGRAIN-045: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-046 frame 0: Preview after a Motion Tile of 300 per cent, the layer moved three pixels right: the box is still the drawing's edge, in the drawing's own space, so columns 0 to 2 show the tiles round it untouched, column 3 and rows 0 and 9 are white. | largest difference 2.6e-7 | yes |
| FX-ADDGRAIN-046: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADDGRAIN-047 frame 0: Viewing Mode "noise_samples", After Effects' Remove Grain word, which this Add Grain has not got. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-047 frame 4: Viewing Mode "noise_samples", After Effects' Remove Grain word, which this Add Grain has not got. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ADDGRAIN-047: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## A wrong word is refused (what it warns)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_addgrain_047.json: the warning names viewing mode | Add Grain's viewing mode is "final_output" or "preview", and this is "noise_samples". | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_addgrain_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_addgrain_042.json is saved with viewing mode preview | {"animate_smoothly":"on","animation_speed":1,"aspect_ratio":1,"blending_mode":"film","blue_intensity":1,"green_intensity":1,"highlights":1,"intensity":1,"midpoint":0.5,"midtones":1,"monochromatic":"off","random_seed":0,"red_intensity":1,"saturation":1,"shadows":1,"size":1,"softness":0,"viewing_mode":"preview"} | yes |
| fx_addgrain_001.json, from before it, is saved without the word | {"animate_smoothly":"on","animation_speed":1,"aspect_ratio":1,"blending_mode":"film","blue_intensity":1,"green_intensity":1,"highlights":1,"intensity":1,"midpoint":0.5,"midtones":1,"monochromatic":"off","random_seed":0,"red_intensity":1,"saturation":1,"shadows":1,"size":1,"softness":0} | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_addgrain_042.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_addgrain_046.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

23 of 23 checks pass.
