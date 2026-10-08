# B-231: the whole picture's statistics, and Stretch Levels, Stretch Contrast, Stretch Color and Spread Tones

D-351 (EFFECTS.md P0-15): a histogram of the layer's whole picture at the effect's place in its stack, and the four effects that read it, after After Effects' Auto Levels, Auto Contrast, Auto Color and Equalize, with Temporal Smoothing and Scene Detect. Every expected pixel is `Fixtures/auto_tone/expected_auto_tone.json`, written by `tools/auto_tone_reference.py` before this code existed and printed in document 25 as FX-AUTO-001 to 032. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5. FX-AUTO-023 warns TEMPORAL_SMOOTHING_SKIPPED on every frame.

## FX-AUTO-001 to 032 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-AUTO-001 frame 0: Stretch Levels as added, clips 0.1 per cent: each channel of the dull, warm drawing stretched to its own darkest and lightest, so the darkest grey nears black, the lightest white, and the warm cast is lessened. | largest difference 3.6e-7 | yes |
| FX-AUTO-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-002 frame 0: Stretch Levels, clips 5 per cent: the darkest and lightest 5 per cent of each channel go to black and white. | largest difference 4.1e-7 | yes |
| FX-AUTO-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-003 frame 0: Stretch Levels, clips 0: the drawing's own darkest and lightest in each channel become exactly 0 and 255. | largest difference 3.6e-7 | yes |
| FX-AUTO-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-004 frame 0: Stretch Contrast as added: one stretch for the three channels together, so the warm cast stays. | largest difference 2.5e-7 | yes |
| FX-AUTO-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-005 frame 0: Stretch Contrast, clips 5 per cent. | largest difference 3.1e-7 | yes |
| FX-AUTO-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-006 frame 0: Stretch Color as added: the darkest pixels' colour goes to black and the lightest pixels' to white. | largest difference 1.8e-7 | yes |
| FX-AUTO-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-007 frame 0: Stretch Color with Snap Neutral Midtones: the average colour is taken to a grey as well. | largest difference 2.0e-7 | yes |
| FX-AUTO-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-008 frame 0: Stretch Color, clips 5 per cent, Snap Neutral Midtones. | largest difference 2.3e-7 | yes |
| FX-AUTO-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-009 frame 0: Stretch Levels at Mix 50, Blend With Original 50 per cent: halfway between the drawing and FX-AUTO-001. | largest difference 2.2e-7 | yes |
| FX-AUTO-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-010 frame 0: A Levels lowering output white to 128, then Stretch Levels: the statistics are of the darkened picture, so the full range comes back. | largest difference 4.0e-7 | yes |
| FX-AUTO-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-011 frame 0: Stretch Levels, then a Levels lowering output white to 128: the stretched picture darkened. | largest difference 7.1e-8 | yes |
| FX-AUTO-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-012 frame 1: Stretch Levels on each frame of the flicker by itself: frames 1 and 2, a brighter and a darker copy of frame 0, are each stretched by their own statistics. | largest difference 3.3e-7 | yes |
| FX-AUTO-012 frame 2: Stretch Levels on each frame of the flicker by itself: frames 1 and 2, a brighter and a darker copy of frame 0, are each stretched by their own statistics. | largest difference 3.0e-7 | yes |
| FX-AUTO-012 frame 4: Stretch Levels on each frame of the flicker by itself: frames 1 and 2, a brighter and a darker copy of frame 0, are each stretched by their own statistics. | largest difference 4.2e-7 | yes |
| FX-AUTO-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-013 frame 0: Stretch Levels, Temporal Smoothing 0.1 seconds, two frames each side: frame 0 reads frames 0 to 2 (the two before are outside the layer); frame 3 reads 1 to 5, across the cut; frame 7 reads 5 to 7. | largest difference 2.6e-7 | yes |
| FX-AUTO-013 frame 3: Stretch Levels, Temporal Smoothing 0.1 seconds, two frames each side: frame 0 reads frames 0 to 2 (the two before are outside the layer); frame 3 reads 1 to 5, across the cut; frame 7 reads 5 to 7. | largest difference 2.8e-7 | yes |
| FX-AUTO-013 frame 7: Stretch Levels, Temporal Smoothing 0.1 seconds, two frames each side: frame 0 reads frames 0 to 2 (the two before are outside the layer); frame 3 reads 1 to 5, across the cut; frame 7 reads 5 to 7. | largest difference 4.1e-7 | yes |
| FX-AUTO-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-014 frame 3: FX-AUTO-013 with Scene Detect: frame 3 reads only 1 to 3 and frame 4 only 4 to 6, each side of the cut. | largest difference 2.6e-7 | yes |
| FX-AUTO-014 frame 4: FX-AUTO-013 with Scene Detect: frame 3 reads only 1 to 3 and frame 4 only 4 to 6, each side of the cut. | largest difference 3.6e-7 | yes |
| FX-AUTO-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-015 frame 2: Stretch Color, Temporal Smoothing 0.1, Scene Detect and Snap Neutral Midtones. | largest difference 1.4e-7 | yes |
| FX-AUTO-015 frame 5: Stretch Color, Temporal Smoothing 0.1, Scene Detect and Snap Neutral Midtones. | largest difference 3.6e-7 | yes |
| FX-AUTO-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-016 frame 3: Stretch Contrast, Temporal Smoothing 0.1. | largest difference 2.3e-7 | yes |
| FX-AUTO-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-017 frame 2: A Levels lowering output white to 128, then Stretch Levels with Temporal Smoothing 0.1: every frame read is the darkened picture. | largest difference 2.3e-7 | yes |
| FX-AUTO-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-018 frame 0: Spread Tones as added, RGB: each channel's values spread evenly over 0 to 255 by its own histogram. | largest difference 4.1e-8 | yes |
| FX-AUTO-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-019 frame 0: Spread Tones, Photoshop Style: one histogram of the three channels for all. | largest difference 4.1e-8 | yes |
| FX-AUTO-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-020 frame 0: Spread Tones, Brightness: each pixel scaled so its brightness is spread. | largest difference 1.6e-7 | yes |
| FX-AUTO-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-021 frame 0: Spread Tones, RGB, amount 50: halfway. | largest difference 1.1e-7 | yes |
| FX-AUTO-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-022 frame 0: Stretch Levels on an adjustment layer above the holder: the frame beneath is the drawing, so FX-AUTO-001. | largest difference 3.6e-7 | yes |
| FX-AUTO-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-023 frame 0: Stretch Levels with Temporal Smoothing 0.1 on an adjustment layer: the frames beneath at other times are not to hand, so each frame is stretched by itself, with a warning each frame. | largest difference 3.6e-7 | yes |
| FX-AUTO-023 frame 3: Stretch Levels with Temporal Smoothing 0.1 on an adjustment layer: the frames beneath at other times are not to hand, so each frame is stretched by itself, with a warning each frame. | largest difference 3.6e-7 | yes |
| FX-AUTO-023: what opening it warns of, and what frame 4 warns of | [] and ["TEMPORAL_SMOOTHING_SKIPPED"] | yes |
| FX-AUTO-024 frame 0: Black clip keyed from 0 at frame 0 to 10 at frame 4: 5 at frame 2. | largest difference 3.6e-7 | yes |
| FX-AUTO-024 frame 2: Black clip keyed from 0 at frame 0 to 10 at frame 4: 5 at frame 2. | largest difference 3.5e-7 | yes |
| FX-AUTO-024 frame 4: Black clip keyed from 0 at frame 0 to 10 at frame 4: 5 at frame 2. | largest difference 5.6e-7 | yes |
| FX-AUTO-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-025 frame 0: FX-AUTO-001 with the holder moved 3 pixels right, its last columns off the composition: the statistics are of the whole layer, so the same, moved. | largest difference 2.6e-7 | yes |
| FX-AUTO-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AUTO-026 frame 0: Black clip 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-AUTO-026 frame 4: Black clip 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-AUTO-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AUTO-027 frame 0: White clip -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-AUTO-027 frame 4: White clip -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-AUTO-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AUTO-028 frame 0: Temporal Smoothing 11 seconds, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-AUTO-028 frame 4: Temporal Smoothing 11 seconds, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-AUTO-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AUTO-029 frame 0: Scene Detect written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-AUTO-029 frame 4: Scene Detect written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-AUTO-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AUTO-030 frame 0: Snap Neutral Midtones written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-AUTO-030 frame 4: Snap Neutral Midtones written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-AUTO-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AUTO-031 frame 0: Spread Tones' equalize written "hsl". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-AUTO-031 frame 4: Spread Tones' equalize written "hsl". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-AUTO-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AUTO-032 frame 0: Spread Tones' amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-AUTO-032 frame 4: Spread Tones' amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.1e-7 | yes |
| FX-AUTO-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_auto_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_auto_001.json: Stretch Levels and Stretch Contrast write no snap neutral midtones | {"black_clip":0.1,"scene_detect":"off","temporal_smoothing":0,"white_clip":0.1} | yes |
| fx_auto_004.json: Stretch Levels and Stretch Contrast write no snap neutral midtones | {"black_clip":0.1,"scene_detect":"off","temporal_smoothing":0,"white_clip":0.1} | yes |
| fx_auto_013.json: the statistics a smoothing added up are never saved | {"black_clip":0.1,"scene_detect":"off","temporal_smoothing":0.1,"white_clip":0.1} | yes |
| fx_auto_015.json: the statistics a smoothing added up are never saved | {"black_clip":0.1,"scene_detect":"on","snap_neutral_midtones":"on","temporal_smoothing":0.1,"white_clip":0.1} | yes |
| a file with no black clip is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a black clip written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with Spread Tones with no amount is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| fx_auto_026.json is refused in a sentence naming it | Stretch Levels's black clip runs from 0 to 10, and this is 11. | yes |
| fx_auto_029.json is refused in a sentence naming it | Stretch Levels's scene detect is "off" or "on", and this is "yes". | yes |
| fx_auto_030.json is refused in a sentence naming it | Stretch Color's snap neutral midtones is "off" or "on", and this is "maybe". | yes |
| fx_auto_031.json is refused in a sentence naming it | Spread Tones equalizes by "rgb", "brightness" or "photoshop", and this is "hsl". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| black clip 11 is refused with a sentence, and nothing changes | Stretch Levels's black clip runs from 0 to 10, and this is 11. | yes |
| temporal smoothing -1 is refused with a sentence, and nothing changes | Stretch Levels's temporal smoothing runs from 0 to 10, and this is -1. | yes |
| scene detect "yes" is refused with a sentence, and nothing changes | Stretch Levels's scene detect is "off" or "on", and this is "yes". | yes |
| clips 5 and 3, smoothing 0.1, scene detect on, is taken | taken | yes |
| black clip keyed 0 to 10 over frames 0 to 4, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_auto_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_auto_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_auto_013.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_auto_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_auto_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_auto_025.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Only the part on screen (B-158): the statistics are still the whole picture's

