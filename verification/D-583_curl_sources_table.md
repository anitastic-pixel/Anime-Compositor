# D-583: Curl Noise, Source This Layer and Other Layer

B-461: `core.curl_noise`'s Source takes After Effects' This Layer and Other Layer. The noise is then the picture's lightness (each pixel's luminance times its covering, read between pixels), blurred first by Softness (a Gaussian, sigma Softness / 10 pixels); its slope, the lightness one pixel on less one pixel back, steers the flow lines as the made noise's does. This Layer reads the layer as it reaches the effect, so it moves with the drawing; Other Layer reads D-189's layer setting Source Layer, fitted to the drawing by Source Mapping (Fill stretches it, Center keeps its own size). Outside the picture is black, or with Mirror at Edge on the picture mirrored back. The rule is this program's own; Adobe publishes none. Every expected pixel is `Fixtures/curl_noise/expected_curl_noise.json`, written by `tools/curl_noise_reference.py` before this code existed and printed in document 25 as FX-CURL-033, 034 and 045 to 060 (033 regenerated as a working This Layer case and 034 reworded, under D-566). Tolerance 2e-5.

## FX-CURL-033, 034 and 045 to 060 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CURL-033 frame 0: Source This Layer, small cells and lines, otherwise as added: the card's own lightness is the noise, so its flat patches are even greys streaked by the seed and its edges steer the flow lines; Speed 10 slides it 1 pixel up a frame. | largest difference 1.1e-7 | yes |
| FX-CURL-033 frame 4: Source This Layer, small cells and lines, otherwise as added: the card's own lightness is the noise, so its flat patches are even greys streaked by the seed and its edges steer the flow lines; Speed 10 slides it 1 pixel up a frame. | largest difference 1.1e-7 | yes |
| FX-CURL-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-034 frame 0: Source Other Layer, with Source Layer naming no layer. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-034 frame 4: Source Other Layer, with Source Layer naming no layer. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-045 frame 0: Source This Layer, still, Input Noise: each shown pixel's grey is its own luminance times its covering. | largest difference 8.5e-8 | yes |
| FX-CURL-045: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-046 frame 0: Source This Layer, still, Curl Generation: inside a flat patch there is no flow (red and green the middle grey); at the edges the flow runs along them. | largest difference 8.5e-8 | yes |
| FX-CURL-046: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-047 frame 0: Source Other Layer, layer "ramp" (8 by 5 greys), Source Mapping Fill, still, Input Noise: the ramp stretched over the card. | largest difference 1.4e-7 | yes |
| FX-CURL-047: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-048 frame 0: Source Other Layer, "ramp", Source Mapping Center, still, Input Noise: the ramp at its own size in the middle, black round it. | largest difference 1.4e-7 | yes |
| FX-CURL-048: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-049 frame 0: Source Other Layer, "ramp", Fill, still: flow lines across the ramp's slope. | largest difference 3.1e-7 | yes |
| FX-CURL-049: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-050 frame 0: Source This Layer, Softness 20, still, Input Noise: FX-CURL-045's greys blurred, sigma 2 pixels. | largest difference 1.5e-7 | yes |
| FX-CURL-050: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-051 frame 0: Source This Layer, Offset 5 right, Mirror at Edge on, still, Input Noise: left of the card's edge the card is read mirrored. | largest difference 8.5e-8 | yes |
| FX-CURL-051: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-052 frame 0: Source This Layer, Offset 5 right, Mirror at Edge off, still, Input Noise: left of the card's edge is black. | largest difference 8.5e-8 | yes |
| FX-CURL-052: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-053 frame 0: Source Other Layer naming "gone", not in the composition: the frames are drawn without the effect, with D-189's warning. | largest difference 1.9e-7 | yes |
| FX-CURL-053 frame 4: Source Other Layer naming "gone", not in the composition: the frames are drawn without the effect, with D-189's warning. | largest difference 1.9e-7 | yes |
| FX-CURL-053: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-CURL-054 frame 0: FX-CURL-046 moved three pixels right: the picture is the drawing's own, so it moves with it. | largest difference 8.5e-8 | yes |
| FX-CURL-054: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-055 frame 0: FX-CURL-046 after a Motion Tile that grows the layer: the picture is the layer as it reaches the effect, so at the card's edges the tiles beyond it turn the flow; inside, the frame is FX-CURL-046's. | largest difference 8.5e-8 | yes |
| FX-CURL-055: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-056 frame 0: Source This Layer, Softness keyed from 0 at frame 0 to 30 at frame 4, still, Input Noise: frame 0 is FX-CURL-045. | largest difference 8.5e-8 | yes |
| FX-CURL-056 frame 4: Source This Layer, Softness keyed from 0 at frame 0 to 30 at frame 4, still, Input Noise: frame 0 is FX-CURL-045. | largest difference 1.2e-7 | yes |
| FX-CURL-056: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CURL-057 frame 0: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-057 frame 4: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-057: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-058 frame 0: Source Mapping "stretch", not fill or center. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-058 frame 4: Source Mapping "stretch", not fill or center. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-058: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-059 frame 0: Mirror at Edge "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-059 frame 4: Mirror at Edge "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-059: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CURL-060 frame 0: Source Layer 3, a number and not a layer's word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-060 frame 4: Source Layer 3, a number and not a layer's word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CURL-060: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_curl_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_054.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_055.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_056.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_057.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_058.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_059.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_060.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_curl_001.json, from before the sources, opens as Internal and saves without the four new settings | {"brightness":0,"channel":"rgb","clip_hdr_results":"on","contrast":100,"density":0,"direction":0,"edge_definition":50,"evolution":0,"flow_falloff":0,"flow_softness":20,"offset":[0,0],"sample_count":12,"sample_radius":30,"size":100,"smoothness":50,"source":"internal","speed":10,"swirl":45,"turbulence_speed":20,"vertical_bias":50,"view":"final_render"} | yes |
| fx_curl_048.json is saved with Source Other Layer, its layer and Center | {"brightness":0,"channel":"rgb","clip_hdr_results":"on","contrast":100,"density":0,"direction":0,"edge_definition":50,"evolution":0,"flow_falloff":0,"flow_softness":20,"offset":[0,0],"sample_count":6,"sample_radius":4,"size":6,"smoothness":50,"source":"other_layer","source_layer":"ramp","source_mapping":"center","speed":0,"swirl":45,"turbulence_speed":0,"vertical_bias":50,"view":"input_noise"} | yes |
| fx_curl_051.json is saved with Mirror at Edge on | {"brightness":0,"channel":"rgb","clip_hdr_results":"on","contrast":100,"density":0,"direction":0,"edge_definition":50,"evolution":0,"flow_falloff":0,"flow_softness":20,"mirror_at_edge":"on","offset":[5,0],"sample_count":6,"sample_radius":4,"size":6,"smoothness":50,"source":"this_layer","speed":0,"swirl":45,"turbulence_speed":0,"vertical_bias":50,"view":"input_noise"} | yes |
| fx_curl_056.json is saved with Softness's keys kept | {"base":0,"keyframes":[{"frame":0,"interp":"linear","value":0},{"frame":4,"interp":"linear","value":30}]} | yes |
| fx_curl_034.json is refused in a sentence | Curl Noise's Source is Other Layer, but Source Layer names no layer: choose one, or set Source to Internal or This Layer. | yes |
| fx_curl_057.json is refused in a sentence | Curl Noise's softness runs from 0 to 100, and this is 101. | yes |
| fx_curl_058.json is refused in a sentence | Curl Noise's source mapping is "fill" or "center", and this is "stretch". | yes |
| fx_curl_059.json is refused in a sentence | Curl Noise's mirror at edge is "off" or "on", and this is "yes". | yes |
| fx_curl_060.json is refused in a sentence | Curl Noise's source layer is the name of a layer of this composition, and this is 3. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| softness 101 is refused with a sentence, and nothing changes | Curl Noise's softness runs from 0 to 100, and this is 101. | yes |
| source mapping "stretch" is refused with a sentence, and nothing changes | Curl Noise's source mapping is "fill" or "center", and this is "stretch". | yes |
| mirror at edge "yes" is refused with a sentence, and nothing changes | Curl Noise's mirror at edge is "off" or "on", and this is "yes". | yes |
| source layer 3 is refused with a sentence, and nothing changes | Curl Noise's source layer is the name of a layer of this composition, and this is 3. | yes |
| Other Layer naming no layer is refused with a sentence, and nothing changes | Curl Noise's Source is Other Layer, but Source Layer names no layer: choose one, or set Source to Internal or This Layer. | yes |
| This Layer, softness 20, mirror at edge on is taken | taken | yes |
| Other Layer "ramp", Center is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_curl_033.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_curl_046.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_curl_049.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_curl_050.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_curl_051.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_curl_055.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_curl_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_045.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_046.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_047.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_048.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_049.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_050.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_051.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_052.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_053.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_054.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_055.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_056.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_057.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_058.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_059.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_curl_060.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Curl Noise This Layer as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072787 pixels changed | yes |
| the reference shot, Curl Noise This Layer as added on three layers, frame 0, Full | largest difference 1 of 255, 1234 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise This Layer as added on three layers, frame 100, Full | largest difference 1 of 255, 1249 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise This Layer as added on three layers, frame 239, Full | largest difference 1 of 255, 1250 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise This Layer as added on three layers, frame 0, Draft | largest difference 1 of 255, 70 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise This Layer as added on three layers, frame 100, Draft | largest difference 1 of 255, 89 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise This Layer as added on three layers, frame 239, Draft | largest difference 1 of 255, 70 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise This Layer, Softness 20, Curl Generation, Mirror at Edge on, offset 30 and -20: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073599 pixels changed | yes |
| the reference shot, Curl Noise This Layer, Softness 20, Curl Generation, Mirror at Edge on, offset 30 and -20 on three layers, frame 0, Full | largest difference 1 of 255, 1488 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise This Layer, Softness 20, Curl Generation, Mirror at Edge on, offset 30 and -20 on three layers, frame 100, Full | largest difference 1 of 255, 968 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise This Layer, Softness 20, Curl Generation, Mirror at Edge on, offset 30 and -20 on three layers, frame 239, Full | largest difference 1 of 255, 1007 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise This Layer, Softness 20, Curl Generation, Mirror at Edge on, offset 30 and -20 on three layers, frame 0, Draft | largest difference 1 of 255, 76 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise This Layer, Softness 20, Curl Generation, Mirror at Edge on, offset 30 and -20 on three layers, frame 100, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise This Layer, Softness 20, Curl Generation, Mirror at Edge on, offset 30 and -20 on three layers, frame 239, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Other Layer layer4, Fill, radius 12: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072499 pixels changed | yes |
| the reference shot, Curl Noise Other Layer layer4, Fill, radius 12 on three layers, frame 0, Full | largest difference 1 of 255, 1246 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Other Layer layer4, Fill, radius 12 on three layers, frame 100, Full | largest difference 1 of 255, 1293 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Other Layer layer4, Fill, radius 12 on three layers, frame 239, Full | largest difference 1 of 255, 1352 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Other Layer layer4, Fill, radius 12 on three layers, frame 0, Draft | largest difference 1 of 255, 94 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Other Layer layer4, Fill, radius 12 on three layers, frame 100, Draft | largest difference 1 of 255, 95 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Other Layer layer4, Fill, radius 12 on three layers, frame 239, Draft | largest difference 1 of 255, 97 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Other Layer layer4, Center, Softness keyed 0 to 40, channel Alpha: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Curl Noise Other Layer layer4, Center, Softness keyed 0 to 40, channel Alpha on three layers, frame 0, Full | largest difference 1 of 255, 1489 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Other Layer layer4, Center, Softness keyed 0 to 40, channel Alpha on three layers, frame 100, Full | largest difference 1 of 255, 1592 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Other Layer layer4, Center, Softness keyed 0 to 40, channel Alpha on three layers, frame 239, Full | largest difference 1 of 255, 1538 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Other Layer layer4, Center, Softness keyed 0 to 40, channel Alpha on three layers, frame 0, Draft | largest difference 1 of 255, 341 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Other Layer layer4, Center, Softness keyed 0 to 40, channel Alpha on three layers, frame 100, Draft | largest difference 1 of 255, 300 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Curl Noise Other Layer layer4, Center, Softness keyed 0 to 40, channel Alpha on three layers, frame 239, Draft | largest difference 1 of 255, 355 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-583 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_this_layer.png, Source This Layer as added: grey flow lines that run along the street's own edges (buildings, windows, road), even where it is flat; draws cleanly | [], differs from the made noise true | yes |
| 3_this_layer_input_noise.png, This Layer, View Input Noise: the street in grey, its own lightness; draws cleanly | [], 0 pixels far from the street's lightness | yes |
| 4_this_layer_softness_40.png, This Layer, Softness 40: the picture blurred first, so the lines follow only the street's large shapes; draws cleanly | [], differs from 2_this_layer.png true | yes |
| 5_rings_curl_generation.png, Source Other Layer, the hidden layer rings (rings.png), View Curl Generation: the flow as red and green running round the rings; draws cleanly | [], 120952 pixels where red and green differ | yes |
| 6_rings_flow.png, Source Other Layer rings, radius 40: grey flow lines in circles round the middle; draws cleanly | [], differs from 2_this_layer.png true | yes |
| the street with Source Other Layer naming gone, not a layer: drawn without the effect, with D-189's warning | ["EFFECT_LAYER_MISSING An effect on layer art reads layer gone, which is not in this composition."] | yes |

## Result

138 of 138 checks pass.
