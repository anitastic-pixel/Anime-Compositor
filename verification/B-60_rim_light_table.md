# B-60: rim light

D-117, accepted by the owner on 2026-09-26 in the batch of ten. Every expected pixel is `Fixtures/rim_light/expected_rim_light.json`, written by `tools/rim_light_reference.py` before this code existed and printed in document 25 as FX-RIM-001 to 028. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-RIM-001 to 028 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RIM-001 frame 0: The settings as they start: white light from 45 degrees, the upper right, width 3, softness 1, intensity 100, normal. The block's top two rows and right two columns turn white, with a fainter third; the red dot, alone, turns wholly white, and itself shades the line's end at (12, 7), whose point toward the light falls beside the dot; the empty pixels stay empty and the block's lower left stays skin. | largest difference 2.1e-7 | yes |
| FX-RIM-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-002 frame 0: Direction 0, light from straight above: the block's top three rows turn white and the fourth takes a trace of it; the soft edge, whose own covering is half, is half lit down its whole length. | largest difference 2.0e-7 | yes |
| FX-RIM-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-003 frame 0: Direction 90, light from the right: the block's right three columns turn white, all but (11, 5), whose point three pixels to the right is the red dot, which shades it; the soft edge on the left is not lit. | largest difference 2.0e-7 | yes |
| FX-RIM-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-004 frame 0: Direction 180, light from below: the line along the block's foot turns white, with the two rows of skin above it, and the third row a trace. | largest difference 2.0e-7 | yes |
| FX-RIM-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-005 frame 0: Direction 270, light from the left: the soft edge turns wholly white at its half covering, and the block's left two columns beside it; the third, whose point three pixels to the left is the soft edge's centre, is half lit. | largest difference 2.0e-7 | yes |
| FX-RIM-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-006 frame 0: Width 0, softness 0: rim = 1 - covering, so only the soft edge, half covered, is lit, by 127/255; every fully covered pixel is untouched. | largest difference 1.9e-7 | yes |
| FX-RIM-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-007 frame 0: Width 1: a rim about a pixel deep on the top and right. | largest difference 2.1e-7 | yes |
| FX-RIM-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-008 frame 0: Width 2.5: between widths 1 and 3, the point sampled falling between pixels. | largest difference 1.9e-7 | yes |
| FX-RIM-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-009 frame 0: Softness 0, direction 0: the rim is hard, the top three rows wholly white and the rest untouched, where FX-RIM-002, at softness 1, gave the fourth row a trace. | largest difference 1.9e-7 | yes |
| FX-RIM-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-010 frame 0: Intensity 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-RIM-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-011 frame 0: Intensity 50: every pixel halfway between the drawing and FX-RIM-001. | largest difference 2.0e-7 | yes |
| FX-RIM-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-012 frame 0: Colour #ffb040, an orange, blend normal: FX-RIM-001's rim in orange. | largest difference 2.1e-7 | yes |
| FX-RIM-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-013 frame 0: Colour #ffb040, blend add: the orange added onto the skin, brighter than white on its red, past what a screen reaches. | largest difference 2.6e-7 | yes |
| FX-RIM-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-014 frame 0: Colour #ffb040, blend screen: lighter than the skin, never past white. | largest difference 2.1e-7 | yes |
| FX-RIM-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-015 frame 0: Colour #ffb040, blend multiply: the rim darkens the skin toward orange, and red, fully lit, keeps its own red channel. | largest difference 1.9e-7 | yes |
| FX-RIM-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-016 frame 0: FX-RIM-013 with the colour written in capitals, #FFB040: the same. | largest difference 2.6e-7 | yes |
| FX-RIM-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-017 frame 0: Direction keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 is FX-RIM-002, frame 2, at 90, FX-RIM-003, and frame 4 FX-RIM-004. | largest difference 2.0e-7 | yes |
| FX-RIM-017 frame 2: Direction keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 is FX-RIM-002, frame 2, at 90, FX-RIM-003, and frame 4 FX-RIM-004. | largest difference 2.0e-7 | yes |
| FX-RIM-017 frame 4: Direction keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 is FX-RIM-002, frame 2, at 90, FX-RIM-003, and frame 4 FX-RIM-004. | largest difference 2.0e-7 | yes |
| FX-RIM-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-018 frame 0: Width keyed from 0 at frame 0 to 4 at frame 4, linear: the rim deepens, width 2 at frame 2. | largest difference 2.0e-7 | yes |
| FX-RIM-018 frame 2: Width keyed from 0 at frame 0 to 4 at frame 4, linear: the rim deepens, width 2 at frame 2. | largest difference 1.9e-7 | yes |
| FX-RIM-018 frame 4: Width keyed from 0 at frame 0 to 4 at frame 4, linear: the rim deepens, width 2 at frame 2. | largest difference 2.1e-7 | yes |
| FX-RIM-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-019 frame 0: Intensity eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-RIM-001; frame 0 is the drawing. | largest difference 1.9e-7 | yes |
| FX-RIM-019 frame 2: Intensity eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-RIM-001; frame 0 is the drawing. | largest difference 2.1e-7 | yes |
| FX-RIM-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-020 frame 0: Direction 450, a turn and a quarter: FX-RIM-003 exactly. | largest difference 2.0e-7 | yes |
| FX-RIM-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-021 frame 0: FX-RIM-001 moved three pixels right: the same, moved; the layer does not grow. | largest difference 2.1e-7 | yes |
| FX-RIM-021 frame 3: FX-RIM-001 moved three pixels right: the same, moved; the layer does not grow. | largest difference 2.1e-7 | yes |
| FX-RIM-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIM-022 frame 0: Width 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIM-022 frame 4: Width 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIM-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RIM-023 frame 0: Intensity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIM-023 frame 4: Intensity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIM-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RIM-024 frame 0: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIM-024 frame 4: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIM-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RIM-025 frame 0: Direction -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIM-025 frame 4: Direction -3601, below -3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIM-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RIM-026 frame 0: Width keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIM-026 frame 4: Width keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIM-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RIM-027 frame 0: Blend "overlay", which is not a blend. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIM-027 frame 4: Blend "overlay", which is not a blend. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIM-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RIM-028 frame 0: Colour "#fff", written in three digits, not six. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIM-028 frame 4: Colour "#fff", written in three digits, not six. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIM-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: only pixels that show are lit | 0 | yes |
| a half-size draft preview halves the width and the softness, and nothing else | RimLight { color: "#ffffff", direction: 45.0, width: 2.0, softness: 1.0, intensity: 100.0, blend: "normal" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rim_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rim_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rim_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rim_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rim_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rim_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rim_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rim_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rim_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rim_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rim_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rim_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rim_016.json, its colour in capitals, is saved with it in small letters | "#ffb040" | yes |
| a file with no `blend` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a width written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| width 101 is refused with a sentence, and nothing changes | Rim Light's width runs from 0 to 100, and this is 101. | yes |
| width -1 is refused with a sentence, and nothing changes | Rim Light's width runs from 0 to 100, and this is -1. | yes |
| softness 101 is refused with a sentence, and nothing changes | Rim Light's softness runs from 0 to 100, and this is 101. | yes |
| intensity -1 is refused with a sentence, and nothing changes | Rim Light's intensity runs from 0 to 100, and this is -1. | yes |
| intensity 101 is refused with a sentence, and nothing changes | Rim Light's intensity runs from 0 to 100, and this is 101. | yes |
| direction -3601 is refused with a sentence, and nothing changes | Rim Light's direction runs from -3600 to 3600, and this is -3601. | yes |
| blend "overlay" is refused with a sentence, and nothing changes | Rim Light's blend is "normal", "add", "screen" or "multiply", and this is "overlay". | yes |
| colour "#fff" is refused with a sentence, and nothing changes | Rim Light's colour is written #rrggbb, and this is "#fff". | yes |
| width keyed to 150 is refused with a sentence, and nothing changes | Rim Light's width runs from 0 to 100, and this is 150. | yes |
| width, softness and intensity 100, direction 3600, the tops, is taken | taken | yes |
| width, softness and intensity 0, direction -3600, the bottoms, is taken | taken | yes |
| direction keyed from 0 to 180 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rim_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rim_018.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

101 of 101 checks pass.
