# D-359: Lens Blur's blur map

B-238, after After Effects' Camera Lens Blur Blur Map group: another layer's luminance or alpha, lying on the layer, says how far out of focus each pixel is. A value at Blur Focal Distance is sharp; the further from it, the wider the iris, to the full radius 255 levels away. Every expected pixel is `Fixtures/lens_blur/expected_lens_blur_map.json`, written by `tools/lens_blur_map_reference.py` before this code existed, FX-LENS-045 to 069. The build's frame is compared sample by sample against the tolerance of 2e-5. FX-LENS-001 to 044 are B-59's and B-64's and must not move.

## FX-LENS-045 to 069

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LENS-045 frame 0: Radius 4 with every Blur Map setting written at its start value: no layer, Center, Luminance, Focal Distance 0, Invert off: FX-LENS-007 exactly. | largest difference 8.3e-8 | yes |
| FX-LENS-045: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-046 frame 0: The ramp as the blur map, Focal Distance 0: black, near, at the left stays sharp, and the blur grows column by column to the full radius 4 at the white right edge. | largest difference 1.8e-7 | yes |
| FX-LENS-046: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-047 frame 0: The ramp, Focal Distance 255: the white right edge in focus, the left blurred most. | largest difference 2.3e-7 | yes |
| FX-LENS-047: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-048 frame 0: The ramp, Focal Distance 128: the middle in focus, both sides blurred about half the radius. | largest difference 2.1e-7 | yes |
| FX-LENS-048: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-049 frame 0: The ramp inverted, Focal Distance 0: FX-LENS-047. | largest difference 2.3e-7 | yes |
| FX-LENS-049: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-050 frame 0: A white solid read by its Alpha, Focal Distance 0: every pixel at the full radius, FX-LENS-007. | largest difference 8.3e-8 | yes |
| FX-LENS-050: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-051 frame 0: A black picture fading from covered at the left to clear at the right, read by its Alpha: blurred most at the left, sharp at the right. | largest difference 1.7e-7 | yes |
| FX-LENS-051: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-052 frame 0: The same picture read by its Luminance: black everywhere, so nothing is blurred; the layer still grows, and the drawing is as it was. | largest difference 1.9e-7 | yes |
| FX-LENS-052: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-053 frame 0: A 4 by 2 checker as the map, Center: it covers columns 6 to 9 of rows 4 and 5; only its four white pixels are blurred, everything else is sharp. | largest difference 1.9e-7 | yes |
| FX-LENS-053: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-054 frame 0: The checker, Stretch: spread over the whole layer and softened between its squares. | largest difference 1.7e-7 | yes |
| FX-LENS-054: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-055 frame 0: FX-LENS-046 with edges repeat on the picture that fills the layer: the map and the picture both held at the edges, every pixel still fully covered. | largest difference 1.2e-7 | yes |
| FX-LENS-055: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-056 frame 0: Focal Distance keyed from 0 at frame 0 to 255 at frame 4: frame 0 is FX-LENS-046 and frame 4 FX-LENS-047; the sharp band moves across. | largest difference 1.8e-7 | yes |
| FX-LENS-056 frame 2: Focal Distance keyed from 0 at frame 0 to 255 at frame 4: frame 0 is FX-LENS-046 and frame 4 FX-LENS-047; the sharp band moves across. | largest difference 2.1e-7 | yes |
| FX-LENS-056 frame 4: Focal Distance keyed from 0 at frame 0 to 255 at frame 4: frame 0 is FX-LENS-046 and frame 4 FX-LENS-047; the sharp band moves across. | largest difference 2.3e-7 | yes |
| FX-LENS-056: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-057 frame 0: A layer that is not in the composition, `gone`: the frame is drawn without the effect, with the warning every frame. | largest difference 1.9e-7 | yes |
| FX-LENS-057 frame 4: A layer that is not in the composition, `gone`: the frame is drawn without the effect, with the warning every frame. | largest difference 1.9e-7 | yes |
| FX-LENS-057: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-LENS-058 frame 0: FX-LENS-046 on the bars moved three pixels right: the same picture moved, since the map lies on the layer. | largest difference 1.8e-7 | yes |
| FX-LENS-058: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-059 frame 0: Depth: the depth fixtures' EXR on a layer of its own with Pass Extract, its depth from 0.5 (black) to 13 (white), stretched as the map, Focal Distance 0: the near top left in focus, the far bottom right blurred most. | largest difference 1.4e-7 | yes |
| FX-LENS-059: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-060 frame 0: The ramp with a hexagon iris and highlights, gain 3 at threshold 80: the skin lit and held at white after the levels are mixed. | largest difference 7.0e-8 | yes |
| FX-LENS-060: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-061 frame 0: The ramp at radius 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-LENS-061: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-062 frame 0: The ramp at radius 2.5, Focal Distance 85: three levels, 0, 0.83 and 1.67 of the radius, the sharp column the sixth. | largest difference 1.9e-7 | yes |
| FX-LENS-062: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-063 frame 0: Focal Distance 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-063 frame 4: Focal Distance 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-063: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-064 frame 0: Focal Distance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-064 frame 4: Focal Distance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-064: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-065 frame 0: Focal Distance keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-065 frame 4: Focal Distance keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-065: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-066 frame 0: Placement "tile", which Lens Blur's map does not offer. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-066 frame 4: Placement "tile", which Lens Blur's map does not offer. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-066: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-067 frame 0: Channel "red", which is not a channel it reads. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-067 frame 4: Channel "red", which is not a channel it reads. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-067: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-068 frame 0: Invert written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-068 frame 4: Invert written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-068: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-069 frame 0: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-069 frame 4: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-069: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| lens_map_cycle.json: `a`'s Lens Blur reads `b` as its map and `b`'s reads `a`: refused, `EFFECT_LAYER_CYCLE`. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |

