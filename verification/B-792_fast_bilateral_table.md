# B-792: Quality Fast on Bilateral Blur and Cartoon

D-913 (wave 11, pick 9): a setting Quality, Exact (the default, as always) or Fast, which works the bilateral filter on a bilateral grid (Paris and Durand 2006; Chen, Paris and Durand 2007): each pixel added into a coarse grid of place and level, the grid blurred, each pixel read back at its own place and level. Every expected pixel of Fast is `expected_bilateral_blur_fast.json` and `expected_cartoon_fast.json`, written by `tools/bilateral_grid_reference.py` before this code existed and printed in document 25 as FX-BILAT-018 to 023 and FX-CARTOON-033 and 034, compared sample by sample against the tolerance of 2e-5. Exact is B-237's and B-328's tables, unchanged.

## FX-BILAT-018 to 023 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BILAT-018 frame 0: FX-BILAT-001 (Radius 5, Threshold 20, Colorize on) at Quality Fast: the grid's cells 2.27 pixels across and 22 levels deep; the grain is smoothed and the line and specks kept, close to FX-BILAT-001 and not the same. The hole stays a hole and the half-covered column keeps its covering. | largest difference 3.0e-7 | yes |
| FX-BILAT-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-019 frame 0: FX-BILAT-004 (Colorize off) at Quality Fast: the luminance worked on the grid, every pixel that shows grey. | largest difference 1.7e-7 | yes |
| FX-BILAT-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-020 frame 0: Radius 4 at Quality Fast: cells under 2 pixels, so the exact rule, the same frame as Quality Exact. | largest difference 2.1e-7 | yes |
| FX-BILAT-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-021 frame 0: Radius 5, Threshold 1 at Quality Fast: the grid would hold more than four cells for each pixel, so the exact rule, the same frame as Quality Exact. | largest difference 1.9e-7 | yes |
| FX-BILAT-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-022 frame 0: Radius 10, Threshold 60 at Quality Fast: cells 4.5 pixels across and 66 levels deep, a wider smoothing. | largest difference 3.4e-7 | yes |
| FX-BILAT-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BILAT-023 frame 0: Bilateral Blur, quality written "quick". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-023 frame 4: Bilateral Blur, quality written "quick". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BILAT-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bilat_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bilat_001.json, an older file with no quality: opens as Exact and saves no quality | quality "exact"; saved {"colorize":"on","radius":5,"threshold":20} | yes |
| fx_bilat_018.json: Fast is saved as "fast" | {"colorize":"on","quality":"fast","radius":5,"threshold":20} | yes |
| fx_bilat_023.json is refused in a sentence naming it | Bilateral Blur's quality is "exact" or "fast", and this is "quick". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Bilateral Blur's quality "quick" is refused with a sentence, and nothing changes | Bilateral Blur's quality is "exact" or "fast", and this is "quick". | yes |
| Bilateral Blur's quality Fast is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bilat_018.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bilat_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## The card and the processor: Fast is not on the card; the frame the viewer shows is the processor's, within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BILAT-018, fx_bilat_018.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-BILAT-019, fx_bilat_019.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-BILAT-020, fx_bilat_020.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-BILAT-021, fx_bilat_021.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-BILAT-022, fx_bilat_022.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-BILAT-023, fx_bilat_023.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |

## FX-CARTOON-033 and 034 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CARTOON-033 frame 0: FX-CARTOON-001 (the settings as they start, Detail Radius 8, Threshold 10) at Quality Fast: the smoothing worked on the grid, cells 3.6 pixels across and 11 levels deep; the rest of Cartoon as it was. | largest difference 1.0e-7 | yes |
| FX-CARTOON-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-034 frame 0: Cartoon, quality written "rough". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-034 frame 4: Cartoon, quality written "rough". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| fx_cartoon_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_001.json, an older file with no quality: opens as Exact and saves no quality | quality "exact"; saved {"detail_radius":8,"detail_threshold":10,"edge_black_level":0,"edge_contrast":0.5,"edge_enhancement":0,"edge_opacity":100,"edge_softness":60,"edge_threshold":1.6,"edge_width":1.5,"render":"fill_and_edges","shading_smoothness":40,"shading_steps":8} | yes |
| fx_cartoon_034.json is refused in a sentence naming it | Cartoon's quality is "exact" or "fast", and this is "rough". | yes |
| fx_cartoon_033.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| FX-CARTOON-033, fx_cartoon_033.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CARTOON-034, fx_cartoon_034.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Bilateral Blur at radius 40, Fast, on layer a: the processor's frame 100 differs from the shot without it | 1959208 pixels changed | yes |
| the reference shot, Bilateral Blur at radius 40, Fast, on layer a, frame 0, Full | largest difference 1 of 255, 92992 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur at radius 40, Fast, on layer a, frame 100, Full | largest difference 1 of 255, 93031 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur at radius 40, Fast, on layer a, frame 239, Full | largest difference 1 of 255, 92239 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur at radius 40, Fast, on layer a, frame 0, Draft | largest difference 1 of 255, 5895 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur at radius 40, Fast, on layer a, frame 100, Draft | largest difference 1 of 255, 5835 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Bilateral Blur at radius 40, Fast, on layer a, frame 239, Draft | largest difference 1 of 255, 5678 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon as added, Fast, on layer a: the processor's frame 100 differs from the shot without it | 1964854 pixels changed | yes |
| the reference shot, Cartoon as added, Fast, on layer a, frame 0, Full | largest difference 1 of 255, 89043 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon as added, Fast, on layer a, frame 100, Full | largest difference 1 of 255, 89919 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon as added, Fast, on layer a, frame 239, Full | largest difference 1 of 255, 88258 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon as added, Fast, on layer a, frame 0, Draft | largest difference 1 of 255, 5514 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon as added, Fast, on layer a, frame 100, Draft | largest difference 1 of 255, 5489 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon as added, Fast, on layer a, frame 239, Draft | largest difference 1 of 255, 5372 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |

## Fast against Exact, the reference shot at 1920 by 1080, frame 100 at Full (an export's frame)

| Check | The build's answer | Matches |
| --- | --- | --- |
| Bilateral Blur as added (radius 5, threshold 20): Exact changes the keyed shot, and Fast against Exact | Exact changes 1963044 pixels; Fast differs from Exact by 0.2166 levels of 255 on average, at most 7; 98.97 per cent of pixels within 1 level | yes |
| Bilateral Blur at radius 40: Exact changes the keyed shot, and Fast against Exact | Exact changes 1963026 pixels; Fast differs from Exact by 0.2491 levels of 255 on average, at most 18; 97.25 per cent of pixels within 1 level | yes |
| Cartoon as added (detail radius 8, threshold 10): Exact changes the keyed shot, and Fast against Exact | Exact changes 1964374 pixels; Fast differs from Exact by 0.2420 levels of 255 on average, at most 17; 95.72 per cent of pixels within 1 level | yes |
| Cartoon heavy (detail radius 40, threshold 20, 16 steps): Exact changes the keyed shot, and Fast against Exact | Exact changes 1964766 pixels; Fast differs from Exact by 0.3335 levels of 255 on average, at most 37; 89.69 per cent of pixels within 1 level | yes |

## Result

66 of 66 checks pass.
