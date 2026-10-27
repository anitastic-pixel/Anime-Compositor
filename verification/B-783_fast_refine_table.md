# B-783: Quality Fast on Refine Hard Matte and Refine Soft Matte

D-903 (wave 11, pick 9): a setting Quality, Exact (the default, as always) or Fast, which works each guided filter on blocks of up to 4 by 4 pixels and reads it back smoothly (He and Sun's Fast Guided Filter). Every expected pixel of Fast is `Fixtures/matte_refine/expected_matte_refine_fast.json`, written by `tools/matte_refine_fast_reference.py` before this code existed and printed in document 25 as FX-MREF-036 to 041, compared sample by sample against the tolerance of 2e-5. Exact is B-232's table, unchanged.

## FX-MREF-036 to 041 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MREF-036 frame 0: FX-MREF-023 (Refine Soft Matte as added, edge radius 10) at Quality Fast: the fit worked on blocks 2 by 2 at radius 5 and read back smoothly, so the covering is close to FX-MREF-023's and not the same. | largest difference 1.3e-7 | yes |
| FX-MREF-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-037 frame 0: Refine Hard Matte, feather 8, contrast 0, at Quality Fast: blocks 2 by 2 at radius 4. | largest difference 4.5e-7 | yes |
| FX-MREF-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-038 frame 0: FX-MREF-013 (Refine Hard Matte as added, feather 2) at Quality Fast: below radius 8 Fast is the exact filter, so FX-MREF-013's frame. | largest difference 2.8e-7 | yes |
| FX-MREF-038: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-039 frame 0: Refine Soft Matte, edge radius 12, feather 13, decontaminate off, at Quality Fast: blocks 3 by 3 at radius 4 for both, the last row of blocks one pixel tall (16 rows). | largest difference 4.0e-7 | yes |
| FX-MREF-039: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-040 frame 0: Refine Soft Matte, edge radius 20, at Quality Fast: blocks 4 by 4 (the most) at radius 5. | largest difference 8.9e-8 | yes |
| FX-MREF-040: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-041 frame 0: Refine Soft Matte, quality written "medium". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-041 frame 4: Refine Soft Matte, quality written "medium". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mref_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_013.json, an older file with no quality: opens as Exact and saves no quality | quality "exact"; saved {"contrast":50,"decontaminate":"off","decontamination_amount":100,"decontamination_radius":0,"feather":2,"shift_edge":0,"view_decontamination_map":"off"} | yes |
| fx_mref_023.json, an older file with no quality: opens as Exact and saves no quality | quality "exact"; saved {"contrast":0,"decontaminate":"on","decontamination_amount":100,"decontamination_radius":0,"edge_radius":10,"feather":0,"shift_edge":0,"view_decontamination_map":"off","view_edge_region":"off"} | yes |
| fx_mref_036.json: Fast is saved as "fast" | {"contrast":0,"decontaminate":"on","decontamination_amount":100,"decontamination_radius":0,"edge_radius":10,"feather":0,"quality":"fast","shift_edge":0,"view_decontamination_map":"off","view_edge_region":"off"} | yes |
| fx_mref_041.json is refused in a sentence naming it | Refine Soft Matte's quality is "exact" or "fast", and this is "medium". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Refine Soft Matte's quality "medium" is refused with a sentence, and nothing changes | Refine Soft Matte's quality is "exact" or "fast", and this is "medium". | yes |
| Refine Soft Matte's quality Fast is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mref_036.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mref_039.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mref_040.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## The card and the processor: neither quality is on the card; the frame the viewer shows is the processor's, within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MREF-036, fx_mref_036.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MREF-037, fx_mref_037.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MREF-038, fx_mref_038.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MREF-039, fx_mref_039.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MREF-040, fx_mref_040.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MREF-041, fx_mref_041.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Refine Soft Matte at radius 40 (edge, feather, decontamination), Fast, on layer a: the processor's frame 100 differs from the shot without it | 1963965 pixels changed | yes |
| the reference shot, Refine Soft Matte at radius 40 (edge, feather, decontamination), Fast, on layer a, frame 0, Full | largest difference 1 of 255, 85872 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Refine Soft Matte at radius 40 (edge, feather, decontamination), Fast, on layer a, frame 100, Full | largest difference 1 of 255, 86342 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Refine Soft Matte at radius 40 (edge, feather, decontamination), Fast, on layer a, frame 239, Full | largest difference 1 of 255, 84264 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Refine Soft Matte at radius 40 (edge, feather, decontamination), Fast, on layer a, frame 0, Draft | largest difference 1 of 255, 5270 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Refine Soft Matte at radius 40 (edge, feather, decontamination), Fast, on layer a, frame 100, Draft | largest difference 1 of 255, 5265 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Refine Soft Matte at radius 40 (edge, feather, decontamination), Fast, on layer a, frame 239, Draft | largest difference 1 of 255, 5139 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |

## Fast against Exact, the reference shot at 1920 by 1080, frame 100 at Full (an export's frame)

| Check | The build's answer | Matches |
| --- | --- | --- |
| Refine Soft Matte as added (edge radius 10: blocks 2 by 2): Exact changes the keyed shot, and Fast against Exact | Exact changes 224649 pixels; Fast differs from Exact by at most 61 of 255, in 198172 of 2,073,600 pixels, 0.1566 levels on average | yes |
| Refine Soft Matte at radius 40 (blocks 4 by 4): Exact changes the keyed shot, and Fast against Exact | Exact changes 494023 pixels; Fast differs from Exact by at most 62 of 255, in 403548 of 2,073,600 pixels, 0.3898 levels on average | yes |
| Refine Hard Matte at feather 40 (blocks 4 by 4): Exact changes the keyed shot, and Fast against Exact | Exact changes 123040 pixels; Fast differs from Exact by at most 225 of 255, in 71827 of 2,073,600 pixels, 0.2624 levels on average | yes |
| Refine Hard Matte as added (feather 2: Fast is Exact below radius 8): Exact changes the keyed shot, and Fast against Exact | Exact changes 119785 pixels; Fast differs from Exact by at most 0 of 255, in 0 of 2,073,600 pixels, 0.0000 levels on average | yes |

## Result

48 of 48 checks pass.
