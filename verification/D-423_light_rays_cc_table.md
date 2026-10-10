# D-423: Light Rays with CC Light Rays' controls

B-302: the light is the layer times each pixel's share of a round or square source of Radius pixels about the Center (the square turned by Direction), white at its covering with Color from Source off; the rays are that light through Spin & Zoom Blur's zoom at amount 100, then its spin of Warp Softness / 10 degrees, Intensity / 100 times (held at 1 with Allow Brightening off), laid on by the Transfer Mode: None over the layer, Add as D-124's last step, Screen, Lighten. The manual gives no formula, ranges or defaults, so those are ours. Every expected pixel is `Fixtures/light_rays_cc/expected_light_rays_cc.json`, written by `tools/light_rays_cc_reference.py` before this code existed and printed in document 25 as FX-RAYSCC-001 to 039. Tolerance 2e-5. A Light Rays saved before D-423 opens as D-124's form and draws as before.

## FX-RAYSCC-001 to 039 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RAYSCC-001 frame 0: The settings as they start: intensity 100, centre 50, 50, the point (8, 5), radius 50, warp softness 50, Round, Color from Source on, Allow Brightening on, Transfer Mode None. The source covers the whole drawing, so all of it is light: each pixel is the drawing averaged along the line from the centre out to it, turned 5 degrees both ways, laid over the drawing. The empty columns and rows round the drawing take the patches' rays; the patches show through where the rays are thin. | largest difference 2.1e-7 | yes |
| FX-RAYSCC-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-002 frame 0: Intensity 0: the drawing, untouched. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-003 frame 0: Radius 3, warp softness 0: only the pixels within 3 of the centre give light, the yellow's columns 5 to 7 and the brown's column 10, their four corner pixels only in part (the source's soft edge); the yellow's half edge in column 4, the brown's column 11 and the purple, outside, give none. Their rays streak outward, the yellow's to the left edge, the brown's to the right, laid over the drawing. | largest difference 1.7e-7 | yes |
| FX-RAYSCC-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-004 frame 0: FX-RAYSCC-003 with a Square source: the square reaches 3 each way and into its corners, so the four corner pixels the round source takes only in part give all their light. | largest difference 1.6e-7 | yes |
| FX-RAYSCC-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-005 frame 0: The square turned 45 degrees: a diamond, reaching 4.2 along the row and the column through the centre, so the brown's column 11 gives part of its light, and less to the corners. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-006 frame 0: Round with direction 45: direction does nothing for a round source; FX-RAYSCC-003. | largest difference 1.7e-7 | yes |
| FX-RAYSCC-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-007 frame 0: Warp softness 300: FX-RAYSCC-003's rays turned 30 degrees both ways, so they spread round the centre and melt together. | largest difference 2.1e-7 | yes |
| FX-RAYSCC-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-008 frame 0: Color from Source off, colour white: the light is white at the source's covering, so the yellow and the brown give white rays. | largest difference 6.8e-8 | yes |
| FX-RAYSCC-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-009 frame 0: Color from Source off, colour #ff8000, orange: orange rays, red as much as the rays' covering, a fifth as much green, no blue. | largest difference 6.8e-8 | yes |
| FX-RAYSCC-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-010 frame 0: Color from Source on with colour #ff8000: the colour does not count; FX-RAYSCC-003. | largest difference 1.7e-7 | yes |
| FX-RAYSCC-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-011 frame 0: FX-RAYSCC-009 with its colour written in capitals, #FF8000: the same. | largest difference 6.8e-8 | yes |
| FX-RAYSCC-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-012 frame 0: Transfer Mode Add: the rays added onto the drawing, D-124's way, so the lit patches brighten rather than being covered. | largest difference 3.3e-7 | yes |
| FX-RAYSCC-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-013 frame 0: Transfer Mode Screen: brighter than the drawing everywhere the rays fall, but less than Add where both are bright. | largest difference 1.2e-7 | yes |
| FX-RAYSCC-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-014 frame 0: Transfer Mode Lighten: each channel the larger of the drawing's and the rays'. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-015 frame 0: Intensity 300, Add: three times FX-RAYSCC-012's rays, past white, not cut off; the covering stops at full. | largest difference 6.3e-7 | yes |
| FX-RAYSCC-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-016 frame 0: Intensity 300, Add, Allow Brightening off: held at 100; FX-RAYSCC-012. | largest difference 3.3e-7 | yes |
| FX-RAYSCC-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-017 frame 0: Intensity 50, Allow Brightening off: below 100 nothing is held; the same as with it on. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-018 frame 0: Intensity 50, Allow Brightening on, the pair of FX-RAYSCC-017. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-019 frame 0: Radius 0: the centre sits on a pixel corner, every pixel centre at least 0.7 from it, so nothing is light; the drawing, untouched. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-020 frame 0: Centre 25, 50, the point (4, 5), radius 3: the yellow alone is the source, its rays streaking left to the edge and right across the gap and over the brown and the purple. | largest difference 2.1e-7 | yes |
| FX-RAYSCC-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-021 frame 0: Radius keyed from 0 at frame 0 to 6 at frame 4, linear: frame 0 is FX-RAYSCC-019, frame 2 is FX-RAYSCC-003, frame 4 lights all but the purple. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-021 frame 2: Radius keyed from 0 at frame 0 to 6 at frame 4, linear: frame 0 is FX-RAYSCC-019, frame 2 is FX-RAYSCC-003, frame 4 lights all but the purple. | largest difference 1.7e-7 | yes |
| FX-RAYSCC-021 frame 4: Radius keyed from 0 at frame 0 to 6 at frame 4, linear: frame 0 is FX-RAYSCC-019, frame 2 is FX-RAYSCC-003, frame 4 lights all but the purple. | largest difference 1.8e-7 | yes |
| FX-RAYSCC-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-022 frame 0: A Square source with direction keyed from 0 at frame 0 to 90 at frame 4, linear: frame 2 is FX-RAYSCC-005, and frame 4, a quarter turn of a square, is frame 0 again. | largest difference 1.6e-7 | yes |
| FX-RAYSCC-022 frame 2: A Square source with direction keyed from 0 at frame 0 to 90 at frame 4, linear: frame 2 is FX-RAYSCC-005, and frame 4, a quarter turn of a square, is frame 0 again. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-022 frame 4: A Square source with direction keyed from 0 at frame 0 to 90 at frame 4, linear: frame 2 is FX-RAYSCC-005, and frame 4, a quarter turn of a square, is frame 0 again. | largest difference 1.6e-7 | yes |
| FX-RAYSCC-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-023 frame 0: Intensity eased from 0 at frame 0 to 2000 at frame 4 on a curve that overshoots, Add: at frame 2 it would pass 2000, is held at 2000, as frame 4 is. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-023 frame 2: Intensity eased from 0 at frame 0 to 2000 at frame 4 on a curve that overshoots, Add: at frame 2 it would pass 2000, is held at 2000, as frame 4 is. | largest difference 2.8e-6 | yes |
| FX-RAYSCC-023 frame 4: Intensity eased from 0 at frame 0 to 2000 at frame 4 on a curve that overshoots, Add: at frame 2 it would pass 2000, is held at 2000, as frame 4 is. | largest difference 2.8e-6 | yes |
| FX-RAYSCC-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-024 frame 0: FX-RAYSCC-003 moved three pixels right: the rays are drawn on the drawing before it is moved, so it is FX-RAYSCC-003 moved, and the three columns left of the drawing stay empty, as the layer does not grow. | largest difference 1.7e-7 | yes |
| FX-RAYSCC-024 frame 3: FX-RAYSCC-003 moved three pixels right: the rays are drawn on the drawing before it is moved, so it is FX-RAYSCC-003 moved, and the three columns left of the drawing stay empty, as the layer does not grow. | largest difference 1.7e-7 | yes |
| FX-RAYSCC-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-025 frame 0: Intensity 250, None: the rays' colour runs past their covering, laid over the drawing; their covering stops at full. | largest difference 4.3e-7 | yes |
| FX-RAYSCC-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-026 frame 0: Square 4, direction 30, warp 120, Color from Source off #40c0ff, intensity 180, Screen, centre 40, 60: the controls together. | largest difference 1.6e-7 | yes |
| FX-RAYSCC-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-027 frame 0: Intensity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-027 frame 4: Intensity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYSCC-028 frame 0: Intensity 2001, above 2000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-028 frame 4: Intensity 2001, above 2000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYSCC-029 frame 0: Intensity keyed to 3000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-029 frame 4: Intensity keyed to 3000 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYSCC-030 frame 0: Centre 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-030 frame 4: Centre 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYSCC-031 frame 0: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-031 frame 4: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYSCC-032 frame 0: Radius 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-032 frame 4: Radius 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYSCC-033 frame 0: Warp softness 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-033 frame 4: Warp softness 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYSCC-034 frame 0: Direction 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-034 frame 4: Direction 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYSCC-035 frame 0: Shape "triangle", neither "round" nor "square". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-035 frame 4: Shape "triangle", neither "round" nor "square". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYSCC-036 frame 0: Color from Source "yes", neither "off" nor "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-036 frame 4: Color from Source "yes", neither "off" nor "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYSCC-037 frame 0: Allow Brightening "yes", neither "off" nor "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-037 frame 4: Allow Brightening "yes", neither "off" nor "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYSCC-038 frame 0: Transfer Mode "multiply", not one of the four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-038 frame 4: Transfer Mode "multiply", not one of the four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYSCC-039 frame 0: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-039 frame 4: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Older projects: D-124's form, as before

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rays_001.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_002.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_003.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_004.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_005.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_006.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_007.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_008.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_009.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_010.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_011.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_012.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_013.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_014.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_015.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_016.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_017.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_018.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_019.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_020.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_021.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_022.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_023.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| fx_rays_024.json (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings | D-124's form; saved the same | yes |
| the street, as it starts: the new form with a source over the whole layer, no warp, Add, against D-124's form at length 100, threshold 0, intensity 1 | largest difference 0 of 255, 0 pixels differ | yes |
| the street, round 30, 70: the new form with a source over the whole layer, no warp, Add, against D-124's form at length 100, threshold 0, intensity 1 | largest difference 0 of 255, 0 pixels differ | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: rays past the layer's edge are cut | 0 | yes |
| a half-size draft preview halves the radius alone: the centre is a share of the drawing, the warp and direction angles | CcLightRays { intensity: 250.0, center: [30.0, 70.0], radius: 40.0, warp_softness: 120.0, shape: "square", direction: 30.0, color_from_source: "off", allow_brightening: "off", color: "#ff8000", transfer_mode: "screen" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rayscc_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rayscc_026.json is saved with all ten settings | {"allow_brightening":"on","center":[40,60],"color":"#40c0ff","color_from_source":"off","direction":30,"intensity":180,"radius":4,"shape":"square","transfer_mode":"screen","warp_softness":120} | yes |
| fx_rayscc_011.json's colour written #FF8000 is read and saved in small letters | {"allow_brightening":"on","center":[50,50],"color":"#ff8000","color_from_source":"off","direction":0,"intensity":100,"radius":3,"shape":"round","transfer_mode":"none","warp_softness":0} | yes |
| fx_rayscc_027.json is refused in a sentence | Light Rays's intensity runs from 0 to 2000, and this is -1. | yes |
| fx_rayscc_028.json is refused in a sentence | Light Rays's intensity runs from 0 to 2000, and this is 2001. | yes |
| fx_rayscc_030.json is refused in a sentence | Light Rays's center runs from -1000 to 1000, and this is -1001. | yes |
| fx_rayscc_031.json is refused in a sentence | Light Rays's radius runs from 0 to 10000, and this is -1. | yes |
| fx_rayscc_032.json is refused in a sentence | Light Rays's radius runs from 0 to 10000, and this is 10001. | yes |
| fx_rayscc_033.json is refused in a sentence | Light Rays's warp softness runs from 0 to 1000, and this is 1001. | yes |
| fx_rayscc_034.json is refused in a sentence | Light Rays's direction runs from -3600 to 3600, and this is 3601. | yes |
| fx_rayscc_035.json is refused in a sentence | Light Rays's shape is "round" or "square", and this is "triangle". | yes |
| fx_rayscc_036.json is refused in a sentence | Light Rays's colour from source is "off" or "on", and this is "yes". | yes |
| fx_rayscc_037.json is refused in a sentence | Light Rays's allow brightening is "off" or "on", and this is "yes". | yes |
| fx_rayscc_038.json is refused in a sentence | Light Rays's transfer mode is "none", "add", "lighten" or "screen", and this is "multiply". | yes |
| fx_rayscc_039.json is refused in a sentence | Light Rays's colour is written #rrggbb, and this is "#12345". | yes |
| a file with a Light Rays with no `center` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Light Rays with one number for its centre is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Light Rays with no `radius` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Light Rays with its shape as a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| radius 10001 is refused with a sentence, and nothing changes | Light Rays's radius runs from 0 to 10000, and this is 10001. | yes |
| intensity 2001 is refused with a sentence, and nothing changes | Light Rays's intensity runs from 0 to 2000, and this is 2001. | yes |
| warp softness -1 is refused with a sentence, and nothing changes | Light Rays's warp softness runs from 0 to 1000, and this is -1. | yes |
| direction -3601 is refused with a sentence, and nothing changes | Light Rays's direction runs from -3600 to 3600, and this is -3601. | yes |
| centre 50, 1001 is refused with a sentence, and nothing changes | Light Rays's center runs from -1000 to 1000, and this is 1001. | yes |
| shape "triangle" is refused with a sentence, and nothing changes | Light Rays's shape is "round" or "square", and this is "triangle". | yes |
| transfer mode "multiply" is refused with a sentence, and nothing changes | Light Rays's transfer mode is "none", "add", "lighten" or "screen", and this is "multiply". | yes |
| colour "orange" is refused with a sentence, and nothing changes | Light Rays's colour is written #rrggbb, and this is "orange". | yes |
| radius keyed to 20000 is refused with a sentence, and nothing changes | Light Rays's radius runs from 0 to 10000, and this is 20000. | yes |
| the tops: intensity 2000, radius 10000, warp 1000, direction 3600, centre 1000, 1000 is taken | taken | yes |
| the bottoms: intensity 0, radius 0, warp 0, direction -3600, centre -1000, -1000 is taken | taken | yes |
| radius keyed from 0 to 100 is taken | taken | yes |
| direction keyed from 0 to 90 is taken | taken | yes |
| centre keyed from 50, 50 to 0, 0 is taken | taken | yes |
| undo 5 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rayscc_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rayscc_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rayscc_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rayscc_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rayscc_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rayscc_022.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rayscc_026.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rayscc_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rayscc_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_rayscc_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_025.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_rayscc_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rayscc_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rayscc_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rayscc_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rayscc_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rayscc_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rayscc_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rayscc_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rayscc_035.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rayscc_036.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rayscc_037.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rayscc_038.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_rayscc_039.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Light Rays as it starts (round, radius 50, warp 50, none): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1960091 pixels changed | yes |
| the reference shot, Light Rays as it starts (round, radius 50, warp 50, none) on three layers, frame 0, Full | largest difference 1 of 255, 987 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays as it starts (round, radius 50, warp 50, none) on three layers, frame 100, Full | largest difference 1 of 255, 1027 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays as it starts (round, radius 50, warp 50, none) on three layers, frame 239, Full | largest difference 1 of 255, 961 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays as it starts (round, radius 50, warp 50, none) on three layers, frame 0, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays as it starts (round, radius 50, warp 50, none) on three layers, frame 100, Draft | largest difference 1 of 255, 74 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays as it starts (round, radius 50, warp 50, none) on three layers, frame 239, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays radius 400, warp 300, Add: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2003192 pixels changed | yes |
| the reference shot, Light Rays radius 400, warp 300, Add on three layers, frame 0, Full | largest difference 1 of 255, 915 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays radius 400, warp 300, Add on three layers, frame 100, Full | largest difference 1 of 255, 874 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays radius 400, warp 300, Add on three layers, frame 239, Full | largest difference 1 of 255, 875 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays radius 400, warp 300, Add on three layers, frame 0, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays radius 400, warp 300, Add on three layers, frame 100, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays radius 400, warp 300, Add on three layers, frame 239, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays square 300 turned 30, Color from Source off #ff8000, intensity 250, Screen: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2003200 pixels changed | yes |
| the reference shot, Light Rays square 300 turned 30, Color from Source off #ff8000, intensity 250, Screen on three layers, frame 0, Full | largest difference 1 of 255, 2648 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays square 300 turned 30, Color from Source off #ff8000, intensity 250, Screen on three layers, frame 100, Full | largest difference 1 of 255, 183 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays square 300 turned 30, Color from Source off #ff8000, intensity 250, Screen on three layers, frame 239, Full | largest difference 1 of 255, 702 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays square 300 turned 30, Color from Source off #ff8000, intensity 250, Screen on three layers, frame 0, Draft | largest difference 1 of 255, 128 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays square 300 turned 30, Color from Source off #ff8000, intensity 250, Screen on three layers, frame 100, Draft | largest difference 1 of 255, 29 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays square 300 turned 30, Color from Source off #ff8000, intensity 250, Screen on three layers, frame 239, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays Lighten, intensity 300 held at 100, radius 600, round 30, 40: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1621189 pixels changed | yes |
| the reference shot, Light Rays Lighten, intensity 300 held at 100, radius 600, round 30, 40 on three layers, frame 0, Full | largest difference 1 of 255, 648 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays Lighten, intensity 300 held at 100, radius 600, round 30, 40 on three layers, frame 100, Full | largest difference 1 of 255, 790 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays Lighten, intensity 300 held at 100, radius 600, round 30, 40 on three layers, frame 239, Full | largest difference 1 of 255, 566 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays Lighten, intensity 300 held at 100, radius 600, round 30, 40 on three layers, frame 0, Draft | largest difference 1 of 255, 41 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays Lighten, intensity 300 held at 100, radius 600, round 30, 40 on three layers, frame 100, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Light Rays Lighten, intensity 300 held at 100, radius 600, round 30, 40 on three layers, frame 239, Draft | largest difference 1 of 255, 30 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-423 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: a round source 50 pixels about the middle, its rays streaming outward in the street's colours, laid over it; draws cleanly | [], 128510 pixels changed | yes |
| 3_big_source.png, radius 300, warp 200: a bigger source, longer and brighter rays, melted together; draws cleanly | [], 129557 pixels changed | yes |
| 4_square.png, a square source 200 turned 30 degrees, no warp: the rays fan out from a turned square; draws cleanly | [], 129527 pixels changed | yes |
| 5_orange_screen.png, Color from Source off #ff8000, radius 250, screen, round 30, 40: orange rays screened over the street; draws cleanly | [], 129600 pixels changed | yes |
| 6_add_bright.png, intensity 300, radius 250, add: the rays added three times, the middle blown out; draws cleanly | [], 129600 pixels changed | yes |

## Result

277 of 277 checks pass.
