# B-654: Drop Shadow's softness blurs at sigma softness / 5 (D-774)

Seven tutorials from seven creators fit After Effects' Drop Shadow edge at sigma softness / 5.04 (ours, at softness / 3, read 1.57 to 1.83 times as wide, median 1.68). The shadow is now the drawing's covering blurred at sigma softness / 5 and grows the layer by that blur's reach. The edge is read off a white 100 by 100 square's black shadow at Opacity 100 and Distance 0: how far out its covering falls to 0.1587, one sigma on a Gaussian's edge; the fixtures' numbers are the reference's, whose softened cases' softness D-774 raised by 5 / 3 so their pixels stay.

## The shadow's edge: sigma, in pixels, within 3 per cent

| Check | The build's answer | Matches |
| --- | --- | --- |
| Softness 10: sigma 2 (it was 3.33 before D-774) | 2.04 pixels | yes |
| Softness 25: sigma 5 (it was 8.33 before D-774) | 5.00 pixels | yes |
| Softness 50: sigma 10 (it was 16.67 before D-774) | 9.98 pixels | yes |
| Softness 100: sigma 20 (it was 33.33 before D-774) | 19.80 pixels | yes |

## The fixtures, at the reference's numbers (tolerance 2e-5)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHADOW-001 frame 0: The defaults: black at 50 per cent, direction 135, distance 5, softness 0. The card's shadow falls down and to the right, 3.54 pixels each way, spread by the bilinear sample over the pixels it falls between; where it lies wholly under the card's shape it is black at half covering. The card itself, fully covered, is unchanged, and the shadow's lower part falls below the frame. | largest difference 1.9e-7 | yes |
| FX-SHADOW-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-002 frame 0: Direction 0, distance 2: the shadow sits exactly two pixels above the card, black at half covering in rows 1 and 2; the soft pixel's shadow, at (10, 2), is half its covering of 128/255. | largest difference 1.9e-7 | yes |
| FX-SHADOW-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-003 frame 0: Direction 90, distance 2: exactly two pixels right. The soft pixel, with the card's shadow behind it, takes the shadow through its uncovered share: its covering goes from 0.502 to 0.751, its colour unchanged, as the shadow is black. | largest difference 1.9e-7 | yes |
| FX-SHADOW-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-004 frame 0: Direction 180, distance 2: exactly two pixels below, in rows 7 and 8, and the soft pixel's, at (10, 6), is half its covering. | largest difference 1.9e-7 | yes |
| FX-SHADOW-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-005 frame 0: Direction 270, distance 2: exactly two pixels left, in columns 3 and 4. | largest difference 1.9e-7 | yes |
| FX-SHADOW-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-006 frame 0: Direction -270 is a quarter turn clockwise from up, the same as 90: FX-SHADOW-003 exactly. | largest difference 1.9e-7 | yes |
| FX-SHADOW-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-007 frame 0: Distance 0, softness 0: the shadow sits exactly behind the card, so it shows only through the soft pixel, whose covering goes from 0.502 to 0.627; nothing else changes and the layer does not grow. | largest difference 1.9e-7 | yes |
| FX-SHADOW-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-008 frame 0: Distance 0, softness 5 (3 before D-774): the shadow, blurred at sigma 1, spreads three pixels out from the card on every side, darkest next to it and fading outward; column 1 and beyond stay empty. | largest difference 1.9e-7 | yes |
| FX-SHADOW-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-009 frame 0: Softness 10 (6 before D-774) at the default direction and distance: the shadow is FX-SHADOW-001's, blurred at sigma 2, so its darkest pixel is lighter than half covering and it reaches further. | largest difference 1.9e-7 | yes |
| FX-SHADOW-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-010 frame 0: Opacity 0: no shadow; the frame is the drawing, untouched, though the layer still grows. | largest difference 1.9e-7 | yes |
| FX-SHADOW-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-011 frame 0: Colour #2040a0, opacity 100, distance 2: a blue shadow at full covering where it lies wholly under the card's shape, its straight colour exactly #2040a0. | largest difference 1.9e-7 | yes |
| FX-SHADOW-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-012 frame 0: FX-SHADOW-011 with the colour written in capitals: the same. | largest difference 1.9e-7 | yes |
| FX-SHADOW-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-013 frame 0: Direction 90, distance keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is FX-SHADOW-007, frame 2 is FX-SHADOW-003, frame 4 is four pixels right. | largest difference 1.9e-7 | yes |
| FX-SHADOW-013 frame 2: Direction 90, distance keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is FX-SHADOW-007, frame 2 is FX-SHADOW-003, frame 4 is four pixels right. | largest difference 1.9e-7 | yes |
| FX-SHADOW-013 frame 4: Direction 90, distance keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is FX-SHADOW-007, frame 2 is FX-SHADOW-003, frame 4 is four pixels right. | largest difference 1.9e-7 | yes |
| FX-SHADOW-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-014 frame 0: Distance 2, direction keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 is FX-SHADOW-002, frame 2, at 90, is FX-SHADOW-003, and frame 4 is FX-SHADOW-004. | largest difference 1.9e-7 | yes |
| FX-SHADOW-014 frame 2: Distance 2, direction keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 is FX-SHADOW-002, frame 2, at 90, is FX-SHADOW-003, and frame 4 is FX-SHADOW-004. | largest difference 1.9e-7 | yes |
| FX-SHADOW-014 frame 4: Distance 2, direction keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 is FX-SHADOW-002, frame 2, at 90, is FX-SHADOW-003, and frame 4 is FX-SHADOW-004. | largest difference 1.9e-7 | yes |
| FX-SHADOW-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-015 frame 0: Opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 0 has no shadow; frame 2 is held at 100 and is frame 4. | largest difference 1.9e-7 | yes |
| FX-SHADOW-015 frame 2: Opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 0 has no shadow; frame 2 is held at 100 and is frame 4. | largest difference 1.9e-7 | yes |
| FX-SHADOW-015 frame 4: Opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 0 has no shadow; frame 2 is held at 100 and is frame 4. | largest difference 1.9e-7 | yes |
| FX-SHADOW-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-016 frame 0: FX-SHADOW-001 moved three pixels right: the same, moved, and the shadow's right end, which would fall in columns 16 and 17, is cut off by the frame's edge. | largest difference 1.9e-7 | yes |
| FX-SHADOW-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-017 frame 0: Direction 0, distance 5: the shadow of the card's two lower rows shows in rows 0 and 1; the rest falls above the frame and is cut off, and row 2 stays empty. | largest difference 1.9e-7 | yes |
| FX-SHADOW-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-018 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-018 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHADOW-019 frame 0: Distance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-019 frame 4: Distance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHADOW-020 frame 0: Softness 2001, above 2000 (501, above 500, before D-829). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-020 frame 4: Softness 2001, above 2000 (501, above 500, before D-829). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHADOW-021 frame 0: Direction 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-021 frame 4: Direction 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHADOW-022 frame 0: A colour written "black". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-022 frame 4: A colour written "black". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHADOW-023 frame 0: Opacity keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-023 frame 4: Opacity keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHADOW-024 frame 0: Shadow Only "on" at the defaults: the card is left out and the shadow alone is drawn, FX-SHADOW-001's shadow exactly, black at half covering wherever it lies wholly under the card's moved shape; in row 6, where the card was, the shadow's top edge shows part covered, and above it, where the card was, the frame is empty. | largest difference 1.0e-8 | yes |
| FX-SHADOW-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-025 frame 0: Shadow Only "on", distance 0, softness 0: the shadow exactly where the card was, black at half the card's covering at every pixel; the soft pixel at (10, 4) is black at half of 128/255. | largest difference 1.5e-8 | yes |
| FX-SHADOW-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-026 frame 0: Shadow Only "off" written in the file: FX-SHADOW-001 exactly, as a file without the word. | largest difference 1.9e-7 | yes |
| FX-SHADOW-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-027 frame 0: Shadow Only "on", colour #2040a0, opacity 100, direction 90, distance 2, softness 5 (3 before D-774): the blue shadow alone, softened, two pixels right of the card's place. | largest difference 1.5e-7 | yes |
| FX-SHADOW-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-028 frame 0: Shadow Only "on" with opacity 0: nothing at all, the frame is empty, though the layer still grows. | largest difference 0.0e0 | yes |
| FX-SHADOW-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-029 frame 0: Shadow Only written "On", the panel's name, not the word "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-029 frame 4: Shadow Only written "On", the panel's name, not the word "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHADOW-030 frame 0: Shadow Only written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-030 frame 4: Shadow Only written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHADOW-031 frame 0: Softness 600, above the old top of 500, Shadow Only "on", opacity 100, distance 0: blurred at sigma 120, the card's shadow is spread so wide that every pixel of the frame takes a faint, almost even shadow, darkest where the card was. | largest difference 3.7e-11 | yes |
| FX-SHADOW-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-032 frame 0: Softness 1000, as FX-SHADOW-031: sigma 200, so about (120 / 200)^2, 0.36, of its covering at every pixel. | largest difference 1.2e-11 | yes |
| FX-SHADOW-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-033 frame 0: Softness 2000, the new top, as FX-SHADOW-031: sigma 400, about a quarter of FX-SHADOW-032's covering at every pixel. On this small frame it is near the tolerance, so B-709 also holds the build to it within a thousandth of each pixel's own value. | largest difference 4.3e-12 | yes |
| FX-SHADOW-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |

## An old project

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shadow_008.json as saved before D-774 (softness 3, its file at 621e141b) opens with no warning | [] | yes |
| and saves back holding what it held: softness still 3, nothing added | the same | yes |
| and draws a tighter shadow (sigma 0.6), not FX-SHADOW-008's sigma 1, with no warning | largest difference from FX-SHADOW-008 6.5e-2; warnings [] | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| Softness 10, Distance 0, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| Softness 50, Distance 20, Opacity 60, frame 0 at Full and Draft | largest difference 1 of 255; refused by the card: false; the same warnings: true | yes |
| Softness 100, Distance 7.5, direction 200, colour #2040a0, frame 0 at Full and Draft | largest difference 1 of 255; refused by the card: false; the same warnings: true | yes |
| Softness 25, Shadow Only on, frame 0 at Full and Draft | largest difference 1 of 255; refused by the card: false; the same warnings: true | yes |
| fx_shadow_008.json, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| fx_shadow_009.json, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| fx_shadow_027.json, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |

## Result

94 of 94 checks pass.
