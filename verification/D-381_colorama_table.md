# D-381: Colorama's remaining controls

B-260, after After Effects' Colorama, on D-316's effect: the phase also from hue, lightness, saturation, value or zero; the Add Phase layer read its own way and added by wrap, clamp, average or screen; Interpolate Palette and an opacity for each colour of the ring; Modify (all, hue, lightness, saturation, red, green, blue or none), Modify Alpha and Change Empty Pixels; a matching colour by RGB, hue or chroma with its tolerance and softness; a mask layer by luminance or alpha, either way round; and Composite Over Layer. Every expected pixel is `Fixtures/colorama/expected_colorama.json`, written by `tools/colorama_reference.py` before this code existed and printed in document 25 as FX-COLORAMA-001 to 047. Tolerance 2e-5.

## FX-COLORAMA-001 to 047 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-COLORAMA-001 frame 0: The settings as the effect is added, every D-381 setting written at its start: phase from intensity round the five hues, each pixel its colour by its brightness, the empty column left empty. | largest difference 5.1e-7 | yes |
| FX-COLORAMA-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-002 frame 0: The same written as a file from before D-381, none of its settings there: the same picture. | largest difference 5.1e-7 | yes |
| FX-COLORAMA-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-003 frame 0: Phase from hue: red and the greys the first colour, the other colours round the ring by their hue. | largest difference 4.9e-7 | yes |
| FX-COLORAMA-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-004 frame 0: Phase from saturation. | largest difference 2.2e-6 | yes |
| FX-COLORAMA-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-005 frame 0: Phase from lightness. | largest difference 6.2e-7 | yes |
| FX-COLORAMA-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-006 frame 0: Phase from value, the largest channel: every full colour the first colour, darker rows further round. | largest difference 4.3e-7 | yes |
| FX-COLORAMA-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-007 frame 0: Phase from zero, shift 120, three colours: every showing pixel the second colour. | largest difference 3.0e-8 | yes |
| FX-COLORAMA-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-008 frame 0: Interpolate off, shift 10: each pixel takes one of the five colours, none between. | largest difference 3.0e-8 | yes |
| FX-COLORAMA-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-009 frame 0: Modify hue: each pixel takes the ring colour's hue and keeps its own saturation and lightness, so the greys stay grey. | largest difference 3.6e-7 | yes |
| FX-COLORAMA-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-010 frame 0: Modify lightness: the ring colour's lightness, the pixel's own hue and saturation. | largest difference 3.9e-7 | yes |
| FX-COLORAMA-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-011 frame 0: Modify saturation. | largest difference 6.0e-7 | yes |
| FX-COLORAMA-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-012 frame 0: Modify red: only red from the ring colour, green and blue the pixel's own. | largest difference 4.0e-7 | yes |
| FX-COLORAMA-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-013 frame 0: Modify none, Modify Alpha on, opacities 100, 0, 100, 30, 100: the colours kept, the covering the ring's opacity at each pixel's place. | largest difference 2.5e-7 | yes |
| FX-COLORAMA-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-014 frame 0: Modify Alpha and Change Empty Pixels on: the empty column worked too, as black, so it turns the first colour at full covering. | largest difference 5.1e-7 | yes |
| FX-COLORAMA-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-015 frame 0: Phase from alpha, two colours, opacities 100 and 0, Modify none and Modify Alpha on: a curve on the covering; the solid pixels stay solid, the half-covered column nearly clear. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-016 frame 0: Matching RGB, red, tolerance 30, softness 20: red and the colours near it change, fading out with distance; the rest stay. | largest difference 2.2e-7 | yes |
| FX-COLORAMA-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-017 frame 0: Matching hue, yellow, tolerance 10, softness 10: the yellows and the orange in part; greys never match by hue. | largest difference 6.7e-7 | yes |
| FX-COLORAMA-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-018 frame 0: Matching chroma, the skin tone, tolerance 5, softness 15. | largest difference 5.1e-7 | yes |
| FX-COLORAMA-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-019 frame 0: FX-COLORAMA-016 with Composite Over Layer off: the pixels not matched go clear, the matched ones alone. | largest difference 2.2e-7 | yes |
| FX-COLORAMA-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-020 frame 0: FX-COLORAMA-019 blended 50 with the original. | largest difference 1.1e-7 | yes |
| FX-COLORAMA-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-021 frame 0: Matching tolerance keyed from 0 at frame 0 to 100 at frame 4, linear, softness 10, matching white: more and more of the layer changes. | largest difference 2.5e-7 | yes |
| FX-COLORAMA-021 frame 2: Matching tolerance keyed from 0 at frame 0 to 100 at frame 4, linear, softness 10, matching white: more and more of the layer changes. | largest difference 5.1e-7 | yes |
| FX-COLORAMA-021 frame 4: Matching tolerance keyed from 0 at frame 0 to 100 at frame 4, linear, softness 10, matching white: more and more of the layer changes. | largest difference 5.1e-7 | yes |
| FX-COLORAMA-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-022 frame 0: Opacity 3 keyed from 100 at frame 0 to 0 at frame 4, Modify Alpha on: the pixels at the third colour fade. | largest difference 5.1e-7 | yes |
| FX-COLORAMA-022 frame 4: Opacity 3 keyed from 100 at frame 0 to 0 at frame 4, Modify Alpha on: the pixels at the third colour fade. | largest difference 6.6e-7 | yes |
| FX-COLORAMA-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-023 frame 0: FX-COLORAMA-009 moved three pixels right: the same, moved. | largest difference 3.6e-7 | yes |
| FX-COLORAMA-023 frame 3: FX-COLORAMA-009 moved three pixels right: the same, moved. | largest difference 3.6e-7 | yes |
| FX-COLORAMA-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-024 frame 0: Add Phase layer `phase`, from its red, add mode wrap: each pixel's place moves on by the layer's red under it, less where it is half covered. | largest difference 8.9e-7 | yes |
| FX-COLORAMA-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-025 frame 0: Add mode clamp: the sum held at 1, the first colour. | largest difference 8.4e-7 | yes |
| FX-COLORAMA-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-026 frame 0: Add mode average. | largest difference 4.4e-7 | yes |
| FX-COLORAMA-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-027 frame 0: Add mode screen. | largest difference 4.7e-7 | yes |
| FX-COLORAMA-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-028 frame 0: An Add Phase layer in a file from before D-381, no add phase from: the layer's intensity is added, as get phase reads it. | largest difference 9.0e-7 | yes |
| FX-COLORAMA-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-029 frame 0: Mask layer `mask`, by luminance: the left full Colorama, the middle part way, the right untouched where the mask is clear and half where it is half covered. | largest difference 4.0e-7 | yes |
| FX-COLORAMA-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-030 frame 0: Masking by inverse luminance: the other way round. | largest difference 4.0e-7 | yes |
| FX-COLORAMA-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-031 frame 0: Masking by alpha: the grey middle counts as fully there. | largest difference 5.1e-7 | yes |
| FX-COLORAMA-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-032 frame 0: Masking by inverse alpha with Composite Over Layer off: only the right of the layer shows. | largest difference 4.0e-7 | yes |
| FX-COLORAMA-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-033 frame 0: Add Phase and mask layers together, add mode screen, masking by luminance. | largest difference 4.5e-7 | yes |
| FX-COLORAMA-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-COLORAMA-035 frame 0: Get phase "Hue": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-035 frame 4: Get phase "Hue": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-COLORAMA-036 frame 0: Add phase from "brightness", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-036 frame 4: Add phase from "brightness", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-COLORAMA-037 frame 0: Add mode "multiply", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-037 frame 4: Add mode "multiply", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-COLORAMA-038 frame 0: Interpolate "yes", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-038 frame 4: Interpolate "yes", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-COLORAMA-039 frame 0: Opacity 3 at 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-039 frame 4: Opacity 3 at 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-COLORAMA-040 frame 0: Modify "rgb", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-040 frame 4: Modify "rgb", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-COLORAMA-041 frame 0: Change empty "yes", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-041 frame 4: Change empty "yes", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-COLORAMA-042 frame 0: Matching mode "lab", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-042 frame 4: Matching mode "lab", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-COLORAMA-043 frame 0: Matching colour "white", not written as #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-043 frame 4: Matching colour "white", not written as #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-043: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-COLORAMA-044 frame 0: Matching tolerance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-044 frame 4: Matching tolerance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-044: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-COLORAMA-045 frame 0: Mask layer 5, a number, not the name of a layer. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-045 frame 4: Mask layer 5, a number, not the name of a layer. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-045: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-COLORAMA-046 frame 0: Masking mode "off", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-046 frame 4: Masking mode "off", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-046: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-COLORAMA-047 frame 0: Composite over "no", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-047 frame 4: Composite over "no", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-COLORAMA-047: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-COLORAMA-034 frame 0: Mask layer `ghost`, not a layer of the composition: no mask, the whole of FX-COLORAMA-001, with EFFECT_LAYER_MISSING each frame. | largest difference 5.1e-7 | yes |
| FX-COLORAMA-034 frame 3: Mask layer `ghost`, not a layer of the composition: no mask, the whole of FX-COLORAMA-001, with EFFECT_LAYER_MISSING each frame. | largest difference 5.1e-7 | yes |
| FX-COLORAMA-034: what opening it warns of, and what frame 4 warns of (D-189: both; the file's `frame_warning` asks for frame 4 only, a PROPOSED correction to `warning`) | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |

## Old projects draw exactly as before

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_colorama_002.json, written as before D-381 (none of its settings), draws byte for byte what fx_colorama_001.json (every one at its start) draws | byte-identical | yes |
| with every D-381 setting at its start the processor works D-316's own sum, `grade::colorama`'s first branch, so a project from before D-381 draws to the bit what it drew; b197's checks of D-316 pass unchanged | tests/b197_colorama.rs: 4 of 4 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_colorama_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_colorama_002.json, from before D-381, is saved without any D-381 setting, as it was | {"blend_with_original":0,"color_1":"#ff0000","color_2":"#ccff00","color_3":"#00ff66","color_4":"#0066ff","color_5":"#cc00ff","cycle_repetitions":1,"fit":"stretch","get_phase":"intensity","layer":"","phase_shift":0,"stops":5} | yes |
| fx_colorama_033.json is saved with its two layers, add mode and masking mode, and no picture | {"add_mode":"screen","add_phase_from":"red","blend_with_original":0,"change_empty":"off","color_1":"#ff0000","color_2":"#ccff00","color_3":"#00ff66","color_4":"#0066ff","color_5":"#cc00ff","composite_over":"on","cycle_repetitions":1,"fit":"stretch","get_phase":"intensity","interpolate":"on","layer":"phase","mask_layer":"mask","masking_mode":"luminance","matching_color":"#ffffff","matching_mode":"off","matching_softness":0,"matching_tolerance":15,"modify":"all","modify_alpha":"off","opacity_1":100,"opacity_2":100,"opacity_3":100,"opacity_4":100,"opacity_5":100,"phase_shift":0,"stops":5} | yes |
| fx_colorama_035.json is refused in a sentence naming "Hue" | Colorama gets its phase from intensity, luminance, red, green, blue, alpha, hue, lightness, saturation, value or zero, and this is "Hue". | yes |
| fx_colorama_036.json is refused in a sentence naming "brightness" | Colorama's add phase from is "intensity", "luminance", "red", "green", "blue", "alpha", "hue", "lightness", "saturation", "value" or "zero", and this is "brightness". | yes |
| fx_colorama_037.json is refused in a sentence naming "multiply" | Colorama's add mode is "wrap", "clamp", "average" or "screen", and this is "multiply". | yes |
| fx_colorama_038.json is refused in a sentence naming "yes" | Colorama's interpolate is "on" or "off", and this is "yes". | yes |
| fx_colorama_039.json is refused in a sentence naming opacity 3 | Colorama's opacity 3 runs from 0 to 100, and this is 101. | yes |
| fx_colorama_040.json is refused in a sentence naming "rgb" | Colorama's modify is "all", "hue", "lightness", "saturation", "red", "green", "blue" or "none", and this is "rgb". | yes |
| fx_colorama_041.json is refused in a sentence naming "yes" | Colorama's change empty pixels is "off" or "on", and this is "yes". | yes |
| fx_colorama_042.json is refused in a sentence naming "lab" | Colorama's matching mode is "off", "rgb", "hue" or "chroma", and this is "lab". | yes |
| fx_colorama_043.json is refused in a sentence naming matching colour | Colorama's matching colour is written #rrggbb, and this is "white". | yes |
| fx_colorama_044.json is refused in a sentence naming matching tolerance | Colorama's matching tolerance runs from 0 to 100, and this is -1. | yes |
| fx_colorama_045.json is refused in a sentence naming mask layer | Colorama's mask layer is the name of a layer of this composition, and this is 5. | yes |
| fx_colorama_046.json is refused in a sentence naming "off" | Colorama's masking mode is "luminance", "inverse_luminance", "alpha" or "inverse_alpha", and this is "off". | yes |
| fx_colorama_047.json is refused in a sentence naming "no" | Colorama's composite over layer is "on" or "off", and this is "no". | yes |
| a file with a Colorama whose opacity 2 is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Colorama whose modify is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| opacity 2 at 100.5 is refused with a sentence, and nothing changes | Colorama's opacity 2 runs from 0 to 100, and this is 100.5. | yes |
| modify "Hue" written with a capital is refused with a sentence, and nothing changes | Colorama's modify is "all", "hue", "lightness", "saturation", "red", "green", "blue" or "none", and this is "Hue". | yes |
| mask layer 2, a number is refused with a sentence, and nothing changes | Colorama's mask layer is the name of a layer of this composition, and this is 2. | yes |
| matching softness keyed to 120 is refused with a sentence, and nothing changes | Colorama's matching softness runs from 0 to 100, and this is 120. | yes |
| modify hue, matching by RGB, interpolate off is taken | taken | yes |
| opacity 4 keyed from 100 to 0 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_colorama_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_colorama_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_colorama_024.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_colorama_033.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_colorama_023.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| a Colorama with an Add Phase or a mask layer stays on the processor, since it reads a pixel of another layer (B-222's rule for the Add Phase layer); the card draws the rest | FX-COLORAMA-024 to 034 below: 0 frames on the card | yes |
| fx_colorama_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_003.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_011.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_026.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_028.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_029.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_030.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_033.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_035.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_036.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_037.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_038.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_039.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_040.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_041.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_042.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_043.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_044.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_045.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_046.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_colorama_047.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Colorama as a file from before D-381 has it: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Colorama as a file from before D-381 has it on three layers, frame 0, Full | largest difference 1 of 255, 1402 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama as a file from before D-381 has it on three layers, frame 100, Full | largest difference 1 of 255, 1066 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama as a file from before D-381 has it on three layers, frame 239, Full | largest difference 1 of 255, 1096 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama as a file from before D-381 has it on three layers, frame 0, Draft | largest difference 1 of 255, 41 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama as a file from before D-381 has it on three layers, frame 100, Draft | largest difference 1 of 255, 31 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama as a file from before D-381 has it on three layers, frame 239, Draft | largest difference 1 of 255, 26 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama hue phase, modify hue, interpolate off: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1872166 pixels changed | yes |
| the reference shot, Colorama hue phase, modify hue, interpolate off on three layers, frame 0, Full | largest difference 1 of 255, 3448 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama hue phase, modify hue, interpolate off on three layers, frame 100, Full | largest difference 1 of 255, 1086 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama hue phase, modify hue, interpolate off on three layers, frame 239, Full | largest difference 1 of 255, 1443 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama hue phase, modify hue, interpolate off on three layers, frame 0, Draft | largest difference 1 of 255, 172 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama hue phase, modify hue, interpolate off on three layers, frame 100, Draft | largest difference 1 of 255, 75 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama hue phase, modify hue, interpolate off on three layers, frame 239, Draft | largest difference 1 of 255, 91 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama matching chroma, tolerance 10, softness 20, composite off: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Colorama matching chroma, tolerance 10, softness 20, composite off on three layers, frame 0, Full | largest difference 1 of 255, 644 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama matching chroma, tolerance 10, softness 20, composite off on three layers, frame 100, Full | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama matching chroma, tolerance 10, softness 20, composite off on three layers, frame 239, Full | largest difference 1 of 255, 486 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama matching chroma, tolerance 10, softness 20, composite off on three layers, frame 0, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama matching chroma, tolerance 10, softness 20, composite off on three layers, frame 100, Draft | largest difference 1 of 255, 12 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama matching chroma, tolerance 10, softness 20, composite off on three layers, frame 239, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama modify lightness, modify alpha with opacities 100, 40, 100, 0, 70, blend 25: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073592 pixels changed | yes |
| the reference shot, Colorama modify lightness, modify alpha with opacities 100, 40, 100, 0, 70, blend 25 on three layers, frame 0, Full | largest difference 1 of 255, 745 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama modify lightness, modify alpha with opacities 100, 40, 100, 0, 70, blend 25 on three layers, frame 100, Full | largest difference 1 of 255, 884 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama modify lightness, modify alpha with opacities 100, 40, 100, 0, 70, blend 25 on three layers, frame 239, Full | largest difference 1 of 255, 820 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama modify lightness, modify alpha with opacities 100, 40, 100, 0, 70, blend 25 on three layers, frame 0, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama modify lightness, modify alpha with opacities 100, 40, 100, 0, 70, blend 25 on three layers, frame 100, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama modify lightness, modify alpha with opacities 100, 40, 100, 0, 70, blend 25 on three layers, frame 239, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama value phase, modify saturation, change empty pixels: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2003200 pixels changed | yes |
| the reference shot, Colorama value phase, modify saturation, change empty pixels on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama value phase, modify saturation, change empty pixels on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama value phase, modify saturation, change empty pixels on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama value phase, modify saturation, change empty pixels on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama value phase, modify saturation, change empty pixels on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama value phase, modify saturation, change empty pixels on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama masked by layer4's luminance (stays on the processor): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 160801 pixels changed | yes |
| the reference shot, Colorama masked by layer4's luminance (stays on the processor) on three layers, frame 0, Full | largest difference 1 of 255, 3099 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama masked by layer4's luminance (stays on the processor) on three layers, frame 100, Full | largest difference 1 of 255, 2950 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama masked by layer4's luminance (stays on the processor) on three layers, frame 239, Full | largest difference 1 of 255, 3278 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama masked by layer4's luminance (stays on the processor) on three layers, frame 0, Draft | largest difference 1 of 255, 17006 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama masked by layer4's luminance (stays on the processor) on three layers, frame 100, Draft | largest difference 1 of 255, 17026 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Colorama masked by layer4's luminance (stays on the processor) on three layers, frame 239, Draft | largest difference 1 of 255, 17220 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-381 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: every brightness its own colour of the rainbow ring, as D-316 drew it; draws cleanly | [], 129600 pixels changed | yes |
| 3_modify_hue.png, Modify hue: the street keeps its light and shade and takes the ring's hues; the white road markings and grey road stay as they were; draws cleanly | [], every grey pixel unchanged: true | yes |
| 4_interpolate_off.png, Interpolate Palette off: hard bands, each pixel exactly one of the five colours; draws cleanly | [], every pixel one of the five: true | yes |
| 5_matching_blue_hue.png, matching the blue wall's hue, tolerance 8, softness 10: only the blues change, the red, yellow and green walls and the greys stay; draws cleanly | [], 49560 of 129600 pixels unchanged | yes |

## Result

285 of 285 checks pass.
