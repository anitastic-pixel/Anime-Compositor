# D-469: Threshold RGB

B-349: `core.threshold_rgb` takes CycoreFX's CC Threshold RGB controls, a threshold and an Invert for each of red, green and blue and Blend With Original, under our own name: a second name over Channel Threshold's rule (D-468, as D-383 and D-394) with a level and an Invert per channel. Every expected pixel is `Fixtures/threshold_rgb/expected_threshold_rgb.json`, written by `tools/threshold_rgb_reference.py` before this code existed and printed in document 25 as FX-THRESHRGB-001 to 014; Channel Threshold's FX-CHTHRESH-001 to 023 and Threshold's FX-THRESH-001 to 018 rerun unchanged (019 is in dispute, D-164). Tolerance 2e-5.

## FX-THRESHRGB-001 to 014 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-THRESHRGB-001 frame 0: As added: every threshold 128, no Invert, Blend 0: each channel on its own, red stays pure red, green pure green, both skins white, grey 127 black; the same as Channel Threshold's RGB at 128 (FX-CHTHRESH-004). | largest difference 3.0e-8 | yes |
| FX-THRESHRGB-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESHRGB-002 frame 0: Red 230, green 128, blue 50: the lit skin (246, 214, 190) stays white, the shadow skin (220, 160, 140) loses its red and turns cyan, the light grey (192) loses its red too. | largest difference 3.0e-8 | yes |
| FX-THRESHRGB-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESHRGB-003 frame 0: Invert Green only: green turned over in every pixel, so pure green turns black, pure red yellow and both skins magenta. | largest difference 3.0e-8 | yes |
| FX-THRESHRGB-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESHRGB-004 frame 0: All three inverted: FX-THRESHRGB-001 turned over, white black and pure red cyan. | largest difference 3.0e-8 | yes |
| FX-THRESHRGB-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESHRGB-005 frame 0: Blend With Original 50: every pixel halfway, in linear light, between the drawing and FX-THRESHRGB-001. | largest difference 1.1e-7 | yes |
| FX-THRESHRGB-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESHRGB-006 frame 0: Red 0, green 255, blue 128: red full in every shown pixel, green only where it is full (green and white), blue as at 128. | largest difference 3.0e-8 | yes |
| FX-THRESHRGB-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESHRGB-007 frame 0: Red keyed from 0 at frame 0 to 255 at frame 4, linear: frame 2 is red 127.5. | largest difference 3.0e-8 | yes |
| FX-THRESHRGB-007 frame 2: Red keyed from 0 at frame 0 to 255 at frame 4, linear: frame 2 is red 127.5. | largest difference 3.0e-8 | yes |
| FX-THRESHRGB-007 frame 4: Red keyed from 0 at frame 0 to 255 at frame 4, linear: frame 2 is red 127.5. | largest difference 3.0e-8 | yes |
| FX-THRESHRGB-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESHRGB-008 frame 0: Blend eased from 50 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it is held at 100, the drawing. | largest difference 1.1e-7 | yes |
| FX-THRESHRGB-008 frame 2: Blend eased from 50 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it is held at 100, the drawing. | largest difference 1.9e-7 | yes |
| FX-THRESHRGB-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESHRGB-009 frame 0: FX-THRESHRGB-002 moved three pixels right: the same, moved. | largest difference 0.0e0 | yes |
| FX-THRESHRGB-009 frame 3: FX-THRESHRGB-002 moved three pixels right: the same, moved. | largest difference 0.0e0 | yes |
| FX-THRESHRGB-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESHRGB-010 frame 0: Red Threshold 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESHRGB-010 frame 4: Red Threshold 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESHRGB-010: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THRESHRGB-011 frame 0: Green Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESHRGB-011 frame 4: Green Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESHRGB-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THRESHRGB-012 frame 0: Blue Threshold keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESHRGB-012 frame 4: Blue Threshold keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESHRGB-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THRESHRGB-013 frame 0: Invert Red written "true", not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESHRGB-013 frame 4: Invert Red written "true", not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESHRGB-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THRESHRGB-014 frame 0: Blend With Original -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESHRGB-014 frame 4: Blend With Original -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESHRGB-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Channel Threshold and Threshold, the rule under the second name: their fixtures, unchanged

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CHTHRESH-001 frame 0: As added: Threshold 128, Luminance, Invert off, Blend 0: exactly Threshold at 128 (FX-THRESH-001), grey 128 and everything lighter white, the rest black, each soft edge at its own covering. | largest difference 3.0e-8 | yes |
| FX-CHTHRESH-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHTHRESH-002 frame 0: Threshold 50: exactly Threshold at 50 (FX-THRESH-005), red white and blue black. | largest difference 3.0e-8 | yes |
| FX-CHTHRESH-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHTHRESH-003 frame 0: Invert on: FX-CHTHRESH-001 turned over, what was white black and what was black white. | largest difference 3.0e-8 | yes |
| FX-CHTHRESH-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHTHRESH-004 frame 0: RGB at 128: each channel on its own, so red stays pure red, green pure green, blue pure blue, both skins white and grey 127 black. | largest difference 3.0e-8 | yes |
| FX-CHTHRESH-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHTHRESH-005 frame 0: RGB at 200: the lit skin, red and green above 200 and blue below, turns yellow; the shadow skin keeps only its red; the light grey turns black. | largest difference 3.0e-8 | yes |
| FX-CHTHRESH-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHTHRESH-006 frame 0: Saturation at 128: the pure colours and both skins white; the greys, black, white and the dull line black. | largest difference 3.0e-8 | yes |
| FX-CHTHRESH-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHTHRESH-007 frame 0: Saturation at 150: the shadow skin, saturation 136, turns black while the lit skin, 193, stays white. | largest difference 3.0e-8 | yes |
| FX-CHTHRESH-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHTHRESH-008 frame 0: Alpha at 129: the two soft edges, covering 128, turn clear; every opaque pixel keeps its colour and covering. | largest difference 1.9e-7 | yes |
| FX-CHTHRESH-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHTHRESH-009 frame 0: Alpha at 100: the soft edges turn opaque in their own colours; the empty pixels stay empty. | largest difference 1.9e-7 | yes |
| FX-CHTHRESH-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHTHRESH-010 frame 0: Alpha at 129, Invert on: the soft edges turn opaque and every opaque pixel clear. | largest difference 1.9e-7 | yes |
| FX-CHTHRESH-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHTHRESH-011 frame 0: Blend With Original 50: every pixel halfway, in linear light, between the drawing and FX-CHTHRESH-001. | largest difference 1.1e-7 | yes |
| FX-CHTHRESH-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHTHRESH-012 frame 0: Blend With Original 100: the drawing exactly as it was. | largest difference 1.9e-7 | yes |
| FX-CHTHRESH-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHTHRESH-013 frame 0: RGB, Invert on, Blend 25: each channel turned over, 75 per cent of the way from the drawing. | largest difference 4.7e-8 | yes |
| FX-CHTHRESH-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHTHRESH-014 frame 0: Alpha at 129, Blend 50: the soft edges at half their covering, the rest as it was. | largest difference 1.9e-7 | yes |
| FX-CHTHRESH-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHTHRESH-015 frame 0: Threshold keyed from 0 at frame 0 to 255 at frame 4, linear: frame 2 is Threshold 127.5, grey 128 white and grey 127 black. | largest difference 3.0e-8 | yes |
| FX-CHTHRESH-015 frame 2: Threshold keyed from 0 at frame 0 to 255 at frame 4, linear: frame 2 is Threshold 127.5, grey 128 white and grey 127 black. | largest difference 3.0e-8 | yes |
| FX-CHTHRESH-015 frame 4: Threshold keyed from 0 at frame 0 to 255 at frame 4, linear: frame 2 is Threshold 127.5, grey 128 white and grey 127 black. | largest difference 3.0e-8 | yes |
| FX-CHTHRESH-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHTHRESH-016 frame 0: Blend eased from 50 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and the frame is the drawing. | largest difference 1.1e-7 | yes |
| FX-CHTHRESH-016 frame 2: Blend eased from 50 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and the frame is the drawing. | largest difference 1.9e-7 | yes |
| FX-CHTHRESH-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHTHRESH-017 frame 0: FX-CHTHRESH-004 moved three pixels right: the same, moved. | largest difference 0.0e0 | yes |
| FX-CHTHRESH-017 frame 3: FX-CHTHRESH-004 moved three pixels right: the same, moved. | largest difference 0.0e0 | yes |
| FX-CHTHRESH-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHTHRESH-018 frame 0: Threshold 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHTHRESH-018 frame 4: Threshold 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHTHRESH-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHTHRESH-019 frame 0: Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHTHRESH-019 frame 4: Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHTHRESH-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHTHRESH-020 frame 0: A channel written "red", not one of the four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHTHRESH-020 frame 4: A channel written "red", not one of the four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHTHRESH-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHTHRESH-021 frame 0: Invert written "yes", not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHTHRESH-021 frame 4: Invert written "yes", not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHTHRESH-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHTHRESH-022 frame 0: Blend With Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHTHRESH-022 frame 4: Blend With Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHTHRESH-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CHTHRESH-023 frame 0: Blend With Original keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHTHRESH-023 frame 4: Blend With Original keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CHTHRESH-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THRESH-001 frame 0: Level 128, the setting as it starts: black, blue, the line, red, the dark grey and grey 127 turn black; grey 128, exactly on the level, turns white, as do the shadow skin, green, the light grey, the skin and white; the soft skin turns white and the soft line black, each at its own half covering. | largest difference 3.0e-8 | yes |
| FX-THRESH-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-002 frame 0: Level 0: every shown pixel turns white, black too; the soft edges are white at half covering. | largest difference 3.0e-8 | yes |
| FX-THRESH-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-003 frame 0: Level 255: only white stays white; every other shown pixel, the skin and the light grey too, turns black. | largest difference 3.0e-8 | yes |
| FX-THRESH-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-004 frame 0: Level 20: only black and blue, the darkest, turn black; the line, darker to the eye than red but brighter than blue, turns white. | largest difference 3.0e-8 | yes |
| FX-THRESH-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-005 frame 0: Level 50: red, at luma 54, turns white while blue and the line stay black: a pure red counts as brighter than a pure blue. | largest difference 3.0e-8 | yes |
| FX-THRESH-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-006 frame 0: Level 64: the dark grey sits exactly on the level and turns white; red, just below, turns black. | largest difference 3.0e-8 | yes |
| FX-THRESH-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-007 frame 0: Level 127: grey 127 is now exactly on the level and turns white, with grey 128. | largest difference 3.0e-8 | yes |
| FX-THRESH-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-008 frame 0: Level 128.5, a level between two 8-bit steps: grey 128 turns black, with grey 127. | largest difference 3.0e-8 | yes |
| FX-THRESH-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-009 frame 0: Level 172: the shadow skin, at luma 171.3, turns black while the lit skin stays white, the cel split into its light and its shadow. | largest difference 3.0e-8 | yes |
| FX-THRESH-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-010 frame 0: Level 185: pure green, at luma 182.4, turns black; the light grey, at 192, stays white. | largest difference 3.0e-8 | yes |
| FX-THRESH-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-011 frame 0: Level keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 is FX-THRESH-002, frame 2 level 127.5, grey 127 black and grey 128 white, and frame 4 FX-THRESH-003; the pixels turn black darkest first. | largest difference 3.0e-8 | yes |
| FX-THRESH-011 frame 1: Level keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 is FX-THRESH-002, frame 2 level 127.5, grey 127 black and grey 128 white, and frame 4 FX-THRESH-003; the pixels turn black darkest first. | largest difference 3.0e-8 | yes |
| FX-THRESH-011 frame 2: Level keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 is FX-THRESH-002, frame 2 level 127.5, grey 127 black and grey 128 white, and frame 4 FX-THRESH-003; the pixels turn black darkest first. | largest difference 3.0e-8 | yes |
| FX-THRESH-011 frame 3: Level keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 is FX-THRESH-002, frame 2 level 127.5, grey 127 black and grey 128 white, and frame 4 FX-THRESH-003; the pixels turn black darkest first. | largest difference 3.0e-8 | yes |
| FX-THRESH-011 frame 4: Level keyed from 0 at frame 0 to 255 at frame 4, linear: frame 0 is FX-THRESH-002, frame 2 level 127.5, grey 127 black and grey 128 white, and frame 4 FX-THRESH-003; the pixels turn black darkest first. | largest difference 3.0e-8 | yes |
| FX-THRESH-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-012 frame 0: Level held at 50 from frame 0, then 185 from frame 3: frames 0 and 2 are FX-THRESH-005, frames 3 and 4 FX-THRESH-010. | largest difference 3.0e-8 | yes |
| FX-THRESH-012 frame 2: Level held at 50 from frame 0, then 185 from frame 3: frames 0 and 2 are FX-THRESH-005, frames 3 and 4 FX-THRESH-010. | largest difference 3.0e-8 | yes |
| FX-THRESH-012 frame 3: Level held at 50 from frame 0, then 185 from frame 3: frames 0 and 2 are FX-THRESH-005, frames 3 and 4 FX-THRESH-010. | largest difference 3.0e-8 | yes |
| FX-THRESH-012 frame 4: Level held at 50 from frame 0, then 185 from frame 3: frames 0 and 2 are FX-THRESH-005, frames 3 and 4 FX-THRESH-010. | largest difference 3.0e-8 | yes |
| FX-THRESH-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-013 frame 0: Level eased from 128 at frame 0 to 0 at frame 4 on a curve that overshoots: at frame 2 the level has gone below 0 and is held there, so frames 2 and 4 are both FX-THRESH-002. | largest difference 3.0e-8 | yes |
| FX-THRESH-013 frame 2: Level eased from 128 at frame 0 to 0 at frame 4 on a curve that overshoots: at frame 2 the level has gone below 0 and is held there, so frames 2 and 4 are both FX-THRESH-002. | largest difference 3.0e-8 | yes |
| FX-THRESH-013 frame 4: Level eased from 128 at frame 0 to 0 at frame 4 on a curve that overshoots: at frame 2 the level has gone below 0 and is held there, so frames 2 and 4 are both FX-THRESH-002. | largest difference 3.0e-8 | yes |
| FX-THRESH-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-014 frame 0: FX-THRESH-001 moved three pixels right: the same, moved. | largest difference 0.0e0 | yes |
| FX-THRESH-014 frame 3: FX-THRESH-001 moved three pixels right: the same, moved. | largest difference 0.0e0 | yes |
| FX-THRESH-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THRESH-015 frame 0: Level -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESH-015 frame 4: Level -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESH-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THRESH-016 frame 0: Level 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESH-016 frame 4: Level 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESH-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THRESH-017 frame 0: Level 255.5, past 255 by half a step. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESH-017 frame 4: Level 255.5, past 255 by half a step. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESH-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THRESH-018 frame 0: Level keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESH-018 frame 4: Level keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-THRESH-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_threshrgb_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threshrgb_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threshrgb_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threshrgb_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threshrgb_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threshrgb_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threshrgb_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threshrgb_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threshrgb_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threshrgb_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threshrgb_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threshrgb_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threshrgb_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threshrgb_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threshrgb_002.json is saved with its seven settings | {"blend_with_original":0,"blue_threshold":50,"green_threshold":128,"invert_blue":"off","invert_green":"off","invert_red":"off","red_threshold":230} | yes |
| fx_threshrgb_010.json is refused in a sentence naming red_threshold | Threshold RGB's red threshold runs from 0 to 255, and this is 256. | yes |
| fx_threshrgb_011.json is refused in a sentence naming green_threshold | Threshold RGB's green threshold runs from 0 to 255, and this is -1. | yes |
| fx_threshrgb_013.json is refused in a sentence naming invert red | Threshold RGB's invert red is "off" or "on", and this is "true". | yes |
| fx_threshrgb_014.json is refused in a sentence naming blend_with_original | Threshold RGB's blend with original runs from 0 to 100, and this is -1. | yes |
| a file with a Threshold RGB whose red threshold is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Threshold RGB without its blue threshold is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Threshold RGB whose invert green is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| red threshold 256 is refused with a sentence, and nothing changes | Threshold RGB's red threshold runs from 0 to 255, and this is 256. | yes |
| invert blue yes is refused with a sentence, and nothing changes | Threshold RGB's invert blue is "off" or "on", and this is "yes". | yes |
| green threshold keyed to 300 is refused with a sentence, and nothing changes | Threshold RGB's green threshold runs from 0 to 255, and this is 300. | yes |
| green turned over, blue at 40 is taken | taken | yes |
| red keyed from 0 to 255 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_threshrgb_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_threshrgb_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_threshrgb_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_threshrgb_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_threshrgb_007.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_threshrgb_009.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_threshrgb_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_threshrgb_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_threshrgb_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_threshrgb_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_threshrgb_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_threshrgb_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_threshrgb_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_threshrgb_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 4 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_threshrgb_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_threshrgb_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_threshrgb_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_threshrgb_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_threshrgb_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_threshrgb_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Threshold RGB as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Threshold RGB as added on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threshold RGB as added on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threshold RGB as added on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threshold RGB as added on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threshold RGB as added on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threshold RGB as added on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threshold RGB red 200, green 90 turned over, blue 40, blend 30: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Threshold RGB red 200, green 90 turned over, blue 40, blend 30 on three layers, frame 0, Full | largest difference 1 of 255, 2567 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threshold RGB red 200, green 90 turned over, blue 40, blend 30 on three layers, frame 100, Full | largest difference 1 of 255, 2359 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threshold RGB red 200, green 90 turned over, blue 40, blend 30 on three layers, frame 239, Full | largest difference 1 of 255, 3167 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threshold RGB red 200, green 90 turned over, blue 40, blend 30 on three layers, frame 0, Draft | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threshold RGB red 200, green 90 turned over, blue 40, blend 30 on three layers, frame 100, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threshold RGB red 200, green 90 turned over, blue 40, blend 30 on three layers, frame 239, Draft | largest difference 1 of 255, 86 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threshold RGB all three turned over: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Threshold RGB all three turned over on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threshold RGB all three turned over on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threshold RGB all three turned over on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threshold RGB all three turned over on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threshold RGB all three turned over on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threshold RGB all three turned over on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-469 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| blend with original 100 changes nothing: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_as_added.png, as added (every threshold 128): every channel of every solid pixel full or nothing, so one of eight pure colours; draws cleanly | [], every channel full or nothing: true | yes |
| 3_red_20.png, red at 20, the rest at 128: red full in more of the street than in 2, green and blue as in 2; draws cleanly | [], red only grows and the rest is kept: true, some pixels gain red: true | yes |
| 4_invert_green.png, Invert Green: 2 with its green turned over and its red and blue kept; draws cleanly | [], green turned over, the rest kept: true | yes |
| 5_blend_50.png, as added at blend 50: nearer the street than 2 is; draws cleanly | [], largest difference from the street 77 against 2's 127 | yes |
| Threshold RGB with every level 60 and Invert off at blend 20 against Channel Threshold's RGB the same: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |
| Threshold RGB with every level 128 and Invert on at blend 20 against Channel Threshold's RGB the same: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |
| Threshold RGB with every level 210 and Invert off at blend 20 against Channel Threshold's RGB the same: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |

## Result

221 of 221 checks pass.
