# B-145: Optics Compensation

D-210, accepted on 2026-09-28 with the After Effects picks (B8). Every expected pixel is `Fixtures/optics_compensation/expected_optics_compensation.json`, written by `tools/optics_compensation_reference.py` before this code existed and printed in document 25 as FX-OPTICS-001 to 020. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-OPTICS-001 to 020 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-OPTICS-001 frame 0: Field of view 90, horizontal, reverse off, the centre in the middle: a fisheye. The middle keeps its size and the stripes bow outward round it, squeezed more the farther out they are; the left and right edges read from past the drawing, so they and the corners are emptied. | largest difference 2.5e-7 | yes |
| FX-OPTICS-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OPTICS-002 frame 0: Field of view 0, as it starts: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-OPTICS-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OPTICS-003 frame 0: Reverse on: the fisheye taken out, every pixel reading from nearer the middle, the stripes stretched outward, the empty right-hand column and top and bottom rows filled from inside. | largest difference 2.5e-7 | yes |
| FX-OPTICS-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OPTICS-004 frame 0: Vertical: the 90 degrees span the height, ten pixels, not the width, so the squeeze is stronger and more of the drawing is emptied than in FX-OPTICS-001. | largest difference 2.5e-7 | yes |
| FX-OPTICS-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OPTICS-005 frame 0: Diagonal: the 90 degrees span the diagonal, so the squeeze is gentler and less is emptied than in FX-OPTICS-001. | largest difference 2.5e-7 | yes |
| FX-OPTICS-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OPTICS-006 frame 0: Field of view 180, the most, reverse off: only the round part within eight pixels of the middle shows, squeezed hard toward its rim; every pixel farther out is empty. | largest difference 2.5e-7 | yes |
| FX-OPTICS-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OPTICS-007 frame 0: Field of view 180, reverse on: the whole picture reads from within eight pixels of the middle, stretched hard toward the edges. | largest difference 2.5e-7 | yes |
| FX-OPTICS-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OPTICS-008 frame 0: Centre 25, 50: the lens's middle at (4, 5), the left of the drawing kept and the right squeezed away. | largest difference 2.5e-7 | yes |
| FX-OPTICS-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OPTICS-009 frame 0: Centre 53.125, 55: the middle is the centre of pixel (8, 5) itself, so that pixel reads itself and is kept exactly. | largest difference 2.6e-7 | yes |
| FX-OPTICS-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OPTICS-010 frame 0: Field of view keyed from 0 at frame 0 to 120 at frame 4, linear: frame 0 is the drawing, frame 2 is field of view 60 and frame 4 is 120. | largest difference 1.9e-7 | yes |
| FX-OPTICS-010 frame 2: Field of view keyed from 0 at frame 0 to 120 at frame 4, linear: frame 0 is the drawing, frame 2 is field of view 60 and frame 4 is 120. | largest difference 2.5e-7 | yes |
| FX-OPTICS-010 frame 4: Field of view keyed from 0 at frame 0 to 120 at frame 4, linear: frame 0 is the drawing, frame 2 is field of view 60 and frame 4 is 120. | largest difference 2.5e-7 | yes |
| FX-OPTICS-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OPTICS-011 frame 0: Centre keyed from 50, 50 at frame 0 to 25, 50 at frame 4, linear: frame 0 is FX-OPTICS-001, frame 2 is centre 37.5, 50, and frame 4 is FX-OPTICS-008. | largest difference 2.5e-7 | yes |
| FX-OPTICS-011 frame 2: Centre keyed from 50, 50 at frame 0 to 25, 50 at frame 4, linear: frame 0 is FX-OPTICS-001, frame 2 is centre 37.5, 50, and frame 4 is FX-OPTICS-008. | largest difference 2.5e-7 | yes |
| FX-OPTICS-011 frame 4: Centre keyed from 50, 50 at frame 0 to 25, 50 at frame 4, linear: frame 0 is FX-OPTICS-001, frame 2 is centre 37.5, 50, and frame 4 is FX-OPTICS-008. | largest difference 2.5e-7 | yes |
| FX-OPTICS-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OPTICS-012 frame 0: Field of view eased from 90 at frame 0 to 180 at frame 4 on a curve that overshoots: at frame 2 it would pass 180, is held at 180, and is FX-OPTICS-006. | largest difference 2.5e-7 | yes |
| FX-OPTICS-012 frame 2: Field of view eased from 90 at frame 0 to 180 at frame 4 on a curve that overshoots: at frame 2 it would pass 180, is held at 180, and is FX-OPTICS-006. | largest difference 2.5e-7 | yes |
| FX-OPTICS-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OPTICS-013 frame 0: FX-OPTICS-001 moved three pixels right: the lens moves with the drawing, and nothing grows. | largest difference 2.5e-7 | yes |
| FX-OPTICS-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OPTICS-014 frame 0: After a Motion Tile that grows the layer: the lens's middle and its span are still the drawing's own, so the stripes bow as in FX-OPTICS-001, but the edges now read the tiles around instead of emptiness. | largest difference 2.5e-7 | yes |
| FX-OPTICS-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-OPTICS-015 frame 0: Field of view -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OPTICS-015 frame 4: Field of view -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OPTICS-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-OPTICS-016 frame 0: Field of view 181, above 180. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OPTICS-016 frame 4: Field of view 181, above 180. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OPTICS-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-OPTICS-017 frame 0: Orientation "sideways", which is not "horizontal", "vertical" or "diagonal". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OPTICS-017 frame 4: Orientation "sideways", which is not "horizontal", "vertical" or "diagonal". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OPTICS-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-OPTICS-018 frame 0: Reverse "yes", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OPTICS-018 frame 4: Reverse "yes", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OPTICS-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-OPTICS-019 frame 0: Centre 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OPTICS-019 frame 4: Centre 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OPTICS-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-OPTICS-020 frame 0: Field of view keyed to 200 at frame 4, above 180. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OPTICS-020 frame 4: Field of view keyed to 200 at frame 4, above 180. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-OPTICS-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it never grows the layer: it declares no growth | 0 | yes |
| a half-size draft preview keeps every setting, as none is a distance in pixels | OpticsCompensation { field_of_view: 120.0, reverse: "on", orientation: "diagonal", center: [25.0, 75.0] } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_optics_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_optics_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `orientation` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a centre of one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a field of view that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| field of view -1 is refused with a sentence, and nothing changes | Optics Compensation's field of view runs from 0 to 180, and this is -1. | yes |
| field of view 181 is refused with a sentence, and nothing changes | Optics Compensation's field of view runs from 0 to 180, and this is 181. | yes |
| centre down -1001 is refused with a sentence, and nothing changes | Optics Compensation's center runs from -1000 to 1000, and this is -1001. | yes |
| reverse "yes" is refused with a sentence, and nothing changes | Optics Compensation's reverse lens distortion is "on" or "off", and this is "yes". | yes |
| orientation "sideways" is refused with a sentence, and nothing changes | Optics Compensation's FOV orientation is "horizontal", "vertical" or "diagonal", and this is "sideways". | yes |
| orientation "Horizontal", written with a capital is refused with a sentence, and nothing changes | Optics Compensation's FOV orientation is "horizontal", "vertical" or "diagonal", and this is "Horizontal". | yes |
| field of view keyed to 200 is refused with a sentence, and nothing changes | Optics Compensation's field of view runs from 0 to 180, and this is 200. | yes |
| centre keyed to 2000 across is refused with a sentence, and nothing changes | Optics Compensation's center runs from -1000 to 1000, and this is 2000. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| field of view keyed from 0 to 120 is taken | taken | yes |
| centre keyed across the drawing is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_optics_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_optics_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_optics_010.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_optics_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a made-up drawing, in `verification/B-145 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the drawing with no effect, draws cleanly | [] | yes |
| fov_90.png, field of view 90: a fisheye, the middle four pixels kept, the lines bowed, the left and right edges emptied from top to bottom, as they read from past the drawing; draws cleanly | [], middle kept true, 100 of 100 rows empty at both ends, 4016 clear | yes |
| fov_180.png, field of view 180, the most: only the round part within 80 pixels of the middle shows, every pixel farther out empty; draws cleanly | [], 1112 of 1112 farther out empty | yes |
| reverse.png, reverse on: the edges stretched out, so the blue frame is pushed off the sides; the middle kept, nothing clear; draws cleanly | [], middle kept true, 0 clear, left edge [98, 96, 106, 255] | yes |
| vertical.png and diagonal.png: the 90 degrees across the height bends harder than across the width, and across the diagonal more gently, so more of vertical.png is empty than of fov_90.png, and less of diagonal.png; both draw cleanly | [] [], empty pixels: vertical 7224, horizontal 4016, diagonal 3092 | yes |
| centre_25.png, the centre a quarter of the way across: the lens's middle moves there and the four pixels round it are kept; the picture differs from fov_90.png; draws cleanly | [], kept true | yes |
| fov_60.png, then in_then_out.png, the same lens put in and taken out again: back to the drawing, softer, as each thin ruled line is sampled twice, but on average closer to before.png than the fisheye alone; both draw cleanly | [] [], average difference: fisheye 36.05, put in and taken out 16.87 | yes |

## Result

100 of 100 checks pass.
