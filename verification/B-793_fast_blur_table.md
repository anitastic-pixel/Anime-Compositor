# B-793: Quality Fast on the big blurs inside Glow, Bloom, Deband and Halation

D-914 (wave 11, pick 9, step 4): a setting Quality, Exact (the default, as always) or Fast. Fast blurs a picture reduced to cells of 2, 4 or 8 pixels and reads it back smoothly (as Nuke's Blur quality), only where the blur is wide enough that a cell is at most an eighth of it; below sigma 8 Fast is Exact. Glow and Bloom blur the reduced picture; Deband reads a tap every cell through its edges; Halation's levels are one cell coarser. Every expected pixel of Fast is `Fixtures/*/expected_*_fast.json`, written by `tools/fast_blur_reference.py` before this code existed and printed in document 25 as FX-GLOW-034 to 037, FX-BLOOM-029 to 032, FX-DEBAND-025 to 028 and FX-HALO-019 to 022, compared sample by sample against the tolerance of 2e-5. Exact is each effect's own table, unchanged. The card has no reduced blur for Glow, Bloom and Deband, so at Fast they are drawn on the processor and the card draws what follows them; Halation's Fast is on the card.

## Glow: the fixtures (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GLOW-034 frame 0: FX-GLOW-001 at radius 21 (sigma 7) at Quality Fast: below sigma 8 Fast is the exact blur, so the same frame as at Exact. | largest difference 1.8e-7 | yes |
| FX-GLOW-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-035 frame 0: FX-GLOW-001 at radius 30 (sigma 10) at Quality Fast: the light blurred on cells 2 by 2 at sigma 4.97 and read back smoothly, close to Exact and not the same. | largest difference 1.8e-7 | yes |
| FX-GLOW-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-036 frame 0: Radius 60 (sigma 20), Screen, at Quality Fast: cells 4 by 4. | largest difference 2.1e-7 | yes |
| FX-GLOW-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-037 frame 0: Quality written "medium". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-037 frame 4: Quality written "medium". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Glow: the file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glow_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_001.json, an older file with no quality: opens as Exact and saves no quality | quality "exact"; saved {"based_on":"bright","colors":[],"intensity":1,"operation":"add","radius":4,"threshold":60,"tint":"","tolerance":0} | yes |
| fx_glow_035.json: Fast is saved as "fast" | {"based_on":"bright","colors":[],"intensity":1,"operation":"add","quality":"fast","radius":30,"threshold":60,"tint":"","tolerance":0} | yes |
| fx_glow_037.json is refused in a sentence naming it | Glow's quality is "exact", "fast", and this is "medium". | yes |

## Glow: commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Glow's quality "medium" is refused with a sentence, and nothing changes | Glow's quality is "exact", "fast", and this is "medium". | yes |
| Glow's quality Fast is taken | taken | yes |
| undo once: frame 0 is the frame it was | byte-identical | yes |

## Glow: the frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glow_035.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glow_036.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Glow: the card and the processor, within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GLOW-034, fx_glow_034.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GLOW-035, fx_glow_035.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GLOW-036, fx_glow_036.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-GLOW-037, fx_glow_037.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |

## Bloom: the fixtures (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BLOOM-029 frame 0: FX-BLOOM-010 (radius 20, the widest blur sigma 6.67) at Quality Fast: every blur below sigma 8, so the same frame as at Exact. | largest difference 2.4e-7 | yes |
| FX-BLOOM-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-030 frame 0: Radius 30 at Quality Fast: the widest blur (sigma 10) on cells 2 by 2, the three narrower exact. | largest difference 2.3e-7 | yes |
| FX-BLOOM-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-031 frame 0: Radius 100 with a star of streaks, length 60, at Quality Fast: the four blurs on cells 8, 4, 2 and 1 wide; the streaks as at Exact. | largest difference 2.3e-7 | yes |
| FX-BLOOM-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLOOM-032 frame 0: Quality written "medium". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-032 frame 4: Quality written "medium". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BLOOM-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Bloom: the file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bloom_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bloom_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bloom_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bloom_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bloom_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bloom_010.json, an older file with no quality: opens as Exact and saves no quality | quality "exact"; saved {"angle":0,"intensity":1,"length":60,"radius":20,"streaks":"none","threshold":80} | yes |
| fx_bloom_030.json: Fast is saved as "fast" | {"angle":0,"intensity":1,"length":6,"quality":"fast","radius":30,"streaks":"none","threshold":80} | yes |
| fx_bloom_032.json is refused in a sentence naming it | Bloom's quality is "exact" or "fast", and this is "medium". | yes |

## Bloom: commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Bloom's quality "medium" is refused with a sentence, and nothing changes | Bloom's quality is "exact" or "fast", and this is "medium". | yes |
| Bloom's quality Fast is taken | taken | yes |
| undo once: frame 0 is the frame it was | byte-identical | yes |

## Bloom: the frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bloom_030.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bloom_031.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Bloom: the card and the processor, within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BLOOM-029, fx_bloom_029.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-BLOOM-030, fx_bloom_030.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-BLOOM-031, fx_bloom_031.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-BLOOM-032, fx_bloom_032.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |

## Deband: the fixtures (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-DEBAND-025 frame 0: Blur 16 alone (sigma 5.33) at Quality Fast: below sigma 8, the same frame as at Exact. | largest difference 5.7e-8 | yes |
| FX-DEBAND-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-026 frame 0: Blur 30 alone (sigma 10) at Quality Fast: the edge-stopping blur reading every second pixel; the edge to the dark block kept as at Exact. | largest difference 5.7e-8 | yes |
| FX-DEBAND-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-027 frame 0: The settings as added with Blur 60 (sigma 20) at Quality Fast: every fourth pixel; frame 1 has fresh dither. | largest difference 5.7e-8 | yes |
| FX-DEBAND-027 frame 1: The settings as added with Blur 60 (sigma 20) at Quality Fast: every fourth pixel; frame 1 has fresh dither. | largest difference 5.7e-8 | yes |
| FX-DEBAND-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DEBAND-028 frame 0: Quality written "medium". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 5.7e-8 | yes |
| FX-DEBAND-028 frame 4: Quality written "medium". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 5.7e-8 | yes |
| FX-DEBAND-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Deband: the file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_deband_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_deband_001.json, an older file with no quality: opens as Exact and saves no quality | quality "exact"; saved {"blur_amount":8,"diffusion_amount":1,"diffusion_reseed_interval":1,"edge_blur_reduction":100,"edge_diffusion_reduction":100,"edge_noise_reduction":100,"edge_softness":2,"edge_threshold":4,"noise_amount":1,"noise_reseed_interval":1,"view":"normal"} | yes |
| fx_deband_026.json: Fast is saved as "fast" | {"blur_amount":30,"diffusion_amount":0,"diffusion_reseed_interval":1,"edge_blur_reduction":100,"edge_diffusion_reduction":100,"edge_noise_reduction":100,"edge_softness":2,"edge_threshold":4,"noise_amount":0,"noise_reseed_interval":1,"quality":"fast","view":"normal"} | yes |
| fx_deband_028.json is refused in a sentence naming it | Deband's quality is "exact" or "fast", and this is "medium". | yes |

## Deband: commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Deband's quality "medium" is refused with a sentence, and nothing changes | Deband's quality is "exact" or "fast", and this is "medium". | yes |
| Deband's quality Fast is taken | taken | yes |
| undo once: frame 0 is the frame it was | byte-identical | yes |

## Deband: the frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_deband_026.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_deband_027.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Deband: the card and the processor, within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-DEBAND-025, fx_deband_025.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-DEBAND-026, fx_deband_026.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-DEBAND-027, fx_deband_027.json, frames [0, 1] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-DEBAND-028, fx_deband_028.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |

## Halation: the fixtures (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-HALO-019 frame 0: Radius 6 (the widest level sigma 2) at Quality Fast: the cell of 2 sigma is 1 for every level, the same frame as at Exact. | largest difference 1.5e-7 | yes |
| FX-HALO-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALO-020 frame 0: FX-HALO-001 (as added, radius 12) at Quality Fast: the widest level (sigma 4) on cells 2 by 2 where Exact has 1, close and not the same. | largest difference 1.5e-7 | yes |
| FX-HALO-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALO-021 frame 0: FX-HALO-004 (radius 100) at Quality Fast: every level of sigma 4 and up one cell coarser. | largest difference 1.5e-7 | yes |
| FX-HALO-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HALO-022 frame 0: Quality written "medium". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HALO-022 frame 4: Quality written "medium". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-HALO-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Halation: the file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_halo_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_halo_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_halo_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_halo_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_halo_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_halo_001.json, an older file with no quality: opens as Exact and saves no quality | quality "exact"; saved {"amount":100,"color":"#ff4020","radius":12,"threshold":60,"threshold_smooth":50} | yes |
| fx_halo_020.json: Fast is saved as "fast" | {"amount":100,"color":"#ff4020","quality":"fast","radius":12,"threshold":60,"threshold_smooth":50} | yes |
| fx_halo_022.json is refused in a sentence naming it | Halation's quality is "exact" or "fast", and this is "medium". | yes |

## Halation: commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Halation's quality "medium" is refused with a sentence, and nothing changes | Halation's quality is "exact" or "fast", and this is "medium". | yes |
| Halation's quality Fast is taken | taken | yes |
| undo once: frame 0 is the frame it was | byte-identical | yes |

## Halation: the frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_halo_020.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_halo_021.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Halation: the card and the processor, within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-HALO-019, fx_halo_019.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-HALO-020, fx_halo_020.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-HALO-021, fx_halo_021.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-HALO-022, fx_halo_022.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |

## Halation at Fast on the card, the reference shot

| Check | The build's answer | Matches |
| --- | --- | --- |
| the reference shot, Halation at radius 100, Fast, on layer a: the processor's frame 100 differs from the shot without it | 1964267 pixels changed | yes |
| the reference shot, Halation at radius 100, Fast, on layer a, frame 0, Full | largest difference 1 of 255, 1642 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Halation at radius 100, Fast, on layer a, frame 100, Full | largest difference 1 of 255, 1954 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Halation at radius 100, Fast, on layer a, frame 239, Full | largest difference 1 of 255, 1790 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Halation at radius 100, Fast, on layer a, frame 0, Draft | largest difference 1 of 255, 85 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Halation at radius 100, Fast, on layer a, frame 100, Draft | largest difference 1 of 255, 112 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Halation at radius 100, Fast, on layer a, frame 239, Draft | largest difference 1 of 255, 95 pixels differ; 1 of 1 on the card; warnings CPU [] GPU [] | yes |

## Fast against Exact, the reference shot at 1920 by 1080, frame 100 at Full (an export's frame)

| Check | The build's answer | Matches |
| --- | --- | --- |
| Glow at radius 60 (sigma 20: cells 4 by 4): Exact changes the shot, and Fast against Exact (close: under a tenth of a level on average) | Exact changes 1912818 pixels; Fast differs from Exact by at most 1 of 255, in 82903 of 2,073,600 pixels, 0.0103 levels on average | yes |
| Bloom at radius 100 with a star (cells 8, 4, 2 and 1): Exact changes the shot, and Fast against Exact (close: under a tenth of a level on average) | Exact changes 1825497 pixels; Fast differs from Exact by at most 1 of 255, in 71377 of 2,073,600 pixels, 0.0089 levels on average | yes |
| Deband at blur 60 (sigma 20: a tap every 4 pixels): Exact changes the shot, and Fast against Exact (close: under a tenth of a level on average) | Exact changes 1358363 pixels; Fast differs from Exact by at most 23 of 255, in 381668 of 2,073,600 pixels, 0.0760 levels on average | yes |
| Halation at radius 100 (each level one cell coarser): Exact changes the shot, and Fast against Exact (close: under a tenth of a level on average) | Exact changes 1208563 pixels; Fast differs from Exact by at most 4 of 255, in 241388 of 2,073,600 pixels, 0.0296 levels on average | yes |
| Glow at radius 21 (sigma 7: Fast is Exact below sigma 8): Exact changes the shot, and Fast against Exact (close: under a tenth of a level on average) | Exact changes 1830156 pixels; Fast differs from Exact by at most 0 of 255, in 0 of 2,073,600 pixels, 0.0000 levels on average | yes |
| Deband at blur 16 (sigma 5.33: Fast is Exact below sigma 8): Exact changes the shot, and Fast against Exact (close: under a tenth of a level on average) | Exact changes 1198261 pixels; Fast differs from Exact by at most 0 of 255, in 0 of 2,073,600 pixels, 0.0000 levels on average | yes |

## Result

118 of 118 checks pass.
