# D-320: Polar Coordinates, ellipse or circle

From D-308, approved by the owner on 2026-10-04 after P-26's tutorial 1. Polar Coordinates has a Shape: Ellipse, D-201's rule, which bends the layer into the ellipse touching its sides, or Circle, After Effects' look, round the middle with half the shorter side as its radius. A file without a shape is the ellipse and draws as before; a new one is a circle. Every expected pixel is `Fixtures/polar_coordinates/expected_polar_circle.json`, written by `tools/polar_circle_reference.py` before the build had the circle, printed in document 25 as FX-POLAR-016 to 022. Tolerance 2e-5.

## FX-POLAR-016 to 022 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-POLAR-016 frame 0: Shape circle, Rect to Polar at 100, on the 16 by 10 drawing: the drawing bent into a circle of radius 5 round the frame's middle. Its blue band is a round ring halfway out, its foot lies round the circle, which touches the top and bottom, and the frame's left and right ends, beyond the circle, are clear. | largest difference 1.9e-7 | yes |
| FX-POLAR-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLAR-017 frame 0: Shape circle, Polar to Rect at 100: the drawing unrolled from the circle; each row reads a ring of the same radius across and down. | largest difference 2.5e-7 | yes |
| FX-POLAR-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLAR-018 frame 0: Shape circle, Rect to Polar at 50: half way to FX-POLAR-016. | largest difference 2.5e-7 | yes |
| FX-POLAR-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLAR-019 frame 0: Shape circle, Rect to Polar at 100, then a second, shape circle, Polar to Rect at 100: the drawing back again, roughly, as FX-POLAR-011 is for the ellipse. | largest difference 2.0e-7 | yes |
| FX-POLAR-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLAR-020 frame 0: Shape circle, Rect to Polar at 100, the layer moved three pixels right: FX-POLAR-016 moved with it. | largest difference 1.9e-7 | yes |
| FX-POLAR-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLAR-021 frame 0: Shape ellipse, written: FX-POLAR-001 exactly. | largest difference 2.2e-7 | yes |
| FX-POLAR-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-POLAR-022 frame 0: Shape "square", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-POLAR-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_polar_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polar_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polar_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polar_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polar_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_polar_001.json, from before D-320, has no shape and is saved without one | {"conversion":"rect_to_polar","interpolation":100} | yes |
| fx_polar_003.json, from before D-320, has no shape and is saved without one | {"conversion":"polar_to_rect","interpolation":100} | yes |
| fx_polar_011.json, from before D-320, has no shape and is saved without one | {"conversion":"rect_to_polar","interpolation":100} | yes |
| A circle is saved as shape: circle | {"conversion":"rect_to_polar","interpolation":100,"shape":"circle"} | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| shape "square" is refused with a sentence, and nothing changes | Polar Coordinates' shape is "ellipse" or "circle", and this is "square". | yes |
| shape "Circle", written in capitals is refused with a sentence, and nothing changes | Polar Coordinates' shape is "ellipse" or "circle", and this is "Circle". | yes |
| FX-POLAR-001 set to a circle, is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |
| FX-POLAR-001 set to a circle by the command draws FX-POLAR-016's frame | largest difference 1.9e-7 | yes |

## The preview card

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_polar_016.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_polar_017.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_polar_019.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_polar_020.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_polar_001.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_polar_003.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |

## Pictures: a wide plate bent both ways, in `verification/D-320 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_plate.png, the plate with no effect, draws cleanly | [] | yes |
| 2_ellipse_old_file.png, a file from before D-320, no shape: the ellipse as before, the ring stretched to the frame's sides, so wider than tall; draws cleanly | [], the ring starts 57 pixels right of the middle and 32 down; the ends clear: false | yes |
| 3_circle_new.png, shape circle, as a new Polar Coordinates is: the ring round, as far out across as down, and the frame's ends beyond the circle clear; draws cleanly | [], the ring starts 32 pixels right of the middle and 32 down; the ends clear: true | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_polar_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_polar_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_polar_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

40 of 40 checks pass.
