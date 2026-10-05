# B-136: Polar Coordinates

D-201, accepted on 2026-09-28 with the After Effects picks (A11). Every expected pixel is `Fixtures/polar_coordinates/expected_polar_coordinates.json`, written by `tools/polar_coordinates_reference.py` before this code existed and printed in document 25 as FX-POLAR-001 to 015. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-POLAR-001 to 015 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-POLAR-001 frame 0: The settings as they start, Interpolation 100 and Rect to Polar: the drawing bent round its middle. Its top row is squeezed into the middle, its blue band, rows 4 and 5, is a ring halfway out, and its foot, the empty row 9, lies round the ellipse that touches the frame's sides, so the frame's corners, beyond it, are clear. Its upright stripes are spokes from the middle, and its empty right-hand column is a clear spoke straight up. | largest difference 2.2e-7 | yes |
| FX-POLAR-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLAR-002 frame 0: Interpolation 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-POLAR-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLAR-003 frame 0: Polar to Rect at 100: the drawing unrolled. Its middle, blue, is spread along the frame's top rows; each lower row reads a ring further out, the left column straight up from the middle and the columns going round clockwise, so the stripes cross the frame as slanting bands. | largest difference 2.5e-7 | yes |
| FX-POLAR-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLAR-004 frame 0: Rect to Polar at 50: half way, each pixel read from halfway between where it is and where FX-POLAR-001 reads it. | largest difference 2.5e-7 | yes |
| FX-POLAR-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLAR-005 frame 0: Polar to Rect at 50: half way to FX-POLAR-003. | largest difference 2.5e-7 | yes |
| FX-POLAR-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLAR-006 frame 0: Interpolation keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-POLAR-004 and frame 4 is FX-POLAR-001. | largest difference 1.9e-7 | yes |
| FX-POLAR-006 frame 2: Interpolation keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-POLAR-004 and frame 4 is FX-POLAR-001. | largest difference 2.5e-7 | yes |
| FX-POLAR-006 frame 4: Interpolation keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-POLAR-004 and frame 4 is FX-POLAR-001. | largest difference 2.2e-7 | yes |
| FX-POLAR-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLAR-007 frame 0: Interpolation keyed from 0 at frame 0 to 100 at frame 4, eased past its end (about 131 at frame 2): frame 0 is the drawing, and frame 2 is held at 100, FX-POLAR-001. | largest difference 1.9e-7 | yes |
| FX-POLAR-007 frame 2: Interpolation keyed from 0 at frame 0 to 100 at frame 4, eased past its end (about 131 at frame 2): frame 0 is the drawing, and frame 2 is held at 100, FX-POLAR-001. | largest difference 2.2e-7 | yes |
| FX-POLAR-007 frame 4: Interpolation keyed from 0 at frame 0 to 100 at frame 4, eased past its end (about 131 at frame 2): frame 0 is the drawing, and frame 2 is held at 100, FX-POLAR-001. | largest difference 2.2e-7 | yes |
| FX-POLAR-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLAR-008 frame 0: Rect to Polar at 100, the layer moved three pixels right: FX-POLAR-001 moved with it. | largest difference 2.1e-7 | yes |
| FX-POLAR-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLAR-009 frame 0: Motion Tile at 300% by 300%, then Rect to Polar at 100, the layer moved eight pixels right: inside the ellipse FX-POLAR-001 moved with the layer, and beyond it, where Rect to Polar reads below the drawing's foot, the tiles beneath the drawing, bent round. | largest difference 2.5e-7 | yes |
| FX-POLAR-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLAR-010 frame 0: Motion Tile at 300% by 300%, then Polar to Rect at 100, the layer moved eight pixels right: FX-POLAR-003 in the drawing's place, and to its left FX-POLAR-003's right half again, the rule going round once more past the drawing's side; only the middle of the foot row differs, reading a little of the tile below the drawing where FX-POLAR-003 reads nothing. | largest difference 2.5e-7 | yes |
| FX-POLAR-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLAR-011 frame 0: Rect to Polar at 100, then a second Polar Coordinates, Polar to Rect at 100: the drawing back again, roughly: its band and stripes where they were, but soft in its top row, which the first squeezed into the middle, and all round its edges, which the first bent into the ellipse's rim and the wedge its empty column made. | largest difference 2.3e-7 | yes |
| FX-POLAR-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLAR-012 frame 0: Interpolation 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POLAR-012 frame 4: Interpolation 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POLAR-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POLAR-013 frame 0: Interpolation -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POLAR-013 frame 4: Interpolation -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POLAR-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POLAR-014 frame 0: Interpolation keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POLAR-014 frame 4: Interpolation keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POLAR-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-POLAR-015 frame 0: Conversion "sideways", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POLAR-015 frame 4: Conversion "sideways", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POLAR-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it never grows the layer: it declares no growth | 0 | yes |
| a half-size draft preview keeps Interpolation, which is a share, not a distance | PolarCoordinates { interpolation: 60.0, conversion: "polar_to_rect", shape: "ellipse" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_polar_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polar_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polar_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polar_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polar_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polar_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polar_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polar_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polar_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polar_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `conversion` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no `interpolation` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a conversion that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Interpolation 100.5 is refused with a sentence, and nothing changes | Polar Coordinates's interpolation runs from 0 to 100, and this is 100.5. | yes |
| Interpolation -0.5 is refused with a sentence, and nothing changes | Polar Coordinates's interpolation runs from 0 to 100, and this is -0.5. | yes |
| conversion "sideways" is refused with a sentence, and nothing changes | Polar Coordinates' conversion is "rect_to_polar" or "polar_to_rect", and this is "sideways". | yes |
| conversion "Rect_To_Polar", written in capitals is refused with a sentence, and nothing changes | Polar Coordinates' conversion is "rect_to_polar" or "polar_to_rect", and this is "Rect_To_Polar". | yes |
| Interpolation keyed to 150 is refused with a sentence, and nothing changes | Polar Coordinates's interpolation runs from 0 to 100, and this is 150. | yes |
| Interpolation 0, is taken | taken | yes |
| Polar to Rect at 100, is taken | taken | yes |
| Interpolation keyed from 0 to 100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: speed lines and a target, in `verification/B-136 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before_lines.png, the speed lines with no effect, draws cleanly | [] | yes |
| focus_lines.png, Rect to Polar at 100: the streaks turned into spokes pointing in to the middle, which the lines never reach, as their top third was clear, and the corners clear, as the drawing's edge is its outer ring; of the pixels near the edge a tenth at least dark and a tenth at least clear; draws cleanly | [], middle [0, 0, 0, 0], corners [0, 0, 0, 0], near the edge 693 dark and 490 clear of 1644 | yes |
| half_way.png, Rect to Polar at 50: between the two, neither the speed lines nor the focus lines; draws cleanly | [], same as the speed lines: false, as the focus lines: false | yes |
| back_again.png, Rect to Polar then a second effect, Polar to Rect: the speed lines back, a little softer from being drawn twice; at least three pixels in four within 32 of 255 of their covering in before_lines.png; draws cleanly | [], 15460 of 16000 | yes |
| before_target.png, the target with no effect, draws cleanly | [] | yes |
| target_unrolled.png, Polar to Rect at 100: the rings unrolled into stripes, the middle at the top and the outer ring at the foot; rows 9, 29, 49, 69 and 89, one in each band, each its band's colour all the way across; draws cleanly | [], [[255, 208, 64, 255], [200, 40, 40, 255], [244, 236, 216, 255], [58, 111, 216, 255], [30, 26, 36, 255]], every one flat: true | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_polar_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_polar_006.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_polar_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_polar_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

72 of 72 checks pass.
