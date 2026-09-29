# B-146: Radial Shadow

D-211, accepted on 2026-09-28 with the After Effects picks (B9). Every expected pixel is `Fixtures/radial_shadow/expected_radial_shadow.json`, written by `tools/radial_shadow_reference.py` before this code existed and printed in document 25 as FX-RSHADOW-001 to 025. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-RSHADOW-001 to 025 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RSHADOW-001 frame 0: As it starts: black at 50 per cent, the light at the top of the middle (8, 0), distance 10, so the shadow is the card made a tenth bigger about the light, peeping out below the card and a little to each side; the card itself, fully covered, is unchanged. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSHADOW-002 frame 0: Distance 0: the shadow lies exactly under the card, so it shows only through the half-covered pixel (10, 4), and the light's place does not matter. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSHADOW-003 frame 0: The light in the middle (8, 5), distance 50: the card half as big again about the middle, a dark ring round it on every side. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSHADOW-004 frame 0: The light at the top-left corner, distance 40: the shadow thrown down and to the right, away from the light, and grown by 1.4. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSHADOW-005 frame 0: FX-RSHADOW-004 in blue #2040a0 at opacity 100: where the shadow lies wholly within the card's scaled shape it is exactly that blue, fully covered. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSHADOW-006 frame 0: FX-RSHADOW-004 with softness 3: the shadow blurred at sigma 1, fading out further, its darkest lighter. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSHADOW-007 frame 0: Render Glass Edge, colour influence 100, opacity 100: the shadow takes the card's own colours, skin inside and line round it, as light through stained glass, instead of black. | largest difference 2.5e-7 | yes |
| FX-RSHADOW-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSHADOW-008 frame 0: Glass Edge at colour influence 50: halfway between the card's colours and black. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSHADOW-009 frame 0: Shadow Only on: the card is gone and only its shadow is left, under where the card was too. | largest difference 1.5e-8 | yes |
| FX-RSHADOW-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSHADOW-010 frame 0: Opacity 0: no shadow; the frame is the drawing, untouched, though the layer still grows. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSHADOW-011 frame 0: Distance 40, the light keyed from the top-left corner at frame 0 to the top-right at frame 4, linear: frame 0 is FX-RSHADOW-004, frame 2 has the light at the top of the middle, and at frame 4 the shadow is thrown down and to the left. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-011 frame 2: Distance 40, the light keyed from the top-left corner at frame 0 to the top-right at frame 4, linear: frame 0 is FX-RSHADOW-004, frame 2 has the light at the top of the middle, and at frame 4 the shadow is thrown down and to the left. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-011 frame 4: Distance 40, the light keyed from the top-left corner at frame 0 to the top-right at frame 4, linear: frame 0 is FX-RSHADOW-004, frame 2 has the light at the top of the middle, and at frame 4 the shadow is thrown down and to the left. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSHADOW-012 frame 0: The light at the top-left, distance keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 is FX-RSHADOW-002, frame 4 is FX-RSHADOW-004, and frame 2 lies between. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-012 frame 2: The light at the top-left, distance keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 is FX-RSHADOW-002, frame 4 is FX-RSHADOW-004, and frame 2 lies between. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-012 frame 4: The light at the top-left, distance keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 is FX-RSHADOW-002, frame 4 is FX-RSHADOW-004, and frame 2 lies between. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSHADOW-013 frame 0: Opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 2 is held at 100 and is frame 4. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-013 frame 2: Opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 2 is held at 100 and is frame 4. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-013 frame 4: Opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 2 is held at 100 and is frame 4. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSHADOW-014 frame 0: FX-RSHADOW-004 moved three pixels right: the shadow moves with the drawing. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSHADOW-015 frame 0: The light far off to the right (1000, 50), distance 1000: the shadow is thrown far to the left, past the most the layer grows, its own width, so none of it is kept, and the frame is the drawing. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RSHADOW-016 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-016 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RSHADOW-017 frame 0: Distance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-017 frame 4: Distance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RSHADOW-018 frame 0: Distance 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-018 frame 4: Distance 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RSHADOW-019 frame 0: Softness 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-019 frame 4: Softness 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RSHADOW-020 frame 0: A colour written "black". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-020 frame 4: A colour written "black". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RSHADOW-021 frame 0: Render "glassy", which is not "regular" or "glass_edge". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-021 frame 4: Render "glassy", which is not "regular" or "glass_edge". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RSHADOW-022 frame 0: Shadow only "yes", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-022 frame 4: Shadow only "yes", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RSHADOW-023 frame 0: Colour influence 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-023 frame 4: Colour influence 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RSHADOW-024 frame 0: Light 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-024 frame 4: Light 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RSHADOW-025 frame 0: Distance keyed to 1200 at frame 4, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-025 frame 4: Distance keyed to 1200 at frame 4, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RSHADOW-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it declares no fixed growth, as Corner Pin does: how far the layer grows depends on the light and the layer's own size, and is worked out as it is drawn (FX-RSHADOW-001, 004 and 015 above grow it) | 0 | yes |
| a half-size draft preview halves the softening, a distance in pixels, and keeps the light, a share of the drawing, and the projection distance, a ratio | RadialShadow { color: "#2040a0", opacity: 75.0, light: [20.0, 30.0], distance: 40.0, softness: 6.0, render: "glass_edge", color_influence: 60.0, shadow_only: "on" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rshadow_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rshadow_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `render` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a light of one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a distance that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| opacity 101 is refused with a sentence, and nothing changes | Radial Shadow's opacity runs from 0 to 100, and this is 101. | yes |
| distance -1 is refused with a sentence, and nothing changes | Radial Shadow's distance runs from 0 to 1000, and this is -1. | yes |
| distance 1001 is refused with a sentence, and nothing changes | Radial Shadow's distance runs from 0 to 1000, and this is 1001. | yes |
| softness 501 is refused with a sentence, and nothing changes | Radial Shadow's softness runs from 0 to 500, and this is 501. | yes |
| colour influence 101 is refused with a sentence, and nothing changes | Radial Shadow's color influence runs from 0 to 100, and this is 101. | yes |
| light down 1001 is refused with a sentence, and nothing changes | Radial Shadow's light runs from -1000 to 1000, and this is 1001. | yes |
| colour "black" is refused with a sentence, and nothing changes | Radial Shadow's colour is written #rrggbb, and this is "black". | yes |
| render "glassy" is refused with a sentence, and nothing changes | Radial Shadow's render is "regular" or "glass_edge", and this is "glassy". | yes |
| render "Glass_Edge", written with capitals is refused with a sentence, and nothing changes | Radial Shadow's render is "regular" or "glass_edge", and this is "Glass_Edge". | yes |
| shadow only "yes" is refused with a sentence, and nothing changes | Radial Shadow's shadow only is "on" or "off", and this is "yes". | yes |
| distance keyed to 1200 is refused with a sentence, and nothing changes | Radial Shadow's distance runs from 0 to 1000, and this is 1200. | yes |
| light keyed to 2000 across is refused with a sentence, and nothing changes | Radial Shadow's light runs from -1000 to 1000, and this is 2000. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| light keyed across the top is taken | taken | yes |
| colour influence keyed from 0 to 100 is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rshadow_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rshadow_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rshadow_011.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rshadow_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a made-up cel, in `verification/B-146 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, opacity 0: the drawing as it is, drawn cleanly | [], 1664 pixels shown | yes |
| starts.png, as it starts: the light at the top of the middle, a tenth bigger, so half-black shadow peeps out below the chin at (80, 78) and nothing is above the hair at (80, 22); the drawing's covered pixels unchanged; draws cleanly | [], below [0, 0, 0, 128], above [0, 0, 0, 0], 0 changed | yes |
| corner.png, the light at the top-left, distance 40: the shadow thrown down and to the right, half black at (120, 85), nothing up and to the left at (50, 20); draws cleanly | [], [0, 0, 0, 128] and [0, 0, 0, 0], 0 changed | yes |
| middle.png, the light in the middle, distance 50: a dark ring on every side, left, right, above and below, where the drawing had nothing; draws cleanly | [], [[0, 0, 0, 128], [0, 0, 0, 128], [0, 0, 0, 128], [0, 0, 0, 128]] | yes |
| soft.png, corner.png with softening 12: the shadow's edge blurred, so it reaches more pixels than corner.png's; draws cleanly | [], pixels shown: soft 5840, corner 4512 | yes |
| blue.png, corner.png in blue #2040a0 at opacity 100: inside the shadow it is exactly that blue, fully covered; draws cleanly | [], [32, 64, 160, 255] | yes |
| glass.png, Glass Edge at colour influence 100: the shadow at (120, 85) is the skin it was cast from, (86, 61) on the drawing; glass_half.png, influence 50, is darker than the skin and lighter than black in every channel; both draw cleanly | [] [], skin [246, 214, 190, 255], glass [246, 214, 190, 255], half [181, 157, 139, 255] | yes |
| shadow_only.png, Shadow Only on: the drawing is gone, so (90, 60), skin before, is now the half-black shadow, and the shadow elsewhere is corner.png's; draws cleanly | [], before [246, 214, 190, 255], now [0, 0, 0, 128] | yes |
| low_right.png, the light low and off to the right, distance 60: the shadow thrown up and to the left, dark at (40, 20) where the drawing had nothing; draws cleanly | [], [0, 0, 0, 128] | yes |

## Result

126 of 126 checks pass.
