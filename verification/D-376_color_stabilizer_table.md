# D-376: Color Stabilizer

B-255, after After Effects' Color Stabilizer: one, two or three points sampled at a reference frame, and every frame corrected so the same points match them, by brightness, levels or curves. Every expected pixel is `Fixtures/color_stabilizer/expected_color_stabilizer.json`, written by `tools/color_stabilizer_reference.py` before this code existed and printed in document 25 as FX-CSTAB-001 to 016. Tolerance 2e-5.

## FX-CSTAB-001 to 016 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CSTAB-001 frame 0: The settings as added: brightness, reference frame 0, sample size 5. Frame 0 is the reference, so untouched; frame 1, the ramp lifted by 20, comes back to frame 0; frames 2 and 3 move by their black point's brightness only. | largest difference 1.4e-7 | yes |
| FX-CSTAB-001 frame 1: The settings as added: brightness, reference frame 0, sample size 5. Frame 0 is the reference, so untouched; frame 1, the ramp lifted by 20, comes back to frame 0; frames 2 and 3 move by their black point's brightness only. | largest difference 1.4e-7 | yes |
| FX-CSTAB-001 frame 2: The settings as added: brightness, reference frame 0, sample size 5. Frame 0 is the reference, so untouched; frame 1, the ramp lifted by 20, comes back to frame 0; frames 2 and 3 move by their black point's brightness only. | largest difference 1.5e-7 | yes |
| FX-CSTAB-001 frame 3: The settings as added: brightness, reference frame 0, sample size 5. Frame 0 is the reference, so untouched; frame 1, the ramp lifted by 20, comes back to frame 0; frames 2 and 3 move by their black point's brightness only. | largest difference 1.2e-7 | yes |
| FX-CSTAB-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CSTAB-002 frame 1: Levels: each channel mapped through the black and white samples, so frame 2's colour cast and contrast are taken out, to the drawing's own rounding. | largest difference 1.4e-7 | yes |
| FX-CSTAB-002 frame 2: Levels: each channel mapped through the black and white samples, so frame 2's colour cast and contrast are taken out, to the drawing's own rounding. | largest difference 1.7e-7 | yes |
| FX-CSTAB-002 frame 3: Levels: each channel mapped through the black and white samples, so frame 2's colour cast and contrast are taken out, to the drawing's own rounding. | largest difference 1.4e-7 | yes |
| FX-CSTAB-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CSTAB-003 frame 2: Curves: through black, mid and white, so frame 3's raised mid-tones come down too. | largest difference 1.6e-7 | yes |
| FX-CSTAB-003 frame 3: Curves: through black, mid and white, so frame 3's raised mid-tones come down too. | largest difference 1.5e-7 | yes |
| FX-CSTAB-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CSTAB-004 frame 0: Levels with reference frame 2: frame 0, the plain ramp, is given frame 2's cast. | largest difference 1.5e-7 | yes |
| FX-CSTAB-004 frame 2: Levels with reference frame 2: frame 0, the plain ramp, is given frame 2's cast. | largest difference 1.4e-7 | yes |
| FX-CSTAB-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CSTAB-005 frame 0: Reference frame 2.7, taken down to 2: the same as FX-CSTAB-004. | largest difference 1.5e-7 | yes |
| FX-CSTAB-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CSTAB-006 frame 2: Sample size 0: each sample the one pixel holding its point. | largest difference 2.2e-7 | yes |
| FX-CSTAB-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CSTAB-007 frame 2: Sample size keyed from 1 at frame 0 to 9 at frame 4: the reference samples taken at size 1 as frame 0 has it, the current ones at 5 at frame 2 and 7 at frame 3. | largest difference 2.9e-7 | yes |
| FX-CSTAB-007 frame 3: Sample size keyed from 1 at frame 0 to 9 at frame 4: the reference samples taken at size 1 as frame 0 has it, the current ones at 5 at frame 2 and 7 at frame 3. | largest difference 4.2e-7 | yes |
| FX-CSTAB-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CSTAB-008 frame 2: The black point at (10, 20) and the white point at (90, 80), levels: the white sample reaches the half-covered column, counted at half weight. | largest difference 1.6e-7 | yes |
| FX-CSTAB-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CSTAB-009 frame 2: The black point keyed from (25, 50) at frame 0 to (5, 50) at frame 4, levels: the reference sample is taken where the point is at frame 0, the current one where it is at frame 2. | largest difference 1.6e-7 | yes |
| FX-CSTAB-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CSTAB-010 frame 1: Reference frame 20, after the layer's last frame: no reference, so every frame as it is. | largest difference 1.5e-7 | yes |
| FX-CSTAB-010 frame 2: Reference frame 20, after the layer's last frame: no reference, so every frame as it is. | largest difference 1.4e-7 | yes |
| FX-CSTAB-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CSTAB-011 frame 2: A Levels lowering output white to 200, then the stabilizer, levels: the samples are of the darkened pictures, so frame 2 comes back to the darkened frame 0. | largest difference 1.0e-7 | yes |
| FX-CSTAB-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CSTAB-012 frame 1: On an adjustment layer above the holder: no reference frame, so each frame as it is, with a warning each frame. | largest difference 1.5e-7 | yes |
| FX-CSTAB-012 frame 2: On an adjustment layer above the holder: no reference frame, so each frame as it is, with a warning each frame. | largest difference 1.4e-7 | yes |
| FX-CSTAB-012: what opening it warns of, and what frame 4 warns of | [] and ["TEMPORAL_SMOOTHING_SKIPPED"] | yes |
| FX-CSTAB-013 frame 0: Stabilize "colour", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-CSTAB-013 frame 4: Stabilize "colour", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-CSTAB-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CSTAB-014 frame 0: Stabilize "Levels": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-CSTAB-014 frame 4: Stabilize "Levels": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-CSTAB-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CSTAB-015 frame 0: Reference frame -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-CSTAB-015 frame 4: Reference frame -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-CSTAB-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CSTAB-016 frame 0: Sample size 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-CSTAB-016 frame 4: Sample size 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 | yes |
| FX-CSTAB-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cstab_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cstab_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cstab_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cstab_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cstab_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cstab_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cstab_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cstab_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cstab_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cstab_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cstab_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cstab_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cstab_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cstab_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cstab_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cstab_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cstab_008.json is saved with its mode, reference frame, three points and sample size, and no samples | {"black_point":[10,20],"mid_point":[50,50],"reference_frame":0,"sample_size":5,"stabilize":"levels","white_point":[90,80]} | yes |
| fx_cstab_013.json is refused in a sentence naming "colour" | Color Stabilizer's stabilize is "brightness", "levels" or "curves", and this is "colour". | yes |
| fx_cstab_014.json is refused in a sentence naming "Levels" | Color Stabilizer's stabilize is "brightness", "levels" or "curves", and this is "Levels". | yes |
| a file with a Color Stabilizer whose black point is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Color Stabilizer with no `reference_frame` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| sample size 100.5 is refused with a sentence, and nothing changes | Color Stabilizer's sample size runs from 0 to 100, and this is 100.5. | yes |
| reference frame -0.5 is refused with a sentence, and nothing changes | Color Stabilizer's reference frame runs from 0 to 1000000, and this is -0.5. | yes |
| stabilize "color" is refused with a sentence, and nothing changes | Color Stabilizer's stabilize is "brightness", "levels" or "curves", and this is "color". | yes |
| the white point at x 1000.5 is refused with a sentence, and nothing changes | Color Stabilizer's white point runs from -1000 to 1000, and this is 1000.5. | yes |
| curves at reference frame 2 is taken | taken | yes |
| the black point keyed from (25, 50) to (5, 50) is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cstab_001.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cstab_002.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cstab_003.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cstab_008.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| Color Stabilizer stays on the processor, since its reference frame is read there; the card draws the rest of the frame round it | every file below: 0 frames on the card | yes |
| fx_cstab_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cstab_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cstab_003.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cstab_004.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cstab_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cstab_006.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cstab_007.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cstab_008.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cstab_009.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cstab_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cstab_011.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cstab_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cstab_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cstab_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cstab_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cstab_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, an Exposure Flicker then Color Stabilizer brightness, reference frame 0: the processor's frame 100 differs from the flicker alone, so the comparisons below test the effect | 1957660 pixels changed | yes |
| the reference shot, an Exposure Flicker then Color Stabilizer brightness, reference frame 0 on three layers, frame 0, Full | largest difference 1 of 255, 18703 pixels differ; refused by the card: false; warnings CPU [] GPU [] | yes |
| the reference shot, an Exposure Flicker then Color Stabilizer brightness, reference frame 0 on three layers, frame 100, Full | largest difference 1 of 255, 44101 pixels differ; refused by the card: false; warnings CPU [] GPU [] | yes |
| the reference shot, an Exposure Flicker then Color Stabilizer brightness, reference frame 0 on three layers, frame 239, Full | largest difference 1 of 255, 41481 pixels differ; refused by the card: false; warnings CPU [] GPU [] | yes |
| the reference shot, an Exposure Flicker then Color Stabilizer brightness, reference frame 0 on three layers, frame 0, Draft | largest difference 1 of 255, 2731 pixels differ; refused by the card: false; warnings CPU [] GPU [] | yes |
| the reference shot, an Exposure Flicker then Color Stabilizer brightness, reference frame 0 on three layers, frame 100, Draft | largest difference 1 of 255, 3606 pixels differ; refused by the card: false; warnings CPU [] GPU [] | yes |
| the reference shot, an Exposure Flicker then Color Stabilizer brightness, reference frame 0 on three layers, frame 239, Draft | largest difference 1 of 255, 5080 pixels differ; refused by the card: false; warnings CPU [] GPU [] | yes |
| the reference shot, an Exposure Flicker then Color Stabilizer curves, reference frame 120: the processor's frame 100 differs from the flicker alone, so the comparisons below test the effect | 1965505 pixels changed | yes |
| the reference shot, an Exposure Flicker then Color Stabilizer curves, reference frame 120 on three layers, frame 0, Full | largest difference 1 of 255, 67725 pixels differ; refused by the card: false; warnings CPU [] GPU [] | yes |
| the reference shot, an Exposure Flicker then Color Stabilizer curves, reference frame 120 on three layers, frame 100, Full | largest difference 1 of 255, 70528 pixels differ; refused by the card: false; warnings CPU [] GPU [] | yes |
| the reference shot, an Exposure Flicker then Color Stabilizer curves, reference frame 120 on three layers, frame 239, Full | largest difference 1 of 255, 67578 pixels differ; refused by the card: false; warnings CPU [] GPU [] | yes |
| the reference shot, an Exposure Flicker then Color Stabilizer curves, reference frame 120 on three layers, frame 0, Draft | largest difference 1 of 255, 3496 pixels differ; refused by the card: false; warnings CPU [] GPU [] | yes |
| the reference shot, an Exposure Flicker then Color Stabilizer curves, reference frame 120 on three layers, frame 100, Draft | largest difference 1 of 255, 3065 pixels differ; refused by the card: false; warnings CPU [] GPU [] | yes |
| the reference shot, an Exposure Flicker then Color Stabilizer curves, reference frame 120 on three layers, frame 239, Draft | largest difference 1 of 255, 3668 pixels differ; refused by the card: false; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-376 pictures/`, each pixel grown 24 times

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_reference_frame_0.png, 2_frame_2_before.png and 3_frame_2_levels.png: frame 2's colour cast and flattened contrast (before) are taken out by Levels, so it looks like frame 0 again | frame 2 against frame 0: largest difference 23 of 255 before, 1 after | yes |
| 4_frame_1_before.png and 5_frame_1_brightness.png: frame 1, lifted by 20, is brought back down to frame 0's brightness | frame 1 against frame 0: largest difference 20 of 255 before, 0 after | yes |

## Result

111 of 111 checks pass.
