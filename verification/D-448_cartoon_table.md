# D-448: Cartoon

B-328: `core.cartoon` takes After Effects' Cartoon controls (Render, Detail Radius, Detail Threshold, Shading Steps, Shading Smoothness, Edge Threshold, Edge Width, Edge Softness, Edge Opacity, Edge Enhancement, Edge Black Level, Edge Contrast). Every expected pixel is `Fixtures/cartoon/expected_cartoon.json`, written by `tools/cartoon_reference.py` before this code existed and printed in document 25 as FX-CARTOON-001 to 032. Tolerance 2e-5.

## FX-CARTOON-001 to 032 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CARTOON-001 frame 0: The settings as they start: Fill & Edges, Detail Radius 8, Threshold 10, 8 shading steps at smoothness 40, edges at threshold 1.6, width 1.5, softness 60: the grain smoothed into the skin, the skin's shading cut into steps, and dark lines along the line, the specks, the hole and the drawing's outline. | largest difference 6.8e-8 | yes |
| FX-CARTOON-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-002 frame 0: Render Fill: the smoothed, stepped colours, no lines. | largest difference 1.6e-7 | yes |
| FX-CARTOON-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-003 frame 0: Render Edges: the lines alone, black on white, at the drawing's own covering. | largest difference 6.7e-8 | yes |
| FX-CARTOON-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-004 frame 0: Detail Radius 0: nothing smoothed, so the grain keeps its edges. | largest difference 4.6e-8 | yes |
| FX-CARTOON-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-005 frame 0: Detail Threshold 255: the smoothing hardly minds the colours, a soft blur that softens the lines. | largest difference 3.5e-7 | yes |
| FX-CARTOON-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-006 frame 0: Render Fill, 2 shading steps, smoothness 0: every pixel's shade pushed to black or white. | largest difference 1.6e-7 | yes |
| FX-CARTOON-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-007 frame 0: Render Fill, Detail Radius 0, smoothness 100: the drawing as it was. | largest difference 1.9e-7 | yes |
| FX-CARTOON-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-008 frame 0: Edge Threshold 10: only the strongest edges are drawn. | largest difference 1.3e-7 | yes |
| FX-CARTOON-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-009 frame 0: Edge Width 0, softness 0: lines one pixel wide, on the edges alone. | largest difference 1.6e-7 | yes |
| FX-CARTOON-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-010 frame 0: Edge Width 4: thick lines. | largest difference 3.0e-8 | yes |
| FX-CARTOON-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-011 frame 0: Edge Softness 0: the lines' sides hard. | largest difference 1.1e-7 | yes |
| FX-CARTOON-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-012 frame 0: Edge Opacity 0: no lines, the same as Render Fill. | largest difference 1.6e-7 | yes |
| FX-CARTOON-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-013 frame 0: Edge Opacity 50: the lines at half strength. | largest difference 8.7e-8 | yes |
| FX-CARTOON-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-014 frame 0: Render Fill, Edge Enhancement 100: the colours either side of an edge pushed apart. | largest difference 1.9e-7 | yes |
| FX-CARTOON-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-015 frame 0: Render Fill, Edge Enhancement -100: the colours either side of an edge drawn together. | largest difference 1.6e-7 | yes |
| FX-CARTOON-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-016 frame 0: Edge Black Level 0.5: grey lines. | largest difference 6.4e-8 | yes |
| FX-CARTOON-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-017 frame 0: Render Edges, Edge Black Level 1: white lines on black. | largest difference 2.4e-7 | yes |
| FX-CARTOON-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-018 frame 0: Edge Contrast 1: every edge past the threshold drawn fully. | largest difference 6.7e-8 | yes |
| FX-CARTOON-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-019 frame 0: Edge Contrast 0: the lines fade in slowly with the edge. | largest difference 6.7e-8 | yes |
| FX-CARTOON-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-020 frame 0: Detail Radius keyed from 0 at frame 0 to 16 at frame 4, linear, 8 at frame 2; Edge Width keyed from 0 to 10, eased past its end, held at 10 at frame 2. | largest difference 1.8e-7 | yes |
| FX-CARTOON-020 frame 2: Detail Radius keyed from 0 at frame 0 to 16 at frame 4, linear, 8 at frame 2; Edge Width keyed from 0 to 10, eased past its end, held at 10 at frame 2. | largest difference 3.0e-8 | yes |
| FX-CARTOON-020 frame 4: Detail Radius keyed from 0 at frame 0 to 16 at frame 4, linear, 8 at frame 2; Edge Width keyed from 0 to 10, eased past its end, held at 10 at frame 2. | largest difference 3.0e-8 | yes |
| FX-CARTOON-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-021 frame 0: FX-CARTOON-001 moved three pixels right. | largest difference 6.8e-8 | yes |
| FX-CARTOON-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-022 frame 0: After a Motion Tile that grows the layer, unmirrored: the edges are the grown buffer's, so the tile's empty row and column now meet the top row and the left column, and lines run along the top and the left. | largest difference 8.5e-8 | yes |
| FX-CARTOON-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-023 frame 0: Detail Radius 3, Threshold 30, 5 steps at smoothness 20, edges at threshold 2.5, width 2.5, softness 30, opacity 80, enhancement 40, black level 0.2, contrast 0.7: the controls together. | largest difference 2.5e-7 | yes |
| FX-CARTOON-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CARTOON-024 frame 0: Render "outline", not one of fill, edges or fill_and_edges. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-024 frame 4: Render "outline", not one of fill, edges or fill_and_edges. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CARTOON-025 frame 0: Detail Radius 51, above 50. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-025 frame 4: Detail Radius 51, above 50. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CARTOON-026 frame 0: Detail Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-026 frame 4: Detail Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CARTOON-027 frame 0: Shading Steps 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-027 frame 4: Shading Steps 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CARTOON-028 frame 0: Shading Steps 65, above 64. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-028 frame 4: Shading Steps 65, above 64. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CARTOON-029 frame 0: Edge Width 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-029 frame 4: Edge Width 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CARTOON-030 frame 0: Edge Enhancement -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-030 frame 4: Edge Enhancement -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CARTOON-031 frame 0: Edge Black Level 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-031 frame 4: Edge Black Level 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CARTOON-032 frame 0: Edge Contrast keyed to 2 at frame 4, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-032 frame 4: Edge Contrast keyed to 2 at frame 4, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CARTOON-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cartoon_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cartoon_023.json is saved with its twelve settings | {"detail_radius":3,"detail_threshold":30,"edge_black_level":0.2,"edge_contrast":0.7,"edge_enhancement":40,"edge_opacity":80,"edge_softness":30,"edge_threshold":2.5,"edge_width":2.5,"render":"fill_and_edges","shading_smoothness":20,"shading_steps":5} | yes |
| fx_cartoon_024.json is refused in a sentence naming Cartoon and its render | Cartoon's render is one of fill, edges, fill_and_edges, and this is "outline". | yes |
| fx_cartoon_025.json is refused in a sentence naming Cartoon and its detail radius | Cartoon's detail radius runs from 0 to 50, and this is 51. | yes |
| fx_cartoon_026.json is refused in a sentence naming Cartoon and its detail threshold | Cartoon's detail threshold runs from 0 to 255, and this is -1. | yes |
| fx_cartoon_027.json is refused in a sentence naming Cartoon and its shading steps | Cartoon's shading steps runs from 2 to 64, and this is 1. | yes |
| fx_cartoon_028.json is refused in a sentence naming Cartoon and its shading steps | Cartoon's shading steps runs from 2 to 64, and this is 65. | yes |
| fx_cartoon_029.json is refused in a sentence naming Cartoon and its edge width | Cartoon's edge width runs from 0 to 10, and this is 11. | yes |
| fx_cartoon_030.json is refused in a sentence naming Cartoon and its edge enhancement | Cartoon's edge enhancement runs from -100 to 100, and this is -101. | yes |
| fx_cartoon_031.json is refused in a sentence naming Cartoon and its edge black level | Cartoon's edge black level runs from 0 to 1, and this is 1.5. | yes |
| a file with a Cartoon whose detail radius is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Cartoon without its edge width is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Cartoon whose render is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| shading steps 1 is refused with a sentence, and nothing changes | Cartoon's shading steps runs from 2 to 64, and this is 1. | yes |
| render Fill (a capital) is refused with a sentence, and nothing changes | Cartoon's render is one of fill, edges, fill_and_edges, and this is "Fill". | yes |
| edge opacity keyed to 150 is refused with a sentence, and nothing changes | Cartoon's edge opacity runs from 0 to 100, and this is 150. | yes |
| Edges, edge width 4 is taken | taken | yes |
| detail radius keyed from 0 to 20 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cartoon_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cartoon_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cartoon_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cartoon_020.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cartoon_022.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cartoon_023.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cartoon_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_024.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_025.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_026.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_027.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_028.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_029.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_030.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_031.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cartoon_032.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Cartoon as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2053121 pixels changed | yes |
| the reference shot, Cartoon as added on three layers, frame 0, Full | largest difference 1 of 255, 1034 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon as added on three layers, frame 100, Full | largest difference 1 of 255, 1212 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon as added on three layers, frame 239, Full | largest difference 1 of 255, 1086 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon as added on three layers, frame 0, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon as added on three layers, frame 100, Draft | largest difference 1 of 255, 79 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon as added on three layers, frame 239, Draft | largest difference 1 of 255, 83 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon Edges, width 4, softness 0, enhancement 60, black level 0.3: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073485 pixels changed | yes |
| the reference shot, Cartoon Edges, width 4, softness 0, enhancement 60, black level 0.3 on three layers, frame 0, Full | largest difference 1 of 255, 845 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon Edges, width 4, softness 0, enhancement 60, black level 0.3 on three layers, frame 100, Full | largest difference 1 of 255, 1215 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon Edges, width 4, softness 0, enhancement 60, black level 0.3 on three layers, frame 239, Full | largest difference 1 of 255, 1001 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon Edges, width 4, softness 0, enhancement 60, black level 0.3 on three layers, frame 0, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon Edges, width 4, softness 0, enhancement 60, black level 0.3 on three layers, frame 100, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon Edges, width 4, softness 0, enhancement 60, black level 0.3 on three layers, frame 239, Draft | largest difference 1 of 255, 34 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon Fill, radius 0, steps 3, smoothness 0: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2067143 pixels changed | yes |
| the reference shot, Cartoon Fill, radius 0, steps 3, smoothness 0 on three layers, frame 0, Full | largest difference 1 of 255, 449 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon Fill, radius 0, steps 3, smoothness 0 on three layers, frame 100, Full | largest difference 1 of 255, 715 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon Fill, radius 0, steps 3, smoothness 0 on three layers, frame 239, Full | largest difference 1 of 255, 514 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon Fill, radius 0, steps 3, smoothness 0 on three layers, frame 0, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon Fill, radius 0, steps 3, smoothness 0 on three layers, frame 100, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cartoon Fill, radius 0, steps 3, smoothness 0 on three layers, frame 239, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-448 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as added: the street in flat smoothed colours with dark lines along its edges, so more near-black pixels than the street; draws cleanly | [], pixels changed 128160, near-black (all three at most 40): street 0, cartoon 13849 | yes |
| 3_fill.png, Render Fill: the smoothed, stepped colours with no lines, fewer near-black pixels than 2_as_added.png; Edge Opacity 0 draws the same picture byte for byte; both draw cleanly | [] [], near-black: as added 13849, fill 0; opacity 0 the same: true | yes |
| 4_edges.png, Render Edges: dark lines on white paper, so most pixels near white and some near black; draws cleanly | [], near-white (all three at least 215) 101627 and near-black 11167 of 129600 | yes |
| 5_thick_edges.png, Edges, width 5, softness 0: thicker lines, more near-black pixels than 4_edges.png; draws cleanly | [], near-black: width 1.5 11167, width 5 30613 | yes |
| 6_smoother.png, Fill, detail radius 20, threshold 60: flatter areas, fewer edges between neighbours than 3_fill.png; draws cleanly | [], neighbour pairs differing by more than 8 in red: radius 8 4953, radius 20 4438 | yes |
| 7_black_level_1.png, Edges, black level 1: the lines white and the paper black, so most pixels near black; draws cleanly | [], near-black 101627 of 129600 | yes |

## Result

191 of 191 checks pass.
