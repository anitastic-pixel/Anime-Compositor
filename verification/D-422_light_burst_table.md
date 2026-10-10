# D-422: Light Burst

B-301, our name for CC Light Burst 2.5: the whole layer is the light, zoomed outward about the Center by Spin & Zoom Blur's straight, fading or centered zoom of amount Ray Length, and added on as Light Rays' rays are, Intensity / 100 times; with Set Color on, the rays are the Color at their covering. The manual gives no formula, ranges or defaults, so those are ours. Every expected pixel is `Fixtures/light_burst/expected_light_burst.json`, written by `tools/light_burst_reference.py` before this code existed and printed in document 25 as FX-BURST-001 to 030. Tolerance 2e-5.

## FX-BURST-001 to 030 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BURST-001 frame 0: The settings as they start: centre 50, 50, the point (8, 5), intensity 100, ray length 50, Straight, Set Color off. Every pixel that shows is light, the yellow, the brown and the purple alike, in its own colour: the yellow streaks left into the empty columns 0 to 3, the brown and purple right to the drawing's edge, and each streaks up and down into the empty rows, fading as it goes; each is brightened by its own light. Nothing is drawn past the layer's edge. | largest difference 3.1e-7 | yes |
| FX-BURST-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-002 frame 0: Intensity 0: the drawing, untouched. | largest difference 1.5e-7 | yes |
| FX-BURST-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-003 frame 0: Ray length 0: no streaks, but the light is still added onto itself: every colour twice as bright, and the half-covering edge doubled to full covering. | largest difference 3.1e-7 | yes |
| FX-BURST-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-004 frame 0: Fade: the streaks reach as far as FX-BURST-001's but fade as they run out, each pixel's own light counted most, so the streaks over the empty pixels are fainter, and the brown's inner column, which a straight burst mixes with the empty gap beside it, keeps more of its own colour. | largest difference 3.1e-7 | yes |
| FX-BURST-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-005 frame 0: Center: the streaks run half as far out and half as far in, toward the centre, so the brown's light falls inward on the empty column 9, which FX-BURST-001 leaves empty. | largest difference 3.7e-7 | yes |
| FX-BURST-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-006 frame 0: Ray length 100, the most: each pixel gathers light all the way from the centre out to itself, so the streaks reach further than FX-BURST-001's. | largest difference 3.1e-7 | yes |
| FX-BURST-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-007 frame 0: Intensity 250: two and a half times FX-BURST-001's rays, past white and not cut off; the covering stops at full. | largest difference 6.2e-7 | yes |
| FX-BURST-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-008 frame 0: Intensity 2000, the most: twenty times the rays. | largest difference 3.3e-6 | yes |
| FX-BURST-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-009 frame 0: Set Color on, #ff8000, orange: the rays take the colour where the layer shows, so the yellow, brown and purple all burst orange: red as much as the covering, a fifth as much green and no blue. The covering as FX-BURST-001's. | largest difference 1.5e-7 | yes |
| FX-BURST-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-010 frame 0: Set Color on, white: the rays are white light, so even the purple bursts white. | largest difference 2.1e-7 | yes |
| FX-BURST-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-011 frame 0: Set Color off with colour #ff8000: the colour does not count; FX-BURST-001. | largest difference 3.1e-7 | yes |
| FX-BURST-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-012 frame 0: FX-BURST-009 with its colour written in capitals, #FF8000: the same. | largest difference 1.5e-7 | yes |
| FX-BURST-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-013 frame 0: Set Color on, #000000, black: the rays add covering but no colour, a dark shadow over the empty pixels, and the half-covering edge darkens. | largest difference 1.5e-7 | yes |
| FX-BURST-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-014 frame 0: Centre 0, 0, the top left corner: the streaks run down and right, away from the corner; the corner itself stays empty. | largest difference 3.1e-7 | yes |
| FX-BURST-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-015 frame 0: Centre 50, -100, above the drawing: the streaks run straight down to the bottom edge, and nothing lights the empty rows above the drawing. | largest difference 2.9e-7 | yes |
| FX-BURST-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-016 frame 0: Ray length keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-BURST-003, frame 2 is FX-BURST-001, frame 4 is FX-BURST-006. | largest difference 3.1e-7 | yes |
| FX-BURST-016 frame 2: Ray length keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-BURST-003, frame 2 is FX-BURST-001, frame 4 is FX-BURST-006. | largest difference 3.1e-7 | yes |
| FX-BURST-016 frame 4: Ray length keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-BURST-003, frame 2 is FX-BURST-001, frame 4 is FX-BURST-006. | largest difference 3.1e-7 | yes |
| FX-BURST-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-017 frame 0: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-BURST-001, frame 2 bursts from 25, 25, frame 4 is FX-BURST-014. | largest difference 3.1e-7 | yes |
| FX-BURST-017 frame 2: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-BURST-001, frame 2 bursts from 25, 25, frame 4 is FX-BURST-014. | largest difference 3.7e-7 | yes |
| FX-BURST-017 frame 4: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-BURST-001, frame 2 bursts from 25, 25, frame 4 is FX-BURST-014. | largest difference 3.1e-7 | yes |
| FX-BURST-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-018 frame 0: Intensity eased from 0 at frame 0 to 2000 at frame 4 on a curve that overshoots: at frame 2 it would pass 2000, is held at 2000, and is FX-BURST-008, as frame 4 is. | largest difference 1.5e-7 | yes |
| FX-BURST-018 frame 2: Intensity eased from 0 at frame 0 to 2000 at frame 4 on a curve that overshoots: at frame 2 it would pass 2000, is held at 2000, and is FX-BURST-008, as frame 4 is. | largest difference 3.3e-6 | yes |
| FX-BURST-018 frame 4: Intensity eased from 0 at frame 0 to 2000 at frame 4 on a curve that overshoots: at frame 2 it would pass 2000, is held at 2000, and is FX-BURST-008, as frame 4 is. | largest difference 3.3e-6 | yes |
| FX-BURST-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-019 frame 0: Center, ray length 100: half the way in and half again out. | largest difference 3.5e-7 | yes |
| FX-BURST-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-020 frame 0: Fade, Set Color on, #40c0ff, intensity 300, centre 25, 75: the kinds and the colour together. | largest difference 2.0e-7 | yes |
| FX-BURST-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-021 frame 0: FX-BURST-001 moved three pixels right: the burst is drawn on the drawing before it is moved, so it is FX-BURST-001 moved, and the three columns left of the drawing stay empty, as the layer does not grow. | largest difference 3.1e-7 | yes |
| FX-BURST-021 frame 3: FX-BURST-001 moved three pixels right: the burst is drawn on the drawing before it is moved, so it is FX-BURST-001 moved, and the three columns left of the drawing stay empty, as the layer does not grow. | largest difference 3.1e-7 | yes |
| FX-BURST-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BURST-022 frame 0: Ray length 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BURST-022 frame 4: Ray length 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BURST-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BURST-023 frame 0: Intensity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BURST-023 frame 4: Intensity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BURST-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BURST-024 frame 0: Intensity 2001, above 2000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BURST-024 frame 4: Intensity 2001, above 2000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BURST-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BURST-025 frame 0: Intensity keyed to 3000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BURST-025 frame 4: Intensity keyed to 3000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BURST-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BURST-026 frame 0: Centre -1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BURST-026 frame 4: Centre -1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BURST-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BURST-027 frame 0: Burst "sideways", not one of the three. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BURST-027 frame 4: Burst "sideways", not one of the three. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BURST-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BURST-028 frame 0: Set Color "yes", neither "off" nor "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BURST-028 frame 4: Set Color "yes", neither "off" nor "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BURST-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BURST-029 frame 0: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BURST-029 frame 4: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BURST-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BURST-030 frame 0: A colour written "orange", a name, not #rrggbb, with Set Color on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BURST-030 frame 4: A colour written "orange", a name, not #rrggbb, with Set Color on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-BURST-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: rays past the layer's edge are cut | 0 | yes |
| a half-size draft preview changes nothing: the length and the centre are shares of the drawing | LightBurst { center: [30.0, 70.0], intensity: 250.0, ray_length: 80.0, burst: "fade", set_color: "on", color: "#ff8000" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_burst_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_burst_020.json is saved with all six settings | {"burst":"fade","center":[25,75],"color":"#40c0ff","intensity":300,"ray_length":50,"set_color":"on"} | yes |
| fx_burst_012.json's colour written #FF8000 is read and saved in small letters | {"burst":"straight","center":[50,50],"color":"#ff8000","intensity":100,"ray_length":50,"set_color":"on"} | yes |
| fx_burst_022.json is refused in a sentence | Light Burst's ray length runs from 0 to 100, and this is 101. | yes |
| fx_burst_023.json is refused in a sentence | Light Burst's intensity runs from 0 to 2000, and this is -1. | yes |
| fx_burst_024.json is refused in a sentence | Light Burst's intensity runs from 0 to 2000, and this is 2001. | yes |
| fx_burst_026.json is refused in a sentence | Light Burst's center runs from -1000 to 1000, and this is -1001. | yes |
| fx_burst_027.json is refused in a sentence | Light Burst's burst is "straight", "fade" or "center", and this is "sideways". | yes |
| fx_burst_028.json is refused in a sentence | Light Burst's set colour is "off" or "on", and this is "yes". | yes |
| fx_burst_029.json is refused in a sentence | Light Burst's colour is written #rrggbb, and this is "#12345". | yes |
| fx_burst_030.json is refused in a sentence | Light Burst's colour is written #rrggbb, and this is "orange". | yes |
| a file with a Light Burst with no `center` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Light Burst with one number for its centre is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Light Burst with its burst as a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Light Burst with no `set_color` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| ray length 101 is refused with a sentence, and nothing changes | Light Burst's ray length runs from 0 to 100, and this is 101. | yes |
| intensity 2001 is refused with a sentence, and nothing changes | Light Burst's intensity runs from 0 to 2000, and this is 2001. | yes |
| centre 50, 1001 is refused with a sentence, and nothing changes | Light Burst's center runs from -1000 to 1000, and this is 1001. | yes |
| burst "sideways" is refused with a sentence, and nothing changes | Light Burst's burst is "straight", "fade" or "center", and this is "sideways". | yes |
| set colour "yes" is refused with a sentence, and nothing changes | Light Burst's set colour is "off" or "on", and this is "yes". | yes |
| colour "orange" is refused with a sentence, and nothing changes | Light Burst's colour is written #rrggbb, and this is "orange". | yes |
| intensity keyed to 3000 is refused with a sentence, and nothing changes | Light Burst's intensity runs from 0 to 2000, and this is 3000. | yes |
| the tops: intensity 2000, ray length 100, centre 1000, 1000, centered is taken | taken | yes |
| the bottoms: intensity 0, ray length 0, centre -1000, -1000, fade is taken | taken | yes |
| ray length keyed from 0 to 100 is taken | taken | yes |
| centre keyed from 50, 50 to 0, 0 is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_burst_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_burst_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_burst_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_burst_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_burst_016.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_burst_020.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_burst_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_burst_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_burst_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_burst_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_burst_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_burst_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_burst_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_burst_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_burst_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_burst_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_burst_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_burst_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Light Burst as it starts (straight, intensity 100, ray length 50): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073160 pixels changed | yes |
| the reference shot, Light Burst as it starts (straight, intensity 100, ray length 50) on three layers, frame 0, Full | largest difference 1 of 255, 942 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst as it starts (straight, intensity 100, ray length 50) on three layers, frame 100, Full | largest difference 1 of 255, 910 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst as it starts (straight, intensity 100, ray length 50) on three layers, frame 239, Full | largest difference 1 of 255, 852 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst as it starts (straight, intensity 100, ray length 50) on three layers, frame 0, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst as it starts (straight, intensity 100, ray length 50) on three layers, frame 100, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst as it starts (straight, intensity 100, ray length 50) on three layers, frame 239, Draft | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst as the manual's tutorial (intensity 1200, ray length 35): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Light Burst as the manual's tutorial (intensity 1200, ray length 35) on three layers, frame 0, Full | largest difference 1 of 255, 298 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst as the manual's tutorial (intensity 1200, ray length 35) on three layers, frame 100, Full | largest difference 1 of 255, 203 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst as the manual's tutorial (intensity 1200, ray length 35) on three layers, frame 239, Full | largest difference 1 of 255, 235 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst as the manual's tutorial (intensity 1200, ray length 35) on three layers, frame 0, Draft | largest difference 1 of 255, 15 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst as the manual's tutorial (intensity 1200, ray length 35) on three layers, frame 100, Draft | largest difference 1 of 255, 10 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst as the manual's tutorial (intensity 1200, ray length 35) on three layers, frame 239, Draft | largest difference 1 of 255, 13 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst fade, Set Color on #ff8000, intensity 300, round 30, 40: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Light Burst fade, Set Color on #ff8000, intensity 300, round 30, 40 on three layers, frame 0, Full | largest difference 1 of 255, 2630 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst fade, Set Color on #ff8000, intensity 300, round 30, 40 on three layers, frame 100, Full | largest difference 1 of 255, 120 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst fade, Set Color on #ff8000, intensity 300, round 30, 40 on three layers, frame 239, Full | largest difference 1 of 255, 666 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst fade, Set Color on #ff8000, intensity 300, round 30, 40 on three layers, frame 0, Draft | largest difference 1 of 255, 115 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst fade, Set Color on #ff8000, intensity 300, round 30, 40 on three layers, frame 100, Draft | largest difference 1 of 255, 26 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst fade, Set Color on #ff8000, intensity 300, round 30, 40 on three layers, frame 239, Draft | largest difference 1 of 255, 35 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst center, ray length 100, round -10, 50, off the edge: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073570 pixels changed | yes |
| the reference shot, Light Burst center, ray length 100, round -10, 50, off the edge on three layers, frame 0, Full | largest difference 1 of 255, 831 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst center, ray length 100, round -10, 50, off the edge on three layers, frame 100, Full | largest difference 1 of 255, 904 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst center, ray length 100, round -10, 50, off the edge on three layers, frame 239, Full | largest difference 1 of 255, 743 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst center, ray length 100, round -10, 50, off the edge on three layers, frame 0, Draft | largest difference 1 of 255, 42 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst center, ray length 100, round -10, 50, off the edge on three layers, frame 100, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Burst center, ray length 100, round -10, 50, off the edge on three layers, frame 239, Draft | largest difference 1 of 255, 40 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Straight with Set Color off is Light Rays with no bright test

| Check | The build's answer | Matches |
| --- | --- | --- |
| the street, Light Burst as it starts against Light Rays at threshold 0 and intensity 1, the same length and centre | largest difference 0 of 255, 0 pixels differ | yes |
| the street, Light Burst ray length 80 round 30, 70 against Light Rays at threshold 0 and intensity 1, the same length and centre | largest difference 0 of 255, 0 pixels differ | yes |

## Pictures: the street, in `verification/D-422 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: every house streaked outward from the middle in its own colours, and brightened; draws cleanly | [], 129600 pixels changed | yes |
| 3_tutorial.png, the manual's tutorial settings, intensity 1200, ray length 35: on an opaque layer the street is blown out almost to white, streaks showing only in the road; draws cleanly | [], 129600 pixels changed | yes |
| 4_fade.png, fade, ray length 80: the streaks fade as they go out; draws cleanly | [], 129600 pixels changed | yes |
| 5_center.png, center, ray length 80: the streaks run both in and out from each house; draws cleanly | [], 129600 pixels changed | yes |
| 6_set_color.png, Set Color on #ff8000, intensity 150, round 25, 40: the street covers its whole frame, so its light is even and the picture is washed orange with no streaks (the rays take the layer's shape only where it is see-through, as FX-BURST-009's lamp shows); draws cleanly | [], 129600 pixels changed | yes |

## Result

205 of 205 checks pass.
