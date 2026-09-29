# B-118: Color Lookup

D-182, accepted on 2026-09-28 by the owner's words "3D trilinear, 1D if cheap. Collect/package copies the file; a missing .cube is diagnosed and kept." Every expected pixel, refusal and manifest line is `Fixtures/cube_lut/expected_color_lookup.json`, written by `tools/cube_lut_reference.py` before this code existed and printed in document 25 as FX-LUT-001 to 013. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-LUT-001 to 012 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LUT-001 frame 0: identity_2.cube, a 3D table of 2 points a side that gives each colour back: the drawing, unchanged but for rounding. | largest difference 1.9e-7 | yes |
| FX-LUT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LUT-002 frame 0: warm_17.cube, a warm look in a 3D table of 17 points a side, the size grading programs write: more contrast, white turned cream #fef4d8, black lifted to a dark brown #0f0904, the blue #3a6fd8 a duller #426cb3; each colour is the mix of the eight table lines around it. | largest difference 1.1e-7 | yes |
| FX-LUT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LUT-003 frame 0: cool_3.cube, a 3D table of 3 points a side written with Windows line ends, a title, comments between its lines, a tab and numbers such as -5e-2: read the same as any other; its red channel mixes in green, so only a 3D table holds it, and a value below 0 or above 1 is clamped to 0 or 1 after the mix: white turns a pale blue #e5f2ff and black a navy #00001a. | largest difference 2.0e-7 | yes |
| FX-LUT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LUT-004 frame 0: tint_1d.cube, a 1D table of 4 lines: each channel through its own curve, black to a dark brown #140d08 (the first line, 0.08 0.05 0.03) and white to a cream #fff0cc (the last), between them mixed from the two nearest lines. | largest difference 1.2e-7 | yes |
| FX-LUT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LUT-005 frame 0: domain.cube, a 3D identity table over the domain 0.25 to 0.75 for red, 0.2 to 0.9 for green and 0.1 to 1 for blue: each channel below its domain turns 0 and above it 1, and between is stretched to fill 0 to 1, so the picture gains contrast, most in red. | largest difference 1.6e-7 | yes |
| FX-LUT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LUT-006 frame 0: range_1d.cube, a 1D table of 2 lines over the input range 0 to 0.5: each channel doubles, and a channel above one half turns 1. | largest difference 1.4e-7 | yes |
| FX-LUT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LUT-007 frame 0: No lookup file chosen, the effect as it is added: the drawing, untouched, with no warning. | largest difference 1.9e-7 | yes |
| FX-LUT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LUT-008 frame 0: FX-LUT-002 moved three pixels right: the same, moved. | largest difference 1.1e-7 | yes |
| FX-LUT-008 frame 3: FX-LUT-002 moved three pixels right: the same, moved. | largest difference 1.1e-7 | yes |
| FX-LUT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LUT-009 frame 0: The lookup asset's file luts/gone.cube is not there: the drawing, untouched, with MEDIA_MISSING when the file is opened and at every frame; the asset and the setting are kept, to relink. | largest difference 1.9e-7 | yes |
| FX-LUT-009: what opening it warns of, and what frame 4 warns of | ["MEDIA_MISSING"] and ["MEDIA_MISSING"] | yes |
| FX-LUT-010 frame 0: The setting names asset-nothing, which the project does not have: the drawing, untouched, with EFFECT_PARAMETER_INVALID; the setting is kept as written. | largest difference 1.9e-7 | yes |
| FX-LUT-010: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LUT-011 frame 0: The setting names asset-bands, the drawing, which is not a lookup file: the drawing, untouched, with EFFECT_PARAMETER_INVALID. | largest difference 1.9e-7 | yes |
| FX-LUT-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LUT-012 frame 0: FX-LUT-002's effect on an adjustment layer above the drawing instead of on it: the same picture, as the drawing over nothing is the drawing. | largest difference 1.1e-7 | yes |
| FX-LUT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |

## FX-LUT-013: a lookup file the reading rule refuses

| Check | The build's answer | Matches |
| --- | --- | --- |
| opening it says nothing: the file is read at the frame, not on opening | [] | yes |
| frame 0: the drawing, untouched | largest difference 1.9e-7 | yes |
| frame 0 says MEDIA_DECODE_FAILED, once | ["MEDIA_DECODE_FAILED"] | yes |
| frame 4: the drawing, untouched | largest difference 1.9e-7 | yes |
| frame 4 says MEDIA_DECODE_FAILED, once | ["MEDIA_DECODE_FAILED"] | yes |
| the asset and the setting are kept as written | "luts/refused/count.cube" and "asset-lut" | yes |

## The reading rule: each refused file, and its reason word for word

