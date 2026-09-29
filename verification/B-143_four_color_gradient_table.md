# B-143: 4-Color Gradient

D-208, accepted on 2026-09-28 with the After Effects picks (B6). Every expected pixel is `Fixtures/four_color_gradient/expected_four_color_gradient.json`, written by `tools/four_color_gradient_reference.py` before this code existed and printed in document 25 as FX-4CG-001 to 026. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-4CG-001 to 026 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-4CG-001 frame 0: The settings as they start: yellow at the top-left, (10, 10) per cent, green at the top-right, (90, 10), magenta at the bottom-left, (10, 90), and blue at the bottom-right, (90, 90), blend 100, opacity 100, normal: the cel is painted over with the four colours running into each other, each strongest at its own corner, the line and the shadow gone under it, every pixel at its own covering, and the empty pixels empty. | largest difference 4.1e-8 | yes |
| FX-4CG-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-002 frame 0: Blend 1: each pixel all but wholly its nearest point's colour, four quarters meeting at hard seams down the middle and across it. | largest difference 3.0e-8 | yes |
| FX-4CG-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-003 frame 0: Blend 1000: the four colours mixed nearly evenly, the whole cel close to one colour. | largest difference 3.0e-8 | yes |
| FX-4CG-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-004 frame 0: Point 1 at (28.125, 25) per cent, the centre of the pixel in column 4 and row 2: that pixel is yellow exactly. | largest difference 3.9e-8 | yes |
| FX-4CG-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-005 frame 0: Opacity 50: the colours laid on at half strength over the cel. | largest difference 1.2e-7 | yes |
| FX-4CG-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-006 frame 0: Opacity 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-4CG-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-007 frame 0: Blending mode multiply: the cel darkened and tinted by the colours, the line still showing through. | largest difference 1.7e-7 | yes |
| FX-4CG-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-008 frame 0: Blending mode screen: the cel lightened by the colours, none darkened. | largest difference 1.8e-7 | yes |
| FX-4CG-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-009 frame 0: Blending mode add: the colours added to the cel. | largest difference 2.4e-7 | yes |
| FX-4CG-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-010 frame 0: Colours written in capitals, #FF0000, #00FF00, #0000FF and #FFFFFF: red, green, blue and white. | largest difference 3.7e-8 | yes |
| FX-4CG-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-011 frame 0: All four colours #6450a0: every pixel that shows is #6450a0 at its own covering, wherever the points are. | largest difference 3.0e-8 | yes |
| FX-4CG-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-012 frame 0: All four points at (50, 50): every pixel as far from each, so every pixel is the four colours mixed evenly. | largest difference 3.0e-8 | yes |
| FX-4CG-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-013 frame 0: Point 1 keyed from (10, 10) at frame 0 to (90, 90) at frame 4, linear: the yellow slides across the cel; frame 0 is FX-4CG-001. | largest difference 4.1e-8 | yes |
| FX-4CG-013 frame 2: Point 1 keyed from (10, 10) at frame 0 to (90, 90) at frame 4, linear: the yellow slides across the cel; frame 0 is FX-4CG-001. | largest difference 4.0e-8 | yes |
| FX-4CG-013 frame 4: Point 1 keyed from (10, 10) at frame 0 to (90, 90) at frame 4, linear: the yellow slides across the cel; frame 0 is FX-4CG-001. | largest difference 3.4e-8 | yes |
| FX-4CG-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-014 frame 0: Blend keyed from 100 at frame 0 to 1000 at frame 4, eased past its end: frame 2 would pass 1000, is held at 1000, and is blend 1000. | largest difference 4.1e-8 | yes |
| FX-4CG-014 frame 2: Blend keyed from 100 at frame 0 to 1000 at frame 4, eased past its end: frame 2 would pass 1000, is held at 1000, and is blend 1000. | largest difference 3.0e-8 | yes |
| FX-4CG-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-015 frame 0: FX-4CG-001 moved three pixels right: the colours move with the drawing. | largest difference 4.1e-8 | yes |
| FX-4CG-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-016 frame 0: After a Motion Tile that grows the layer: the points are the drawing's own, so the frame is FX-4CG-001's. | largest difference 4.1e-8 | yes |
| FX-4CG-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-017 frame 0: The points outside the drawing, at (-100, -100), (200, -100), (-100, 200) and (200, 200): the colours mix more evenly across the cel than FX-4CG-001. | largest difference 3.0e-8 | yes |
| FX-4CG-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-018 frame 0: Blend 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-018 frame 4: Blend 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-4CG-019 frame 0: Blend 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-019 frame 4: Blend 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-4CG-020 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-020 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-4CG-021 frame 0: Opacity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-021 frame 4: Opacity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-4CG-022 frame 0: Point 3 1001 per cent across, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-022 frame 4: Point 3 1001 per cent across, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-4CG-023 frame 0: Colour 2 "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-023 frame 4: Colour 2 "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-4CG-024 frame 0: Colour 4 "blue", a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-024 frame 4: Colour 4 "blue", a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-4CG-025 frame 0: Blending mode "darken", which is not "normal", "multiply", "screen" or "add". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-025 frame 4: Blending mode "darken", which is not "normal", "multiply", "screen" or "add". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-4CG-026 frame 0: Opacity keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-026 frame 4: Opacity keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it never grows the layer: it declares no growth | 0 | yes |
| a half-size draft preview changes nothing: the points are in per cent, and blend and opacity are not distances | FourColorGradient { point_1: [10.0, 10.0], point_2: [90.0, 10.0], point_3: [10.0, 90.0], point_4: [90.0, 90.0], color_1: "#ffff00", color_2: "#00ff00", color_3: "#ff00ff", color_4: "#0000ff", blend: 60.0, opacity: 80.0, blending_mode: "multiply" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_4cg_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_010.json, its colours written in capitals, is saved in small letters, as Snowfall's is | "#ff0000" "#00ff00" "#0000ff" "#ffffff" | yes |
| a file with no `blending_mode` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a point 1 that is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a colour 3 that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| blend 0 is refused with a sentence, and nothing changes | 4-Color Gradient's blend runs from 1 to 1000, and this is 0. | yes |
| blend 1001 is refused with a sentence, and nothing changes | 4-Color Gradient's blend runs from 1 to 1000, and this is 1001. | yes |
| opacity -1 is refused with a sentence, and nothing changes | 4-Color Gradient's opacity runs from 0 to 100, and this is -1. | yes |
| opacity 101 is refused with a sentence, and nothing changes | 4-Color Gradient's opacity runs from 0 to 100, and this is 101. | yes |
| point 1 -1001 across is refused with a sentence, and nothing changes | 4-Color Gradient's point 1 runs from -1000 to 1000, and this is -1001. | yes |
| point 4 1001 down is refused with a sentence, and nothing changes | 4-Color Gradient's point 4 runs from -1000 to 1000, and this is 1001. | yes |
| colour 2 "#12345" is refused with a sentence, and nothing changes | 4-Color Gradient's colour 2 is written #rrggbb, and this is "#12345". | yes |
| colour 4 "blue" is refused with a sentence, and nothing changes | 4-Color Gradient's colour 4 is written #rrggbb, and this is "blue". | yes |
| blending mode "darken" is refused with a sentence, and nothing changes | 4-Color Gradient's blending mode is "normal", "multiply", "screen" or "add", and this is "darken". | yes |
| blending mode "Normal", written with a capital is refused with a sentence, and nothing changes | 4-Color Gradient's blending mode is "normal", "multiply", "screen" or "add", and this is "Normal". | yes |
| opacity keyed to 101 is refused with a sentence, and nothing changes | 4-Color Gradient's opacity runs from 0 to 100, and this is 101. | yes |
| blend keyed to 0 is refused with a sentence, and nothing changes | 4-Color Gradient's blend runs from 1 to 1000, and this is 0. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| blend keyed from 1 to 1000 is taken | taken | yes |
| point 2 keyed from (90, 10) to (10, 90) is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_4cg_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_4cg_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_4cg_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_4cg_013.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a made-up cel, in `verification/B-143 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the drawing with no effect, draws cleanly | [] | yes |
| as_it_starts.png, as it starts: yellow in the top-left corner of the card, green in the top-right, magenta in the bottom-left and blue in the bottom-right, the drawing's lines painted over; every covering kept, nothing around the card; draws cleanly | [], corners [254, 254, 1, 255] [1, 254, 1, 255] [254, 1, 254, 255] [1, 1, 254, 255] | yes |
| blend_1.png, blend 1: four flat patches, each point's own colour, yellow, green, magenta and blue, to within 1 in each quarter; draws cleanly | [], [[255, 255, 0, 255], [0, 255, 0, 255], [255, 0, 255, 255], [0, 0, 255, 255]] | yes |
| blend_1000.png, blend 1000: the four mixed nearly evenly, every channel across the card spanning under half of what it spans as it starts; draws cleanly | [], widest channel 85 against 255 | yes |
| opacity_50.png, opacity 50: the colours laid on at half strength, the drawing's dark line showing through, darker at the head's top than as it starts; draws cleanly | [], [93, 127, 64, 255] against [126, 172, 83, 255] at the head's top | yes |
| multiply.png, multiply: tinted and darkened, the lines kept dark, no pixel lighter than the drawing; every covering kept; draws cleanly | [], 11097 pixels changed, 0 the wrong way | yes |
| screen.png, screen: lightened, no pixel darker than the drawing; every covering kept; draws cleanly | [], 11097 pixels changed, 0 the wrong way | yes |
| add_30.png, add at opacity 30: lightened, no pixel darker than the drawing; every covering kept; draws cleanly | [], 11097 pixels changed, 0 the wrong way | yes |
| point_1_middle.png, point 1 moved to the middle: yellow in the middle of the head, at 80, 50; draws cleanly | [], [255, 255, 0, 255] at 80, 50 | yes |
| sunset_multiply.png, gold, orange, purple and navy laid on by multiply: warm along the top, red over blue, and cool along the bottom, blue over red; draws cleanly | [], [160, 102, 75, 255] at the top, [74, 57, 103, 255] at the bottom | yes |

## Result

126 of 126 checks pass.
