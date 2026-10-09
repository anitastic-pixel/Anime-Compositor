# D-375: Color Link

B-254, after After Effects' Color Link: one colour read from a whole layer's picture (its average, median, brightest, darkest, or each channel's highest or lowest, a share of each end clipped), laid over the layer by a blending mode at an opacity, only where it shows or over all of it. Every expected pixel is `Fixtures/color_link/expected_color_link.json`, written by `tools/color_link_reference.py` before this code existed and printed in document 25 as FX-CLINK-001 to 025. Tolerance 2e-5.

## FX-CLINK-001 to 025 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CLINK-001 frame 0: The settings as added: the layer's own picture, average, clip 5, stencil off, opacity 100, normal: the whole layer, its empty column too, one colour, the clipped average of its own colours. | largest difference 2.7e-9 | yes |
| FX-CLINK-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CLINK-002 frame 0: Source layer `swatch`, hidden: the layer turns the swatch's average colour, warm at frame 0 and cold at frame 3 when the swatch's drawing changes. | largest difference 1.0e-8 | yes |
| FX-CLINK-002 frame 3: Source layer `swatch`, hidden: the layer turns the swatch's average colour, warm at frame 0 and cold at frame 3 when the swatch's drawing changes. | largest difference 2.9e-8 | yes |
| FX-CLINK-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CLINK-003 frame 0: Sample median. | largest difference 1.9e-8 | yes |
| FX-CLINK-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CLINK-004 frame 0: Sample brightest: the colour of the swatch's brightest pixels, the top 5 per cent clipped. | largest difference 1.1e-7 | yes |
| FX-CLINK-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CLINK-005 frame 0: Sample darkest. | largest difference 6.6e-8 | yes |
| FX-CLINK-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CLINK-006 frame 0: Sample max RGB: each channel's own highest, 5 per cent clipped. | largest difference 1.4e-8 | yes |
| FX-CLINK-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CLINK-007 frame 0: Sample min RGB. | largest difference 4.9e-9 | yes |
| FX-CLINK-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CLINK-008 frame 0: Average with clip 0: every pixel of the swatch counted. | largest difference 6.3e-9 | yes |
| FX-CLINK-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CLINK-009 frame 0: Stencil on: the colour only where the layer shows; the empty column stays empty and the half-covered one stays half covered. | largest difference 3.0e-8 | yes |
| FX-CLINK-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CLINK-010 frame 0: Opacity 50, stencil on: halfway from the drawing to the colour. | largest difference 6.3e-8 | yes |
| FX-CLINK-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CLINK-011 frame 0: Opacity 50, stencil off: the drawing halfway to the colour, and the empty column the colour at half covering. | largest difference 6.3e-8 | yes |
| FX-CLINK-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CLINK-012 frame 0: Blending mode multiply, opacity 50, stencil on. | largest difference 1.2e-7 | yes |
| FX-CLINK-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CLINK-013 frame 0: Blending mode screen, opacity 50, stencil on. | largest difference 1.5e-7 | yes |
| FX-CLINK-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CLINK-014 frame 0: Blending mode add, opacity 50, stencil on. | largest difference 1.6e-7 | yes |
| FX-CLINK-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CLINK-015 frame 0: Blending mode overlay, opacity 100, stencil on: the drawing keeps its light and dark, coloured by the swatch. | largest difference 1.5e-7 | yes |
| FX-CLINK-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CLINK-016 frame 0: Blending mode soft light, opacity 100, stencil off: the empty column the plain colour, the rest softly tinted. | largest difference 1.6e-7 | yes |
| FX-CLINK-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CLINK-017 frame 0: Opacity keyed from 0 at frame 0 to 100 at frame 4, linear, stencil on: frame 0 untouched, frame 2 halfway, frame 4 the swatch's cold colour. | largest difference 1.9e-7 | yes |
| FX-CLINK-017 frame 2: Opacity keyed from 0 at frame 0 to 100 at frame 4, linear, stencil on: frame 0 untouched, frame 2 halfway, frame 4 the swatch's cold colour. | largest difference 9.5e-8 | yes |
| FX-CLINK-017 frame 4: Opacity keyed from 0 at frame 0 to 100 at frame 4, linear, stencil on: frame 0 untouched, frame 2 halfway, frame 4 the swatch's cold colour. | largest difference 3.1e-8 | yes |
| FX-CLINK-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CLINK-018 frame 0: Source layer `big`, 20 by 12, bigger than the holder: read whole, so its magenta border counts and pulls the average toward magenta. | largest difference 1.1e-8 | yes |
| FX-CLINK-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CLINK-020 frame 0: Clip 50, above 49. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CLINK-020 frame 4: Clip 50, above 49. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CLINK-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CLINK-021 frame 0: Opacity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CLINK-021 frame 4: Opacity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CLINK-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CLINK-022 frame 0: Sample "mean", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CLINK-022 frame 4: Sample "mean", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CLINK-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CLINK-023 frame 0: Stencil "yes", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CLINK-023 frame 4: Stencil "yes", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CLINK-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CLINK-024 frame 0: Blending mode "color_dodge", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CLINK-024 frame 4: Blending mode "color_dodge", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CLINK-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CLINK-025 frame 0: Blending mode "Normal": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CLINK-025 frame 4: Blending mode "Normal": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CLINK-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CLINK-019 frame 0: Source layer `ghost`, not a layer of the composition: the layer as it is, with EFFECT_LAYER_MISSING each frame. | largest difference 1.9e-7 | yes |
| FX-CLINK-019 frame 3: Source layer `ghost`, not a layer of the composition: the layer as it is, with EFFECT_LAYER_MISSING each frame. | largest difference 1.9e-7 | yes |
| FX-CLINK-019: what opening it warns of, and what frame 4 warns of (D-189: both; the file's `frame_warning` asks for frame 4 only, a PROPOSED correction to `warning`) | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_clink_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_clink_012.json is saved with its layer, sample, clip, stencil, opacity and blending mode, and no picture | {"blending_mode":"multiply","clip":5,"layer":"swatch","opacity":50,"sample":"average","stencil":"on"} | yes |
| fx_clink_022.json is refused in a sentence naming "mean" | Color Link's sample is "average", "median", "brightest", "darkest", "max_rgb" or "min_rgb", and this is "mean". | yes |
| fx_clink_023.json is refused in a sentence naming "yes" | Color Link's stencil is "off" or "on", and this is "yes". | yes |
| fx_clink_024.json is refused in a sentence naming "color_dodge" | Color Link's blending mode is "normal", "multiply", "screen", "add", "overlay" or "soft_light", and this is "color_dodge". | yes |
| fx_clink_025.json is refused in a sentence naming "Normal" | Color Link's blending mode is "normal", "multiply", "screen", "add", "overlay" or "soft_light", and this is "Normal". | yes |
| a file with a Color Link with no `sample` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Color Link whose opacity is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| clip 49.5 is refused with a sentence, and nothing changes | Color Link's clip runs from 0 to 49, and this is 49.5. | yes |
| opacity 100.5 is refused with a sentence, and nothing changes | Color Link's opacity runs from 0 to 100, and this is 100.5. | yes |
| sample "Median" written with a capital is refused with a sentence, and nothing changes | Color Link's sample is "average", "median", "brightest", "darkest", "max_rgb" or "min_rgb", and this is "Median". | yes |
| blending mode "difference" is refused with a sentence, and nothing changes | Color Link's blending mode is "normal", "multiply", "screen", "add", "overlay" or "soft_light", and this is "difference". | yes |
| the median, stencil on, overlay at 60 is taken | taken | yes |
| opacity keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_clink_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_clink_002.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_clink_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_clink_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_clink_018.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| the layer's own picture (no source layer) stays on the processor, since the card does not read a layer's colour back yet | FX-CLINK-001 below: 0 frames on the card | yes |
| fx_clink_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_008.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_009.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_011.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_012.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_clink_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Color Link layer4's average, normal at 50: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Color Link layer4's average, normal at 50 on three layers, frame 0, Full | largest difference 1 of 255, 32248 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Link layer4's average, normal at 50 on three layers, frame 100, Full | largest difference 1 of 255, 32275 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Link layer4's average, normal at 50 on three layers, frame 239, Full | largest difference 1 of 255, 32158 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Link layer4's average, normal at 50 on three layers, frame 0, Draft | largest difference 1 of 255, 2045 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Link layer4's average, normal at 50 on three layers, frame 100, Draft | largest difference 1 of 255, 2041 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Link layer4's average, normal at 50 on three layers, frame 239, Draft | largest difference 1 of 255, 2037 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Link layer4's brightest, multiply, stencil on: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Color Link layer4's brightest, multiply, stencil on on three layers, frame 0, Full | largest difference 1 of 255, 2031 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Link layer4's brightest, multiply, stencil on on three layers, frame 100, Full | largest difference 1 of 255, 2894 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Link layer4's brightest, multiply, stencil on on three layers, frame 239, Full | largest difference 1 of 255, 2152 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Link layer4's brightest, multiply, stencil on on three layers, frame 0, Draft | largest difference 1 of 255, 80 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Link layer4's brightest, multiply, stencil on on three layers, frame 100, Draft | largest difference 1 of 255, 42 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Link layer4's brightest, multiply, stencil on on three layers, frame 239, Draft | largest difference 1 of 255, 57 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Link layer4's median, soft light at 80: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Color Link layer4's median, soft light at 80 on three layers, frame 0, Full | largest difference 1 of 255, 1526 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Link layer4's median, soft light at 80 on three layers, frame 100, Full | largest difference 1 of 255, 3176 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Link layer4's median, soft light at 80 on three layers, frame 239, Full | largest difference 1 of 255, 1835 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Link layer4's median, soft light at 80 on three layers, frame 0, Draft | largest difference 1 of 255, 73 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Link layer4's median, soft light at 80 on three layers, frame 100, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Link layer4's median, soft light at 80 on three layers, frame 239, Draft | largest difference 1 of 255, 79 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-375 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts (its own average, normal at 100): the whole street one flat colour, its clipped average; draws cleanly | [], every pixel [127, 144, 161]: true | yes |
| 3_overlay_50.png, its own average laid over it by overlay at 50: the street keeps its light and dark under a wash of its own average colour; draws cleanly | [], 129600 pixels changed | yes |
| 4_brightest_multiply_40.png, its brightest colour by multiply at 40: a little darker everywhere, tinted by the lit windows; draws cleanly | [], every pixel darker or the same: true | yes |

## Result

155 of 155 checks pass.
