# B-641: Cell Pattern's Offset, Overflow, Evolution Options and Tiling Options (D-762)

D-762 adds After Effects' Offset (`offset`, pixels, x then y, keyable, absent 0, 0), Overflow (`overflow`: clip, soft_clamp or wrap_back, absent clip), Evolution Options (`cycle_evolution` off or on, absent off; `cycle`, 1 to 1000 whole turns, absent 1) and Tiling Options (`tiling` off or on, absent off; `tiles_horizontal` and `tiles_vertical`, 1 to 1000 cells, absent 10), and lets Contrast reach 10000 (document 21). A file without them draws as before. Every expected pixel is `Fixtures/cell_pattern/expected_cell_pattern.json`, written by `tools/cell_pattern_reference.py` before this code existed and printed in document 25 as FX-CELL-034 to 059. Tolerance 2e-5.

## FX-CELL-034 to 059 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CELL-034 frame 0: Offset 2, 0: the cells slid two pixels right over the unmoved drawing; each shown pixel's colour is FX-CELL-001's two pixels to its left. | largest difference 3.0e-8 | yes |
| FX-CELL-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-035 frame 0: Offset 0, 0 written: FX-CELL-001 exactly. | largest difference 3.0e-8 | yes |
| FX-CELL-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-036 frame 0: Offset -1.5, 2.5: the cells slid a pixel and a half left and two and a half down, between pixels. | largest difference 3.0e-8 | yes |
| FX-CELL-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-037 frame 0: Offset keyed from 0, 0 at frame 0 to 4, 0 at frame 4, linear: frame 0 is FX-CELL-001 and frame 4 the cells four pixels right, one whole cell. | largest difference 3.0e-8 | yes |
| FX-CELL-037 frame 2: Offset keyed from 0, 0 at frame 0 to 4, 0 at frame 4, linear: frame 0 is FX-CELL-001 and frame 4 the cells four pixels right, one whole cell. | largest difference 3.0e-8 | yes |
| FX-CELL-037 frame 4: Offset keyed from 0, 0 at frame 0 to 4, 0 at frame 4, linear: frame 0 is FX-CELL-001 and frame 4 the cells four pixels right, one whole cell. | largest difference 3.0e-8 | yes |
| FX-CELL-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-038 frame 0: Overflow clip written, contrast 300: FX-CELL-006 exactly. | largest difference 3.0e-8 | yes |
| FX-CELL-038: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-039 frame 0: Overflow soft clamp, contrast 300: the greys past black and white eased back inside, so no pixel is pure black or white and the order of the greys is kept. | largest difference 3.0e-8 | yes |
| FX-CELL-039: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-040 frame 0: Overflow wrap back, contrast 300: the greys past white fold back down and those past black fold back up, so the brightest balls get dark middles. | largest difference 3.0e-8 | yes |
| FX-CELL-040: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-041 frame 0: Contrast 10000, the new top, clipped: almost every pixel pure black or white. | largest difference 3.0e-8 | yes |
| FX-CELL-041: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-042 frame 0: Contrast 10000 with wrap back: the greys folded many times, rings in the balls. | largest difference 3.0e-8 | yes |
| FX-CELL-042: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-043 frame 0: Cycle Evolution on, cycle 1, evolution 90: one revolution a cycle turns every point as before, so this is FX-CELL-010 exactly. | largest difference 3.0e-8 | yes |
| FX-CELL-043: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-044 frame 0: Cycle Evolution on, cycle 2, evolution keyed from 0 at frame 0 to 720 at frame 4, linear: frame 0 is FX-CELL-001, frame 2 (360 degrees) is not, and frame 4, the end of the two-revolution cycle, is FX-CELL-001 again exactly: the loop closes. | largest difference 3.0e-8 | yes |
| FX-CELL-044 frame 2: Cycle Evolution on, cycle 2, evolution keyed from 0 at frame 0 to 720 at frame 4, linear: frame 0 is FX-CELL-001, frame 2 (360 degrees) is not, and frame 4, the end of the two-revolution cycle, is FX-CELL-001 again exactly: the loop closes. | largest difference 3.0e-8 | yes |
| FX-CELL-044 frame 4: Cycle Evolution on, cycle 2, evolution keyed from 0 at frame 0 to 720 at frame 4, linear: frame 0 is FX-CELL-001, frame 2 (360 degrees) is not, and frame 4, the end of the two-revolution cycle, is FX-CELL-001 again exactly: the loop closes. | largest difference 3.0e-8 | yes |
| FX-CELL-044: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-045 frame 0: Cycle Evolution off, cycle 3 written, evolution 90: the cycle is not used, so this is FX-CELL-010. | largest difference 3.0e-8 | yes |
| FX-CELL-045: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-046 frame 0: Cycle Evolution on, cycle 2.7, evolution 720: the whole part, 2, is counted, so 720 degrees is a whole cycle and this is FX-CELL-001. | largest difference 3.0e-8 | yes |
| FX-CELL-046: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-047 frame 0: Tiling on, 2 by 2 tiles: the cells repeat every two across and two down, so the pattern repeats every eight pixels. | largest difference 3.0e-8 | yes |
| FX-CELL-047: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-048 frame 0: Tiling off, 2 by 2 tiles written: FX-CELL-001. | largest difference 3.0e-8 | yes |
| FX-CELL-048: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-049 frame 0: Every new setting written at the value a file without it means: offset 0, 0, clip, cycle off, cycle 1, tiling off, 10 by 10 tiles: FX-CELL-001 exactly. | largest difference 3.0e-8 | yes |
| FX-CELL-049: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CELL-050 frame 0: Contrast 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-050 frame 4: Contrast 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-050: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-051 frame 0: Overflow "allow_hdr", which is not "clip", "soft_clamp" or "wrap_back". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-051 frame 4: Overflow "allow_hdr", which is not "clip", "soft_clamp" or "wrap_back". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-051: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-052 frame 0: Cycle Evolution "yes", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-052 frame 4: Cycle Evolution "yes", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-052: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-053 frame 0: Cycle 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-053 frame 4: Cycle 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-053: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-054 frame 0: Cycle 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-054 frame 4: Cycle 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-054: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-055 frame 0: Tiling "yes", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-055 frame 4: Tiling "yes", which is not "on" or "off". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-055: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-056 frame 0: Tiles Horizontal 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-056 frame 4: Tiles Horizontal 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-056: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-057 frame 0: Tiles Vertical 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-057 frame 4: Tiles Vertical 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-057: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-058 frame 0: Offset 100001, 0, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-058 frame 4: Offset 100001, 0, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-058: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CELL-059 frame 0: Offset keyed to -100001, 0 at frame 4, below -100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-059 frame 4: Offset keyed to -100001, 0 at frame 4, below -100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CELL-059: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Older projects: none of the new settings, drawn and saved as before

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cell_001.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_002.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_003.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_004.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_005.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_006.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_007.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_008.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_009.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_010.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_011.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_012.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_013.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_014.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_015.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_016.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_017.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_018.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_019.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_020.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_021.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_022.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_023.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_024.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_025.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_026.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_027.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_028.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_029.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_030.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_031.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_032.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |
| fx_cell_033.json (none of the new settings) is saved as it was written (letters' case aside: 014's capital colour is read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with all seven written at their absent values | saved the same; bit-identical | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cell_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cell_001.json (none of the new settings) is saved with none of them written | {"blend":"normal","contrast":100,"dark_color":"#000000","disperse":1,"evolution":0,"invert":"off","light_color":"#ffffff","opacity":100,"pattern":"bubbles","seed":0,"size":4} | yes |
| fx_cell_049.json, all seven written at their absent values, is saved with all seven kept | {"blend":"normal","contrast":100,"cycle":1,"cycle_evolution":"off","dark_color":"#000000","disperse":1,"evolution":0,"invert":"off","light_color":"#ffffff","offset":[0,0],"opacity":100,"overflow":"clip","pattern":"bubbles","seed":0,"size":4,"tiles_horizontal":10,"tiles_vertical":10,"tiling":"off"} | yes |
| fx_cell_050.json (Contrast 10001) is refused in a sentence naming contrast | Cell Pattern's contrast runs from 0 to 10000, and this is 10001. | yes |
| fx_cell_051.json (Overflow "allow_hdr") is refused in a sentence naming overflow | Cell Pattern's overflow is "clip", "soft_clamp" or "wrap_back", and this is "allow_hdr". | yes |
| fx_cell_052.json (Cycle Evolution "yes") is refused in a sentence naming cycle evolution | Cell Pattern's cycle evolution is "on" or "off", and this is "yes". | yes |
| fx_cell_053.json (Cycle 0) is refused in a sentence naming cycle | Cell Pattern's cycle runs from 1 to 1000, and this is 0. | yes |
| fx_cell_054.json (Cycle 1001) is refused in a sentence naming cycle | Cell Pattern's cycle runs from 1 to 1000, and this is 1001. | yes |
| fx_cell_055.json (Enable Tiling "yes") is refused in a sentence naming tiling | Cell Pattern's tiling is "on" or "off", and this is "yes". | yes |
| fx_cell_056.json (Cells Horizontal 0) is refused in a sentence naming tiles horizontal | Cell Pattern's tiles horizontal runs from 1 to 1000, and this is 0. | yes |
| fx_cell_057.json (Cells Vertical 1001) is refused in a sentence naming tiles vertical | Cell Pattern's tiles vertical runs from 1 to 1000, and this is 1001. | yes |
| fx_cell_058.json (Offset 100001, 0) is refused in a sentence naming offset | Cell Pattern's offset runs from -100000 to 100000, and this is 100001. | yes |
| a file with an offset that is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an overflow that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a cycle that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a tiling that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| a half-size draft preview halves the offset, -1.5, 2.5 to -0.75, 1.25, with the size | CellPattern { pattern: "bubbles", invert: "off", contrast: 100.0, disperse: 1.0, size: 2.0, evolution: 0.0, seed: 0.0, dark_color: "#000000", light_color: "#ffffff", opacity: 100.0, blend: "normal", offset: [-0.75, 1.25], overflow: "clip", cycle_evolution: "off", cycle: 1.0, tiling: "off", tiles_horizontal: 10.0, tiles_vertical: 10.0 } | yes |
| it keeps the cells across and down and the cycle, which are counts, not distances | CellPattern { pattern: "bubbles", invert: "off", contrast: 100.0, disperse: 1.0, size: 2.0, evolution: 0.0, seed: 0.0, dark_color: "#000000", light_color: "#ffffff", opacity: 100.0, blend: "normal", offset: [0.0, 0.0], overflow: "clip", cycle_evolution: "off", cycle: 1.0, tiling: "on", tiles_horizontal: 2.0, tiles_vertical: 2.0 } | yes |
| an offset never grows the layer | 0 | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| overflow "allow_hdr" is refused with a sentence, and nothing changes | Cell Pattern's overflow is "clip", "soft_clamp" or "wrap_back", and this is "allow_hdr". | yes |
| overflow "Clip", with a capital is refused with a sentence, and nothing changes | Cell Pattern's overflow is "clip", "soft_clamp" or "wrap_back", and this is "Clip". | yes |
| cycle evolution "yes" is refused with a sentence, and nothing changes | Cell Pattern's cycle evolution is "on" or "off", and this is "yes". | yes |
| cycle 0 is refused with a sentence, and nothing changes | Cell Pattern's cycle runs from 1 to 1000, and this is 0. | yes |
| cycle 1001 is refused with a sentence, and nothing changes | Cell Pattern's cycle runs from 1 to 1000, and this is 1001. | yes |
| tiling "yes" is refused with a sentence, and nothing changes | Cell Pattern's tiling is "on" or "off", and this is "yes". | yes |
| cells horizontal 0 is refused with a sentence, and nothing changes | Cell Pattern's tiles horizontal runs from 1 to 1000, and this is 0. | yes |
| cells vertical 1001 is refused with a sentence, and nothing changes | Cell Pattern's tiles vertical runs from 1 to 1000, and this is 1001. | yes |
| offset 100001, 0 is refused with a sentence, and nothing changes | Cell Pattern's offset runs from -100000 to 100000, and this is 100001. | yes |
| contrast 10001 is refused with a sentence, and nothing changes | Cell Pattern's contrast runs from 0 to 10000, and this is 10001. | yes |
| offset keyed to -100001, 0 is refused with a sentence, and nothing changes | Cell Pattern's offset runs from -100000 to 100000, and this is -100001. | yes |
| cycle keyed to 0 is refused with a sentence, and nothing changes | Cell Pattern's cycle runs from 1 to 1000, and this is 0. | yes |
| every new setting at its bottom, contrast 0, is taken | taken | yes |
| every new setting at its top, contrast 10000, is taken | taken | yes |
| offset keyed from 0, 0 to 50, -50 is taken | taken | yes |
| cycle keyed from 1 to 5 is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cell_036.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cell_037.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cell_040.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cell_042.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cell_044.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_cell_047.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CELL-001, fx_cell_001.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-002, fx_cell_002.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-003, fx_cell_003.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-004, fx_cell_004.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-005, fx_cell_005.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-006, fx_cell_006.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-007, fx_cell_007.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-008, fx_cell_008.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-009, fx_cell_009.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-010, fx_cell_010.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-011, fx_cell_011.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-012, fx_cell_012.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-013, fx_cell_013.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-014, fx_cell_014.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-015, fx_cell_015.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-016, fx_cell_016.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-017, fx_cell_017.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-018, fx_cell_018.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-019, fx_cell_019.json, frames [0, 2, 4] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-020, fx_cell_020.json, frames [0, 2] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-021, fx_cell_021.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-022, fx_cell_022.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-023, fx_cell_023.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-024, fx_cell_024.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-025, fx_cell_025.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-026, fx_cell_026.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-027, fx_cell_027.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-028, fx_cell_028.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-029, fx_cell_029.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-030, fx_cell_030.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-031, fx_cell_031.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-032, fx_cell_032.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-033, fx_cell_033.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-034, fx_cell_034.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-035, fx_cell_035.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-036, fx_cell_036.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-037, fx_cell_037.json, frames [0, 2, 4] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-038, fx_cell_038.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-039, fx_cell_039.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-040, fx_cell_040.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-041, fx_cell_041.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-042, fx_cell_042.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-043, fx_cell_043.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-044, fx_cell_044.json, frames [0, 2, 4] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-045, fx_cell_045.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-046, fx_cell_046.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-047, fx_cell_047.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-048, fx_cell_048.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-049, fx_cell_049.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-050, fx_cell_050.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-051, fx_cell_051.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-052, fx_cell_052.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-053, fx_cell_053.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-054, fx_cell_054.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-055, fx_cell_055.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-056, fx_cell_056.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-057, fx_cell_057.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-058, fx_cell_058.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-CELL-059, fx_cell_059.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Cell Pattern, opacity 60, offset 25, -10, overflow wrap back at contrast 400: the processor's frame 100 differs from the shot without it | 2072521 pixels changed | yes |
| the reference shot, Cell Pattern, opacity 60, offset 25, -10, overflow wrap back at contrast 400, frame 0, Full | largest difference 1 of 255, 1188 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cell Pattern, opacity 60, offset 25, -10, overflow wrap back at contrast 400, frame 100, Full | largest difference 1 of 255, 1158 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cell Pattern, opacity 60, offset 25, -10, overflow wrap back at contrast 400, frame 239, Full | largest difference 1 of 255, 1202 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cell Pattern, opacity 60, offset 25, -10, overflow wrap back at contrast 400, frame 0, Draft | largest difference 1 of 255, 83 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cell Pattern, opacity 60, offset 25, -10, overflow wrap back at contrast 400, frame 100, Draft | largest difference 1 of 255, 75 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cell Pattern, opacity 60, offset 25, -10, overflow wrap back at contrast 400, frame 239, Draft | largest difference 1 of 255, 78 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cell Pattern, opacity 60, soft clamp at contrast 300, tiled 6 by 4, cycle 3 at evolution 500: the processor's frame 100 differs from the shot without it | 2073061 pixels changed | yes |
| the reference shot, Cell Pattern, opacity 60, soft clamp at contrast 300, tiled 6 by 4, cycle 3 at evolution 500, frame 0, Full | largest difference 1 of 255, 1089 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cell Pattern, opacity 60, soft clamp at contrast 300, tiled 6 by 4, cycle 3 at evolution 500, frame 100, Full | largest difference 1 of 255, 1040 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cell Pattern, opacity 60, soft clamp at contrast 300, tiled 6 by 4, cycle 3 at evolution 500, frame 239, Full | largest difference 1 of 255, 1099 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cell Pattern, opacity 60, soft clamp at contrast 300, tiled 6 by 4, cycle 3 at evolution 500, frame 0, Draft | largest difference 1 of 255, 65 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cell Pattern, opacity 60, soft clamp at contrast 300, tiled 6 by 4, cycle 3 at evolution 500, frame 100, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Cell Pattern, opacity 60, soft clamp at contrast 300, tiled 6 by 4, cycle 3 at evolution 500, frame 239, Draft | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/B-641 pictures/`, the street under size-40 bubbles

| Check | The build's answer | Matches |
| --- | --- | --- |
| offset_0.png and offset_40_0.png: Offset 40, 0 slides the whole pattern 40 pixels right, each pixel the one 40 to its left in the picture without; both draw cleanly | [] []; slid exactly: true | yes |
| overflow_clip.png, overflow_soft_clamp.png, overflow_wrap_back.png, contrast 300: Clip pins many pixels at black or white; Soft Clamp eases them in, so far fewer sit there; Wrap Back folds them back, drawing rings inside the balls; all draw cleanly | pixels at black or white: clip 79050, soft clamp 0, wrap back 426; [] [] [] | yes |
| cycle_frame_0.png to cycle_frame_4.png: Cycle Evolution on, Cycle 2, Evolution keyed 0 to 720 over frames 0 to 4: the cells turn at their own speeds, so frame 2 (one turn) is not frame 0, and frame 4 (two turns, the cycle's end) is frame 0 again pixel for pixel: the loop closes; all draw cleanly | frame 4 against frame 0: 0 pixels differ; frame 2 against frame 0: 100931 | yes |
| tiling_4_by_2.png: Enable Tiling on, 4 cells across and 2 down at size 40: the pattern repeats every 160 pixels across and 80 down; draws cleanly | []; repeats: true | yes |
| bubbles_unchanged.png: the Bubbles look is kept: a Bubbles with every new setting written at its absent value is offset_0.png pixel for pixel | []; 0 pixels differ | yes |

## Result

234 of 234 checks pass.
