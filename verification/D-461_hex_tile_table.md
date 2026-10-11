# D-461: Hex Tile

B-341: `core.hex_tile` takes After Effects' CC HexTile controls (Render, Radius, Center, Lock Center Tile, Rotate, Smearing). Every expected pixel is `Fixtures/hex_tile/expected_hex_tile.json`, written by `tools/hex_tile_reference.py` before this code existed and printed in document 25 as FX-HEX-001 to 022. Tolerance 2e-5.

## FX-HEX-001 to 022 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-HEX-001 frame 0: The settings as they start: Tile, radius 50, the centre in the middle, unlocked, no turn, no smearing. The middle tile holds all of this small drawing, so the picture is as it was. | largest difference 1.9e-7 | yes |
| FX-HEX-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HEX-002 frame 0: Radius 3, Tile: the hexagon round the middle repeated in a honeycomb, every tile the same. | largest difference 1.9e-7 | yes |
| FX-HEX-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HEX-003 frame 0: Radius 3, Fold Aligned: each tile six turned copies of one triangle of the middle. | largest difference 2.5e-7 | yes |
| FX-HEX-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HEX-004 frame 0: Radius 3, Fold Seamlessly: each tile six triangles, every other one mirrored, meeting their neighbours with no seam. | largest difference 2.5e-7 | yes |
| FX-HEX-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HEX-005 frame 0: Radius 3, Rotate 30: the honeycomb turned a twelfth round. | largest difference 1.9e-7 | yes |
| FX-HEX-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HEX-006 frame 0: Radius 3, the centre at 30 per cent across and 40 down: the tiles show the picture there, and the honeycomb starts there. | largest difference 1.9e-7 | yes |
| FX-HEX-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HEX-007 frame 0: FX-HEX-006 with Lock Center Tile on: the same picture in the tiles, the honeycomb started at the drawing's middle. | largest difference 2.5e-7 | yes |
| FX-HEX-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HEX-008 frame 0: Radius 3, Smearing 50: each tile shows the picture half way from the centre to its own place. | largest difference 2.5e-7 | yes |
| FX-HEX-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HEX-009 frame 0: Radius 3, Smearing 100, Tile, unlocked: every tile shows its own place, the picture as it was. | largest difference 1.9e-7 | yes |
| FX-HEX-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HEX-010 frame 0: Radius keyed from 2 at frame 0 to 6 at frame 4, linear: frame 2 is radius 4. | largest difference 1.9e-7 | yes |
| FX-HEX-010 frame 2: Radius keyed from 2 at frame 0 to 6 at frame 4, linear: frame 2 is radius 4. | largest difference 1.9e-7 | yes |
| FX-HEX-010 frame 4: Radius keyed from 2 at frame 0 to 6 at frame 4, linear: frame 2 is radius 4. | largest difference 1.9e-7 | yes |
| FX-HEX-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HEX-011 frame 0: Smearing keyed from 50 at frame 0 to 100 at frame 4, eased past its end, radius 3: frame 2 would pass 100, is held at 100, and is FX-HEX-009. | largest difference 2.5e-7 | yes |
| FX-HEX-011 frame 2: Smearing keyed from 50 at frame 0 to 100 at frame 4, eased past its end, radius 3: frame 2 would pass 100, is held at 100, and is FX-HEX-009. | largest difference 1.9e-7 | yes |
| FX-HEX-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HEX-012 frame 0: FX-HEX-002 moved three pixels right: the honeycomb moves with the drawing and the three columns left bare stay empty. | largest difference 1.9e-7 | yes |
| FX-HEX-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HEX-013 frame 0: After a Motion Tile that grows the layer, radius 3: the centre is the drawing's own, not the grown buffer's, and each tile is FX-HEX-002's. | largest difference 1.9e-7 | yes |
| FX-HEX-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HEX-014 frame 0: Radius 4, Fold Seamlessly, Rotate 15, the centre at 40, 60, Smearing 25: everything at once. | largest difference 2.5e-7 | yes |
| FX-HEX-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-HEX-015 frame 0: Radius 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HEX-015 frame 4: Radius 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HEX-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HEX-016 frame 0: Radius 4001, above 4000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HEX-016 frame 4: Radius 4001, above 4000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HEX-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HEX-017 frame 0: Render "honeycomb", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HEX-017 frame 4: Render "honeycomb", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HEX-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HEX-018 frame 0: Lock Center Tile "maybe", neither on nor off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HEX-018 frame 4: Lock Center Tile "maybe", neither on nor off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HEX-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HEX-019 frame 0: Smearing 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HEX-019 frame 4: Smearing 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HEX-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HEX-020 frame 0: Rotate 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HEX-020 frame 4: Rotate 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HEX-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HEX-021 frame 0: Centre across 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HEX-021 frame 4: Centre across 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HEX-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-HEX-022 frame 0: Radius keyed to 5000 at frame 4, above 4000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HEX-022 frame 4: Radius keyed to 5000 at frame 4, above 4000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-HEX-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_hex_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_hex_014.json is saved with its six settings | {"center":[40,60],"lock_center":"off","radius":4,"render":"fold_seamless","rotate":15,"smearing":25} | yes |
| fx_hex_015.json is refused in a sentence naming Hex Tile and its radius | Hex Tile's radius runs from 1 to 4000, and this is 0.5. | yes |
| fx_hex_016.json is refused in a sentence naming Hex Tile and its radius | Hex Tile's radius runs from 1 to 4000, and this is 4001. | yes |
| fx_hex_017.json is refused in a sentence naming Hex Tile and its render | Hex Tile's render is one of tile, fold_aligned, fold_seamless, and this is "honeycomb". | yes |
| fx_hex_018.json is refused in a sentence naming Hex Tile and its lock center | Hex Tile's lock center is "on" or "off", and this is "maybe". | yes |
| fx_hex_019.json is refused in a sentence naming Hex Tile and its smearing | Hex Tile's smearing runs from 0 to 100, and this is 101. | yes |
| fx_hex_020.json is refused in a sentence naming Hex Tile and its rotate | Hex Tile's rotate runs from -3600 to 3600, and this is 3601. | yes |
| fx_hex_021.json is refused in a sentence naming Hex Tile and its center | Hex Tile's center runs from -1000 to 1000, and this is 1001. | yes |
| a file with a Hex Tile whose radius is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Hex Tile without its smearing is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Hex Tile whose center is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Hex Tile whose render is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| radius 4001 is refused with a sentence, and nothing changes | Hex Tile's radius runs from 1 to 4000, and this is 4001. | yes |
| center 50, -1001 is refused with a sentence, and nothing changes | Hex Tile's center runs from -1000 to 1000, and this is -1001. | yes |
| render honeycomb is refused with a sentence, and nothing changes | Hex Tile's render is one of tile, fold_aligned, fold_seamless, and this is "honeycomb". | yes |
| radius keyed to 5000 is refused with a sentence, and nothing changes | Hex Tile's radius runs from 1 to 4000, and this is 5000. | yes |
| Fold Aligned, radius 12, center 20, 30, locked, rotate 45, smearing 60 is taken | taken | yes |
| center keyed from 0, 0 to 100, 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_hex_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_hex_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_hex_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_hex_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_hex_010.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_hex_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_hex_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_hex_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_003.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_004.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_006.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_007.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_008.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_009.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_011.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_hex_022.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Hex Tile radius 40: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2069028 pixels changed | yes |
| the reference shot, Hex Tile radius 40 on three layers, frame 0, Full | largest difference 1 of 255, 1122 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hex Tile radius 40 on three layers, frame 100, Full | largest difference 1 of 255, 1129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hex Tile radius 40 on three layers, frame 239, Full | largest difference 1 of 255, 1134 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hex Tile radius 40 on three layers, frame 0, Draft | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hex Tile radius 40 on three layers, frame 100, Draft | largest difference 1 of 255, 38 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hex Tile radius 40 on three layers, frame 239, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hex Tile Fold Seamlessly, radius 60, center 30, 40, rotate 20, smearing 30: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2071602 pixels changed | yes |
| the reference shot, Hex Tile Fold Seamlessly, radius 60, center 30, 40, rotate 20, smearing 30 on three layers, frame 0, Full | largest difference 1 of 255, 979 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hex Tile Fold Seamlessly, radius 60, center 30, 40, rotate 20, smearing 30 on three layers, frame 100, Full | largest difference 1 of 255, 1088 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hex Tile Fold Seamlessly, radius 60, center 30, 40, rotate 20, smearing 30 on three layers, frame 239, Full | largest difference 1 of 255, 995 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hex Tile Fold Seamlessly, radius 60, center 30, 40, rotate 20, smearing 30 on three layers, frame 0, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hex Tile Fold Seamlessly, radius 60, center 30, 40, rotate 20, smearing 30 on three layers, frame 100, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hex Tile Fold Seamlessly, radius 60, center 30, 40, rotate 20, smearing 30 on three layers, frame 239, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hex Tile Fold Aligned, radius 25, locked, center 70, 20: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2069706 pixels changed | yes |
| the reference shot, Hex Tile Fold Aligned, radius 25, locked, center 70, 20 on three layers, frame 0, Full | largest difference 1 of 255, 186 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hex Tile Fold Aligned, radius 25, locked, center 70, 20 on three layers, frame 100, Full | largest difference 1 of 255, 197 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hex Tile Fold Aligned, radius 25, locked, center 70, 20 on three layers, frame 239, Full | largest difference 1 of 255, 183 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hex Tile Fold Aligned, radius 25, locked, center 70, 20 on three layers, frame 0, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hex Tile Fold Aligned, radius 25, locked, center 70, 20 on three layers, frame 100, Draft | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Hex Tile Fold Aligned, radius 25, locked, center 70, 20 on three layers, frame 239, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-461 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| as added (Tile, radius 50): the middle pixel kept, the picture changed, every pixel still covered; draws cleanly | [], middle kept true, changed true, covered 129600 of 129600 | yes |
| 2_tile_radius_20.png, Tile, radius 20: the street's middle repeated in a honeycomb, the picture the same 60 pixels to the right (within 1 level) almost everywhere, every pixel covered; draws cleanly | [], the same 60 to the right at 113400 of 113400 pixels, covered 129600 of 129600 | yes |
| 3_fold_aligned.png and 4_fold_seamless.png, the folds at radius 20: neither is 2_tile_radius_20.png nor each other, every pixel covered; draw cleanly | [] [], different true true true, covered 129600 129600 | yes |
| 5_rotate_30.png, radius 20 turned 30 degrees: not 2_tile_radius_20.png, the middle pixel kept (within 1 level); draws cleanly | [], different true, middle kept within 1 level true | yes |
| 6_smearing_50.png, radius 20 smeared half way: neither the tiles nor the street; at Smearing 100 every tile shows its own place, the street within 1 level everywhere; draws cleanly | [] [], different true true, Smearing 100 within 1 level at 129600 of 129600 | yes |
| 7_locked_center_30_40.png, the tiles showing the street at 30, 40 with the honeycomb held at the middle: not the unlocked picture, and its middle pixel is the street's at 30, 40; draws cleanly | [] [], different true, middle is 30, 40's true | yes |

## Result

153 of 153 checks pass.
