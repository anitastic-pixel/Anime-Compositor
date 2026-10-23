# B-632: Directional Blur's streak tapers at its ends

D-753, accepted on 2026-10-23 by the owner's message "Proceed, if an effect is more like a inspiration/modified version of a AE effect, only improve it. But it is to be a 1 to 1 match, then go for it." Each streak is now a tent `length` long in all, heaviest at the drawing and fading to nothing at its two ends, as After Effects' does, where it was a flat average with hard ends. Every expected pixel is written by `tools/directional_blur_reference.py` (and `tools/bloom_reference.py`) before this code and printed in document 25. The build's frame is compared sample by sample against the catalogue's tolerance of 2e-5.

## FX-DIRBLUR-001 to 015 (document 25), on the processor

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-DIRBLUR-001 frame 0: Direction 0, length 4: every edge streaks up and down, two pixels each way, the second faint, and left and right stay sharp. | largest difference 1.5e-7 | yes |
| FX-DIRBLUR-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-002 frame 0: Direction 90, length 4: the same streak left and right, and up and down stay sharp. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-003 frame 0: Direction 270, the opposite way: the streak runs both ways, so this is FX-DIRBLUR-002. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-004 frame 0: Direction 45, length 6: a diagonal streak, up and right and down and left, between pixels. | largest difference 2.0e-7 | yes |
| FX-DIRBLUR-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-005 frame 0: Length 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-006 frame 0: Length 1, direction 90: each column mixes itself and the two beside it at a twelfth, five sixths and a twelfth, so the block's left edge column is eleven twelfths covered and the column outside it one twelfth. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-007 frame 0: Length 2.5, direction 90: a length that is not a whole number, the streak fading to its ends. | largest difference 2.0e-7 | yes |
| FX-DIRBLUR-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-008 frame 0: Direction 3600, ten turns: this is FX-DIRBLUR-001. | largest difference 1.5e-7 | yes |
| FX-DIRBLUR-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-009 frame 0: Direction 90, length 6, moved three pixels right: the line on the drawing's left edge streaks three pixels past it into the grown border. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-009 frame 3: Direction 90, length 6, moved three pixels right: the line on the drawing's left edge streaks three pixels past it into the grown border. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-010 frame 0: Length keyed from 0 at frame 0 to 8 at frame 4, direction 90, linear: frame 0 untouched, frame 2 at length 4 is FX-DIRBLUR-002, frame 4 at 8. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-010 frame 2: Length keyed from 0 at frame 0 to 8 at frame 4, direction 90, linear: frame 0 untouched, frame 2 at length 4 is FX-DIRBLUR-002, frame 4 at 8. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-010 frame 4: Length keyed from 0 at frame 0 to 8 at frame 4, direction 90, linear: frame 0 untouched, frame 2 at length 4 is FX-DIRBLUR-002, frame 4 at 8. | largest difference 1.5e-7 | yes |
| FX-DIRBLUR-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-011 frame 0: Direction keyed from 0 at frame 0 to 90 at frame 4, length 4: frame 0 is FX-DIRBLUR-001, frame 2 streaks at 45 degrees, frame 4 is FX-DIRBLUR-002. | largest difference 1.5e-7 | yes |
| FX-DIRBLUR-011 frame 2: Direction keyed from 0 at frame 0 to 90 at frame 4, length 4: frame 0 is FX-DIRBLUR-001, frame 2 streaks at 45 degrees, frame 4 is FX-DIRBLUR-002. | largest difference 2.1e-7 | yes |
| FX-DIRBLUR-011 frame 4: Direction keyed from 0 at frame 0 to 90 at frame 4, length 4: frame 0 is FX-DIRBLUR-001, frame 2 streaks at 45 degrees, frame 4 is FX-DIRBLUR-002. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIRBLUR-012 frame 0: Length 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-012 frame 4: Length 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIRBLUR-013 frame 0: Length -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-013 frame 4: Length -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIRBLUR-014 frame 0: Direction 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-014 frame 4: Direction 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIRBLUR-015 frame 0: Length keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-015 frame 4: Length keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIRBLUR-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The streak's shape: one half-covered red pixel streaked up and down, length 8

| Check | The build's answer | Matches |
| --- | --- | --- |
| direction 0, length 8 is taken | taken | yes |
| 0 rows below the pixel its covering is 0.229167 of the pixel's, and less than the row before | 0.229167 | yes |
| 1 rows below the pixel its covering is 0.187500 of the pixel's, and less than the row before | 0.187500 | yes |
| 2 rows below the pixel its covering is 0.125000 of the pixel's, and less than the row before | 0.125000 | yes |
| 3 rows below the pixel its covering is 0.062500 of the pixel's, and less than the row before | 0.062500 | yes |
| 4 rows below the pixel its covering is 0.010417 of the pixel's, and less than the row before | 0.010417 | yes |
| 5 rows below the pixel its covering is 0.000000 of the pixel's, and less than the row before | 0.000000 | yes |
| the streak is the same above as below: 1 row above equals 1 row below | 0.187500 against 0.187500 | yes |
| the whole column keeps the pixel's covering: the weights sum to 1 | 1.000000 | yes |
| five rows out, past the streak's end at four, nothing | 0.000000 | yes |

