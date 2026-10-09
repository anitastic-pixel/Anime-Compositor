# B-252: Path Stroke along text outlines

D-373, P0-22's third part: Path Stroke's Path From Text Outlines draws along a text layer's letters, every outline closed, glyph by glyph in reading order, text animators included. Every expected pixel and outline number is `Fixtures/stroke/expected_stroke_text.json`, written by `tools/stroke_text_reference.py` before this code existed and printed in document 25 as FX-STROKE-062 to 069. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-STROKE-062 to 069 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-STROKE-062 frame 0: "Yes", All Masks on: every letter's outline drawn in white, Brush Size 2, the e's and its eye's both. | largest difference 3.0e-8 | yes |
| FX-STROKE-062: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-063 frame 0: "Yes", Path 2: the e's eye alone. The Y has one outline and the e two, and the font stores the e's eye before its outer edge. | largest difference 2.9e-8 | yes |
| FX-STROKE-063: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-064 frame 0: "Yes", All Masks and Stroke Sequentially on, End keyed from 0 at frame 0 to 100 at frame 4: nothing, then the outlines one after another as one length to half way, then all. | largest difference 0.0e0 | yes |
| FX-STROKE-064 frame 2: "Yes", All Masks and Stroke Sequentially on, End keyed from 0 at frame 0 to 100 at frame 4: nothing, then the outlines one after another as one length to half way, then all. | largest difference 2.9e-8 | yes |
| FX-STROKE-064 frame 4: "Yes", All Masks and Stroke Sequentially on, End keyed from 0 at frame 0 to 100 at frame 4: nothing, then the outlines one after another as one length to half way, then all. | largest difference 3.0e-8 | yes |
| FX-STROKE-064: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-065 frame 0: "Yes", All Masks on, End 50, Stroke Sequentially off: each outline drawn half way round from where it begins. | largest difference 3.0e-8 | yes |
| FX-STROKE-065: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-066 frame 0: "Yes" with a text animator, Position (0, -6), Start 50: the e moved up 3, the s up 6, and the stroke follows them. | largest difference 3.0e-8 | yes |
| FX-STROKE-066: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STROKE-067 frame 0: Path From Text Outlines on a shape layer with no shapes, not a text layer. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 0.0e0 | yes |
| FX-STROKE-067 frame 4: Path From Text Outlines on a shape layer with no shapes, not a text layer. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 0.0e0 | yes |
| FX-STROKE-067: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-STROKE-068 frame 0: A text layer of one space, which has no outline. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 0.0e0 | yes |
| FX-STROKE-068 frame 4: A text layer of one space, which has no outline. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 0.0e0 | yes |
| FX-STROKE-068: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-STROKE-069: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-STROKE-069 frame 0 (Path 9 of four outlines) is the frame with the effect switched off | byte-identical | yes |

## The outlines: "Yes" in M PLUS Rounded 1c, size 24, against the reference

| Check | The build's answer | Matches |
| --- | --- | --- |
| 4 outlines (Y; e's eye, then its edge, as the font stores them; s) | 4 | yes |
| each outline cut into as many points as the reference's | [230, 65, 225, 384] (reference [230, 65, 225, 384]) | yes |
| each outline's length, closed, in pixels | ["61.4055", "18.9387", "53.2766", "59.8314"], largest difference 7.1e-15 | yes |
| the total length | 193.452106 (reference 193.452106) | yes |
| each outline begins where the reference's does (D-372's slide included) | largest difference 3.6e-15 | yes |
| Stroke Sequentially, End 50: what each outline draws (from, to), the frame is FX-STROKE-064 frame 2 | [Some((0.000, 61.405)), Some((0.000, 18.939)), Some((0.000, 16.382)), None] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_stroke_062.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_063.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_064.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_065.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_066.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_067.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_068.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stroke_069.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| Path From Text Outlines is written "text" | {"all_masks":"on","brush_hardness":75,"brush_size":2,"color":"#ffffff","end":100,"mask":1,"opacity":100,"paint_style":"on_transparent","source":"text","spacing":15,"start":0,"stroke_sequentially":"off"} | yes |
| End keyed from 0 to 100 is saved with its keys | {"base":0,"keyframes":[{"frame":0,"interp":"linear","value":0},{"frame":4,"interp":"linear","value":100}]} | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Path 3, All Masks off, is taken | taken | yes |
| End keyed from 0 at frame 0 to 100 at frame 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_stroke_062.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_stroke_064.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_stroke_066.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Draw on: GPU against the processor: within 1 level of 255 (the stroke itself on the processor, as every effect on a text layer is)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_stroke_062.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_063.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_064.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_065.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_066.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_067.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_068.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| fx_stroke_069.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; the stroke on the card in 0 of 10 frames (expected 0); the card refused a frame: false; the same warnings: true | yes |
| the town under "Write", Path Stroke along its outlines written on (End keyed 0 to 100), frame 0, Full | largest difference 1 of 255, 34 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under "Write", Path Stroke along its outlines written on (End keyed 0 to 100), frame 12, Full | largest difference 1 of 255, 128 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under "Write", Path Stroke along its outlines written on (End keyed 0 to 100), frame 24, Full | largest difference 1 of 255, 196 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under "Write", Path Stroke along its outlines written on (End keyed 0 to 100), frame 0, Draft | largest difference 1 of 255, 806 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under "Write", Path Stroke along its outlines written on (End keyed 0 to 100), frame 12, Draft | largest difference 1 of 255, 781 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under "Write", Path Stroke along its outlines written on (End keyed 0 to 100), frame 24, Draft | largest difference 1 of 255, 773 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under "Write", Path Stroke along its outlines Brush Size 24, Hardness 0, Spacing 0, Reveal Original Image, frame 0, Full | largest difference 1 of 255, 322 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under "Write", Path Stroke along its outlines Brush Size 24, Hardness 0, Spacing 0, Reveal Original Image, frame 12, Full | largest difference 1 of 255, 322 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under "Write", Path Stroke along its outlines Brush Size 24, Hardness 0, Spacing 0, Reveal Original Image, frame 24, Full | largest difference 1 of 255, 322 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under "Write", Path Stroke along its outlines Brush Size 24, Hardness 0, Spacing 0, Reveal Original Image, frame 0, Draft | largest difference 1 of 255, 824 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under "Write", Path Stroke along its outlines Brush Size 24, Hardness 0, Spacing 0, Reveal Original Image, frame 12, Draft | largest difference 1 of 255, 824 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |
| the town under "Write", Path Stroke along its outlines Brush Size 24, Hardness 0, Spacing 0, Reveal Original Image, frame 24, Draft | largest difference 1 of 255, 824 pixels differ; the stroke on the card 0 (expected 0); warnings CPU [] GPU [] | yes |

## Pictures: the town under a text layer, in `verification/D-373 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| write_strip.png (End 0, 50 and 100, each also alone as write_end_NNN.png): "Write" in white over the town, an orange line drawing itself round the letters' edges one after another, W first; nothing changes further than the brush from an outline | pixels changed and of them off the outlines, per frame [(0, 0), (7211, 0), (14418, 0)] | yes |
| write_reveal.png: Reveal Original Image, Brush Size 24: only a soft band along the letters' edges shows, the white letters' insides and the town under them seen only there | [] | yes |

## Result

64 of 64 checks pass.
