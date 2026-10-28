# B-801: Layer Styles, Inner Shadow and Drop Shadow

D-922, accepted 2026-10-27 with wave 12's tutorial follow-alongs ("go through this playlist to finely tune our effects ... to fully replicate all these tutorials"). Every expected pixel is `Fixtures/layer_styles/expected_layer_styles.json`, written by `tools/layer_styles_reference.py` before this code existed and printed in document 25 as FX-LSTYLE-001 to 023. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-LSTYLE-001 to 023 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LSTYLE-001 frame 0: Inner Shadow at After Effects' defaults: Multiply, black, 75 per cent, Global Light on at the composition's 120 (the file has no angle of its own), distance 5, choke 0, size 5. The light comes from the upper left, so the plate darkens inside its upper and left edges, softly; its lower right corner stays near its own skin; its alpha never changes. | largest difference 1.4e-7 | yes |
| FX-LSTYLE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LSTYLE-002 frame 0: Normal, #ff8000 at 100, Global Light off, angle 0 (light from the right), distance 2, size 0: the plate's two right-hand columns, 10 and 11, are exactly orange; every other skin pixel is unchanged; the half-covered pixel at (12, 4) is orange at its own covering. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LSTYLE-003 frame 0: The tutorial's rim light (w12a 28:54): Linear Dodge (Add), #ff8000 at 50, Global Light off, angle 361 (1x+1.0), distance 2, size 2: an orange glow added inside the right-hand edge, brighter than the skin, held to 1. | largest difference 2.3e-7 | yes |
| FX-LSTYLE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LSTYLE-004 frame 0: w12c's (video 2, 5:23): Color Dodge, #ffff00 at 75, Global Light off, angle 178, distance 2, size 1: the skin brightened toward yellow inside its left edge, on the encoded colours. | largest difference 2.0e-7 | yes |
| FX-LSTYLE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LSTYLE-005 frame 0: Choke 50, size 4, distance 0, Normal black at 100: the covering is eroded by the disc of radius 2 and blurred at sigma 1, so a dark band runs inside every edge, darkest at the edge. | largest difference 9.7e-8 | yes |
| FX-LSTYLE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LSTYLE-006 frame 0: Noise 50, Normal black at 100, Global Light off, angle 90 (light from above), distance 1, size 0: the plate's top row is shadowed, each pixel by between half and all of it, the same grain every time. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LSTYLE-007 frame 0: Global Light on with the composition's angle 90 in the file, the style's own angle 0 left unused: FX-LSTYLE-008 exactly. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LSTYLE-008 frame 0: Global Light off, angle 90, distance 2, size 0, Normal black at 100: the plate's top two rows are black. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LSTYLE-009 frame 0: Distance keyed from 0 at frame 0 to 4 at frame 4, linear, angle 0, Normal black at 100, size 0: frame 0 is the plate unchanged but for its half-covered pixel's outside, frame 2 has two black right-hand columns, frame 4 four. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-009 frame 2: Distance keyed from 0 at frame 0 to 4 at frame 4, linear, angle 0, Normal black at 100, size 0: frame 0 is the plate unchanged but for its half-covered pixel's outside, frame 2 has two black right-hand columns, frame 4 four. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-009 frame 4: Distance keyed from 0 at frame 0 to 4 at frame 4, linear, angle 0, Normal black at 100, size 0: frame 0 is the plate unchanged but for its half-covered pixel's outside, frame 2 has two black right-hand columns, frame 4 four. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LSTYLE-010 frame 0: Opacity 0: the plate untouched. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LSTYLE-011 frame 0: Drop Shadow at After Effects' defaults: Multiply, black, 75, Global Light on at 120, distance 5, spread 0, size 5: a soft shadow down and to the right of the plate, behind it. Multiply with black is Normal's pixels exactly, so nothing is noted. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LSTYLE-012 frame 0: Drop Shadow, Normal, #2040a0 at 100, Global Light off, angle 90, distance 2, size 0: the plate's shape moved two pixels down, blue, in rows 8 and 9 below it; the plate itself unchanged. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LSTYLE-013 frame 0: Drop Shadow, spread 50, size 4, distance 0, Normal black at 100: the covering dilated by the disc of radius 2 and blurred at sigma 1, a dark halo round the plate. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LSTYLE-014 frame 0: Both, listed Drop Shadow first: the Inner Shadow is drawn first, from the plate's own covering, then the Drop Shadow behind the result. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LSTYLE-015 frame 0: Drop Shadow in Screen, white at 100: drawn as Normal's would be, behind the plate, with a note on each frame that its mode is not mixed with what is beneath. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-015: what opening it warns of, and what frame 4 warns of | [] and ["PROJECT_FEATURE_UNSUPPORTED"] | yes |
| FX-LSTYLE-016 frame 0: Opacity 101, above 100. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-016 frame 4: Opacity 101, above 100. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LSTYLE-017 frame 0: Size 251, above 250. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-017 frame 4: Size 251, above 250. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LSTYLE-018 frame 0: Choke 101, above 100. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-018 frame 4: Choke 101, above 100. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LSTYLE-019 frame 0: Noise -1, below 0. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-019 frame 4: Noise -1, below 0. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LSTYLE-020 frame 0: A colour written "black". The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-020 frame 4: A colour written "black". The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LSTYLE-021 frame 0: Use Global Light written "yes". The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-021 frame 4: Use Global Light written "yes". The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LSTYLE-022 frame 0: Blend mode "dissolve", which this program does not draw. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-022 frame 4: Blend mode "dissolve", which this program does not draw. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LSTYLE-023 frame 0: Blend mode "stencil_alpha", a stencil, not a style's mode. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-023 frame 4: Blend mode "stencil_alpha", a stencil, not a style's mode. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LSTYLE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The composition's Global Light

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lstyle_001.json, which has no `global_light_angle`, opens at 120 | 120 | yes |
| saved again, it still has no `global_light_angle` | absent | yes |
| fx_lstyle_007.json is saved with its angle 90 | 90 | yes |
| fx_lstyle_008.json with Use Global Light turned on, its angle 0 and the Global Light set to 90 draws its own frame (light from above) | byte-identical | yes |

