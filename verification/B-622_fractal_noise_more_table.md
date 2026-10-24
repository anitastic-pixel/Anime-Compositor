# B-622: Fractal Noise's Rotation, Sub Settings and Overflow (D-743)

`core.fractal_noise` now keeps in its file, and shows in the panel, the settings its engine drew since D-647 but always at their start: Overflow, Rotation, Perspective Offset, Sub Influence, Sub Scaling, Sub Rotation, Sub Offset and Center Subscale. A file without them reads Allow HDR Results, 0, off, 50, 50, 0, (0, 0), off, which draws every Fractal Noise before D-743 as before (FX-FRACTAL-001 to 038 rerun unchanged in `b71_fractal_noise`). One added from now on starts at After Effects' Sub Influence 70 and Sub Scaling 56 (the app's test `b622_fractal_noise_sub_settings_typed_in_the_panel`). No fixture is added or changed: Turbulent Noise is Fractal Noise's engine (D-445), so each of Turbulent Noise's reference cases FX-TURBNOISE-033 to 058 (`tools/turbulent_more_reference.py`) is drawn below as a Fractal Noise with the same settings and held to its numbers, tolerance 2e-5. Uniform Scaling (FX-TURBNOISE-038, 059) and Turbulent Noise's further blending modes (049 to 053, 060) are not Fractal Noise's.

