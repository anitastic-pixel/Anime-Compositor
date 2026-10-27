# B-791: Quality Fast on Key Cleaner

D-912 (wave 11, pick 9, second step): Key Cleaner gets D-903's setting Quality, Exact (the default, as always) or Fast, which works its guided filter on blocks of up to 4 by 4 pixels and reads it back smoothly (He and Sun's Fast Guided Filter), exactly as Refine Soft Matte's Fast does. Every expected pixel of Fast is `Fixtures/key_cleaner/expected_key_cleaner_fast.json`, written by `tools/key_cleaner_fast_reference.py` before this code existed and printed in document 25 as FX-KCLEAN-019 to 023, compared sample by sample against the tolerance of 2e-5. Exact is B-383's table, unchanged.

## FX-KCLEAN-019 to 023 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-KCLEAN-019 frame 0: FX-KCLEAN-001 (Key Cleaner as added, Additional Edge Radius 10) at Quality Fast: the fit worked on blocks 2 by 2 at radius 5 and read back smoothly, so the covering is close to FX-KCLEAN-001's and not the same. | largest difference 1.2e-7 | yes |
| FX-KCLEAN-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KCLEAN-020 frame 0: FX-KCLEAN-002 (Additional Edge Radius 4) at Quality Fast: below radius 8 Fast is the exact filter, so FX-KCLEAN-002's frame. | largest difference 1.7e-7 | yes |
| FX-KCLEAN-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KCLEAN-021 frame 0: Additional Edge Radius 12, Alpha Contrast 50, at Quality Fast: blocks 3 by 3 at radius 4, the last row of blocks one pixel tall (16 rows), then steepened. | largest difference 1.1e-7 | yes |
| FX-KCLEAN-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KCLEAN-022 frame 0: Additional Edge Radius 16, Strength 50, at Quality Fast: blocks 4 by 4 (the most) at radius 4, then halfway back to the drawing. | largest difference 1.1e-7 | yes |
| FX-KCLEAN-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KCLEAN-023 frame 0: Key Cleaner, quality written "medium". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-023 frame 4: Key Cleaner, quality written "medium". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-KCLEAN-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_kclean_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kclean_001.json, an older file with no quality: opens as Exact and saves no quality | quality "exact"; saved {"additional_edge_radius":10,"alpha_contrast":0,"strength":100} | yes |
| fx_kclean_002.json, an older file with no quality: opens as Exact and saves no quality | quality "exact"; saved {"additional_edge_radius":4,"alpha_contrast":0,"strength":100} | yes |
| fx_kclean_019.json: Fast is saved as "fast" | {"additional_edge_radius":10,"alpha_contrast":0,"quality":"fast","strength":100} | yes |
| fx_kclean_023.json is refused in a sentence naming it | Key Cleaner's quality is "exact" or "fast", and this is "medium". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Key Cleaner's quality "medium" is refused with a sentence, and nothing changes | Key Cleaner's quality is "exact" or "fast", and this is "medium". | yes |
| Key Cleaner's quality Fast is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_kclean_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_kclean_021.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_kclean_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## The card and the processor: neither quality is on the card; the frame the viewer shows is the processor's, within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-KCLEAN-019, fx_kclean_019.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-KCLEAN-020, fx_kclean_020.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-KCLEAN-021, fx_kclean_021.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-KCLEAN-022, fx_kclean_022.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-KCLEAN-023, fx_kclean_023.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Key Cleaner at radius 40, Fast, on layer a: the processor's frame 100 differs from the shot without it | 1963962 pixels changed | yes |
| the reference shot, Key Cleaner at radius 40, Fast, on layer a, frame 0, Full | largest difference 1 of 255, 76775 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Key Cleaner at radius 40, Fast, on layer a, frame 100, Full | largest difference 1 of 255, 76234 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Key Cleaner at radius 40, Fast, on layer a, frame 239, Full | largest difference 1 of 255, 75561 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Key Cleaner at radius 40, Fast, on layer a, frame 0, Draft | largest difference 1 of 255, 4755 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Key Cleaner at radius 40, Fast, on layer a, frame 100, Draft | largest difference 1 of 255, 4701 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Key Cleaner at radius 40, Fast, on layer a, frame 239, Draft | largest difference 1 of 255, 4620 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |

## Fast against Exact, the reference shot at 1920 by 1080, frame 100 at Full (an export's frame)

| Check | The build's answer | Matches |
| --- | --- | --- |
| Key Cleaner as added (radius 10: blocks 2 by 2): Exact changes the keyed shot, and Fast against Exact | Exact changes 220760 pixels; Fast differs from Exact by at most 61 of 255, in 192781 of 2,073,600 pixels, 0.1359 levels on average | yes |
| Key Cleaner at radius 40 (blocks 4 by 4): Exact changes the keyed shot, and Fast against Exact | Exact changes 369438 pixels; Fast differs from Exact by at most 97 of 255, in 320964 of 2,073,600 pixels, 0.3793 levels on average | yes |
| Key Cleaner at radius 4 (Fast is Exact below radius 8): Exact changes the keyed shot, and Fast against Exact | Exact changes 166285 pixels; Fast differs from Exact by at most 0 of 255, in 0 of 2,073,600 pixels, 0.0000 levels on average | yes |

## Result

43 of 43 checks pass.
