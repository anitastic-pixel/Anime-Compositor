# B-135: Radio Waves

D-200, accepted on 2026-09-28 with the After Effects picks (A10). Every expected pixel is `Fixtures/radio_waves/expected_radio_waves.json`, written by `tools/radio_waves_reference.py` before this code existed and printed in document 25 as FX-RWAVE-001 to 027. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-RWAVE-001 to 027 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RWAVE-001 frame 0: The settings as they start: producer point (50, 50), 64 sides, a wave every 24 frames growing 5 pixels a frame, orientation 0, direction 90, velocity 0, spin 0, lifespan 96, opacity 100, fade-in 0, fade-out 48, widths 5 and 5, square, white. At frame 0 the first wave is born, a white dot 5 pixels across in the middle; at frame 2 it is a ring 10 pixels out that only the frame's corners reach; by frame 4 it has left the frame. | largest difference 1.4e-8 | yes |
| FX-RWAVE-001 frame 2: The settings as they start: producer point (50, 50), 64 sides, a wave every 24 frames growing 5 pixels a frame, orientation 0, direction 90, velocity 0, spin 0, lifespan 96, opacity 100, fade-in 0, fade-out 48, widths 5 and 5, square, white. At frame 0 the first wave is born, a white dot 5 pixels across in the middle; at frame 2 it is a ring 10 pixels out that only the frame's corners reach; by frame 4 it has left the frame. | largest difference 2.9e-8 | yes |
| FX-RWAVE-001 frame 4: The settings as they start: producer point (50, 50), 64 sides, a wave every 24 frames growing 5 pixels a frame, orientation 0, direction 90, velocity 0, spin 0, lifespan 96, opacity 100, fade-in 0, fade-out 48, widths 5 and 5, square, white. At frame 0 the first wave is born, a white dot 5 pixels across in the middle; at frame 2 it is a ring 10 pixels out that only the frame's corners reach; by frame 4 it has left the frame. | largest difference 7.3e-9 | yes |
| FX-RWAVE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-002 frame 0: Opacity 0: no waves, the drawing untouched. | largest difference 7.3e-9 | yes |
| FX-RWAVE-002 frame 2: Opacity 0: no waves, the drawing untouched. | largest difference 7.3e-9 | yes |
| FX-RWAVE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-003 frame 0: Start width 0 and end width 0: no waves, the drawing untouched. | largest difference 7.3e-9 | yes |
| FX-RWAVE-003 frame 2: Start width 0 and end width 0: no waves, the drawing untouched. | largest difference 7.3e-9 | yes |
| FX-RWAVE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-004 frame 2: "The ring": four sides, orientation 45, a wave every 10 frames growing 2 pixels a frame, lifespan 10, no fades, widths 1 and 1. At frame 2 it is a square round the middle, (8, 5), its sides 2.83 pixels out, each side's pixel-wide line shared between two rows or columns, 0.672 and 0.328, over the night and the empty half alike. | largest difference 1.4e-8 | yes |
| FX-RWAVE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-005 frame 2: As the ring, three sides, orientation 0: a triangle, its top corner at (8, 1) and its foot along y = 7, half in row 6 and half in row 7. | largest difference 2.9e-8 | yes |
| FX-RWAVE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-006 frame 2: As the ring, orientation 0 and spin 22.5 degrees a frame: at frame 2 the wave has turned 45 degrees, FX-RWAVE-004 exactly. | largest difference 1.4e-8 | yes |
| FX-RWAVE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-007 frame 2: As the ring, velocity 1 toward direction 90: at frame 2 the wave's middle has drifted 2 pixels right, the same as the ring sent from (62.5, 50). | largest difference 1.4e-8 | yes |
| FX-RWAVE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-008 frame 4: Round, 64 sides, a wave every other frame, lifespan 5, otherwise the ring: at frame 4 three waves, a dot in the middle born this frame, a ring 4 pixels out and one 8 pixels out that touches the frame's left and right edges. | largest difference 2.9e-8 | yes |
| FX-RWAVE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-009 frame 4: As FX-RWAVE-008, lifespan 3: the wave born at frame 0 has died by frame 4, and only the dot and the ring 4 pixels out are left. | largest difference 2.9e-8 | yes |
| FX-RWAVE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-010 frame 0: As the ring, start width 4, end width 0, lifespan 4: born 4 pixels wide, at frame 2, halfway through its life, it is 2 pixels wide. | largest difference 7.3e-9 | yes |
| FX-RWAVE-010 frame 2: As the ring, start width 4, end width 0, lifespan 4: born 4 pixels wide, at frame 2, halfway through its life, it is 2 pixels wide. | largest difference 2.0e-8 | yes |
| FX-RWAVE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-011 frame 1: As the ring, lifespan 4, fade-in 2, fade-out 2: at half strength at frame 1, whole at frame 2, FX-RWAVE-004, and at half again at frame 3. | largest difference 1.9e-8 | yes |
| FX-RWAVE-011 frame 2: As the ring, lifespan 4, fade-in 2, fade-out 2: at half strength at frame 1, whole at frame 2, FX-RWAVE-004, and at half again at frame 3. | largest difference 1.4e-8 | yes |
| FX-RWAVE-011 frame 3: As the ring, lifespan 4, fade-in 2, fade-out 2: at half strength at frame 1, whole at frame 2, FX-RWAVE-004, and at half again at frame 3. | largest difference 1.7e-8 | yes |
| FX-RWAVE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-012 frame 2: As the ring, widths 4 and 4, triangle: each side brightest on its line, fading to nothing 2 pixels either side. | largest difference 2.7e-8 | yes |
| FX-RWAVE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-013 frame 2: As FX-RWAVE-012, sine: softer at the middle, brighter than the triangle wherever either reaches. | largest difference 2.9e-8 | yes |
| FX-RWAVE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-014 frame 2: As the ring in orange #ffb040 at opacity 50: the square painted half strength, orange. | largest difference 7.3e-9 | yes |
| FX-RWAVE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-015 frame 0: As FX-RWAVE-008, the producer point keyed from (25, 50) at frame 0 to (75, 50) at frame 4, linear: every wave alive is sent from where the point is now, so frame 4's three are round (12, 5). | largest difference 1.4e-8 | yes |
| FX-RWAVE-015 frame 4: As FX-RWAVE-008, the producer point keyed from (25, 50) at frame 0 to (75, 50) at frame 4, linear: every wave alive is sent from where the point is now, so frame 4's three are round (12, 5). | largest difference 2.9e-8 | yes |
| FX-RWAVE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-016 frame 0: As the ring, the opacity eased from 0 at frame 0 to 100 at frame 4 past its end: frame 0 is the drawing, and frame 2 is held at 100, FX-RWAVE-004. | largest difference 7.3e-9 | yes |
| FX-RWAVE-016 frame 2: As the ring, the opacity eased from 0 at frame 0 to 100 at frame 4 past its end: frame 0 is the drawing, and frame 2 is held at 100, FX-RWAVE-004. | largest difference 1.4e-8 | yes |
| FX-RWAVE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-017 frame 2: As the ring, orientation 405, a turn and 45 degrees: FX-RWAVE-004. | largest difference 1.4e-8 | yes |
| FX-RWAVE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-018 frame 2: As the ring, the layer moved three pixels right: the square moves with it, round (11, 5). | largest difference 1.4e-8 | yes |
| FX-RWAVE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-019 frame 2: As the ring, round, after a Motion Tile at 300% by 300%, the producer point at (0, 50), the drawing's left edge, the layer moved eight pixels right: a ring round (8, 5) of the frame, drawn across the tile on the left and the drawing on the right alike. | largest difference 2.9e-8 | yes |
| FX-RWAVE-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-020 frame 2: As the ring, 4.7 sides, counted as 4: FX-RWAVE-004 exactly. | largest difference 1.4e-8 | yes |
| FX-RWAVE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-021 frame 0: Sides 2, below 3. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-RWAVE-021 frame 4: Sides 2, below 3. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-RWAVE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RWAVE-022 frame 0: Interval 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-RWAVE-022 frame 4: Interval 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-RWAVE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RWAVE-023 frame 0: Lifespan 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-RWAVE-023 frame 4: Lifespan 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-RWAVE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RWAVE-024 frame 0: Opacity keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-RWAVE-024 frame 4: Opacity keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-RWAVE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RWAVE-025 frame 0: Start width -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-RWAVE-025 frame 4: Start width -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-RWAVE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RWAVE-026 frame 0: Profile "bell", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-RWAVE-026 frame 4: Profile "bell", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-RWAVE-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RWAVE-027 frame 0: Colour "#fff", written in three digits, not six. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-RWAVE-027 frame 4: Colour "#fff", written in three digits, not six. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-RWAVE-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| as added, it never grows the drawing's bounds | 0 | yes |
| expansion 1000, the most, does not grow them either | 0 | yes |
| end width 1000, the most, does not grow them either | 0 | yes |
| a half-size draft preview halves the expansion, 5 to 2.5, the velocity, 3 to 1.5, and the widths, 5 to 2.5 and 8 to 4, and keeps the rest | RadioWaves { producer_point: [50.0, 50.0], sides: 64.0, interval: 24.0, expansion: 2.5, orientation: 0.0, direction: 90.0, velocity: 1.5, spin: 2.0, lifespan: 96.0, opacity: 100.0, fade_in_time: 0.0, fade_out_time: 48.0, start_width: 2.5, end_width: 4.0, profile: "square", color: "#ffffff", frame: 0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rwave_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwave_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwave_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwave_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwave_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwave_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwave_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwave_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwave_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwave_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwave_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwave_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwave_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwave_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwave_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| the frame the waves are drawn at is never saved | None | yes |
| a file with no `color` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a producer point of one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with sides that are a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| producer x 1000.5 is refused with a sentence, and nothing changes | Radio Waves's producer point runs from -1000 to 1000, and this is 1000.5. | yes |
| sides 2.9 is refused with a sentence, and nothing changes | Radio Waves's sides runs from 3 to 64, and this is 2.9. | yes |
| sides 64.5 is refused with a sentence, and nothing changes | Radio Waves's sides runs from 3 to 64, and this is 64.5. | yes |
| interval 0.9 is refused with a sentence, and nothing changes | Radio Waves's interval runs from 1 to 1000, and this is 0.9. | yes |
| expansion -1 is refused with a sentence, and nothing changes | Radio Waves's expansion runs from 0 to 1000, and this is -1. | yes |
| orientation 3600.5 is refused with a sentence, and nothing changes | Radio Waves's orientation runs from -3600 to 3600, and this is 3600.5. | yes |
| direction -3601 is refused with a sentence, and nothing changes | Radio Waves's direction runs from -3600 to 3600, and this is -3601. | yes |
| velocity 1000.5 is refused with a sentence, and nothing changes | Radio Waves's velocity runs from 0 to 1000, and this is 1000.5. | yes |
| spin 361 is refused with a sentence, and nothing changes | Radio Waves's spin runs from -360 to 360, and this is 361. | yes |
| lifespan 0.5 is refused with a sentence, and nothing changes | Radio Waves's lifespan runs from 1 to 1000, and this is 0.5. | yes |
| opacity 101 is refused with a sentence, and nothing changes | Radio Waves's opacity runs from 0 to 100, and this is 101. | yes |
| fade-in time -1 is refused with a sentence, and nothing changes | Radio Waves's fade in time runs from 0 to 1000, and this is -1. | yes |
| fade-out time 1000.5 is refused with a sentence, and nothing changes | Radio Waves's fade out time runs from 0 to 1000, and this is 1000.5. | yes |
| start width -1 is refused with a sentence, and nothing changes | Radio Waves's start width runs from 0 to 1000, and this is -1. | yes |
| end width 1000.5 is refused with a sentence, and nothing changes | Radio Waves's end width runs from 0 to 1000, and this is 1000.5. | yes |
| profile "bell" is refused with a sentence, and nothing changes | Radio Waves' profile is "square", "triangle" or "sine", and this is "bell". | yes |
| colour "#fff" is refused with a sentence, and nothing changes | Radio Waves's colour is written #rrggbb, and this is "#fff". | yes |
| sides keyed to 70 is refused with a sentence, and nothing changes | Radio Waves's sides runs from 3 to 64, and this is 70. | yes |
| every number at its top, the last profile, is taken | taken | yes |
| every number at its bottom, the other profile, is taken | taken | yes |
| the producer point keyed across the layer is taken | taken | yes |
| the spin keyed from 0 to 90 is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a badge, in `verification/B-135 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the badge with no effect, draws cleanly | [] | yes |
| as_added_frame_8.png, as it is added, at frame 8: one ring 40 pixels out, over the badge and the empty space above it; draws cleanly, changes some pixels, paints [(120, 50), (80, 10)] and leaves [(80, 50), (5, 5)] exactly | []; 1496 changed; painted [[255, 255, 255, 255], [255, 255, 255, 255]]; left [true, true] | yes |
| as_added_frame_30.png, at frame 30: the second ring 30 pixels out, the first gone past the edges; draws cleanly, changes some pixels, paints [(110, 50)] and leaves [(120, 50), (80, 50), (5, 5)] exactly | []; 1144 changed; painted [[255, 255, 255, 255]]; left [true, true, true] | yes |
| shockwave.png, a shockwave at frame 8: one ring, 10 wide thinning to 1 over 20 frames, triangle, fading out; draws cleanly, changes some pixels, paints [(128, 50)] and leaves [(80, 50), (5, 5)] exactly | []; 1888 changed; painted [[255, 255, 255, 213]]; left [true, true] | yes |
| turning_squares.png, turning squares at frame 20: 4 sides, a new one every 6 frames, spin 4 a frame; draws cleanly, changes some pixels, paints [] and leaves [(80, 50), (5, 5)] exactly | []; 2054 changed; painted []; left [true, true] | yes |
| drifting_triangles.png, triangles sent from (25, 50) at frame 24, drifting right 2 pixels a frame; draws cleanly, changes some pixels, paints [(40, 50)] and leaves [(5, 5)] exactly | []; 1297 changed; painted [[255, 255, 255, 255]]; left [true] | yes |
| orange_sine.png, orange #ffb040, sine, 8 wide, opacity 80, at frame 30; draws cleanly, changes some pixels, paints [(98, 50)] redder than blue and leaves [(80, 50), (110, 50)] exactly | []; 5168 changed; painted [[244, 159, 60, 255]]; left [true, true] | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rwave_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rwave_008.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rwave_019.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

126 of 126 checks pass.
