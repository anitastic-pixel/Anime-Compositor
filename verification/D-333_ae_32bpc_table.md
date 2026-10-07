# D-333: 32 bpc (After Effects) working depth

From P-26: after its step F tutorial 2 is in After Effects' 32 bpc, which by default has no linear working space. Its Lightning Diff copy blurs the bolt's tips wide and lifts them 20 stops with Exposure; in Float this program lit a block where the tutorial shows a soft pool. The working depth gains a fourth choice, 32 bpc (After Effects): Float, except that the four blurs average display colours and Exposure works through a 2.2 curve, which lifts faint light far less. Which curve After Effects uses is not written anywhere found; the 2.2 curve is this program's best reading. Every expected pixel is `Fixtures/ae_32bpc/expected_ae_32bpc.json`, written by `tools/ae_32bpc_reference.py` before the build had this depth. Tolerance 2e-5.

## FX-AE32-001 to 006 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-AE32-001 frame 0: 32 bpc (After Effects): Fast Box Blur radius 2 (3 passes), Solid Composite on black, then Exposure +4, tutorial 2's reflection in small. The blur averages display values and Exposure lifts them through the 2.2 curve, so the faint edge stays dim where Float lights it; light past white is kept. | largest difference 7.9e-7 | yes |
| FX-AE32-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AE32-002 frame 0: 32 bpc (After Effects): Gaussian Blur, Blurriness 4, Solid Composite on black, then Exposure +4. The long faint tail stays dim. | largest difference 1.9e-6 | yes |
| FX-AE32-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AE32-003 frame 0: 32 bpc (After Effects): Exposure -1 alone, the display value times 2^(-1/2.2). | largest difference 1.1e-7 | yes |
| FX-AE32-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AE32-004 frame 0: 32 bpc (After Effects): Exposure +2, then Fast Box Blur radius 2. The blur averages display values past white without holding them. | largest difference 3.6e-7 | yes |
| FX-AE32-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AE32-005 frame 0: 32 bpc (After Effects): Solid Composite on black, then Fast Box Blur radius 2. The orange fades to black through display values. | largest difference 2.0e-7 | yes |
| FX-AE32-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AE32-006 frame 0: 32 bpc (After Effects) with no effect: the drawing untouched. | largest difference 3.1e-8 | yes |
| FX-AE32-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| fx_ae32_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ae32_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ae32_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| ae32_alone.json: `ae_32bpc` true without `float_depth`: refused, `PROJECT_SCHEMA_INVALID`. | Some("PROJECT_SCHEMA_INVALID") | yes |
| ae32_word.json: `ae_32bpc` is the word "yes", not true or false: refused, `PROJECT_SCHEMA_INVALID`. | Some("PROJECT_SCHEMA_INVALID") | yes |

## Saved, read back and undone

| Check | The build's answer | Matches |
| --- | --- | --- |
| Set to Float, no ae_32bpc line is written | no line | yes |
| And FX-AE32-001 is drawn as Float draws it, not as expected here | largest difference 3.8e0 | yes |
| Set to 32 bpc (After Effects) through the command, it is written as ae_32bpc: true | Some(Bool(true)) | yes |
| And draws FX-AE32-001's frame again | largest difference 7.9e-7 | yes |
| And is read back as 32 bpc (After Effects), Float on | float true, ae true | yes |
| Undo puts it back to Float, and nothing is written | Float, no line | yes |
| 32 bpc (After Effects) without Float is refused by the command | Err("32 bpc (After Effects) is a kind of Float: turn Float on with it.") | yes |

## A composition inside another follows the outermost one

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-AE32-001's layer inside a composition: outer AE, inner Display, draws FX-AE32-001's frame | largest difference 7.9e-7 | yes |
| FX-AE32-001's layer inside a composition: outer Display, inner AE, does not | largest difference 3.8e0 | yes |
| FX-AE32-001's layer inside a composition: outer Float, inner AE, does not | largest difference 3.8e0 | yes |
| FX-AE32-001's layer inside a composition: outer AE, inner AE, draws FX-AE32-001's frame | largest difference 7.9e-7 | yes |

## The viewer

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-AE32-001 in the viewer, first time (its effects are not kept between frames) | largest difference 7.9e-7 | yes |
| FX-AE32-001 in the viewer, second time (its effects are not kept between frames) | largest difference 7.9e-7 | yes |
| fx_ae32_001.json: the preview draws the CPU's picture, within 1 level of 255 | CPU, largest difference 0 of 255 | yes |
| fx_ae32_002.json: the preview draws the CPU's picture, within 1 level of 255 | CPU, largest difference 0 of 255 | yes |
| fx_ae32_005.json: the preview draws the CPU's picture, within 1 level of 255 | CPU, largest difference 0 of 255 | yes |

## Pictures: tutorial 2's Lightning Diff copy after step F, in `verification/D-333 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| built_1_float.png, Float, as before: a block of light about 600 pixels wide; draws cleanly | [], 206573 pixels brighter than a quarter | yes |
| built_2_ae_32bpc.png, 32 bpc (After Effects): a soft pool where the bolt lands; draws cleanly | [], 23267 pixels brighter than a quarter | yes |
| In 32 bpc (After Effects) under a quarter as many pixels are brighter than a quarter as in Float | Float 206573, 32 bpc (After Effects) 23267 | yes |

## Files from before D-333

| Check | The build's answer | Matches |
| --- | --- | --- |
| The reference shot is not 32 bpc (After Effects) in any composition and saves no ae_32bpc line | no line | yes |
| A Float file from D-319 stays plain Float and saves no ae_32bpc line | no line | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ae32_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ae32_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

40 of 40 checks pass.
