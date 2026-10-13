# D-701: Mesh Warp's vertex handles

B-571: `core.mesh_warp` gains `vertex_handles`, eight numbers a vertex (its right, left, lower and upper Bezier handle, each moved from where the smooth mesh has it, per cent of the drawing). Each cell is the Bezier patch of D-603's surface with its control points moved by the handles; none moved is D-603's frame. Every expected pixel is `Fixtures/mesh_handles/expected_mesh_handles.json`, written by `tools/mesh_handles_reference.py` before this code existed and printed in document 25 as FX-MESHH-001 to 013. Tolerance 2e-5.

## FX-MESHH-001 to 013 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MESHH-001 frame 0: D-603's FX-MESH-003 (the middle vertex pushed to 75 per cent) with every handle written and none moved: the same frame as FX-MESH-003. | largest difference 2.5e-7 | yes |
| FX-MESHH-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESHH-002 frame 0: The mesh at rest written out, every handle written and none moved: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-MESHH-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESHH-003 frame 0: A 2 by 2 mesh at rest (no list), the middle vertex's right handle lifted 20 per cent: no vertex moved, but the stripes right of the middle bend up. | largest difference 2.5e-7 | yes |
| FX-MESHH-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESHH-004 frame 0: FX-MESHH-003 with the mesh at rest written out: the same frame. | largest difference 2.5e-7 | yes |
| FX-MESHH-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESHH-005 frame 0: A 1 by 1 mesh, the top edge's two handles pulled 30 per cent down: the top edge sags in a curve, clear above it, the corners where they were. | largest difference 2.5e-7 | yes |
| FX-MESHH-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESHH-006 frame 0: FX-MESH-003's push with the middle vertex's lower handle swung left and upper right: the bow becomes an S, leaning the other way above and below. | largest difference 2.5e-7 | yes |
| FX-MESHH-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESHH-007 frame 0: FX-MESHH-005's handles keyed from none moved at frame 0 to moved at frame 4, linear: frame 0 the drawing, frame 4 FX-MESHH-005, frame 2 between. | largest difference 1.9e-7 | yes |
| FX-MESHH-007 frame 2: FX-MESHH-005's handles keyed from none moved at frame 0 to moved at frame 4, linear: frame 0 the drawing, frame 4 FX-MESHH-005, frame 2 between. | largest difference 2.5e-7 | yes |
| FX-MESHH-007 frame 4: FX-MESHH-005's handles keyed from none moved at frame 0 to moved at frame 4, linear: frame 0 the drawing, frame 4 FX-MESHH-005, frame 2 between. | largest difference 2.5e-7 | yes |
| FX-MESHH-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESHH-008 frame 0: FX-MESHH-006 with the layer moved three pixels right: the bent drawing moves with it. | largest difference 2.5e-7 | yes |
| FX-MESHH-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESHH-009 frame 0: FX-MESHH-005 at Quality 1: the cell is drawn as one piece straight between its corners, so the sag is lost and the drawing untouched. | largest difference 1.9e-7 | yes |
| FX-MESHH-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESHH-010 frame 0: A handle on the mesh's outside, the top left vertex's left handle, pulled far: it moves nothing, the drawing untouched. | largest difference 1.9e-7 | yes |
| FX-MESHH-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MESHH-011 frame 0: A 2 by 2 mesh's handles written with 70 numbers, not 72. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MESHH-011 frame 4: A 2 by 2 mesh's handles written with 70 numbers, not 72. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MESHH-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MESHH-012 frame 0: A handle moved 401 per cent, past 400. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MESHH-012 frame 4: A handle moved 401 per cent, past 400. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MESHH-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MESHH-013 frame 0: A handle keyed out to -500 per cent at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MESHH-013 frame 4: A handle keyed out to -500 per cent at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MESHH-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_meshh_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_meshh_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_meshh_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_meshh_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_meshh_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_meshh_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_meshh_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_meshh_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_meshh_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_meshh_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_meshh_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_meshh_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_meshh_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_meshh_003.json is saved with its five settings, the handles one flat list of 72 numbers, the middle vertex's right handle's y -20 | {"columns":2,"distortion_mesh":[],"quality":8,"rows":2,"vertex_handles":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-20,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]} | yes |
| fx_meshh_011.json is refused in a sentence naming Mesh Warp and its vertex handles are none, or 72 numbers | Mesh Warp's vertex handles are none, or 72 numbers, eight for each vertex, and this has 70. | yes |
| fx_meshh_012.json is refused in a sentence naming Mesh Warp and its vertex handles | Mesh Warp's vertex handles runs from -400 to 400, and this is 401. | yes |
| a file with a Mesh Warp whose vertex handles are one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| a 2 by 2 mesh's handles as 70 numbers is refused with a sentence, and nothing changes | Mesh Warp's vertex handles are none, or 72 numbers, eight for each vertex, and this has 70. | yes |
| a handle keyed out to 500 per cent is refused with a sentence, and nothing changes | Mesh Warp's vertex handles runs from -400 to 400, and this is 500. | yes |
| the middle's right handle also 10 per cent right is taken | taken | yes |
| the handles keyed from none moved to the middle's lifted is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_meshh_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_meshh_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_meshh_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_meshh_007.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MESHH-001, fx_meshh_001.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MESHH-002, fx_meshh_002.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MESHH-003, fx_meshh_003.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MESHH-004, fx_meshh_004.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MESHH-005, fx_meshh_005.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MESHH-006, fx_meshh_006.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MESHH-007, fx_meshh_007.json, frames [0, 2, 4] at Full and Draft | largest difference 1 of 255; on the card in 4 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-MESHH-008, fx_meshh_008.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MESHH-009, fx_meshh_009.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MESHH-010, fx_meshh_010.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-MESHH-011, fx_meshh_011.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-MESHH-012, fx_meshh_012.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-MESHH-013, fx_meshh_013.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Mesh Warp 2 by 2 at rest, the middle's right handle lifted 20 per cent: the processor's frame 100 differs from the shot without it | 838520 pixels changed | yes |
| the reference shot, Mesh Warp 2 by 2 at rest, the middle's right handle lifted 20 per cent, frame 0, Full | largest difference 1 of 255, 4636 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 2 by 2 at rest, the middle's right handle lifted 20 per cent, frame 100, Full | largest difference 1 of 255, 1423 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 2 by 2 at rest, the middle's right handle lifted 20 per cent, frame 239, Full | largest difference 1 of 255, 1481 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 2 by 2 at rest, the middle's right handle lifted 20 per cent, frame 0, Draft | largest difference 1 of 255, 163 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 2 by 2 at rest, the middle's right handle lifted 20 per cent, frame 100, Draft | largest difference 1 of 255, 40 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 2 by 2 at rest, the middle's right handle lifted 20 per cent, frame 239, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 1 by 1, the top edge's handles pulled 30 per cent down: the processor's frame 100 differs from the shot without it | 1909667 pixels changed | yes |
| the reference shot, Mesh Warp 1 by 1, the top edge's handles pulled 30 per cent down, frame 0, Full | largest difference 1 of 255, 901 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 1 by 1, the top edge's handles pulled 30 per cent down, frame 100, Full | largest difference 1 of 255, 806 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 1 by 1, the top edge's handles pulled 30 per cent down, frame 239, Full | largest difference 1 of 255, 546 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 1 by 1, the top edge's handles pulled 30 per cent down, frame 0, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 1 by 1, the top edge's handles pulled 30 per cent down, frame 100, Draft | largest difference 1 of 255, 66 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 1 by 1, the top edge's handles pulled 30 per cent down, frame 239, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 2 by 2, the middle pushed and its handles turned into an S: the processor's frame 100 differs from the shot without it | 1892318 pixels changed | yes |
| the reference shot, Mesh Warp 2 by 2, the middle pushed and its handles turned into an S, frame 0, Full | largest difference 1 of 255, 464 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 2 by 2, the middle pushed and its handles turned into an S, frame 100, Full | largest difference 1 of 255, 664 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 2 by 2, the middle pushed and its handles turned into an S, frame 239, Full | largest difference 1 of 255, 404 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 2 by 2, the middle pushed and its handles turned into an S, frame 0, Draft | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 2 by 2, the middle pushed and its handles turned into an S, frame 100, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mesh Warp 2 by 2, the middle pushed and its handles turned into an S, frame 239, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-701 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect, every pixel covered; draws cleanly | [], 0 clear pixels | yes |
| every handle of the 7 by 7 mesh written and none moved: the street byte for byte; draws cleanly | [], 0 pixels differ | yes |
| 2_handle_lifted.png, no vertex moved, the middle vertex's right handle lifted 20 per cent: the street right of the middle bends up, the outline kept; draws cleanly | [], 31347 pixels differ from the street, 0 clear | yes |
| 3_top_sags.png, one cell, the top edge's handles pulled 30 per cent down: clear above the top edge, deepest in the middle (about 22 per cent), none at the corners; draws cleanly | [], clear rows from the top at the left edge, a quarter and half across: 0, 46, 61 | yes |
| 4_s_curve.png, the middle pushed right and its handles turned: differs from 5_bow_without_handles.png, the same push with no handle moved; draws cleanly | [], 16821 pixels differ from the bow | yes |

## Result

96 of 96 checks pass.
