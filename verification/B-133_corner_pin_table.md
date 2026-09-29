# B-133: Corner Pin

D-198, accepted on 2026-09-28 with the After Effects picks (A8). Every expected pixel is `Fixtures/corner_pin/expected_corner_pin.json`, written by `tools/corner_pin_reference.py` before this code existed and printed in document 25 as FX-PIN-001 to 018. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-PIN-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PIN-001 frame 0: The settings as they start, the corners at (0, 0), (100, 0), (0, 100) and (100, 100): the drawing, untouched, and nothing grows. | largest difference 1.9e-7 | yes |
| FX-PIN-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PIN-002 frame 0: Shrunk to the middle, the corners at (25, 25), (75, 25), (25, 75) and (75, 75): the whole drawing at half its size in columns 4 to 11 and rows 2 to 6, its stripes one pixel wide; everything round it is clear. Nothing grows. | largest difference 1.9e-7 | yes |
| FX-PIN-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PIN-003 frame 0: A keystone, Upper Left at (25, 0) and Upper Right at (75, 0): the top is half as wide as the foot, as if the drawing leaned back, and in perspective its far upper half is drawn smaller than its near lower half, so the blue band, halfway down the drawing, lands in rows 2 and 3 instead of 4 and 5. | largest difference 2.5e-7 | yes |
| FX-PIN-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PIN-004 frame 0: Turned over left to right, Upper Left at (100, 0), Upper Right at (0, 0), Lower Left at (100, 100) and Lower Right at (0, 100): the drawing mirrored, column 15 of the frame being column 0 of the drawing. | largest difference 1.9e-7 | yes |
| FX-PIN-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PIN-005 frame 0: A half turn, Upper Left at (100, 100), Upper Right at (0, 100), Lower Left at (100, 0) and Lower Right at (0, 0): the drawing upside down and mirrored. | largest difference 1.9e-7 | yes |
| FX-PIN-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PIN-006 frame 0: Upper Left pulled out to (-25, 0), the layer moved three pixels right: the top left corner stretches four pixels past the drawing's left edge, the layer grows four pixels each side, and the three columns left of the drawing show the stretched corner in rows 0 to 8. | largest difference 2.5e-7 | yes |
| FX-PIN-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PIN-007 frame 0: Lower Right pulled in to (60, 60): the drawing's lower right corner folds in towards its middle, the frame's lower right is clear, and the stripes and band bend towards it in perspective. | largest difference 2.5e-7 | yes |
| FX-PIN-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PIN-008 frame 0: Upper Right at (100, 100) and Lower Right at (100, 0), crossed like a bow tie: no drawable shape, so the frame is clear. | largest difference 0.0e0 | yes |
| FX-PIN-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PIN-009 frame 0: Lower Right at (30, 30), inside the other three, a shape bent in: clear. | largest difference 0.0e0 | yes |
| FX-PIN-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PIN-010 frame 0: Upper Right at (50, 0) and Lower Right at (100, 0), three corners in a line: clear. | largest difference 0.0e0 | yes |
| FX-PIN-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PIN-011 frame 0: Upper Right keyed from (100, 0) at frame 0 to (100, 50) at frame 4, linear: frame 0 is the drawing, frame 2 is Upper Right at (100, 25) and frame 4 at (100, 50), the drawing's right side ever shorter and further away. | largest difference 1.9e-7 | yes |
| FX-PIN-011 frame 2: Upper Right keyed from (100, 0) at frame 0 to (100, 50) at frame 4, linear: frame 0 is the drawing, frame 2 is Upper Right at (100, 25) and frame 4 at (100, 50), the drawing's right side ever shorter and further away. | largest difference 2.5e-7 | yes |
| FX-PIN-011 frame 4: Upper Right keyed from (100, 0) at frame 0 to (100, 50) at frame 4, linear: frame 0 is the drawing, frame 2 is Upper Right at (100, 25) and frame 4 at (100, 50), the drawing's right side ever shorter and further away. | largest difference 2.5e-7 | yes |
| FX-PIN-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PIN-012 frame 0: Upper Left keyed from (0, 0) at frame 0 to (-400, 0) at frame 4, eased past its end (about (-530, 0) at frame 2): frame 0 is the drawing, and frame 2 is held at (-400, 0), the same as frame 4. | largest difference 1.9e-7 | yes |
| FX-PIN-012 frame 2: Upper Left keyed from (0, 0) at frame 0 to (-400, 0) at frame 4, eased past its end (about (-530, 0) at frame 2): frame 0 is the drawing, and frame 2 is held at (-400, 0), the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-PIN-012 frame 4: Upper Left keyed from (0, 0) at frame 0 to (-400, 0) at frame 4, eased past its end (about (-530, 0) at frame 2): frame 0 is the drawing, and frame 2 is held at (-400, 0), the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-PIN-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PIN-013 frame 0: All four corners 25 to the left, the layer moved four pixels right: the drawing is moved four pixels left inside the layer and back again, so the frame is the drawing as it was, its first four columns kept in the layer's growth. | largest difference 1.9e-7 | yes |
| FX-PIN-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PIN-014 frame 0: Motion Tile at 300% by 300%, then shrunk to the middle as FX-PIN-002: the tiles come along, so the frame is filled with half-size tiles, 8 pixels wide and 5 high, repeating across and down. | largest difference 1.9e-7 | yes |
| FX-PIN-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PIN-015 frame 0: Motion Tile at 300% by 300%, then Upper Left at (0, 45) and Lower Left at (0, 55), the drawing's left side squeezed small and far away, the layer moved eight pixels right: the tiles run away to a horizon 1.78 pixels left of the drawing, and the columns left of it are clear, where a map without the horizon test would draw the tiles again turned over. | largest difference 2.1e-7 | yes |
| FX-PIN-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PIN-016 frame 0: Upper Left at (-401, 0), its x below -400. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PIN-016 frame 4: Upper Left at (-401, 0), its x below -400. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PIN-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PIN-017 frame 0: Lower Right at (100, 501), its y above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PIN-017 frame 4: Lower Right at (100, 501), its y above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PIN-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PIN-018 frame 0: Upper Right keyed to (600, 0) at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PIN-018 frame 4: Upper Right keyed to (600, 0) at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PIN-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it declares no fixed growth to the card, which never runs it: how far it grows depends on the size the drawing reaches it at, and is counted as the stack runs, as FX-PIN-006, 013 and 015 show | 0 | yes |
| a half-size draft preview keeps every corner, each being a share of the drawing | CornerPin { upper_left: [0.0, 0.0], upper_right: [100.0, 0.0], lower_left: [0.0, 100.0], lower_right: [60.0, 60.0] } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pin_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pin_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pin_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pin_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pin_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pin_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pin_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pin_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pin_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_pin_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `lower_right` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a corner of one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a corner that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| upper left x -401 is refused with a sentence, and nothing changes | Corner Pin's upper left runs from -400 to 500, and this is -401. | yes |
| upper right y 501 is refused with a sentence, and nothing changes | Corner Pin's upper right runs from -400 to 500, and this is 501. | yes |
| lower left x 500.5 is refused with a sentence, and nothing changes | Corner Pin's lower left runs from -400 to 500, and this is 500.5. | yes |
| lower right y -400.5 is refused with a sentence, and nothing changes | Corner Pin's lower right runs from -400 to 500, and this is -400.5. | yes |
| lower right keyed to (600, 100) is refused with a sentence, and nothing changes | Corner Pin's lower right runs from -400 to 500, and this is 600. | yes |
| every corner at the bottom of its range, is taken | taken | yes |
| every corner at the top of its range, is taken | taken | yes |
| a bow tie, which draws nothing, is taken | taken | yes |
| upper left keyed from (0, 0) to (-400, 500) is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a poster, in `verification/B-133 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the poster with no effect, draws cleanly | [] | yes |
| keystone.png, Upper Left (25, 0) and Upper Right (75, 0): the top drawn in, the bottom kept; the top corners clear, the bottom ones and the middle as they were; draws cleanly | [], [[0, 0, 0, 0], [0, 0, 0, 0], [30, 26, 36, 255], [30, 26, 36, 255], [106, 160, 255, 255]] | yes |
| wall.png, pinned to (52, 18), (88, 8), (52, 70) and (88, 86), a wall on the right leaning away: the left half clear, the poster's sky inside the four corners; draws cleanly | [], [[0, 0, 0, 0], [0, 0, 0, 0], [106, 160, 255, 255]] | yes |
| flipped.png, the left corners and the right ones swapped: the poster turned over left to right, the sun on the left; draws cleanly | [], every pixel the poster's across from it: true | yes |
| crossed.png, the bottom corners swapped, a bow tie: nothing drawn, as crossed corners draw nothing; draws cleanly | [], every pixel clear: true | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pin_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pin_012.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_pin_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

76 of 76 checks pass.