## Bloom keeps its own streaks: FX-BLOOM-001 to 028 (document 25), unmoved by D-753

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
| FX-BLOOM-007 frame 0: The same with a star: four lines, so the diagonals off the patch's corners are lit too, and the straight lines are half as strong, as the light is shared among four lines, not two. | largest difference 4.9e-7 | yes |
| FX-BLOOM-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-008 frame 0: A cross at angle 45: the streaks run along the diagonals only. | largest difference 4.8e-7 | yes |
| FX-BLOOM-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-009 frame 0: A cross at angle 90: the same two lines as angle 0, each walked the other way, so this is FX-BLOOM-006 to rounding. | largest difference 5.6e-7 | yes |
| FX-BLOOM-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-010 frame 0: The defaults, threshold 80, radius 20, intensity 1, no streaks: a halo wider than the whole drawing. | largest difference 2.4e-7 | yes |
| FX-BLOOM-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-011 frame 0: The defaults with a star of streaks, length 60. | largest difference 2.5e-7 | yes |
| FX-BLOOM-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-012 frame 0: Radius 4, a cross of length 6, intensity 2.5: two and a half times as strong, and where it adds past white it is not cut off. | largest difference 1.0e-6 | yes |
| FX-BLOOM-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-013 frame 0: The patches half covering: the yellow blooms as in FX-BLOOM-001, at half the strength. | largest difference 1.8e-7 | yes |
| FX-BLOOM-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-014 frame 0: Radius keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is FX-BLOOM-003, frame 2 is FX-BLOOM-001, frame 4 is radius 8. | largest difference 3.1e-7 | yes |
| FX-BLOOM-014 frame 2: Radius keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is FX-BLOOM-003, frame 2 is FX-BLOOM-001, frame 4 is radius 8. | largest difference 3.2e-7 | yes |
| FX-BLOOM-014 frame 4: Radius keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is FX-BLOOM-003, frame 2 is FX-BLOOM-001, frame 4 is radius 8. | largest difference 2.9e-7 | yes |
| FX-BLOOM-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-015 frame 0: Length keyed from 0 at frame 0 to 12 at frame 4, radius 0, a cross: frame 0 is each yellow pixel added onto itself twice over, frame 2 is FX-BLOOM-006. | largest difference 4.0e-7 | yes |
| FX-BLOOM-015 frame 2: Length keyed from 0 at frame 0 to 12 at frame 4, radius 0, a cross: frame 0 is each yellow pixel added onto itself twice over, frame 2 is FX-BLOOM-006. | largest difference 5.6e-7 | yes |
| FX-BLOOM-015 frame 4: Length keyed from 0 at frame 0 to 12 at frame 4, radius 0, a cross: frame 0 is each yellow pixel added onto itself twice over, frame 2 is FX-BLOOM-006. | largest difference 4.1e-7 | yes |
| FX-BLOOM-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-016 frame 0: Angle keyed from 0 at frame 0 to 90 at frame 4, radius 0, a cross: frame 0 is FX-BLOOM-006, frame 2 is FX-BLOOM-008, and frame 4 is FX-BLOOM-006 again. | largest difference 5.6e-7 | yes |
| FX-BLOOM-016 frame 2: Angle keyed from 0 at frame 0 to 90 at frame 4, radius 0, a cross: frame 0 is FX-BLOOM-006, frame 2 is FX-BLOOM-008, and frame 4 is FX-BLOOM-006 again. | largest difference 4.8e-7 | yes |
| FX-BLOOM-016 frame 4: Angle keyed from 0 at frame 0 to 90 at frame 4, radius 0, a cross: frame 0 is FX-BLOOM-006, frame 2 is FX-BLOOM-008, and frame 4 is FX-BLOOM-006 again. | largest difference 5.6e-7 | yes |
| FX-BLOOM-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-017 frame 0: Threshold keyed from 100 at frame 0 to 20 at frame 4, linear: frame 0 is the drawing, frame 4 has the purple blooming too. | largest difference 1.5e-7 | yes |
| FX-BLOOM-017 frame 2: Threshold keyed from 100 at frame 0 to 20 at frame 4, linear: frame 0 is the drawing, frame 4 has the purple blooming too. | largest difference 3.2e-7 | yes |
| FX-BLOOM-017 frame 4: Threshold keyed from 100 at frame 0 to 20 at frame 4, linear: frame 0 is the drawing, frame 4 has the purple blooming too. | largest difference 3.2e-7 | yes |
| FX-BLOOM-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-018 frame 0: Radius 4 and a cross of length 6, moved three pixels right: the bloom is done on the drawing before it is moved, and the streak that ran past the drawing's left edge now shows in columns 0 to 2. | largest difference 4.5e-7 | yes |
| FX-BLOOM-018 frame 3: Radius 4 and a cross of length 6, moved three pixels right: the bloom is done on the drawing before it is moved, and the streak that ran past the drawing's left edge now shows in columns 0 to 2. | largest difference 4.5e-7 | yes |
| FX-BLOOM-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-019 frame 0: Radius 2.5 and length 2.5: neither is rounded. | largest difference 5.2e-7 | yes |
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

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-DIRBLUR-001, fx_dirblur_001.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-DIRBLUR-002, fx_dirblur_002.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-DIRBLUR-003, fx_dirblur_003.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-DIRBLUR-004, fx_dirblur_004.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-DIRBLUR-005, fx_dirblur_005.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-DIRBLUR-006, fx_dirblur_006.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-DIRBLUR-007, fx_dirblur_007.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-DIRBLUR-008, fx_dirblur_008.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-DIRBLUR-009, fx_dirblur_009.json, frames [0, 3] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-DIRBLUR-010, fx_dirblur_010.json, frames [0, 2, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-DIRBLUR-011, fx_dirblur_011.json, frames [0, 2, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-DIRBLUR-012, fx_dirblur_012.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-DIRBLUR-013, fx_dirblur_013.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-DIRBLUR-014, fx_dirblur_014.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-DIRBLUR-015, fx_dirblur_015.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dirblur_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dirblur_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dirblur_011.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

141 of 141 checks pass.