| Check | The build's answer | Matches |
| --- | --- | --- |
| refused/big.cube is refused: line 1: LUT_3D_SIZE must be one whole number from 2 to 256 | line 1: LUT_3D_SIZE must be one whole number from 2 to 256 | yes |
| refused/both.cube is refused: the file gives both LUT_1D_SIZE and LUT_3D_SIZE, a 1D table before a 3D one, which this program does not read | the file gives both LUT_1D_SIZE and LUT_3D_SIZE, a 1D table before a 3D one, which this program does not read | yes |
| refused/count.cube is refused: the table has 7 lines where LUT_3D_SIZE 2 needs 8 | the table has 7 lines where LUT_3D_SIZE 2 needs 8 | yes |
| refused/domain_order.cube is refused: the domain's top must be above its bottom in each colour | the domain's top must be above its bottom in each colour | yes |
| refused/domain_short.cube is refused: line 2: DOMAIN_MAX must be three numbers | line 2: DOMAIN_MAX must be three numbers | yes |
| refused/domain_twice.cube is refused: the file gives its domain more than one way | the file gives its domain more than one way | yes |
| refused/empty.cube is refused: the file gives no LUT_3D_SIZE or LUT_1D_SIZE | the file gives no LUT_3D_SIZE or LUT_1D_SIZE | yes |
| refused/four.cube is refused: line 2: a table line must be three numbers | line 2: a table line must be three numbers | yes |
| refused/half.cube is refused: line 1: LUT_3D_SIZE must be one whole number from 2 to 256 | line 1: LUT_3D_SIZE must be one whole number from 2 to 256 | yes |
| refused/late.cube is refused: line 3: DOMAIN_MIN comes after the table has begun | line 3: DOMAIN_MIN comes after the table has begun | yes |
| refused/nan.cube is refused: line 2: a table line must be three numbers | line 2: a table line must be three numbers | yes |
| refused/no_size.cube is refused: the file gives no LUT_3D_SIZE or LUT_1D_SIZE | the file gives no LUT_3D_SIZE or LUT_1D_SIZE | yes |
| refused/not_text.cube is refused: the file is not text | the file is not text | yes |
| refused/range_short.cube is refused: line 2: LUT_1D_INPUT_RANGE must be two numbers | line 2: LUT_1D_INPUT_RANGE must be two numbers | yes |
| refused/twice.cube is refused: line 2: LUT_3D_SIZE is given twice | line 2: LUT_3D_SIZE is given twice | yes |
| refused/unknown.cube is refused: line 2: LUT_4D_SIZE is not a keyword of a .cube file this program reads | line 2: LUT_4D_SIZE is not a keyword of a .cube file this program reads | yes |
| refused/word.cube is refused: line 4: a table line must be three numbers | line 4: a table line must be three numbers | yes |
| a lookup file changed on disk is read afresh, not kept from before | Ok("read"), then Err("the table has 3 lines where LUT_1D_SIZE 2 needs 2") | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing, and a half-size draft leaves it as it is | 0, ColorLookup { lut: "asset-lut", table: None } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lut_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lut_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lut_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lut_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lut_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lut_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lut_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lut_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lut_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lut_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lut_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lut_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lut_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file whose layer shows the lookup file as its drawing is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| naming asset-nothing, which the project does not have, is refused with a sentence, and nothing changes | Color Lookup cannot use asset-nothing: it is not a lookup file of this project. | yes |
| naming asset-bands, a drawing and not a lookup file, is refused with a sentence, and nothing changes | Color Lookup cannot use asset-bands: it is not a lookup file of this project. | yes |
| a layer showing the lookup file as its drawing is refused with a sentence | "art" cannot show a colour lookup file: add Color Lookup to a layer and choose the file there. | yes |
| no file chosen, is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |

## Choosing a file on the card

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LUT-007 with warm_17.cube chosen is FX-LUT-002's frame | largest difference 1.1e-7 | yes |
| choosing cool_3.cube, a file the project does not have yet, brings it in and uses it: FX-LUT-003's frame | taken, largest difference 2.0e-7 | yes |
| one undo takes back both the new file and the setting | the frame and the asset list as they were | yes |

## Collect Files and Check Package

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LUT-002 collected: the drawing and the lookup file are both copied | [("media/asset-bands/bands.png", "copied"), ("media/asset-lut/warm_17.cube", "copied")] | yes |
| the manifest lists the lookup file as kind lut, used by the layer whose effect names it | Some((String("lut"), Array [Object {"composition": String("comp-main"), "layer": String("art")}])) | yes |
| the collected project names the copy, and draws the same frame from it | Some(String("media/asset-lut/warm_17.cube")), the same frame | yes |
| Check Package finds the lookup file and the drawing whole | ["media/asset-bands/bands.png ok", "media/asset-lut/warm_17.cube ok"] | yes |
| FX-LUT-009 collected: media/asset-lut/gone.cube is listed as missing, and the asset is kept | [("media/asset-bands/bands.png", "copied"), ("media/asset-lut/gone.cube", "missing")] | yes |

## Pictures: frame 100 of the reference shot, in `verification/B-118 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the shot with no lookup, draws cleanly | [] | yes |
| warm_17.png: warm_17.cube on an adjustment layer above the shot draws cleanly and changes the picture | [], 129600 of 129600 pixels changed | yes |
| cool_3.png: cool_3.cube on an adjustment layer above the shot draws cleanly and changes the picture | [], 129600 of 129600 pixels changed | yes |
| tint_1d.png: tint_1d.cube on an adjustment layer above the shot draws cleanly and changes the picture | [], 129600 of 129600 pixels changed | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lut_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lut_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lut_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

84 of 84 checks pass.
