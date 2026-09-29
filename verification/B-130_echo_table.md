# B-130: echo

D-195, accepted by the owner on 2026-09-28 ("take everything"). Every expected pixel is `Fixtures/echo/expected_echo.json`, written by `tools/echo_reference.py` before this code existed and printed in document 25 as FX-ECHO-001 to 030. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-ECHO-001 to 030 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-ECHO-001 frame 0: As added: one echo, one frame back, starting intensity and decay 1, Add. At frame 0 the frame before is outside the layer, so only the ball; at frame 2 drawings 3 and 2 added, brighter where they overlap; at frame 5 drawing 5, held, added to itself. | largest difference 9.1e-8 | yes |
| FX-ECHO-001 frame 2: As added: one echo, one frame back, starting intensity and decay 1, Add. At frame 0 the frame before is outside the layer, so only the ball; at frame 2 drawings 3 and 2 added, brighter where they overlap; at frame 5 drawing 5, held, added to itself. | largest difference 1.2e-7 | yes |
| FX-ECHO-001 frame 5: As added: one echo, one frame back, starting intensity and decay 1, Add. At frame 0 the frame before is outside the layer, so only the ball; at frame 2 drawings 3 and 2 added, brighter where they overlap; at frame 5 drawing 5, held, added to itself. | largest difference 1.6e-7 | yes |
| FX-ECHO-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-002 frame 0: No echoes: the layer exactly as it is. | largest difference 9.1e-8 | yes |
| FX-ECHO-002 frame 3: No echoes: the layer exactly as it is. | largest difference 9.1e-8 | yes |
| FX-ECHO-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-003 frame 3: No echoes, starting intensity 0.5: the ball at half strength. | largest difference 4.6e-8 | yes |
| FX-ECHO-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-004 frame 4: Three echoes one frame apart, decay 0.5, Add: drawing 5 whole, 4 at a half, 3 at a quarter, 2 at an eighth. | largest difference 1.5e-7 | yes |
| FX-ECHO-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-005 frame 1: Echo time +1, two echoes: the frames after. At frame 1 drawings 2, 3 and 4; at frame 6 drawing 5 twice and frame 8, past the out point, empty. | largest difference 1.7e-7 | yes |
| FX-ECHO-005 frame 6: Echo time +1, two echoes: the frames after. At frame 1 drawings 2, 3 and 4; at frame 6 drawing 5 twice and frame 8, past the out point, empty. | largest difference 1.6e-7 | yes |
| FX-ECHO-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-006 frame 4: Echo time -2, two echoes: drawings 5, 3 and 1 at frame 4. | largest difference 9.8e-8 | yes |
| FX-ECHO-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-007 frame 4: Three echoes, decay 0.7, Maximum. | largest difference 9.7e-8 | yes |
| FX-ECHO-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-008 frame 3: One echo, Minimum: only where drawings 4 and 3 overlap is anything left. | largest difference 9.8e-8 | yes |
| FX-ECHO-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-009 frame 4: Three echoes, decay 0.7, Screen. | largest difference 9.7e-8 | yes |
| FX-ECHO-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-010 frame 4: Three echoes, decay 0.7, Composite In Back: the present ball on top. | largest difference 1.2e-7 | yes |
| FX-ECHO-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-011 frame 4: Three echoes, decay 0.7, Composite In Front: the furthest echo on top. | largest difference 1.2e-7 | yes |
| FX-ECHO-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-012 frame 4: Three echoes, Blend: the four drawings averaged. | largest difference 4.6e-8 | yes |
| FX-ECHO-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-013 frame 3: The layer's in point at frame 2, three echoes: at frame 3 its drawings 2 and 1; frames 1 and 0 are before it, empty. | largest difference 1.4e-7 | yes |
| FX-ECHO-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-014 frame 2: Drawings on twos, one echo: at frame 2 drawings 2 and 1; at frame 3 drawing 2 added to itself. | largest difference 1.4e-7 | yes |
| FX-ECHO-014 frame 3: Drawings on twos, one echo: at frame 2 drawings 2 and 1; at frame 3 drawing 2 added to itself. | largest difference 1.6e-7 | yes |
| FX-ECHO-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-015 frame 0: Number of echoes keyed from 0 at frame 0 to 3 at frame 4, decay 0.5: 1.5 at frame 2 counts as 1. | largest difference 9.1e-8 | yes |
| FX-ECHO-015 frame 2: Number of echoes keyed from 0 at frame 0 to 3 at frame 4, decay 0.5: 1.5 at frame 2 counts as 1. | largest difference 1.5e-7 | yes |
| FX-ECHO-015 frame 4: Number of echoes keyed from 0 at frame 0 to 3 at frame 4, decay 0.5: 1.5 at frame 2 counts as 1. | largest difference 1.5e-7 | yes |
| FX-ECHO-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-016 frame 4: Echo time -1.5 counts as -2: FX-ECHO-006. | largest difference 9.8e-8 | yes |
| FX-ECHO-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-017 frame 2: An Exposure of +1 before the Echo is not seen: Echo reads the drawing, not what came before it. FX-ECHO-001. | largest difference 1.2e-7 | yes |
| FX-ECHO-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-018 frame 2: An Exposure of -1 after the Echo darkens the trail too. | largest difference 5.9e-8 | yes |
| FX-ECHO-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-019 frame 3: A mask keeping columns 0 to 6, two echoes: every echo is masked. | largest difference 1.4e-7 | yes |
| FX-ECHO-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-020 frame 2: The holder moved 2 right and 3 down: FX-ECHO-001 moved, since the echoes lie on the layer. | largest difference 1.2e-7 | yes |
| FX-ECHO-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-021 frame 2: The effect on an adjustment layer above the holder: nothing changes. | largest difference 9.8e-8 | yes |
| FX-ECHO-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-022 frame 2: Echo time 0, two echoes, decay 0.5: the same drawing three times, 1.75 of it, held at 1. | largest difference 1.7e-7 | yes |
| FX-ECHO-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-023 frame 0: Starting intensity keyed from 1 at frame 0 to 0 at frame 4: at frame 4 nothing is left. | largest difference 9.1e-8 | yes |
| FX-ECHO-023 frame 2: Starting intensity keyed from 1 at frame 0 to 0 at frame 4: at frame 4 nothing is left. | largest difference 9.1e-8 | yes |
| FX-ECHO-023 frame 4: Starting intensity keyed from 1 at frame 0 to 0 at frame 4: at frame 4 nothing is left. | largest difference 0.0e0 | yes |
| FX-ECHO-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-024 frame 0: Number of echoes 31, above 30. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-ECHO-024 frame 4: Number of echoes 31, above 30. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.7e-8 | yes |
| FX-ECHO-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ECHO-025 frame 0: Number of echoes -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-ECHO-025 frame 4: Number of echoes -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.7e-8 | yes |
| FX-ECHO-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ECHO-026 frame 0: Starting intensity 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-ECHO-026 frame 4: Starting intensity 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.7e-8 | yes |
| FX-ECHO-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ECHO-027 frame 0: Decay -0.1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-ECHO-027 frame 4: Decay -0.1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.7e-8 | yes |
| FX-ECHO-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ECHO-028 frame 0: Echo time 121, above 120. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-ECHO-028 frame 4: Echo time 121, above 120. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.7e-8 | yes |
| FX-ECHO-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ECHO-029 frame 0: Echo time keyed to -200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-ECHO-029 frame 4: Echo time keyed to -200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.7e-8 | yes |
| FX-ECHO-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ECHO-030 frame 0: An operator written "multiply". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-ECHO-030 frame 4: An operator written "multiply". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.7e-8 | yes |
| FX-ECHO-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| an echo never grows the drawing's bounds | 0 | yes |
| a half-size draft preview leaves every setting as it is, since none is a distance | Echo { echo_time: -2.0, echoes: 6.0, intensity: 0.9, decay: 0.7, operator: "screen", picture: None } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_echo_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_echo_019.json, its mask in the first form, is saved as one mask of the same four corners in the current form, and the saved file draws frame 3 the same | 1 mask(s), frame 3 byte-identical | yes |
| the echoes it lays are never saved | None | yes |
| a file with no `operator` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with no `echoes` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an echo time that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an operator that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Number Of Echoes 31 is refused with a sentence, and nothing changes | Echo's echoes runs from 0 to 30, and this is 31. | yes |
| Starting Intensity 1.5 is refused with a sentence, and nothing changes | Echo's intensity runs from 0 to 1, and this is 1.5. | yes |
| Decay -0.1 is refused with a sentence, and nothing changes | Echo's decay runs from 0 to 1, and this is -0.1. | yes |
| Echo Time 121 is refused with a sentence, and nothing changes | Echo's echo time runs from -120 to 120, and this is 121. | yes |
| Echo Operator "multiply" is refused with a sentence, and nothing changes | Echo's operator is "add", "maximum", "minimum", "screen", "composite_in_back", "composite_in_front" or "blend", and this is "multiply". | yes |
| Number Of Echoes keyed to 40 at frame 4 is refused with a sentence, and nothing changes | Echo's echoes runs from 0 to 30, and this is 40. | yes |
| Screen with three echoes two frames back is taken | taken | yes |
| Echo Time keyed from -1 at frame 0 to -3 at frame 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a ball bouncing on ones over a night-blue solid, in `verification/B-130 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, frame 16 with no effect, draws cleanly, the ball in columns 282 to 330 | [], columns 282 to 329 | yes |
| trail_add.png, 6 echoes 2 frames back, decay 0.7, Add: a trail back to drawing 5, brighter where the balls overlap, draws cleanly in columns 90 to 330 | [], columns 90 to 329, 352 pixels of the present ball brighter | yes |
| trail_in_back.png, the same, Composite In Back: the present ball whole on top, draws cleanly in columns 90 to 330 | [], columns 90 to 329, 0 pixels of the present ball differ from before.png | yes |
| smear.png, 11 echoes 1 frame back, decay 0.8, Add: a smear back to drawing 6, draws cleanly in columns 106 to 330 | [], columns 106 to 329 | yes |
| ahead_screen.png, Echo Time +2, 3 echoes, decay 0.5, Screen: where the ball is going, to drawing 23, draws cleanly in columns 282 to 426 | [], columns 282 to 425 | yes |
| blend.png, 3 echoes 1 frame back, Blend: four drawings averaged, draws cleanly in columns 234 to 330 | [], columns 234 to 329 | yes |
| trail_add at Draft draws cleanly, a quarter the size: the trail in columns 22 to 83 (the last row, half covered by the solid since 270 is no whole number of quarters, left out) | [], 120 wide, columns 22 to 82 | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_echo_004.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_echo_010.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_echo_011.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_echo_019.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_echo_020.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

134 of 134 checks pass.
