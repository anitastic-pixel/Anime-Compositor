# B-88: emboss

D-145, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the twelfth of the third batch. Every expected pixel is `Fixtures/emboss/expected_emboss.json`, written by `tools/emboss_reference.py` before this code existed and printed in document 25 as FX-EMBOSS-001 to 026. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-EMBOSS-001 to 026 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-EMBOSS-001 frame 0: The settings as they start: direction 135, relief 1, contrast 100, mode grey. Every pixel that shows turns a grey, lighter where the picture ahead of it (down and to the right) is brighter than behind it and darker where it is darker: the flat skin and the middle of the square stay a middle grey (0.5 as written). The drawing, brighter than the emptiness around it, stands up: its top row is light, and its bottom row and its soft right-hand column dark. The dark square and the band sink in: their top edges go dark and their bottom edges light, and the square's left edge dark and its right edge light. The soft column is grey at its half covering; the empty pixels stay empty. | largest difference 5.9e-8 | yes |
| FX-EMBOSS-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EMBOSS-002 frame 0: Relief 0: each pixel looks nowhere, so every pixel that shows is the flat middle grey (0.5 as written) at its own covering. | largest difference 3.0e-8 | yes |
| FX-EMBOSS-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EMBOSS-003 frame 0: Relief 0, mode color: the flat relief laid over the colours changes nothing: the drawing, within a rounding step. | largest difference 1.9e-7 | yes |
| FX-EMBOSS-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EMBOSS-004 frame 0: Contrast 0: no difference shows, so it is FX-EMBOSS-002, flat grey. | largest difference 3.0e-8 | yes |
| FX-EMBOSS-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EMBOSS-005 frame 0: Mode color at the start settings: the same relief as FX-EMBOSS-001 laid over the drawing's own colours, each channel as written moved by v - 0.5, so the flat skin and the middle of the square keep their colours and the edges are lit or shaded. | largest difference 2.2e-7 | yes |
| FX-EMBOSS-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EMBOSS-006 frame 0: Direction 90, relief 1: each pixel looks exactly one pixel right and one left, v = 0.5 + y(right) - y(left), so only upright edges show: the band, running level from the drawing's left edge, vanishes into the grey except beside the square, whose left side goes dark and right side light, and the drawing's soft right-hand edge goes dark. The first column's look to the left is held at the drawing's own first column, not read as empty, so it stays the flat grey. | largest difference 5.6e-8 | yes |
| FX-EMBOSS-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EMBOSS-007 frame 0: Direction 270, relief 1: lit from the other side, FX-EMBOSS-006 turned inside out: wherever neither is held at black or white, each pixel's grey is 1 less FX-EMBOSS-006's. | largest difference 5.8e-8 | yes |
| FX-EMBOSS-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EMBOSS-008 frame 0: Direction 0, relief 1: each pixel looks one pixel up and one down, so only level edges show: the square's upright sides and the drawing's soft right-hand edge vanish, the band and the square's top and bottom stand out, the top row, looking up at the empty row, goes dark, and the bottom row, looking down at it, light. | largest difference 3.0e-8 | yes |
| FX-EMBOSS-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EMBOSS-009 frame 0: Contrast 300: each difference three times as strong, the strongest edges held at black or white. | largest difference 1.3e-7 | yes |
| FX-EMBOSS-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EMBOSS-010 frame 0: Contrast 1000: ten times as strong: every edge pixel but the eight faintest is black or white, and the flat parts stay the middle grey. | largest difference 2.1e-7 | yes |
| FX-EMBOSS-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EMBOSS-011 frame 0: Relief 2.5: each pixel looks two and a half pixels ahead and behind, so the ridges are wider and almost nothing stays flat: the skin rows just inside the top and bottom and the whole square now reach an edge. Only the soft column's top pixel is the middle grey, both its looks landing on emptiness. | largest difference 1.1e-7 | yes |
| FX-EMBOSS-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EMBOSS-012 frame 0: Relief 100, direction 270, contrast 50: both looks fall far outside the drawing and are held at its edges, ahead at its first column and behind at its empty last one, so every pixel of a row that shows is one grey, 0.5 + half the first column's luma: lighter in the skin's rows than in the band's. | largest difference 5.5e-8 | yes |
| FX-EMBOSS-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EMBOSS-013 frame 0: Direction keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 is FX-EMBOSS-008, frame 2 is direction 90, FX-EMBOSS-006, and frame 4 direction 180, lit from below. | largest difference 3.0e-8 | yes |
| FX-EMBOSS-013 frame 2: Direction keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 is FX-EMBOSS-008, frame 2 is direction 90, FX-EMBOSS-006, and frame 4 direction 180, lit from below. | largest difference 5.6e-8 | yes |
| FX-EMBOSS-013 frame 4: Direction keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 is FX-EMBOSS-008, frame 2 is direction 90, FX-EMBOSS-006, and frame 4 direction 180, lit from below. | largest difference 3.0e-8 | yes |
| FX-EMBOSS-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EMBOSS-014 frame 0: Relief keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is FX-EMBOSS-002, flat grey, frame 2 is relief 1, FX-EMBOSS-001, and frame 4 relief 2. | largest difference 3.0e-8 | yes |
| FX-EMBOSS-014 frame 2: Relief keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is FX-EMBOSS-002, flat grey, frame 2 is relief 1, FX-EMBOSS-001, and frame 4 relief 2. | largest difference 5.9e-8 | yes |
| FX-EMBOSS-014 frame 4: Relief keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is FX-EMBOSS-002, flat grey, frame 2 is relief 1, FX-EMBOSS-001, and frame 4 relief 2. | largest difference 1.4e-7 | yes |
| FX-EMBOSS-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EMBOSS-015 frame 0: Contrast eased from 100 at frame 0 to 1000 at frame 4 on a curve that overshoots: frame 0 is FX-EMBOSS-001, and at frame 2 it has gone past 1000 and is held there, so frames 2 and 4 are both FX-EMBOSS-010. | largest difference 5.9e-8 | yes |
| FX-EMBOSS-015 frame 2: Contrast eased from 100 at frame 0 to 1000 at frame 4 on a curve that overshoots: frame 0 is FX-EMBOSS-001, and at frame 2 it has gone past 1000 and is held there, so frames 2 and 4 are both FX-EMBOSS-010. | largest difference 2.1e-7 | yes |
| FX-EMBOSS-015 frame 4: Contrast eased from 100 at frame 0 to 1000 at frame 4 on a curve that overshoots: frame 0 is FX-EMBOSS-001, and at frame 2 it has gone past 1000 and is held there, so frames 2 and 4 are both FX-EMBOSS-010. | largest difference 2.1e-7 | yes |
| FX-EMBOSS-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EMBOSS-016 frame 0: Direction 495, one turn past 135: FX-EMBOSS-001. | largest difference 5.9e-8 | yes |
| FX-EMBOSS-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EMBOSS-017 frame 0: Direction -225, the same way as 135: FX-EMBOSS-001. | largest difference 5.9e-8 | yes |
| FX-EMBOSS-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EMBOSS-018 frame 0: FX-EMBOSS-005 moved three pixels right: the relief is worked in the drawing's own space, so it moves with it, and nothing grows: the three columns left of the drawing stay empty. | largest difference 2.1e-7 | yes |
| FX-EMBOSS-018 frame 3: FX-EMBOSS-005 moved three pixels right: the relief is worked in the drawing's own space, so it moves with it, and nothing grows: the three columns left of the drawing stay empty. | largest difference 2.1e-7 | yes |
| FX-EMBOSS-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EMBOSS-019 frame 0: Relief 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EMBOSS-019 frame 4: Relief 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EMBOSS-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EMBOSS-020 frame 0: Relief -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EMBOSS-020 frame 4: Relief -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EMBOSS-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EMBOSS-021 frame 0: Contrast 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EMBOSS-021 frame 4: Contrast 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EMBOSS-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EMBOSS-022 frame 0: Contrast -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EMBOSS-022 frame 4: Contrast -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EMBOSS-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EMBOSS-023 frame 0: Direction 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EMBOSS-023 frame 4: Direction 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EMBOSS-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EMBOSS-024 frame 0: Mode "gray", which is not a choice: the word is "grey". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EMBOSS-024 frame 4: Mode "gray", which is not a choice: the word is "grey". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EMBOSS-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EMBOSS-025 frame 0: Mode "Grey": the word is exact, so a capital is not the choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EMBOSS-025 frame 4: Mode "Grey": the word is exact, so a capital is not the choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EMBOSS-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EMBOSS-026 frame 0: Relief keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EMBOSS-026 frame 4: Relief keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-EMBOSS-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview halves the relief, 3 to 1.5, and nothing else | Emboss { direction: 135.0, relief: 1.5, contrast: 100.0, mode: "grey" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_emboss_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_emboss_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_emboss_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_emboss_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_emboss_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_emboss_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_emboss_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_emboss_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_emboss_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_emboss_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_emboss_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `mode` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a relief that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| relief 101 is refused with a sentence, and nothing changes | Emboss's relief runs from 0 to 100, and this is 101. | yes |
| relief -1 is refused with a sentence, and nothing changes | Emboss's relief runs from 0 to 100, and this is -1. | yes |
| contrast 1001 is refused with a sentence, and nothing changes | Emboss's contrast runs from 0 to 1000, and this is 1001. | yes |
| direction 3601 is refused with a sentence, and nothing changes | Emboss's direction runs from -3600 to 3600, and this is 3601. | yes |
| mode "gray" is refused with a sentence, and nothing changes | Emboss's mode is "grey" or "color", and this is "gray". | yes |
| relief keyed to 150 is refused with a sentence, and nothing changes | Emboss's relief runs from 0 to 100, and this is 150. | yes |
| direction -3600, relief 0 and contrast 0, the bottoms, is taken | taken | yes |
| direction 3600, relief 100 and contrast 1000, the tops, is taken | taken | yes |
| direction keyed from 0 to 180 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_emboss_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_emboss_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_emboss_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

95 of 95 checks pass.
