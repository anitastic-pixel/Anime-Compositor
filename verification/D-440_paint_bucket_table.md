# D-440: Paint Bucket

B-320, after After Effects' Paint Bucket: the pixel holding the Fill Point (per cent of the drawing); the pixels that match it within Tolerance by the Fill Selector (Color & Alpha, Straight Color, Transparency, Opacity or Alpha Channel), joined to it up, down, left and right (every match for Alpha Channel), turned over by Invert Fill; spread, choked or stroked by distances between pixel centres; blurred by a box three pixels wide (or Feather Softness's Gaussian for Feather), edges held; then the colour laid on by the blending mode at Opacity (Fill Only the fill alone). View Threshold shows the matches white on black. The layer never grows. Every expected pixel is `Fixtures/paint_bucket/expected_paint_bucket.json`, written by `tools/paint_bucket_reference.py` before this code existed and printed in document 25 as FX-PAINTBUCKET-001 to 045. Tolerance 2e-5.

## FX-PAINTBUCKET-001 to 045 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PAINTBUCKET-001 frame 0: The settings as they start: the point in the middle on the skin, Color & Alpha, tolerance 10, Antialias, red, opacity 100, normal: the skin inside the line, columns 3 to 9 and rows 2 to 7, turns red, its edge softened a pixel into the line and the shadow. | largest difference 1.5e-7 | yes |
| FX-PAINTBUCKET-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-002 frame 0: Tolerance 25: the shadow, 54 levels from the skin at most, matches too, so the whole inside of the line turns red. | largest difference 1.2e-7 | yes |
| FX-PAINTBUCKET-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-003 frame 0: The point on the line, Color & Alpha: the line's box turns red; its half-covered left edge differs by half in alpha and stays. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-004 frame 0: The same point, Straight Color: the soft edge has the line's own colour, so it is filled too, red only as far as it shows. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-005 frame 0: Transparency with the point in the empty corner: the empty border all round the cel turns red and opaque. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-006 frame 0: Transparency with the point on the skin: the point must be in a clear place, so nothing is filled: the cel as it was. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-007 frame 0: Opacity with the point on the skin: every opaque pixel joined to it, line, skin and shadow, turns red; the soft edge stays. | largest difference 5.7e-8 | yes |
| FX-PAINTBUCKET-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-008 frame 0: Alpha Channel with the point on the skin: every pixel as opaque as the skin, the same as FX-PAINTBUCKET-007 here. | largest difference 5.7e-8 | yes |
| FX-PAINTBUCKET-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-009 frame 0: Invert Fill: everything but the skin turns red, the empty border included. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-010 frame 0: View Threshold: the skin white, everything else black, all opaque. | largest difference 0.0e0 | yes |
| FX-PAINTBUCKET-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-011 frame 0: View Threshold at tolerance 25: skin and shadow white. | largest difference 0.0e0 | yes |
| FX-PAINTBUCKET-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-012 frame 0: Feather, softness 2: the red skin's edge fades over a few pixels. | largest difference 1.8e-7 | yes |
| FX-PAINTBUCKET-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-013 frame 0: Spread, radius 1: the red reaches a pixel further, over the line and into the shadow, then is softened. | largest difference 1.5e-7 | yes |
| FX-PAINTBUCKET-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-014 frame 0: Choke, radius 1: the red pulls a pixel in from the line and the shadow. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-015 frame 0: Stroke, width 1: only the skin's outer ring of pixels turns red. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-016 frame 0: Spread radius 0: nothing added, FX-PAINTBUCKET-001's frame. | largest difference 1.5e-7 | yes |
| FX-PAINTBUCKET-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-017 frame 0: Fill Only: the red fill alone, the rest of the layer clear. | largest difference 7.9e-8 | yes |
| FX-PAINTBUCKET-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-018 frame 0: Blue, multiply: the skin darkened toward blue. | largest difference 1.5e-7 | yes |
| FX-PAINTBUCKET-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-019 frame 0: Blue, screen: the skin lightened toward blue. | largest difference 2.0e-7 | yes |
| FX-PAINTBUCKET-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-020 frame 0: Opacity 50: half way from the skin to red. | largest difference 1.5e-7 | yes |
| FX-PAINTBUCKET-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-021 frame 0: Opacity 0: the cel exactly as it was. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-022 frame 0: The point keyed from (25, 50) at frame 0 to (75, 50) at frame 4: the skin at frames 0 and 2, the shadow at frame 4. | largest difference 1.5e-7 | yes |
| FX-PAINTBUCKET-022 frame 2: The point keyed from (25, 50) at frame 0 to (75, 50) at frame 4: the skin at frames 0 and 2, the shadow at frame 4. | largest difference 1.5e-7 | yes |
| FX-PAINTBUCKET-022 frame 4: The point keyed from (25, 50) at frame 0 to (75, 50) at frame 4: the skin at frames 0 and 2, the shadow at frame 4. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-023 frame 0: Tolerance keyed from 10 to 30: skin only at frames 0 and 2 (20), skin and shadow at frame 4. | largest difference 1.5e-7 | yes |
| FX-PAINTBUCKET-023 frame 2: Tolerance keyed from 10 to 30: skin only at frames 0 and 2 (20), skin and shadow at frame 4. | largest difference 1.5e-7 | yes |
| FX-PAINTBUCKET-023 frame 4: Tolerance keyed from 10 to 30: skin only at frames 0 and 2 (20), skin and shadow at frame 4. | largest difference 1.2e-7 | yes |
| FX-PAINTBUCKET-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-024 frame 0: Opacity keyed from 40 at frame 0 to 100 at frame 4 by an ease that passes its end: held at 100 at frame 2, FX-PAINTBUCKET-001's frame. | largest difference 2.0e-7 | yes |
| FX-PAINTBUCKET-024 frame 2: Opacity keyed from 40 at frame 0 to 100 at frame 4 by an ease that passes its end: held at 100 at frame 2, FX-PAINTBUCKET-001's frame. | largest difference 1.5e-7 | yes |
| FX-PAINTBUCKET-024 frame 4: Opacity keyed from 40 at frame 0 to 100 at frame 4 by an ease that passes its end: held at 100 at frame 2, FX-PAINTBUCKET-001's frame. | largest difference 1.5e-7 | yes |
| FX-PAINTBUCKET-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-025 frame 0: FX-PAINTBUCKET-001 moved three pixels right: the layer's own pixels move, the three columns it left are empty. | largest difference 1.5e-7 | yes |
| FX-PAINTBUCKET-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-026 frame 0: After a Motion Tile that grows the layer: the point is the drawing's own, and the skin is closed in by its line, so the frame is FX-PAINTBUCKET-001's. | largest difference 1.5e-7 | yes |
| FX-PAINTBUCKET-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-027 frame 0: The point past the layer's left edge, (-50, 50), with Invert Fill: no pixel matches, so the whole layer turns red and opaque. | largest difference 0.0e0 | yes |
| FX-PAINTBUCKET-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-028 frame 0: Straight Color on the line with Fill Only: the fill alone, the soft edge at half covering. | largest difference 7.9e-8 | yes |
| FX-PAINTBUCKET-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-029 frame 0: Choke, radius 2: only the skin's middle, columns 5 to 7 and rows 4 and 5, is filled, then softened. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-030 frame 0: Blue, add: the skin lifted by blue, held at 1. | largest difference 2.0e-7 | yes |
| FX-PAINTBUCKET-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-031 frame 0: Transparency in the empty corner at tolerance 60: the soft edge, half covered, matches too. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-PAINTBUCKET-032 frame 0: Fill Point across 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-032 frame 4: Fill Point across 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAINTBUCKET-033 frame 0: Tolerance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-033 frame 4: Tolerance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAINTBUCKET-034 frame 0: Tolerance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-034 frame 4: Tolerance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAINTBUCKET-035 frame 0: Spread Radius 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-035 frame 4: Spread Radius 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAINTBUCKET-036 frame 0: Stroke Width -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-036 frame 4: Stroke Width -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAINTBUCKET-037 frame 0: Feather Softness 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-037 frame 4: Feather Softness 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAINTBUCKET-038 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-038 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAINTBUCKET-039 frame 0: Fill Selector "luma", not one of the five. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-039 frame 4: Fill Selector "luma", not one of the five. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAINTBUCKET-040 frame 0: Stroke "glow", not one of the five. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-040 frame 4: Stroke "glow", not one of the five. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAINTBUCKET-041 frame 0: View Threshold "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-041 frame 4: View Threshold "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAINTBUCKET-042 frame 0: Invert Fill "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-042 frame 4: Invert Fill "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAINTBUCKET-043 frame 0: Blending mode "darken", which this program does not have. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-043 frame 4: Blending mode "darken", which this program does not have. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-043: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAINTBUCKET-044 frame 0: Colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-044 frame 4: Colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-044: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-PAINTBUCKET-045 frame 0: Tolerance keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-045 frame 4: Tolerance keyed to 101 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-PAINTBUCKET-045: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_paintbucket_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_paintbucket_018.json is saved with its words and numbers as written | {"blending_mode":"multiply","color":"#3080ff","feather_softness":10,"fill_point":[50,50],"fill_selector":"color_and_alpha","invert_fill":"off","opacity":100,"spread_radius":3,"stroke":"antialias","stroke_width":3,"tolerance":10,"view_threshold":"off"} | yes |
| fx_paintbucket_032.json is refused in a sentence | Paint Bucket's fill point runs from -1000 to 1000, and this is 1001. | yes |
| fx_paintbucket_033.json is refused in a sentence | Paint Bucket's tolerance runs from 0 to 100, and this is -1. | yes |
| fx_paintbucket_034.json is refused in a sentence | Paint Bucket's tolerance runs from 0 to 100, and this is 101. | yes |
| fx_paintbucket_035.json is refused in a sentence | Paint Bucket's spread radius runs from 0 to 10000, and this is 10001. | yes |
| fx_paintbucket_036.json is refused in a sentence | Paint Bucket's stroke width runs from 0 to 10000, and this is -1. | yes |
| fx_paintbucket_037.json is refused in a sentence | Paint Bucket's feather softness runs from 0 to 10000, and this is 10001. | yes |
| fx_paintbucket_038.json is refused in a sentence | Paint Bucket's opacity runs from 0 to 100, and this is 101. | yes |
| fx_paintbucket_039.json is refused in a sentence | Paint Bucket's fill selector is one of color_and_alpha, straight_color, transparency, opacity, alpha_channel, and this is "luma". | yes |
| fx_paintbucket_040.json is refused in a sentence | Paint Bucket's stroke is one of antialias, feather, spread, choke, stroke, and this is "glow". | yes |
| fx_paintbucket_041.json is refused in a sentence | Paint Bucket's view threshold is "off" or "on", and this is "yes". | yes |
| fx_paintbucket_042.json is refused in a sentence | Paint Bucket's invert fill is "off" or "on", and this is "yes". | yes |
| fx_paintbucket_043.json is refused in a sentence | Paint Bucket's blending mode is one of normal, add, multiply, screen, overlay, soft_light, fill_only, and this is "darken". | yes |
| fx_paintbucket_044.json is refused in a sentence | Paint Bucket's colour is written #rrggbb, and this is "#12345". | yes |
| a file with a Paint Bucket with no `tolerance` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Paint Bucket whose fill point is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Paint Bucket whose invert fill is true is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| fill point 1001, 50 is refused with a sentence, and nothing changes | Paint Bucket's fill point runs from -1000 to 1000, and this is 1001. | yes |
| tolerance 101 is refused with a sentence, and nothing changes | Paint Bucket's tolerance runs from 0 to 100, and this is 101. | yes |
| spread radius -1 is refused with a sentence, and nothing changes | Paint Bucket's spread radius runs from 0 to 10000, and this is -1. | yes |
| fill selector "luma" is refused with a sentence, and nothing changes | Paint Bucket's fill selector is one of color_and_alpha, straight_color, transparency, opacity, alpha_channel, and this is "luma". | yes |
| stroke "glow" is refused with a sentence, and nothing changes | Paint Bucket's stroke is one of antialias, feather, spread, choke, stroke, and this is "glow". | yes |
| blending mode "darken" is refused with a sentence, and nothing changes | Paint Bucket's blending mode is one of normal, add, multiply, screen, overlay, soft_light, fill_only, and this is "darken". | yes |
| colour "red" is refused with a sentence, and nothing changes | Paint Bucket's colour is written #rrggbb, and this is "red". | yes |
| tolerance keyed to 101 is refused with a sentence, and nothing changes | Paint Bucket's tolerance runs from 0 to 100, and this is 101. | yes |
| point 15.625, 55, Straight Color, tolerance 20, Stroke width 2, blue, screen is taken | taken | yes |
| fill point keyed from 50, 50 to 25, 50 is taken | taken | yes |
| feather softness keyed from 0 to 6 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_paintbucket_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_paintbucket_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_paintbucket_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_paintbucket_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_paintbucket_022.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_paintbucket_026.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_paintbucket_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_032.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_033.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_034.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_035.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_036.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_037.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_038.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_039.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_040.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_041.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_042.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_043.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_044.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_paintbucket_045.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Paint Bucket as added (the middle): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2004712 pixels changed | yes |
| the reference shot, Paint Bucket as added (the middle) on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket as added (the middle) on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket as added (the middle) on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket as added (the middle) on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket as added (the middle) on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket as added (the middle) on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket point 30, 40, Straight Color, tolerance 30, Feather 8, blue, multiply: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 530444 pixels changed | yes |
| the reference shot, Paint Bucket point 30, 40, Straight Color, tolerance 30, Feather 8, blue, multiply on three layers, frame 0, Full | largest difference 1 of 255, 3331 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket point 30, 40, Straight Color, tolerance 30, Feather 8, blue, multiply on three layers, frame 100, Full | largest difference 1 of 255, 354 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket point 30, 40, Straight Color, tolerance 30, Feather 8, blue, multiply on three layers, frame 239, Full | largest difference 1 of 255, 1123 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket point 30, 40, Straight Color, tolerance 30, Feather 8, blue, multiply on three layers, frame 0, Draft | largest difference 1 of 255, 131 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket point 30, 40, Straight Color, tolerance 30, Feather 8, blue, multiply on three layers, frame 100, Draft | largest difference 1 of 255, 34 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket point 30, 40, Straight Color, tolerance 30, Feather 8, blue, multiply on three layers, frame 239, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Transparency at the corner, tolerance 50, Spread 6: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2013448 pixels changed | yes |
| the reference shot, Paint Bucket Transparency at the corner, tolerance 50, Spread 6 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Transparency at the corner, tolerance 50, Spread 6 on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Transparency at the corner, tolerance 50, Spread 6 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Transparency at the corner, tolerance 50, Spread 6 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Transparency at the corner, tolerance 50, Spread 6 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Transparency at the corner, tolerance 50, Spread 6 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Alpha Channel, Invert Fill, Fill Only, opacity 60: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Paint Bucket Alpha Channel, Invert Fill, Fill Only, opacity 60 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Alpha Channel, Invert Fill, Fill Only, opacity 60 on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Alpha Channel, Invert Fill, Fill Only, opacity 60 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Alpha Channel, Invert Fill, Fill Only, opacity 60 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Alpha Channel, Invert Fill, Fill Only, opacity 60 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Alpha Channel, Invert Fill, Fill Only, opacity 60 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Opacity, Stroke width 4, View Threshold: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Paint Bucket Opacity, Stroke width 4, View Threshold on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Opacity, Stroke width 4, View Threshold on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Opacity, Stroke width 4, View Threshold on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Opacity, Stroke width 4, View Threshold on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Opacity, Stroke width 4, View Threshold on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Opacity, Stroke width 4, View Threshold on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Choke 5, tolerance 40, soft light: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1979264 pixels changed | yes |
| the reference shot, Paint Bucket Choke 5, tolerance 40, soft light on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Choke 5, tolerance 40, soft light on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Choke 5, tolerance 40, soft light on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Choke 5, tolerance 40, soft light on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Choke 5, tolerance 40, soft light on three layers, frame 100, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Paint Bucket Choke 5, tolerance 40, soft light on three layers, frame 239, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-440 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as added: the house wall at the middle filled red; draws cleanly | [], 4434 pixels changed of 129600 | yes |
| 3_road_feather_blue.png, point on the road, Feather 6, blue, multiply: the road tinted blue, its markings and edge softened; draws cleanly | [], 32160 pixels changed of 129600 | yes |
| 4_sky_threshold.png, point in the sky, tolerance 20, View Threshold: what matches white, the rest black; draws cleanly | [], 129600 pixels changed of 129600 | yes |
| 5_sky_stroke.png, point in the sky, tolerance 20, Stroke width 4, white: a white band round the edge of the sky; draws cleanly | [], 7727 pixels changed of 129600 | yes |

## Result

282 of 282 checks pass.
