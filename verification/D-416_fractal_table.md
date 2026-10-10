# D-416: Fractal

B-295, after After Effects' Fractal: the Mandelbrot or Julia set in place of the layer. A pixel's point is the view's centre plus its distance from the drawing's middle times 3 / (height 2^magnification); z^n + c is iterated until a part passes 2 or the Escape Limit runs out (inside the set). The count picks a band: Lightness Gradient's 8 gradients of Cycle Steps lightnesses, each 45 degrees of hue on, Hue Wheel's Cycle Steps hues, Black And White's two, or Solid Color's one colour for the set. Edge Detect works a pixel Factor by Factor where its band differs from a neighbour's, Brute Force works every pixel so; at Factor 1 Edge Highlight whitens the pixels whose band differs from the left or above. Overlay ghosts the other set and draws its centre's cross. Every expected pixel is `Fixtures/fractal/expected_fractal.json`, written by `tools/fractal_reference.py` before this code existed and printed in document 25 as FX-FRACTAL-001 to 042. Tolerance 2e-5.

## FX-FRACTAL-001 to 042 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FRACTAL-001 frame 0: The settings as they start: the Mandelbrot set from -3.15 to 1.65 across and -1.5 to 1.5 up, black inside, the bands of Lightness Gradient's first gradient (red) outside, oversampled 2 by 2 where the bands change; the cel itself is gone. | largest difference 2.9e-8 | yes |
| FX-FRACTAL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-002 frame 0: Brute Force on Solid Color at magnification 2, (-0.75, 0.1): every pixel worked 2 by 2, so two pixels whose centre and four neighbours are all outside, but which a filament of the set crosses, turn partly red; Edge Detect misses them. (On the settings as they start every pixel of this small frame is on a band change, so there the two methods agree.) | largest difference 0.0e0 | yes |
| FX-FRACTAL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-003 frame 0: Oversample Factor 1: one point a pixel, hard band edges. | largest difference 1.6e-8 | yes |
| FX-FRACTAL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-004 frame 0: Factor 1 with Edge Highlight: pixels where the band changes from the left or above are white. | largest difference 1.4e-8 | yes |
| FX-FRACTAL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-005 frame 0: Edge Highlight with factor 2: oversampling wins, the frame is FX-FRACTAL-001's. | largest difference 2.9e-8 | yes |
| FX-FRACTAL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-006 frame 0: Transparency: inside the set is clear. | largest difference 2.9e-8 | yes |
| FX-FRACTAL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-007 frame 0: Hue Wheel: the bands go round the wheel in 10 steps. | largest difference 1.8e-8 | yes |
| FX-FRACTAL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-008 frame 0: Black And White: the bands alternate. | largest difference 0.0e0 | yes |
| FX-FRACTAL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-009 frame 0: Solid Color: the set in red, outside clear. | largest difference 0.0e0 | yes |
| FX-FRACTAL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-010 frame 0: Solid Color with Transparency: the set clear, outside red. | largest difference 0.0e0 | yes |
| FX-FRACTAL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-011 frame 0: Hue 120: the gradients start green. | largest difference 2.9e-8 | yes |
| FX-FRACTAL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-012 frame 0: Cycle Steps 3, Cycle Offset 2: shorter gradients, started two on. | largest difference 2.6e-8 | yes |
| FX-FRACTAL-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-013 frame 0: Julia: the Julia set of c = -0.75 (the Mandelbrot centre), at the Julia view (0, 0). | largest difference 1.8e-8 | yes |
| FX-FRACTAL-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-014 frame 0: Julia Inverse. | largest difference 2.9e-8 | yes |
| FX-FRACTAL-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-015 frame 0: Mandelbrot Inverse, centred on 0 so the middle pixel's corner is the point 1 / 0. | largest difference 2.9e-8 | yes |
| FX-FRACTAL-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-016 frame 0: Mandelbrot Over Julia, the Julia centre (0.3, 0.2): z starts there. | largest difference 2.9e-8 | yes |
| FX-FRACTAL-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-017 frame 0: Mandelbrot Inverse Over Julia, the same Julia centre. | largest difference 2.9e-8 | yes |
| FX-FRACTAL-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-018 frame 0: Equation z^3 + c. | largest difference 2.9e-8 | yes |
| FX-FRACTAL-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-019 frame 0: Equation z^6 + c. | largest difference 2.9e-8 | yes |
| FX-FRACTAL-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-020 frame 0: Overlay on the Mandelbrot: the Julia set ghosted half way to white and the white cross with its black shadow at pixel (8, 5). | largest difference 2.9e-8 | yes |
| FX-FRACTAL-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-021 frame 0: Overlay on the Julia: the Mandelbrot ghosted. | largest difference 2.8e-8 | yes |
| FX-FRACTAL-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-022 frame 0: Magnification 2 at (-0.75, 0.1): four times closer, on the neck. | largest difference 2.9e-8 | yes |
| FX-FRACTAL-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-023 frame 0: Escape Limit 5: far more of the plane counts as inside. | largest difference 2.9e-8 | yes |
| FX-FRACTAL-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-024 frame 0: Magnification keyed from 0 at frame 0 to 4 at frame 4, linear: frame 2 is magnification 2. | largest difference 2.9e-8 | yes |
| FX-FRACTAL-024 frame 2: Magnification keyed from 0 at frame 0 to 4 at frame 4, linear: frame 2 is magnification 2. | largest difference 2.6e-8 | yes |
| FX-FRACTAL-024 frame 4: Magnification keyed from 0 at frame 0 to 4 at frame 4, linear: frame 2 is magnification 2. | largest difference 1.5e-8 | yes |
| FX-FRACTAL-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-025 frame 0: Hue keyed from 0 at frame 0 to 360 at frame 4: frame 4 is frame 0, frame 2 hue 180. | largest difference 2.9e-8 | yes |
| FX-FRACTAL-025 frame 2: Hue keyed from 0 at frame 0 to 360 at frame 4: frame 4 is frame 0, frame 2 hue 180. | largest difference 2.9e-8 | yes |
| FX-FRACTAL-025 frame 4: Hue keyed from 0 at frame 0 to 360 at frame 4: frame 4 is frame 0, frame 2 hue 180. | largest difference 2.9e-8 | yes |
| FX-FRACTAL-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-026 frame 0: FX-FRACTAL-003 moved three pixels right: the set moves with the layer. | largest difference 1.6e-8 | yes |
| FX-FRACTAL-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-027 frame 0: After a Motion Tile that grows the layer: the view is the drawing's own, so the frame is FX-FRACTAL-001's. | largest difference 2.9e-8 | yes |
| FX-FRACTAL-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-028 frame 0: A deep zoom, magnification 40 at (-0.743643887037151, 0.131825904205330) with Escape Limit 2000: a pixel is 2.7e-13 units, which double precision still separates. | largest difference 2.7e-8 | yes |
| FX-FRACTAL-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-029 frame 4: Magnification keyed from 0 at frame 0 to 40 at frame 4 past its end by an ease: held at 40, frame 4 is magnification 40. | largest difference 0.0e0 | yes |
| FX-FRACTAL-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-030 frame 0: Cycle Steps 3.7 and Escape Limit 100.9: their whole parts count, so the frame is Cycle Steps 3's with Escape Limit 100. | largest difference 2.5e-8 | yes |
| FX-FRACTAL-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-031 frame 0: Set Choice "burning_ship", not one of its six words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-031 frame 4: Set Choice "burning_ship", not one of its six words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-032 frame 0: Equation "z7". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-032 frame 4: Equation "z7". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-033 frame 0: Escape Limit 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-033 frame 4: Escape Limit 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-034 frame 0: Cycle Offset 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-034 frame 4: Cycle Offset 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-035 frame 0: Magnification 41, above 40. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-035 frame 4: Magnification 41, above 40. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-036 frame 0: Julia centre 11 across, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-036 frame 4: Julia centre 11 across, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-037 frame 0: Palette "rainbow". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-037 frame 4: Palette "rainbow". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-038 frame 0: Cycle Steps 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-038 frame 4: Cycle Steps 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-039 frame 0: Oversample Factor 9, above 8. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-039 frame 4: Oversample Factor 9, above 8. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-040 frame 0: Hue keyed to 36001 at frame 4, above 36000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-040 frame 4: Hue keyed to 36001 at frame 4, above 36000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-041 frame 0: Oversample Method "fast". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-041 frame 4: Oversample Method "fast". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-042 frame 0: Transparency "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-042 frame 4: Transparency "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fractal_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fractal_030.json is saved with its counts as written, 3.7 and 100.9, though only their whole parts count | {"cycle_offset":0,"cycle_steps":3.7,"edge_highlight":"off","equation":"z2","hue":0,"julia_center":[0,0],"julia_escape_limit":100,"julia_magnification":0,"mandelbrot_center":[-0.75,0],"mandelbrot_escape_limit":100.9,"mandelbrot_magnification":0,"overlay":"off","oversample_factor":2,"oversample_method":"edge_detect","palette":"lightness_gradient","set_choice":"mandelbrot","transparency":"off"} | yes |
| fx_fractal_031.json is refused in a sentence | Fractal's set choice is "mandelbrot", "mandelbrot_inverse", "mandelbrot_over_julia", "mandelbrot_inverse_over_julia", "julia" or "julia_inverse", and this is "burning_ship". | yes |
| fx_fractal_032.json is refused in a sentence | Fractal's equation is "z2", "z3", "z4", "z5" or "z6", and this is "z7". | yes |
| fx_fractal_033.json is refused in a sentence | Fractal's mandelbrot escape limit runs from 1 to 10000, and this is 0. | yes |
| fx_fractal_034.json is refused in a sentence | Fractal's cycle offset runs from 0 to 1000, and this is 1001. | yes |
| fx_fractal_035.json is refused in a sentence | Fractal's mandelbrot magnification runs from -10 to 40, and this is 41. | yes |
| fx_fractal_036.json is refused in a sentence | Fractal's julia center runs from -10 to 10, and this is 11. | yes |
| fx_fractal_037.json is refused in a sentence | Fractal's palette is "lightness_gradient", "hue_wheel", "black_and_white" or "solid_color", and this is "rainbow". | yes |
| fx_fractal_038.json is refused in a sentence | Fractal's cycle steps runs from 1 to 1000, and this is 0. | yes |
| fx_fractal_039.json is refused in a sentence | Fractal's oversample factor runs from 1 to 8, and this is 9. | yes |
| fx_fractal_041.json is refused in a sentence | Fractal's oversample method is "edge_detect" or "brute_force", and this is "fast". | yes |
| fx_fractal_042.json is refused in a sentence | Fractal's transparency is "off" or "on", and this is "yes". | yes |
| a file with a Fractal with no `oversample_factor` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Fractal with a factor in words is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Fractal whose centre is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| set choice "burning_ship" is refused with a sentence, and nothing changes | Fractal's set choice is "mandelbrot", "mandelbrot_inverse", "mandelbrot_over_julia", "mandelbrot_inverse_over_julia", "julia" or "julia_inverse", and this is "burning_ship". | yes |
| escape limit 0 is refused with a sentence, and nothing changes | Fractal's mandelbrot escape limit runs from 1 to 10000, and this is 0. | yes |
| oversample factor 9 is refused with a sentence, and nothing changes | Fractal's oversample factor runs from 1 to 8, and this is 9. | yes |
| magnification keyed to 41 is refused with a sentence, and nothing changes | Fractal's mandelbrot magnification runs from -10 to 40, and this is 41. | yes |
| Julia, hue 120, cycle steps 3, factor 1 is taken | taken | yes |
| magnification keyed from 0 to 4 is taken | taken | yes |
| Mandelbrot centre keyed from -0.75, 0 to -0.75, 0.1 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fractal_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fractal_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fractal_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fractal_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fractal_020.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fractal_027.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fractal_028.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fractal_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_fractal_031.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fractal_032.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fractal_033.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fractal_034.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fractal_035.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fractal_036.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fractal_037.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fractal_038.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fractal_039.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fractal_040.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fractal_041.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_fractal_042.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Fractal as it starts (the Mandelbrot set, Lightness Gradient, Edge Detect 2 by 2): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Fractal as it starts (the Mandelbrot set, Lightness Gradient, Edge Detect 2 by 2) on three layers, frame 0, Full | largest difference 1 of 255, 543 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal as it starts (the Mandelbrot set, Lightness Gradient, Edge Detect 2 by 2) on three layers, frame 100, Full | largest difference 1 of 255, 579 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal as it starts (the Mandelbrot set, Lightness Gradient, Edge Detect 2 by 2) on three layers, frame 239, Full | largest difference 1 of 255, 621 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal as it starts (the Mandelbrot set, Lightness Gradient, Edge Detect 2 by 2) on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal as it starts (the Mandelbrot set, Lightness Gradient, Edge Detect 2 by 2) on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal as it starts (the Mandelbrot set, Lightness Gradient, Edge Detect 2 by 2) on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Julia, Hue Wheel, hue 200: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Fractal Julia, Hue Wheel, hue 200 on three layers, frame 0, Full | largest difference 1 of 255, 24 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Julia, Hue Wheel, hue 200 on three layers, frame 100, Full | largest difference 1 of 255, 16 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Julia, Hue Wheel, hue 200 on three layers, frame 239, Full | largest difference 1 of 255, 21 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Julia, Hue Wheel, hue 200 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Julia, Hue Wheel, hue 200 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Julia, Hue Wheel, hue 200 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Brute Force 4 by 4, Black And White: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073585 pixels changed | yes |
| the reference shot, Fractal Brute Force 4 by 4, Black And White on three layers, frame 0, Full | largest difference 1 of 255, 1071 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Brute Force 4 by 4, Black And White on three layers, frame 100, Full | largest difference 1 of 255, 1209 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Brute Force 4 by 4, Black And White on three layers, frame 239, Full | largest difference 1 of 255, 1260 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Brute Force 4 by 4, Black And White on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Brute Force 4 by 4, Black And White on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Brute Force 4 by 4, Black And White on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal factor 1 with Edge Highlight, Transparency: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Fractal factor 1 with Edge Highlight, Transparency on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal factor 1 with Edge Highlight, Transparency on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal factor 1 with Edge Highlight, Transparency on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal factor 1 with Edge Highlight, Transparency on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal factor 1 with Edge Highlight, Transparency on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal factor 1 with Edge Highlight, Transparency on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Overlay, Mandelbrot Over Julia from 0.3, 0.2: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073596 pixels changed | yes |
| the reference shot, Fractal Overlay, Mandelbrot Over Julia from 0.3, 0.2 on three layers, frame 0, Full | largest difference 1 of 255, 312 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Overlay, Mandelbrot Over Julia from 0.3, 0.2 on three layers, frame 100, Full | largest difference 1 of 255, 169 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Overlay, Mandelbrot Over Julia from 0.3, 0.2 on three layers, frame 239, Full | largest difference 1 of 255, 268 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Overlay, Mandelbrot Over Julia from 0.3, 0.2 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Overlay, Mandelbrot Over Julia from 0.3, 0.2 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Overlay, Mandelbrot Over Julia from 0.3, 0.2 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal z^4, Julia Inverse, Solid Color: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Fractal z^4, Julia Inverse, Solid Color on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal z^4, Julia Inverse, Solid Color on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal z^4, Julia Inverse, Solid Color on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal z^4, Julia Inverse, Solid Color on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal z^4, Julia Inverse, Solid Color on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal z^4, Julia Inverse, Solid Color on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal magnification 30 at the seahorse valley, Escape Limit 500: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Fractal magnification 30 at the seahorse valley, Escape Limit 500 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal magnification 30 at the seahorse valley, Escape Limit 500 on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal magnification 30 at the seahorse valley, Escape Limit 500 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal magnification 30 at the seahorse valley, Escape Limit 500 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal magnification 30 at the seahorse valley, Escape Limit 500 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal magnification 30 at the seahorse valley, Escape Limit 500 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Mandelbrot Inverse at 0, 0, cycle steps 3, offset 2: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Fractal Mandelbrot Inverse at 0, 0, cycle steps 3, offset 2 on three layers, frame 0, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Mandelbrot Inverse at 0, 0, cycle steps 3, offset 2 on three layers, frame 100, Full | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Mandelbrot Inverse at 0, 0, cycle steps 3, offset 2 on three layers, frame 239, Full | largest difference 1 of 255, 13 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Mandelbrot Inverse at 0, 0, cycle steps 3, offset 2 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Mandelbrot Inverse at 0, 0, cycle steps 3, offset 2 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Mandelbrot Inverse at 0, 0, cycle steps 3, offset 2 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Escape Limit 10000 on three layers (1.9e11 steps at Full, past the card's 1e10), frame 100, Full: drawn on the processor | largest difference 0 of 255; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Escape Limit 10000 on three layers (1.9e11 steps at Full, past the card's 1e10), frame 100, Draft: drawn on the processor | largest difference 0 of 255; 0 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-416 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: the Mandelbrot set, black, in red and orange bands, in place of the street; draws cleanly | [], 129600 pixels changed | yes |
| 3_julia_hue_wheel.png, the Julia set of -0.75, its outside going round the colour wheel; draws cleanly | [], 129600 pixels changed | yes |
| 4_seahorse.png, deep in the seahorse valley, magnification 12, Escape Limit 1000; draws cleanly | [], 129600 pixels changed | yes |
| 5_solid_clear.png, the set alone in blue, the rest clear; draws cleanly | [], 129600 pixels changed | yes |
| 6_overlay.png, Edge Highlight on its bands, the Julia set ghosted and the white cross at its centre; draws cleanly | [], 129600 pixels changed | yes |

## Result

278 of 278 checks pass.
