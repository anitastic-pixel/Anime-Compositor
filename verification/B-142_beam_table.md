# B-142: Beam

D-207, accepted on 2026-09-28 with the After Effects picks (B5). Every expected pixel is `Fixtures/beam/expected_beam.json`, written by `tools/beam_reference.py` before this code existed and printed in document 25 as FX-BEAM-001 to 029. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-BEAM-001 to 029 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BEAM-001 frame 0: From (25, 45) to (75, 45) per cent, from 4 to 12 pixels across the middle of row 4, length 100, time 0, 4 pixels thick at both ends, softness 0, white inside, blue #3c8cff outside, composite on: a hard beam over the night sky and the empty half, white along row 4 turning blue at its edges, rows 3 to 5 wholly covered and rows 2 and 6 half, its ends round. | largest difference 2.2e-8 | yes |
| FX-BEAM-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-002 frame 0: Length 25, time 0: the first quarter of the line, from 4 to 6 pixels across. | largest difference 2.2e-8 | yes |
| FX-BEAM-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-003 frame 0: Length 25, time 50: the quarter in the middle of the line. | largest difference 2.2e-8 | yes |
| FX-BEAM-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-004 frame 0: Length 25, time 100: the last quarter, from 10 to 12 pixels across. | largest difference 2.1e-8 | yes |
| FX-BEAM-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-005 frame 0: Length 0: a round dot 4 pixels across at the start. | largest difference 1.9e-8 | yes |
| FX-BEAM-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-006 frame 0: 2 pixels thick at the start and 8 at the end: the beam widens along the line. | largest difference 3.0e-8 | yes |
| FX-BEAM-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-007 frame 0: Softness 50: the edges fade over two pixels. | largest difference 2.9e-8 | yes |
| FX-BEAM-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-008 frame 0: Softness 100: solid only along its middle, fading out to twice its thickness. | largest difference 3.2e-8 | yes |
| FX-BEAM-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-009 frame 0: Composite off: the beam alone, the night sky gone. | largest difference 2.1e-8 | yes |
| FX-BEAM-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-010 frame 0: Inside #ffe080, a pale yellow, outside #ff3020, a red. | largest difference 2.6e-8 | yes |
| FX-BEAM-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-011 frame 0: Both thicknesses 0: nothing is drawn; the drawing, untouched. | largest difference 7.3e-9 | yes |
| FX-BEAM-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-012 frame 0: Both thicknesses 0 with composite off: nothing at all. | largest difference 0.0e0 | yes |
| FX-BEAM-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-013 frame 0: From the top-left corner to the bottom-right, slanted. | largest difference 2.7e-8 | yes |
| FX-BEAM-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-014 frame 0: The start and the end the same point, (50, 50): a round dot there. | largest difference 1.9e-8 | yes |
| FX-BEAM-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-015 frame 0: Length 25, time keyed from 0 at frame 0 to 100 at frame 4, linear: the shot travels, frame 0 FX-BEAM-002, frame 2 FX-BEAM-003 and frame 4 FX-BEAM-004. | largest difference 2.2e-8 | yes |
| FX-BEAM-015 frame 2: Length 25, time keyed from 0 at frame 0 to 100 at frame 4, linear: the shot travels, frame 0 FX-BEAM-002, frame 2 FX-BEAM-003 and frame 4 FX-BEAM-004. | largest difference 2.2e-8 | yes |
| FX-BEAM-015 frame 4: Length 25, time keyed from 0 at frame 0 to 100 at frame 4, linear: the shot travels, frame 0 FX-BEAM-002, frame 2 FX-BEAM-003 and frame 4 FX-BEAM-004. | largest difference 2.1e-8 | yes |
| FX-BEAM-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-016 frame 0: Start thickness keyed from 4 at frame 0 to 500 at frame 4, eased past its end: frame 2 would pass 500, is held at 500, and is start thickness 500. | largest difference 2.2e-8 | yes |
| FX-BEAM-016 frame 2: Start thickness keyed from 4 at frame 0 to 500 at frame 4, eased past its end: frame 2 would pass 500, is held at 500, and is start thickness 500. | largest difference 3.0e-8 | yes |
| FX-BEAM-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-017 frame 0: FX-BEAM-001 moved three pixels right: the beam moves with the drawing. | largest difference 2.2e-8 | yes |
| FX-BEAM-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-018 frame 0: After a Motion Tile that grows the layer: the points are the drawing's own, so the frame is FX-BEAM-001's. | largest difference 2.2e-8 | yes |
| FX-BEAM-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-019 frame 0: Starting past the drawing's left edge, from (-50, 45): the beam comes in from outside it. | largest difference 2.2e-8 | yes |
| FX-BEAM-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BEAM-020 frame 0: Length 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-020 frame 4: Length 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEAM-021 frame 0: Time -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-021 frame 4: Time -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEAM-022 frame 0: Start thickness 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-022 frame 4: Start thickness 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEAM-023 frame 0: End thickness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-023 frame 4: End thickness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEAM-024 frame 0: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-024 frame 4: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEAM-025 frame 0: A start 1001 per cent across, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-025 frame 4: A start 1001 per cent across, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEAM-026 frame 0: An inside colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-026 frame 4: An inside colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEAM-027 frame 0: An outside colour "blue", a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-027 frame 4: An outside colour "blue", a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEAM-028 frame 0: Composite "yes", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-028 frame 4: Composite "yes", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BEAM-029 frame 0: Time keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-029 frame 4: Time keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-BEAM-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it never grows the layer: it declares no growth | 0 | yes |
| a half-size draft preview halves the two thicknesses, and keeps the points, length, time and softness | Beam { start: [10.0, 50.0], end: [90.0, 50.0], length: 40.0, time: 30.0, start_thickness: 3.0, end_thickness: 10.0, softness: 50.0, inside_color: "#ffffff", outside_color: "#3c8cff", composite: "on" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_beam_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_beam_010.json, its outside colour written in capitals, is saved in small letters, as Snowfall's is | "#ff3020" | yes |
| a file with no `composite` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a start that is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an inside colour that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| length -1 is refused with a sentence, and nothing changes | Beam's length runs from 0 to 100, and this is -1. | yes |
| length 101 is refused with a sentence, and nothing changes | Beam's length runs from 0 to 100, and this is 101. | yes |
| time -1 is refused with a sentence, and nothing changes | Beam's time runs from 0 to 100, and this is -1. | yes |
| time 101 is refused with a sentence, and nothing changes | Beam's time runs from 0 to 100, and this is 101. | yes |
| starting thickness -1 is refused with a sentence, and nothing changes | Beam's start thickness runs from 0 to 500, and this is -1. | yes |
| ending thickness 501 is refused with a sentence, and nothing changes | Beam's end thickness runs from 0 to 500, and this is 501. | yes |
| softness 101 is refused with a sentence, and nothing changes | Beam's softness runs from 0 to 100, and this is 101. | yes |
| start -1001 across is refused with a sentence, and nothing changes | Beam's start runs from -1000 to 1000, and this is -1001. | yes |
| start 1001 down is refused with a sentence, and nothing changes | Beam's start runs from -1000 to 1000, and this is 1001. | yes |
| inside colour "#12345" is refused with a sentence, and nothing changes | Beam's inside colour is written #rrggbb, and this is "#12345". | yes |
| outside colour "blue" is refused with a sentence, and nothing changes | Beam's outside colour is written #rrggbb, and this is "blue". | yes |
| composite "yes" is refused with a sentence, and nothing changes | Beam's composite is "on" or "off", and this is "yes". | yes |
| composite "On", written with a capital is refused with a sentence, and nothing changes | Beam's composite is "on" or "off", and this is "On". | yes |
| time keyed to 101 is refused with a sentence, and nothing changes | Beam's time runs from 0 to 100, and this is 101. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| time keyed from 0 to 100 is taken | taken | yes |
| start keyed from (0, 0) to (100, 100) is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_beam_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_beam_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_beam_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_beam_015.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a made-up night, in `verification/B-142 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the drawing with no effect, draws cleanly | [] | yes |
| as_it_starts.png, as it starts, length 25, time 0: the first quarter of the line lit, white in the middle at 32 across; unchanged at (80, 50); nothing changed further than 8 pixels from the lit stretch; draws cleanly | [], [241, 244, 255, 255] at (32, 50), 496 pixels changed | yes |
| time_50.png, time 50: the lit quarter in the middle of the line, white at 80 across, the start dark; unchanged at (32, 50); nothing changed further than 8 pixels from the lit stretch; draws cleanly | [], [241, 244, 255, 255] at (80, 50), 496 pixels changed | yes |
| time_100.png, time 100: the lit quarter at the end of the line, white at 128 across; unchanged at (80, 50); nothing changed further than 8 pixels from the lit stretch; draws cleanly | [], [241, 244, 255, 255] at (128, 50), 496 pixels changed | yes |
| length_100.png, length 100: the whole line lit, white at 32, 80 and 128 across; nothing changed further than 8 pixels from it; draws cleanly | [], 1648 pixels changed | yes |
| thickness_2_to_20.png, from 2 pixels thick at the start to 20 at the end: more rows lit at 136 across than at 80, and more at 80 than at 24; draws cleanly | [], rows changed at 24, 80 and 136 across: 4, 16, 28 | yes |
| softness_0.png and softness_100.png: both white along the line at 80 across; 6 rows above it untouched at softness 0 and lit at softness 100, which reaches twice as far; both draw cleanly | [] [], 1084 and 2256 pixels changed | yes |
| yellow_in_red.png, yellow #ffe080 in red #ff3020 from low left to high right, 12 thick: the middle at 80 across near the yellow; the corners top left and bottom right untouched; draws cleanly | [], [255, 213, 121, 255] in the middle | yes |
| composite_off.png, Composite On Original off: the night gone, the beam alone, white along the line and clear more than 8 pixels from it; draws cleanly | [], [241, 244, 255, 255] on the line, 14352 of 16000 pixels clear | yes |

## Result

137 of 137 checks pass.
