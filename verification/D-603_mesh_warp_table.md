# D-603: Mesh Warp

B-482: `core.mesh_warp` takes After Effects' Mesh Warp controls (Rows, Columns, Quality, Distortion Mesh) and bends the drawing smoothly with the mesh's vertices, a Catmull-Rom surface through them, drawn through D-601's forward mesh. Every expected pixel is `Fixtures/mesh_warp/expected_mesh_warp.json`, written by `tools/mesh_warp_reference.py` before this code existed and printed in document 25 as FX-MESH-001 to 018. Tolerance 2e-5.

## FX-MESH-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MESH-001 frame 0: The settings as they start, the mesh at rest (an empty list): the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-MESH-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESH-002 frame 0: The 7 by 7 mesh written out where it rests: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-MESH-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESH-003 frame 0: A 2 by 2 mesh, its middle vertex pushed from 50 to 75 per cent across: the stripes bow right round it, smoothly, and the edges stay. | largest difference 2.5e-7 | yes |
| FX-MESH-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESH-004 frame 0: FX-MESH-003 at Quality 1: each cell left whole, so the bow is a few straight pieces and the frame differs. | largest difference 2.5e-7 | yes |
| FX-MESH-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESH-005 frame 0: A 3 by 3 mesh, the top edge's inner vertices raised and lowered and the bottom's too: the top and bottom edges wave, clear where they dip in. | largest difference 2.5e-7 | yes |
| FX-MESH-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESH-006 frame 0: A 1 by 1 mesh, its top left vertex pulled in to (20, 20): with only the corners the bend is straight-edged, the corner it left clear. | largest difference 2.5e-7 | yes |
| FX-MESH-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESH-007 frame 0: FX-MESH-003's middle vertex keyed from where it rests at frame 0 to 75 per cent at frame 4, linear: frame 0 the drawing, frame 4 FX-MESH-003, frame 2 between. | largest difference 1.9e-7 | yes |
| FX-MESH-007 frame 2: FX-MESH-003's middle vertex keyed from where it rests at frame 0 to 75 per cent at frame 4, linear: frame 0 the drawing, frame 4 FX-MESH-003, frame 2 between. | largest difference 2.5e-7 | yes |
| FX-MESH-007 frame 4: FX-MESH-003's middle vertex keyed from where it rests at frame 0 to 75 per cent at frame 4, linear: frame 0 the drawing, frame 4 FX-MESH-003, frame 2 between. | largest difference 2.5e-7 | yes |
| FX-MESH-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESH-008 frame 0: FX-MESH-003 with the layer moved three pixels right: the bent drawing moves with it. | largest difference 2.5e-7 | yes |
| FX-MESH-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESH-009 frame 0: The middle vertex dragged out to 150 per cent, past the right edge: the mesh folds; where it overlaps, the part later in the grid is on top. | largest difference 2.5e-7 | yes |
| FX-MESH-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESH-010 frame 0: Four rows by two columns, the inner vertices twisted: the stripes bend one way above the middle and the other way below. | largest difference 2.5e-7 | yes |
| FX-MESH-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESH-011 frame 0: Quality keyed from 1 at frame 0 to 10 at frame 4, eased past its end, with FX-MESH-003's push: frame 2 would pass 10 and is held there, the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-MESH-011 frame 2: Quality keyed from 1 at frame 0 to 10 at frame 4, eased past its end, with FX-MESH-003's push: frame 2 would pass 10 and is held there, the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-MESH-011 frame 4: Quality keyed from 1 at frame 0 to 10 at frame 4, eased past its end, with FX-MESH-003's push: frame 2 would pass 10 and is held there, the same as frame 4. | largest difference 2.5e-7 | yes |
| FX-MESH-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESH-012 frame 0: Rows 2.4 and Columns 1.6, which round to 2 and 2: the same as FX-MESH-003. | largest difference 2.5e-7 | yes |
| FX-MESH-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESH-013 frame 0: Rows 32, above 31. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MESH-013 frame 4: Rows 32, above 31. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MESH-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MESH-014 frame 0: Columns 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MESH-014 frame 4: Columns 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MESH-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MESH-015 frame 0: A 2 by 2 mesh written with 16 numbers, not 18. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MESH-015 frame 4: A 2 by 2 mesh written with 16 numbers, not 18. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MESH-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MESH-016 frame 0: A vertex at 501 per cent across, past 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MESH-016 frame 4: A vertex at 501 per cent across, past 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MESH-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MESH-017 frame 0: Quality 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MESH-017 frame 4: Quality 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MESH-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MESH-018 frame 0: The middle vertex keyed out to 600 per cent at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MESH-018 frame 4: The middle vertex keyed out to 600 per cent at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MESH-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mesh_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mesh_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mesh_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mesh_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mesh_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mesh_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mesh_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mesh_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mesh_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mesh_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mesh_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mesh_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mesh_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mesh_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mesh_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mesh_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mesh_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mesh_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mesh_003.json is saved with its four settings, the mesh one flat list of 18 numbers, the middle vertex's x 75 | {"columns":2,"distortion_mesh":[0,0,50,0,100,0,0,50,75,50,100,50,0,100,50,100,100,100],"quality":8,"rows":2} | yes |
| fx_mesh_001.json, the mesh at rest, is saved with an empty list | {"columns":7,"distortion_mesh":[],"quality":8,"rows":7} | yes |
| fx_mesh_013.json is refused in a sentence naming Mesh Warp and its rows | Mesh Warp's rows runs from 1 to 31, and this is 32. | yes |
| fx_mesh_014.json is refused in a sentence naming Mesh Warp and its columns | Mesh Warp's columns runs from 1 to 31, and this is 0. | yes |
| fx_mesh_015.json is refused in a sentence naming Mesh Warp and its distortion mesh is none, or 18 numbers | Mesh Warp's distortion mesh is none, or 18 numbers, x then y for each vertex, and this has 16. | yes |
| fx_mesh_016.json is refused in a sentence naming Mesh Warp and its distortion mesh | Mesh Warp's distortion mesh runs from -400 to 500, and this is 501. | yes |
| fx_mesh_017.json is refused in a sentence naming Mesh Warp and its quality | Mesh Warp's quality runs from 1 to 10, and this is 11. | yes |
| a file with a Mesh Warp whose rows are a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Mesh Warp without its distortion mesh is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Mesh Warp whose distortion mesh is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| rows 32 is refused with a sentence, and nothing changes | Mesh Warp's distortion mesh is none, or 192 numbers, x then y for each vertex, and this has 18. | yes |
| a 2 by 2 mesh of 16 numbers is refused with a sentence, and nothing changes | Mesh Warp's distortion mesh is none, or 18 numbers, x then y for each vertex, and this has 16. | yes |
| the middle vertex keyed out to 600 per cent is refused with a sentence, and nothing changes | Mesh Warp's distortion mesh runs from -400 to 500, and this is 600. | yes |
| the middle vertex at (60, 40), quality 3 is taken | taken | yes |
| the mesh keyed from rest to the middle pushed is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mesh_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mesh_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mesh_007.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mesh_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mesh_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mesh_001.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mesh_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mesh_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mesh_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mesh_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mesh_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mesh_007.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mesh_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mesh_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mesh_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mesh_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mesh_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mesh_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mesh_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mesh_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mesh_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mesh_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_mesh_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Mesh Warp 2 by 2, the middle vertex pushed to 75 per cent across: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1896685 pixels changed | yes |
| the reference shot, Mesh Warp 2 by 2, the middle vertex pushed to 75 per cent across on three layers, frame 0, Full | largest difference 1 of 255, 506 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 2 by 2, the middle vertex pushed to 75 per cent across on three layers, frame 100, Full | largest difference 1 of 255, 694 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 2 by 2, the middle vertex pushed to 75 per cent across on three layers, frame 239, Full | largest difference 1 of 255, 441 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 2 by 2, the middle vertex pushed to 75 per cent across on three layers, frame 0, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 2 by 2, the middle vertex pushed to 75 per cent across on three layers, frame 100, Draft | largest difference 1 of 255, 79 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 2 by 2, the middle vertex pushed to 75 per cent across on three layers, frame 239, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 3 by 3, the top and bottom edges waving, at quality 2: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1681691 pixels changed | yes |
| the reference shot, Mesh Warp 3 by 3, the top and bottom edges waving, at quality 2 on three layers, frame 0, Full | largest difference 1 of 255, 2628 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 3 by 3, the top and bottom edges waving, at quality 2 on three layers, frame 100, Full | largest difference 1 of 255, 960 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 3 by 3, the top and bottom edges waving, at quality 2 on three layers, frame 239, Full | largest difference 1 of 255, 1276 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 3 by 3, the top and bottom edges waving, at quality 2 on three layers, frame 0, Draft | largest difference 1 of 255, 100 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 3 by 3, the top and bottom edges waving, at quality 2 on three layers, frame 100, Draft | largest difference 1 of 255, 75 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 3 by 3, the top and bottom edges waving, at quality 2 on three layers, frame 239, Draft | largest difference 1 of 255, 71 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp folded (the middle vertex out at 150 per cent): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1977859 pixels changed | yes |
| the reference shot, Mesh Warp folded (the middle vertex out at 150 per cent) on three layers, frame 0, Full | largest difference 1 of 255, 442 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp folded (the middle vertex out at 150 per cent) on three layers, frame 100, Full | largest difference 1 of 255, 648 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp folded (the middle vertex out at 150 per cent) on three layers, frame 239, Full | largest difference 1 of 255, 469 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp folded (the middle vertex out at 150 per cent) on three layers, frame 0, Draft | largest difference 1 of 255, 37 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp folded (the middle vertex out at 150 per cent) on three layers, frame 100, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp folded (the middle vertex out at 150 per cent) on three layers, frame 239, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-603 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect, every pixel covered; draws cleanly | [], 0 clear pixels | yes |
| as added, the mesh at rest, and the 7 by 7 mesh written out where it rests: the street byte for byte; both draw cleanly | [] [], 0 and 0 pixels differ | yes |
| 2_middle_pushed.png, a 2 by 2 mesh with its middle vertex pushed to 75 per cent across: the street bowed right round the middle, the outline kept, so nothing cleared; draws cleanly | [], 32547 pixels differ from the street, 0 clear | yes |
| 3_edges_waving.png, a 3 by 3 mesh with the top edge's inner vertices at 20 and -15 per cent down: clear above the top edge where it dips at a third across, none at two thirds; draws cleanly | [], clear rows from the top at a third and two thirds across: 54, 0 | yes |
| 4_folded.png, the middle vertex dragged out to 150 per cent: the mesh folds over itself, unlike the push; draws cleanly | [], 33298 pixels differ from 2_middle_pushed.png | yes |
| 5_middle_pushed_quality_1.png, 2_middle_pushed.png at Quality 1: each cell left whole, the bow drawn in straight pieces, so the two differ; draws cleanly | [], 20870 pixels differ between the qualities | yes |

## Result

130 of 130 checks pass.
