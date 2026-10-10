# D-441: Write-on

B-321, after After Effects' Write-on: a brush mark laid every Brush Spacing seconds of the layer's time from its in point, where Brush Position then was, each kept for Stroke Length seconds (0 for ever); each mark's size and hardness taken when it was laid or now as Brush Time Properties says, its opacity as Paint Time Properties says (the colour is always the one now, since colours are not keyable here); Path Stroke's round brush at each mark, the mark covering most winning, laid on the layer, on transparent or revealing the layer. The layer never grows. Every expected pixel is `Fixtures/writeon/expected_writeon.json`, written by `tools/writeon_reference.py` before this code existed and printed in document 25 as FX-WRITEON-001 to 041. Tolerance 2e-5.

## FX-WRITEON-001 to 041 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-WRITEON-001 frame 0: The settings as added: the brush at the middle and never moved, Brush Size 6, Hardness 75, white: every mark at the same place, one white dot in the middle. | largest difference 1.9e-7 | yes |
| FX-WRITEON-001 frame 4: The settings as added: the brush at the middle and never moved, Brush Size 6, Hardness 75, white: every mark at the same place, one white dot in the middle. | largest difference 1.9e-7 | yes |
| FX-WRITEON-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-002 frame 0: The brush keyed from 10, 50 at frame 0 to 90, 50 at frame 8, Brush Size 3, Brush Spacing 0.02 seconds: a white line written on left to right along the middle, a dot at frame 0, reaching the right at frame 8. | largest difference 1.9e-7 | yes |
| FX-WRITEON-002 frame 2: The brush keyed from 10, 50 at frame 0 to 90, 50 at frame 8, Brush Size 3, Brush Spacing 0.02 seconds: a white line written on left to right along the middle, a dot at frame 0, reaching the right at frame 8. | largest difference 1.9e-7 | yes |
| FX-WRITEON-002 frame 4: The brush keyed from 10, 50 at frame 0 to 90, 50 at frame 8, Brush Size 3, Brush Spacing 0.02 seconds: a white line written on left to right along the middle, a dot at frame 0, reaching the right at frame 8. | largest difference 1.9e-7 | yes |
| FX-WRITEON-002 frame 8: The brush keyed from 10, 50 at frame 0 to 90, 50 at frame 8, Brush Size 3, Brush Spacing 0.02 seconds: a white line written on left to right along the middle, a dot at frame 0, reaching the right at frame 8. | largest difference 1.9e-7 | yes |
| FX-WRITEON-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-003 frame 4: FX-WRITEON-002 with Brush Spacing 0.25 seconds (6 frames): marks far apart, one dot at frame 4, two at frame 8. | largest difference 1.9e-7 | yes |
| FX-WRITEON-003 frame 8: FX-WRITEON-002 with Brush Spacing 0.25 seconds (6 frames): marks far apart, one dot at frame 4, two at frame 8. | largest difference 2.0e-7 | yes |
| FX-WRITEON-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-004 frame 4: FX-WRITEON-002 with Stroke Length 0.1 seconds (2.4 frames): only the last 0.1 seconds of the line, a short dash travelling right. | largest difference 1.9e-7 | yes |
| FX-WRITEON-004 frame 8: FX-WRITEON-002 with Stroke Length 0.1 seconds (2.4 frames): only the last 0.1 seconds of the line, a short dash travelling right. | largest difference 1.9e-7 | yes |
| FX-WRITEON-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-005 frame 8: FX-WRITEON-002 with Hardness 0: soft from its middle out. | largest difference 2.0e-7 | yes |
| FX-WRITEON-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-006 frame 8: FX-WRITEON-002 with Hardness 100: hard, smoothed over one pixel. | largest difference 1.9e-7 | yes |
| FX-WRITEON-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-007 frame 8: FX-WRITEON-002 with Brush Opacity 50: half covered at most. | largest difference 1.9e-7 | yes |
| FX-WRITEON-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-008 frame 8: FX-WRITEON-002 in red, #ff3020. | largest difference 1.9e-7 | yes |
| FX-WRITEON-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-009 frame 4: FX-WRITEON-002 On Transparent: the line alone, the drawing gone. | largest difference 1.5e-8 | yes |
| FX-WRITEON-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-010 frame 4: FX-WRITEON-002 Reveal Original Image: the drawing only under the line, the colour unused. | largest difference 1.3e-7 | yes |
| FX-WRITEON-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-011 frame 4: FX-WRITEON-002 with Brush Size 0: nothing is drawn; the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-WRITEON-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-012 frame 4: Brush Size 0, On Transparent: nothing at all. | largest difference 0.0e0 | yes |
| FX-WRITEON-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-013 frame 2: Brush Size keyed from 1 at frame 0 to 6 at frame 8, Brush Time Properties None: the whole line takes the size now, thin at frame 2 and thick at frame 8. | largest difference 1.9e-7 | yes |
| FX-WRITEON-013 frame 8: Brush Size keyed from 1 at frame 0 to 6 at frame 8, Brush Time Properties None: the whole line takes the size now, thin at frame 2 and thick at frame 8. | largest difference 1.9e-7 | yes |
| FX-WRITEON-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-014 frame 2: FX-WRITEON-013 with Brush Time Properties Size: each mark keeps the size it was laid with, so the line swells from thin on the left to thick on the right. | largest difference 1.9e-7 | yes |
| FX-WRITEON-014 frame 8: FX-WRITEON-013 with Brush Time Properties Size: each mark keeps the size it was laid with, so the line swells from thin on the left to thick on the right. | largest difference 1.9e-7 | yes |
| FX-WRITEON-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-015 frame 8: Hardness keyed from 0 at frame 0 to 100 at frame 8, Brush Size 5, Brush Time Properties Hardness: soft on the left, hard on the right. | largest difference 1.9e-7 | yes |
| FX-WRITEON-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-016 frame 8: FX-WRITEON-014 with Hardness keyed too, from 0 to 100, Brush Time Properties Size & Hardness: thin and soft to thick and hard. | largest difference 1.9e-7 | yes |
| FX-WRITEON-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-017 frame 8: Brush Opacity keyed from 100 at frame 0 to 20 at frame 8, Paint Time Properties None: the whole line fades together, to 20 at frame 8. | largest difference 2.0e-7 | yes |
| FX-WRITEON-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-018 frame 8: FX-WRITEON-017 with Paint Time Properties Opacity: each mark keeps its opacity, so the line fades from solid on the left to faint on the right. | largest difference 1.9e-7 | yes |
| FX-WRITEON-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-019 frame 8: FX-WRITEON-017 with Paint Time Properties Color: the colour cannot be keyed, so FX-WRITEON-017's frame. | largest difference 2.0e-7 | yes |
| FX-WRITEON-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-020 frame 8: FX-WRITEON-017 with Paint Time Properties Color & Opacity: FX-WRITEON-018's frame. | largest difference 1.9e-7 | yes |
| FX-WRITEON-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-021 frame 4: The brush keyed through three places, 10, 20 at frame 0, 50, 80 at frame 4 and 90, 20 at frame 8: a V written on. | largest difference 1.9e-7 | yes |
| FX-WRITEON-021 frame 8: The brush keyed through three places, 10, 20 at frame 0, 50, 80 at frame 4 and 90, 20 at frame 8: a V written on. | largest difference 1.9e-7 | yes |
| FX-WRITEON-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-022 frame 2: The brush held at 25, 50 until frame 4, then at 75, 50: two dots by frame 8, no line between. | largest difference 1.9e-7 | yes |
| FX-WRITEON-022 frame 8: The brush held at 25, 50 until frame 4, then at 75, 50: two dots by frame 8, no line between. | largest difference 1.9e-7 | yes |
| FX-WRITEON-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-023 frame 8: FX-WRITEON-002 with the layer moved three pixels right: the same, moved; nothing grows. | largest difference 1.9e-7 | yes |
| FX-WRITEON-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-024 frame 1: FX-WRITEON-002 on a layer that starts at frame 2: nothing before it, and the first mark laid at frame 2, where the brush then is. | largest difference 0.0e0 | yes |
| FX-WRITEON-024 frame 2: FX-WRITEON-002 on a layer that starts at frame 2: nothing before it, and the first mark laid at frame 2, where the brush then is. | largest difference 1.9e-7 | yes |
| FX-WRITEON-024 frame 8: FX-WRITEON-002 on a layer that starts at frame 2: nothing before it, and the first mark laid at frame 2, where the brush then is. | largest difference 1.9e-7 | yes |
| FX-WRITEON-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-025 frame 4: Brush Opacity eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots, Paint Time Properties Opacity: each mark's opacity held at 100 before it is used. | largest difference 1.9e-7 | yes |
| FX-WRITEON-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-026 frame 4: FX-WRITEON-003's spacing with Stroke Length 0.1 seconds: at frame 4 the only mark is older than its length, so nothing is drawn; at frame 8 the second mark, laid two frames before, is. | largest difference 1.9e-7 | yes |
| FX-WRITEON-026 frame 8: FX-WRITEON-003's spacing with Stroke Length 0.1 seconds: at frame 4 the only mark is older than its length, so nothing is drawn; at frame 8 the second mark, laid two frames before, is. | largest difference 2.0e-7 | yes |
| FX-WRITEON-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-027 frame 8: FX-WRITEON-002 on a layer stretched to 200 per cent: the keys and the marks stretch with it, so frame 8 is FX-WRITEON-002's frame 4. | largest difference 1.9e-7 | yes |
| FX-WRITEON-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRITEON-028 frame 0: Brush Position at 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-028 frame 4: Brush Position at 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WRITEON-029 frame 0: Brush Size 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-029 frame 4: Brush Size 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WRITEON-030 frame 0: Brush Size -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-030 frame 4: Brush Size -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WRITEON-031 frame 0: Brush Hardness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-031 frame 4: Brush Hardness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WRITEON-032 frame 0: Brush Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-032 frame 4: Brush Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WRITEON-033 frame 0: Stroke Length -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-033 frame 4: Stroke Length -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WRITEON-034 frame 0: Stroke Length 3601, above 3600 seconds. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-034 frame 4: Stroke Length 3601, above 3600 seconds. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WRITEON-035 frame 0: Brush Spacing 0, below 0.001 seconds. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-035 frame 4: Brush Spacing 0, below 0.001 seconds. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WRITEON-036 frame 0: Brush Spacing 11, above 10 seconds. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-036 frame 4: Brush Spacing 11, above 10 seconds. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WRITEON-037 frame 0: Paint Time Properties "size", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-037 frame 4: Paint Time Properties "size", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WRITEON-038 frame 0: Brush Time Properties "color", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-038 frame 4: Brush Time Properties "color", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WRITEON-039 frame 0: Paint Style "glow", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-039 frame 4: Paint Style "glow", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WRITEON-040 frame 0: Colour "#12345", not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-040 frame 4: Colour "#12345", not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WRITEON-041 frame 0: Brush Size keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-041 frame 4: Brush Size keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRITEON-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_writeon_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_writeon_009.json is saved with its words and numbers as written, the position's keys kept and no marks | {"brush_hardness":75,"brush_opacity":100,"brush_position":{"base":[10,50],"keyframes":[{"frame":0,"interp":"linear","value":[10,50]},{"frame":8,"interp":"linear","value":[90,50]}]},"brush_size":3,"brush_spacing":0.02,"brush_time_properties":"none","color":"#ffffff","paint_style":"on_transparent","paint_time_properties":"none","stroke_length":0} | yes |
| fx_writeon_028.json is refused in a sentence | Write-on's brush position runs from -1000 to 1000, and this is 1001. | yes |
| fx_writeon_029.json is refused in a sentence | Write-on's brush size runs from 0 to 200, and this is 201. | yes |
| fx_writeon_030.json is refused in a sentence | Write-on's brush size runs from 0 to 200, and this is -1. | yes |
| fx_writeon_031.json is refused in a sentence | Write-on's brush hardness runs from 0 to 100, and this is 101. | yes |
| fx_writeon_032.json is refused in a sentence | Write-on's brush opacity runs from 0 to 100, and this is 101. | yes |
| fx_writeon_033.json is refused in a sentence | Write-on's stroke length runs from 0 to 3600, and this is -1. | yes |
| fx_writeon_034.json is refused in a sentence | Write-on's stroke length runs from 0 to 3600, and this is 3601. | yes |
| fx_writeon_035.json is refused in a sentence | Write-on's brush spacing runs from 0.001 to 10, and this is 0. | yes |
| fx_writeon_036.json is refused in a sentence | Write-on's brush spacing runs from 0.001 to 10, and this is 11. | yes |
| fx_writeon_037.json is refused in a sentence | Write-on's paint time properties is one of none, color, opacity, color_and_opacity, and this is "size". | yes |
| fx_writeon_038.json is refused in a sentence | Write-on's brush time properties is one of none, size, hardness, size_and_hardness, and this is "color". | yes |
| fx_writeon_039.json is refused in a sentence | Write-on's paint style is one of on_original, on_transparent, reveal, and this is "glow". | yes |
| fx_writeon_040.json is refused in a sentence | Write-on's colour is written #rrggbb, and this is "#12345". | yes |
| a file with a Write-on with no `brush_spacing` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Write-on whose brush position is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Write-on whose paint style is true is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| brush position 1001, 50 is refused with a sentence, and nothing changes | Write-on's brush position runs from -1000 to 1000, and this is 1001. | yes |
| brush size 201 is refused with a sentence, and nothing changes | Write-on's brush size runs from 0 to 200, and this is 201. | yes |
| brush spacing 0 is refused with a sentence, and nothing changes | Write-on's brush spacing runs from 0.001 to 10, and this is 0. | yes |
| paint time properties "size" is refused with a sentence, and nothing changes | Write-on's paint time properties is one of none, color, opacity, color_and_opacity, and this is "size". | yes |
| brush time properties "color" is refused with a sentence, and nothing changes | Write-on's brush time properties is one of none, size, hardness, size_and_hardness, and this is "color". | yes |
| paint style "glow" is refused with a sentence, and nothing changes | Write-on's paint style is one of on_original, on_transparent, reveal, and this is "glow". | yes |
| colour "red" is refused with a sentence, and nothing changes | Write-on's colour is written #rrggbb, and this is "red". | yes |
| brush size keyed to 300 is refused with a sentence, and nothing changes | Write-on's brush size runs from 0 to 200, and this is 300. | yes |
| position 25, 60, size 12, hardness 40, opacity 70, stroke length 2, spacing 0.05, blue, Opacity, Size & Hardness, reveal is taken | taken | yes |
| brush position keyed from 10, 50 to 90, 50 is taken | taken | yes |
| brush size keyed from 2 to 8 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_writeon_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_writeon_002.json frame 8 in tiles of 1 and of 64 | byte-identical | yes |
| fx_writeon_009.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_writeon_014.json frame 8 in tiles of 1 and of 64 | byte-identical | yes |
| fx_writeon_021.json frame 8 in tiles of 1 and of 64 | byte-identical | yes |
| fx_writeon_023.json frame 8 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_writeon_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_011.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_writeon_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 6 of 10 frames; the same warnings: true | yes |
| fx_writeon_025.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_writeon_026.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 6 of 10 frames; the same warnings: true | yes |
| fx_writeon_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_writeon_028.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_writeon_029.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_writeon_030.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_writeon_031.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_writeon_032.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_writeon_033.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_writeon_034.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_writeon_035.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_writeon_036.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_writeon_037.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_writeon_038.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_writeon_039.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_writeon_040.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_writeon_041.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Write-on as added (a dot in the middle): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 32 pixels changed | yes |
| the reference shot, Write-on as added (a dot in the middle) on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on as added (a dot in the middle) on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on as added (a dot in the middle) on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on as added (a dot in the middle) on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on as added (a dot in the middle) on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on as added (a dot in the middle) on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on brush keyed across, size 40, hardness 50, spacing 0.05: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 29348 pixels changed | yes |
| the reference shot, Write-on brush keyed across, size 40, hardness 50, spacing 0.05 on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on brush keyed across, size 40, hardness 50, spacing 0.05 on three layers, frame 100, Full | largest difference 1 of 255, 1386 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on brush keyed across, size 40, hardness 50, spacing 0.05 on three layers, frame 239, Full | largest difference 1 of 255, 1595 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on brush keyed across, size 40, hardness 50, spacing 0.05 on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on brush keyed across, size 40, hardness 50, spacing 0.05 on three layers, frame 100, Draft | largest difference 1 of 255, 30 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on brush keyed across, size 40, hardness 50, spacing 0.05 on three layers, frame 239, Draft | largest difference 1 of 255, 42 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on keyed across, On Transparent, stroke length 2, red, size 24: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Write-on keyed across, On Transparent, stroke length 2, red, size 24 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on keyed across, On Transparent, stroke length 2, red, size 24 on three layers, frame 100, Full | largest difference 1 of 255, 7 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on keyed across, On Transparent, stroke length 2, red, size 24 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on keyed across, On Transparent, stroke length 2, red, size 24 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on keyed across, On Transparent, stroke length 2, red, size 24 on three layers, frame 100, Draft | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on keyed across, On Transparent, stroke length 2, red, size 24 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on keyed across, Reveal, size keyed 10 to 80 kept per mark, opacity keyed 100 to 30 kept per mark: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073517 pixels changed | yes |
| the reference shot, Write-on keyed across, Reveal, size keyed 10 to 80 kept per mark, opacity keyed 100 to 30 kept per mark on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on keyed across, Reveal, size keyed 10 to 80 kept per mark, opacity keyed 100 to 30 kept per mark on three layers, frame 100, Full | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on keyed across, Reveal, size keyed 10 to 80 kept per mark, opacity keyed 100 to 30 kept per mark on three layers, frame 239, Full | largest difference 1 of 255, 116 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on keyed across, Reveal, size keyed 10 to 80 kept per mark, opacity keyed 100 to 30 kept per mark on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on keyed across, Reveal, size keyed 10 to 80 kept per mark, opacity keyed 100 to 30 kept per mark on three layers, frame 100, Draft | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Write-on keyed across, Reveal, size keyed 10 to 80 kept per mark, opacity keyed 100 to 30 kept per mark on three layers, frame 239, Draft | largest difference 1 of 255, 20 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-441 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, frame 0, as added: a white dot in the middle; draws cleanly | [], 32 pixels changed of 129600 | yes |
| 3_vee_half.png, frame 24, brush keyed through a V over two seconds, size 12, halfway: the first stroke of the V written in white; draws cleanly | [], 3045 pixels changed of 129600 | yes |
| 4_vee_whole.png, frame 47, the same at the end: the whole V written; draws cleanly | [], 5934 pixels changed of 129600 | yes |
| 5_vee_transparent_tail.png, frame 40, the V in red on transparent, stroke length 0.5 seconds: only the last half second of the line, the street gone; draws cleanly | [], 129600 pixels changed of 129600 | yes |
| 6_vee_reveal_swelling.png, frame 47, the V revealing the street, each mark keeping its size, 4 growing to 40: the street seen through a line that swells; draws cleanly | [], 121258 pixels changed of 129600 | yes |

## Result

260 of 260 checks pass.
