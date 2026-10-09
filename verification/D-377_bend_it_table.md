# D-377: Bend It

B-256, after CycoreFX's CC Bend It: the strip of the layer along a bar from Start to End curled into an arc that turns Bend degrees over the bar's length, as a bar of rubber bends; before the Start the drawing can be left out, stay, carry the bend on or mirror it, and past the End it can be left out or run on straight. The formulas are this program's own. Every expected pixel is `Fixtures/bend_it/expected_bend_it.json`, written by `tools/bend_it_reference.py` before this code existed and printed in document 25 as FX-BENDIT-001 to 023. Tolerance 2e-5.

## FX-BENDIT-001 to 023 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BENDIT-001 frame 0: The settings as they start: Bend 45, the bar standing up from the middle of the bottom edge to the middle of the top, None, Legal. The drawing leans and curves to the right as it rises, its top an eighth of a turn over; the left part of the top rows, carried past the edge, is cut. | largest difference 2.5e-7 | yes |
| FX-BENDIT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-002 frame 0: Bend 0: the straight bar, the whole drawing on it: untouched. | largest difference 1.9e-7 | yes |
| FX-BENDIT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-003 frame 0: Bend 90: a quarter turn, the top of the bar lying flat to the right, the circle's centre 6.37 pixels right of the bottom middle. | largest difference 2.5e-7 | yes |
| FX-BENDIT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-004 frame 0: Bend -90: the same quarter turn to the left, the picture FX-BENDIT-003's turned over left to right about the bar, as far as the drawing, not quite symmetric about its middle, allows. | largest difference 2.5e-7 | yes |
| FX-BENDIT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-005 frame 0: Bend 360: the bar bent into a whole circle 1.59 pixels round its centre, so the stripes' two ends meet; pixels the circle's sheets cross twice show the one farther along the bar over the other. | largest difference 2.8e-7 | yes |
| FX-BENDIT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-006 frame 0: Start 50, 80 and End 50, 20, a bar from row 8 to row 2, Bend 60, None, Legal: what lies before the Start (rows 8 and 9) and past the End (rows 0 and 1) is not drawn; the six rows between bend. | largest difference 2.5e-7 | yes |
| FX-BENDIT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-007 frame 0: The same with Render Prestart Static: rows 8 and 9, before the Start, drawn unbent, under the bent part where it swings over them. | largest difference 2.5e-7 | yes |
| FX-BENDIT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-008 frame 0: The same with Render Prestart Bend: the bend carried on back past the Start the other way round the same circle, so rows 8 and 9 curve away to the left below it. | largest difference 2.5e-7 | yes |
| FX-BENDIT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-009 frame 0: The same with Render Prestart Mirror: the bar's own Start to End stretch bent back from the Start as Bend bends it, mirrored, so the rows just above the Start reappear below it. | largest difference 2.5e-7 | yes |
| FX-BENDIT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-010 frame 0: FX-BENDIT-006 with Distort Extended: rows 0 and 1, past the End, laid straight on along the bent bar's last direction, 60 degrees over. | largest difference 2.5e-7 | yes |
| FX-BENDIT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-011 frame 0: Start 0, 50 and End 100, 50, the bar lying across, Bend 90: its right half curls down, n pointing down from a bar that runs to the right; rows above and below the bar bend with it. | largest difference 2.5e-7 | yes |
| FX-BENDIT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-012 frame 0: Start and End the same point, 50, 50: no bar, the drawing untouched. | largest difference 1.9e-7 | yes |
| FX-BENDIT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-013 frame 0: Bend keyed from 0 at frame 0 to 90 at frame 4, linear: frame 0 the drawing, frame 2 FX-BENDIT-001 and frame 4 FX-BENDIT-003. | largest difference 1.9e-7 | yes |
| FX-BENDIT-013 frame 2: Bend keyed from 0 at frame 0 to 90 at frame 4, linear: frame 0 the drawing, frame 2 FX-BENDIT-001 and frame 4 FX-BENDIT-003. | largest difference 2.5e-7 | yes |
| FX-BENDIT-013 frame 4: Bend keyed from 0 at frame 0 to 90 at frame 4, linear: frame 0 the drawing, frame 2 FX-BENDIT-001 and frame 4 FX-BENDIT-003. | largest difference 2.5e-7 | yes |
| FX-BENDIT-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-014 frame 0: Bend 60, End keyed from 50, 0 at frame 0 to 50, 50 at frame 4: the bar shortens, the same turn in less length, a tighter curve; with Legal the rows past the End go. | largest difference 2.5e-7 | yes |
| FX-BENDIT-014 frame 2: Bend 60, End keyed from 50, 0 at frame 0 to 50, 50 at frame 4: the bar shortens, the same turn in less length, a tighter curve; with Legal the rows past the End go. | largest difference 2.5e-7 | yes |
| FX-BENDIT-014 frame 4: Bend 60, End keyed from 50, 0 at frame 0 to 50, 50 at frame 4: the bar shortens, the same turn in less length, a tighter curve; with Legal the rows past the End go. | largest difference 2.5e-7 | yes |
| FX-BENDIT-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-015 frame 0: FX-BENDIT-003 moved three pixels right: the same, moved; the bend is worked in the drawing's own space and moves with it, nothing grows, and the three columns left of the drawing stay empty. | largest difference 2.5e-7 | yes |
| FX-BENDIT-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-016 frame 0: Bend eased from 0 at frame 0 to 360 at frame 4 on a curve that overshoots: at frame 2 it would pass 360, is held at 360, and is FX-BENDIT-005, as frame 4 is. | largest difference 1.9e-7 | yes |
| FX-BENDIT-016 frame 2: Bend eased from 0 at frame 0 to 360 at frame 4 on a curve that overshoots: at frame 2 it would pass 360, is held at 360, and is FX-BENDIT-005, as frame 4 is. | largest difference 2.8e-7 | yes |
| FX-BENDIT-016 frame 4: Bend eased from 0 at frame 0 to 360 at frame 4 on a curve that overshoots: at frame 2 it would pass 360, is held at 360, and is FX-BENDIT-005, as frame 4 is. | largest difference 2.8e-7 | yes |
| FX-BENDIT-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-017 frame 0: Bend -150 on the short bar with Static and Extended: the bar curls back on itself to the left, past the Start row, over the static rows, and runs on straight from the End. | largest difference 3.1e-7 | yes |
| FX-BENDIT-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-018 frame 0: Bend 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDIT-018 frame 4: Bend 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDIT-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BENDIT-019 frame 0: Bend -361, below -360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDIT-019 frame 4: Bend -361, below -360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDIT-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BENDIT-020 frame 0: A Render Prestart written "Bend", with a capital. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDIT-020 frame 4: A Render Prestart written "Bend", with a capital. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDIT-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BENDIT-021 frame 0: A Distort written "extend". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDIT-021 frame 4: A Distort written "extend". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDIT-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BENDIT-022 frame 0: Start 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDIT-022 frame 4: Start 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDIT-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BENDIT-023 frame 0: Bend keyed to 400 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDIT-023 frame 4: Bend keyed to 400 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BENDIT-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bendit_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bendit_017.json is saved with its bend, start, end, render prestart and distort | {"bend":-150,"distort":"extended","end":[50,20],"render_prestart":"static","start":[50,80]} | yes |
| fx_bendit_018.json is refused in a sentence | Bend It's bend runs from -360 to 360, and this is 361. | yes |
| fx_bendit_019.json is refused in a sentence | Bend It's bend runs from -360 to 360, and this is -361. | yes |
| fx_bendit_020.json is refused in a sentence | Bend It's render prestart is none, static, bend or mirror, and this is "Bend". | yes |
| fx_bendit_021.json is refused in a sentence | Bend It's distort is legal or extended, and this is "extend". | yes |
| fx_bendit_022.json is refused in a sentence | Bend It's start runs from -1000 to 1000, and this is 1001. | yes |
| a file with a Bend It with no `bend` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Bend It whose start is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| bend 360.5 is refused with a sentence, and nothing changes | Bend It's bend runs from -360 to 360, and this is 360.5. | yes |
| start 50, 1000.5 is refused with a sentence, and nothing changes | Bend It's start runs from -1000 to 1000, and this is 1000.5. | yes |
| render prestart "Mirror", written with a capital is refused with a sentence, and nothing changes | Bend It's render prestart is none, static, bend or mirror, and this is "Mirror". | yes |
| distort "extend" is refused with a sentence, and nothing changes | Bend It's distort is legal or extended, and this is "extend". | yes |
| bend keyed to 400 is refused with a sentence, and nothing changes | Bend It's bend runs from -360 to 360, and this is 400. | yes |
| bend -90 from 20, 80 to 70, 10, mirror, extended, is taken | taken | yes |
| bend keyed from 0 to 90 is taken | taken | yes |
| end keyed from 50, 0 to 50, 50 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bendit_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bendit_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bendit_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bendit_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bendit_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bendit_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bendit_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bendit_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bendit_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bendit_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bendit_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bendit_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bendit_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bendit_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bendit_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bendit_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bendit_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bendit_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bendit_012.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bendit_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bendit_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bendit_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bendit_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bendit_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bendit_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bendit_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bendit_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bendit_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bendit_022.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bendit_023.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Bend It as it starts (45, up the middle): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2004692 pixels changed | yes |
| the reference shot, Bend It as it starts (45, up the middle) on three layers, frame 0, Full | largest difference 1 of 255, 922 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It as it starts (45, up the middle) on three layers, frame 100, Full | largest difference 1 of 255, 837 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It as it starts (45, up the middle) on three layers, frame 239, Full | largest difference 1 of 255, 979 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It as it starts (45, up the middle) on three layers, frame 0, Draft | largest difference 1 of 255, 62 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It as it starts (45, up the middle) on three layers, frame 100, Draft | largest difference 1 of 255, 70 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It as it starts (45, up the middle) on three layers, frame 239, Draft | largest difference 1 of 255, 66 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It -120 from 20, 90 to 80, 10, bend before, extended: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2044464 pixels changed | yes |
| the reference shot, Bend It -120 from 20, 90 to 80, 10, bend before, extended on three layers, frame 0, Full | largest difference 1 of 255, 3787 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It -120 from 20, 90 to 80, 10, bend before, extended on three layers, frame 100, Full | largest difference 1 of 255, 721 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It -120 from 20, 90 to 80, 10, bend before, extended on three layers, frame 239, Full | largest difference 1 of 255, 3539 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It -120 from 20, 90 to 80, 10, bend before, extended on three layers, frame 0, Draft | largest difference 1 of 255, 160 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It -120 from 20, 90 to 80, 10, bend before, extended on three layers, frame 100, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It -120 from 20, 90 to 80, 10, bend before, extended on three layers, frame 239, Draft | largest difference 1 of 255, 143 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It 300 across the middle, mirror: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2039106 pixels changed | yes |
| the reference shot, Bend It 300 across the middle, mirror on three layers, frame 0, Full | largest difference 1 of 255, 108 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It 300 across the middle, mirror on three layers, frame 100, Full | largest difference 1 of 255, 262 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It 300 across the middle, mirror on three layers, frame 239, Full | largest difference 1 of 255, 213 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It 300 across the middle, mirror on three layers, frame 0, Draft | largest difference 1 of 255, 20 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It 300 across the middle, mirror on three layers, frame 100, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It 300 across the middle, mirror on three layers, frame 239, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It 75 on the upper half, static: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1193294 pixels changed | yes |
| the reference shot, Bend It 75 on the upper half, static on three layers, frame 0, Full | largest difference 1 of 255, 2039 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It 75 on the upper half, static on three layers, frame 100, Full | largest difference 1 of 255, 1079 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It 75 on the upper half, static on three layers, frame 239, Full | largest difference 1 of 255, 1233 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It 75 on the upper half, static on three layers, frame 0, Draft | largest difference 1 of 255, 65 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It 75 on the upper half, static on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bend It 75 on the upper half, static on three layers, frame 239, Draft | largest difference 1 of 255, 37 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-377 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts (45 up the middle): the street leans and curls to the right as it rises; draws cleanly | [], 99450 pixels changed | yes |
| 3_bend_0.png, bend 0: the straight bar, nothing changes; draws cleanly | [], 0 pixels changed | yes |
| 4_curl_180_static.png, 180 from 70 per cent up to 10, static: the upper street rolls over into a half circle, the lower part stays; draws cleanly | [], 109903 pixels changed | yes |
| 5_banner_across_mirror.png, -90 from the middle to the right edge, mirror: the street curls up at both sides like a banner; draws cleanly | [], 109210 pixels changed | yes |

## Result

161 of 161 checks pass.
