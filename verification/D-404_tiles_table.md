# D-404: Tiles

B-283, after CycoreFX's CC Tiler: the layer shrunk to Scale per cent and repeated in a grid across its own size, one tile centred on Center, by Motion Tile's rule (D-304: each pixel the mean of a grid of points, a pixel's worth of the layer each), then mixed with the untiled layer by Blend w. Original. The layer never grows. Every expected pixel is `Fixtures/tiles/expected_tiles.json`, written by `tools/tiles_reference.py` before this code existed and printed in document 25 as FX-TILES-001 to 021. Tolerance 2e-5.

## FX-TILES-001 to 021 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TILES-001 frame 0: The settings as they start, scale 50, centre 50, 50, blend 0: tiles 8 by 5 set round the middle, one starting at column 4 and row 2.5, so the frame holds two tiles across and two down, cut at the edges, each pixel the mean of 2 by 2 points. | largest difference 1.9e-7 | yes |
| FX-TILES-001 frame 4: The settings as they start, scale 50, centre 50, 50, blend 0: tiles 8 by 5 set round the middle, one starting at column 4 and row 2.5, so the frame holds two tiles across and two down, cut at the edges, each pixel the mean of 2 by 2 points. | largest difference 1.9e-7 | yes |
| FX-TILES-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILES-002 frame 0: Scale 100 with the centre at 50, 50: the one tile is the picture itself, untouched. | largest difference 1.9e-7 | yes |
| FX-TILES-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILES-003 frame 0: Scale 100 with the centre at 25, 50: the picture slid four columns left, the four columns that fall off the left coming round on the right, pixel for pixel: column x is the drawing's column (x + 4) mod 16. | largest difference 1.9e-7 | yes |
| FX-TILES-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILES-004 frame 0: Scale 25: tiles 4 by 2.5, one starting at column 6 and row 3.75, each pixel the mean of 4 by 4 points; across, the frame repeats every 4 columns. | largest difference 3.1e-7 | yes |
| FX-TILES-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILES-005 frame 0: Scale 75: tiles 12 by 7.5, more than half the picture, so the tiles' edges show near the frame's sides, each pixel the mean of 2 by 2 points. | largest difference 2.5e-7 | yes |
| FX-TILES-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILES-006 frame 0: Scale 33.3: 1 / 0.333 is just over 3, so 4 by 4 points a pixel; tiles 5.328 by 3.33. | largest difference 3.3e-7 | yes |
| FX-TILES-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILES-007 frame 0: Scale 1, the least: tiles 0.16 by 0.1 of a pixel, each pixel the mean of 16 by 16 points, the most there are, so the picture becomes a fine even mixture. | largest difference 1.6e-6 | yes |
| FX-TILES-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILES-008 frame 0: Scale 50 with the centre at 25, 50: the tiles start at column 0 instead of 4, so the frame is FX-TILES-001's slid four columns left, wrapping. | largest difference 1.9e-7 | yes |
| FX-TILES-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILES-009 frame 0: Blend 50: halfway between FX-TILES-001's tiles and the drawing, every channel. | largest difference 2.0e-7 | yes |
| FX-TILES-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILES-010 frame 0: Blend 100: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-TILES-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILES-011 frame 0: Scale keyed from 100 at frame 0 to 50 at frame 4, linear: frame 0 the drawing, frame 2 scale 75 (FX-TILES-005), frame 4 FX-TILES-001. | largest difference 1.9e-7 | yes |
| FX-TILES-011 frame 2: Scale keyed from 100 at frame 0 to 50 at frame 4, linear: frame 0 the drawing, frame 2 scale 75 (FX-TILES-005), frame 4 FX-TILES-001. | largest difference 2.5e-7 | yes |
| FX-TILES-011 frame 4: Scale keyed from 100 at frame 0 to 50 at frame 4, linear: frame 0 the drawing, frame 2 scale 75 (FX-TILES-005), frame 4 FX-TILES-001. | largest difference 1.9e-7 | yes |
| FX-TILES-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILES-012 frame 0: Blend keyed from 100 at frame 0 to 0 at frame 4: frame 0 the drawing, frame 2 FX-TILES-009, frame 4 FX-TILES-001. | largest difference 1.9e-7 | yes |
| FX-TILES-012 frame 2: Blend keyed from 100 at frame 0 to 0 at frame 4: frame 0 the drawing, frame 2 FX-TILES-009, frame 4 FX-TILES-001. | largest difference 2.0e-7 | yes |
| FX-TILES-012 frame 4: Blend keyed from 100 at frame 0 to 0 at frame 4: frame 0 the drawing, frame 2 FX-TILES-009, frame 4 FX-TILES-001. | largest difference 1.9e-7 | yes |
| FX-TILES-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILES-013 frame 0: Centre keyed from 50, 50 at frame 0 to 25, 50 at frame 4 at scale 100: the picture slides left and wraps, two columns at frame 2. | largest difference 1.9e-7 | yes |
| FX-TILES-013 frame 2: Centre keyed from 50, 50 at frame 0 to 25, 50 at frame 4 at scale 100: the picture slides left and wraps, two columns at frame 2. | largest difference 1.9e-7 | yes |
| FX-TILES-013 frame 4: Centre keyed from 50, 50 at frame 0 to 25, 50 at frame 4 at scale 100: the picture slides left and wraps, two columns at frame 2. | largest difference 1.9e-7 | yes |
| FX-TILES-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILES-014 frame 0: Scale eased from 50 at frame 0 to 1 at frame 4 on a curve that overshoots: at frame 2 it would pass below 1 and is held there, so frames 2 and 4 are FX-TILES-007. | largest difference 1.9e-7 | yes |
| FX-TILES-014 frame 2: Scale eased from 50 at frame 0 to 1 at frame 4 on a curve that overshoots: at frame 2 it would pass below 1 and is held there, so frames 2 and 4 are FX-TILES-007. | largest difference 1.6e-6 | yes |
| FX-TILES-014 frame 4: Scale eased from 50 at frame 0 to 1 at frame 4 on a curve that overshoots: at frame 2 it would pass below 1 and is held there, so frames 2 and 4 are FX-TILES-007. | largest difference 1.6e-6 | yes |
| FX-TILES-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILES-015 frame 0: FX-TILES-001 with the layer moved three pixels right: the same tiles, moved; nothing grows, so the three columns past the right edge are cut and columns 0 to 2 stay empty. | largest difference 1.9e-7 | yes |
| FX-TILES-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TILES-016 frame 0: Scale 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILES-016 frame 4: Scale 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILES-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILES-017 frame 0: Scale 100.5, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILES-017 frame 4: Scale 100.5, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILES-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILES-018 frame 0: Blend -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILES-018 frame 4: Blend -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILES-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILES-019 frame 0: Blend 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILES-019 frame 4: Blend 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILES-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILES-020 frame 0: Centre at 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILES-020 frame 4: Centre at 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILES-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TILES-021 frame 0: Scale keyed to 0 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILES-021 frame 4: Scale keyed to 0 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TILES-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Tiles is Motion Tile's sized tile

| Check | The build's answer | Matches |
| --- | --- | --- |
| Tiles as it starts, scale 50, blend 0, on the street is Motion Tile's picture with the same tile byte for byte | 0 pixels differ | yes |
| Tiles scale 25, centre 30, 40, blend 0, on the street is Motion Tile's picture with the same tile byte for byte | 0 pixels differ | yes |
| Tiles scale 7, blend 0, on the street is Motion Tile's picture with the same tile byte for byte | 0 pixels differ | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tiles_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tiles_009.json is saved with its scale, centre and blend | {"blend":50,"center":[50,50],"scale":50} | yes |
| fx_tiles_016.json is refused in a sentence | Tiles's scale runs from 1 to 100, and this is 0.5. | yes |
| fx_tiles_017.json is refused in a sentence | Tiles's scale runs from 1 to 100, and this is 100.5. | yes |
| fx_tiles_018.json is refused in a sentence | Tiles's blend runs from 0 to 100, and this is -1. | yes |
| fx_tiles_019.json is refused in a sentence | Tiles's blend runs from 0 to 100, and this is 101. | yes |
| fx_tiles_020.json is refused in a sentence | Tiles's center runs from -1000 to 1000, and this is 1001. | yes |
| a file with a Tiles with no `blend` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Tiles with a centre of one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| scale 0.5 is refused with a sentence, and nothing changes | Tiles's scale runs from 1 to 100, and this is 0.5. | yes |
| blend 100.5 is refused with a sentence, and nothing changes | Tiles's blend runs from 0 to 100, and this is 100.5. | yes |
| scale keyed to 0 is refused with a sentence, and nothing changes | Tiles's scale runs from 1 to 100, and this is 0. | yes |
| scale 25, centre 30, 40, blend 20 is taken | taken | yes |
| scale keyed from 100 to 20 is taken | taken | yes |
| centre keyed from 50, 50 to 10, 90 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tiles_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tiles_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tiles_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tiles_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tiles_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tiles_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tiles_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_tiles_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_tiles_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_tiles_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_tiles_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_tiles_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_tiles_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_tiles_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_tiles_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_tiles_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_tiles_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_tiles_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_tiles_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_tiles_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_tiles_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_tiles_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_tiles_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_tiles_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_tiles_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_tiles_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_tiles_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Tiles as it starts (scale 50, centre 50, 50, blend 0): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2069644 pixels changed | yes |
| the reference shot, Tiles as it starts (scale 50, centre 50, 50, blend 0) on three layers, frame 0, Full | largest difference 1 of 255, 2030 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tiles as it starts (scale 50, centre 50, 50, blend 0) on three layers, frame 100, Full | largest difference 1 of 255, 596 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tiles as it starts (scale 50, centre 50, 50, blend 0) on three layers, frame 239, Full | largest difference 1 of 255, 1248 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tiles as it starts (scale 50, centre 50, 50, blend 0) on three layers, frame 0, Draft | largest difference 1 of 255, 83 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tiles as it starts (scale 50, centre 50, 50, blend 0) on three layers, frame 100, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tiles as it starts (scale 50, centre 50, 50, blend 0) on three layers, frame 239, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tiles scale 25, centre 30, 40, blend 20: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2062533 pixels changed | yes |
| the reference shot, Tiles scale 25, centre 30, 40, blend 20 on three layers, frame 0, Full | largest difference 1 of 255, 853 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tiles scale 25, centre 30, 40, blend 20 on three layers, frame 100, Full | largest difference 1 of 255, 1036 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tiles scale 25, centre 30, 40, blend 20 on three layers, frame 239, Full | largest difference 1 of 255, 816 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tiles scale 25, centre 30, 40, blend 20 on three layers, frame 0, Draft | largest difference 1 of 255, 84 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tiles scale 25, centre 30, 40, blend 20 on three layers, frame 100, Draft | largest difference 1 of 255, 76 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tiles scale 25, centre 30, 40, blend 20 on three layers, frame 239, Draft | largest difference 1 of 255, 80 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tiles scale 7, centre -120, 300: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2069337 pixels changed | yes |
| the reference shot, Tiles scale 7, centre -120, 300 on three layers, frame 0, Full | largest difference 1 of 255, 944 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tiles scale 7, centre -120, 300 on three layers, frame 100, Full | largest difference 1 of 255, 956 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tiles scale 7, centre -120, 300 on three layers, frame 239, Full | largest difference 1 of 255, 943 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tiles scale 7, centre -120, 300 on three layers, frame 0, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tiles scale 7, centre -120, 300 on three layers, frame 100, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tiles scale 7, centre -120, 300 on three layers, frame 239, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-404 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: four copies of the street at half size, 2 by 2; draws cleanly | [], 128190 pixels changed | yes |
| 3_scale_25.png, scale 25: 4 by 4 copies at a quarter size; draws cleanly | [], 123102 pixels changed | yes |
| 4_off_centre.png, scale 33, centre 20, 30: the grid slid up and left, tiles cut at the edges; draws cleanly | [], 126448 pixels changed | yes |
| 5_blend_50.png, scale 25, blend 50: the grid of copies seen through the whole street; draws cleanly | [], 121950 pixels changed | yes |
| 6_scale_3.png, scale 3: copies so small the street becomes a fine texture; draws cleanly | [], 129120 pixels changed | yes |

## Result

150 of 150 checks pass.
