# D-636: Shatter's other ten patterns

B-516, After Effects' Herringbone 2, Squares & Triangles, Stars & Triangles, Overlapping Squares, Chevrons, Puzzle, Js, Eggs, Crescents and Carpenter's Wheel, under the D-630 approval for the owner's /loop request ("The D-630 design is approved; go ahead.") and the coordinator's "Go on with D-636." Adobe names the patterns and shows no geometry, so the shapes are ours, stated in `tools/shatter_reference.py` (step 1). Every expected pixel was written by the tool before this code existed. Tolerance 2e-5.

## FX-SH-093 to 104 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SH-093 frame 3: Pattern herringbone 2, three across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 1.7e-7 | yes |
| FX-SH-093: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-094 frame 3: Pattern squares triangles, three across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 1.8e-7 | yes |
| FX-SH-094: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-095 frame 3: Pattern stars triangles, six across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 1.8e-7 | yes |
| FX-SH-095: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-096 frame 3: Pattern overlapping squares, three across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 1.6e-7 | yes |
| FX-SH-096: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-097 frame 3: Pattern chevrons, three across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 1.9e-7 | yes |
| FX-SH-097: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-098 frame 3: Pattern puzzle, two across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 2.1e-7 | yes |
| FX-SH-098: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-099 frame 3: Pattern js, three across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 1.7e-7 | yes |
| FX-SH-099: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-100 frame 3: Pattern eggs, two across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 2.0e-7 | yes |
| FX-SH-100: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-101 frame 3: Pattern crescents, two across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 1.4e-7 | yes |
| FX-SH-101: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-102 frame 3: Pattern carpenters wheel, two across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 1.9e-7 | yes |
| FX-SH-102: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-103 frame 0: Wireframe Front at rest, pattern puzzle two across turned 30 degrees: each piece's outline, knobs and all. | largest difference 0.0e0 | yes |
| FX-SH-103: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-104 frame 0: Pattern "Puzzle": the words are exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-104 frame 4: Pattern "Puzzle": the words are exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-104: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| fx_sh_104.json, pattern "Puzzle", is refused in a sentence naming the pattern | Shatter's pattern is one of "bricks", "squares", "hexagons", "triangles_1", "triangles_2", "rhombus", "planks", "herringbone_1", "octagons_squares", "glass", "custom", "herringbone_2", "squares_triangles", "stars_triangles", "overlapping_squares", "chevrons", "puzzle", "js", "eggs", "crescents", "carpenters_wheel", and this is "Puzzle". | yes |
| fx_sh_093.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_094.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_095.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_096.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_097.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_098.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_099.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_100.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_101.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_102.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_103.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_104.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |

## Pictures (verification/D-636 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_herringbone_2_outlines.png: drawn without a warning, white outlines on clear and nothing else | 10808 white, 118792 clear, 0 other; warnings [] | yes |
| 2_squares_triangles_outlines.png: drawn without a warning, white outlines on clear and nothing else | 10437 white, 119163 clear, 0 other; warnings [] | yes |
| 3_stars_triangles_outlines.png: drawn without a warning, white outlines on clear and nothing else | 11180 white, 118420 clear, 0 other; warnings [] | yes |
| 4_overlapping_squares_outlines.png: drawn without a warning, white outlines on clear and nothing else | 8842 white, 120758 clear, 0 other; warnings [] | yes |
| 5_chevrons_outlines.png: drawn without a warning, white outlines on clear and nothing else | 11190 white, 118410 clear, 0 other; warnings [] | yes |
| 6_puzzle_outlines.png: drawn without a warning, white outlines on clear and nothing else | 9468 white, 120132 clear, 0 other; warnings [] | yes |
| 7_js_outlines.png: drawn without a warning, white outlines on clear and nothing else | 9134 white, 120466 clear, 0 other; warnings [] | yes |
| 8_eggs_outlines.png: drawn without a warning, white outlines on clear and nothing else | 9928 white, 119672 clear, 0 other; warnings [] | yes |
| 9_crescents_outlines.png: drawn without a warning, white outlines on clear and nothing else | 15540 white, 114060 clear, 0 other; warnings [] | yes |
| 10_carpenters_wheel_outlines.png: drawn without a warning, white outlines on clear and nothing else | 19778 white, 109822 clear, 0 other; warnings [] | yes |
| 11_puzzle_burst.png: drawn without a warning, the flown pieces leaving clear holes | 40199 clear; warnings [] | yes |
| 12_stars_burst.png: drawn without a warning, the flown pieces leaving clear holes | 36025 clear; warnings [] | yes |

## Result

50 of 50 checks pass.
