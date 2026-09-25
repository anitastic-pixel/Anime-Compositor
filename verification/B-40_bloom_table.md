# B-40: bloom

D-96, accepted by the owner on 2026-09-25. Every expected pixel is `Fixtures/bloom/expected_bloom.json`, written by `tools/bloom_reference.py` before this code existed and printed in document 25 as FX-BLOOM-001 to 028. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-BLOOM-001 to 028 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BLOOM-001 frame 0: Threshold 80, radius 4, no streaks: only the yellow patch, whose brightest channel is 98 %, blooms; the brown at 60 % and the purple do not, though the halo reaches the brown. The halo is brightest close to the yellow and fades into the empty space round it. | largest difference 3.2e-7 | yes |
| FX-BLOOM-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-002 frame 0: Intensity 0: the drawing, untouched. | largest difference 1.5e-7 | yes |
| FX-BLOOM-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-003 frame 0: Radius 0: nothing spreads, and each yellow pixel is added onto itself, twice as bright. | largest difference 3.1e-7 | yes |
| FX-BLOOM-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-004 frame 0: Threshold 100: nothing is that bright, so the drawing is untouched. | largest difference 1.5e-7 | yes |
| FX-BLOOM-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-005 frame 0: Threshold 60: the brown, at exactly 60 %, blooms too. | largest difference 3.2e-7 | yes |
| FX-BLOOM-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-006 frame 0: Radius 0 with a cross of streaks, length 6, angle 0: the yellow's light runs straight up and down and straight left and right, and nowhere else: above the patch it is lit, off its corners it is not. | largest difference 5.6e-7 | yes |
| FX-BLOOM-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-007 frame 0: The same with a star: four lines, so the diagonals off the patch's corners are lit too, and the straight lines are half as strong, as the light is shared among four lines, not two. | largest difference 4.2e-7 | yes |
| FX-BLOOM-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-008 frame 0: A cross at angle 45: the streaks run along the diagonals only. | largest difference 5.1e-7 | yes |
| FX-BLOOM-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-009 frame 0: A cross at angle 90: the same two lines as angle 0, each walked the other way, so this is FX-BLOOM-006 to rounding. | largest difference 5.6e-7 | yes |
| FX-BLOOM-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-010 frame 0: The defaults, threshold 80, radius 20, intensity 1, no streaks: a halo wider than the whole drawing. | largest difference 2.4e-7 | yes |
| FX-BLOOM-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-011 frame 0: The defaults with a star of streaks, length 60. | largest difference 3.0e-7 | yes |
| FX-BLOOM-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-012 frame 0: Radius 4, a cross of length 6, intensity 2.5: two and a half times as strong, and where it adds past white it is not cut off. | largest difference 7.8e-7 | yes |
| FX-BLOOM-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-013 frame 0: The patches half covering: the yellow blooms as in FX-BLOOM-001, at half the strength. | largest difference 1.8e-7 | yes |
| FX-BLOOM-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-014 frame 0: Radius keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is FX-BLOOM-003, frame 2 is FX-BLOOM-001, frame 4 is radius 8. | largest difference 3.1e-7 | yes |
| FX-BLOOM-014 frame 2: Radius keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is FX-BLOOM-003, frame 2 is FX-BLOOM-001, frame 4 is radius 8. | largest difference 3.2e-7 | yes |
| FX-BLOOM-014 frame 4: Radius keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is FX-BLOOM-003, frame 2 is FX-BLOOM-001, frame 4 is radius 8. | largest difference 2.9e-7 | yes |
| FX-BLOOM-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-015 frame 0: Length keyed from 0 at frame 0 to 12 at frame 4, radius 0, a cross: frame 0 is each yellow pixel added onto itself twice over, frame 2 is FX-BLOOM-006. | largest difference 4.0e-7 | yes |
| FX-BLOOM-015 frame 2: Length keyed from 0 at frame 0 to 12 at frame 4, radius 0, a cross: frame 0 is each yellow pixel added onto itself twice over, frame 2 is FX-BLOOM-006. | largest difference 5.6e-7 | yes |
| FX-BLOOM-015 frame 4: Length keyed from 0 at frame 0 to 12 at frame 4, radius 0, a cross: frame 0 is each yellow pixel added onto itself twice over, frame 2 is FX-BLOOM-006. | largest difference 5.0e-7 | yes |
| FX-BLOOM-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-016 frame 0: Angle keyed from 0 at frame 0 to 90 at frame 4, radius 0, a cross: frame 0 is FX-BLOOM-006, frame 2 is FX-BLOOM-008, and frame 4 is FX-BLOOM-006 again. | largest difference 5.6e-7 | yes |
| FX-BLOOM-016 frame 2: Angle keyed from 0 at frame 0 to 90 at frame 4, radius 0, a cross: frame 0 is FX-BLOOM-006, frame 2 is FX-BLOOM-008, and frame 4 is FX-BLOOM-006 again. | largest difference 5.1e-7 | yes |
| FX-BLOOM-016 frame 4: Angle keyed from 0 at frame 0 to 90 at frame 4, radius 0, a cross: frame 0 is FX-BLOOM-006, frame 2 is FX-BLOOM-008, and frame 4 is FX-BLOOM-006 again. | largest difference 5.6e-7 | yes |
| FX-BLOOM-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-017 frame 0: Threshold keyed from 100 at frame 0 to 20 at frame 4, linear: frame 0 is the drawing, frame 4 has the purple blooming too. | largest difference 1.5e-7 | yes |
| FX-BLOOM-017 frame 2: Threshold keyed from 100 at frame 0 to 20 at frame 4, linear: frame 0 is the drawing, frame 4 has the purple blooming too. | largest difference 3.2e-7 | yes |
| FX-BLOOM-017 frame 4: Threshold keyed from 100 at frame 0 to 20 at frame 4, linear: frame 0 is the drawing, frame 4 has the purple blooming too. | largest difference 3.2e-7 | yes |
| FX-BLOOM-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-018 frame 0: Radius 4 and a cross of length 6, moved three pixels right: the bloom is done on the drawing before it is moved, and the streak that ran past the drawing's left edge now shows in columns 0 to 2. | largest difference 4.5e-7 | yes |
| FX-BLOOM-018 frame 3: Radius 4 and a cross of length 6, moved three pixels right: the bloom is done on the drawing before it is moved, and the streak that ran past the drawing's left edge now shows in columns 0 to 2. | largest difference 4.5e-7 | yes |
| FX-BLOOM-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-019 frame 0: Radius 2.5 and length 2.5: neither is rounded. | largest difference 4.6e-7 | yes |
| FX-BLOOM-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-020 frame 0: Threshold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-020 frame 4: Threshold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOOM-021 frame 0: Radius 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-021 frame 4: Radius 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOOM-022 frame 0: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-022 frame 4: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOOM-023 frame 0: Intensity 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-023 frame 4: Intensity 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOOM-024 frame 0: Length 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-024 frame 4: Length 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOOM-025 frame 0: Angle 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-025 frame 4: Angle 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOOM-026 frame 0: Radius keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-026 frame 4: Radius keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOOM-027 frame 0: Streaks "rays", which is not none, cross or star. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-027 frame 4: Streaks "rays", which is not none, cross or star. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLOOM-028 frame 0: Streaks "Cross", with a capital: words are matched exactly. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-028 frame 4: Streaks "Cross", with a capital: words are matched exactly. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| radius 20, no streaks, reaches the radius | 20 | yes |
| radius 20 with a star of length 60 reaches the length | 60 | yes |
| a cross of length 2.5 at radius 0 reaches 3 | 3 | yes |
| with no streaks the length reaches nowhere | 0 | yes |
| a half-size draft preview halves the radius and the length, and nothing else | Bloom { threshold: 80.0, radius: 10.0, intensity: 1.0, streaks: "cross", length: 30.0, angle: 30.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bloom_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bloom_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bloom_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bloom_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bloom_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bloom_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bloom_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bloom_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bloom_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bloom_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `streaks` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an angle written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| threshold 101 is refused with a sentence, and nothing changes | Bloom's threshold runs from 0 to 100, and this is 101. | yes |
| radius 501 is refused with a sentence, and nothing changes | Bloom's radius runs from 0 to 500, and this is 501. | yes |
| radius -1 is refused with a sentence, and nothing changes | Bloom's radius runs from 0 to 500, and this is -1. | yes |
| intensity 11 is refused with a sentence, and nothing changes | Bloom's intensity runs from 0 to 10, and this is 11. | yes |
| length 501 is refused with a sentence, and nothing changes | Bloom's length runs from 0 to 500, and this is 501. | yes |
| angle -3601 is refused with a sentence, and nothing changes | Bloom's angle runs from -3600 to 3600, and this is -3601. | yes |
| streaks "rays" is refused with a sentence, and nothing changes | Bloom's streaks are "none", "cross" or "star", and this is "rays". | yes |
| streaks "Star" is refused with a sentence, and nothing changes | Bloom's streaks are "none", "cross" or "star", and this is "Star". | yes |
| length keyed to 600 is refused with a sentence, and nothing changes | Bloom's length runs from 0 to 500, and this is 600. | yes |
| radius 500, intensity 10, a star of length 500 at angle 3600, the tops of the ranges, is taken | taken | yes |
| everything at the bottom of its range, angle -3600, is taken | taken | yes |
| angle keyed from 0 to 90 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bloom_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bloom_018.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

106 of 106 checks pass.
