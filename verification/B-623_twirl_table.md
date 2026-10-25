# B-623: Twirl turns as After Effects' does

D-744, accepted on 2026-10-23 by the owner's message "Proceed, if an effect is more like a inspiration/modified version of a AE effect, only improve it. But it is to be a 1 to 1 match, then go for it." The turn now eases out (a smoothstep, since D-845 the fall-off B-725 fits), and a new Twirl Radius gives the reach in per cent of the layer's larger side, as After Effects' does; Radius in pixels stays for projects made before it. Every expected pixel is `Fixtures/twirl/expected_twirl.json`, written by `tools/twirl_reference.py` before this code and printed in document 25 as FX-TWIRL-001 to 027. The build's frame is compared sample by sample against the catalogue's tolerance of 2e-5.

## FX-TWIRL-001 to 027 (document 25)

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

## A project saved before Twirl Radius

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_twirl_001.json, written before D-744 with no twirl_radius, opens with Twirl Radius 0, so its Radius of 50 pixels still sets the reach | EffectInstance { instance_id: Id("fx-0-0"), enabled: true, effect: Twirl { angle: 90.0, radius: 50.0, center: [50.0, 50.0], twirl_radius: 0.0 }, tracks: {}, mix: 100.0, record: FileRecord(Some(Object {"enabled": Bool(true), "instance_id": String("fx-0-0"), "parameters": Object {"angle": Number(90), "center": Array [Number(50), Number(50)], "radius": Number(50)}, "type_id": String("core.twirl")})) } | yes |
| saving it again writes no twirl_radius | {"angle":90,"center":[50,50],"radius":50} | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview keeps Twirl Radius at 30 per cent, as the layer it is a share of is halved too | Twirl { angle: 90.0, radius: 0.0, center: [50.0, 50.0], twirl_radius: 30.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_twirl_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twirl_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twirl_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twirl_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twirl_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twirl_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_twirl_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with a twirl radius that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| twirl radius 101 is refused with a sentence, and nothing changes | Twirl's twirl radius runs from 0 to 100, and this is 101. | yes |
| twirl radius -1 is refused with a sentence, and nothing changes | Twirl's twirl radius runs from 0 to 100, and this is -1. | yes |
| twirl radius keyed to 150 is refused with a sentence, and nothing changes | Twirl's twirl radius runs from 0 to 100, and this is 150. | yes |
| twirl radius 0, is taken | taken | yes |
| twirl radius 100, is taken | taken | yes |
| twirl radius keyed from 0 to 60 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_twirl_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_twirl_025.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

93 of 93 checks pass.
