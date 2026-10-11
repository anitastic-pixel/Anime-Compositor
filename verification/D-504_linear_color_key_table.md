# D-504: Linear Color Key

B-384, after After Effects' Linear Color Key (Keying): each pixel compared with a key colour by its RGB, its hue or its chroma, the closer ones made more transparent. Adobe describes the effect and names its controls but publishes no formula and no defaults we found, so the rule and the settings when added are ours, stated in `tools/linear_color_key_reference.py`: a distance from the key from 0 to 100, less the matching tolerance, over the matching softness, is the share of the pixel kept. Key Operation's Keep Colors is not built. Every expected pixel is `Fixtures/linear_color_key/expected_linear_color_key.json`, written by the tool before this code existed and printed in document 25 as FX-LCKEY-001 to 018. Tolerance 2e-5.

## FX-LCKEY-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LCKEY-001 frame 0: The settings as they start: final, key colour pure blue, RGB, tolerance 10, softness 10. Pure blue's top two rows (within 10% of it) lose all their covering, its third row keeps a share, its darker rows and every other colour keep all of theirs. | largest difference 2.7e-7 | yes |
| FX-LCKEY-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LCKEY-002 frame 0: RGB, key colour the blue screen (30, 70, 200), tolerance 10, softness 10: the blue screen's top two rows and the half-covered one's fall, its third row keeps a quarter, its darker rows all of it. | largest difference 6.7e-7 | yes |
| FX-LCKEY-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LCKEY-003 frame 0: The same with softness 0: inside 10% gone, outside kept, nothing between. | largest difference 1.2e-7 | yes |
| FX-LCKEY-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LCKEY-004 frame 0: Hue, key colour the green screen (hue about 129 degrees), tolerance 20, softness 10: every row of the green screen and of pure green (hue 120) falls, lightness not counted; cyan (hue 180) keeps a share; the greys, which have no hue, and the far hues keep all their covering. | largest difference 1.2e-7 | yes |
| FX-LCKEY-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LCKEY-005 frame 0: Chroma, key colour the green screen, tolerance 10, softness 10: every row of the green screen falls, its darker self has the same chromaticity; pure green, far in chromaticity, keeps all its covering. | largest difference 1.2e-7 | yes |
| FX-LCKEY-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LCKEY-006 frame 0: Tolerance 100: every pixel is within it, so the whole layer is keyed out. | largest difference 0.0e0 | yes |
| FX-LCKEY-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LCKEY-007 frame 0: Tolerance 0, softness 20: only pure blue's top row, the key colour itself, is gone; its next two rows, within 20% of it, keep a share d / 20; every other pixel keeps all its covering. | largest difference 2.0e-7 | yes |
| FX-LCKEY-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LCKEY-008 frame 0: Hue with a grey key colour (#808080), tolerance 50: a grey has no hue, so nothing matches and the layer is as it was. | largest difference 1.2e-7 | yes |
| FX-LCKEY-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LCKEY-009 frame 0: View matte, FX-LCKEY-004's settings: the share kept as grey, black where the greens were keyed, white where nothing was, covering kept. | largest difference 3.0e-8 | yes |
| FX-LCKEY-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LCKEY-010 frame 0: View source, tolerance 100: the layer exactly as it is. | largest difference 1.2e-7 | yes |
| FX-LCKEY-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LCKEY-011 frame 0: Tolerance keyed from 0 at frame 0 to 40 at frame 4, linear, RGB on the blue screen. | largest difference 4.3e-7 | yes |
| FX-LCKEY-011 frame 2: Tolerance keyed from 0 at frame 0 to 40 at frame 4, linear, RGB on the blue screen. | largest difference 8.3e-7 | yes |
| FX-LCKEY-011 frame 4: Tolerance keyed from 0 at frame 0 to 40 at frame 4, linear, RGB on the blue screen. | largest difference 8.2e-7 | yes |
| FX-LCKEY-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LCKEY-012 frame 2: Softness keyed from 0 at frame 0 to 60 at frame 4, linear, hue on the green screen, tolerance 20. | largest difference 1.2e-7 | yes |
| FX-LCKEY-012 frame 4: Softness keyed from 0 at frame 0 to 60 at frame 4, linear, hue on the green screen, tolerance 20. | largest difference 1.7e-7 | yes |
| FX-LCKEY-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LCKEY-013 frame 0: FX-LCKEY-002 moved three pixels right: the same, moved. | largest difference 6.7e-7 | yes |
| FX-LCKEY-013 frame 3: FX-LCKEY-002 moved three pixels right: the same, moved. | largest difference 6.7e-7 | yes |
| FX-LCKEY-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LCKEY-014 frame 0: Matching tolerance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-LCKEY-014 frame 4: Matching tolerance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-LCKEY-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LCKEY-015 frame 0: Matching softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-LCKEY-015 frame 4: Matching softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-LCKEY-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LCKEY-016 frame 0: View "Final": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-LCKEY-016 frame 4: View "Final": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-LCKEY-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LCKEY-017 frame 0: Match colors "lab": not one of RGB, hue and chroma. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-LCKEY-017 frame 4: Match colors "lab": not one of RGB, hue and chroma. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-LCKEY-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LCKEY-018 frame 0: Key colour "blue", not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-LCKEY-018 frame 4: Key colour "blue", not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-LCKEY-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lckey_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lckey_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lckey_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lckey_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lckey_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lckey_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lckey_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lckey_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lckey_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lckey_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lckey_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lckey_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lckey_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lckey_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lckey_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lckey_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lckey_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lckey_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lckey_004.json is saved with its five settings, view final, key colour #28b43c, match colors hue and tolerance 20 | {"key_color":"#28b43c","match_colors":"hue","matching_softness":10,"matching_tolerance":20,"view":"final"} | yes |
| fx_lckey_014.json is refused in a sentence naming matching tolerance | Linear Color Key's matching tolerance runs from 0 to 100, and this is 101. | yes |
| fx_lckey_015.json is refused in a sentence naming matching softness | Linear Color Key's matching softness runs from 0 to 100, and this is -1. | yes |
| fx_lckey_016.json is refused in a sentence naming view | Linear Color Key's view is "final", "source" or "matte", and this is "Final". | yes |
| fx_lckey_017.json is refused in a sentence naming match colors | Linear Color Key's match colors is "rgb", "hue" or "chroma", and this is "lab". | yes |
| fx_lckey_018.json is refused in a sentence naming key colour | Linear Color Key's key colour is written #rrggbb, and this is "blue". | yes |
| a file with a Linear Color Key whose tolerance is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Linear Color Key without its key colour is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Linear Color Key without its softness is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| matching tolerance 101 is refused with a sentence, and nothing changes | Linear Color Key's matching tolerance runs from 0 to 100, and this is 101. | yes |
| match colors "lab" is refused with a sentence, and nothing changes | Linear Color Key's match colors is "rgb", "hue" or "chroma", and this is "lab". | yes |
| matching softness keyed to 300 is refused with a sentence, and nothing changes | Linear Color Key's matching softness runs from 0 to 100, and this is 300. | yes |
| match colors hue on the green screen's colour is taken | taken | yes |
| matching tolerance keyed from 0 to 40 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lckey_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lckey_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lckey_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lckey_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lckey_011.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lckey_013.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lckey_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lckey_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lckey_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lckey_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lckey_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lckey_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lckey_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lckey_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lckey_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lckey_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lckey_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lckey_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lckey_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lckey_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lckey_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lckey_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lckey_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_lckey_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Linear Color Key RGB, key white, tolerance 60, softness 20: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1234015 pixels changed | yes |
| the reference shot, Linear Color Key RGB, key white, tolerance 60, softness 20 on three layers, frame 0, Full | largest difference 1 of 255, 4024 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Linear Color Key RGB, key white, tolerance 60, softness 20 on three layers, frame 100, Full | largest difference 1 of 255, 1521 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Linear Color Key RGB, key white, tolerance 60, softness 20 on three layers, frame 239, Full | largest difference 1 of 255, 3459 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Linear Color Key RGB, key white, tolerance 60, softness 20 on three layers, frame 0, Draft | largest difference 1 of 255, 183 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Linear Color Key RGB, key white, tolerance 60, softness 20 on three layers, frame 100, Draft | largest difference 1 of 255, 20 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Linear Color Key RGB, key white, tolerance 60, softness 20 on three layers, frame 239, Draft | largest difference 1 of 255, 128 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Linear Color Key hue, key orange, tolerance 30, softness 20: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 551270 pixels changed | yes |
| the reference shot, Linear Color Key hue, key orange, tolerance 30, softness 20 on three layers, frame 0, Full | largest difference 1 of 255, 3369 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Linear Color Key hue, key orange, tolerance 30, softness 20 on three layers, frame 100, Full | largest difference 1 of 255, 1415 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Linear Color Key hue, key orange, tolerance 30, softness 20 on three layers, frame 239, Full | largest difference 1 of 255, 1519 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Linear Color Key hue, key orange, tolerance 30, softness 20 on three layers, frame 0, Draft | largest difference 1 of 255, 238 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Linear Color Key hue, key orange, tolerance 30, softness 20 on three layers, frame 100, Draft | largest difference 1 of 255, 171 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Linear Color Key hue, key orange, tolerance 30, softness 20 on three layers, frame 239, Draft | largest difference 1 of 255, 180 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Linear Color Key chroma, key grey, tolerance 10, softness 0, the matte: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073578 pixels changed | yes |
| the reference shot, Linear Color Key chroma, key grey, tolerance 10, softness 0, the matte on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Linear Color Key chroma, key grey, tolerance 10, softness 0, the matte on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Linear Color Key chroma, key grey, tolerance 10, softness 0, the matte on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Linear Color Key chroma, key grey, tolerance 10, softness 0, the matte on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Linear Color Key chroma, key grey, tolerance 10, softness 0, the matte on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Linear Color Key chroma, key grey, tolerance 10, softness 0, the matte on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-504 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the green-screen shot with no effect (a face whose edge is spilled green, a blue shirt); draws cleanly | [] | yes |
| as added (pure blue, RGB, tolerance 10) nothing in the shot is within 10% of pure blue, so nothing is keyed: the shot byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_rgb.png, RGB on the screen's colour, tolerance 10, softness 10: the screen see-through, the face, the spilled pixel (45% off in red) and the shirt fully covered; colour kept wherever anything is left; draws cleanly | [], screen 0, face 255, spilled 255, shirt 255, colour kept true | yes |
| 3_hue.png, hue on the screen's colour, tolerance 20, softness 10: the screen see-through, the face and the shirt whole, the spilled green edge fading (covering 201 by hand at the spilled pixel); draws cleanly | [], screen 0, face 255, spilled 201, shirt 255 | yes |
| 4_hue_matte.png, the same as Matte Only: the screen black, the face and the shirt white, all fully covered; draws cleanly | [], screen [0, 0, 0, 255], face [255, 255, 255, 255], shirt [255, 255, 255, 255] | yes |
| by hand: the screen half lit (20, 90, 30), in RGB keyed on the screen's colour, tolerance 10, softness 10, keeps covering 255 of 255; draws cleanly | [], covering 255 | yes |
| by hand: the screen half lit (20, 90, 30), in chroma keyed on the screen's colour, tolerance 10, softness 10, keeps covering 0 of 255; draws cleanly | [], covering 0 | yes |
| by hand: the screen a little bluer (40, 180, 90), in RGB keyed on the screen's colour, tolerance 10, softness 10, keeps covering 45 of 255; draws cleanly | [], covering 45 | yes |

## Result

131 of 131 checks pass.
