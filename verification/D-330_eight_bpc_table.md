# D-330: 8 bpc (After Effects) working depth

From P-26: tutorial 2 is drawn in After Effects' 8 bpc until its step F, and its reflection blurs the bolt's tips wide and lifts them 17 stops with Exposure. In 8 bpc After Effects blurs display colours and keeps each layer's pixels as whole numbers 0 to 255 between effects, so the blur's faint edge is 0 before Exposure sees it. The working depth gains a third choice, 8 bpc: after every effect each pixel is rounded to 8 bits, and the four blurs average display colours. Every expected pixel is `Fixtures/eight_bpc/expected_eight_bpc.json`, written by `tools/eight_bpc_reference.py` before the build had 8 bpc. Tolerance 2e-5.

## FX-8BPC-001 to 005 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-8BPC-001 frame 0: 8 bpc: Fast Box Blur radius 2 (3 passes), then Exposure +6. The blur averages display values and its faint edge rounds to 0, so the light ends sooner than in Float (FX-FASTBOX-007), and nothing past white is kept. | largest difference 1.3e-8 | yes |
| FX-8BPC-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-8BPC-002 frame 0: 8 bpc: Gaussian Blur, Blurriness 4, then Exposure +6. The long faint tail that Float lights (FX-BLURRY-007) rounds to 0. | largest difference 2.5e-8 | yes |
| FX-8BPC-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-8BPC-003 frame 0: 8 bpc: Exposure -1 alone, worked in linear light, then rounded to 8 bits. | largest difference 1.4e-7 | yes |
| FX-8BPC-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-8BPC-004 frame 0: 8 bpc: Fast Box Blur radius 2 alone. Every value is a whole 255th in display values. | largest difference 1.3e-8 | yes |
| FX-8BPC-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-8BPC-005 frame 0: 8 bpc with no effect: the drawing untouched. | largest difference 3.1e-8 | yes |
| FX-8BPC-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| fx_8bpc_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_8bpc_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_8bpc_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| both_depths.json: `float_depth` and `eight_bpc` both true: refused, `PROJECT_SCHEMA_INVALID`. | Some("PROJECT_SCHEMA_INVALID") | yes |
| eight_word.json: `eight_bpc` is the word "yes", not true or false: refused, `PROJECT_SCHEMA_INVALID`. | Some("PROJECT_SCHEMA_INVALID") | yes |

## Saved, read back and undone

| Check | The build's answer | Matches |
| --- | --- | --- |
| Set to Display, no eight_bpc line is written | no line | yes |
| And FX-8BPC-004's blur is no longer rounded | largest difference 1.9e-3 | yes |
| Set to 8 bpc through the command, it is written as eight_bpc: true | Some(Bool(true)) | yes |
| And draws FX-8BPC-004's frame again | largest difference 1.3e-8 | yes |
| And is read back as 8 bpc | 8 bpc | yes |
| Undo puts it back to Display, and nothing is written | Display, no line | yes |
| Float and 8 bpc both at once is refused by the command | Err("A composition is in one working depth: 8 bpc or Float, not both.") | yes |

## A composition inside another follows the outermost one

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-8BPC-001's layer inside a composition: outer 8 bpc, inner Display, draws FX-8BPC-001's frame | largest difference 1.3e-8 | yes |
| FX-8BPC-001's layer inside a composition: outer Display, inner 8 bpc, is not rounded | largest difference 2.0e1 | yes |
| FX-8BPC-001's layer inside a composition: outer 8 bpc, inner 8 bpc, draws FX-8BPC-001's frame | largest difference 1.3e-8 | yes |

## The viewer

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-8BPC-001 in the viewer, first time (its effects are not kept between frames) | largest difference 1.3e-8 | yes |
| FX-8BPC-001 in the viewer, second time (its effects are not kept between frames) | largest difference 1.3e-8 | yes |
| fx_8bpc_001.json: the preview draws the CPU's picture, within 1 level of 255 (the effects all run on the CPU) | card, largest difference 0 of 255 | yes |
| fx_8bpc_002.json: the preview draws the CPU's picture, within 1 level of 255 (the effects all run on the CPU) | card, largest difference 0 of 255 | yes |
| fx_8bpc_004.json: the preview draws the CPU's picture, within 1 level of 255 (the effects all run on the CPU) | card, largest difference 0 of 255 | yes |

## Pictures: tutorial 2's reflection, in `verification/D-330 pictures/`, over black

| Check | The build's answer | Matches |
| --- | --- | --- |
| built_1_display.png, Display, as before: the blur's faint edge lit wide; draws cleanly | [], 83207 pixels show | yes |
| built_2_float.png, Float: the same wide light; draws cleanly | [], 83207 pixels show | yes |
| built_3_eight_bpc.png, 8 bpc: the faint edge rounds to nothing, one small spot where the tips are; draws cleanly | [], 6691 pixels show | yes |
| In 8 bpc under a tenth as many pixels show as in Display | Display 83207, Float 83207, 8 bpc 6691 | yes |

## Files from before D-330

| Check | The build's answer | Matches |
| --- | --- | --- |
| The reference shot is not 8 bpc in any composition and saves no eight_bpc line | no line | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_8bpc_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_8bpc_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

37 of 37 checks pass.
