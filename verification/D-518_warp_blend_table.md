# D-518: Warp Blend

B-398: `core.warp_blend`, after CycoreFX's CC WarpoMatic (Transition), under our own name: the layer and the Layer to Reveal (D-189's layer setting, centred on the drawing; nothing when none is chosen) are pulled out of shape along what differs between them while one blends into the other. CycoreFX names the controls but publishes no formula and no defaults, so the rule and the settings when added are ours, stated in `tools/warp_blend_reference.py`: a reactor worked from the two layers (Brightness, Contrast Differences, Brightness Differences or Local Differences), blurred by the Smoothness; its slope times the Warp Amount is the pull, turned a quarter when Twisting; the layer is read pulled by the completion's share of it and the reveal by the rest, the same way (Joint, Twisting) or the other (Opposing); and the reveal's share rises across the Blend Span, centred on the middle of the transition. Every expected pixel is `Fixtures/warp_blend/expected_warp_blend.json`, written by the tool before this code existed and printed in document 25 as FX-WARPB-001 to 039. Tolerance 2e-5.

## FX-WARPB-001 to 039 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-WARPB-001 frame 0: As added: completion 0, so nothing changes. | largest difference 1.5e-7 | yes |
| FX-WARPB-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-002 frame 0: Completion 100 with nothing to reveal: every pixel clear. | largest difference 0.0e0 | yes |
| FX-WARPB-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-003 frame 0: Completion 100 revealing the paint: the paint exactly. | largest difference 1.5e-7 | yes |
| FX-WARPB-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-004 frame 0: Completion 25, brightness, smoothness 2, amount 8, joint, span 50: the blend has not begun, so this is the holder alone, pulled a quarter of the way. | largest difference 4.0e-7 | yes |
| FX-WARPB-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-005 frame 0: Completion 50, otherwise FX-WARPB-004: half each, both pulled half the way. | largest difference 3.1e-7 | yes |
| FX-WARPB-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-006 frame 0: Completion 75, otherwise FX-WARPB-004: the blend done, so this is the paint alone, a quarter of the pull left. | largest difference 3.0e-7 | yes |
| FX-WARPB-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-007 frame 0: FX-WARPB-005 opposing: the paint pulled the other way. | largest difference 5.6e-7 | yes |
| FX-WARPB-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-008 frame 0: FX-WARPB-005 twisting: both pulled a quarter turn round. | largest difference 2.3e-7 | yes |
| FX-WARPB-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-009 frame 0: FX-WARPB-005 with amount -8: pulled the other way. | largest difference 3.1e-7 | yes |
| FX-WARPB-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-010 frame 0: FX-WARPB-005 with brightness differences. | largest difference 1.9e-7 | yes |
| FX-WARPB-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-011 frame 0: FX-WARPB-005 with contrast differences. | largest difference 2.0e-7 | yes |
| FX-WARPB-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-012 frame 0: FX-WARPB-005 with local differences. | largest difference 3.9e-7 | yes |
| FX-WARPB-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-013 frame 0: Smoothness 0, amount 4, completion 50: the reactor's own steps pull. | largest difference 1.9e-7 | yes |
| FX-WARPB-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-014 frame 0: Smoothness 6, amount 20, completion 50: a broad, soft pull. | largest difference 2.7e-6 | yes |
| FX-WARPB-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-015 frame 0: Blend span 0, completion 50: the paint alone, half the pull left. | largest difference 5.1e-7 | yes |
| FX-WARPB-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-016 frame 0: Blend span 100, completion 30: three tenths the paint. | largest difference 3.2e-7 | yes |
| FX-WARPB-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-017 frame 0: Completion 50 with nothing to reveal: the pulled holder at half its covering. | largest difference 1.5e-7 | yes |
| FX-WARPB-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-018 frame 0: A reveal layer that is not in the composition, `gone`: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-WARPB-018 frame 4: A reveal layer that is not in the composition, `gone`: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-WARPB-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-WARPB-019 frame 0: The paint with its in point at frame 4 as the reveal: before it, FX-WARPB-017; at frame 4 FX-WARPB-005. | largest difference 1.5e-7 | yes |
| FX-WARPB-019 frame 4: The paint with its in point at frame 4 as the reveal: before it, FX-WARPB-017; at frame 4 FX-WARPB-005. | largest difference 3.1e-7 | yes |
| FX-WARPB-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-020 frame 0: The paint's middle 8 by 6 as the reveal, centred: it lies where it was cut, clear round it. | largest difference 2.8e-7 | yes |
| FX-WARPB-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-021 frame 0: The holder revealing itself, completion 50: the layer warped on its own, pulled half one way and half the other. | largest difference 4.1e-7 | yes |
| FX-WARPB-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-022 frame 0: FX-WARPB-005 on the holder moved 2 right and 1 down: the same picture moved, since the reveal lies on the layer. | largest difference 3.1e-7 | yes |
| FX-WARPB-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-023 frame 0: The effect on an adjustment layer above the holder: the reveal lies on the frame, which here is the holder's own rectangle, so this is FX-WARPB-005. | largest difference 3.1e-7 | yes |
| FX-WARPB-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-024 frame 0: Completion keyed from 0 at frame 0 to 100 at frame 4: frame 0 the holder, frame 2 FX-WARPB-005, frame 4 the paint. | largest difference 1.5e-7 | yes |
| FX-WARPB-024 frame 2: Completion keyed from 0 at frame 0 to 100 at frame 4: frame 0 the holder, frame 2 FX-WARPB-005, frame 4 the paint. | largest difference 3.1e-7 | yes |
| FX-WARPB-024 frame 4: Completion keyed from 0 at frame 0 to 100 at frame 4: frame 0 the holder, frame 2 FX-WARPB-005, frame 4 the paint. | largest difference 1.5e-7 | yes |
| FX-WARPB-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-025 frame 0: Amount keyed from 0 at frame 0 to 16 at frame 4, completion 50: frame 2 is FX-WARPB-005. | largest difference 1.7e-7 | yes |
| FX-WARPB-025 frame 2: Amount keyed from 0 at frame 0 to 16 at frame 4, completion 50: frame 2 is FX-WARPB-005. | largest difference 3.1e-7 | yes |
| FX-WARPB-025 frame 4: Amount keyed from 0 at frame 0 to 16 at frame 4, completion 50: frame 2 is FX-WARPB-005. | largest difference 6.6e-7 | yes |
| FX-WARPB-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-026 frame 0: Smoothness keyed from 0 at frame 0 to 4 at frame 4, completion 50, amount 8: frame 0 smoothness 0, frame 4 softer. | largest difference 2.0e-7 | yes |
| FX-WARPB-026 frame 4: Smoothness keyed from 0 at frame 0 to 4 at frame 4, completion 50, amount 8: frame 0 smoothness 0, frame 4 softer. | largest difference 5.5e-7 | yes |
| FX-WARPB-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-027 frame 0: Blend span keyed from 0 at frame 0 to 100 at frame 4, completion 30: frame 0 the pulled holder alone, frame 4 FX-WARPB-016. | largest difference 4.1e-7 | yes |
| FX-WARPB-027 frame 4: Blend span keyed from 0 at frame 0 to 100 at frame 4, completion 30: frame 0 the pulled holder alone, frame 4 FX-WARPB-016. | largest difference 3.2e-7 | yes |
| FX-WARPB-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WARPB-028 frame 0: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-028 frame 4: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WARPB-029 frame 0: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-029 frame 4: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WARPB-030 frame 0: Smoothness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-030 frame 4: Smoothness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WARPB-031 frame 0: Smoothness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-031 frame 4: Smoothness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WARPB-032 frame 0: Warp amount 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-032 frame 4: Warp amount 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WARPB-033 frame 0: Warp amount -501, below -500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-033 frame 4: Warp amount -501, below -500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WARPB-034 frame 0: Blend span 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-034 frame 4: Blend span 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WARPB-035 frame 0: Blend span -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-035 frame 4: Blend span -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WARPB-036 frame 0: Reactor "hue", not one of the four words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-036 frame 4: Reactor "hue", not one of the four words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WARPB-037 frame 0: Warp direction "sideways", not one of the three words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-037 frame 4: Warp direction "sideways", not one of the three words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WARPB-038 frame 0: A reveal layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-038 frame 4: A reveal layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WARPB-039 frame 0: Completion keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-039 frame 4: Completion keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WARPB-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Files whose layers read each other (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| cycle_two.json: `a` and `b`, each the holder's drawing, each with a Warp Blend revealing the other: refused, `EFFECT_LAYER_CYCLE`. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_warpb_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_warpb_007.json is saved with its seven settings, the layer and the words as written | {"blend_span":50,"completion":50,"layer_to_reveal":"paint","reactor":"brightness","smoothness":2,"warp_amount":8,"warp_direction":"opposing"} | yes |
| fx_warpb_028.json is refused in a sentence naming completion | Warp Blend's completion runs from 0 to 100, and this is 101. | yes |
| fx_warpb_029.json is refused in a sentence naming completion | Warp Blend's completion runs from 0 to 100, and this is -1. | yes |
| fx_warpb_030.json is refused in a sentence naming smoothness | Warp Blend's smoothness runs from 0 to 100, and this is 101. | yes |
| fx_warpb_031.json is refused in a sentence naming smoothness | Warp Blend's smoothness runs from 0 to 100, and this is -1. | yes |
| fx_warpb_032.json is refused in a sentence naming warp amount | Warp Blend's warp amount runs from -500 to 500, and this is 501. | yes |
| fx_warpb_033.json is refused in a sentence naming warp amount | Warp Blend's warp amount runs from -500 to 500, and this is -501. | yes |
| fx_warpb_034.json is refused in a sentence naming blend span | Warp Blend's blend span runs from 0 to 100, and this is 101. | yes |
| fx_warpb_035.json is refused in a sentence naming blend span | Warp Blend's blend span runs from 0 to 100, and this is -1. | yes |
| fx_warpb_036.json is refused in a sentence naming reactor | Warp Blend's reactor is one of brightness, contrast_differences, brightness_differences, local_differences, and this is "hue". | yes |
| fx_warpb_037.json is refused in a sentence naming warp direction | Warp Blend's warp direction is one of joint, opposing, twisting, and this is "sideways". | yes |
| fx_warpb_038.json is refused in a sentence naming layer to reveal | Warp Blend's layer to reveal is the name of a layer of this composition, and this is 3. | yes |
| a file with a Warp Blend whose completion is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Warp Blend without its completion is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Warp Blend without its layer to reveal is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Warp Blend whose reactor is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| completion 101 is refused with a sentence, and nothing changes | Warp Blend's completion runs from 0 to 100, and this is 101. | yes |
| reactor "hue" is refused with a sentence, and nothing changes | Warp Blend's reactor is one of brightness, contrast_differences, brightness_differences, local_differences, and this is "hue". | yes |
| warp amount keyed to 600 is refused with a sentence, and nothing changes | Warp Blend's warp amount runs from -500 to 500, and this is 600. | yes |
| completion 70, twisting, revealing nothing is taken | taken | yes |
| completion keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |
| a half-size draft preview halves the smoothness and the warp amount and keeps the rest | WarpBlend { completion: 50.0, layer_to_reveal: String("paint"), reactor: "brightness", smoothness: 4.0, warp_amount: -20.0, warp_direction: "joint", blend_span: 50.0, map: None } | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_warpb_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_warpb_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_warpb_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_warpb_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_warpb_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_warpb_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_warpb_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_warpb_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_warpb_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_warpb_020.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_warpb_021.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_warpb_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_warpb_023.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_warpb_024.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_warpb_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_009.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_022.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_023.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_024.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_025.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_026.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_027.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_035.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_036.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_037.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_038.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_warpb_039.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Warp Blend completion 40, revealing layer 4 (as added otherwise): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Warp Blend completion 40, revealing layer 4 (as added otherwise) on three layers, frame 0, Full | largest difference 1 of 255, 223481 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 40, revealing layer 4 (as added otherwise) on three layers, frame 100, Full | largest difference 1 of 255, 225096 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 40, revealing layer 4 (as added otherwise) on three layers, frame 239, Full | largest difference 1 of 255, 222587 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 40, revealing layer 4 (as added otherwise) on three layers, frame 0, Draft | largest difference 1 of 255, 11338 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 40, revealing layer 4 (as added otherwise) on three layers, frame 100, Draft | largest difference 1 of 255, 11151 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 40, revealing layer 4 (as added otherwise) on three layers, frame 239, Draft | largest difference 1 of 255, 11108 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 60, local differences, twisting, smoothness 4, amount -30, revealing layer 4: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Warp Blend completion 60, local differences, twisting, smoothness 4, amount -30, revealing layer 4 on three layers, frame 0, Full | largest difference 1 of 255, 56038 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 60, local differences, twisting, smoothness 4, amount -30, revealing layer 4 on three layers, frame 100, Full | largest difference 1 of 255, 61272 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 60, local differences, twisting, smoothness 4, amount -30, revealing layer 4 on three layers, frame 239, Full | largest difference 1 of 255, 59578 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 60, local differences, twisting, smoothness 4, amount -30, revealing layer 4 on three layers, frame 0, Draft | largest difference 1 of 255, 11632 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 60, local differences, twisting, smoothness 4, amount -30, revealing layer 4 on three layers, frame 100, Draft | largest difference 1 of 255, 11363 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 60, local differences, twisting, smoothness 4, amount -30, revealing layer 4 on three layers, frame 239, Draft | largest difference 1 of 255, 11750 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 50, contrast differences, opposing, smoothness 0, span 0, revealing nothing: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Warp Blend completion 50, contrast differences, opposing, smoothness 0, span 0, revealing nothing on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 50, contrast differences, opposing, smoothness 0, span 0, revealing nothing on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 50, contrast differences, opposing, smoothness 0, span 0, revealing nothing on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 50, contrast differences, opposing, smoothness 0, span 0, revealing nothing on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 50, contrast differences, opposing, smoothness 0, span 0, revealing nothing on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 50, contrast differences, opposing, smoothness 0, span 0, revealing nothing on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 100, revealing layer 4: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Warp Blend completion 100, revealing layer 4 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 100, revealing layer 4 on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 100, revealing layer 4 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 100, revealing layer 4 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 100, revealing layer 4 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Warp Blend completion 100, revealing layer 4 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-518 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 3_begun.png, completion 25: the street alone, pulled out of shape along its edges, before the blend begins; draws cleanly | [], 4974 pixels changed | yes |
| 4_halfway.png, completion 50: half the street and half its opposite, both pulled the same way; draws cleanly | [], 129600 pixels differ from an even mix without the pull | yes |
| 5_opposing.png, completion 50, opposing: the two pulled opposite ways; draws cleanly | [], 13709 pixels differ from 4 | yes |
| 6_twisting.png, completion 50, twisting, amount 150: the two swirled round along the edges; draws cleanly | [], 16049 pixels differ from 4 | yes |
| 7_done.png, completion 100: the street's opposite exactly; draws cleanly | [], largest difference 0 of 255 from the reveal, 0 pixels | yes |
| a layer to reveal not in the composition: the street exactly as it was, with the warning | ["EFFECT_LAYER_MISSING An effect on layer art reads layer gone, which is not in this composition."], 0 pixels changed | yes |

## Result

248 of 248 checks pass.
