# D-411: Heat Shimmer

B-290: PLUGINS.md's pick #17, merged into `core.turbulent_displace` as a drift ("if anything", PLUGINS.md says): Drift Direction (`drift_direction`, -3600 to 3600 degrees, 0 up, 90 right) and Drift Speed (`drift_speed`, 0 to 1000 pixels a frame), both keyable. On frame f the field's point is D-127's moved back by v f (sin a, -cos a), so the ripple slides that way. Speed 0, what a file without them means, is D-127's and D-328's rule. Every expected pixel is `Fixtures/heat_shimmer/expected_heat_shimmer.json`, written by `tools/heat_shimmer_reference.py` before this code existed and printed in document 25 as FX-SHIMMER-001 to 016. Tolerance 2e-5.

## FX-SHIMMER-001 to 016 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHIMMER-001 frame 0: Drift up (direction 0) at 1 pixel a frame, amount 30, size 8, speed 0: frame 0 is FX-TURB-AE-007; on frame f each pixel gets the warp the pixel f rows below it got on frame 0, so the ripple rises a pixel a frame. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-001 frame 1: Drift up (direction 0) at 1 pixel a frame, amount 30, size 8, speed 0: frame 0 is FX-TURB-AE-007; on frame f each pixel gets the warp the pixel f rows below it got on frame 0, so the ripple rises a pixel a frame. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-001 frame 2: Drift up (direction 0) at 1 pixel a frame, amount 30, size 8, speed 0: frame 0 is FX-TURB-AE-007; on frame f each pixel gets the warp the pixel f rows below it got on frame 0, so the ripple rises a pixel a frame. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-001 frame 3: Drift up (direction 0) at 1 pixel a frame, amount 30, size 8, speed 0: frame 0 is FX-TURB-AE-007; on frame f each pixel gets the warp the pixel f rows below it got on frame 0, so the ripple rises a pixel a frame. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIMMER-002 frame 0: Drift speed 0 written, direction 45: no drift, every frame FX-TURB-AE-007. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-002 frame 2: Drift speed 0 written, direction 45: no drift, every frame FX-TURB-AE-007. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-002 frame 4: Drift speed 0 written, direction 45: no drift, every frame FX-TURB-AE-007. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIMMER-003 frame 0: Drift right (direction 90) at 2 pixels a frame: the ripple slides two columns right each frame. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-003 frame 1: Drift right (direction 90) at 2 pixels a frame: the ripple slides two columns right each frame. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-003 frame 2: Drift right (direction 90) at 2 pixels a frame: the ripple slides two columns right each frame. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIMMER-004 frame 1: Drift down (direction 180) at half a pixel a frame: on frame 1 the field is read half a row up, between whole rows. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-004 frame 2: Drift down (direction 180) at half a pixel a frame: on frame 1 the field is read half a row up, between whole rows. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-004 frame 3: Drift down (direction 180) at half a pixel a frame: on frame 1 the field is read half a row up, between whole rows. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIMMER-005 frame 1: Drift at 30 degrees, 3 pixels a frame: up and to the right, 1.5 pixels right and about 2.6 up each frame. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-005 frame 2: Drift at 30 degrees, 3 pixels a frame: up and to the right, 1.5 pixels right and about 2.6 up each frame. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIMMER-006 frame 0: Drift up at 1 pixel a frame with speed 20: the ripple rises and changes as it goes, the usual heat shimmer. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-006 frame 1: Drift up at 1 pixel a frame with speed 20: the ripple rises and changes as it goes, the usual heat shimmer. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-006 frame 2: Drift up at 1 pixel a frame with speed 20: the ripple rises and changes as it goes, the usual heat shimmer. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-006 frame 3: Drift up at 1 pixel a frame with speed 20: the ripple rises and changes as it goes, the usual heat shimmer. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIMMER-007 frame 1: Drift speed keyed from 0 at frame 0 to 4 at frame 4, linear, up: frame f drifts f times f pixels (frame 2: 2 a frame, 4 pixels; frame 3: 9 pixels). | largest difference 2.5e-7 | yes |
| FX-SHIMMER-007 frame 2: Drift speed keyed from 0 at frame 0 to 4 at frame 4, linear, up: frame f drifts f times f pixels (frame 2: 2 a frame, 4 pixels; frame 3: 9 pixels). | largest difference 2.6e-7 | yes |
| FX-SHIMMER-007 frame 3: Drift speed keyed from 0 at frame 0 to 4 at frame 4, linear, up: frame f drifts f times f pixels (frame 2: 2 a frame, 4 pixels; frame 3: 9 pixels). | largest difference 2.6e-7 | yes |
| FX-SHIMMER-007 frame 4: Drift speed keyed from 0 at frame 0 to 4 at frame 4, linear, up: frame f drifts f times f pixels (frame 2: 2 a frame, 4 pixels; frame 3: 9 pixels). | largest difference 2.5e-7 | yes |
| FX-SHIMMER-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIMMER-008 frame 2: Drift direction keyed from 0 at frame 0 to 90 at frame 4, 2 pixels a frame: frame 2 drifts 4 pixels at 45 degrees, frame 4 8 pixels right. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-008 frame 4: Drift direction keyed from 0 at frame 0 to 90 at frame 4, 2 pixels a frame: frame 2 drifts 4 pixels at 45 degrees, frame 4 8 pixels right. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIMMER-009 frame 0: Drift up at 1 pixel a frame in a file without units (D-127's classic push), amount 3, size 8, speed 0: frame 0 is FX-TURB-004. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-009 frame 2: Drift up at 1 pixel a frame in a file without units (D-127's classic push), amount 3, size 8, speed 0: frame 0 is FX-TURB-004. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIMMER-010 frame 0: FX-SHIMMER-001 moved three pixels right: the drift is in the drawing's own space, so the picture is the same, moved. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-010 frame 2: FX-SHIMMER-001 moved three pixels right: the drift is in the drawing's own space, so the picture is the same, moved. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIMMER-011 frame 0: FX-SHIMMER-001 with edges repeat: a push past the edge reads the nearest edge pixel; the ripple drifts as before. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-011 frame 2: FX-SHIMMER-001 with edges repeat: a push past the edge reads the nearest edge pixel; the ripple drifts as before. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIMMER-012 frame 1: Drift up at 1 pixel a frame with a new seed every 2 frames (D-410): the seed steps on frame 2 and the drift goes on. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-012 frame 2: Drift up at 1 pixel a frame with a new seed every 2 frames (D-410): the seed steps on frame 2 and the drift goes on. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-012 frame 3: Drift up at 1 pixel a frame with a new seed every 2 frames (D-410): the seed steps on frame 2 and the drift goes on. | largest difference 2.5e-7 | yes |
| FX-SHIMMER-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIMMER-013 frame 0: Drift speed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHIMMER-013 frame 4: Drift speed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHIMMER-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHIMMER-014 frame 0: Drift speed 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHIMMER-014 frame 4: Drift speed 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHIMMER-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHIMMER-015 frame 0: Drift direction 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHIMMER-015 frame 4: Drift direction 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHIMMER-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHIMMER-016 frame 0: Drift speed keyed from 1 at frame 0 to 2000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHIMMER-016 frame 4: Drift speed keyed from 1 at frame 0 to 2000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHIMMER-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Older projects: no drift, as before

