# B-96: mirror

D-153, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the twentieth of the third batch. Every expected pixel is `Fixtures/mirror/expected_mirror.json`, written by `tools/mirror_reference.py` before this code existed and printed in document 25 as FX-MIRROR-001 to 024. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-MIRROR-001 to 024 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MIRROR-001 frame 0: The settings as they start: centre 50, 50, the point (8, 5), angle 0. The line runs straight down between columns 7 and 8; the right half is kept exactly and the left half shows its reflection, column 7 taking column 8, 6 taking 9, down to 0 taking 15: the arrow gets a second head pointing left, with the soft tip in column 1, the skin block spreads to columns 6 to 9, and the dark block is gone. | largest difference 1.9e-7 | yes |
| FX-MIRROR-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIRROR-002 frame 0: Angle 90: the line lies across between rows 4 and 5; the bottom half is kept and the top half shows its reflection, row 4 taking row 5 up to row 0 taking row 9. The dark block is gone and the skin block shows in rows 1 and 2 as well as 7 and 8; the arrow, even top to bottom, is unchanged. | largest difference 1.9e-7 | yes |
| FX-MIRROR-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIRROR-003 frame 0: Angle 180: the left half is kept and the right half shows its reflection, column 8 taking 7 up to 15 taking 0: the head and tip are gone, the shaft runs on to column 13, the dark block shows again in columns 12 to 14 with its soft edge in column 11, and the skin block spans columns 5 to 10. | largest difference 1.9e-7 | yes |
| FX-MIRROR-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIRROR-004 frame 0: Angle 270: the top half is kept and the bottom half shows its reflection, row 5 taking row 4 down to row 9 taking row 0: the skin block is gone and the dark block shows in rows 7 and 8 too. | largest difference 1.3e-7 | yes |
| FX-MIRROR-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIRROR-005 frame 0: Angle 360, a whole turn: FX-MIRROR-001. | largest difference 1.9e-7 | yes |
| FX-MIRROR-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIRROR-006 frame 0: Angle -90, a quarter turn the other way: FX-MIRROR-004. | largest difference 1.3e-7 | yes |
| FX-MIRROR-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIRROR-007 frame 0: Angle 45: the line runs corner-wise through the point (8, 5), down to the left, and the part below and right of it is kept. Each pixel above and left of it takes the pixel mirrored across it, row and column swapped about it: pixel (x, y) takes (12 - y, 12 - x), and one whose mirror falls outside the drawing is empty. | largest difference 1.9e-7 | yes |
| FX-MIRROR-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIRROR-008 frame 0: Angle 30: the line leans a third of a quarter turn; the kept side is exact and the reflected side falls between pixels, so its colours are blends of neighbours, softening the reflected edges. | largest difference 2.5e-7 | yes |
| FX-MIRROR-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIRROR-009 frame 0: Centre 53.125, 50, the point (8.5, 5): the line runs down the middle of column 8, which is kept, with column 7 taking column 9 up to column 0 taking column 16, past the drawing, so column 0 is empty. | largest difference 1.9e-7 | yes |
| FX-MIRROR-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIRROR-010 frame 0: Centre 51.5625, 50, the point (8.25, 5): the line runs a quarter of the way into column 8. The right is kept, and each pixel on the left reads its mirror half-way between two pixels, so it is an even blend of the two: column 7 half column 8 and half column 9, and column 5, where the shaft meets the head, half blue and half red. | largest difference 1.9e-7 | yes |
| FX-MIRROR-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIRROR-011 frame 0: Centre 25, 50, the point (4, 5): columns 4 to 15 are kept and columns 0 to 3 show columns 7 to 4: the dark block gives way to its own soft edge in column 3, the shaft reaches the left edge, and the skin block shows again in columns 0 to 2. | largest difference 1.9e-7 | yes |
| FX-MIRROR-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIRROR-012 frame 0: Centre 0, 50: the line is the drawing's left edge and the whole drawing is on the kept side: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-MIRROR-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIRROR-013 frame 0: Centre 100, 50: the line is the drawing's right edge and the whole drawing is on the reflected side, whose mirror lies past the edge: every pixel is empty. | largest difference 0.0e0 | yes |
| FX-MIRROR-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIRROR-014 frame 0: Centre 50, 0 at angle 0: the line still runs straight down through column 8's left edge, so the centre's height changes nothing: FX-MIRROR-001. | largest difference 1.9e-7 | yes |
| FX-MIRROR-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIRROR-015 frame 0: Centre keyed from 50, 50 at frame 0 to 0, 50 at frame 4, linear: frame 0 is FX-MIRROR-001, frame 2 is centre 25, 50, FX-MIRROR-011, and frame 4 the drawing, untouched, FX-MIRROR-012. | largest difference 1.9e-7 | yes |
| FX-MIRROR-015 frame 2: Centre keyed from 50, 50 at frame 0 to 0, 50 at frame 4, linear: frame 0 is FX-MIRROR-001, frame 2 is centre 25, 50, FX-MIRROR-011, and frame 4 the drawing, untouched, FX-MIRROR-012. | largest difference 1.9e-7 | yes |
| FX-MIRROR-015 frame 4: Centre keyed from 50, 50 at frame 0 to 0, 50 at frame 4, linear: frame 0 is FX-MIRROR-001, frame 2 is centre 25, 50, FX-MIRROR-011, and frame 4 the drawing, untouched, FX-MIRROR-012. | largest difference 1.9e-7 | yes |
| FX-MIRROR-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIRROR-016 frame 0: Angle keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 is FX-MIRROR-001, frame 2 is angle 90, FX-MIRROR-002, frame 4 is FX-MIRROR-003. | largest difference 1.9e-7 | yes |
| FX-MIRROR-016 frame 2: Angle keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 is FX-MIRROR-001, frame 2 is angle 90, FX-MIRROR-002, frame 4 is FX-MIRROR-003. | largest difference 1.9e-7 | yes |
| FX-MIRROR-016 frame 4: Angle keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 is FX-MIRROR-001, frame 2 is angle 90, FX-MIRROR-002, frame 4 is FX-MIRROR-003. | largest difference 1.9e-7 | yes |
| FX-MIRROR-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIRROR-017 frame 0: Angle eased from 0 at frame 0 to 3600 at frame 4 on a curve that overshoots: at frame 2 it would be 4770, a quarter turn past a whole number of turns, is held at 3600 and is FX-MIRROR-001, as frames 0 and 4 are. | largest difference 1.9e-7 | yes |
| FX-MIRROR-017 frame 2: Angle eased from 0 at frame 0 to 3600 at frame 4 on a curve that overshoots: at frame 2 it would be 4770, a quarter turn past a whole number of turns, is held at 3600 and is FX-MIRROR-001, as frames 0 and 4 are. | largest difference 1.9e-7 | yes |
| FX-MIRROR-017 frame 4: Angle eased from 0 at frame 0 to 3600 at frame 4 on a curve that overshoots: at frame 2 it would be 4770, a quarter turn past a whole number of turns, is held at 3600 and is FX-MIRROR-001, as frames 0 and 4 are. | largest difference 1.9e-7 | yes |
| FX-MIRROR-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIRROR-018 frame 0: FX-MIRROR-001 moved three pixels right: the centre is per cent of the drawing, so the line moves with it and the picture is FX-MIRROR-001 moved; nothing grows, so the three columns left of the drawing stay empty. | largest difference 1.9e-7 | yes |
| FX-MIRROR-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIRROR-019 frame 0: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIRROR-019 frame 4: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIRROR-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MIRROR-020 frame 0: Centre 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIRROR-020 frame 4: Centre 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIRROR-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MIRROR-021 frame 0: Angle 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIRROR-021 frame 4: Angle 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIRROR-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MIRROR-022 frame 0: Angle -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIRROR-022 frame 4: Angle -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIRROR-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MIRROR-023 frame 0: Angle keyed to 4000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIRROR-023 frame 4: Angle keyed to 4000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIRROR-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MIRROR-024 frame 0: Centre keyed to 50, 1500 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIRROR-024 frame 4: Centre keyed to 50, 1500 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIRROR-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview changes nothing: the centre is a share of the drawing | Mirror { center: [50.0, 50.0], angle: 30.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mirror_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mirror_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mirror_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mirror_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mirror_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mirror_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mirror_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mirror_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mirror_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mirror_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mirror_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mirror_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `angle` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a centre of one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| centre 1001, 50 is refused with a sentence, and nothing changes | Mirror's center runs from -1000 to 1000, and this is 1001. | yes |
| centre 50, -1001 is refused with a sentence, and nothing changes | Mirror's center runs from -1000 to 1000, and this is -1001. | yes |
| angle 3601 is refused with a sentence, and nothing changes | Mirror's angle runs from -3600 to 3600, and this is 3601. | yes |
| angle -3601 is refused with a sentence, and nothing changes | Mirror's angle runs from -3600 to 3600, and this is -3601. | yes |
| angle keyed to 4000 is refused with a sentence, and nothing changes | Mirror's angle runs from -3600 to 3600, and this is 4000. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| angle keyed from 0 to 180 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mirror_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mirror_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mirror_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mirror_016.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

89 of 89 checks pass.
