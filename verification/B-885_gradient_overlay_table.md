# B-885: the Layer Style Gradient Overlay

D-1005, accepted 2026-10-27 with wave 12's tutorial follow-alongs ("go through this playlist to finely tune our effects ... to fully replicate all these tutorials"). Every expected pixel is `Fixtures/layer_styles/expected_gradient_overlay.json`, written by `tools/gradient_overlay_reference.py` before this code existed and printed in document 25 as FX-GOVER-001 to 025. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-GOVER-001 to 025 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GOVER-001 frame 0: Gradient Overlay as added: Normal, 100, white at 0 to black at 100, angle 90, linear, scale 100. The plate turns from black along its top row to white along its bottom row (location 0 is where the angle points away from); its alpha never changes, so the empty corners stay empty. | largest difference 3.0e-8 | yes |
| FX-GOVER-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GOVER-002 frame 0: Angle 0, red at 0 to blue at 100: red down the plate's left edge, blue down its right, the same in every row. | largest difference 3.6e-8 | yes |
| FX-GOVER-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GOVER-003 frame 0: The tutorial's (PVUPTcfJ5HY 4:00-6:40, pv_400): Overlay, 100, #232444 at both colour stops, opacity 0 at 65.6 and 100 at 100, angle 90, Radial, scale 150. Only the plate's two left-hand corners, the pixels farthest from the box's centre, darken, like a vignette's edge; the rest is untouched. | largest difference 2.0e-7 | yes |
| FX-GOVER-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GOVER-004 frame 0: Reflected, angle 0, red at 0 to blue at 100: red down the middle, blue at both side edges, left and right the same. | largest difference 3.0e-8 | yes |
| FX-GOVER-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GOVER-005 frame 0: Angle style at 0, red to blue: the colour goes round the box's centre counter-clockwise from the right, red just above the right-hand middle, blue just below it. | largest difference 3.2e-8 | yes |
| FX-GOVER-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GOVER-006 frame 0: Diamond, angle 0, red to blue: red in the middle, a diamond of blue growing out to the plate's corners. | largest difference 3.0e-8 | yes |
| FX-GOVER-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GOVER-007 frame 0: FX-GOVER-001 with Reverse on: white along the top, black along the bottom. | largest difference 3.1e-8 | yes |
| FX-GOVER-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GOVER-008 frame 0: Radial, red to blue, offset 3 pixels right: the red middle moves three columns right of the box's centre. | largest difference 3.0e-8 | yes |
| FX-GOVER-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GOVER-009 frame 0: Angle keyed from 0 at frame 0 to 180 at frame 4, linear, red to blue: at frame 0 red on the left, at frame 2 (90) red on the bottom, at frame 4 red on the right. | largest difference 3.6e-8 | yes |
| FX-GOVER-009 frame 2: Angle keyed from 0 at frame 0 to 180 at frame 4, linear, red to blue: at frame 0 red on the left, at frame 2 (90) red on the bottom, at frame 4 red on the right. | largest difference 3.1e-8 | yes |
| FX-GOVER-009 frame 4: Angle keyed from 0 at frame 0 to 180 at frame 4, linear, red to blue: at frame 0 red on the left, at frame 2 (90) red on the bottom, at frame 4 red on the right. | largest difference 3.6e-8 | yes |
| FX-GOVER-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GOVER-010 frame 0: Three colour stops, red at 0, #ff8000 at 50 and blue at 100, angle 0, scale 50: red over the left quarter, through orange at the middle, blue past the right quarter. | largest difference 3.0e-8 | yes |
| FX-GOVER-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GOVER-011 frame 0: Opacity 0: the plate untouched. | largest difference 1.9e-7 | yes |
| FX-GOVER-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GOVER-012 frame 0: Multiply, #ff8000 to blue, angle 0: the skin multiplied by orange on the left, by blue on the right. | largest difference 1.5e-7 | yes |
| FX-GOVER-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GOVER-013 frame 0: An Inner Shadow listed first (Normal, #ff8000, 100, Global Light off, angle 0, distance 2, size 0), then the Gradient Overlay as added: the overlay is drawn first, so the plate's two right-hand columns are orange over the black-to-white ramp. | largest difference 3.1e-8 | yes |
| FX-GOVER-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GOVER-014 frame 0: Opacity stops 100 at 0 and 0 at 100, angle 0, red to red: solid red on the left fading to the plate's own skin on the right. | largest difference 1.5e-7 | yes |
| FX-GOVER-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GOVER-015 frame 0: Two stops at the same place, red at 0 and 50, blue at 50 and 100, angle 0: a hard step from red to blue down the middle, no blend between. | largest difference 3.0e-8 | yes |
| FX-GOVER-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GOVER-016 frame 0: Colour stops at 60 then 40, falling. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-016 frame 4: Colour stops at 60 then 40, falling. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GOVER-017 frame 0: A colour stop at 101, past 100. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-017 frame 4: A colour stop at 101, past 100. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GOVER-018 frame 0: An opacity stop of 101 per cent. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-018 frame 4: An opacity stop of 101 per cent. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GOVER-019 frame 0: A colour written "black". The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-019 frame 4: A colour written "black". The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GOVER-020 frame 0: One colour stop only. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-020 frame 4: One colour stop only. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GOVER-021 frame 0: Style "spiral". The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-021 frame 4: Style "spiral". The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GOVER-022 frame 0: Scale 151, above 150. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-022 frame 4: Scale 151, above 150. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GOVER-023 frame 0: Blend mode "stencil_luma", a stencil, not a style's mode. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-023 frame 4: Blend mode "stencil_luma", a stencil, not a style's mode. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GOVER-024 frame 0: Reverse written "yes". The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-024 frame 4: Reverse written "yes". The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GOVER-025 frame 0: An opacity stop of three numbers. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-025 frame 4: An opacity stop of three numbers. The file is read, the style is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GOVER-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Saving and opening

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gover_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gover_001.json opens as a Gradient Overlay at After Effects' defaults | GradientOverlay { blend_mode: "normal", opacity: 100.0, color_stops: [(0.0, "#ffffff"), (100.0, "#000000")], opacity_stops: [[0.0, 100.0], [100.0, 100.0]], angle: 90.0, style: "linear", reverse: "off", scale: 100.0, offset: [0.0, 0.0] } | yes |
| the tutorial's file is saved with its stops as written | [[0,"#232444"],[100,"#232444"]] [[65.6,0],[100,100]] | yes |
| a file with a colour stop that is a word, not a location and a colour is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an opacity stop that is not numbers is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| a blend mode "stencil_alpha" is refused with a sentence, and nothing changes | Gradient Overlay's blend mode is a layer mode but the stencils, and this is "stencil_alpha". | yes |
| a style "spiral" is refused with a sentence, and nothing changes | Gradient Overlay's style is linear, radial, angle, reflected or diamond, and this is "spiral". | yes |
| a stop colour "white" is refused with a sentence, and nothing changes | Gradient Overlay's stop colour is written #rrggbb, and this is "white". | yes |
| colour stops at 80 then 20 is refused with a sentence, and nothing changes | Gradient Overlay's colour stops go along from 0 to 100, and 20 comes after 80. | yes |
| seventeen colour stops is refused with a sentence, and nothing changes | Gradient Overlay takes 2 to 16 colour stops, and this has 17. | yes |
| an opacity stop of -1 is refused with a sentence, and nothing changes | Gradient Overlay's stop opacity runs from 0 to 100, and this is -1. | yes |
| the tutorial's radial overlay is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gover_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gover_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gover_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gover_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

103 of 103 checks pass.