## With no map, Lens Blur is the Lens Blur it was

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lens_045.json, every Blur Map setting at its start, against fx_lens_007.json, the same blur without them | byte-identical | yes |
| fx_lens_050.json, a white solid read by its alpha so every pixel is at the full radius, against fx_lens_007.json: the map's last level is Lens Blur's own sums, to the bit | byte-identical | yes |
| fx_lens_001.json, from before D-359, is saved without any of the new settings | {"edges":"transparent","radius":10} | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| with a map, the layer may grow as far as the full radius's iris reaches, 4 | 4 | yes |
| a half-size draft halves the radius and keeps Blur Focal Distance, which is a map value, not a distance | LensBlur { radius: 2.0, edges: "transparent", iris: "circle", roundness: 0.0, rotation: 0.0, aspect: 1.0, highlight_gain: 0.0, highlight_threshold: 100.0, layer: String("ramp"), fit: "center", channel: "luminance", focal_distance: 128.0, invert: "off", map: None } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lens_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_054.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_055.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_056.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_057.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_058.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_059.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_060.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_061.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_062.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_063.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_064.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_065.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_066.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_067.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_068.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_069.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with a Lens Blur whose Blur Focal Distance is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Blur Focal Distance 256 is refused with a sentence, and nothing changes | Lens Blur's focal distance runs from 0 to 255, and this is 256. | yes |
| Placement "tile" is refused with a sentence, and nothing changes | Lens Blur's map placement is "center" or "stretch", and this is "tile". | yes |
| Channel "red" is refused with a sentence, and nothing changes | Lens Blur's map channel is "luminance" or "alpha", and this is "red". | yes |
| Invert "yes" is refused with a sentence, and nothing changes | Lens Blur's invert blur map is "off" or "on", and this is "yes". | yes |
| Stretch, Alpha, Blur Focal Distance 128, Invert on, reading the card layer is taken | taken | yes |
| Blur Focal Distance keyed from 0 to 255 is taken | taken | yes |
| no layer, which turns the map off is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lens_046.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lens_053.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lens_055.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lens_056.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lens_059.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lens_045.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_lens_046.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_lens_047.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_lens_048.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_lens_049.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_lens_050.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_lens_051.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_lens_052.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_lens_053.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_lens_054.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_lens_055.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_lens_056.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_lens_057.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_lens_058.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_lens_059.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_lens_060.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_lens_061.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_lens_062.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| the reference shot, Lens Blur Radius 10, layer 4's luminance, Blur Focal Distance 0: the processor's frame 100 differs from the same blur without the map, so the comparisons below test the map | 1656917 pixels changed | yes |
| the reference shot, Lens Blur Radius 10, layer 4's luminance, Blur Focal Distance 0 on three layers, frame 0, Full | largest difference 1 of 255, 861 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Blur Radius 10, layer 4's luminance, Blur Focal Distance 0 on three layers, frame 100, Full | largest difference 1 of 255, 970 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Blur Radius 10, layer 4's luminance, Blur Focal Distance 0 on three layers, frame 239, Full | largest difference 1 of 255, 772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Blur Radius 10, layer 4's luminance, Blur Focal Distance 0 on three layers, frame 0, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Blur Radius 10, layer 4's luminance, Blur Focal Distance 0 on three layers, frame 100, Draft | largest difference 1 of 255, 64 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Blur Radius 10, layer 4's luminance, Blur Focal Distance 0 on three layers, frame 239, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Blur Radius 20, a hexagon with highlights, layer 4 stretched, Blur Focal Distance 128, inverted: the processor's frame 100 differs from the same blur without the map, so the comparisons below test the map | 1529693 pixels changed | yes |
| the reference shot, Lens Blur Radius 20, a hexagon with highlights, layer 4 stretched, Blur Focal Distance 128, inverted on three layers, frame 0, Full | largest difference 1 of 255, 946 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Blur Radius 20, a hexagon with highlights, layer 4 stretched, Blur Focal Distance 128, inverted on three layers, frame 100, Full | largest difference 1 of 255, 1320 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Blur Radius 20, a hexagon with highlights, layer 4 stretched, Blur Focal Distance 128, inverted on three layers, frame 239, Full | largest difference 1 of 255, 536 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Blur Radius 20, a hexagon with highlights, layer 4 stretched, Blur Focal Distance 128, inverted on three layers, frame 0, Draft | largest difference 1 of 255, 64 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Blur Radius 20, a hexagon with highlights, layer 4 stretched, Blur Focal Distance 128, inverted on three layers, frame 100, Draft | largest difference 1 of 255, 43 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Blur Radius 20, a hexagon with highlights, layer 4 stretched, Blur Focal Distance 128, inverted on three layers, frame 239, Draft | largest difference 1 of 255, 43 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Blur Radius 8, repeat edges, layer 4's alpha, Blur Focal Distance 40: the processor's frame 100 differs from the same blur without the map, so the comparisons below test the map | 1569373 pixels changed | yes |
| the reference shot, Lens Blur Radius 8, repeat edges, layer 4's alpha, Blur Focal Distance 40 on three layers, frame 0, Full | largest difference 1 of 255, 1014 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Blur Radius 8, repeat edges, layer 4's alpha, Blur Focal Distance 40 on three layers, frame 100, Full | largest difference 1 of 255, 884 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Blur Radius 8, repeat edges, layer 4's alpha, Blur Focal Distance 40 on three layers, frame 239, Full | largest difference 1 of 255, 691 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Blur Radius 8, repeat edges, layer 4's alpha, Blur Focal Distance 40 on three layers, frame 0, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Blur Radius 8, repeat edges, layer 4's alpha, Blur Focal Distance 40 on three layers, frame 100, Draft | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Blur Radius 8, repeat edges, layer 4's alpha, Blur Focal Distance 40 on three layers, frame 239, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: a town with a depth ramp, in `verification/D-359 pictures/`, twice enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the town with no effect; draws cleanly | [] | yes |
| lens_blur_no_map.png, Lens Blur radius 6 with no map: everything equally soft, houses and road markings alike | []; houses 4280337 from 100456965, road 4177738 from 64057824 | yes |
| lens_blur_road_sharp.png, the ramp as the blur map, Blur Focal Distance 255: the road at the bottom, white on the map, nearly sharp, the houses higher up softer and the sky softest, as a camera focused on the near ground | []; houses 20399181, road 28494240 of 64057824 before | yes |
| lens_blur_houses_sharp.png, Blur Focal Distance 140, the houses' grey on the map: the houses kept sharper than with no map, the road blurred more than in lens_blur_road_sharp.png | []; houses 61258332 (no map 4280337), road 16844127 (road sharp 28494240) | yes |

## Result

148 of 148 checks pass.
