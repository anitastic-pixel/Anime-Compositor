# B-629: Curl Noise, Uniform Scaling, Scale Width and Scale Height (D-750)

`core.curl_noise` takes After Effects' Uniform Scaling (on when added) and, with it off, Scale Width and Scale Height (100 each): the noise is read at q = c + (mx (d_x cos + d_y sin), my (-d_x sin + d_y cos)), mx = 100 / Scale Width and my = 100 / Scale Height; with Uniform Scaling on, mx = my = 100 / Scale as before. A picture (This Layer or Other Layer) is not scaled. A file from before reads as Uniform Scaling on and draws as it did. Every expected pixel is `Fixtures/curl_noise/expected_curl_transform.json`, written by `tools/curl_transform_reference.py` before this code existed and printed in document 25 as FX-CURL-079 to 085. Tolerance 2e-5.

## FX-CURL-079 to 085 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CURL-079 frame 0: Uniform Scaling off, Scale Width 200, Scale Height 100, still, Input Noise: the noise twice as wide round the card's middle, as tall as before. | largest difference 3.0e-8 | yes |
| FX-CURL-079: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-080 frame 0: Uniform Scaling off, Scale Width 200, Scale Height 200, still, Input Noise: FX-CURL-063's Scale 200. | largest difference 3.9e-8 | yes |
| FX-CURL-080: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-081 frame 0: Uniform Scaling on, Scale Width 300 and Scale Height 50 written, still, Input Noise: with Uniform Scaling on the two are not read, so it is Scale 100's frame. | largest difference 3.0e-8 | yes |
| FX-CURL-081: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-082 frame 0: Uniform Scaling off, Scale Width 7500, Scale Height 5, still (a tutorial's long streaks): the noise stretched far across and squeezed down, so the greys change down the card far more than across it. | largest difference 3.0e-8 | yes |
| FX-CURL-082: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-083 frame 0: Scale Width 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-083 frame 4: Scale Width 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-083: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-084 frame 0: Uniform Scaling "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-084 frame 4: Uniform Scaling "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-084: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-085 frame 0: Scale Height keyed to 20000 at frame 4, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-085 frame 4: Scale Height keyed to 20000 at frame 4, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-085: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_curl_079.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_080.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_081.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_082.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_083.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_084.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_085.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_001.json, from before D-750, saves without the three new settings | {"brightness":0,"channel":"rgb","clip_hdr_results":"on","contrast":100,"density":0,"direction":0,"edge_definition":50,"evolution":0,"flow_falloff":0,"flow_softness":20,"offset":[0,0],"sample_count":12,"sample_radius":30,"size":100,"smoothness":50,"source":"internal","speed":10,"swirl":45,"turbulence_speed":20,"vertical_bias":50,"view":"final_render"} | yes |
| fx_curl_001.json opens as Uniform Scaling on, Scale Width 100, Scale Height 100 | true | yes |
| fx_curl_079.json is saved with Uniform Scaling off, Scale Width 200 and Scale Height 100 | {"brightness":0,"channel":"rgb","clip_hdr_results":"on","contrast":100,"density":0,"direction":0,"edge_definition":50,"evolution":0,"flow_falloff":0,"flow_softness":20,"offset":[0,0],"sample_count":6,"sample_radius":4,"scale_width":200,"size":6,"smoothness":50,"source":"internal","speed":0,"swirl":45,"turbulence_speed":0,"uniform_scaling":"off","vertical_bias":50,"view":"input_noise"} | yes |
| fx_curl_083.json is refused in a sentence | Curl Noise's scale width runs from 1 to 10000, and this is 0.5. | yes |
| fx_curl_084.json is refused in a sentence | Curl Noise's uniform scaling is "on" or "off", and this is "yes". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| scale height 20000 is refused with a sentence, and nothing changes | Curl Noise's scale height runs from 1 to 10000, and this is 20000. | yes |
| uniform scaling "yes" is refused with a sentence, and nothing changes | Curl Noise's uniform scaling is "on" or "off", and this is "yes". | yes |
| uniform scaling off, scale width 50, scale height 400 is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_curl_079.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_curl_082.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_curl_079.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_080.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_081.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_082.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_083.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_084.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_085.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |

## Pictures: the street, Input Noise, in `verification/B-629 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_uniform_on.png, Size 10, Edge Definition 0 (no greys clipped), Uniform Scaling on: the noise as before, as busy across as down; draws cleanly | []; mean step across 2.29, down 2.36 levels | yes |
| Uniform Scaling on with Scale Width 300 and Scale Height 50 written: the two are not read, the same picture as 1_uniform_on.png | [], the same true | yes |
| 2_width_300.png, Uniform Scaling off, Scale Width 300, Scale Height 100: the blobs three times as wide, as tall as before, so the steps across are under half the steps down | []; mean step across 0.75, down 2.58 levels | yes |
| 3_streaks_7500_by_5.png, Scale Width 7500, Scale Height 5 (fimje's long streaks): long level streaks, the steps down more than ten times the steps across | []; mean step across 0.04, down 38.78 levels | yes |

## Result

46 of 46 checks pass.
