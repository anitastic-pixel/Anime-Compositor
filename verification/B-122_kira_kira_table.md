# B-122: kira-kira

D-186, accepted by the owner on 2026-09-28. Every expected pixel is `Fixtures/kira_kira/expected_kira_kira.json`, written by `tools/kira_kira_reference.py` before this code existed and printed in document 25 as FX-KIRA-001 to 029. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-KIRA-001 to 029 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-KIRA-001 frame 0: The settings as they start: threshold 95, spacing 64, density 60, size 40, a star, angle 0, twinkle 100, period 24, seed 0, opacity 100, white. The frame is one cell, and it is chosen, so its three white glints, the point, the square and the soft one, share one star at their middle, (8.5, 4.17), on the clear; the grey, 60 %, is no highlight, nor is the gold, bright but coloured. At frame 0 the star is early in its twinkle and small; four frames later it is larger and brighter. | largest difference 1.5e-7 | yes |
| FX-KIRA-001 frame 4: The settings as they start: threshold 95, spacing 64, density 60, size 40, a star, angle 0, twinkle 100, period 24, seed 0, opacity 100, white. The frame is one cell, and it is chosen, so its three white glints, the point, the square and the soft one, share one star at their middle, (8.5, 4.17), on the clear; the grey, 60 %, is no highlight, nor is the gold, bright but coloured. At frame 0 the star is early in its twinkle and small; four frames later it is larger and brighter. | largest difference 2.0e-7 | yes |
| FX-KIRA-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-002 frame 0: Opacity 0: the drawing, untouched, and nothing grows. | largest difference 1.5e-7 | yes |
| FX-KIRA-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-003 frame 0: Size 0: the drawing, untouched, and nothing grows. | largest difference 1.5e-7 | yes |
| FX-KIRA-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-004 frame 0: Density 0: no cell is chosen; the drawing, untouched, and nothing grows. | largest difference 1.5e-7 | yes |
| FX-KIRA-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-005 frame 0: Threshold 100: nothing is that bright, so there is no star and the drawing is untouched. | largest difference 1.5e-7 | yes |
| FX-KIRA-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-006 frame 0: Spacing 8, size 6, density 100, twinkle 0, a cross: three steady stars, one in each cell with a highlight, a plus of four arms fading to nothing, with a small round core: on the white point, (2.5, 4.5), landing on whole pixels; on the square's middle, (11, 3), a pixel corner, so its arms are shared between the pixels either side; and on the soft glint, (4.5, 8.5), as strong as the others. The grey and the gold have none; pixels off the arms stay as they were. | largest difference 1.5e-7 | yes |
| FX-KIRA-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-007 frame 0: FX-KIRA-006 as a star: each also has four short arms on the diagonals, half as long, so the pixels diagonally next to the white point light too. | largest difference 1.7e-7 | yes |
| FX-KIRA-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-008 frame 0: FX-KIRA-006 at angle 45: each plus turned into an X; the pixels straight beside the white point, lit in FX-KIRA-006, now keep only the core's light. | largest difference 2.0e-7 | yes |
| FX-KIRA-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-009 frame 0: Angle 90: a quarter turn of a plus is the same plus, so it is FX-KIRA-006. | largest difference 1.5e-7 | yes |
| FX-KIRA-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-010 frame 0: FX-KIRA-006 at threshold 60: the grey, at exactly 60 %, is a highlight too, and has its own star at (13.5, 8.5); the gold, its smallest channel 47 %, is still none. | largest difference 1.5e-7 | yes |
| FX-KIRA-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-011 frame 0: FX-KIRA-006 at spacing 16: the frame is one cell, so the three glints share one star at their middle, (8.5, 4.17). | largest difference 1.8e-7 | yes |
| FX-KIRA-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-012 frame 0: FX-KIRA-006 at density 60: the square's cell is not chosen at seed 0, so the square has no star; the other two are FX-KIRA-006's exactly. | largest difference 1.5e-7 | yes |
| FX-KIRA-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-013 frame 0: FX-KIRA-006 at seed 7: the stars stay on the glints, each a different size. | largest difference 1.7e-7 | yes |
| FX-KIRA-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-014 frame 0: FX-KIRA-006 twinkling fully, twinkle 100, period 4: each star grows and fades on its own beat, frame 1 and frame 2 different from frame 0, and frame 4, a whole period on, frame 0 again. | largest difference 1.9e-7 | yes |
| FX-KIRA-014 frame 1: FX-KIRA-006 twinkling fully, twinkle 100, period 4: each star grows and fades on its own beat, frame 1 and frame 2 different from frame 0, and frame 4, a whole period on, frame 0 again. | largest difference 1.7e-7 | yes |
| FX-KIRA-014 frame 2: FX-KIRA-006 twinkling fully, twinkle 100, period 4: each star grows and fades on its own beat, frame 1 and frame 2 different from frame 0, and frame 4, a whole period on, frame 0 again. | largest difference 1.5e-7 | yes |
| FX-KIRA-014 frame 4: FX-KIRA-006 twinkling fully, twinkle 100, period 4: each star grows and fades on its own beat, frame 1 and frame 2 different from frame 0, and frame 4, a whole period on, frame 0 again. | largest difference 1.9e-7 | yes |
| FX-KIRA-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-015 frame 0: Twinkle 50, period 4: each star breathes between half and full, and never goes out. | largest difference 1.5e-7 | yes |
| FX-KIRA-015 frame 2: Twinkle 50, period 4: each star breathes between half and full, and never goes out. | largest difference 1.9e-7 | yes |
| FX-KIRA-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-016 frame 0: Colour #ff8000, orange: FX-KIRA-006's stars tinted, their red as before, their green a fifth and their blue gone; the covering as FX-KIRA-006's. | largest difference 1.5e-7 | yes |
| FX-KIRA-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-017 frame 0: FX-KIRA-016 with its colour written in capitals, #FF8000: the same. | largest difference 1.5e-7 | yes |
| FX-KIRA-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-018 frame 0: Opacity 50: FX-KIRA-006's light at half strength. | largest difference 1.5e-7 | yes |
| FX-KIRA-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-019 frame 0: Size keyed from 0 at frame 0 to 12 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-KIRA-006, frame 4 is size 12. | largest difference 1.5e-7 | yes |
| FX-KIRA-019 frame 2: Size keyed from 0 at frame 0 to 12 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-KIRA-006, frame 4 is size 12. | largest difference 1.5e-7 | yes |
| FX-KIRA-019 frame 4: Size keyed from 0 at frame 0 to 12 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-KIRA-006, frame 4 is size 12. | largest difference 1.7e-7 | yes |
| FX-KIRA-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-020 frame 0: Opacity eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-KIRA-006, as frame 4 is. | largest difference 1.5e-7 | yes |
| FX-KIRA-020 frame 2: Opacity eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-KIRA-006, as frame 4 is. | largest difference 1.5e-7 | yes |
| FX-KIRA-020 frame 4: Opacity eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-KIRA-006, as frame 4 is. | largest difference 1.5e-7 | yes |
| FX-KIRA-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-021 frame 0: FX-KIRA-006 moved three pixels right: the stars move with the drawing, and the white point's left arm, which runs past the drawing's left edge, shows in the grown column just left of it; the two columns further left stay empty. | largest difference 1.5e-7 | yes |
| FX-KIRA-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KIRA-022 frame 0: Threshold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-KIRA-022 frame 4: Threshold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-KIRA-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KIRA-023 frame 0: Spacing 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-KIRA-023 frame 4: Spacing 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-KIRA-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KIRA-024 frame 0: Size 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-KIRA-024 frame 4: Size 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-KIRA-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KIRA-025 frame 0: Period 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-KIRA-025 frame 4: Period 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-KIRA-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KIRA-026 frame 0: Twinkle keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-KIRA-026 frame 4: Twinkle keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-KIRA-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KIRA-027 frame 0: A shape written "circle", not cross or star. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-KIRA-027 frame 4: A shape written "circle", not cross or star. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-KIRA-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KIRA-028 frame 0: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-KIRA-028 frame 4: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-KIRA-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KIRA-029 frame 0: Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-KIRA-029 frame 4: Seed 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-KIRA-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| as added, size 40, grows the drawing's bounds by 40 | 40 | yes |
| size 12.5 grows them by 13, the size rounded up | 13 | yes |
| size 0 grows them by nothing | 0 | yes |
| density 0 grows them by nothing | 0 | yes |
| opacity 0 grows them by nothing | 0 | yes |
| a half-size draft preview halves the spacing and the size, and nothing else | KiraKira { threshold: 95.0, spacing: 32.0, density: 60.0, size: 20.0, shape: "star", angle: 0.0, twinkle: 100.0, period: 24.0, seed: 0.0, opacity: 100.0, color: "#ffffff", frame: 0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_kira_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kira_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kira_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kira_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kira_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kira_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kira_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kira_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kira_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kira_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kira_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kira_017.json, its colour written in capitals, is saved in small letters, as Rain's is | "#ff8000" | yes |
| the frame the twinkle is worked at is never saved | None | yes |
| a file with no `period` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a size that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| threshold 101 is refused with a sentence, and nothing changes | Kira-kira's threshold runs from 0 to 100, and this is 101. | yes |
| spacing 1 is refused with a sentence, and nothing changes | Kira-kira's spacing runs from 2 to 1000, and this is 1. | yes |
| density -1 is refused with a sentence, and nothing changes | Kira-kira's density runs from 0 to 100, and this is -1. | yes |
| size 1001 is refused with a sentence, and nothing changes | Kira-kira's size runs from 0 to 1000, and this is 1001. | yes |
| angle 3601 is refused with a sentence, and nothing changes | Kira-kira's angle runs from -3600 to 3600, and this is 3601. | yes |
| twinkle 101 is refused with a sentence, and nothing changes | Kira-kira's twinkle runs from 0 to 100, and this is 101. | yes |
| period 0 is refused with a sentence, and nothing changes | Kira-kira's period runs from 1 to 1000, and this is 0. | yes |
| seed 100001 is refused with a sentence, and nothing changes | Kira-kira's seed runs from 0 to 100000, and this is 100001. | yes |
| opacity 101 is refused with a sentence, and nothing changes | Kira-kira's opacity runs from 0 to 100, and this is 101. | yes |
| shape "circle" is refused with a sentence, and nothing changes | Kira-kira's shape is "cross" or "star", and this is "circle". | yes |
| colour "#12345" is refused with a sentence, and nothing changes | Kira-kira's colour is written #rrggbb, and this is "#12345". | yes |
| twinkle keyed to 150 is refused with a sentence, and nothing changes | Kira-kira's twinkle runs from 0 to 100, and this is 150. | yes |
| every number at its bottom, a cross, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| period keyed from 1 to 1000 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a night scene at a quarter of 1920 by 1080, in `verification/B-122 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the scene with no effect, draws cleanly | [] | yes |
| as_added_frame_0.png, as it is added, at frame 0, draws cleanly, changes some pixels, some within 10 of each of [], and leaves the moon, the cheek, the collar and the sky, [(360, 50), (130, 175), (130, 240), (20, 20)], exactly | [], 629 changed, lit [], left [true, true, true, true] | yes |
| as_added_frame_12.png, as it is added, at frame 12, half a twinkle on, draws cleanly, changes some pixels, some within 10 of each of [], and leaves the moon, the cheek, the collar and the sky, [(360, 50), (130, 175), (130, 240), (20, 20)], exactly | [], 594 changed, lit [], left [true, true, true, true] | yes |
| density_100.png, density 100, a star in every lit cell, draws cleanly, changes some pixels, some within 10 of each of [(106, 118), (160, 140)], and leaves the moon, the cheek, the collar and the sky, [(360, 50), (130, 175), (130, 240), (20, 20)], exactly | [], 993 changed, lit [true, true], left [true, true, true, true] | yes |
| small_and_close.png, spacing 32 and size 24, draws cleanly, changes some pixels, some within 10 of each of [], and leaves the moon, the cheek, the collar and the sky, [(360, 50), (130, 175), (130, 240), (20, 20)], exactly | [], 534 changed, lit [], left [true, true, true, true] | yes |
| warm_cross.png, a warm cross at angle 45, draws cleanly, changes some pixels, some within 10 of each of [], and leaves the moon, the cheek, the collar and the sky, [(360, 50), (130, 175), (130, 240), (20, 20)], exactly | [], 499 changed, lit [], left [true, true, true, true] | yes |
| the twinkle moves: frame 12 is not frame 0 | true | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_kira_001.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_kira_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_kira_021.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

122 of 122 checks pass.
