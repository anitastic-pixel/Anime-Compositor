# D-390: Ripple Pulse

B-269, after CycoreFX's CC Ripple Pulse: every change in Pulse Level sends a ring out from the centre that reaches the corners after Time Span seconds, pushing the picture outward (or drawing it in) Amplitude pixels per unit of change; Render Bump Map draws the heights in grey instead. The formulas are this program's own. Every expected pixel is `Fixtures/ripple_pulse/expected_ripple_pulse.json`, written by `tools/ripple_pulse_reference.py` before this code existed and printed in document 25 as FX-RPULSE-001 to 019. Tolerance 2e-5.

## FX-RPULSE-001 to 019 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RPULSE-001 frame 0: The settings as they start: Pulse Level 0 and never keyed, so there is no ring: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-RPULSE-001 frame 4: The settings as they start: Pulse Level 0 and never keyed, so there is no ring: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-RPULSE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RPULSE-002 frame 0: A drop: the level held at 0 until frame 1, then 10, Time Span 0.25 seconds (6 frames), Amplitude 2: frame 0 nothing; at frame 1 the middle pushed out 2 pixels; then a ring 2 pixels strong running outward, half way to the corners by frame 4. | largest difference 1.9e-7 | yes |
| FX-RPULSE-002 frame 1: A drop: the level held at 0 until frame 1, then 10, Time Span 0.25 seconds (6 frames), Amplitude 2: frame 0 nothing; at frame 1 the middle pushed out 2 pixels; then a ring 2 pixels strong running outward, half way to the corners by frame 4. | largest difference 1.9e-7 | yes |
| FX-RPULSE-002 frame 2: A drop: the level held at 0 until frame 1, then 10, Time Span 0.25 seconds (6 frames), Amplitude 2: frame 0 nothing; at frame 1 the middle pushed out 2 pixels; then a ring 2 pixels strong running outward, half way to the corners by frame 4. | largest difference 1.9e-7 | yes |
| FX-RPULSE-002 frame 4: A drop: the level held at 0 until frame 1, then 10, Time Span 0.25 seconds (6 frames), Amplitude 2: frame 0 nothing; at frame 1 the middle pushed out 2 pixels; then a ring 2 pixels strong running outward, half way to the corners by frame 4. | largest difference 2.2e-7 | yes |
| FX-RPULSE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RPULSE-003 frame 1: The same drop downward, 0 to -10: the ring draws the picture in. | largest difference 2.5e-7 | yes |
| FX-RPULSE-003 frame 4: The same drop downward, 0 to -10: the ring draws the picture in. | largest difference 2.5e-7 | yes |
| FX-RPULSE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RPULSE-004 frame 0: The level rising steadily from 0 at frame 0 to 20 at frame 4: a pulse of 5 a frame, a growing disc pushed out a pixel. | largest difference 1.9e-7 | yes |
| FX-RPULSE-004 frame 2: The level rising steadily from 0 at frame 0 to 20 at frame 4: a pulse of 5 a frame, a growing disc pushed out a pixel. | largest difference 2.0e-7 | yes |
| FX-RPULSE-004 frame 4: The level rising steadily from 0 at frame 0 to 20 at frame 4: a pulse of 5 a frame, a growing disc pushed out a pixel. | largest difference 2.5e-7 | yes |
| FX-RPULSE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RPULSE-005 frame 4: FX-RPULSE-002 centred at the left quarter, 25, 50. | largest difference 2.2e-7 | yes |
| FX-RPULSE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RPULSE-006 frame 0: FX-RPULSE-002 with Render Bump Map on: grey heights instead, the middle at 0.51 and the ring's slope down to 0.5 at frame 4. | largest difference 0.0e0 | yes |
| FX-RPULSE-006 frame 1: FX-RPULSE-002 with Render Bump Map on: grey heights instead, the middle at 0.51 and the ring's slope down to 0.5 at frame 4. | largest difference 1.0e-8 | yes |
| FX-RPULSE-006 frame 4: FX-RPULSE-002 with Render Bump Map on: grey heights instead, the middle at 0.51 and the ring's slope down to 0.5 at frame 4. | largest difference 2.6e-8 | yes |
| FX-RPULSE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RPULSE-007 frame 1: FX-RPULSE-002 with Time Span 0: no history, so no ring. | largest difference 1.9e-7 | yes |
| FX-RPULSE-007 frame 4: FX-RPULSE-002 with Time Span 0: no history, so no ring. | largest difference 1.9e-7 | yes |
| FX-RPULSE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RPULSE-008 frame 4: FX-RPULSE-002 with Time Span 1 second (24 frames): the ring runs four times slower, still near the middle at frame 4. | largest difference 1.9e-7 | yes |
| FX-RPULSE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RPULSE-009 frame 4: FX-RPULSE-002 with the layer moved three pixels right: the same, moved; nothing grows. | largest difference 2.2e-7 | yes |
| FX-RPULSE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RPULSE-010 frame 4: FX-RPULSE-002 with Amplitude 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-RPULSE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RPULSE-011 frame 2: The level eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots, Amplitude 0.01, Time Span 0.25: the level is held at 1000 before the history is read, so frame 4 is the history 0, ..., 1000, 1000. | largest difference 2.5e-7 | yes |
| FX-RPULSE-011 frame 4: The level eased from 0 at frame 0 to 1000 at frame 4 on a curve that overshoots, Amplitude 0.01, Time Span 0.25: the level is held at 1000 before the history is read, so frame 4 is the history 0, ..., 1000, 1000. | largest difference 2.5e-7 | yes |
| FX-RPULSE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RPULSE-012 frame 4: FX-RPULSE-006's bump map with Amplitude 200: the middle's grey held at white. | largest difference 2.6e-8 | yes |
| FX-RPULSE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RPULSE-013 frame 0: Pulse Level 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RPULSE-013 frame 4: Pulse Level 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RPULSE-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RPULSE-014 frame 0: Time Span 11, above 10 seconds. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RPULSE-014 frame 4: Time Span 11, above 10 seconds. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RPULSE-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RPULSE-015 frame 0: Time Span -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RPULSE-015 frame 4: Time Span -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RPULSE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RPULSE-016 frame 0: Amplitude 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RPULSE-016 frame 4: Amplitude 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RPULSE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RPULSE-017 frame 0: Render Bump Map "yes", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RPULSE-017 frame 4: Render Bump Map "yes", not a word it takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RPULSE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RPULSE-018 frame 0: Centre at 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RPULSE-018 frame 4: Centre at 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RPULSE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RPULSE-019 frame 0: Pulse Level keyed to -2000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RPULSE-019 frame 4: Pulse Level keyed to -2000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RPULSE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rpulse_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rpulse_005.json is saved with its centre, keyed pulse level, time span, amplitude and bump map | {"amplitude":2,"center":[25,50],"pulse_level":{"base":0,"keyframes":[{"frame":0,"interp":"hold","value":0},{"frame":1,"interp":"linear","value":10}]},"render_bump_map":"off","time_span":0.25} | yes |
| fx_rpulse_013.json is refused in a sentence | Ripple Pulse's pulse level runs from -1000 to 1000, and this is 1001. | yes |
| fx_rpulse_014.json is refused in a sentence | Ripple Pulse's time span runs from 0 to 10, and this is 11. | yes |
| fx_rpulse_015.json is refused in a sentence | Ripple Pulse's time span runs from 0 to 10, and this is -1. | yes |
| fx_rpulse_016.json is refused in a sentence | Ripple Pulse's amplitude runs from 0 to 1000, and this is 1001. | yes |
| fx_rpulse_017.json is refused in a sentence | Ripple Pulse's render bump map is "on" or "off", and this is "yes". | yes |
| fx_rpulse_018.json is refused in a sentence | Ripple Pulse's center runs from -1000 to 1000, and this is 1001. | yes |
| a file with a Ripple Pulse with no `time_span` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Ripple Pulse whose amplitude is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| pulse level -1000.5 is refused with a sentence, and nothing changes | Ripple Pulse's pulse level runs from -1000 to 1000, and this is -1000.5. | yes |
| time span 10.5 is refused with a sentence, and nothing changes | Ripple Pulse's time span runs from 0 to 10, and this is 10.5. | yes |
| amplitude -0.5 is refused with a sentence, and nothing changes | Ripple Pulse's amplitude runs from 0 to 1000, and this is -0.5. | yes |
| render bump map "On", written with a capital is refused with a sentence, and nothing changes | Ripple Pulse's render bump map is "on" or "off", and this is "On". | yes |
| pulse level keyed to 1500 is refused with a sentence, and nothing changes | Ripple Pulse's pulse level runs from -1000 to 1000, and this is 1500. | yes |
| centre 40, 60, level 30, span 0.5, amplitude 25, bump map is taken | taken | yes |
| pulse level keyed from 0 to 20 is taken | taken | yes |
| center keyed from 30, 50 to 70, 50 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rpulse_002.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rpulse_003.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rpulse_004.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rpulse_005.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rpulse_006.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rpulse_008.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rpulse_001.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rpulse_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_rpulse_003.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_rpulse_004.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_rpulse_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_rpulse_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rpulse_007.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rpulse_008.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_rpulse_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_rpulse_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rpulse_011.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_rpulse_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rpulse_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rpulse_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rpulse_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rpulse_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rpulse_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rpulse_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rpulse_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Ripple Pulse with a moving pulse level, time span 1, amplitude 10: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1683496 pixels changed | yes |
| the reference shot, Ripple Pulse with a moving pulse level, time span 1, amplitude 10 on three layers, frame 0, Full | largest difference 1 of 255, 1823 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ripple Pulse with a moving pulse level, time span 1, amplitude 10 on three layers, frame 100, Full | largest difference 1 of 255, 885 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ripple Pulse with a moving pulse level, time span 1, amplitude 10 on three layers, frame 239, Full | largest difference 1 of 255, 809 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ripple Pulse with a moving pulse level, time span 1, amplitude 10 on three layers, frame 0, Draft | largest difference 1 of 255, 65 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ripple Pulse with a moving pulse level, time span 1, amplitude 10 on three layers, frame 100, Draft | largest difference 1 of 255, 62 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ripple Pulse with a moving pulse level, time span 1, amplitude 10 on three layers, frame 239, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ripple Pulse the same, time span 0.25, amplitude 40, off centre: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1673734 pixels changed | yes |
| the reference shot, Ripple Pulse the same, time span 0.25, amplitude 40, off centre on three layers, frame 0, Full | largest difference 1 of 255, 1838 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ripple Pulse the same, time span 0.25, amplitude 40, off centre on three layers, frame 100, Full | largest difference 1 of 255, 916 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ripple Pulse the same, time span 0.25, amplitude 40, off centre on three layers, frame 239, Full | largest difference 1 of 255, 820 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ripple Pulse the same, time span 0.25, amplitude 40, off centre on three layers, frame 0, Draft | largest difference 1 of 255, 73 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ripple Pulse the same, time span 0.25, amplitude 40, off centre on three layers, frame 100, Draft | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ripple Pulse the same, time span 0.25, amplitude 40, off centre on three layers, frame 239, Draft | largest difference 1 of 255, 39 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ripple Pulse the same as a bump map, amplitude 3: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072792 pixels changed | yes |
| the reference shot, Ripple Pulse the same as a bump map, amplitude 3 on three layers, frame 0, Full | largest difference 1 of 255, 1026 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ripple Pulse the same as a bump map, amplitude 3 on three layers, frame 100, Full | largest difference 1 of 255, 975 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ripple Pulse the same as a bump map, amplitude 3 on three layers, frame 239, Full | largest difference 1 of 255, 936 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ripple Pulse the same as a bump map, amplitude 3 on three layers, frame 0, Draft | largest difference 1 of 255, 64 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ripple Pulse the same as a bump map, amplitude 3 on three layers, frame 100, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Ripple Pulse the same as a bump map, amplitude 3 on three layers, frame 239, Draft | largest difference 1 of 255, 66 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-390 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts, the level never moved: nothing changes; draws cleanly | [], 0 pixels changed | yes |
| 3_rings.png, a drop five frames ago, time span 0.5, amplitude 10: one ring, part way out, bending the street outward; draws cleanly | [], 15779 pixels changed | yes |
| 4_bump_map.png, the same as a bump map: the ring drawn in grey instead of the street; draws cleanly | [], 129600 pixels changed | yes |

## Result

143 of 143 checks pass.
