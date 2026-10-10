# D-446: Curl Noise

B-326, after After Effects' Curl Noise (26.3 beta): a four-octave smooth value noise Size pixels across, drifting Speed towards Direction and changing with Evolution and Turbulence Speed; its slope turned a right angle (plus Swirl times the noise) is the flow, which neither gathers nor spreads at Swirl 0, scaled across or down by Vertical Bias. From each pixel the flow is followed both ways, Sample Count steps over Sample Radius pixels, streaking a seed noise (blocky by Edge Definition) into grey flow lines, softened back towards the noise by Flow Softness and faded where the noise is low by Flow Falloff. View shows the lines, the noise, or the flow as colours; Contrast and Brightness follow, held in 0 and 1 unless Clip HDR Results is off in a Float composition; Channel puts the grey into the colour, one channel or the covering. Source This Layer and Other Layer are refused in a sentence, not built. The rule is this program's own; Adobe publishes none. Every expected pixel is `Fixtures/curl_noise/expected_curl_noise.json`, written by `tools/curl_noise_reference.py` before this code existed and printed in document 25 as FX-CURL-001 to 044. Tolerance 2e-5.

## FX-CURL-001 to 044 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CURL-001 frame 0: The settings as they start: size 100, so across the 16 by 10 card the noise is one soft cloud, swirling slowly; every shown pixel is a grey, the empty pixels stay empty and the soft edge keeps its half covering. | largest difference 3.0e-8 | yes |
| FX-CURL-001 frame 4: The settings as they start: size 100, so across the 16 by 10 card the noise is one soft cloud, swirling slowly; every shown pixel is a grey, the empty pixels stay empty and the soft edge keeps its half covering. | largest difference 3.0e-8 | yes |
| FX-CURL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-002 frame 0: Size 6, sample radius 4, 6 samples: grey flow lines through cells of 6 pixels. | largest difference 3.0e-8 | yes |
| FX-CURL-002 frame 2: Size 6, sample radius 4, 6 samples: grey flow lines through cells of 6 pixels. | largest difference 3.0e-8 | yes |
| FX-CURL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-003 frame 0: View Input Noise: the smooth noise the flow follows, without its lines. | largest difference 3.0e-8 | yes |
| FX-CURL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-004 frame 0: View Curl Generation: red is the flow across, green the flow down, blue the noise. | largest difference 3.5e-8 | yes |
| FX-CURL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-005 frame 0: Sample Radius 0: no lines, so the frame is FX-CURL-003's Input Noise. | largest difference 3.0e-8 | yes |
| FX-CURL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-006 frame 0: Flow Softness 100: the lines softened away, FX-CURL-003's Input Noise again. | largest difference 3.0e-8 | yes |
| FX-CURL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-007 frame 0: Swirl 0, Curl Generation: the flow is the noise's curl, crossing its slope at a right angle. | largest difference 3.3e-8 | yes |
| FX-CURL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-008 frame 0: Swirl 180, Curl Generation: the flow turns with the noise, unlike FX-CURL-007's. | largest difference 5.2e-8 | yes |
| FX-CURL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-009 frame 0: Vertical Bias 100, Curl Generation: the flow only runs up and down, so red is the middle grey throughout. | largest difference 3.0e-8 | yes |
| FX-CURL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-010 frame 0: Vertical Bias 0, Curl Generation: the flow only runs across, so green is the middle grey throughout. | largest difference 3.0e-8 | yes |
| FX-CURL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-011 frame 0: Speed 20, Direction 90, Turbulence Speed 0, Input Noise: the noise drifts 2 pixels right a frame, so frame 1 is frame 0 moved 2 right. | largest difference 3.0e-8 | yes |
| FX-CURL-011 frame 1: Speed 20, Direction 90, Turbulence Speed 0, Input Noise: the noise drifts 2 pixels right a frame, so frame 1 is frame 0 moved 2 right. | largest difference 3.0e-8 | yes |
| FX-CURL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-012 frame 0: Offset 3 right, still, Input Noise: the noise moved 3 pixels right. | largest difference 3.0e-8 | yes |
| FX-CURL-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-013 frame 0: Turbulence Speed 100, Speed 0: the noise changes from frame to frame in place. | largest difference 3.0e-8 | yes |
| FX-CURL-013 frame 2: Turbulence Speed 100, Speed 0: the noise changes from frame to frame in place. | largest difference 3.0e-8 | yes |
| FX-CURL-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-014 frame 0: Evolution 90, still: a different noise from FX-CURL-002's. | largest difference 3.0e-8 | yes |
| FX-CURL-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-015 frame 0: Density 50: the field's cells half the size, so it is size 3 at density 0, FX-CURL-016. | largest difference 3.0e-8 | yes |
| FX-CURL-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-016 frame 0: Size 3, still. | largest difference 3.0e-8 | yes |
| FX-CURL-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-017 frame 0: Smoothness 0, still, Input Noise: the fine octaves stronger. | largest difference 3.0e-8 | yes |
| FX-CURL-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-018 frame 0: Smoothness 100, still, Input Noise: the fine octaves weaker. | largest difference 3.0e-8 | yes |
| FX-CURL-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-019 frame 0: Edge Definition 100, still: the lines streak blocky seeds. | largest difference 3.0e-8 | yes |
| FX-CURL-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-020 frame 0: Edge Definition 0, still: the lines streak smooth seeds. | largest difference 3.0e-8 | yes |
| FX-CURL-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-021 frame 0: Flow Falloff 100, still: the lines fade where the noise is low. | largest difference 3.0e-8 | yes |
| FX-CURL-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-022 frame 0: Contrast 300, Brightness 10, still: harder greys, held in 0 and 1. | largest difference 3.0e-8 | yes |
| FX-CURL-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-023 frame 0: Contrast 300, Brightness 10, Clip HDR Results off, in a composition that does not work in Float: still held at 1, so the frame is FX-CURL-022's. | largest difference 3.0e-8 | yes |
| FX-CURL-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-024 frame 0: Contrast 300, Brightness 10, Clip HDR Results off, in a Float composition: past white is kept. | largest difference 1.2e-7 | yes |
| FX-CURL-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-025 frame 0: Channel Red, still: only red is replaced; green and blue are the drawing's. | largest difference 1.3e-7 | yes |
| FX-CURL-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-026 frame 0: Channel Alpha, still: the covering times the grey, the colour kept. | largest difference 1.4e-7 | yes |
| FX-CURL-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-027 frame 0: Sample Count 3.9, still: its whole part counts, so this is sample count 3, FX-CURL-028. | largest difference 3.0e-8 | yes |
| FX-CURL-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-028 frame 0: Sample Count 3, still. | largest difference 3.0e-8 | yes |
| FX-CURL-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-029 frame 0: Sample Radius keyed from 0 at frame 0 to 8 at frame 4, still: frame 0 is the Input Noise, frame 4 long lines. | largest difference 3.0e-8 | yes |
| FX-CURL-029 frame 2: Sample Radius keyed from 0 at frame 0 to 8 at frame 4, still: frame 0 is the Input Noise, frame 4 long lines. | largest difference 3.0e-8 | yes |
| FX-CURL-029 frame 4: Sample Radius keyed from 0 at frame 0 to 8 at frame 4, still: frame 0 is the Input Noise, frame 4 long lines. | largest difference 3.1e-8 | yes |
| FX-CURL-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-030 frame 0: FX-CURL-002 moved three pixels right: the noise is the drawing's own, so it moves with it. | largest difference 1.5e-8 | yes |
| FX-CURL-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-031 frame 0: After a Motion Tile that grows the layer: the noise is worked in the drawing's own pixels, so the frame is FX-CURL-002's. | largest difference 3.0e-8 | yes |
| FX-CURL-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-032 frame 0: Swirl -200, Speed 35, Direction 200, Evolution 45, Turbulence Speed 60, Density -30, Smoothness 20, Vertical Bias 70, 9 samples, radius 6.5, Flow Softness 10, Edge Definition 30, Falloff 40, Contrast 150, Brightness -5, Channel Green: the controls together. | largest difference 1.9e-7 | yes |
| FX-CURL-032 frame 3: Swirl -200, Speed 35, Direction 200, Evolution 45, Turbulence Speed 60, Density -30, Smoothness 20, Vertical Bias 70, 9 samples, radius 6.5, Flow Softness 10, Edge Definition 30, Falloff 40, Contrast 150, Brightness -5, Channel Green: the controls together. | largest difference 1.9e-7 | yes |
| FX-CURL-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-033 frame 0: Source This Layer, not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-033 frame 4: Source This Layer, not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-034 frame 0: Source Other Layer, not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-034 frame 4: Source Other Layer, not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-035 frame 0: Source "noise", not one of its words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-035 frame 4: Source "noise", not one of its words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-036 frame 0: Size 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-036 frame 4: Size 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-037 frame 0: Speed 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-037 frame 4: Speed 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-038 frame 0: Sample Count 2, below 3. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-038 frame 4: Sample Count 2, below 3. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-039 frame 0: Sample Radius 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-039 frame 4: Sample Radius 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-040 frame 0: Swirl 400, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-040 frame 4: Swirl 400, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-041 frame 0: View "lines", not one of its words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-041 frame 4: View "lines", not one of its words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-042 frame 0: Channel "Red", in capitals, kept as written and not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-042 frame 4: Channel "Red", in capitals, kept as written and not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-043 frame 0: Clip HDR Results "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-043 frame 4: Clip HDR Results "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-043: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-044 frame 0: Density keyed to 150 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-044 frame 4: Density keyed to 150 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-044: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_curl_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_032.json is saved with its words and numbers as written, and no frame or depth | {"brightness":-5,"channel":"green","clip_hdr_results":"on","contrast":150,"density":-30,"direction":200,"edge_definition":30,"evolution":45,"flow_falloff":40,"flow_softness":10,"offset":[0,0],"sample_count":9,"sample_radius":6.5,"size":6,"smoothness":20,"source":"internal","speed":35,"swirl":-200,"turbulence_speed":60,"vertical_bias":70,"view":"final_render"} | yes |
| fx_curl_029.json is saved with Sample Radius's keys kept | {"base":0,"keyframes":[{"frame":0,"interp":"linear","value":0},{"frame":4,"interp":"linear","value":8}]} | yes |
| fx_curl_033.json is refused in a sentence | Curl Noise's This Layer source is not built yet: it makes its own noise only, so set Source to Internal. | yes |
| fx_curl_034.json is refused in a sentence | Curl Noise's Other Layer source is not built yet: it makes its own noise only, so set Source to Internal. | yes |
| fx_curl_035.json is refused in a sentence | Curl Noise's source is "internal", "this_layer" or "other_layer", and this is "noise". | yes |
| fx_curl_036.json is refused in a sentence | Curl Noise's size runs from 1 to 1000, and this is 0.5. | yes |
| fx_curl_037.json is refused in a sentence | Curl Noise's speed runs from 0 to 100, and this is 101. | yes |
| fx_curl_038.json is refused in a sentence | Curl Noise's sample count runs from 3 to 24, and this is 2. | yes |
| fx_curl_039.json is refused in a sentence | Curl Noise's sample radius runs from 0 to 200, and this is 201. | yes |
| fx_curl_040.json is refused in a sentence | Curl Noise's swirl runs from -360 to 360, and this is 400. | yes |
| fx_curl_041.json is refused in a sentence | Curl Noise's view is one of final_render, input_noise, curl_generation, and this is "lines". | yes |
| fx_curl_042.json is refused in a sentence | Curl Noise's channel is one of rgb, red, green, blue, alpha, and this is "Red". | yes |
| fx_curl_043.json is refused in a sentence | Curl Noise's clip HDR results is "on" or "off", and this is "yes". | yes |
| a file with a Curl Noise with no `view` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Curl Noise whose channel is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Curl Noise whose offset has one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| size 0.5 is refused with a sentence, and nothing changes | Curl Noise's size runs from 1 to 1000, and this is 0.5. | yes |
| sample count 25 is refused with a sentence, and nothing changes | Curl Noise's sample count runs from 3 to 24, and this is 25. | yes |
| source This Layer is refused with a sentence, and nothing changes | Curl Noise's This Layer source is not built yet: it makes its own noise only, so set Source to Internal. | yes |
| view "lines" is refused with a sentence, and nothing changes | Curl Noise's view is one of final_render, input_noise, curl_generation, and this is "lines". | yes |
| channel "Red" is refused with a sentence, and nothing changes | Curl Noise's channel is one of rgb, red, green, blue, alpha, and this is "Red". | yes |
| density keyed to 150 is refused with a sentence, and nothing changes | Curl Noise's density runs from -100 to 100, and this is 150. | yes |
| speed 35, direction 200, size 6, offset 2 and -1, evolution 45, turbulence 60, swirl -200, density -30, smoothness 20, bias 70, 9 samples, radius 6.5, softness 10, edges 30, falloff 40, contrast 150, brightness -5, Curl Generation, clip off, green is taken | taken | yes |
| sample radius keyed from 0 to 8 is taken | taken | yes |
| offset keyed from 0, 0 to 5, 3 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_curl_001.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_curl_002.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_curl_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_curl_026.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_curl_031.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_curl_032.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_curl_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_024.json, frames 0 to 4 at Full and Draft: refused by the card, drawn by the processor | largest difference 0 of 255; refused: true | yes |
| fx_curl_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_curl_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_curl_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_curl_035.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_curl_036.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_curl_037.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_curl_038.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_curl_039.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_curl_040.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_curl_041.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_curl_042.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_curl_043.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_curl_044.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Curl Noise as added (size 100, radius 30, 12 samples): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072913 pixels changed | yes |
| the reference shot, Curl Noise as added (size 100, radius 30, 12 samples) on three layers, frame 0, Full | largest difference 1 of 255, 1229 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise as added (size 100, radius 30, 12 samples) on three layers, frame 100, Full | largest difference 1 of 255, 1253 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise as added (size 100, radius 30, 12 samples) on three layers, frame 239, Full | largest difference 1 of 255, 1224 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise as added (size 100, radius 30, 12 samples) on three layers, frame 0, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise as added (size 100, radius 30, 12 samples) on three layers, frame 100, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise as added (size 100, radius 30, 12 samples) on three layers, frame 239, Draft | largest difference 1 of 255, 78 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise size 20, Curl Generation, swirl 120, vertical bias 70: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Curl Noise size 20, Curl Generation, swirl 120, vertical bias 70 on three layers, frame 0, Full | largest difference 1 of 255, 1162 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise size 20, Curl Generation, swirl 120, vertical bias 70 on three layers, frame 100, Full | largest difference 1 of 255, 1181 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise size 20, Curl Generation, swirl 120, vertical bias 70 on three layers, frame 239, Full | largest difference 1 of 255, 1229 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise size 20, Curl Generation, swirl 120, vertical bias 70 on three layers, frame 0, Draft | largest difference 1 of 255, 89 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise size 20, Curl Generation, swirl 120, vertical bias 70 on three layers, frame 100, Draft | largest difference 1 of 255, 87 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise size 20, Curl Generation, swirl 120, vertical bias 70 on three layers, frame 239, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise size 40, radius 12, 8 samples, edges 100, falloff 60, contrast 200, brightness -10, channel Alpha: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2071725 pixels changed | yes |
| the reference shot, Curl Noise size 40, radius 12, 8 samples, edges 100, falloff 60, contrast 200, brightness -10, channel Alpha on three layers, frame 0, Full | largest difference 1 of 255, 1591 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise size 40, radius 12, 8 samples, edges 100, falloff 60, contrast 200, brightness -10, channel Alpha on three layers, frame 100, Full | largest difference 1 of 255, 1624 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise size 40, radius 12, 8 samples, edges 100, falloff 60, contrast 200, brightness -10, channel Alpha on three layers, frame 239, Full | largest difference 1 of 255, 1513 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise size 40, radius 12, 8 samples, edges 100, falloff 60, contrast 200, brightness -10, channel Alpha on three layers, frame 0, Draft | largest difference 1 of 255, 492 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise size 40, radius 12, 8 samples, edges 100, falloff 60, contrast 200, brightness -10, channel Alpha on three layers, frame 100, Draft | largest difference 1 of 255, 376 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise size 40, radius 12, 8 samples, edges 100, falloff 60, contrast 200, brightness -10, channel Alpha on three layers, frame 239, Draft | largest difference 1 of 255, 484 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise radius keyed 2 to 40 and evolution keyed 0 to 720, direction 135, speed 40, smoothness 10, channel Blue: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2067962 pixels changed | yes |
| the reference shot, Curl Noise radius keyed 2 to 40 and evolution keyed 0 to 720, direction 135, speed 40, smoothness 10, channel Blue on three layers, frame 0, Full | largest difference 1 of 255, 2211 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise radius keyed 2 to 40 and evolution keyed 0 to 720, direction 135, speed 40, smoothness 10, channel Blue on three layers, frame 100, Full | largest difference 1 of 255, 2348 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise radius keyed 2 to 40 and evolution keyed 0 to 720, direction 135, speed 40, smoothness 10, channel Blue on three layers, frame 239, Full | largest difference 1 of 255, 2108 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise radius keyed 2 to 40 and evolution keyed 0 to 720, direction 135, speed 40, smoothness 10, channel Blue on three layers, frame 0, Draft | largest difference 1 of 255, 86 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise radius keyed 2 to 40 and evolution keyed 0 to 720, direction 135, speed 40, smoothness 10, channel Blue on three layers, frame 100, Draft | largest difference 1 of 255, 78 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise radius keyed 2 to 40 and evolution keyed 0 to 720, direction 135, speed 40, smoothness 10, channel Blue on three layers, frame 239, Draft | largest difference 1 of 255, 88 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-446 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, frame 0, as added: soft swirling grey flow over the whole street; draws cleanly | [], 129600 pixels changed of 129600, the largest by 255 of 255 | yes |
| 3_fine_lines.png, frame 0, size 30, radius 20, 16 samples, softness 0, edges 100: fine streaked flow lines; draws cleanly | [], 129600 pixels changed of 129600, the largest by 255 of 255 | yes |
| 4_curl_generation.png, frame 0, Curl Generation, size 60: the flow as red and green, the noise as blue; draws cleanly | [], 129600 pixels changed of 129600, the largest by 255 of 255 | yes |
| 5_alpha.png, frame 0, Channel Alpha, size 40, contrast 250: the street shows through the light parts, holes in the dark; draws cleanly | [], 122534 pixels changed of 129600, the largest by 255 of 255 | yes |
| 6_frame_24.png, frame 24, as added at frame 24: the flow has drifted up and changed; draws cleanly | [], 129600 pixels changed of 129600, the largest by 255 of 255 | yes |

## Result

261 of 261 checks pass.
