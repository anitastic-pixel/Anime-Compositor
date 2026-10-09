# D-388: Page Turn

B-267, after CycoreFX's CC Page Turn: a corner (or, in Classic, a straight fold at a chosen angle) turned over towards Fold Position, rolling round a cylinder Fold Radius pixels across, the back drawn as the paper colour or a Back Page layer at Back Opacity, shaded by Light Direction; Render picks the front, the back or both. The formulas are this program's own. Every expected pixel is `Fixtures/page_turn/expected_page_turn.json`, written by `tools/page_turn_reference.py` before this code existed and printed in document 25 as FX-PAGETURN-001 to 027. Tolerance 2e-5.

## FX-PAGETURN-001 to 027 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PAGETURN-001 frame 0: The settings as they start: the bottom right corner turned up to three quarters of the way across and down, Fold Radius 30. On a drawing 16 pixels wide a cylinder that size has rolled the whole page up and out of the frame: clear. | largest difference 0.0e0 | yes |
| FX-PAGETURN-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAGETURN-002 frame 0: Classic, the fold line upright down the middle, the page travelling right, Fold Radius 0: the left half turned over flat onto the right, showing its back, the paper colour #f0f0f0, cut to the page's shape; the left half is clear. | largest difference 2.2e-8 | yes |
| FX-PAGETURN-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAGETURN-003 frame 0: The same with Back Opacity 0: the back shows the page's own picture seen through, so the right half is the left half mirrored. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAGETURN-004 frame 0: The same with Back Opacity 100 and the layer itself as the Back Page: the back is the page's picture again, so the frame is FX-PAGETURN-003's. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAGETURN-005 frame 0: Classic down the middle, Fold Radius 2: the fold rolls round a cylinder two pixels across, its back shaded where it curls, the turned part laid down past it. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAGETURN-006 frame 0: FX-PAGETURN-005 with Render Front: only the page's front, no back. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAGETURN-007 frame 0: FX-PAGETURN-005 with Render Back: only the back; laid over FX-PAGETURN-006 it gives FX-PAGETURN-005. | largest difference 2.2e-8 | yes |
| FX-PAGETURN-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAGETURN-008 frame 0: FX-PAGETURN-005 lit from the right, Light Direction 90: the curl's crest faces left, away from the light, and darkens, where lit from the upper left, as it starts, it shows the paper. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAGETURN-009 frame 0: The bottom right corner turned to the middle, Fold Radius 1: the corner laid flat on the middle, its back the paper colour, the lower right of the frame clear. | largest difference 2.1e-7 | yes |
| FX-PAGETURN-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAGETURN-010 frame 0: The top left corner turned to the middle, Fold Radius 1. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAGETURN-011 frame 0: The bottom right corner's Fold Position keyed from (100, 100) at frame 0 to (0, 0) at frame 4, linear, Fold Radius 1: frame 0 the page as it was, then turned ever further, frame 2 FX-PAGETURN-009. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-011 frame 2: The bottom right corner's Fold Position keyed from (100, 100) at frame 0 to (0, 0) at frame 4, linear, Fold Radius 1: frame 0 the page as it was, then turned ever further, frame 2 FX-PAGETURN-009. | largest difference 2.1e-7 | yes |
| FX-PAGETURN-011 frame 4: The bottom right corner's Fold Position keyed from (100, 100) at frame 0 to (0, 0) at frame 4, linear, Fold Radius 1: frame 0 the page as it was, then turned ever further, frame 2 FX-PAGETURN-009. | largest difference 2.3e-7 | yes |
| FX-PAGETURN-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAGETURN-012 frame 0: Classic as it starts, Fold Direction -60, the line through the middle, Fold Radius 2: the lower right part turned up and to the left. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAGETURN-013 frame 0: FX-PAGETURN-002 with Paper Color #2040a0: the back is blue. | largest difference 8.8e-9 | yes |
| FX-PAGETURN-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAGETURN-014 frame 0: FX-PAGETURN-002 with Back Page a layer that is not in the composition, `gone`: the paper colour, as FX-PAGETURN-002, and the warning every frame. | largest difference 2.2e-8 | yes |
| FX-PAGETURN-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-PAGETURN-015 frame 0: FX-PAGETURN-009 with the layer moved three pixels right: the same, moved; nothing grows. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAGETURN-016 frame 0: FX-PAGETURN-002 with Back Opacity keyed from 0 at frame 0 to 100 at frame 4: frame 0 FX-PAGETURN-003, frame 4 FX-PAGETURN-002, frame 2 half way. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-016 frame 2: FX-PAGETURN-002 with Back Opacity keyed from 0 at frame 0 to 100 at frame 4: frame 0 FX-PAGETURN-003, frame 4 FX-PAGETURN-002, frame 2 half way. | largest difference 1.1e-7 | yes |
| FX-PAGETURN-016 frame 4: FX-PAGETURN-002 with Back Opacity keyed from 0 at frame 0 to 100 at frame 4: frame 0 FX-PAGETURN-003, frame 4 FX-PAGETURN-002, frame 2 half way. | largest difference 2.2e-8 | yes |
| FX-PAGETURN-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAGETURN-017 frame 0: Classic, Fold Radius 1, Fold Direction keyed from 90 at frame 0 to 450 at frame 4: the fold line turns a whole turn about the middle, frames 0 and 4 the same. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-017 frame 2: Classic, Fold Radius 1, Fold Direction keyed from 90 at frame 0 to 450 at frame 4: the fold line turns a whole turn about the middle, frames 0 and 4 the same. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-017 frame 4: Classic, Fold Radius 1, Fold Direction keyed from 90 at frame 0 to 450 at frame 4: the fold line turns a whole turn about the middle, frames 0 and 4 the same. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAGETURN-018 frame 0: Classic down the middle, Fold Radius eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots: frame 2 would pass 1000 and is held there, the same as frame 4. | largest difference 2.2e-8 | yes |
| FX-PAGETURN-018 frame 2: Classic down the middle, Fold Radius eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots: frame 2 would pass 1000 and is held there, the same as frame 4. | largest difference 2.0e-7 | yes |
| FX-PAGETURN-018 frame 4: Classic down the middle, Fold Radius eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots: frame 2 would pass 1000 and is held there, the same as frame 4. | largest difference 2.0e-7 | yes |
| FX-PAGETURN-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAGETURN-019 frame 0: Controls "middle", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-019 frame 4: Controls "middle", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAGETURN-020 frame 0: Render "sides", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-020 frame 4: Render "sides", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAGETURN-021 frame 0: Fold Radius 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-021 frame 4: Fold Radius 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAGETURN-022 frame 0: Fold Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-022 frame 4: Fold Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAGETURN-023 frame 0: Back Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-023 frame 4: Back Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAGETURN-024 frame 0: Paper Color "white", not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-024 frame 4: Paper Color "white", not a colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAGETURN-025 frame 0: Back Page written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-025 frame 4: Back Page written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAGETURN-026 frame 0: Fold Position at (50, 1001), past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-026 frame 4: Fold Position at (50, 1001), past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAGETURN-027 frame 0: Fold Direction keyed to 3601 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-027 frame 4: Fold Direction keyed to 3601 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAGETURN-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pageturn_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pageturn_013.json is saved with its controls, fold, light, render, back page, back opacity and paper colour | {"back_opacity":100,"back_page":"","controls":"classic","fold_direction":90,"fold_position":[50,50],"fold_radius":0,"light_direction":-45,"paper_color":"#2040a0","render":"full"} | yes |
| fx_pageturn_019.json is refused in a sentence | Page Turn's controls is classic, top_left, top_right, bottom_left or bottom_right, and this is "middle". | yes |
| fx_pageturn_020.json is refused in a sentence | Page Turn's render is full, front or back, and this is "sides". | yes |
| fx_pageturn_021.json is refused in a sentence | Page Turn's fold radius runs from 0 to 1000, and this is 1001. | yes |
| fx_pageturn_022.json is refused in a sentence | Page Turn's fold radius runs from 0 to 1000, and this is -1. | yes |
| fx_pageturn_023.json is refused in a sentence | Page Turn's back opacity runs from 0 to 100, and this is 101. | yes |
| fx_pageturn_024.json is refused in a sentence | Page Turn's paper colour is written #rrggbb, and this is "white". | yes |
| fx_pageturn_025.json is refused in a sentence | Page Turn's back page is the name of a layer of this composition, and this is 3. | yes |
| fx_pageturn_026.json is refused in a sentence | Page Turn's fold position runs from -1000 to 1000, and this is 1001. | yes |
| a file with a Page Turn with no `fold_radius` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Page Turn whose fold position is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| fold radius 1000.5 is refused with a sentence, and nothing changes | Page Turn's fold radius runs from 0 to 1000, and this is 1000.5. | yes |
| controls "Classic", written with a capital is refused with a sentence, and nothing changes | Page Turn's controls is classic, top_left, top_right, bottom_left or bottom_right, and this is "Classic". | yes |
| render "both" is refused with a sentence, and nothing changes | Page Turn's render is full, front or back, and this is "both". | yes |
| paper colour "#fff" is refused with a sentence, and nothing changes | Page Turn's paper colour is written #rrggbb, and this is "#fff". | yes |
| back opacity keyed to 150 is refused with a sentence, and nothing changes | Page Turn's back opacity runs from 0 to 100, and this is 150. | yes |
| classic at 40, 60, turned -30, radius 12, lit from 60, front only, opacity 70, cream paper is taken | taken | yes |
| as it starts again is taken | taken | yes |
| fold position keyed from 100, 100 to 0, 0 is taken | taken | yes |
| fold radius keyed from 0 to 50 is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pageturn_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pageturn_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pageturn_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pageturn_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pageturn_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pageturn_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pageturn_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_pageturn_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_pageturn_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_pageturn_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_pageturn_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_pageturn_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_pageturn_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_pageturn_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_pageturn_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_pageturn_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_pageturn_011.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_pageturn_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_pageturn_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_pageturn_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_pageturn_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_pageturn_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_pageturn_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_pageturn_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_pageturn_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_pageturn_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_pageturn_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_pageturn_022.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_pageturn_023.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_pageturn_024.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_pageturn_025.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_pageturn_026.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_pageturn_027.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Page Turn as it starts (bottom right to 75, 75, radius 30): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 158000 pixels changed | yes |
| the reference shot, Page Turn as it starts (bottom right to 75, 75, radius 30) on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn as it starts (bottom right to 75, 75, radius 30) on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn as it starts (bottom right to 75, 75, radius 30) on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn as it starts (bottom right to 75, 75, radius 30) on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn as it starts (bottom right to 75, 75, radius 30) on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn as it starts (bottom right to 75, 75, radius 30) on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn classic down the middle, radius 80, lit from the right: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1734724 pixels changed | yes |
| the reference shot, Page Turn classic down the middle, radius 80, lit from the right on three layers, frame 0, Full | largest difference 1 of 255, 802 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn classic down the middle, radius 80, lit from the right on three layers, frame 100, Full | largest difference 1 of 255, 401 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn classic down the middle, radius 80, lit from the right on three layers, frame 239, Full | largest difference 1 of 255, 802 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn classic down the middle, radius 80, lit from the right on three layers, frame 0, Draft | largest difference 1 of 255, 100 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn classic down the middle, radius 80, lit from the right on three layers, frame 100, Draft | largest difference 1 of 255, 100 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn classic down the middle, radius 80, lit from the right on three layers, frame 239, Draft | largest difference 1 of 255, 100 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn the top left corner to 40, 30, radius 5, blue paper at 60: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 455097 pixels changed | yes |
| the reference shot, Page Turn the top left corner to 40, 30, radius 5, blue paper at 60 on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn the top left corner to 40, 30, radius 5, blue paper at 60 on three layers, frame 100, Full | largest difference 1 of 255, 1365 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn the top left corner to 40, 30, radius 5, blue paper at 60 on three layers, frame 239, Full | largest difference 1 of 255, 1777 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn the top left corner to 40, 30, radius 5, blue paper at 60 on three layers, frame 0, Draft | largest difference 1 of 255, 131 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn the top left corner to 40, 30, radius 5, blue paper at 60 on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn the top left corner to 40, 30, radius 5, blue paper at 60 on three layers, frame 239, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn classic turned -20, radius 200, back only: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073521 pixels changed | yes |
| the reference shot, Page Turn classic turned -20, radius 200, back only on three layers, frame 0, Full | largest difference 1 of 255, 4 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn classic turned -20, radius 200, back only on three layers, frame 100, Full | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn classic turned -20, radius 200, back only on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn classic turned -20, radius 200, back only on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn classic turned -20, radius 200, back only on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn classic turned -20, radius 200, back only on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn the bottom right corner to the middle, its back the fourth layer: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 685828 pixels changed | yes |
| the reference shot, Page Turn the bottom right corner to the middle, its back the fourth layer on three layers, frame 0, Full | largest difference 1 of 255, 1536 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn the bottom right corner to the middle, its back the fourth layer on three layers, frame 100, Full | largest difference 1 of 255, 1420 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn the bottom right corner to the middle, its back the fourth layer on three layers, frame 239, Full | largest difference 1 of 255, 1567 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn the bottom right corner to the middle, its back the fourth layer on three layers, frame 0, Draft | largest difference 1 of 255, 42 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn the bottom right corner to the middle, its back the fourth layer on three layers, frame 100, Draft | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Page Turn the bottom right corner to the middle, its back the fourth layer on three layers, frame 239, Draft | largest difference 1 of 255, 41 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-388 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: the bottom right corner of the street curling up and over, the paper's back showing, the corner left clear; draws cleanly | [], 12830 pixels changed | yes |
| 3_half_flat.png, classic down the middle, radius 0: the left half folded flat over the right, its back the pale paper; the left half clear; draws cleanly | [], 129600 pixels changed | yes |
| 4_corner_blue.png, the bottom right corner turned to the middle on blue paper, lit from the right; draws cleanly | [], 36418 pixels changed | yes |
| 5_seen_through.png, classic, back opacity 0: the turned part shows the street through itself, mirrored; draws cleanly | [], 113591 pixels changed | yes |

## Result

192 of 192 checks pass.
