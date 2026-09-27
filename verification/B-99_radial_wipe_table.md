# B-99: radial wipe

D-156, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the twenty-third of the third batch. Every expected pixel is `Fixtures/radial_wipe/expected_radial_wipe.json`, written by `tools/radial_wipe_reference.py` before this code existed and printed in document 25 as FX-RWIPE-001 to 029. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-RWIPE-001 to 029 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RWIPE-001 frame 0: The settings as they start: completion 0, start angle 0, centre 50, 50, clockwise, feather 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-RWIPE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-002 frame 0: Completion 25: a quarter turn swept clockwise from straight up about the middle, the point (8, 5), so the top right quarter, columns 8 to 15 of rows 0 to 4, is transparent, and every other pixel is kept exactly. | largest difference 1.9e-7 | yes |
| FX-RWIPE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-003 frame 0: Completion 50: half a turn swept, from straight up round to straight down, so the right half, columns 8 to 15, is transparent; the line down column 8 goes with it, and the left half is kept exactly. | largest difference 1.9e-7 | yes |
| FX-RWIPE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-004 frame 0: Completion 75: three quarters swept, so only the top left quarter, columns 0 to 7 of rows 0 to 4, the soft edge among it, is left. | largest difference 1.9e-7 | yes |
| FX-RWIPE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-005 frame 0: Completion 100: every pixel transparent, all four channels 0. | largest difference 0.0e0 | yes |
| FX-RWIPE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-006 frame 0: Counterclockwise, completion 25: the quarter turn swept the other way from straight up, so the top left quarter, columns 0 to 7 of rows 0 to 4, is transparent instead. | largest difference 1.9e-7 | yes |
| FX-RWIPE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-007 frame 0: Both, completion 50: the wipe sweeps both ways from straight up at once, a quarter turn each way, so the top half, rows 0 to 4, is transparent and the line along row 5 and everything below it is kept. | largest difference 1.9e-7 | yes |
| FX-RWIPE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-008 frame 0: Start angle 90, completion 25: the quarter turn swept clockwise from straight right, so the bottom right quarter, columns 8 to 15 of rows 5 to 9, is transparent. | largest difference 1.9e-7 | yes |
| FX-RWIPE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-009 frame 0: Start angle 30, completion 50: half a turn swept from a line 30 degrees clockwise of straight up, so the half of the drawing right of a slanted line through the middle is transparent: 76 pixels, the top of column 8 kept and the bottom of column 7 gone. | largest difference 1.9e-7 | yes |
| FX-RWIPE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-010 frame 0: Feather 90, completion 50: the sweeping edge, straight down, fades over 90 degrees, so pixels less than 135 degrees round from straight up are transparent, those more than 225 round are kept exactly, and between, down and either side of the line down column 8, each keeps a part of its colour and covering; the start line, straight up, stays a hard edge. | largest difference 1.9e-7 | yes |
| FX-RWIPE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-011 frame 0: Feather 360, completion 50: the fade is a whole turn wide, so each pixel keeps exactly its angle round from straight up over 360 of itself, a fan clear just right of straight up and nearly whole just left of it. | largest difference 2.1e-7 | yes |
| FX-RWIPE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-012 frame 0: Counterclockwise, start angle 180, feather 60, completion 50: swept from straight down back round through the right, so the right half is transparent except near the top, where a soft edge 60 degrees wide straddles straight up; the start line, straight down, stays hard. | largest difference 1.9e-7 | yes |
| FX-RWIPE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-013 frame 0: Centre 0, 0, the top left corner, completion 40: every pixel lies between 90 and 180 degrees round from that corner, so the sweep, 144 degrees, takes every pixel less than 54 degrees down from straight right of the corner, the top right of the drawing, and keeps the pixels nearer straight down from it, the bottom left, exactly. | largest difference 1.9e-7 | yes |
| FX-RWIPE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-014 frame 0: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: the clock hand sweeps round once, frame 0 the drawing, frames 1 to 3 FX-RWIPE-002, 003 and 004, and frame 4 FX-RWIPE-005. | largest difference 1.9e-7 | yes |
| FX-RWIPE-014 frame 1: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: the clock hand sweeps round once, frame 0 the drawing, frames 1 to 3 FX-RWIPE-002, 003 and 004, and frame 4 FX-RWIPE-005. | largest difference 1.9e-7 | yes |
| FX-RWIPE-014 frame 2: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: the clock hand sweeps round once, frame 0 the drawing, frames 1 to 3 FX-RWIPE-002, 003 and 004, and frame 4 FX-RWIPE-005. | largest difference 1.9e-7 | yes |
| FX-RWIPE-014 frame 3: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: the clock hand sweeps round once, frame 0 the drawing, frames 1 to 3 FX-RWIPE-002, 003 and 004, and frame 4 FX-RWIPE-005. | largest difference 1.9e-7 | yes |
| FX-RWIPE-014 frame 4: Completion keyed from 0 at frame 0 to 100 at frame 4, linear: the clock hand sweeps round once, frame 0 the drawing, frames 1 to 3 FX-RWIPE-002, 003 and 004, and frame 4 FX-RWIPE-005. | largest difference 0.0e0 | yes |
| FX-RWIPE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-015 frame 0: Start angle keyed from 0 at frame 0 to 360 at frame 4 with completion 25, linear: the swept quarter turns round the middle, frame 0 FX-RWIPE-002, frame 1 FX-RWIPE-008, frame 2 the bottom left quarter, and frame 4 back to FX-RWIPE-002. | largest difference 1.9e-7 | yes |
| FX-RWIPE-015 frame 1: Start angle keyed from 0 at frame 0 to 360 at frame 4 with completion 25, linear: the swept quarter turns round the middle, frame 0 FX-RWIPE-002, frame 1 FX-RWIPE-008, frame 2 the bottom left quarter, and frame 4 back to FX-RWIPE-002. | largest difference 1.9e-7 | yes |
| FX-RWIPE-015 frame 2: Start angle keyed from 0 at frame 0 to 360 at frame 4 with completion 25, linear: the swept quarter turns round the middle, frame 0 FX-RWIPE-002, frame 1 FX-RWIPE-008, frame 2 the bottom left quarter, and frame 4 back to FX-RWIPE-002. | largest difference 1.9e-7 | yes |
| FX-RWIPE-015 frame 4: Start angle keyed from 0 at frame 0 to 360 at frame 4 with completion 25, linear: the swept quarter turns round the middle, frame 0 FX-RWIPE-002, frame 1 FX-RWIPE-008, frame 2 the bottom left quarter, and frame 4 back to FX-RWIPE-002. | largest difference 1.9e-7 | yes |
| FX-RWIPE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-016 frame 0: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4 with completion 40, linear: frame 0 is completion 40 about the middle, frame 2 about 25, 25, and frame 4 FX-RWIPE-013. | largest difference 1.9e-7 | yes |
| FX-RWIPE-016 frame 2: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4 with completion 40, linear: frame 0 is completion 40 about the middle, frame 2 about 25, 25, and frame 4 FX-RWIPE-013. | largest difference 1.9e-7 | yes |
| FX-RWIPE-016 frame 4: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4 with completion 40, linear: frame 0 is completion 40 about the middle, frame 2 about 25, 25, and frame 4 FX-RWIPE-013. | largest difference 1.9e-7 | yes |
| FX-RWIPE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-017 frame 0: Feather keyed from 0 at frame 0 to 360 at frame 4 with completion 50, linear: frame 0 is FX-RWIPE-003, frame 2 feather 180, and frame 4 FX-RWIPE-011. | largest difference 1.9e-7 | yes |
| FX-RWIPE-017 frame 2: Feather keyed from 0 at frame 0 to 360 at frame 4 with completion 50, linear: frame 0 is FX-RWIPE-003, frame 2 feather 180, and frame 4 FX-RWIPE-011. | largest difference 1.9e-7 | yes |
| FX-RWIPE-017 frame 4: Feather keyed from 0 at frame 0 to 360 at frame 4 with completion 50, linear: frame 0 is FX-RWIPE-003, frame 2 feather 180, and frame 4 FX-RWIPE-011. | largest difference 2.1e-7 | yes |
| FX-RWIPE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-018 frame 0: Completion held at 25 from frame 0, then 75 from frame 3: frames 0 and 2 are FX-RWIPE-002, frames 3 and 4 FX-RWIPE-004. | largest difference 1.9e-7 | yes |
| FX-RWIPE-018 frame 2: Completion held at 25 from frame 0, then 75 from frame 3: frames 0 and 2 are FX-RWIPE-002, frames 3 and 4 FX-RWIPE-004. | largest difference 1.9e-7 | yes |
| FX-RWIPE-018 frame 3: Completion held at 25 from frame 0, then 75 from frame 3: frames 0 and 2 are FX-RWIPE-002, frames 3 and 4 FX-RWIPE-004. | largest difference 1.9e-7 | yes |
| FX-RWIPE-018 frame 4: Completion held at 25 from frame 0, then 75 from frame 3: frames 0 and 2 are FX-RWIPE-002, frames 3 and 4 FX-RWIPE-004. | largest difference 1.9e-7 | yes |
| FX-RWIPE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-019 frame 0: Completion eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-RWIPE-005; frame 0 is the drawing. | largest difference 1.9e-7 | yes |
| FX-RWIPE-019 frame 2: Completion eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-RWIPE-005; frame 0 is the drawing. | largest difference 0.0e0 | yes |
| FX-RWIPE-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-020 frame 0: FX-RWIPE-002 moved three pixels right: the same, moved; the wipe moves with the drawing. | largest difference 1.9e-7 | yes |
| FX-RWIPE-020 frame 3: FX-RWIPE-002 moved three pixels right: the same, moved; the wipe moves with the drawing. | largest difference 1.9e-7 | yes |
| FX-RWIPE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-021 frame 0: A directional blur, direction 90 and length 4, then completion 70 about centre 25, 25, all moved three pixels right: the blur grew the layer two pixels on every side, the two grown columns left of the drawing show in columns 1 and 2 and are wiped by the same rule, and the centre is still the drawing's own point (4, 2.5), not a point of the grown layer. | largest difference 1.9e-7 | yes |
| FX-RWIPE-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWIPE-022 frame 0: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RWIPE-022 frame 4: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RWIPE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RWIPE-023 frame 0: Start angle -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RWIPE-023 frame 4: Start angle -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RWIPE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RWIPE-024 frame 0: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RWIPE-024 frame 4: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RWIPE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RWIPE-025 frame 0: Feather 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RWIPE-025 frame 4: Feather 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RWIPE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RWIPE-026 frame 0: Feather -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RWIPE-026 frame 4: Feather -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RWIPE-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RWIPE-027 frame 0: Wipe "spiral", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RWIPE-027 frame 4: Wipe "spiral", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RWIPE-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RWIPE-028 frame 0: Wipe "Clockwise", in capitals, which is kept as written and is not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RWIPE-028 frame 4: Wipe "Clockwise", in capitals, which is kept as written and is not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RWIPE-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RWIPE-029 frame 0: Completion keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RWIPE-029 frame 4: Completion keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RWIPE-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview changes nothing: the feather is in degrees, not pixels | RadialWipe { completion: 50.0, start_angle: 30.0, center: [50.0, 50.0], wipe: "clockwise", feather: 90.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rwipe_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwipe_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwipe_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwipe_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwipe_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwipe_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwipe_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwipe_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwipe_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwipe_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwipe_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwipe_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwipe_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwipe_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwipe_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `wipe` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a centre of one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| completion 101 is refused with a sentence, and nothing changes | Radial Wipe's completion runs from 0 to 100, and this is 101. | yes |
| start angle -3601 is refused with a sentence, and nothing changes | Radial Wipe's start angle runs from -3600 to 3600, and this is -3601. | yes |
| centre 1001, 50 is refused with a sentence, and nothing changes | Radial Wipe's center runs from -1000 to 1000, and this is 1001. | yes |
| feather 361 is refused with a sentence, and nothing changes | Radial Wipe's feather runs from 0 to 360, and this is 361. | yes |
| feather -1 is refused with a sentence, and nothing changes | Radial Wipe's feather runs from 0 to 360, and this is -1. | yes |
| wipe "spiral" is refused with a sentence, and nothing changes | Radial Wipe's wipe is "clockwise", "counterclockwise" or "both", and this is "spiral". | yes |
| wipe "Clockwise" is refused with a sentence, and nothing changes | Radial Wipe's wipe is "clockwise", "counterclockwise" or "both", and this is "Clockwise". | yes |
| completion keyed to 150 is refused with a sentence, and nothing changes | Radial Wipe's completion runs from 0 to 100, and this is 150. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| completion keyed from 0 to 100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rwipe_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rwipe_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rwipe_020.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rwipe_021.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

117 of 117 checks pass.