| Check | The build's answer | Matches |
| --- | --- | --- |
| turbulent_displace/fx_turb_001.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_002.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_003.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_004.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_005.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_006.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_007.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_008.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_009.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_010.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_011.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_012.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_013.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_014.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_015.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_016.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_017.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_018.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_019.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_020.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_021.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_022.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_023.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_024.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_025.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_026.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_001.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_002.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_003.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_004.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_005.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_006.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_007.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_008.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_009.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_010.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_011.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_012.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| line_boil/fx_boil_001.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| line_boil/fx_boil_002.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| line_boil/fx_boil_003.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| line_boil/fx_boil_004.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| line_boil/fx_boil_005.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| line_boil/fx_boil_006.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| line_boil/fx_boil_007.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| line_boil/fx_boil_008.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| line_boil/fx_boil_009.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| line_boil/fx_boil_010.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| line_boil/fx_boil_011.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| line_boil/fx_boil_012.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |
| line_boil/fx_boil_013.json (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0 | saved the same; bit-identical | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shimmer_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shimmer_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shimmer_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shimmer_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shimmer_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shimmer_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shimmer_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shimmer_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shimmer_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shimmer_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shimmer_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shimmer_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shimmer_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shimmer_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shimmer_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shimmer_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shimmer_003.json is saved with Drift Direction 90 and Drift Speed 2 | {"amount":30,"complexity":2,"drift_direction":90,"drift_speed":2,"edges":"transparent","evolution":0,"seed":0,"size":8,"speed":0,"units":"after_effects"} | yes |
| fx_shimmer_013.json (Drift Speed -1) is refused in a sentence naming drift speed | Turbulent Displace's drift speed runs from 0 to 1000, and this is -1. | yes |
| fx_shimmer_014.json (Drift Speed 1001) is refused in a sentence naming drift speed | Turbulent Displace's drift speed runs from 0 to 1000, and this is 1001. | yes |
| fx_shimmer_015.json (Drift Direction 3601) is refused in a sentence naming drift direction | Turbulent Displace's drift direction runs from -3600 to 3600, and this is 3601. | yes |
| a file with a Turbulent Displace whose Drift Speed is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| a half-size draft preview halves Drift Speed, a distance a frame, and leaves its direction | TurbulentDisplace { amount: 30.0, size: 4.0, complexity: 2.0, evolution: 0.0, speed: 0.0, seed: 0.0, edges: "transparent", frame: 0, displacement: "turbulent", pinning: "none", units: "after_effects", new_seed_every: 0.0, drift_direction: 90.0, drift_speed: 1.0 } | yes |
| the drift leaves the drawing's bounds as they were | 3 against 3 without the drift | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| drift speed 1001 is refused with a sentence, and nothing changes | Turbulent Displace's drift speed runs from 0 to 1000, and this is 1001. | yes |
| drift direction 3601 is refused with a sentence, and nothing changes | Turbulent Displace's drift direction runs from -3600 to 3600, and this is 3601. | yes |
| drift speed keyed to 2000 is refused with a sentence, and nothing changes | Turbulent Displace's drift speed runs from 0 to 1000, and this is 2000. | yes |
| drift right at 2 pixels a frame is taken | taken | yes |
| drift speed keyed from 0 to 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shimmer_001.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shimmer_004.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shimmer_005.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shimmer_006.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shimmer_007.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shimmer_010.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shimmer_011.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shimmer_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shimmer_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shimmer_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shimmer_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shimmer_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shimmer_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shimmer_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shimmer_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shimmer_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shimmer_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shimmer_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shimmer_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shimmer_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shimmer_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shimmer_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shimmer_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Turbulent Displace amount 30, size 40, drifting up at 4 pixels a frame: the processor's frame 102 differs from the same warp without the drift, so the comparisons below test the drift | 1675751 pixels changed | yes |
| the reference shot, Turbulent Displace amount 30, size 40, drifting up at 4 pixels a frame on three layers, frame 0, Full | largest difference 1 of 255, 2278 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, drifting up at 4 pixels a frame on three layers, frame 1, Full | largest difference 1 of 255, 2293 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, drifting up at 4 pixels a frame on three layers, frame 100, Full | largest difference 1 of 255, 972 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, drifting up at 4 pixels a frame on three layers, frame 102, Full | largest difference 1 of 255, 1066 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, drifting up at 4 pixels a frame on three layers, frame 239, Full | largest difference 1 of 255, 958 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, drifting up at 4 pixels a frame on three layers, frame 0, Draft | largest difference 1 of 255, 73 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, drifting up at 4 pixels a frame on three layers, frame 1, Draft | largest difference 1 of 255, 74 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, drifting up at 4 pixels a frame on three layers, frame 100, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, drifting up at 4 pixels a frame on three layers, frame 102, Draft | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, drifting up at 4 pixels a frame on three layers, frame 239, Draft | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, a new seed every 3: the processor's frame 102 differs from the same warp without the drift, so the comparisons below test the drift | 1642853 pixels changed | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, a new seed every 3 on three layers, frame 0, Full | largest difference 1 of 255, 2064 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, a new seed every 3 on three layers, frame 1, Full | largest difference 1 of 255, 2094 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, a new seed every 3 on three layers, frame 100, Full | largest difference 1 of 255, 1079 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, a new seed every 3 on three layers, frame 102, Full | largest difference 1 of 255, 1102 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, a new seed every 3 on three layers, frame 239, Full | largest difference 1 of 255, 1031 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, a new seed every 3 on three layers, frame 0, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, a new seed every 3 on three layers, frame 1, Draft | largest difference 1 of 255, 81 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, a new seed every 3 on three layers, frame 100, Draft | largest difference 1 of 255, 57 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, a new seed every 3 on three layers, frame 102, Draft | largest difference 1 of 255, 76 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, a new seed every 3 on three layers, frame 239, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-411 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_rising_frame_0.png, 3_rising_frame_2.png and 4_rising_frame_4.png, amount 8, size 20, drifting up at 6 pixels a frame: frame 0 the warp without drift, frames 2 and 4 the ripple moved up 12 and 24 pixels, each different; without the drift frame 4 is frame 0; all draw cleanly | pixels differing: frame 0 from no drift 0, 0 to 2 23964, 2 to 4 23705; without drift 0 to 4 0 | yes |
| 5_shimmer_speed_20_frame_4.png, the same with speed 20, frame 4: the rising ripple also changes as it goes; draws cleanly | []; 10461 pixels differ from 4_rising_frame_4.png | yes |

## Result

186 of 186 checks pass.
