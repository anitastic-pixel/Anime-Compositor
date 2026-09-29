# B-134: Light Sweep

D-199, accepted on 2026-09-28 with the After Effects picks (A9). Every expected pixel is `Fixtures/light_sweep/expected_light_sweep.json`, written by `tools/light_sweep_reference.py` before this code existed and printed in document 25 as FX-SWEEP-001 to 026. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-SWEEP-001 to 026 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SWEEP-001 frame 0: The settings as they start: centre (50, 50), direction -30, smooth, width 50, sweep intensity 50, edge intensity 100, edge thickness 1, white, add. The band, wider than the frame, lights the whole figure, most along a line through the middle leaning left, and its edges most of all: the block's border, the soft edge and the lone red dot. | largest difference 3.0e-7 | yes |
| FX-SWEEP-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-002 frame 0: Width 0: no band, the drawing untouched. | largest difference 1.9e-7 | yes |
| FX-SWEEP-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-003 frame 0: Sweep intensity 0 and edge intensity 0: no light, the drawing untouched. | largest difference 1.9e-7 | yes |
| FX-SWEEP-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-004 frame 0: Direction 0, linear, width 5, sweep intensity 100, edge intensity 0: an upright band down the middle of the frame, x = 8; columns 7 and 8 gain 0.8 of white, columns 6 and 9 0.4, and the rest is untouched. | largest difference 2.2e-7 | yes |
| FX-SWEEP-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-005 frame 0: As FX-SWEEP-004, smooth: columns 7 and 8 gain 0.896 of white and columns 6 and 9 0.352. | largest difference 2.4e-7 | yes |
| FX-SWEEP-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-006 frame 0: As FX-SWEEP-004, sharp: columns 6 to 9 gain all of white and columns 5 and 10, the band's soft sides, half. | largest difference 1.9e-7 | yes |
| FX-SWEEP-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-007 frame 0: As FX-SWEEP-004, direction 90, width 3: a band lying across rows 4 and 5, which gain two thirds of white; the red dot, in row 5, is lit too. | largest difference 2.1e-7 | yes |
| FX-SWEEP-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-008 frame 0: As FX-SWEEP-004, direction 45, width 6: the band leans right, from the lower left to the upper right through the middle. | largest difference 2.3e-7 | yes |
| FX-SWEEP-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-009 frame 0: The edges alone: width 10000, sweep intensity 0, edge intensity 100. The block's outer ring of pixels, the soft edge and the red dot are lit about wholly white; the column beside the soft edge, half covered on its left, half; the block's middle is untouched. | largest difference 2.4e-7 | yes |
| FX-SWEEP-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-010 frame 0: As FX-SWEEP-009, edge thickness 2.7, counted as 2: the ring two pixels deep. | largest difference 2.4e-7 | yes |
| FX-SWEEP-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-011 frame 0: As FX-SWEEP-004 in orange #ffb040: columns 7 and 8 gain 0.8 of orange. | largest difference 2.2e-7 | yes |
| FX-SWEEP-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-012 frame 0: As FX-SWEEP-011, composite: the band paints the drawing orange, columns 7 and 8 four fifths of the way and columns 6 and 9 two fifths, never brighter than the orange. | largest difference 1.9e-7 | yes |
| FX-SWEEP-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-013 frame 0: As FX-SWEEP-004, cutout: only the band's light is left, white in columns 7 and 8 at 0.8 covering and in columns 6 and 9 at 0.4, the rest clear. | largest difference 1.2e-8 | yes |
| FX-SWEEP-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-014 frame 0: As FX-SWEEP-013, edge intensity 100: the band's cut-out light is whole where it crosses the block's top and foot, the edges adding to it, held at full; inside, 0.8 as before. | largest difference 1.2e-8 | yes |
| FX-SWEEP-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-015 frame 0: As FX-SWEEP-004, the centre keyed from (20, 50) at frame 0 to (80, 50) at frame 4, linear: the band sweeps across left to right, over the soft edge at frame 0, down the middle at frame 2 (FX-SWEEP-004) and over the red dot at frame 4. | largest difference 2.4e-7 | yes |
| FX-SWEEP-015 frame 2: As FX-SWEEP-004, the centre keyed from (20, 50) at frame 0 to (80, 50) at frame 4, linear: the band sweeps across left to right, over the soft edge at frame 0, down the middle at frame 2 (FX-SWEEP-004) and over the red dot at frame 4. | largest difference 2.2e-7 | yes |
| FX-SWEEP-015 frame 4: As FX-SWEEP-004, the centre keyed from (20, 50) at frame 0 to (80, 50) at frame 4, linear: the band sweeps across left to right, over the soft edge at frame 0, down the middle at frame 2 (FX-SWEEP-004) and over the red dot at frame 4. | largest difference 2.4e-7 | yes |
| FX-SWEEP-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-016 frame 0: As FX-SWEEP-004, the sweep intensity eased from 0 at frame 0 to 100 at frame 4 past its end: frame 0 is the drawing, and frame 2 is held at 100, FX-SWEEP-004. | largest difference 1.9e-7 | yes |
| FX-SWEEP-016 frame 2: As FX-SWEEP-004, the sweep intensity eased from 0 at frame 0 to 100 at frame 4 past its end: frame 0 is the drawing, and frame 2 is held at 100, FX-SWEEP-004. | largest difference 2.2e-7 | yes |
| FX-SWEEP-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-017 frame 0: As FX-SWEEP-007, direction 450, a turn and a quarter: FX-SWEEP-007 exactly. | largest difference 2.1e-7 | yes |
| FX-SWEEP-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-018 frame 0: As FX-SWEEP-004, the layer moved three pixels right: the band moves with it, down x = 11. | largest difference 2.2e-7 | yes |
| FX-SWEEP-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-019 frame 0: Motion Tile at 300% by 300% before it, width 7, the band's centre at (0, 50), the drawing's left edge, the layer moved eight pixels right: the band crosses from the drawing into the tile on its left, lighting the soft edge in column 10 and the tile's red dot in column 6, and nowhere else. | largest difference 1.9e-7 | yes |
| FX-SWEEP-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-020 frame 0: Width -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SWEEP-020 frame 4: Width -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SWEEP-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SWEEP-021 frame 0: Edge thickness 51, above 50. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SWEEP-021 frame 4: Edge thickness 51, above 50. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SWEEP-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SWEEP-022 frame 0: Edge thickness 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SWEEP-022 frame 4: Edge thickness 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SWEEP-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SWEEP-023 frame 0: Sweep intensity keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SWEEP-023 frame 4: Sweep intensity keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SWEEP-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SWEEP-024 frame 0: Light reception "glow", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SWEEP-024 frame 4: Light reception "glow", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SWEEP-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SWEEP-025 frame 0: Shape "round", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SWEEP-025 frame 4: Shape "round", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SWEEP-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SWEEP-026 frame 0: Light colour "#fff", written in three digits, not six. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SWEEP-026 frame 4: Light colour "#fff", written in three digits, not six. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SWEEP-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview halves the width, 50 to 25, and the edge thickness, 6 to 3, and keeps the rest | LightSweep { center: [50.0, 50.0], direction: -30.0, shape: "smooth", width: 25.0, sweep_intensity: 50.0, edge_intensity: 100.0, edge_thickness: 3.0, light_color: "#ffffff", light_reception: "add" } | yes |
| and an edge thickness of 1 stays 1, the least it can be | LightSweep { center: [50.0, 50.0], direction: -30.0, shape: "smooth", width: 25.0, sweep_intensity: 50.0, edge_intensity: 100.0, edge_thickness: 1.0, light_color: "#ffffff", light_reception: "add" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sweep_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sweep_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sweep_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sweep_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sweep_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sweep_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sweep_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sweep_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sweep_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sweep_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sweep_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `light_reception` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a centre of one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a width that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| centre x 1000.5 is refused with a sentence, and nothing changes | Light Sweep's center runs from -1000 to 1000, and this is 1000.5. | yes |
| direction -3600.5 is refused with a sentence, and nothing changes | Light Sweep's direction runs from -3600 to 3600, and this is -3600.5. | yes |
| width -1 is refused with a sentence, and nothing changes | Light Sweep's width runs from 0 to 10000, and this is -1. | yes |
| width 10000.5 is refused with a sentence, and nothing changes | Light Sweep's width runs from 0 to 10000, and this is 10000.5. | yes |
| sweep intensity 101 is refused with a sentence, and nothing changes | Light Sweep's sweep intensity runs from 0 to 100, and this is 101. | yes |
| edge intensity -0.5 is refused with a sentence, and nothing changes | Light Sweep's edge intensity runs from 0 to 100, and this is -0.5. | yes |
| edge thickness 0.9 is refused with a sentence, and nothing changes | Light Sweep's edge thickness runs from 1 to 50, and this is 0.9. | yes |
| edge thickness 50.5 is refused with a sentence, and nothing changes | Light Sweep's edge thickness runs from 1 to 50, and this is 50.5. | yes |
| shape "round" is refused with a sentence, and nothing changes | Light Sweep's shape is "linear", "smooth" or "sharp", and this is "round". | yes |
| light colour "#fff" is refused with a sentence, and nothing changes | Light Sweep's light colour is written #rrggbb, and this is "#fff". | yes |
| light reception "glow" is refused with a sentence, and nothing changes | Light Sweep's light reception is "add", "composite" or "cutout", and this is "glow". | yes |
| edge thickness keyed to 60 is refused with a sentence, and nothing changes | Light Sweep's edge thickness runs from 1 to 50, and this is 60. | yes |
| every setting at the top of its range, the last words, is taken | taken | yes |
| every setting at the bottom, the other words, is taken | taken | yes |
| the centre keyed from (0, 50) to (100, 50) is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a badge, in `verification/B-134 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the badge with no effect, draws cleanly | [] | yes |
| start.png, as it is added: a soft white band leaning left through the middle; draws cleanly; the gold middle brighter, (118, 20) off the band and the empty corner (5, 5) as they were | []; (40, 50), (42, 50), (43, 50), (48, 50), (80, 50), (112, 50), (118, 20), (5, 5): [[30, 26, 36, 255], [30, 26, 36, 255], [200, 40, 40, 255], [200, 40, 40, 255], [255, 242, 192, 255], [200, 40, 40, 255], [30, 26, 36, 255], [0, 0, 0, 0]] | yes |
| sweep_1.png, a sharp band keyed along, a third of the way; draws cleanly; the band's own spot on the middle row brighter, the other two spots of (48, 50), (80, 50) and (112, 50) as they were | []; (40, 50), (42, 50), (43, 50), (48, 50), (80, 50), (112, 50), (118, 20), (5, 5): [[255, 255, 255, 255], [220, 219, 220, 255], [255, 221, 221, 255], [255, 221, 221, 255], [224, 168, 48, 255], [200, 40, 40, 255], [30, 26, 36, 255], [0, 0, 0, 0]] | yes |
| sweep_2.png, half way; draws cleanly; the band's own spot on the middle row brighter, the other two spots of (48, 50), (80, 50) and (112, 50) as they were | []; (40, 50), (42, 50), (43, 50), (48, 50), (80, 50), (112, 50), (118, 20), (5, 5): [[30, 26, 36, 255], [30, 26, 36, 255], [200, 40, 40, 255], [200, 40, 40, 255], [255, 255, 222, 255], [200, 40, 40, 255], [30, 26, 36, 255], [0, 0, 0, 0]] | yes |
| sweep_3.png, two thirds of the way; draws cleanly; the band's own spot on the middle row brighter, the other two spots of (48, 50), (80, 50) and (112, 50) as they were | []; (40, 50), (42, 50), (43, 50), (48, 50), (80, 50), (112, 50), (118, 20), (5, 5): [[30, 26, 36, 255], [30, 26, 36, 255], [200, 40, 40, 255], [200, 40, 40, 255], [224, 168, 48, 255], [255, 221, 221, 255], [30, 26, 36, 255], [0, 0, 0, 0]] | yes |
| edges.png, the edges alone, three pixels deep; draws cleanly; the border's outer three pixels at (40, 50) and (42, 50) brighter, (43, 50) and the gold middle as they were | []; (40, 50), (42, 50), (43, 50), (48, 50), (80, 50), (112, 50), (118, 20), (5, 5): [[255, 255, 255, 255], [255, 255, 255, 255], [200, 40, 40, 255], [200, 40, 40, 255], [224, 168, 48, 255], [200, 40, 40, 255], [255, 255, 255, 255], [0, 0, 0, 0]] | yes |
| composite.png, composited in orange #ffb040; draws cleanly; the gold middle within 8 of the orange, the empty corner (5, 5) as it was | []; (40, 50), (42, 50), (43, 50), (48, 50), (80, 50), (112, 50), (118, 20), (5, 5): [[94, 65, 40, 255], [85, 59, 39, 255], [206, 70, 43, 255], [213, 92, 46, 255], [252, 175, 63, 255], [212, 90, 46, 255], [30, 26, 36, 255], [0, 0, 0, 0]] | yes |
| cutout.png, cutout: only the light is left; draws cleanly; the gold middle white, (118, 20) off the band cut away, the empty corner (5, 5) as it was | []; (40, 50), (42, 50), (43, 50), (48, 50), (80, 50), (112, 50), (118, 20), (5, 5): [[255, 255, 255, 27], [255, 255, 255, 22], [255, 255, 255, 27], [255, 255, 255, 59], [255, 255, 255, 255], [255, 255, 255, 56], [0, 0, 0, 0], [0, 0, 0, 0]] | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sweep_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_sweep_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_sweep_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

106 of 106 checks pass.
