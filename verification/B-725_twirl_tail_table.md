# B-725: Twirl's fall-off, fitted on After Effects' own

D-845. Twirl turns each pixel by Angle times (e^(-2.25 u^2) - e^(-2.25)) / (1 - e^(-2.25)), u its distance over the reach, in place of D-744's smoothstep: the whole angle at the centre, none at the edge, and about 0.04 of the angle more from 0.6 to 0.9 of the reach, as seven After Effects twirls from six tutorial makers measured. Every expected pixel is `Fixtures/twirl/expected_twirl.json`, written by `tools/twirl_reference.py` before this code and printed in document 25 as FX-TWIRL-001 to 027. The build's frame is compared sample by sample against the catalogue's tolerance of 2e-5, and the card's against the processor's within 1 level of 255.

## FX-TWIRL-001 to 027 (document 25), on the processor

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TWIRL-001 frame 0: The settings as they start: angle 90, radius 50, about the middle, the point (8, 5). The whole drawing lies inside the circle, so every pixel is turned clockwise, by about 90 degrees beside the centre and by about 83 at the corners: the two lines bend into a spiral about the middle, and column 15 and rows 2 to 9 of column 0, which take their picture from past the drawing's top and bottom edges, are empty. Pixels deep in the skin take skin and look unchanged. | largest difference 2.5e-7 | yes |
| FX-TWIRL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-002 frame 0: Angle -90: the same twirl the other way, counter-clockwise; each pixel takes its picture from the point FX-TWIRL-001 takes it from, mirrored across the line from the centre to the pixel. | largest difference 2.5e-7 | yes |
| FX-TWIRL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-003 frame 0: Angle 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-TWIRL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-004 frame 0: Radius 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-TWIRL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-005 frame 0: Radius 4: only the pixels whose centres are less than 4 pixels from the middle are turned, the crossing of the two lines swirled; every pixel 4 or more from it is kept exactly. | largest difference 2.5e-7 | yes |
| FX-TWIRL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-006 frame 0: Angle 360, radius 8: a whole turn at the very centre, less further out, so the lines wind round the middle; the corners, 8 or more from it, are kept exactly. | largest difference 2.9e-7 | yes |
| FX-TWIRL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-007 frame 0: Angle 3600, the most: ten turns at the centre, the picture wound so tightly that the middle pixels sample it almost at random. | largest difference 2.5e-7 | yes |
| FX-TWIRL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-008 frame 0: Radius 10000: the falloff is so slow that every pixel is turned by within a fifth of a degree of 90, nearly a plain quarter turn about the middle: the drawing stands on end in columns 3 to 12, the line down column 8 now runs along row 5 and the line along row 5 runs down column 7, and the three columns on either side, turned in from past the drawing's top and bottom, are empty or nearly. | largest difference 2.5e-7 | yes |
| FX-TWIRL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-009 frame 0: Centre 0, 0, the top left corner, radius 6: only the pixels less than 6 from that corner are turned, the soft edge among them; the rest is kept exactly. | largest difference 2.5e-7 | yes |
| FX-TWIRL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-010 frame 0: Angle keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 the drawing, frame 2 FX-TWIRL-001, and frame 4 angle 180. | largest difference 1.9e-7 | yes |
| FX-TWIRL-010 frame 2: Angle keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 the drawing, frame 2 FX-TWIRL-001, and frame 4 angle 180. | largest difference 2.5e-7 | yes |
| FX-TWIRL-010 frame 4: Angle keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 the drawing, frame 2 FX-TWIRL-001, and frame 4 angle 180. | largest difference 2.5e-7 | yes |
| FX-TWIRL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-011 frame 0: Radius keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 2 FX-TWIRL-001, and frame 4 radius 100, which turns every pixel further than radius 50 does. | largest difference 1.9e-7 | yes |
| FX-TWIRL-011 frame 2: Radius keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 2 FX-TWIRL-001, and frame 4 radius 100, which turns every pixel further than radius 50 does. | largest difference 2.5e-7 | yes |
| FX-TWIRL-011 frame 4: Radius keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the drawing, frame 2 FX-TWIRL-001, and frame 4 radius 100, which turns every pixel further than radius 50 does. | largest difference 2.5e-7 | yes |
| FX-TWIRL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-012 frame 0: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-TWIRL-001, frame 2 turns about 25, 25, and frame 4 about the top left corner. | largest difference 2.5e-7 | yes |
| FX-TWIRL-012 frame 2: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-TWIRL-001, frame 2 turns about 25, 25, and frame 4 about the top left corner. | largest difference 2.5e-7 | yes |
| FX-TWIRL-012 frame 4: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-TWIRL-001, frame 2 turns about 25, 25, and frame 4 about the top left corner. | largest difference 1.5e-7 | yes |
| FX-TWIRL-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-013 frame 0: Angle eased from 0 at frame 0 to 3600 at frame 4 on a curve that overshoots: at frame 2 it would pass 3600, is held at 3600, and is FX-TWIRL-007; frame 0 is the drawing. | largest difference 1.9e-7 | yes |
| FX-TWIRL-013 frame 2: Angle eased from 0 at frame 0 to 3600 at frame 4 on a curve that overshoots: at frame 2 it would pass 3600, is held at 3600, and is FX-TWIRL-007; frame 0 is the drawing. | largest difference 2.5e-7 | yes |
| FX-TWIRL-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-014 frame 0: FX-TWIRL-001 moved three pixels right: the same, moved; the twirl moves with the drawing, and nothing is drawn left of the drawing's edge. | largest difference 2.5e-7 | yes |
| FX-TWIRL-014 frame 3: FX-TWIRL-001 moved three pixels right: the same, moved; the twirl moves with the drawing, and nothing is drawn left of the drawing's edge. | largest difference 2.5e-7 | yes |
| FX-TWIRL-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-015 frame 0: A directional blur, direction 90 and length 4, then angle 90 about centre 25, 25: the blur grew the layer two pixels on every side, its grown pixels are turned too, and the centre is still the drawing's own point (4, 2.5), not a point of the grown layer. | largest difference 2.9e-7 | yes |
| FX-TWIRL-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-016 frame 0: Angle 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-016 frame 4: Angle 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIRL-017 frame 0: Angle -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-017 frame 4: Angle -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIRL-018 frame 0: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-018 frame 4: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIRL-019 frame 0: Radius 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-019 frame 4: Radius 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIRL-020 frame 0: Centre 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-020 frame 4: Centre 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIRL-021 frame 0: Radius keyed to 20000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-021 frame 4: Radius keyed to 20000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIRL-022 frame 0: D-744, the settings as a new Twirl starts: angle 90, radius 0, twirl radius 30, After Effects' start, about the middle. The circle is 30 per cent of the drawing's larger side, 16, so 4.8 pixels: the crossing is swirled, a little wider than FX-TWIRL-005's radius 4, and every pixel 4.8 or more from the middle is kept exactly. | largest difference 2.5e-7 | yes |
| FX-TWIRL-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-023 frame 0: Radius 4 with twirl radius 100: radius, in pixels, is above 0, so it is the circle's size and twirl radius is not used; FX-TWIRL-005 exactly. | largest difference 2.5e-7 | yes |
| FX-TWIRL-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-024 frame 0: Radius 0, twirl radius keyed from 0 at frame 0 to 60 at frame 4, linear: frame 0 the drawing, frame 2 FX-TWIRL-022, and frame 4 a circle of 9.6 pixels. | largest difference 1.9e-7 | yes |
| FX-TWIRL-024 frame 2: Radius 0, twirl radius keyed from 0 at frame 0 to 60 at frame 4, linear: frame 0 the drawing, frame 2 FX-TWIRL-022, and frame 4 a circle of 9.6 pixels. | largest difference 2.5e-7 | yes |
| FX-TWIRL-024 frame 4: Radius 0, twirl radius keyed from 0 at frame 0 to 60 at frame 4, linear: frame 0 the drawing, frame 2 FX-TWIRL-022, and frame 4 a circle of 9.6 pixels. | largest difference 2.5e-7 | yes |
| FX-TWIRL-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-025 frame 0: FX-TWIRL-015's directional blur, then radius 0, twirl radius 50 about centre 25, 25: the circle is half the drawing's own larger side, 8 pixels, not half the grown layer's 20. | largest difference 2.6e-7 | yes |
| FX-TWIRL-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TWIRL-026 frame 0: Twirl radius 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-026 frame 4: Twirl radius 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TWIRL-027 frame 0: Twirl radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-027 frame 4: Twirl radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TWIRL-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The same cases on the card

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TWIRL-001, fx_twirl_001.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-002, fx_twirl_002.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-003, fx_twirl_003.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-004, fx_twirl_004.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-005, fx_twirl_005.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-006, fx_twirl_006.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-007, fx_twirl_007.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-008, fx_twirl_008.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-009, fx_twirl_009.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-010, fx_twirl_010.json, frames [0, 2, 4] at Full and Draft | largest difference 0 of 255; on the card in 4 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-011, fx_twirl_011.json, frames [0, 2, 4] at Full and Draft | largest difference 0 of 255; on the card in 4 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-012, fx_twirl_012.json, frames [0, 2, 4] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-013, fx_twirl_013.json, frames [0, 2] at Full and Draft | largest difference 0 of 255; on the card in 2 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-014, fx_twirl_014.json, frames [0, 3] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-015, fx_twirl_015.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-016, fx_twirl_016.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-017, fx_twirl_017.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-018, fx_twirl_018.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-019, fx_twirl_019.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-020, fx_twirl_020.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-021, fx_twirl_021.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-022, fx_twirl_022.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-023, fx_twirl_023.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-024, fx_twirl_024.json, frames [0, 2, 4] at Full and Draft | largest difference 0 of 255; on the card in 4 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-025, fx_twirl_025.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-026, fx_twirl_026.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-TWIRL-027, fx_twirl_027.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_twirl_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_twirl_025.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

101 of 101 checks pass.
