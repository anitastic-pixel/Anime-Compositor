# D-646: Curl Noise, Rotation, Scale and Perspective Offset

B-525: `core.curl_noise` takes After Effects' Transform controls. With c the drawing's middle, m = 100 / Scale and theta = Rotation, octave o of the made noise is read at c + m R(-theta) (p - w_o (Offset + drift frame) - c), w_o = 0.5^o with Perspective Offset on, else 1; its slope is brought back by m R(theta) before Swirl and Vertical Bias, and the seed is read at octave 0's point. With Source This Layer or Other Layer the picture is read unturned, so Rotation and Scale move only the seed. Rotation 0, Scale 100 and Perspective Offset off draw every file before D-646 as before. The rule is this program's own, after Fractal Noise's same controls; Adobe publishes none. Every expected pixel is `Fixtures/curl_noise/expected_curl_transform.json`, written by `tools/curl_transform_reference.py` before this code existed and printed in document 25 as FX-CURL-061 to 077. Tolerance 2e-5.

## FX-CURL-061 to 077 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CURL-061 frame 0: Rotation 90, still, Input Noise: the noise turned a quarter clockwise round the card's middle. | largest difference 3.0e-8 | yes |
| FX-CURL-061: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-062 frame 0: Rotation 90, still, Swirl 0, Curl Generation: the flow turned with the noise, a quarter clockwise. | largest difference 4.1e-8 | yes |
| FX-CURL-062: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-063 frame 0: Scale 200, still, Input Noise: the noise twice as big round the card's middle, so it is Size 12 with Offset (-8, -5), FX-CURL-064. | largest difference 3.0e-8 | yes |
| FX-CURL-063: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-064 frame 0: Size 12, Offset (-8, -5), still, Input Noise. | largest difference 3.0e-8 | yes |
| FX-CURL-064: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-065 frame 0: Scale 50, still: flow lines through cells half the size. | largest difference 3.0e-8 | yes |
| FX-CURL-065: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-066 frame 0: Perspective Offset on, Speed 20, Direction 90, Turbulence Speed 0, Input Noise: at frame 0 nothing has slid, so it is FX-CURL-011's frame 0; at frame 1 the finer octaves have slid less than the coarse one. | largest difference 3.0e-8 | yes |
| FX-CURL-066 frame 1: Perspective Offset on, Speed 20, Direction 90, Turbulence Speed 0, Input Noise: at frame 0 nothing has slid, so it is FX-CURL-011's frame 0; at frame 1 the finer octaves have slid less than the coarse one. | largest difference 3.0e-8 | yes |
| FX-CURL-066: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-067 frame 0: Rotation 45, Scale 150, Perspective Offset on, Offset (3, 2), Speed 20, Direction 90: the Transform together, flow lines. | largest difference 3.0e-8 | yes |
| FX-CURL-067 frame 2: Rotation 45, Scale 150, Perspective Offset on, Offset (3, 2), Speed 20, Direction 90: the Transform together, flow lines. | largest difference 3.0e-8 | yes |
| FX-CURL-067: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-068 frame 0: Source This Layer, Rotation 90, Scale 200, Perspective Offset on, still, Input Noise: the picture is read unturned, so it is FX-CURL-045. | largest difference 8.5e-8 | yes |
| FX-CURL-068: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-069 frame 0: Source This Layer, Rotation 90, still: only the seed is turned, so the lines differ from Rotation 0's. | largest difference 1.0e-7 | yes |
| FX-CURL-069: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-070 frame 0: Rotation keyed from 0 at frame 0 to 90 at frame 4, still, Input Noise: frame 0 is the unturned noise, frame 4 FX-CURL-061. | largest difference 3.0e-8 | yes |
| FX-CURL-070 frame 4: Rotation keyed from 0 at frame 0 to 90 at frame 4, still, Input Noise: frame 0 is the unturned noise, frame 4 FX-CURL-061. | largest difference 3.0e-8 | yes |
| FX-CURL-070: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-071 frame 0: Rotation 30, still, after a Motion Tile that grows the layer: turned round the drawing's middle, not the grown buffer's, so it is FX-CURL-072. | largest difference 3.0e-8 | yes |
| FX-CURL-071: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-072 frame 0: Rotation 30, still. | largest difference 3.0e-8 | yes |
| FX-CURL-072: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-073 frame 0: Rotation 360, still: a whole turn, the noise as unturned (within the tolerance). | largest difference 3.0e-8 | yes |
| FX-CURL-073: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-074 frame 0: Scale 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-074 frame 4: Scale 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-074: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-075 frame 0: Rotation 200000, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-075 frame 4: Rotation 200000, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-075: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-076 frame 0: Perspective Offset "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-076 frame 4: Perspective Offset "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-076: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-077 frame 0: Scale keyed to 20000 at frame 4, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-077 frame 4: Scale keyed to 20000 at frame 4, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-077: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_curl_061.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_062.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_063.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_064.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_065.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_066.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_067.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_068.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_069.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_070.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_071.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_072.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_073.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_074.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_075.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_076.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_077.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_001.json, from before the Transform, opens as added and saves without the three new settings | {"brightness":0,"channel":"rgb","clip_hdr_results":"on","contrast":100,"density":0,"direction":0,"edge_definition":50,"evolution":0,"flow_falloff":0,"flow_softness":20,"offset":[0,0],"sample_count":12,"sample_radius":30,"size":100,"smoothness":50,"source":"internal","speed":10,"swirl":45,"turbulence_speed":20,"vertical_bias":50,"view":"final_render"} | yes |
| fx_curl_067.json is saved with Rotation 45, Scale 150 and Perspective Offset on | {"brightness":0,"channel":"rgb","clip_hdr_results":"on","contrast":100,"density":0,"direction":90,"edge_definition":50,"evolution":0,"flow_falloff":0,"flow_softness":20,"offset":[3,2],"perspective_offset":"on","rotation":45,"sample_count":6,"sample_radius":4,"scale":150,"size":6,"smoothness":50,"source":"internal","speed":20,"swirl":45,"turbulence_speed":0,"vertical_bias":50,"view":"final_render"} | yes |
| fx_curl_070.json is saved with Rotation's keys kept | {"base":0,"keyframes":[{"frame":0,"interp":"linear","value":0},{"frame":4,"interp":"linear","value":90}]} | yes |
| fx_curl_074.json is refused in a sentence | Curl Noise's scale runs from 1 to 10000, and this is 0.5. | yes |
| fx_curl_075.json is refused in a sentence | Curl Noise's rotation runs from -100000 to 100000, and this is 200000. | yes |
| fx_curl_076.json is refused in a sentence | Curl Noise's perspective offset is "off" or "on", and this is "yes". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| scale 0.5 is refused with a sentence, and nothing changes | Curl Noise's scale runs from 1 to 10000, and this is 0.5. | yes |
| rotation 200000 is refused with a sentence, and nothing changes | Curl Noise's rotation runs from -100000 to 100000, and this is 200000. | yes |
| perspective offset "yes" is refused with a sentence, and nothing changes | Curl Noise's perspective offset is "off" or "on", and this is "yes". | yes |
| rotation -45, scale 300, perspective offset on is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_curl_062.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_curl_067.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_curl_069.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_curl_071.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_curl_061.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_062.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_063.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_064.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_065.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_066.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_067.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_068.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_069.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_070.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_071.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_072.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_073.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_074.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_075.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_076.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_077.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Curl Noise Rotation 30: the processor's frame 100 differs from the shot without it and from Curl Noise as added, so the comparisons below test the Transform | 2072928 pixels changed, 2055173 differ from as added | yes |
| the reference shot, Curl Noise Rotation 30 on three layers, frame 0, Full | largest difference 1 of 255, 1237 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Rotation 30 on three layers, frame 100, Full | largest difference 1 of 255, 1234 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Rotation 30 on three layers, frame 239, Full | largest difference 1 of 255, 1194 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Rotation 30 on three layers, frame 0, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Rotation 30 on three layers, frame 100, Draft | largest difference 1 of 255, 75 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Rotation 30 on three layers, frame 239, Draft | largest difference 1 of 255, 74 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Rotation 45, Scale 250, Perspective Offset on, Speed 40: the processor's frame 100 differs from the shot without it and from Curl Noise as added, so the comparisons below test the Transform | 2072945 pixels changed, 2059460 differ from as added | yes |
| the reference shot, Curl Noise Rotation 45, Scale 250, Perspective Offset on, Speed 40 on three layers, frame 0, Full | largest difference 1 of 255, 1168 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Rotation 45, Scale 250, Perspective Offset on, Speed 40 on three layers, frame 100, Full | largest difference 1 of 255, 1225 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Rotation 45, Scale 250, Perspective Offset on, Speed 40 on three layers, frame 239, Full | largest difference 1 of 255, 1242 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Rotation 45, Scale 250, Perspective Offset on, Speed 40 on three layers, frame 0, Draft | largest difference 1 of 255, 82 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Rotation 45, Scale 250, Perspective Offset on, Speed 40 on three layers, frame 100, Draft | largest difference 1 of 255, 75 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Rotation 45, Scale 250, Perspective Offset on, Speed 40 on three layers, frame 239, Draft | largest difference 1 of 255, 85 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Rotation keyed 0 to 720, Curl Generation: the processor's frame 100 differs from the shot without it and from Curl Noise as added, so the comparisons below test the Transform | 2073598 pixels changed, 2073600 differ from as added | yes |
| the reference shot, Curl Noise Rotation keyed 0 to 720, Curl Generation on three layers, frame 0, Full | largest difference 1 of 255, 1277 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Rotation keyed 0 to 720, Curl Generation on three layers, frame 100, Full | largest difference 1 of 255, 1311 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Rotation keyed 0 to 720, Curl Generation on three layers, frame 239, Full | largest difference 1 of 255, 1242 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Rotation keyed 0 to 720, Curl Generation on three layers, frame 0, Draft | largest difference 1 of 255, 75 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Rotation keyed 0 to 720, Curl Generation on three layers, frame 100, Draft | largest difference 1 of 255, 67 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Rotation keyed 0 to 720, Curl Generation on three layers, frame 239, Draft | largest difference 1 of 255, 86 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise This Layer, Rotation 90, Scale 50: the processor's frame 100 differs from the shot without it and from Curl Noise as added, so the comparisons below test the Transform | 2072829 pixels changed, 2054556 differ from as added | yes |
| the reference shot, Curl Noise This Layer, Rotation 90, Scale 50 on three layers, frame 0, Full | largest difference 1 of 255, 1261 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise This Layer, Rotation 90, Scale 50 on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise This Layer, Rotation 90, Scale 50 on three layers, frame 239, Full | largest difference 1 of 255, 1169 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise This Layer, Rotation 90, Scale 50 on three layers, frame 0, Draft | largest difference 1 of 255, 84 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise This Layer, Rotation 90, Scale 50 on three layers, frame 100, Draft | largest difference 1 of 255, 71 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise This Layer, Rotation 90, Scale 50 on three layers, frame 239, Draft | largest difference 1 of 255, 102 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-646 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_noise.png, Size 40, View Input Noise: the noise as made, its blobs upright; draws cleanly | [] | yes |
| 2_noise_rotation_90.png, the same with Rotation 90: the same blobs turned a quarter clockwise round the middle; draws cleanly | [], differs from 1_noise.png true | yes |
| 3_noise_scale_300.png, the same with Scale 300: the blobs three times as big, grown from the middle; draws cleanly | [], differs from 1_noise.png true | yes |
| 4_flow_upright.png and 5_flow_rotation_45.png, the flow lines at Rotation 0 and 45: the second's swirls are the first's turned an eighth; draws cleanly | [], they differ true | yes |

## Result

122 of 122 checks pass.
