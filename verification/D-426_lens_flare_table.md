# D-426: Lens Flare

B-305, after After Effects' Lens Flare: the flare a bright light makes shining into a camera lens, added over the layer. Its controls are After Effects' (Flare Center, Flare Brightness, Lens Type: 50-300mm Zoom, 35mm Prime or 105mm Prime, Blend With Original); each lens is a fixed set of glows, a halo ring, a star of rays and coloured ghost discs placed along the line from the flare through the layer's middle, after the parts of Video Copilot's Optical Flares. The parts, sizes, colours, ranges and defaults are ours. Every expected pixel is `Fixtures/lens_flare/expected_lens_flare.json`, written by `tools/lens_flare_reference.py` before this code existed and printed in document 25 as FX-FLARE-001 to 019. Tolerance 2e-5.

## FX-FLARE-001 to 019 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FLARE-001 frame 0: As added: the 50-300mm zoom flare at 30, 30 per cent, brightness 100: a white-hot core and a warm glow round (4.8, 3), six faint rays, a blue halo, and coloured ghosts along the line through the middle. | largest difference 1.9e-7 | yes |
| FX-FLARE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-002 frame 0: The flare at the middle, 50, 50: every ghost sits on the flare itself, so the picture is round about the middle. | largest difference 2.0e-7 | yes |
| FX-FLARE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-003 frame 0: Brightness 0: no light; the photo untouched. | largest difference 1.5e-7 | yes |
| FX-FLARE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-004 frame 0: Brightness 300: three times FX-FLARE-001's light. | largest difference 2.9e-7 | yes |
| FX-FLARE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-005 frame 0: The 35mm prime: a smaller glow, eight rays, three ghosts. | largest difference 1.8e-7 | yes |
| FX-FLARE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-006 frame 0: The 105mm prime: a bigger, warmer glow, twelve long rays, an orange halo, two ghosts. | largest difference 2.4e-7 | yes |
| FX-FLARE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-007 frame 0: Blend With Original 100: the photo untouched. | largest difference 1.5e-7 | yes |
| FX-FLARE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-008 frame 0: Blend With Original 50: halfway between FX-FLARE-001 and the photo. | largest difference 1.8e-7 | yes |
| FX-FLARE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-009 frame 0: On the shapes drawing, clear between its blocks: the light shows in the clear parts too, as covering as it is bright. | largest difference 1.9e-7 | yes |
| FX-FLARE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-010 frame 0: FX-FLARE-001 on the holder moved 2 right and 1 down: the same, moved; the flare is drawn on the drawing before it moves. | largest difference 1.9e-7 | yes |
| FX-FLARE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-011 frame 0: The centre keyed from 0, 0 at frame 0 to 100, 100 at frame 4, linear: frame 2 is FX-FLARE-002. | largest difference 1.9e-7 | yes |
| FX-FLARE-011 frame 2: The centre keyed from 0, 0 at frame 0 to 100, 100 at frame 4, linear: frame 2 is FX-FLARE-002. | largest difference 2.0e-7 | yes |
| FX-FLARE-011 frame 4: The centre keyed from 0, 0 at frame 0 to 100, 100 at frame 4, linear: frame 2 is FX-FLARE-002. | largest difference 1.8e-7 | yes |
| FX-FLARE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-012 frame 0: Brightness eased from 0 at frame 0 to 300 at frame 4 on a curve that overshoots: at frame 2 it would pass 300, is held at 300, as frame 4 is. | largest difference 1.5e-7 | yes |
| FX-FLARE-012 frame 2: Brightness eased from 0 at frame 0 to 300 at frame 4 on a curve that overshoots: at frame 2 it would pass 300, is held at 300, as frame 4 is. | largest difference 2.9e-7 | yes |
| FX-FLARE-012 frame 4: Brightness eased from 0 at frame 0 to 300 at frame 4 on a curve that overshoots: at frame 2 it would pass 300, is held at 300, as frame 4 is. | largest difference 2.9e-7 | yes |
| FX-FLARE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-013 frame 0: The flare off the layer, at -50, 150: only its ghosts and the tail of its glow and rays reach the drawing. | largest difference 1.8e-7 | yes |
| FX-FLARE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-014 frame 0: The 105mm prime at 70, 20, brightness 150, blend 25, on the ramp: the controls together. | largest difference 1.3e-7 | yes |
| FX-FLARE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FLARE-015 frame 0: Brightness 301, above 300. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-015 frame 4: Brightness 301, above 300. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLARE-016 frame 0: Brightness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-016 frame 4: Brightness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLARE-017 frame 0: Blend With Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-017 frame 4: Blend With Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLARE-018 frame 0: A lens type written "200mm". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-018 frame 4: A lens type written "200mm". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FLARE-019 frame 0: A centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-019 frame 4: A centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FLARE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: the flare is drawn on the layer as it is | 0 | yes |
| a half-size draft preview changes no setting: the flare's place is a share of the drawing and its parts' sizes shares of its diagonal | LensFlare { flare_center: [30.0, 30.0], flare_brightness: 100.0, lens_type: "zoom", blend_with_original: 0.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_flare_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_flare_014.json is saved with all 4 settings | {"blend_with_original":25,"flare_brightness":150,"flare_center":[70,20],"lens_type":"105mm"} | yes |
| fx_flare_011.json is saved with the centre's two keys | {"base":[0,0],"keyframes":[{"frame":0,"interp":"linear","value":[0,0]},{"frame":4,"interp":"linear","value":[100,100]}]} | yes |
| fx_flare_015.json is refused in a sentence | Lens Flare's flare brightness runs from 0 to 300, and this is 301. | yes |
| fx_flare_016.json is refused in a sentence | Lens Flare's flare brightness runs from 0 to 300, and this is -1. | yes |
| fx_flare_017.json is refused in a sentence | Lens Flare's blend with original runs from 0 to 100, and this is 101. | yes |
| fx_flare_018.json is refused in a sentence | Lens Flare's lens type is "zoom", "35mm" or "105mm", and this is "200mm". | yes |
| fx_flare_019.json is refused in a sentence | Lens Flare's flare center runs from -1000 to 1000, and this is 1001. | yes |
| a file with a Lens Flare with no `flare_center` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Lens Flare whose centre is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Lens Flare whose brightness is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Lens Flare whose lens type is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Lens Flare with no `lens_type` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| brightness 300.5 is refused with a sentence, and nothing changes | Lens Flare's flare brightness runs from 0 to 300, and this is 300.5. | yes |
| brightness -0.5 is refused with a sentence, and nothing changes | Lens Flare's flare brightness runs from 0 to 300, and this is -0.5. | yes |
| blend 100.5 is refused with a sentence, and nothing changes | Lens Flare's blend with original runs from 0 to 100, and this is 100.5. | yes |
| lens type "fisheye" is refused with a sentence, and nothing changes | Lens Flare's lens type is "zoom", "35mm" or "105mm", and this is "fisheye". | yes |
| centre -1001, 50 is refused with a sentence, and nothing changes | Lens Flare's flare center runs from -1000 to 1000, and this is -1001. | yes |
| brightness keyed to 301 is refused with a sentence, and nothing changes | Lens Flare's flare brightness runs from 0 to 300, and this is 301. | yes |
| centre keyed to 50, 1001 is refused with a sentence, and nothing changes | Lens Flare's flare center runs from -1000 to 1000, and this is 1001. | yes |
| the tops: centre 1000, 1000, brightness 300, blend 100, the 35mm prime is taken | taken | yes |
| the bottoms: centre -1000, -1000, brightness 0, blend 0, the 105mm prime is taken | taken | yes |
| brightness keyed from 0 to 300 is taken | taken | yes |
| centre keyed from 0, 0 to 100, 100 is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_flare_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_flare_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_flare_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_flare_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_flare_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_flare_011.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_flare_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_flare_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_flare_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flare_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flare_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flare_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flare_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flare_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flare_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flare_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flare_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flare_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flare_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flare_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flare_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flare_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_flare_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_flare_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_flare_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_flare_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_flare_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Lens Flare as it starts (the 50-300mm zoom at 30, 30, brightness 100): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1108876 pixels changed | yes |
| the reference shot, Lens Flare as it starts (the 50-300mm zoom at 30, 30, brightness 100) on three layers, frame 0, Full | largest difference 1 of 255, 1000 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare as it starts (the 50-300mm zoom at 30, 30, brightness 100) on three layers, frame 100, Full | largest difference 1 of 255, 936 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare as it starts (the 50-300mm zoom at 30, 30, brightness 100) on three layers, frame 239, Full | largest difference 1 of 255, 942 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare as it starts (the 50-300mm zoom at 30, 30, brightness 100) on three layers, frame 0, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare as it starts (the 50-300mm zoom at 30, 30, brightness 100) on three layers, frame 100, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare as it starts (the 50-300mm zoom at 30, 30, brightness 100) on three layers, frame 239, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare the 35mm prime at 80, 60, brightness 200, blend 30: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 429136 pixels changed | yes |
| the reference shot, Lens Flare the 35mm prime at 80, 60, brightness 200, blend 30 on three layers, frame 0, Full | largest difference 1 of 255, 3367 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare the 35mm prime at 80, 60, brightness 200, blend 30 on three layers, frame 100, Full | largest difference 1 of 255, 871 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare the 35mm prime at 80, 60, brightness 200, blend 30 on three layers, frame 239, Full | largest difference 1 of 255, 1285 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare the 35mm prime at 80, 60, brightness 200, blend 30 on three layers, frame 0, Draft | largest difference 1 of 255, 118 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare the 35mm prime at 80, 60, brightness 200, blend 30 on three layers, frame 100, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare the 35mm prime at 80, 60, brightness 200, blend 30 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare the 105mm prime, its centre keyed from 10, 20 to 90, 80: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1008690 pixels changed | yes |
| the reference shot, Lens Flare the 105mm prime, its centre keyed from 10, 20 to 90, 80 on three layers, frame 0, Full | largest difference 1 of 255, 3816 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare the 105mm prime, its centre keyed from 10, 20 to 90, 80 on three layers, frame 100, Full | largest difference 1 of 255, 1016 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare the 105mm prime, its centre keyed from 10, 20 to 90, 80 on three layers, frame 239, Full | largest difference 1 of 255, 1691 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare the 105mm prime, its centre keyed from 10, 20 to 90, 80 on three layers, frame 0, Draft | largest difference 1 of 255, 155 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare the 105mm prime, its centre keyed from 10, 20 to 90, 80 on three layers, frame 100, Draft | largest difference 1 of 255, 71 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Lens Flare the 105mm prime, its centre keyed from 10, 20 to 90, 80 on three layers, frame 239, Draft | largest difference 1 of 255, 67 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street at frame 0, in `verification/D-426 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: the 50-300mm zoom at 30, 30, a hot glow and rays up and left, a blue halo, ghosts down to the right; draws cleanly | [], 61572 pixels changed | yes |
| 3_prime_35mm.png, the 35mm prime: a smaller, whiter glow with eight rays and three ghosts; draws cleanly | [], 20574 pixels changed | yes |
| 4_prime_105mm.png, the 105mm prime at 75, 25: a large white glow, twelve long rays, a faint warm halo; draws cleanly | [], 51639 pixels changed | yes |
| 5_bright.png, brightness 250 at 50, 20: the glow washes the sky white; draws cleanly | [], 61375 pixels changed | yes |
| 6_blended.png, blend 60: the zoom flare at 40% strength; draws cleanly | [], 54547 pixels changed | yes |

## Result

146 of 146 checks pass.
