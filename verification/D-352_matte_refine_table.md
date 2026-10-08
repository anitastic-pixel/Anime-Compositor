# B-232: the guided filter, and Matte Choker, Refine Hard Matte and Refine Soft Matte

D-352 (EFFECTS.md P0-20): an edge-aware (guided) filter, and three effects that clean a keyed layer's edge, after After Effects' Matte Choker, Refine Hard Matte and Refine Soft Matte. Every expected pixel is `Fixtures/matte_refine/expected_matte_refine.json`, written by `tools/matte_refine_reference.py` before this code existed and printed in document 25 as FX-MREF-001 to 035. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5. Matte Choker is drawn on the card too: the card's picture against the processor's, within 1 level of 255 (ADR-006, D-100).

## FX-MREF-001 to 035 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MREF-001 frame 0: Matte Choker as added: stage 1 geometric softness 4, choke 75, gray level softness 10 per cent; stage 2 geometric softness 0, choke 0, gray level softness 100 per cent; one iteration. The rectangle is choked in from its edges and its corners rounded; the speck, the strands and the half-covered edge go; the hole at (11, 8) fills, in the orange about it. | largest difference 7.9e-8 | yes |
| FX-MREF-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-002 frame 0: Matte Choker spreading then choking: stage 1 geometric softness 3, choke -60, gray level softness 20; stage 2 geometric softness 3, choke 60, gray level softness 20. The hole fills and the strands join, then the edges come back in. | largest difference 1.1e-7 | yes |
| FX-MREF-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-003 frame 0: Matte Choker with both stages at geometric softness 0, choke 0, gray level softness 100: each pixel's disc is itself and the ramp runs 0 to 1, so the output is the drawing. | largest difference 9.8e-8 | yes |
| FX-MREF-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-004 frame 0: Matte Choker, stage 1 geometric softness 2, choke 10, gray level softness 0: a hard step, every pixel covered in full where more than 54 per cent of its disc is covered and empty elsewhere. | largest difference 9.8e-8 | yes |
| FX-MREF-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-005 frame 0: Matte Choker, stage 1 geometric softness 2, choke 0, gray level softness 100: the covering is the disc's average, a soft blur of the matte that spreads into the empty pixels round it. | largest difference 1.0e-7 | yes |
| FX-MREF-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-006 frame 0: Matte Choker, stage 1 geometric softness 1.5, choke -40, gray level softness 50, three iterations: each pass spreads the matte further, though a lone corner can thin. | largest difference 1.2e-7 | yes |
| FX-MREF-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-007 frame 0: Matte Choker, stage 1 choke keyed from -100 at frame 0 to 100 at frame 4: frame 2 is choke 0. | largest difference 1.0e-7 | yes |
| FX-MREF-007 frame 2: Matte Choker, stage 1 choke keyed from -100 at frame 0 to 100 at frame 4: frame 2 is choke 0. | largest difference 9.8e-8 | yes |
| FX-MREF-007 frame 4: Matte Choker, stage 1 choke keyed from -100 at frame 0 to 100 at frame 4: frame 2 is choke 0. | largest difference 1.0e-7 | yes |
| FX-MREF-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-008 frame 0: FX-MREF-001 moved three pixels right: worked in the drawing's own space, so the same, moved. | largest difference 7.9e-8 | yes |
| FX-MREF-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-009 frame 0: Matte Choker, choke 1 at 128, above 127. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-009 frame 4: Matte Choker, choke 1 at 128, above 127. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-009: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MREF-010 frame 0: Matte Choker, gray level softness 2 at 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-010 frame 4: Matte Choker, gray level softness 2 at 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-010: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MREF-011 frame 0: Matte Choker, iterations 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-011 frame 4: Matte Choker, iterations 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MREF-012 frame 0: Matte Choker, geometric softness 1 at -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-012 frame 4: Matte Choker, geometric softness 1 at -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MREF-013 frame 0: Refine Hard Matte as added: feather 2 pixels, contrast 50 per cent. The edge-aware filter softens the edges and contrast 50 steepens them again, so the rectangle stays nearly hard while the strands, the half-covered right edge and the speck change, and the hole at (11, 8) fills a little. | largest difference 2.8e-7 | yes |
| FX-MREF-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-014 frame 0: Refine Hard Matte, feather 4, contrast 0: a wider, softer edge. | largest difference 3.7e-7 | yes |
| FX-MREF-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-015 frame 0: FX-MREF-014 with shift edge 50: the edge moves out, more covering. | largest difference 1.3e-7 | yes |
| FX-MREF-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-016 frame 0: FX-MREF-014 with shift edge -50: the edge moves in, less covering. | largest difference 5.0e-7 | yes |
| FX-MREF-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-017 frame 0: Refine Hard Matte, feather 0, contrast 80: no filter, only the soft pixels steepened: the strands and the half-covered edge. | largest difference 1.5e-7 | yes |
| FX-MREF-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-018 frame 0: Refine Hard Matte, feather 3, contrast 0, decontaminate on, amount 100, radius 2: each part-covered pixel's colour is pulled toward the colours of the covered pixels about it and away from those of the less covered ones; the covering is FX-MREF-014's at feather 3. | largest difference 3.6e-7 | yes |
| FX-MREF-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-019 frame 0: FX-MREF-018 at amount 50: halfway, in linear light. | largest difference 3.6e-7 | yes |
| FX-MREF-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-020 frame 0: FX-MREF-018 with View Decontamination Map: the layer opaque, grey where the colour is decontaminated, brightest at half covering. | largest difference 1.2e-6 | yes |
| FX-MREF-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-021 frame 0: Refine Hard Matte, feather keyed from 0 at frame 0 to 6 at frame 4: feather is whole pixels, so frame 1 (1.5) is feather 1, frame 2 feather 3 and frame 3 (4.5) feather 4. | largest difference 9.8e-8 | yes |
| FX-MREF-021 frame 1: Refine Hard Matte, feather keyed from 0 at frame 0 to 6 at frame 4: feather is whole pixels, so frame 1 (1.5) is feather 1, frame 2 feather 3 and frame 3 (4.5) feather 4. | largest difference 2.3e-7 | yes |
| FX-MREF-021 frame 2: Refine Hard Matte, feather keyed from 0 at frame 0 to 6 at frame 4: feather is whole pixels, so frame 1 (1.5) is feather 1, frame 2 feather 3 and frame 3 (4.5) feather 4. | largest difference 3.4e-7 | yes |
| FX-MREF-021 frame 3: Refine Hard Matte, feather keyed from 0 at frame 0 to 6 at frame 4: feather is whole pixels, so frame 1 (1.5) is feather 1, frame 2 feather 3 and frame 3 (4.5) feather 4. | largest difference 1.8e-7 | yes |
| FX-MREF-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-022 frame 0: FX-MREF-013 moved three pixels right. | largest difference 2.8e-7 | yes |
| FX-MREF-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-023 frame 0: Refine Soft Matte as added: edge radius 10, decontaminate on. Within 10 pixels of the matte's edge, on so small a drawing nearly all of it, the covering is fitted to the colours: the strands' covering follows their orange and teal rows, and the rectangle's teal ring and right side lose some; the colours are decontaminated. | largest difference 1.2e-7 | yes |
| FX-MREF-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-024 frame 0: Refine Soft Matte, edge radius 4, View Edge Region: white where the edge region is, black elsewhere, the layer opaque. | largest difference 0.0e0 | yes |
| FX-MREF-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-025 frame 0: Refine Soft Matte, edge radius 4, feather 2, contrast 30, decontaminate off. | largest difference 2.4e-7 | yes |
| FX-MREF-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-026 frame 0: Refine Soft Matte, edge radius 0, feather 0, contrast 0, decontaminate off: nothing to do, the output is the drawing. | largest difference 9.8e-8 | yes |
| FX-MREF-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-027 frame 0: Refine Soft Matte, edge radius 4, shift edge 30, contrast 20, decontaminate on with radius 1, amount 70. | largest difference 2.3e-7 | yes |
| FX-MREF-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MREF-028 frame 0: Refine Hard Matte, feather 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-028 frame 4: Refine Hard Matte, feather 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MREF-029 frame 0: Refine Hard Matte, contrast -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-029 frame 4: Refine Hard Matte, contrast -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MREF-030 frame 0: Refine Hard Matte, decontaminate written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-030 frame 4: Refine Hard Matte, decontaminate written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MREF-031 frame 0: Refine Hard Matte, shift edge 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-031 frame 4: Refine Hard Matte, shift edge 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MREF-032 frame 0: Refine Hard Matte, decontamination radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-032 frame 4: Refine Hard Matte, decontamination radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MREF-033 frame 0: Refine Soft Matte, edge radius 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-033 frame 4: Refine Soft Matte, edge radius 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MREF-034 frame 0: Refine Soft Matte, view edge region written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-034 frame 4: Refine Soft Matte, view edge region written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MREF-035 frame 0: Refine Soft Matte, decontamination amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-035 frame 4: Refine Soft Matte, decontamination amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.8e-8 | yes |
| FX-MREF-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mref_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mref_013.json: Refine Hard Matte writes no edge radius and no view edge region, which are Refine Soft Matte's | {"contrast":50,"decontaminate":"off","decontamination_amount":100,"decontamination_radius":0,"feather":2,"shift_edge":0,"view_decontamination_map":"off"} | yes |
| fx_mref_023.json: Refine Soft Matte writes its edge radius and view edge region | {"contrast":0,"decontaminate":"on","decontamination_amount":100,"decontamination_radius":0,"edge_radius":10,"feather":0,"shift_edge":0,"view_decontamination_map":"off","view_edge_region":"off"} | yes |
| a file with Matte Choker with no iterations is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a choke written as a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with Refine Hard Matte's decontaminate written as a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with Refine Soft Matte with no edge radius is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| fx_mref_009.json is refused in a sentence naming it | Matte Choker's choke 1 runs from -127 to 127, and this is 128. | yes |
| fx_mref_010.json is refused in a sentence naming it | Matte Choker's gray level softness 2 runs from 0 to 100, and this is 101. | yes |
| fx_mref_011.json is refused in a sentence naming it | Matte Choker's iterations runs from 1 to 10, and this is 0. | yes |
| fx_mref_012.json is refused in a sentence naming it | Matte Choker's geometric softness 1 runs from 0 to 100, and this is -1. | yes |
| fx_mref_028.json is refused in a sentence naming it | Refine Hard Matte's feather runs from 0 to 100, and this is 101. | yes |
| fx_mref_029.json is refused in a sentence naming it | Refine Hard Matte's contrast runs from 0 to 100, and this is -1. | yes |
| fx_mref_030.json is refused in a sentence naming it | Refine Hard Matte's decontaminate is "off" or "on", and this is "yes". | yes |
| fx_mref_031.json is refused in a sentence naming it | Refine Hard Matte's shift edge runs from -100 to 100, and this is 101. | yes |
| fx_mref_032.json is refused in a sentence naming it | Refine Hard Matte's decontamination radius runs from 0 to 100, and this is -1. | yes |
| fx_mref_033.json is refused in a sentence naming it | Refine Soft Matte's edge radius runs from 0 to 100, and this is 101. | yes |
| fx_mref_034.json is refused in a sentence naming it | Refine Soft Matte's view edge region is "off" or "on", and this is "maybe". | yes |
| fx_mref_035.json is refused in a sentence naming it | Refine Soft Matte's decontamination amount runs from 0 to 100, and this is 101. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| choke 1 at 128 is refused with a sentence, and nothing changes | Matte Choker's choke 1 runs from -127 to 127, and this is 128. | yes |
| iterations 11 is refused with a sentence, and nothing changes | Matte Choker's iterations runs from 1 to 10, and this is 11. | yes |
| choke 1 at -40 and two iterations is taken | taken | yes |
| geometric softness 1 keyed 0 to 6 over frames 0 to 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |
| Refine Hard Matte's decontaminate "yes" is refused with a sentence, and nothing changes | Refine Hard Matte's decontaminate is "off" or "on", and this is "yes". | yes |
| Refine Hard Matte with decontaminate on, amount 60 is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mref_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mref_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mref_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mref_018.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mref_023.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mref_027.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Matte Choker on the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mref_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_mref_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_mref_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_mref_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_mref_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_mref_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_mref_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_mref_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_mref_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_mref_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_mref_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_mref_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Matte Choker as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 50741 pixels changed | yes |
| the reference shot, Matte Choker as added on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Matte Choker as added on three layers, frame 100, Full | largest difference 1 of 255, 1417 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Matte Choker as added on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Matte Choker as added on three layers, frame 0, Draft | largest difference 1 of 255, 145 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Matte Choker as added on three layers, frame 100, Draft | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Matte Choker as added on three layers, frame 239, Draft | largest difference 1 of 255, 67 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Matte Choker spread then choke, two iterations: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 49282 pixels changed | yes |
| the reference shot, Matte Choker spread then choke, two iterations on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Matte Choker spread then choke, two iterations on three layers, frame 100, Full | largest difference 1 of 255, 1417 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Matte Choker spread then choke, two iterations on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Matte Choker spread then choke, two iterations on three layers, frame 0, Draft | largest difference 1 of 255, 128 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Matte Choker spread then choke, two iterations on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Matte Choker spread then choke, two iterations on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Matte Choker a wide soft disc: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 83318 pixels changed | yes |
| the reference shot, Matte Choker a wide soft disc on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Matte Choker a wide soft disc on three layers, frame 100, Full | largest difference 1 of 255, 1417 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Matte Choker a wide soft disc on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Matte Choker a wide soft disc on three layers, frame 0, Draft | largest difference 1 of 255, 165 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Matte Choker a wide soft disc on three layers, frame 100, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Matte Choker a wide soft disc on three layers, frame 239, Draft | largest difference 1 of 255, 87 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| a Matte Choker stage with gray level softness 0 is a hard step, so it is left to the processor (D-122's reason) | 0 of 3 on the card | yes |

## Pictures: the town keyed off its sky, in `verification/D-352 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the rough key: a soft, sky-tinted band round every house, specks in the sky, holes in the walls | the band 28.6 levels from the houses' own colour; 165 speck pixels; 119 hole pixels; warnings [] | yes |
| matte_choker.png, Matte Choker as added: the specks gone, the holes filled, the edge taken in and its corners rounded | 0 speck pixels, 0 hole pixels; warnings [] | yes |
| refine_hard_matte.png, Refine Hard Matte, decontaminate off: the edge softened along the houses' outlines and steepened | the band 28.6 levels from the houses' own colour (before 28.6); 0 speck pixels; 54 hole pixels; warnings [] | yes |
| refine_hard_matte_decontaminated.png, Refine Hard Matte, decontaminate on: the sky's tint taken out of the edge, which is the houses' colour again | the band 15.2 levels from the houses' own colour (before 28.6); 0 speck pixels; 54 hole pixels; warnings [] | yes |
| refine_soft_matte.png, Refine Soft Matte as added: the covering within 10 pixels of an edge fitted to the colours, and decontaminated. Here every pixel is within 10 of an edge, a speck or a hole, and the holes the key left keep no colour (black), so the fit takes dark for clear and the dark windows come out part see-through: a gap of D-352, not a fault of this check | the band 27.0 levels from the houses' own colour (before 28.6); 3332 speck pixels; 28019 hole pixels; warnings [] | yes |
| refine_soft_edge_region.png, Refine Soft Matte, edge radius 4, View Edge Region: white within 4 pixels of an edge (the houses' outlines, and round each speck and hole), black elsewhere | opaque true; 37912 pixels lit, 91688 black; warnings [] | yes |
| decontamination_map.png, Refine Hard Matte, View Decontamination Map: grey where the colour is decontaminated, brightest where half covered | opaque true; 3555 pixels lit, 126045 black; warnings [] | yes |

## Result

195 of 195 checks pass.
