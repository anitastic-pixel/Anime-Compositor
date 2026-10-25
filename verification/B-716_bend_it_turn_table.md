# B-716: Bend It turns 1.8 degrees for each unit of Bend (D-836)

Seven tutorials from six creators measure After Effects' CC Bend It turning 1.72 to 1.90 degrees for each unit of Bend (median 1.82): Bend 100 is half a turn. Ours turned a degree a unit. The bar from Start to End is now bent through Bend x 1.8 degrees, on the processor and the card. The turn is read off a white 20 by 200 bar standing up a 400 by 400 precomposition, Start at its foot and End at its top: the turn whose bent bar has its covering's centre where the drawn bar has it. The fixtures' numbers are the reference's, whose cases' Bend D-836 divided by 1.8 so their pixels stay.

## The bar's turn, in degrees, within 1 degree of Bend x 1.8

| Check | The build's answer | Matches |
| --- | --- | --- |
| Bend 25: 45 degrees (25 degrees before D-836) | 45.00 degrees, 1.800 a unit | yes |
| Bend 50: 90 degrees (50 degrees before D-836) | 89.90 degrees, 1.798 a unit | yes |
| Bend 100: 180 degrees (100 degrees before D-836) | 180.00 degrees, 1.800 a unit | yes |
| Bend -50: -90 degrees (-50 degrees before D-836) | -89.90 degrees, 1.798 a unit | yes |
| Bend 150: 270 degrees (150 degrees before D-836) | 270.00 degrees, 1.800 a unit | yes |