## Saving and opening

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lstyle_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lstyle_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| an Inner Shadow is saved with `choke`, not `spread` | {"angle":120,"blend_mode":"normal","choke":50,"color":"#000000","distance":0,"noise":0,"opacity":100,"size":4,"use_global_light":"off"} | yes |
| a Drop Shadow is saved with `spread`, not `choke` | {"angle":120,"blend_mode":"normal","color":"#000000","distance":0,"noise":0,"opacity":100,"size":4,"spread":50,"use_global_light":"off"} | yes |
| fx_lstyle_001.json opens as an Inner Shadow at After Effects' defaults | LayerStyle { inner: true, blend_mode: "multiply", color: "#000000", opacity: 75.0, use_global_light: "on", angle: 120.0, distance: 5.0, spread: 0.0, size: 5.0, noise: 0.0 } | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| a blend mode "dissolve" is refused with a sentence, and nothing changes | Inner Shadow's blend mode is a layer mode but the stencils, and this is "dissolve". | yes |
| a blend mode "stencil_luma" is refused with a sentence, and nothing changes | Inner Shadow's blend mode is a layer mode but the stencils, and this is "stencil_luma". | yes |
| a colour "black" is refused with a sentence, and nothing changes | Inner Shadow's colour is written #rrggbb, and this is "black". | yes |
| Use Global Light "yes" is refused with a sentence, and nothing changes | Inner Shadow's use global light is "on" or "off", and this is "yes". | yes |
| a Global Light angle of 3601 is refused with a sentence, and nothing changes | The Global Light angle is a number from -3600 to 3600 degrees. | yes |
| a Global Light angle that is not a number is refused with a sentence, and nothing changes | The Global Light angle is a number from -3600 to 3600 degrees. | yes |
| an Inner Shadow in Color Dodge, #ffff00 is taken | taken | yes |
| a Global Light angle of 45 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lstyle_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lstyle_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lstyle_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lstyle_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lstyle_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lstyle_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

101 of 101 checks pass.
