# B-148: Bevel Alpha and Bevel Edges

D-213, accepted on 2026-09-28 with the After Effects picks (B11): After Effects' own Bevel Alpha and Bevel Edges, as two effects; CC Glass is left out. Every expected pixel is `Fixtures/bevel/expected_bevel.json`, written by `tools/bevel_reference.py` before this code existed and printed in document 25 as FX-BEVEL-001 to 026. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-BEVEL-001 to 026 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BEVEL-001 frame 0: Bevel Alpha as it starts: edge thickness 2, the light from the upper left at -60, white, intensity 0.4: the card's top and left edges lighter, its bottom and right edges darker, its covering the same. | largest difference 2.3e-7 | yes |
| FX-BEVEL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEVEL-002 frame 0: Bevel Alpha, edge thickness 0: the frame is the drawing. | largest difference 1.9e-7 | yes |
| FX-BEVEL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEVEL-003 frame 0: Bevel Alpha, intensity 0: the frame is the drawing. | largest difference 1.9e-7 | yes |
| FX-BEVEL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEVEL-004 frame 0: Bevel Alpha, the light straight above at 0: the top edge lighter, the bottom darker. | largest difference 2.5e-7 | yes |
| FX-BEVEL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEVEL-005 frame 0: Bevel Alpha, the light from the right at 90: the right edge lighter, the left darker. | largest difference 1.8e-7 | yes |
| FX-BEVEL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEVEL-006 frame 0: Bevel Alpha, edge thickness 6: a wider, gentler bevel reaching every pixel of the card. | largest difference 2.3e-7 | yes |
| FX-BEVEL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEVEL-007 frame 0: Bevel Alpha, a blue light #2040a0 at intensity 1: the lit edges go toward blue, the dark edges toward black. | largest difference 3.1e-7 | yes |
| FX-BEVEL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEVEL-008 frame 0: Bevel Alpha, edge thickness keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is the drawing, frames 2 and 4 bevel at 4 and 8. | largest difference 1.9e-7 | yes |
| FX-BEVEL-008 frame 2: Bevel Alpha, edge thickness keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is the drawing, frames 2 and 4 bevel at 4 and 8. | largest difference 2.1e-7 | yes |
| FX-BEVEL-008 frame 4: Bevel Alpha, edge thickness keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is the drawing, frames 2 and 4 bevel at 4 and 8. | largest difference 2.4e-7 | yes |
| FX-BEVEL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEVEL-009 frame 0: Bevel Alpha, intensity eased from 0 at frame 0 to 1 at frame 4 on a curve that overshoots: at frame 2 it has gone above 1 and is held there, so frames 2 and 4 are the same. | largest difference 1.9e-7 | yes |
| FX-BEVEL-009 frame 2: Bevel Alpha, intensity eased from 0 at frame 0 to 1 at frame 4 on a curve that overshoots: at frame 2 it has gone above 1 and is held there, so frames 2 and 4 are the same. | largest difference 3.1e-7 | yes |
| FX-BEVEL-009 frame 4: Bevel Alpha, intensity eased from 0 at frame 0 to 1 at frame 4 on a curve that overshoots: at frame 2 it has gone above 1 and is held there, so frames 2 and 4 are the same. | largest difference 3.1e-7 | yes |
| FX-BEVEL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEVEL-010 frame 0: FX-BEVEL-001 moved three pixels right: the same, moved. | largest difference 2.3e-7 | yes |
| FX-BEVEL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEVEL-011 frame 0: Bevel Edges as it starts: edge thickness 0.1 of the slab's 10 rows, one pixel: the left column lit by 0.866 of 0.4 and the top row by 0.5 of it, the right column darkened by 0.866 of it and the bottom row by 0.5; the top-left corner pixel goes with the left side; the inside unchanged. | largest difference 1.9e-7 | yes |
| FX-BEVEL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEVEL-012 frame 0: Bevel Edges, edge thickness 0.3, three pixels: a frame three pixels wide, its corners mitred. | largest difference 1.9e-7 | yes |
| FX-BEVEL-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEVEL-013 frame 0: Bevel Edges, edge thickness 0: the frame is the drawing. | largest difference 1.9e-7 | yes |
| FX-BEVEL-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEVEL-014 frame 0: Bevel Edges, the light from below at 180: the bottom row lit and the top row darkened by the whole 0.4, the left and right columns unchanged. | largest difference 1.9e-7 | yes |
| FX-BEVEL-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEVEL-015 frame 0: Bevel Edges, edge thickness 0.5, five pixels: every pixel is on a face, a four-sided pyramid. | largest difference 1.6e-7 | yes |
| FX-BEVEL-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEVEL-016 frame 0: Bevel Edges, a red light #ff0000 at intensity 1: the lit sides go toward red. | largest difference 1.9e-7 | yes |
| FX-BEVEL-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEVEL-017 frame 0: Bevel Edges, the light angle keyed from 0 at frame 0 to 360 at frame 4, linear: at frame 2 it is 180, FX-BEVEL-014, and frame 4 is frame 0 again. | largest difference 1.9e-7 | yes |
| FX-BEVEL-017 frame 2: Bevel Edges, the light angle keyed from 0 at frame 0 to 360 at frame 4, linear: at frame 2 it is 180, FX-BEVEL-014, and frame 4 is frame 0 again. | largest difference 1.9e-7 | yes |
| FX-BEVEL-017 frame 4: Bevel Edges, the light angle keyed from 0 at frame 0 to 360 at frame 4, linear: at frame 2 it is 180, FX-BEVEL-014, and frame 4 is frame 0 again. | largest difference 1.9e-7 | yes |
| FX-BEVEL-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEVEL-018 frame 0: FX-BEVEL-012 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-BEVEL-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEVEL-019 frame 0: Bevel Alpha, edge thickness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEVEL-019 frame 4: Bevel Alpha, edge thickness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEVEL-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEVEL-020 frame 0: Bevel Alpha, edge thickness 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEVEL-020 frame 4: Bevel Alpha, edge thickness 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEVEL-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEVEL-021 frame 0: Bevel Alpha, intensity 1.1, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEVEL-021 frame 4: Bevel Alpha, intensity 1.1, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEVEL-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEVEL-022 frame 0: Bevel Alpha, light angle 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEVEL-022 frame 4: Bevel Alpha, light angle 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEVEL-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEVEL-023 frame 0: Bevel Alpha, light colour "white", not a colour written #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEVEL-023 frame 4: Bevel Alpha, light colour "white", not a colour written #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEVEL-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEVEL-024 frame 0: Bevel Edges, edge thickness 0.6, above 0.5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEVEL-024 frame 4: Bevel Edges, edge thickness 0.6, above 0.5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEVEL-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEVEL-025 frame 0: Bevel Edges, intensity -0.1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEVEL-025 frame 4: Bevel Edges, intensity -0.1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEVEL-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEVEL-026 frame 0: Bevel Edges, edge thickness keyed to 0.8 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEVEL-026 frame 4: Bevel Edges, edge thickness keyed to 0.8 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BEVEL-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far they reach

