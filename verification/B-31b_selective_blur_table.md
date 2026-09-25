# B-31b: selective colour blur

D-87, accepted by the owner on 2026-09-25. Every expected pixel is `Fixtures/selblur/expected_selblur.json`, written by `tools/selblur_reference.py` before this code existed and printed in document 25 as FX-SELBLUR-001 to 024. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-SELBLUR-001 to 024 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SELBLUR-001 frame 0: Skin beside shadow with no line between, both chosen, blur 6: the edge turns into a soft ramp across the whole row, the same on every row. | largest difference 2.0e-7 | yes |
| FX-SELBLUR-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-002 frame 0: The same at blur 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-003 frame 0: The same with only the shadow chosen: it has no chosen colour beside it to mix with, so nothing changes. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-004 frame 0: Skin and shadow with a black line between, both chosen: the line is not chosen, so neither colour reaches the other and nothing changes. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-005 frame 0: Skin, shadow and highlight, with skin and shadow chosen: the skin and shadow edge goes soft; the highlight stays exactly as drawn, and none of it mixes into the shadow. | largest difference 2.2e-7 | yes |
| FX-SELBLUR-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-006 frame 0: The halves as a block on nothing, with black chosen as well: nothing stays nothing, even though its colour is black, and the soft edge reaches no further than the block. | largest difference 2.2e-7 | yes |
| FX-SELBLUR-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-007 frame 0: The halves with the top four rows half covering: every pixel keeps its own covering, and its colour is FX-SELBLUR-001's. | largest difference 2.0e-7 | yes |
| FX-SELBLUR-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-008 frame 0: The halves with the skin chosen one step off in blue (#f6d6bf) beside the shadow: the skin is not chosen, so nothing changes. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-009 frame 0: Blur keyed from 0 at frame 0 to 12 at frame 4, linear: frame 0 is FX-SELBLUR-002, frame 2 is FX-SELBLUR-001, frame 4 is the blur at 12. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-009 frame 2: Blur keyed from 0 at frame 0 to 12 at frame 4, linear: frame 0 is FX-SELBLUR-002, frame 2 is FX-SELBLUR-001, frame 4 is the blur at 12. | largest difference 2.0e-7 | yes |
| FX-SELBLUR-009 frame 4: Blur keyed from 0 at frame 0 to 12 at frame 4, linear: frame 0 is FX-SELBLUR-002, frame 2 is FX-SELBLUR-001, frame 4 is the blur at 12. | largest difference 2.1e-7 | yes |
| FX-SELBLUR-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-010 frame 0: FX-SELBLUR-001 moved two pixels right: the same softened drawing, moved; the blur is done on the drawing's own pixels before it is moved. | largest difference 2.0e-7 | yes |
| FX-SELBLUR-010 frame 3: FX-SELBLUR-001 moved two pixels right: the same softened drawing, moved; the blur is done on the drawing's own pixels before it is moved. | largest difference 2.0e-7 | yes |
| FX-SELBLUR-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-011 frame 0: Blur 5.5: a blur is a whole number of pixels, and a half rounds up, so this is FX-SELBLUR-001. | largest difference 2.0e-7 | yes |
| FX-SELBLUR-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-012 frame 0: Skin above a sloping edge, shadow below, blur 12: the edge goes soft all along its length. | largest difference 2.2e-7 | yes |
| FX-SELBLUR-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-013 frame 0: The same at blur 64, where all five passes act. | largest difference 2.2e-7 | yes |
| FX-SELBLUR-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-014 frame 0: No colours chosen: nothing changes. An effect just added has none. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-015 frame 0: FX-SELBLUR-001 with its colours written in capitals: the same. | largest difference 2.0e-7 | yes |
| FX-SELBLUR-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-020 frame 0: Blur 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-020 frame 4: Blur 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SELBLUR-021 frame 0: Blur -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-021 frame 4: Blur -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SELBLUR-022 frame 0: Blur keyed to 250 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-022 frame 4: Blur keyed to 250 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SELBLUR-023 frame 0: Nine colours, one more than eight. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-023 frame 4: Nine colours, one more than eight. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SELBLUR-024 frame 0: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-024 frame 4: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_selblur_001.json opened and saved holds what it held, out-of-range values and keys included | the same | yes |
| fx_selblur_009.json opened and saved holds what it held, out-of-range values and keys included | the same | yes |
| fx_selblur_020.json opened and saved holds what it held, out-of-range values and keys included | the same | yes |
| fx_selblur_023.json opened and saved holds what it held, out-of-range values and keys included | the same | yes |
| fx_selblur_024.json opened and saved holds what it held, out-of-range values and keys included | the same | yes |
| fx_selblur_015.json, written in capitals, is saved in small letters (D-87) | ["#f6d6be","#dba08e"] | yes |
| a file with no `colors` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with `colors` that is not a list is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a colour that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| blur 201 is refused with a sentence, and nothing changes | Selective colour blur's blur runs from 0 to 200, and this is 201. | yes |
| blur -1 is refused with a sentence, and nothing changes | Selective colour blur's blur runs from 0 to 200, and this is -1. | yes |
| nine colours is refused with a sentence, and nothing changes | Selective colour blur takes up to eight colours, and this has 9. | yes |
| the colour "#12345" is refused with a sentence, and nothing changes | A chosen colour is written # and six hexadecimal digits, such as #f6d6be, and this is "#12345". | yes |
| blur keyed to 250 is refused with a sentence, and nothing changes | Selective colour blur's blur runs from 0 to 200, and this is 250. | yes |
| blur 200 with eight colours, the ends of the ranges, are taken | taken | yes |
| blur 0 with no colour, the ends of the ranges, are taken | taken | yes |
| undo twice: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_selblur_005.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_selblur_013.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## The packaged build carries F's Plugins' licence

| Check | The build's answer | Matches |
| --- | --- | --- |
| `tools/package.ps1` puts `docs/third_party/F-s-PluginsProjects-LICENSE.txt` in the package | it does | yes |
| the port and the licence file carry F's Plugins' copyright notice | both | yes |

## Result

69 of 69 checks pass.