| Check | The build's answer | Matches |
| --- | --- | --- |
| Stretch Levels, the middle quarter of the street drawn alone: the same pixels as the whole frame's | byte-identical | yes |
| Spread Tones, RGB, the middle quarter of the street drawn alone: the same pixels as the whole frame's | byte-identical | yes |

## Draft against Full: the statistics of a quarter-size picture

| Check | The build's answer | Matches |
| --- | --- | --- |
| the dull street's black and white points, clips 0.1 per cent, in levels of 255 (Draft reads its own quarter-size picture) | red 92-210 at Full, 92-210 at Draft; green 82-192 at Full, 82-187 at Draft; blue 71-172 at Full, 71-161 at Draft; brightness 83-194 at Full, 83-188 at Draft | yes |
| the dull street's black and white points, clips 2 per cent, in levels of 255 (Draft reads its own quarter-size picture) | red 103-210 at Full, 103-191 at Draft; green 88-187 at Full, 88-168 at Draft; blue 71-161 at Full, 71-161 at Draft; brightness 90-188 at Full, 90-168 at Draft | yes |
| Draft against Full, the largest difference in a black point (the street's dark road is wide, so it survives the quarter size): within 3 levels of 255 | 0 level(s) | yes |
| Draft against Full, the largest difference in a white point, recorded as it is: the lit windows are a few pixels wide and average away at a quarter size, so Draft stretches harder than Full. Export is always Full | 20 level(s) | yes |

## Pictures: a dull, warm street, in `verification/D-351 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the street greyed and warm: no true black, no true white, a red-orange cast | warnings [] | yes |
| stretch_levels.png, Stretch Levels as added: each channel to its own darkest and lightest, so blacks are black, whites white, and the cast much less | darkest 0, lightest 255 (before: 71, 210); warnings [] | yes |
| stretch_contrast.png, Stretch Contrast as added: darker darks and lighter lights, the warm cast kept | darkest 0, lightest 255 (before: 71, 210); warnings [] | yes |
| stretch_color_snap.png, Stretch Color with Snap Neutral Midtones: the warm cast taken out: the sky blue again, the road near black | darkest 0, lightest 255 (before: 71, 210); warnings [] | yes |
| spread_tones.png, Spread Tones, RGB: every tone spread evenly, strong and a little posterised | darkest 0, lightest 255 (before: 71, 210); warnings [] | yes |

## Result

146 of 146 checks pass.