| Check | The build's answer | Matches |
| --- | --- | --- |
| Bevel Alpha grows the drawing's bounds by nothing | 0 | yes |
| Bevel Edges grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview halves Bevel Alpha's edge thickness, in pixels, and nothing else | BevelAlpha { edge_thickness: 3.0, light_angle: 45.0, light_color: "#2040a0", light_intensity: 0.7 } | yes |
| a half-size draft preview leaves Bevel Edges alone: its thickness is a share of the layer | BevelEdges { edge_thickness: 0.25, light_angle: 45.0, light_color: "#2040a0", light_intensity: 0.7 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bevel_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bevel_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `light_color` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a light intensity that is a list is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands, Bevel Alpha

| Check | The build's answer | Matches |
| --- | --- | --- |
| edge thickness -1 is refused with a sentence, and nothing changes | Bevel Alpha's edge thickness runs from 0 to 200, and this is -1. | yes |
| edge thickness 201 is refused with a sentence, and nothing changes | Bevel Alpha's edge thickness runs from 0 to 200, and this is 201. | yes |
| light angle 3601 is refused with a sentence, and nothing changes | Bevel Alpha's light angle runs from -3600 to 3600, and this is 3601. | yes |
| light angle -3601 is refused with a sentence, and nothing changes | Bevel Alpha's light angle runs from -3600 to 3600, and this is -3601. | yes |
| light intensity 1.1 is refused with a sentence, and nothing changes | Bevel Alpha's light intensity runs from 0 to 1, and this is 1.1. | yes |
| light colour "white" is refused with a sentence, and nothing changes | Bevel Alpha's light colour is written #rrggbb, and this is "white". | yes |
| edge thickness keyed to 300 is refused with a sentence, and nothing changes | Bevel Alpha's edge thickness runs from 0 to 200, and this is 300. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| light angle keyed from 0 to 360 is taken | taken | yes |
| light intensity keyed from 0 to 1 is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## Commands, Bevel Edges

| Check | The build's answer | Matches |
| --- | --- | --- |
| edge thickness 0.6 is refused with a sentence, and nothing changes | Bevel Edges's edge thickness runs from 0 to 0.5, and this is 0.6. | yes |
| edge thickness -0.01 is refused with a sentence, and nothing changes | Bevel Edges's edge thickness runs from 0 to 0.5, and this is -0.01. | yes |
| light intensity -0.1 is refused with a sentence, and nothing changes | Bevel Edges's light intensity runs from 0 to 1, and this is -0.1. | yes |
| light colour "#fff" is refused with a sentence, and nothing changes | Bevel Edges's light colour is written #rrggbb, and this is "#fff". | yes |
| edge thickness keyed to 0.8 is refused with a sentence, and nothing changes | Bevel Edges's edge thickness runs from 0 to 0.5, and this is 0.8. | yes |
| edge thickness 0.5, its top, is taken | taken | yes |
| edge thickness keyed from 0 to 0.5 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bevel_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bevel_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bevel_008.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bevel_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bevel_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bevel_017.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a made-up badge and panel, in `verification/B-148 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| badge_start.png, Bevel Alpha as it starts: the rim lighter at the upper left, darker at the lower right, the middle as it was; the covering unchanged everywhere; draws cleanly | [], upper-left rim +142, lower-right rim -20, middle +0 | yes |
| badge_off.png, light intensity 0: the badge as it was; the covering unchanged everywhere; draws cleanly | [], upper-left rim +0, lower-right rim +0, middle +0 | yes |
| badge_wide.png, edge thickness 8 at intensity 0.8: a wider, stronger bevel, the middle still as it was; the covering unchanged everywhere; draws cleanly | [], upper-left rim +405, lower-right rim -146, middle +0 | yes |
| badge_low_light.png, the light from the lower right at 120: the lower-right rim lighter and the upper-left darker; the covering unchanged everywhere; draws cleanly | [], upper-left rim -20, lower-right rim +142, middle +0 | yes |
| badge_blue.png, a blue light #2040a0 at intensity 1: the lit rim bluer, the far rim darker; the covering unchanged everywhere; draws cleanly | [], upper-left rim +31, lower-right rim -54, middle +0 | yes |
| in badge_blue.png the lit rim has less red and more blue than the drawing there | drawing [200, 40, 50, 255], built [166, 50, 105, 255] | yes |
| panel_start.png, Bevel Edges as it starts, a tenth of the panel's height, 10 pixels: the left and top sides lighter, the right and bottom darker, the left more than the top, the middle as it was; the covering unchanged everywhere; draws cleanly | [], left +195, top +130, right -67, bottom -37, middle +0 | yes |
| panel_below.png, the light from below at 180: the bottom lighter, the top darker, the left and right as they were; the covering unchanged everywhere; draws cleanly | [], left +0, top -79, right +0, bottom +215, middle +0 | yes |
| panel_gold.png, a quarter of the panel's height, 25 pixels, with a gold light #ffd070 at 0.9: a deep frame, the middle as it was; the covering unchanged everywhere; draws cleanly | [], left +190, top +145, right -193, bottom -90, middle +0 | yes |
| panel_pyramid.png, half the panel's height: every pixel on a side, so even the middle changes; the covering unchanged everywhere; draws cleanly | [], left +195, top +130, right -67, bottom -37, middle +130 | yes |

## Result

134 of 134 checks pass.