## Turbulent Noise's reference cases drawn as Fractal Noise

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TURBNOISE-033 as Fractal Noise, frame 0: Size 4, contrast 300, Overflow clip: in an 8-bit composition the same as FX-TURBNOISE-009, allow_hdr holding to 0..1 there too. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-033 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | [] and [] "" | yes |
| FX-TURBNOISE-034 as Fractal Noise, frame 0: Size 4, contrast 300, Overflow soft_clamp: the greys eased toward black and white, none reaching them. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-034 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | [] and [] "" | yes |
| FX-TURBNOISE-035 as Fractal Noise, frame 0: Size 4, contrast 300, Overflow wrap_back: the greys past white and black fold back, light bands inside the dark. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-035 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | [] and [] "" | yes |
| FX-TURBNOISE-036 as Fractal Noise, frame 0: Size 4, Rotation 90: FX-TURBNOISE-002's clouds turned a quarter clockwise round the card's middle. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-036 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | [] and [] "" | yes |
| FX-TURBNOISE-037 as Fractal Noise, frame 0: Size 4, Rotation keyed from 0 at frame 0 to 90 at frame 4, linear: frame 0 is FX-TURBNOISE-002, frame 4 FX-TURBNOISE-036. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-037 as Fractal Noise, frame 2: Size 4, Rotation keyed from 0 at frame 0 to 90 at frame 4, linear: frame 0 is FX-TURBNOISE-002, frame 4 FX-TURBNOISE-036. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-037 as Fractal Noise, frame 4: Size 4, Rotation keyed from 0 at frame 0 to 90 at frame 4, linear: frame 0 is FX-TURBNOISE-002, frame 4 FX-TURBNOISE-036. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-037 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | [] and [] "" | yes |
| FX-TURBNOISE-039 as Fractal Noise, frame 0: Size 4, Offset (6, -4), Perspective Offset on: the coarse clouds slide 6 right and 4 up, each finer octave half as far as the last. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-039 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | [] and [] "" | yes |
| FX-TURBNOISE-040 as Fractal Noise, frame 0: Size 4, Perspective Offset on with no offset: nothing slides, so it is FX-TURBNOISE-002 within the tolerance. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-040 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | [] and [] "" | yes |
| FX-TURBNOISE-041 as Fractal Noise, frame 0: Size 4, Sub Influence 100: every octave as strong as the first, busier fine detail. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-041 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | [] and [] "" | yes |
| FX-TURBNOISE-042 as Fractal Noise, frame 0: Size 4, Sub Influence 0: only the first octave counts, so it is FX-TURBNOISE-007 (complexity 1) within the tolerance. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-042 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | [] and [] "" | yes |
| FX-TURBNOISE-043 as Fractal Noise, frame 0: Size 4, Sub Scaling 70: each octave only a little finer than the last, softer clouds. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-043 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | [] and [] "" | yes |
| FX-TURBNOISE-044 as Fractal Noise, frame 0: Size 4, Sub Rotation 45: each finer octave turned an eighth more. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-044 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | [] and [] "" | yes |
| FX-TURBNOISE-045 as Fractal Noise, frame 0: Size 4, Sub Offset (5, 3): each finer octave slid further. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-045 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | [] and [] "" | yes |
| FX-TURBNOISE-046 as Fractal Noise, frame 0: Size 4, Center Subscale on: the octaves grow finer round the card's middle, not its corner. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-046 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | [] and [] "" | yes |
| FX-TURBNOISE-047 as Fractal Noise, frame 0: Size 4, dynamic on spline, Rotation 30, Offset (3, 2) with Perspective Offset, Sub Influence 70, Sub Scaling 60, Sub Rotation 20, Sub Offset (1, -2), Center Subscale on: everything together. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-047 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | [] and [] "" | yes |
| FX-TURBNOISE-048 as Fractal Noise, frame 0: Size 4, cloudy, Sub Influence 80: cloudy's 0.35 a step scaled to 0.56. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-048 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | [] and [] "" | yes |
| FX-TURBNOISE-054 as Fractal Noise, frame 0: FX-TURBNOISE-036 moved three pixels right: turned round the drawing's own middle, so the turned clouds move with it. | largest difference 1.5e-8 | yes |
| FX-TURBNOISE-054 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | [] and [] "" | yes |
| FX-TURBNOISE-055 as Fractal Noise, frame 0: FX-TURBNOISE-002 with every D-647 setting written as added: FX-TURBNOISE-002 to the bit. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-055 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | [] and [] "" | yes |
| FX-TURBNOISE-056 as Fractal Noise, frame 0: Overflow "hdr", not one of the four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-056 as Fractal Noise, frame 4: Overflow "hdr", not one of the four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-056 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] "Fractal Noise's overflow is one of \"allow_hdr\", \"clip\", \"soft_clamp\", \"wrap_back\", and this is \"hdr\"." | yes |
| FX-TURBNOISE-057 as Fractal Noise, frame 0: Sub Scaling 5, below 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-057 as Fractal Noise, frame 4: Sub Scaling 5, below 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-057 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] "Fractal Noise's sub scaling runs from 10 to 100, and this is 5." | yes |
| FX-TURBNOISE-058 as Fractal Noise, frame 0: Sub Influence 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-058 as Fractal Noise, frame 4: Sub Influence 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-058 as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] "Fractal Noise's sub influence runs from 0 to 100, and this is 101." | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fractal_001.json, from before D-743, saves none of the new settings | {"blend":"normal","brightness":0,"complexity":4,"contrast":100,"dark_color":"#000000","evolution":0,"light_color":"#ffffff","opacity":100,"seed":0,"size":100,"speed":0} | yes |
| and reads them as before D-743: Allow HDR Results, Rotation 0, Sub Influence 50, Sub Scaling 50, Sub Rotation 0, Sub Offset (0, 0), Center Subscale off | NoiseMore { overflow: "allow_hdr", rotation: 0.0, uniform_scaling: "off", perspective_offset: "off", sub_influence: 50.0, sub_scaling: 50.0, sub_rotation: 0.0, sub_offset: [0.0, 0.0], center_subscale: "off" } | yes |
| fx_turbnoise_037.json as Fractal Noise opened and saved holds what it held, keys included | the same | yes |
| fx_turbnoise_047.json as Fractal Noise opened and saved holds what it held, keys included | the same | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_turbnoise_036.json as Fractal Noise, frames 0 to 4 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_037.json as Fractal Noise, frames 0 to 4 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_041.json as Fractal Noise, frames 0 to 4 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_047.json as Fractal Noise, frames 0 to 4 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_048.json as Fractal Noise, frames 0 to 4 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |

## Pictures: the street, in `verification/B-622 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_older_project_50_50.png, a project from before D-743: Sub Influence 50, Sub Scaling 50, as it always drew; unlike every picture before | its own: true | yes |
| 2_new_70_56.png, a Fractal Noise added from now on: After Effects' Sub Influence 70 and Sub Scaling 56, more fine detail; unlike every picture before | its own: true | yes |
| 3_sub_77_60.png, Sub Influence 77.5, Sub Scaling 59.7, as in Cinecom's tutorial: busier still; unlike every picture before | its own: true | yes |
| 4_rotation_36.png, Rotation 36: the clouds turned round the middle; unlike every picture before | its own: true | yes |

## Result

58 of 58 checks pass.
