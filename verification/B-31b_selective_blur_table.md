# B-31b: selective colour blur

D-87, accepted by the owner on 2026-09-25. Every expected pixel is `Fixtures/selblur/expected_selblur.json`, written by `tools/selblur_reference.py` before this code existed and printed in document 25 as FX-SELBLUR-001 to 024; D-88's tolerance, accepted the same day, adds 025 to 033. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-SELBLUR-001 to 033 (document 25)

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
| FX-SELBLUR-025 frame 0: Skin painted unevenly, a checkerboard of skin and skin one step off in blue, beside shadow, tolerance 0: the off pixels are not chosen and stop the softening, so the skin stays as painted except the exact skin pixel touching the shadow on every other row; the shadow goes soft. | largest difference 2.3e-7 | yes |
| FX-SELBLUR-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-026 frame 0: The same at tolerance 1: every skin pixel is chosen, and the edge goes soft across the whole row, as in FX-SELBLUR-001. | largest difference 2.0e-7 | yes |
| FX-SELBLUR-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-027 frame 0: FX-SELBLUR-008, the skin chosen one step off, at tolerance 1: the skin is chosen again, and this is FX-SELBLUR-001. | largest difference 2.0e-7 | yes |
| FX-SELBLUR-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-028 frame 0: The same at tolerance 0.5: one step is more than half a step, so the skin is not chosen and nothing changes. The tolerance is not rounded. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-029 frame 0: Skin, shadow and highlight with skin and shadow chosen, tolerance 255: every colour that shows is chosen, the highlight too, so this is the three colours chosen at tolerance 0. | largest difference 2.0e-7 | yes |
| FX-SELBLUR-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-030 frame 0: Skin and shadow with a black line between, tolerance 255: the line is chosen too, and goes soft. A large tolerance takes in the lines. | largest difference 2.0e-7 | yes |
| FX-SELBLUR-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-031 frame 0: FX-SELBLUR-027 with the tolerance keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frames 2 and 4 are FX-SELBLUR-001. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-031 frame 2: FX-SELBLUR-027 with the tolerance keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frames 2 and 4 are FX-SELBLUR-001. | largest difference 2.0e-7 | yes |
| FX-SELBLUR-031 frame 4: FX-SELBLUR-027 with the tolerance keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frames 2 and 4 are FX-SELBLUR-001. | largest difference 2.0e-7 | yes |
| FX-SELBLUR-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SELBLUR-032 frame 0: Tolerance 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-032 frame 4: Tolerance 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SELBLUR-033 frame 0: Tolerance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-033 frame 4: Tolerance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SELBLUR-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_selblur_001.json opened and saved holds what it held, out-of-range values and keys included | the same | yes |
| fx_selblur_009.json opened and saved holds what it held, out-of-range values and keys included | the same | yes |
| fx_selblur_020.json opened and saved holds what it held, out-of-range values and keys included | the same | yes |
| fx_selblur_023.json opened and saved holds what it held, out-of-range values and keys included | the same | yes |
| fx_selblur_024.json opened and saved holds what it held, out-of-range values and keys included | the same | yes |
| fx_selblur_026.json opened and saved holds what it held, out-of-range values and keys included | the same | yes |
| fx_selblur_031.json opened and saved holds what it held, out-of-range values and keys included | the same | yes |
| fx_selblur_015.json, written in capitals, is saved in small letters (D-87) | ["#f6d6be","#dba08e"] | yes |
| fx_selblur_001.json, from before D-88, is saved without a tolerance | None | yes |
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
| tolerance 256 is refused with a sentence, and nothing changes | Selective colour blur's tolerance runs from 0 to 255, and this is 256. | yes |
| tolerance -1 is refused with a sentence, and nothing changes | Selective colour blur's tolerance runs from 0 to 255, and this is -1. | yes |
| tolerance keyed to 300 is refused with a sentence, and nothing changes | Selective colour blur's tolerance runs from 0 to 255, and this is 300. | yes |
| blur 200 with eight colours, the ends of the ranges, are taken | taken | yes |
| blur 0 with no colour, the ends of the ranges, are taken | taken | yes |
| tolerance 255, the ends of the ranges, are taken | taken | yes |
| tolerance 0, the ends of the ranges, are taken | taken | yes |
| undo four times: frame 0 is the frame it was | byte-identical | yes |
| fx_selblur_026.json's tolerance 1 set back to 0 is saved without a tolerance (D-88) | None | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_selblur_005.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_selblur_013.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## The draft preview (D-66)

| Check | The build's answer | Matches |
| --- | --- | --- |
| an adjustment layer's blur 12 is blur 3 on the quarter-size draft frame, and its tolerance 20, a distance in colour, stays 20 (D-88) | [(3.0, 20.0)] | yes |

## The packaged build carries F's Plugins' licence

| Check | The build's answer | Matches |
| --- | --- | --- |
| `tools/package.ps1` puts `docs/third_party/F-s-PluginsProjects-LICENSE.txt` in the package | it does | yes |
| the port and the licence file carry F's Plugins' copyright notice | both | yes |

## Result

101 of 101 checks pass.
