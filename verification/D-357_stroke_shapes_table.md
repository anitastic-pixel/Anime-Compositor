# B-236: Path Stroke along shape paths

D-357, P0-22's second part: Path Stroke's Path From Shape Paths draws along a shape layer's own shapes, open or closed as each says. Every expected pixel is `Fixtures/stroke/expected_stroke_shapes.json`, written by `tools/stroke_reference.py` before this code existed and printed in document 25 as FX-STROKE-047 to 061. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-STROKE-047 to 061 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-STROKE-047 frame 0: A shape layer holding one shape, the box, with no fill and no stroke (it draws nothing), Path From Shapes: the white line all round the box over nothing, FX-STROKE-013's frame. | largest difference 1.3e-8 | yes |
| FX-STROKE-047: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-048 frame 0: A star of five points (ten corners) as a shape, Brush Size 1, End 60: the line follows its points and dips, clockwise from the top point, three fifths of the way round; the left arms are not drawn yet. | largest difference 2.9e-8 | yes |
| FX-STROKE-048: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-049 frame 0: An open shape, a roof of two legs from (2, 7) up to (8, 2) and down to (14, 7): drawn open, nothing along the bottom. | largest difference 2.9e-8 | yes |
| FX-STROKE-049: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-050 frame 0: The same roof closed: the bottom is drawn too. | largest difference 2.9e-8 | yes |
| FX-STROKE-050: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-051 frame 0: The open roof, End 50: the left leg alone, half of its open length. | largest difference 2.9e-8 | yes |
| FX-STROKE-051: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-052 frame 0: A curved shape, the circle 8 across about (8, 5): FX-STROKE-022's curve. | largest difference 2.9e-8 | yes |
| FX-STROKE-052: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-053 frame 0: Two shapes, the box and the small box, Path 2: the small box alone. | largest difference 1.1e-8 | yes |
| FX-STROKE-053: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-054 frame 0: Two shapes, All Masks and Stroke Sequentially on, End keyed from 0 at frame 0 to 100 at frame 4: the box draws on, then the small box, as FX-STROKE-020 does with masks; frames 0, 2 and 4. | largest difference 0.0e0 | yes |
| FX-STROKE-054 frame 2: Two shapes, All Masks and Stroke Sequentially on, End keyed from 0 at frame 0 to 100 at frame 4: the box draws on, then the small box, as FX-STROKE-020 does with masks; frames 0, 2 and 4. | largest difference 2.9e-8 | yes |
| FX-STROKE-054 frame 4: Two shapes, All Masks and Stroke Sequentially on, End keyed from 0 at frame 0 to 100 at frame 4: the box draws on, then the small box, as FX-STROKE-020 does with masks; frames 0, 2 and 4. | largest difference 1.3e-8 | yes |
| FX-STROKE-054: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-055 frame 0: The box as a shape filled blue: the stroke is drawn over the fill. | largest difference 3.0e-8 | yes |
| FX-STROKE-055: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-056 frame 0: A shape layer with the box as a shape and the small box as a mask of mode None, Path From Shapes: the box alone, FX-STROKE-047's frame. | largest difference 1.3e-8 | yes |
| FX-STROKE-056: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-057 frame 0: The same layer, Path From Masks: the small box, its mask, alone. | largest difference 1.1e-8 | yes |
| FX-STROKE-057: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-058 frame 0: Path From Shapes on the night drawing, which is not a shape layer and has no shapes (its box mask is not a shape). Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-STROKE-058 frame 4: Path From Shapes on the night drawing, which is not a shape layer and has no shapes (its box mask is not a shape). Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-STROKE-058: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-STROKE-059 frame 0: A shape layer whose one shape is switched off. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 0.0e0 | yes |
| FX-STROKE-059 frame 4: A shape layer whose one shape is switched off. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 0.0e0 | yes |
| FX-STROKE-059: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-STROKE-060 frame 0: A shape layer of one shape, Path 2. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 0.0e0 | yes |
| FX-STROKE-060 frame 4: A shape layer of one shape, Path 2. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 0.0e0 | yes |
| FX-STROKE-060: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-STROKE-061 frame 0: Path From "layer", which is not "masks" or "shapes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-061 frame 4: Path From "layer", which is not "masks" or "shapes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-STROKE-061: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_stroke_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_054.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_055.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_056.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_057.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_058.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_059.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_060.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_061.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| Path From Masks, the default, is not written: D-356's files save as they were | {"all_masks":"off","brush_hardness":75,"brush_size":3,"color":"#ffffff","end":100,"mask":1,"opacity":100,"paint_style":"on_original","spacing":15,"start":0,"stroke_sequentially":"off"} | yes |
| Path From Shape Paths is written "shapes" | {"all_masks":"off","brush_hardness":75,"brush_size":3,"color":"#ffffff","end":100,"mask":1,"opacity":100,"paint_style":"on_original","source":"shapes","spacing":15,"start":0,"stroke_sequentially":"off"} | yes |
| fx_stroke_061.json is refused in a sentence | Path Stroke's source is "masks" or "shapes", and this is "layer". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Path From "layer" is refused with a sentence, and nothing changes | Path Stroke's source is "masks" or "shapes", and this is "layer". | yes |
| End 50, Path From Masks, is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_stroke_048.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_stroke_049.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_stroke_054.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_stroke_055.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Draw on: GPU against the processor: within 1 level of 255 (the stroke itself on the processor, as every effect on a shape layer is)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_stroke_047.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_048.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_049.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_050.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_051.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_052.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_053.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_054.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_055.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_056.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_057.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_058.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_059.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_060.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| the town under a shape layer, Path Stroke along the star written on (End keyed 0 to 100), frame 0, Full | largest difference 0 of 255, 0 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the star written on (End keyed 0 to 100), frame 12, Full | largest difference 1 of 255, 68 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the star written on (End keyed 0 to 100), frame 24, Full | largest difference 1 of 255, 137 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the star written on (End keyed 0 to 100), frame 0, Draft | largest difference 1 of 255, 904 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the star written on (End keyed 0 to 100), frame 12, Draft | largest difference 1 of 255, 873 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the star written on (End keyed 0 to 100), frame 24, Draft | largest difference 1 of 255, 836 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the open zigzag, frame 0, Full | largest difference 1 of 255, 79 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the open zigzag, frame 12, Full | largest difference 1 of 255, 79 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the open zigzag, frame 24, Full | largest difference 1 of 255, 79 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the open zigzag, frame 0, Draft | largest difference 1 of 255, 863 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the open zigzag, frame 12, Draft | largest difference 1 of 255, 863 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the open zigzag, frame 24, Draft | largest difference 1 of 255, 863 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the zigzag closed, frame 0, Full | largest difference 1 of 255, 108 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the zigzag closed, frame 12, Full | largest difference 1 of 255, 108 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the zigzag closed, frame 24, Full | largest difference 1 of 255, 108 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the zigzag closed, frame 0, Draft | largest difference 1 of 255, 853 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the zigzag closed, frame 12, Draft | largest difference 1 of 255, 853 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the zigzag closed, frame 24, Draft | largest difference 1 of 255, 853 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the star, Brush Size 30, Hardness 0, Spacing 100, Reveal Original Image, frame 0, Full | largest difference 0 of 255, 0 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the star, Brush Size 30, Hardness 0, Spacing 100, Reveal Original Image, frame 12, Full | largest difference 0 of 255, 0 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the star, Brush Size 30, Hardness 0, Spacing 100, Reveal Original Image, frame 24, Full | largest difference 0 of 255, 0 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the star, Brush Size 30, Hardness 0, Spacing 100, Reveal Original Image, frame 0, Draft | largest difference 1 of 255, 904 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the star, Brush Size 30, Hardness 0, Spacing 100, Reveal Original Image, frame 12, Draft | largest difference 1 of 255, 904 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under a shape layer, Path Stroke along the star, Brush Size 30, Hardness 0, Spacing 100, Reveal Original Image, frame 24, Draft | largest difference 1 of 255, 904 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |

## Pictures: the town under a shape layer, in `verification/D-357 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| star_strip.png (frames 0, 6, 12, 18 and 24, each also alone as star_fNN.png), a five-pointed star drawn as a shape, End keyed from 0 to 100: the orange line draws itself round the star clockwise from the top point; nothing changes off the star | pixels changed per frame [0, 1824, 3589, 5353, 7038] | yes |
| zigzag_open.png, a zigzag of four legs drawn as a shape, open: nothing changes off its path | [], 4991 pixels changed, 0 off the path | yes |
| zigzag_closed.png, a zigzag of four legs drawn as a shape, closed: nothing changes off its path | [], 7910 pixels changed, 0 off the path | yes |
| open, the zigzag's ends are not joined: the pixel at (100, 193) on the closing leg is the street's, and the closed one draws it | open [200, 180, 120, 255], closed [255, 136, 0, 255], street [200, 180, 120, 255] | yes |

## Result

103 of 103 checks pass.
