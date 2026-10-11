# D-502: Inner/Outer Key

B-382, after After Effects' Inner/Outer Key, the rule ours (Adobe publishes none): the foreground mask (its number's floor, on with two points or more) covers the kept pixels by ADR-016's 4 by 4 even-odd rule; the background mask's outside is gone, or with no background the pixels whose centres lie within the highlight radius of the outline are unknown, the rest kept inside it and gone outside; Invert Extraction swaps kept and gone. Each unknown pixel takes the mean colour of the kept and of the gone pixels in the square reaching the nearest of each, is made see-through as much as it is the gone colour along the line between them, and has that colour taken back out. Then Edge Thin (a disc's minimum, or maximum when negative), Edge Feather (a Gaussian of sigma = feather / 2), Edge Threshold, and Blend With Original. Foreground 0 keys nothing; a mask named and missing leaves the layer as it is with a warning each frame. The layer never grows. Every expected pixel is `Fixtures/inner_outer_key/expected_inner_outer_key.json`, written by `tools/inner_outer_key_reference.py` before this code existed and printed in document 25 as FX-IOK-001 to 039. Tolerance 2e-5.

## FX-IOK-001 to 039 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-IOK-001 frame 0: The settings as they start: Foreground 0 (none): nothing is keyed, the picture as it was. | largest difference 1.3e-7 | yes |
| FX-IOK-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-002 frame 0: Foreground mask 1 just inside the yellow block, Background mask 2 two pixels outside it: the block kept, the blue outside the outer mask gone, the edge mixtures between them see-through as much as they are blue, their colour the yellow without the blue. | largest difference 1.1e-7 | yes |
| FX-IOK-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-003 frame 0: One mask, the inner one, Single Mask Highlight Radius 1.2: the pixels whose centres lie within 1.2 of its outline are worked out, the rest kept inside it and gone outside. | largest difference 1.1e-7 | yes |
| FX-IOK-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-004 frame 0: One mask with the radius as added, 10: every pixel of this small picture lies within 10 of the outline, so nothing is surely kept and every pixel goes. | largest difference 0.0e0 | yes |
| FX-IOK-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-005 frame 0: FX-IOK-002 with Invert Extraction: the block is gone and the blue kept. | largest difference 9.3e-8 | yes |
| FX-IOK-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-006 frame 0: Edge Thin 1: the matte shrinks a pixel all round. | largest difference 9.7e-8 | yes |
| FX-IOK-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-007 frame 0: Edge Thin -1: the matte grows a pixel all round. | largest difference 2.1e-7 | yes |
| FX-IOK-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-008 frame 0: Edge Feather 2: the matte's edge softens. | largest difference 1.2e-7 | yes |
| FX-IOK-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-009 frame 0: Edge Threshold 50: the half-kept pixels and less go, the rest harden. | largest difference 9.7e-8 | yes |
| FX-IOK-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-010 frame 0: Blend With Original 50: half way between FX-IOK-002 and the picture. | largest difference 1.0e-7 | yes |
| FX-IOK-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-011 frame 0: Edge Thin 1.5, Edge Feather 3 and Edge Threshold 20 together, in that order. | largest difference 9.1e-8 | yes |
| FX-IOK-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-012 frame 0: One mask, the radius keyed from 0 at frame 0 to 1.2 at frame 4: a hard cut along the outline at frame 0; at frame 2, 0.6, the pixels either side of its straight sides are worked out; at frame 4 its corners too. | largest difference 9.7e-8 | yes |
| FX-IOK-012 frame 2: One mask, the radius keyed from 0 at frame 0 to 1.2 at frame 4: a hard cut along the outline at frame 0; at frame 2, 0.6, the pixels either side of its straight sides are worked out; at frame 4 its corners too. | largest difference 1.1e-7 | yes |
| FX-IOK-012 frame 4: One mask, the radius keyed from 0 at frame 0 to 1.2 at frame 4: a hard cut along the outline at frame 0; at frame 2, 0.6, the pixels either side of its straight sides are worked out; at frame 4 its corners too. | largest difference 1.1e-7 | yes |
| FX-IOK-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-013 frame 0: Blend With Original keyed from 0 at frame 0 to 100 at frame 4: FX-IOK-002, half way, then the picture. | largest difference 1.1e-7 | yes |
| FX-IOK-013 frame 2: Blend With Original keyed from 0 at frame 0 to 100 at frame 4: FX-IOK-002, half way, then the picture. | largest difference 1.0e-7 | yes |
| FX-IOK-013 frame 4: Blend With Original keyed from 0 at frame 0 to 100 at frame 4: FX-IOK-002, half way, then the picture. | largest difference 1.3e-7 | yes |
| FX-IOK-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-014 frame 0: The inner mask's path keyed two rows lower by frame 4: the kept part follows it, frames 0, 2 and 4. | largest difference 1.1e-7 | yes |
| FX-IOK-014 frame 2: The inner mask's path keyed two rows lower by frame 4: the kept part follows it, frames 0, 2 and 4. | largest difference 1.1e-7 | yes |
| FX-IOK-014 frame 4: The inner mask's path keyed two rows lower by frame 4: the kept part follows it, frames 0, 2 and 4. | largest difference 1.1e-7 | yes |
| FX-IOK-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-015 frame 0: FX-IOK-002 moved three pixels right: the key moves with the layer. | largest difference 1.1e-7 | yes |
| FX-IOK-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-016 frame 0: After a Motion Tile that grows the layer: the masks are the drawing's own, so they key the drawing's place in the grown buffer, whose copies lie outside the outer mask and are gone. | largest difference 1.1e-7 | yes |
| FX-IOK-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-017 frame 0: Foreground 1.5 and Background 2.7: their floors, FX-IOK-002's frame. | largest difference 1.1e-7 | yes |
| FX-IOK-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-018 frame 0: Foreground and Background the same mask: nothing is unknown, a hard cut along it. | largest difference 9.7e-8 | yes |
| FX-IOK-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-019 frame 0: A foreground mask right of the picture: nothing is kept, so every pixel goes. | largest difference 0.0e0 | yes |
| FX-IOK-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-020 frame 0: A background mask round the whole picture: nothing is gone, so every pixel is kept as it was. | largest difference 1.3e-7 | yes |
| FX-IOK-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-021 frame 0: One curved mask, a circle 8 across about (8, 5), radius 1.2. | largest difference 1.1e-7 | yes |
| FX-IOK-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-022 frame 0: Blend With Original keyed from 40 at frame 0 to 100 at frame 4 by an ease that passes its end: held at 100 at frame 2, where it would pass it. | largest difference 9.7e-8 | yes |
| FX-IOK-022 frame 2: Blend With Original keyed from 40 at frame 0 to 100 at frame 4 by an ease that passes its end: held at 100 at frame 2, where it would pass it. | largest difference 1.3e-7 | yes |
| FX-IOK-022 frame 4: Blend With Original keyed from 40 at frame 0 to 100 at frame 4 by an ease that passes its end: held at 100 at frame 2, where it would pass it. | largest difference 1.3e-7 | yes |
| FX-IOK-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-023 frame 0: Foreground 0 with a background named: still none, the picture as it was. | largest difference 1.3e-7 | yes |
| FX-IOK-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-024 frame 0: Edge Threshold 100: only the wholly kept pixels stay, whole. | largest difference 9.7e-8 | yes |
| FX-IOK-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IOK-025 frame 0: Foreground 1 with no masks at all. Nothing to key by: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.3e-7 | yes |
| FX-IOK-025 frame 4: Foreground 1 with no masks at all. Nothing to key by: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.3e-7 | yes |
| FX-IOK-025: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-IOK-026 frame 0: Foreground 3, of two. Nothing to key by: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.3e-7 | yes |
| FX-IOK-026 frame 4: Foreground 3, of two. Nothing to key by: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.3e-7 | yes |
| FX-IOK-026: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-IOK-027 frame 0: Background 3, of two. Nothing to key by: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.3e-7 | yes |
| FX-IOK-027 frame 4: Background 3, of two. Nothing to key by: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.3e-7 | yes |
| FX-IOK-027: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-IOK-028 frame 0: Foreground 1, switched off. Nothing to key by: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.3e-7 | yes |
| FX-IOK-028 frame 4: Foreground 1, switched off. Nothing to key by: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.3e-7 | yes |
| FX-IOK-028: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-IOK-029 frame 0: Background 2, switched off. Nothing to key by: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.3e-7 | yes |
| FX-IOK-029 frame 4: Background 2, switched off. Nothing to key by: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 1.3e-7 | yes |
| FX-IOK-029: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-IOK-030 frame 0: Foreground -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-030 frame 4: Foreground -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IOK-031 frame 0: Background 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-031 frame 4: Background 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IOK-032 frame 0: Single Mask Highlight Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-032 frame 4: Single Mask Highlight Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IOK-033 frame 0: Edge Thin -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-033 frame 4: Edge Thin -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IOK-034 frame 0: Edge Thin 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-034 frame 4: Edge Thin 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IOK-035 frame 0: Edge Feather 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-035 frame 4: Edge Feather 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IOK-036 frame 0: Edge Threshold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-036 frame 4: Edge Threshold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IOK-037 frame 0: Blend With Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-037 frame 4: Blend With Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IOK-038 frame 0: Invert Extraction "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-038 frame 4: Invert Extraction "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IOK-039 frame 0: Blend With Original keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-039 frame 4: Blend With Original keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.3e-7 | yes |
| FX-IOK-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_iok_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iok_017.json is saved with its numbers and word as written, no paths | {"background":2.7,"blend_with_original":0,"edge_feather":0,"edge_thin":0,"edge_threshold":0,"foreground":1.5,"invert_extraction":"off","single_mask_highlight_radius":10} | yes |
| fx_iok_030.json is refused in a sentence | Inner/Outer Key's foreground runs from 0 to 1000, and this is -1. | yes |
| fx_iok_031.json is refused in a sentence | Inner/Outer Key's background runs from 0 to 1000, and this is 1001. | yes |
| fx_iok_032.json is refused in a sentence | Inner/Outer Key's single mask highlight radius runs from 0 to 1000, and this is -1. | yes |
| fx_iok_033.json is refused in a sentence | Inner/Outer Key's edge thin runs from -100 to 100, and this is -101. | yes |
| fx_iok_034.json is refused in a sentence | Inner/Outer Key's edge thin runs from -100 to 100, and this is 101. | yes |
| fx_iok_035.json is refused in a sentence | Inner/Outer Key's edge feather runs from 0 to 1000, and this is 1001. | yes |
| fx_iok_036.json is refused in a sentence | Inner/Outer Key's edge threshold runs from 0 to 100, and this is 101. | yes |
| fx_iok_037.json is refused in a sentence | Inner/Outer Key's blend with original runs from 0 to 100, and this is 101. | yes |
| fx_iok_038.json is refused in a sentence | Inner/Outer Key's invert extraction is "off" or "on", and this is "yes". | yes |
| a file with an Inner/Outer Key with no `blend_with_original` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Inner/Outer Key with a foreground in words is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Inner/Outer Key whose invert extraction is true is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| foreground -1 is refused with a sentence, and nothing changes | Inner/Outer Key's foreground runs from 0 to 1000, and this is -1. | yes |
| edge thin 101 is refused with a sentence, and nothing changes | Inner/Outer Key's edge thin runs from -100 to 100, and this is 101. | yes |
| edge feather 1001 is refused with a sentence, and nothing changes | Inner/Outer Key's edge feather runs from 0 to 1000, and this is 1001. | yes |
| invert extraction "yes" is refused with a sentence, and nothing changes | Inner/Outer Key's invert extraction is "off" or "on", and this is "yes". | yes |
| blend with original keyed to 101 is refused with a sentence, and nothing changes | Inner/Outer Key's blend with original runs from 0 to 100, and this is 101. | yes |
| foreground 2, background 1, radius 3, thin -2, feather 4, threshold 30, inverted, blend 20 is taken | taken | yes |
| edge feather keyed from 0 to 6 is taken | taken | yes |
| single mask highlight radius keyed from 0 to 5 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_iok_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_iok_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_iok_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_iok_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_iok_014.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_iok_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_iok_021.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255 (the key is drawn on the processor; the card draws what follows it)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_iok_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_022.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_035.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_036.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_037.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_038.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_iok_039.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Inner/Outer Key foreground 1 (a circle of radius 250), background 2 (radius 400 about it): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1806454 pixels changed | yes |
| the reference shot, Inner/Outer Key foreground 1 (a circle of radius 250), background 2 (radius 400 about it) on three layers, frame 0, Full | largest difference 1 of 255, 10441 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 1 (a circle of radius 250), background 2 (radius 400 about it) on three layers, frame 100, Full | largest difference 1 of 255, 9517 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 1 (a circle of radius 250), background 2 (radius 400 about it) on three layers, frame 239, Full | largest difference 1 of 255, 9876 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 1 (a circle of radius 250), background 2 (radius 400 about it) on three layers, frame 0, Draft | largest difference 1 of 255, 1406 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 1 (a circle of radius 250), background 2 (radius 400 about it) on three layers, frame 100, Draft | largest difference 1 of 255, 1608 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 1 (a circle of radius 250), background 2 (radius 400 about it) on three layers, frame 239, Draft | largest difference 1 of 255, 1514 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 1 alone, radius 10: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1883202 pixels changed | yes |
| the reference shot, Inner/Outer Key foreground 1 alone, radius 10 on three layers, frame 0, Full | largest difference 1 of 255, 3566 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 1 alone, radius 10 on three layers, frame 100, Full | largest difference 1 of 255, 2616 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 1 alone, radius 10 on three layers, frame 239, Full | largest difference 1 of 255, 3173 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 1 alone, radius 10 on three layers, frame 0, Draft | largest difference 1 of 255, 648 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 1 alone, radius 10 on three layers, frame 100, Draft | largest difference 1 of 255, 873 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 1 alone, radius 10 on three layers, frame 239, Draft | largest difference 1 of 255, 737 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 3 alone, radius 40, inverted: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 172592 pixels changed | yes |
| the reference shot, Inner/Outer Key foreground 3 alone, radius 40, inverted on three layers, frame 0, Full | largest difference 1 of 255, 8677 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 3 alone, radius 40, inverted on three layers, frame 100, Full | largest difference 1 of 255, 7366 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 3 alone, radius 40, inverted on three layers, frame 239, Full | largest difference 1 of 255, 7857 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 3 alone, radius 40, inverted on three layers, frame 0, Draft | largest difference 1 of 255, 15529 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 3 alone, radius 40, inverted on three layers, frame 100, Draft | largest difference 1 of 255, 15986 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 3 alone, radius 40, inverted on three layers, frame 239, Draft | largest difference 1 of 255, 15954 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 1, background 2, thin 3, feather 12, threshold 20: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1893758 pixels changed | yes |
| the reference shot, Inner/Outer Key foreground 1, background 2, thin 3, feather 12, threshold 20 on three layers, frame 0, Full | largest difference 1 of 255, 8561 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 1, background 2, thin 3, feather 12, threshold 20 on three layers, frame 100, Full | largest difference 1 of 255, 7818 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 1, background 2, thin 3, feather 12, threshold 20 on three layers, frame 239, Full | largest difference 1 of 255, 8019 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 1, background 2, thin 3, feather 12, threshold 20 on three layers, frame 0, Draft | largest difference 1 of 255, 1597 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 1, background 2, thin 3, feather 12, threshold 20 on three layers, frame 100, Draft | largest difference 1 of 255, 1823 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 1, background 2, thin 3, feather 12, threshold 20 on three layers, frame 239, Draft | largest difference 1 of 255, 1704 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 4 alone, radius 25, thin -4, blend 40: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1812947 pixels changed | yes |
| the reference shot, Inner/Outer Key foreground 4 alone, radius 25, thin -4, blend 40 on three layers, frame 0, Full | largest difference 1 of 255, 12126 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 4 alone, radius 25, thin -4, blend 40 on three layers, frame 100, Full | largest difference 1 of 255, 10446 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 4 alone, radius 25, thin -4, blend 40 on three layers, frame 239, Full | largest difference 1 of 255, 16704 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 4 alone, radius 25, thin -4, blend 40 on three layers, frame 0, Draft | largest difference 1 of 255, 6058 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 4 alone, radius 25, thin -4, blend 40 on three layers, frame 100, Draft | largest difference 1 of 255, 6125 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Inner/Outer Key foreground 4 alone, radius 25, thin -4, blend 40 on three layers, frame 239, Draft | largest difference 1 of 255, 6308 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-502 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect (its three masks cut nothing); draws cleanly | [] | yes |
| 2_two_masks.png, foreground 1 (a circle of radius 70), background 2 (110 about it): the inner disc kept, outside the outer one gone, the ring between see-through as much as it is the colour outside; draws cleanly | [], 110134 pixels changed of 129600 | yes |
| 3_one_mask.png, foreground 1 alone, radius as added (10): the disc kept with a soft rim, the rest gone; draws cleanly | [], 116795 pixels changed of 129600 | yes |
| 4_inverted.png, the two masks with Invert Extraction: the inner disc gone, the street outside the outer one kept; draws cleanly | [], 32492 pixels changed of 129600 | yes |
| 5_feathered.png, the two masks, thin 4, feather 16: the kept disc a little smaller, its edge soft; draws cleanly | [], 122272 pixels changed of 129600 | yes |
| 6_blended.png, foreground 3 (the upper left circle) alone, radius 6, blend 50: the street half see-through outside it; draws cleanly | [], 123218 pixels changed of 129600 | yes |

## Result

249 of 249 checks pass.