## The fixtures, at the reference's numbers (tolerance 2e-5)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BENDIT-001 frame 0: The settings as they start: Bend 25 (45 degrees), the bar standing up from the middle of the bottom edge to the middle of the top, None, Legal. The drawing leans and curves to the right as it rises, its top an eighth of a turn over; the left part of the top rows, carried past the edge, is cut. | largest difference 2.5e-7 | yes |
| FX-BENDIT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-002 frame 0: Bend 0: the straight bar, the whole drawing on it: untouched. | largest difference 1.9e-7 | yes |
| FX-BENDIT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-003 frame 0: Bend 50: a quarter turn, the top of the bar lying flat to the right, the circle's centre 6.37 pixels right of the bottom middle. | largest difference 2.5e-7 | yes |
| FX-BENDIT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-004 frame 0: Bend -50: the same quarter turn to the left, the picture FX-BENDIT-003's turned over left to right about the bar, as far as the drawing, not quite symmetric about its middle, allows. | largest difference 2.5e-7 | yes |
| FX-BENDIT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-005 frame 0: Bend 200: the bar bent into a whole circle 1.59 pixels round its centre, so the stripes' two ends meet; pixels the circle's sheets cross twice show the one farther along the bar over the other. | largest difference 2.8e-7 | yes |
| FX-BENDIT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-006 frame 0: Start 50, 80 and End 50, 20, a bar from row 8 to row 2, Bend 33.3 (60 degrees), None, Legal: what lies before the Start (rows 8 and 9) and past the End (rows 0 and 1) is not drawn; the six rows between bend. | largest difference 2.5e-7 | yes |
| FX-BENDIT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-007 frame 0: The same with Render Prestart Static: rows 8 and 9, before the Start, drawn unbent, under the bent part where it swings over them. | largest difference 2.5e-7 | yes |
| FX-BENDIT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-008 frame 0: The same with Render Prestart Bend: the bend carried on back past the Start the other way round the same circle, so rows 8 and 9 curve away to the left below it. | largest difference 2.5e-7 | yes |
| FX-BENDIT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-009 frame 0: The same with Render Prestart Mirror: the bar's own Start to End stretch bent back from the Start as Bend bends it, mirrored, so the rows just above the Start reappear below it. | largest difference 2.5e-7 | yes |
| FX-BENDIT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-010 frame 0: FX-BENDIT-006 with Distort Extended: rows 0 and 1, past the End, laid straight on along the bent bar's last direction, 60 degrees over. | largest difference 2.5e-7 | yes |
| FX-BENDIT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-011 frame 0: Start 0, 50 and End 100, 50, the bar lying across, Bend 50: its right half curls down, n pointing down from a bar that runs to the right; rows above and below the bar bend with it. | largest difference 2.5e-7 | yes |
| FX-BENDIT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-012 frame 0: Start and End the same point, 50, 50: no bar, the drawing untouched. | largest difference 1.9e-7 | yes |
| FX-BENDIT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-013 frame 0: Bend keyed from 0 at frame 0 to 50 at frame 4, linear: frame 0 the drawing, frame 2 FX-BENDIT-001 and frame 4 FX-BENDIT-003. | largest difference 1.9e-7 | yes |
| FX-BENDIT-013 frame 2: Bend keyed from 0 at frame 0 to 50 at frame 4, linear: frame 0 the drawing, frame 2 FX-BENDIT-001 and frame 4 FX-BENDIT-003. | largest difference 2.5e-7 | yes |
| FX-BENDIT-013 frame 4: Bend keyed from 0 at frame 0 to 50 at frame 4, linear: frame 0 the drawing, frame 2 FX-BENDIT-001 and frame 4 FX-BENDIT-003. | largest difference 2.5e-7 | yes |
| FX-BENDIT-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-014 frame 0: Bend 33.3, End keyed from 50, 0 at frame 0 to 50, 50 at frame 4: the bar shortens, the same turn in less length, a tighter curve; with Legal the rows past the End go. | largest difference 2.5e-7 | yes |
| FX-BENDIT-014 frame 2: Bend 33.3, End keyed from 50, 0 at frame 0 to 50, 50 at frame 4: the bar shortens, the same turn in less length, a tighter curve; with Legal the rows past the End go. | largest difference 2.5e-7 | yes |
| FX-BENDIT-014 frame 4: Bend 33.3, End keyed from 50, 0 at frame 0 to 50, 50 at frame 4: the bar shortens, the same turn in less length, a tighter curve; with Legal the rows past the End go. | largest difference 2.5e-7 | yes |
| FX-BENDIT-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-015 frame 0: FX-BENDIT-003 moved three pixels right: the same, moved; the bend is worked in the drawing's own space and moves with it, nothing grows, and the three columns left of the drawing stay empty. | largest difference 2.5e-7 | yes |
| FX-BENDIT-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-016 frame 0: Bend eased from 0 at frame 0 to 360 at frame 4 on a curve that overshoots: at frame 2 it would pass 360, is held at 360, a turn of 648 degrees, one and four fifths of a whole circle, as at frame 4. | largest difference 1.9e-7 | yes |
| FX-BENDIT-016 frame 2: Bend eased from 0 at frame 0 to 360 at frame 4 on a curve that overshoots: at frame 2 it would pass 360, is held at 360, a turn of 648 degrees, one and four fifths of a whole circle, as at frame 4. | largest difference 2.5e-7 | yes |
| FX-BENDIT-016 frame 4: Bend eased from 0 at frame 0 to 360 at frame 4 on a curve that overshoots: at frame 2 it would pass 360, is held at 360, a turn of 648 degrees, one and four fifths of a whole circle, as at frame 4. | largest difference 2.5e-7 | yes |
| FX-BENDIT-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BENDIT-017 frame 0: Bend -83.3 (-150 degrees) on the short bar with Static and Extended: the bar curls back on itself to the left, past the Start row, over the static rows, and runs on straight from the End. | largest difference 3.1e-7 | yes |
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

## An old project

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bendit_003.json as saved before D-836 (Bend 90, its file at c4b0696c) opens with no warning | [] | yes |
| and saves back holding what it held: Bend still 90, nothing added | the same | yes |
| and bends further (162 degrees), not FX-BENDIT-003's quarter turn, with no warning | largest difference from FX-BENDIT-003 1.0e0; warnings [] | yes |
| Bend 90 on the bar turns it 162 degrees, within 1 | 161.95 degrees | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| The bar at Bend 25, frame 0 at Full and Draft | largest difference 1 of 255; refused by the card: false; the same warnings: true | yes |
| The bar at Bend 100, frame 0 at Full and Draft | largest difference 1 of 255; refused by the card: false; the same warnings: true | yes |
| The bar at Bend -50, frame 0 at Full and Draft | largest difference 1 of 255; refused by the card: false; the same warnings: true | yes |
| The bar at Bend 150, frame 0 at Full and Draft | largest difference 1 of 255; refused by the card: false; the same warnings: true | yes |
| fx_bendit_001.json, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| fx_bendit_005.json, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| fx_bendit_008.json, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| fx_bendit_010.json, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| fx_bendit_016.json, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| fx_bendit_017.json, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |

## Result

77 of 77 checks pass.
