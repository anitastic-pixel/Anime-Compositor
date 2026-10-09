# D-374: Color Balance (HLS)

B-253, after After Effects' Color Balance (HLS): every colour turned round the wheel by the hue, and its HLS lightness and saturation moved by the same amounts everywhere, a grey keeping no saturation. Every expected pixel is `Fixtures/color_balance_hls/expected_color_balance_hls.json`, written by `tools/color_balance_hls_reference.py` before this code existed and printed in document 25 as FX-HLSBAL-001 to 014. Tolerance 2e-5.

## FX-HLSBAL-001 to 014 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-HLSBAL-001 frame 0: The settings as they start: all three 0, so the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-HLSBAL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HLSBAL-002 frame 0: Hue 120: every colour a third of the way round the wheel, red to green, green to blue, the skin and warm tones to greens; black, white and grey, having no hue, stay. | largest difference 1.9e-7 | yes |
| FX-HLSBAL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HLSBAL-003 frame 0: Hue 480, a turn and a third: the same as FX-HLSBAL-002. | largest difference 1.9e-7 | yes |
| FX-HLSBAL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HLSBAL-004 frame 0: Hue -90: every colour a quarter turn the other way. | largest difference 1.9e-7 | yes |
| FX-HLSBAL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HLSBAL-005 frame 0: Lightness 30: every colour, greys and black among them, 0.3 lighter in HLS, the light ones held at white. | largest difference 1.6e-7 | yes |
| FX-HLSBAL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HLSBAL-006 frame 0: Lightness -40: every colour 0.4 darker, the dark ones held at black. | largest difference 6.1e-7 | yes |
| FX-HLSBAL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HLSBAL-007 frame 0: Saturation -100: every colour grey at its own HLS lightness. | largest difference 1.3e-7 | yes |
| FX-HLSBAL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HLSBAL-008 frame 0: Saturation 50: the skin and warm tones much stronger, the pure colours already at full strength unchanged, the greys still grey. | largest difference 3.5e-7 | yes |
| FX-HLSBAL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HLSBAL-009 frame 0: Hue 60, lightness -20, saturation 20 together. | largest difference 3.7e-7 | yes |
| FX-HLSBAL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HLSBAL-010 frame 0: Hue keyed from 0 at frame 0 to 240 at frame 4, linear: frame 0 untouched, frame 2 FX-HLSBAL-002, frame 4 red turned blue. | largest difference 1.9e-7 | yes |
| FX-HLSBAL-010 frame 2: Hue keyed from 0 at frame 0 to 240 at frame 4, linear: frame 0 untouched, frame 2 FX-HLSBAL-002, frame 4 red turned blue. | largest difference 1.9e-7 | yes |
| FX-HLSBAL-010 frame 4: Hue keyed from 0 at frame 0 to 240 at frame 4, linear: frame 0 untouched, frame 2 FX-HLSBAL-002, frame 4 red turned blue. | largest difference 1.9e-7 | yes |
| FX-HLSBAL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HLSBAL-011 frame 0: FX-HLSBAL-009 moved three pixels right: the same, moved. | largest difference 3.7e-7 | yes |
| FX-HLSBAL-011 frame 3: FX-HLSBAL-009 moved three pixels right: the same, moved. | largest difference 3.7e-7 | yes |
| FX-HLSBAL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HLSBAL-012 frame 0: Hue 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HLSBAL-012 frame 4: Hue 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HLSBAL-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HLSBAL-013 frame 0: Lightness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HLSBAL-013 frame 4: Lightness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HLSBAL-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HLSBAL-014 frame 0: Saturation -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HLSBAL-014 frame 4: Saturation -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HLSBAL-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_hlsbal_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hlsbal_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hlsbal_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hlsbal_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hlsbal_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hlsbal_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hlsbal_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hlsbal_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hlsbal_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hlsbal_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hlsbal_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hlsbal_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hlsbal_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hlsbal_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hlsbal_009.json is saved with its hue, lightness and saturation | {"hue":60,"lightness":-20,"saturation":20} | yes |
| fx_hlsbal_012.json is refused in a sentence naming its hue | Color Balance (HLS)'s hue runs from -3600 to 3600, and this is 3601. | yes |
| fx_hlsbal_013.json is refused in a sentence naming its lightness | Color Balance (HLS)'s lightness runs from -100 to 100, and this is 101. | yes |
| fx_hlsbal_014.json is refused in a sentence naming its saturation | Color Balance (HLS)'s saturation runs from -100 to 100, and this is -101. | yes |
| a file with a Color Balance (HLS) with no `hue` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Color Balance (HLS) whose saturation is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| hue 3600.5 is refused with a sentence, and nothing changes | Color Balance (HLS)'s hue runs from -3600 to 3600, and this is 3600.5. | yes |
| lightness -100.5 is refused with a sentence, and nothing changes | Color Balance (HLS)'s lightness runs from -100 to 100, and this is -100.5. | yes |
| saturation keyed to 120 is refused with a sentence, and nothing changes | Color Balance (HLS)'s saturation runs from -100 to 100, and this is 120. | yes |
| hue 60, lightness -20, saturation 20 is taken | taken | yes |
| hue keyed from 0 to 240 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_hlsbal_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_hlsbal_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_hlsbal_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_hlsbal_011.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_hlsbal_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hlsbal_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hlsbal_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hlsbal_004.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hlsbal_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hlsbal_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hlsbal_007.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hlsbal_008.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hlsbal_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hlsbal_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hlsbal_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hlsbal_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hlsbal_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hlsbal_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Color Balance (HLS) hue 120: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1932205 pixels changed | yes |
| the reference shot, Color Balance (HLS) hue 120 on three layers, frame 0, Full | largest difference 1 of 255, 4159 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) hue 120 on three layers, frame 100, Full | largest difference 1 of 255, 1680 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) hue 120 on three layers, frame 239, Full | largest difference 1 of 255, 2462 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) hue 120 on three layers, frame 0, Draft | largest difference 1 of 255, 143 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) hue 120 on three layers, frame 100, Draft | largest difference 1 of 255, 66 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) hue 120 on three layers, frame 239, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) hue -60, lightness 20, saturation 40: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Color Balance (HLS) hue -60, lightness 20, saturation 40 on three layers, frame 0, Full | largest difference 1 of 255, 3767 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) hue -60, lightness 20, saturation 40 on three layers, frame 100, Full | largest difference 1 of 255, 3343 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) hue -60, lightness 20, saturation 40 on three layers, frame 239, Full | largest difference 1 of 255, 3939 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) hue -60, lightness 20, saturation 40 on three layers, frame 0, Draft | largest difference 1 of 255, 150 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) hue -60, lightness 20, saturation 40 on three layers, frame 100, Draft | largest difference 1 of 255, 112 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) hue -60, lightness 20, saturation 40 on three layers, frame 239, Draft | largest difference 1 of 255, 140 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) saturation -100: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1931646 pixels changed | yes |
| the reference shot, Color Balance (HLS) saturation -100 on three layers, frame 0, Full | largest difference 1 of 255, 151326 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) saturation -100 on three layers, frame 100, Full | largest difference 1 of 255, 139325 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) saturation -100 on three layers, frame 239, Full | largest difference 1 of 255, 147046 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) saturation -100 on three layers, frame 0, Draft | largest difference 1 of 255, 2547 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) saturation -100 on three layers, frame 100, Draft | largest difference 1 of 255, 2314 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) saturation -100 on three layers, frame 239, Draft | largest difference 1 of 255, 2373 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) (150, -10, 25), Levels and Color Link (layer4's median, overlay 40) in one colour run on three layers, frame 0, Full | largest difference 1 of 255, 771 pixels differ; 6 of 6 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) (150, -10, 25), Levels and Color Link (layer4's median, overlay 40) in one colour run on three layers, frame 100, Full | largest difference 1 of 255, 504 pixels differ; 6 of 6 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) (150, -10, 25), Levels and Color Link (layer4's median, overlay 40) in one colour run on three layers, frame 239, Full | largest difference 1 of 255, 626 pixels differ; 6 of 6 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) (150, -10, 25), Levels and Color Link (layer4's median, overlay 40) in one colour run on three layers, frame 0, Draft | largest difference 1 of 255, 69 pixels differ; 6 of 6 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) (150, -10, 25), Levels and Color Link (layer4's median, overlay 40) in one colour run on three layers, frame 100, Draft | largest difference 1 of 255, 51 pixels differ; 6 of 6 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Balance (HLS) (150, -10, 25), Levels and Color Link (layer4's median, overlay 40) in one colour run on three layers, frame 239, Draft | largest difference 1 of 255, 60 pixels differ; 6 of 6 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-374 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts (all three 0): nothing changes; draws cleanly | [], 0 pixels changed | yes |
| 3_hue_120.png, hue 120: every colour a third of the way round, the red-brown houses green; draws cleanly | [], a red wall [70, 180, 90] from [180, 90, 70] | yes |
| 4_saturation_-100.png, saturation -100: the street in greys; draws cleanly | [], every pixel grey: true | yes |
| 5_lightness_-30.png, lightness -30: every pixel darker or as dark; draws cleanly | [], every pixel darker or the same: true | yes |

## Result

110 of 110 checks pass.
