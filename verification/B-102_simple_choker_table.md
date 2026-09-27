# B-102: simple choker

D-159, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the twenty-sixth of the third batch. Every expected pixel is `Fixtures/simple_choker/expected_simple_choker.json`, written by `tools/simple_choker_reference.py` before this code existed and printed in document 25 as FX-CHOKE-001 to 017. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-CHOKE-001 to 017 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CHOKE-001 frame 0: The settings as they start: choke 0. The output is the drawing, untouched, on every frame, and nothing grows. | largest difference 1.9e-7 | yes |
| FX-CHOKE-001 frame 4: The settings as they start: choke 0. The output is the drawing, untouched, on every frame, and nothing grows. | largest difference 1.9e-7 | yes |
| FX-CHOKE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHOKE-002 frame 0: Choke 1 shrinks by the four pixels beside each pixel: every pixel with an empty pixel or the drawing's edge beside it loses all its covering, so the box's line goes, and with it the soft pixel at (11, 4), the speck at (13, 8), the four skin pixels round the hole at (7, 4) and the blue block's outer pixels, leaving only (1, 7) and (1, 8) of it; the line at (10, 4), beside the soft pixel, keeps half its covering, 128/255, in its own colour; the half-covered skin at (8, 6) is unchanged, and its three skin neighbours take its half covering. Empty pixels stay empty, and nothing grows. | largest difference 1.9e-7 | yes |
| FX-CHOKE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHOKE-003 frame 0: Choke 1.5: the disc now holds the four diagonal pixels too, a square of nine. The line at (10, 4) now reaches the empty (11, 3) and (11, 5) and goes; the hole takes all eight of its neighbours; the blue block still keeps (1, 7) and (1, 8); and the skin is only ever kept or thinned, never thickened, against FX-CHOKE-002. | largest difference 1.9e-7 | yes |
| FX-CHOKE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHOKE-004 frame 0: Choke 0.5: the disc holds only the pixel itself, so the output is the drawing exactly. | largest difference 1.9e-7 | yes |
| FX-CHOKE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHOKE-005 frame 0: Choke -0.5: the same, spreading: the disc holds only the pixel itself, nothing grows, and the output is the drawing. | largest difference 1.9e-7 | yes |
| FX-CHOKE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHOKE-006 frame 0: Choke -1 spreads by the four pixels beside each pixel, and the layer grows by 1. Each empty pixel beside a covered one takes that covering in full and the covered neighbours' covering-weighted average colour: the hole at (7, 4) fills with skin; (3, 6) and (3, 7), between the blue block and the box's line, take half blue and half line; (12, 4), beside only the soft pixel, takes the line at half covering; the speck grows to a cross of line. A covered pixel keeps its own colour and takes the largest covering in reach, so the soft pixel at (11, 4) becomes the line in full and the half-covered skin at (8, 6) the skin in full. | largest difference 1.9e-7 | yes |
| FX-CHOKE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHOKE-007 frame 0: Choke -1.5: the square of nine, so the corners fill too: (3, 1), diagonal from the box's corner, takes the line in full, and (3, 8) takes three parts blue to one part line, where FX-CHOKE-006 gives it blue alone. The layer still grows by 1. | largest difference 1.9e-7 | yes |
| FX-CHOKE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHOKE-008 frame 0: Choke -2: the box's line reaches two pixels out, to row 0 and columns 2 and 12, its corners rounded, so (2, 0), two across and two up from the box's corner, stays empty; the gap between the blue block and the box fills; the layer grows by 2. | largest difference 2.0e-7 | yes |
| FX-CHOKE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHOKE-009 frame 0: Choke keyed from 0 at frame 0 to 3 at frame 4, linear: frame 0 is the drawing, frame 2 is choke 1.5, FX-CHOKE-003, and at frame 4, choke 3, nothing is left: no part of the drawing is seven pixels across both ways. | largest difference 1.9e-7 | yes |
| FX-CHOKE-009 frame 2: Choke keyed from 0 at frame 0 to 3 at frame 4, linear: frame 0 is the drawing, frame 2 is choke 1.5, FX-CHOKE-003, and at frame 4, choke 3, nothing is left: no part of the drawing is seven pixels across both ways. | largest difference 1.9e-7 | yes |
| FX-CHOKE-009 frame 4: Choke keyed from 0 at frame 0 to 3 at frame 4, linear: frame 0 is the drawing, frame 2 is choke 1.5, FX-CHOKE-003, and at frame 4, choke 3, nothing is left: no part of the drawing is seven pixels across both ways. | largest difference 0.0e0 | yes |
| FX-CHOKE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHOKE-010 frame 0: Choke keyed from 0 at frame 0 to -100 at frame 4, eased past its end: frame 2 would be below -100 and is held at -100, so it is frame 4. At spread 100 the disc about every pixel of the frame holds the whole drawing, so the whole frame is covered in full: each covered pixel in its own colour, and every empty one in one colour, the whole drawing's covering-weighted average. | largest difference 1.9e-7 | yes |
| FX-CHOKE-010 frame 2: Choke keyed from 0 at frame 0 to -100 at frame 4, eased past its end: frame 2 would be below -100 and is held at -100, so it is frame 4. At spread 100 the disc about every pixel of the frame holds the whole drawing, so the whole frame is covered in full: each covered pixel in its own colour, and every empty one in one colour, the whole drawing's covering-weighted average. | largest difference 1.9e-7 | yes |
| FX-CHOKE-010 frame 4: Choke keyed from 0 at frame 0 to -100 at frame 4, eased past its end: frame 2 would be below -100 and is held at -100, so it is frame 4. At spread 100 the disc about every pixel of the frame holds the whole drawing, so the whole frame is covered in full: each covered pixel in its own colour, and every empty one in one colour, the whole drawing's covering-weighted average. | largest difference 1.9e-7 | yes |
| FX-CHOKE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHOKE-011 frame 0: Choke 100, the most: every disc reaches past the drawing's edge, where the covering counts as 0, so nothing is left. | largest difference 0.0e0 | yes |
| FX-CHOKE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHOKE-012 frame 0: FX-CHOKE-002 moved three pixels right: the choke is worked in the drawing's own space, so it moves with it, and the drawing's left edge still counts as empty beside the blue block. | largest difference 1.9e-7 | yes |
| FX-CHOKE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHOKE-013 frame 0: FX-CHOKE-008 moved three pixels right: the layer grew by 2, and the two columns left of the drawing show the grown pixels, into which the blue block spreads, while the column left of those stays empty. | largest difference 2.0e-7 | yes |
| FX-CHOKE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHOKE-014 frame 0: Choke 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHOKE-014 frame 4: Choke 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHOKE-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHOKE-015 frame 0: Choke -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHOKE-015 frame 4: Choke -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHOKE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHOKE-016 frame 0: Choke keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHOKE-016 frame 4: Choke keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHOKE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHOKE-017 frame 0: Choke keyed from -120 at frame 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHOKE-017 frame 4: Choke keyed from -120 at frame 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHOKE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| a spread of 2.5 grows the drawing's bounds by 2 pixels | 2 | yes |
| a spread of 100 grows them by 100 | 100 | yes |
| a shrink grows them by nothing | 0 | yes |
| choke 0 grows them by nothing | 0 | yes |
| a half-size draft preview halves the choke, -4 to -2 | SimpleChoker { choke: -2.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_choke_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_choke_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_choke_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_choke_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_choke_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_choke_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_choke_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_choke_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_choke_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `choke` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a choke that is a list is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| choke 101 is refused with a sentence, and nothing changes | Simple Choker's choke runs from -100 to 100, and this is 101. | yes |
| choke -101 is refused with a sentence, and nothing changes | Simple Choker's choke runs from -100 to 100, and this is -101. | yes |
| choke keyed to 150 is refused with a sentence, and nothing changes | Simple Choker's choke runs from -100 to 100, and this is 150. | yes |
| choke keyed from -120 is refused with a sentence, and nothing changes | Simple Choker's choke runs from -100 to 100, and this is -120. | yes |
| choke at its bottom, -100, is taken | taken | yes |
| choke at its top, 100, is taken | taken | yes |
| choke keyed from 0 to -100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_choke_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_choke_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_choke_010.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |

## Result

70 of 70 checks pass.
