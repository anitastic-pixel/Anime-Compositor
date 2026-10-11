# D-548: Spill Suppressor

B-428: `core.spill_suppressor` takes After Effects' Spill Suppressor (Keying, Obsolete) controls, Color To Suppress and Suppression, as a second name over Screen Key's despill (D-540, as D-383 and D-394) with its matte turned off: the colour's leading channel is held toward the mean of the other two by the suppression, every covering kept. There is no Color Accuracy (one careful rule for every colour, as D-459). Every expected pixel is `Fixtures/spill_suppressor/expected_spill_suppressor.json`, written by `tools/spill_suppressor_reference.py` before this code existed and printed in document 25 as FX-SSUP-001 to 015; Screen Key's FX-SKEY-001 to 024 rerun unchanged. Tolerance 2e-5.

## FX-SSUP-001 to 015 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SSUP-001 frame 0: The settings as they start: Color To Suppress blue, Suppression 100. Each pixel's blue is held no higher than the mean of its red and green: pure blue turns black, cyan and magenta keep half their blue, the greys, white, yellow, green, red, skin and the warm tones are untouched; every covering is kept. | largest difference 1.9e-7 | yes |
| FX-SSUP-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SSUP-002 frame 0: Color To Suppress green: green turns black, yellow and cyan keep half their green; blue, magenta, red, the greys and skin are untouched. | largest difference 1.9e-7 | yes |
| FX-SSUP-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SSUP-003 frame 0: Color To Suppress green, Suppression 50: each green half way, in encoded values, between the drawing's and FX-SSUP-002's. | largest difference 1.9e-7 | yes |
| FX-SSUP-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SSUP-004 frame 0: Suppression 0: the drawing exactly as it is. | largest difference 1.9e-7 | yes |
| FX-SSUP-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SSUP-005 frame 0: Color To Suppress picked from a green screen, (40, 180, 60): green leads it, so it is FX-SSUP-002 exactly. | largest difference 1.9e-7 | yes |
| FX-SSUP-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SSUP-006 frame 0: Color To Suppress red: red turns black, yellow and magenta keep half their red; skin and orange, whose red leads, lose some of it. | largest difference 1.3e-7 | yes |
| FX-SSUP-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SSUP-007 frame 0: Color To Suppress #808080, a grey: no channel leads, so green is taken, as Screen Key takes it: FX-SSUP-002 exactly. | largest difference 1.9e-7 | yes |
| FX-SSUP-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SSUP-008 frame 0: FX-SSUP-002 with its colour written in capitals, #00FF00: the same. | largest difference 1.9e-7 | yes |
| FX-SSUP-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SSUP-009 frame 0: Suppression keyed from 0 at frame 0 to 100 at frame 4, linear, green: frame 0 is the drawing, frame 2 is FX-SSUP-003, frame 4 is FX-SSUP-002. | largest difference 1.9e-7 | yes |
| FX-SSUP-009 frame 2: Suppression keyed from 0 at frame 0 to 100 at frame 4, linear, green: frame 0 is the drawing, frame 2 is FX-SSUP-003, frame 4 is FX-SSUP-002. | largest difference 1.9e-7 | yes |
| FX-SSUP-009 frame 4: Suppression keyed from 0 at frame 0 to 100 at frame 4, linear, green: frame 0 is the drawing, frame 2 is FX-SSUP-003, frame 4 is FX-SSUP-002. | largest difference 1.9e-7 | yes |
| FX-SSUP-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SSUP-010 frame 0: FX-SSUP-002 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-SSUP-010 frame 3: FX-SSUP-002 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-SSUP-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SSUP-011 frame 0: Suppression 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SSUP-011 frame 4: Suppression 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SSUP-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SSUP-012 frame 0: Suppression -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SSUP-012 frame 4: Suppression -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SSUP-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SSUP-013 frame 0: Suppression keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SSUP-013 frame 4: Suppression keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SSUP-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SSUP-014 frame 0: Color To Suppress written "blue", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SSUP-014 frame 4: Color To Suppress written "blue", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SSUP-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SSUP-015 frame 0: Color To Suppress written "#00ff0", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SSUP-015 frame 4: Color To Suppress written "#00ff0", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SSUP-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Screen Key, the engine under the second name: its fixtures, unchanged

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SKEY-001 frame 0: The settings as they start: screen colour pure green, gain 100, balance 50, no clip, despill bias 50 at full strength. Pure green's top row is keyed out, its darker rows keep a share in step with their darkness; the green screen, less saturated than pure green, keeps about half its covering and loses its green spill; skin, the greys and the reds keep all of theirs. | largest difference 1.2e-7 | yes |
| FX-SKEY-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-002 frame 0: Screen colour picked from the green screen (40, 180, 60): its top row is keyed out, its darker rows keep a little more each (their green stands less above the rest); pure green, more saturated than the screen, is gone in its top five rows; skin, greys, reds and blues are untouched apart from the despill. | largest difference 1.8e-7 | yes |
| FX-SKEY-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-003 frame 0: Screen gain 150: more is keyed; the green screen's darker rows fall further and cyan and yellow, which stand half as far above, lose more. | largest difference 1.2e-7 | yes |
| FX-SKEY-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-004 frame 0: Screen balance 0: the screen channel is measured against the larger of the other two, so yellow and cyan no longer read as screen. | largest difference 2.1e-7 | yes |
| FX-SKEY-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-005 frame 0: Screen balance 100: against the smaller of the other two, so yellow and cyan read as screen as strongly as green. | largest difference 1.8e-7 | yes |
| FX-SKEY-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-006 frame 0: Clip black 30, clip white 70: the matte below 0.3 is screen, above 0.7 foreground, stretched between; the green screen's darker rows clean up. | largest difference 1.7e-7 | yes |
| FX-SKEY-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-007 frame 0: Clip black 53 and clip white 53: a hard matte, each pixel in or out. | largest difference 1.2e-7 | yes |
| FX-SKEY-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-008 frame 0: Despill bias 0: the green is held to the larger of red and blue, the gentlest despill. | largest difference 1.8e-7 | yes |
| FX-SKEY-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-009 frame 0: Despill bias 100: the green is held to the smaller of red and blue, the strongest despill. | largest difference 1.8e-7 | yes |
| FX-SKEY-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-010 frame 0: Despill 0: the matte as FX-SKEY-002, the colours not despilled. | largest difference 1.8e-7 | yes |
| FX-SKEY-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-011 frame 0: Blue screen (30, 70, 200) at balance 95, the advice for blue screens: the blue screen and its half-covered copy are keyed, pure blue is gone, and so are cyan, magenta and violet, whose blue stands as far above; pale blue keeps a share; green, yellow and red keep all their covering. | largest difference 1.4e-7 | yes |
| FX-SKEY-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-012 frame 0: View screen matte, FX-SKEY-006's settings: the matte as grey, covering kept. | largest difference 1.9e-7 | yes |
| FX-SKEY-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-013 frame 0: View status, FX-SKEY-006's settings: white where all foreground, black where all screen, one grey for every blend. | largest difference 3.0e-8 | yes |
| FX-SKEY-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-014 frame 0: Shrink/grow -1: the foreground matte shrinks a pixel, so the columns beside the keyed green screen and pure green lose their covering too; the frame's own edge is not eaten. | largest difference 1.7e-7 | yes |
| FX-SKEY-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-015 frame 0: Shrink/grow 1.5: the foreground matte grows a pixel and a half, filling the keyed green screen from its neighbours; pure green, between cyan and yellow that are keyed too at the top, stays keyed there. | largest difference 1.7e-7 | yes |
| FX-SKEY-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-016 frame 0: Softness 1: the matte blurred at sigma 1, the edge between the screen and the foreground soft. | largest difference 2.0e-7 | yes |
| FX-SKEY-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-017 frame 0: Edge refine 2: Refine Soft Matte's guided fit at radius 2 on the keyed result's edges. | largest difference 1.3e-7 | yes |
| FX-SKEY-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-018 frame 0: Screen gain keyed from 0 at frame 0 to 200 at frame 4, linear: nothing keyed at frame 0, twice FX-SKEY-002's at frame 4. | largest difference 1.2e-7 | yes |
| FX-SKEY-018 frame 2: Screen gain keyed from 0 at frame 0 to 200 at frame 4, linear: nothing keyed at frame 0, twice FX-SKEY-002's at frame 4. | largest difference 1.8e-7 | yes |
| FX-SKEY-018 frame 4: Screen gain keyed from 0 at frame 0 to 200 at frame 4, linear: nothing keyed at frame 0, twice FX-SKEY-002's at frame 4. | largest difference 1.2e-7 | yes |
| FX-SKEY-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-019 frame 0: FX-SKEY-014 moved three pixels right: the same, moved. | largest difference 1.7e-7 | yes |
| FX-SKEY-019 frame 3: FX-SKEY-014 moved three pixels right: the same, moved. | largest difference 1.7e-7 | yes |
| FX-SKEY-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-020 frame 0: Screen gain 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-020 frame 4: Screen gain 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SKEY-021 frame 0: Clip black -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-021 frame 4: Clip black -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SKEY-022 frame 0: Shrink/grow 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-022 frame 4: Shrink/grow 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SKEY-023 frame 0: View "Status": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-023 frame 4: View "Status": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SKEY-024 frame 0: Screen colour "green", not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-024 frame 4: Screen colour "green", not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ssup_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ssup_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ssup_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ssup_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ssup_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ssup_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ssup_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ssup_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ssup_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ssup_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ssup_002.json is saved with its two settings, colour to suppress #00ff00 and suppression 100 | {"color_to_suppress":"#00ff00","suppression":100} | yes |
| fx_ssup_011.json is refused in a sentence naming suppression | Spill Suppressor's suppression runs from 0 to 100, and this is 101. | yes |
| fx_ssup_012.json is refused in a sentence naming suppression | Spill Suppressor's suppression runs from 0 to 100, and this is -1. | yes |
| fx_ssup_014.json is refused in a sentence naming color to suppress | Spill Suppressor's color to suppress is written #rrggbb, and this is "blue". | yes |
| fx_ssup_015.json is refused in a sentence naming color to suppress | Spill Suppressor's color to suppress is written #rrggbb, and this is "#00ff0". | yes |
| a file with a Spill Suppressor whose suppression is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Spill Suppressor without its colour is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Spill Suppressor whose colour is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| suppression 101 is refused with a sentence, and nothing changes | Spill Suppressor's suppression runs from 0 to 100, and this is 101. | yes |
| colour to suppress "green" is refused with a sentence, and nothing changes | Spill Suppressor's color to suppress is written #rrggbb, and this is "green". | yes |
| suppression keyed to 150 is refused with a sentence, and nothing changes | Spill Suppressor's suppression runs from 0 to 100, and this is 150. | yes |
| a green screen's colour picked, suppression 40 is taken | taken | yes |
| suppression keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ssup_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ssup_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ssup_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ssup_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ssup_009.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ssup_010.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ssup_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ssup_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ssup_003.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ssup_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ssup_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ssup_006.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ssup_007.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ssup_008.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ssup_009.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ssup_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ssup_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ssup_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ssup_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ssup_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_ssup_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Spill Suppressor as added (blue, 100): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1333710 pixels changed | yes |
| the reference shot, Spill Suppressor as added (blue, 100) on three layers, frame 0, Full | largest difference 1 of 255, 93975 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spill Suppressor as added (blue, 100) on three layers, frame 100, Full | largest difference 1 of 255, 83854 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spill Suppressor as added (blue, 100) on three layers, frame 239, Full | largest difference 1 of 255, 86329 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spill Suppressor as added (blue, 100) on three layers, frame 0, Draft | largest difference 1 of 255, 1365 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spill Suppressor as added (blue, 100) on three layers, frame 100, Draft | largest difference 1 of 255, 1151 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spill Suppressor as added (blue, 100) on three layers, frame 239, Draft | largest difference 1 of 255, 1209 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spill Suppressor green screen colour #28b43c, suppression 60: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1647991 pixels changed | yes |
| the reference shot, Spill Suppressor green screen colour #28b43c, suppression 60 on three layers, frame 0, Full | largest difference 1 of 255, 70439 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spill Suppressor green screen colour #28b43c, suppression 60 on three layers, frame 100, Full | largest difference 1 of 255, 66779 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spill Suppressor green screen colour #28b43c, suppression 60 on three layers, frame 239, Full | largest difference 1 of 255, 67334 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spill Suppressor green screen colour #28b43c, suppression 60 on three layers, frame 0, Draft | largest difference 1 of 255, 2779 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spill Suppressor green screen colour #28b43c, suppression 60 on three layers, frame 100, Draft | largest difference 1 of 255, 2690 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spill Suppressor green screen colour #28b43c, suppression 60 on three layers, frame 239, Draft | largest difference 1 of 255, 2651 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spill Suppressor red, suppression 30: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 121742 pixels changed | yes |
| the reference shot, Spill Suppressor red, suppression 30 on three layers, frame 0, Full | largest difference 1 of 255, 3812 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spill Suppressor red, suppression 30 on three layers, frame 100, Full | largest difference 1 of 255, 1534 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spill Suppressor red, suppression 30 on three layers, frame 239, Full | largest difference 1 of 255, 1816 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spill Suppressor red, suppression 30 on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spill Suppressor red, suppression 30 on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Spill Suppressor red, suppression 30 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-548 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| suppression 0 changes nothing: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_as_added.png, as added (blue, 100): every solid pixel's blue held no higher than the mean of its red and green, red and green untouched, each pixel within 1 level of the rule worked by hand; draws cleanly | [], worst miss 0.50 levels, 107844 pixels lose some | yes |
| 3_green.png, a green screen's colour picked (#28b43c, green leads): every solid pixel's green held no higher than the mean of its red and blue, red and blue untouched, each pixel within 1 level of the rule worked by hand; draws cleanly | [], worst miss 0.50 levels, 39924 pixels lose some | yes |
| 4_green_50.png, the same at suppression 50: green brought half way down, red and blue untouched, each pixel within 1 level of the rule worked by hand; draws cleanly | [], worst miss 0.50 levels, 36444 pixels lose some | yes |
| 5_red.png, red: every solid pixel's red held no higher than the mean of its green and blue, each pixel within 1 level of the rule worked by hand; draws cleanly | [], worst miss 0.00 levels, 16020 pixels lose some | yes |
| Spill Suppressor #0000ff at 100 against Screen Key with gain 0, balance 50, despill bias 50 and despill 100: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |
| Spill Suppressor #28b43c at 60 against Screen Key with gain 0, balance 50, despill bias 50 and despill 60: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |
| Spill Suppressor #ff0000 at 30 against Screen Key with gain 0, balance 50, despill bias 50 and despill 30: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |

## Result

169 of 169 checks pass.
