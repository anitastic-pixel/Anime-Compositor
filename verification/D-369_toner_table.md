# D-369: Toner

B-248, after CycoreFX's CC Toner: the picture coloured by its lightness along two, three or five tones from shadows to highlights. Every expected pixel is `Fixtures/toner/expected_toner.json`, written by `tools/toner_reference.py` before this code existed and printed in document 25 as FX-TONER-001 to 013. Tolerance 2e-5.

## FX-TONER-001 to 013 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TONER-001 frame 0: Tritone, the colours as they start: black, a sepia brown and white, a warm sepia print of the picture by its lightness. | largest difference 1.5e-7 | yes |
| FX-TONER-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TONER-002 frame 0: Duotone, as it starts: black to white, the picture turned to greys by its lightness. | largest difference 1.2e-7 | yes |
| FX-TONER-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TONER-003 frame 0: Pentone, as it starts: black, dark brown, sepia, cream and white, a sepia print with more steps. | largest difference 2.0e-7 | yes |
| FX-TONER-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TONER-004 frame 0: Tritone, navy, red and pale yellow: a night-to-sunset colouring. | largest difference 1.4e-7 | yes |
| FX-TONER-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TONER-005 frame 0: Duotone, deep purple to warm yellow. | largest difference 1.3e-7 | yes |
| FX-TONER-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TONER-006 frame 0: Pentone, five colours from near-black purple through violet and orange to pale yellow. | largest difference 4.7e-7 | yes |
| FX-TONER-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TONER-007 frame 0: Tritone with brights and darktones set to loud green and blue: tritone does not use them, so the same as FX-TONER-001. | largest difference 1.5e-7 | yes |
| FX-TONER-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TONER-008 frame 0: Duotone with midtones set to green: duotone does not use it, so the same as FX-TONER-002. | largest difference 1.2e-7 | yes |
| FX-TONER-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TONER-009 frame 0: FX-TONER-004 moved three pixels right: the same, moved. | largest difference 1.4e-7 | yes |
| FX-TONER-009 frame 3: FX-TONER-004 moved three pixels right: the same, moved. | largest difference 1.4e-7 | yes |
| FX-TONER-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TONER-010 frame 0: Tones "quadtone", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TONER-010 frame 4: Tones "quadtone", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TONER-010: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TONER-011 frame 0: Tones "Tritone": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TONER-011 frame 4: Tones "Tritone": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TONER-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TONER-012 frame 0: Highlights "#fff", which is not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TONER-012 frame 4: Highlights "#fff", which is not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TONER-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TONER-013 frame 0: Midtones "brown", which is not a colour code. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TONER-013 frame 4: Midtones "brown", which is not a colour code. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TONER-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_toner_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_toner_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_toner_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_toner_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_toner_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_toner_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_toner_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_toner_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_toner_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_toner_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_toner_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_toner_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_toner_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_toner_006.json is saved with its tones and all five colours | {"brights":"#ff9e00","darktones":"#3c096c","highlights":"#fff3b0","midtones":"#9d4edd","shadows":"#10002b","tones":"pentone"} | yes |
| fx_toner_010.json is refused in a sentence | Toner's tones are "duotone", "tritone" or "pentone", and this is "quadtone". | yes |
| fx_toner_011.json is refused in a sentence | Toner's tones are "duotone", "tritone" or "pentone", and this is "Tritone". | yes |
| fx_toner_012.json is refused in a sentence | Toner's highlights is written #rrggbb, and this is "#fff". | yes |
| fx_toner_013.json is refused in a sentence | Toner's midtones is written #rrggbb, and this is "brown". | yes |
| a file with a Toner with no `tones` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Toner whose shadows are a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| tones "monotone" is refused with a sentence, and nothing changes | Toner's tones are "duotone", "tritone" or "pentone", and this is "monotone". | yes |
| midtones "#8C7355" written with capitals and a missing digit, "#8C735" is refused with a sentence, and nothing changes | Toner's midtones is written #rrggbb, and this is "#8C735". | yes |
| shadows "black" is refused with a sentence, and nothing changes | Toner's shadows is written #rrggbb, and this is "black". | yes |
| pentone, the purple and orange colours, is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_toner_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_toner_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_toner_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_toner_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_toner_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_toner_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_toner_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_toner_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_toner_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_toner_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_toner_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_toner_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_toner_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_toner_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_toner_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_toner_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_toner_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Toner as it starts (sepia tritone): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Toner as it starts (sepia tritone) on three layers, frame 0, Full | largest difference 1 of 255, 1213 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner as it starts (sepia tritone) on three layers, frame 100, Full | largest difference 1 of 255, 1126 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner as it starts (sepia tritone) on three layers, frame 239, Full | largest difference 1 of 255, 1136 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner as it starts (sepia tritone) on three layers, frame 0, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner as it starts (sepia tritone) on three layers, frame 100, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner as it starts (sepia tritone) on three layers, frame 239, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner duotone, deep purple to gold: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Toner duotone, deep purple to gold on three layers, frame 0, Full | largest difference 1 of 255, 3272 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner duotone, deep purple to gold on three layers, frame 100, Full | largest difference 1 of 255, 2186 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner duotone, deep purple to gold on three layers, frame 239, Full | largest difference 1 of 255, 1631 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner duotone, deep purple to gold on three layers, frame 0, Draft | largest difference 1 of 255, 154 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner duotone, deep purple to gold on three layers, frame 100, Draft | largest difference 1 of 255, 144 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner duotone, deep purple to gold on three layers, frame 239, Draft | largest difference 1 of 255, 96 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner pentone, purple and orange: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Toner pentone, purple and orange on three layers, frame 0, Full | largest difference 1 of 255, 2571 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner pentone, purple and orange on three layers, frame 100, Full | largest difference 1 of 255, 1032 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner pentone, purple and orange on three layers, frame 239, Full | largest difference 1 of 255, 2774 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner pentone, purple and orange on three layers, frame 0, Draft | largest difference 1 of 255, 170 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner pentone, purple and orange on three layers, frame 100, Draft | largest difference 1 of 255, 57 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner pentone, purple and orange on three layers, frame 239, Draft | largest difference 1 of 255, 182 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner (pentone), Levels and Change Color (purple turned 150) in one colour run on three layers, frame 0, Full | largest difference 1 of 255, 928 pixels differ; 6 of 6 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner (pentone), Levels and Change Color (purple turned 150) in one colour run on three layers, frame 100, Full | largest difference 1 of 255, 1292 pixels differ; 6 of 6 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner (pentone), Levels and Change Color (purple turned 150) in one colour run on three layers, frame 239, Full | largest difference 1 of 255, 959 pixels differ; 6 of 6 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner (pentone), Levels and Change Color (purple turned 150) in one colour run on three layers, frame 0, Draft | largest difference 1 of 255, 72 pixels differ; 6 of 6 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner (pentone), Levels and Change Color (purple turned 150) in one colour run on three layers, frame 100, Draft | largest difference 1 of 255, 60 pixels differ; 6 of 6 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Toner (pentone), Levels and Change Color (purple turned 150) in one colour run on three layers, frame 239, Draft | largest difference 1 of 255, 65 pixels differ; 6 of 6 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-369 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_sepia_tritone.png, as it starts (black, #8c7355, white): an old brown photograph, every pixel's red at least its green and green at least its blue; draws cleanly | [], every pixel warm: true | yes |
| 3_duotone_black_white.png, duotone from black to white: the street in plain greys; draws cleanly | [], every pixel grey: true | yes |
| 4_duotone_purple_gold.png, duotone from deep purple to gold: a poster-like two-colour street; draws cleanly | [], 129600 pixels changed | yes |
| 5_pentone_purple_orange.png, pentone from near-black purple through violet and orange to pale yellow: a sunset-coloured street; draws cleanly | [], 129600 pixels changed | yes |

## Result

105 of 105 checks pass.
