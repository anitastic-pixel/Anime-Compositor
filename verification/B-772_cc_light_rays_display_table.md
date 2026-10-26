# B-772: CC Light Rays on display values, at 0.625 of the zoomed light

D-892, accepted on 2026-10-26. In an 8 bpc composition and in 32 bpc (After Effects) CC Light Rays works on display values, as Glow does there, so its rays fade with distance as After Effects' do; at every depth the rays are laid on at 0.625 of the zoomed light. Every expected pixel is `Fixtures/light_rays_cc/expected_light_rays_cc.json`, written by `tools/light_rays_cc_reference.py`; the answer is the largest difference against the catalogue's tolerance of 2e-5. The card must be within 1 level of 255 of the processor.

## FX-RAYSCC-001 to 026 and 040 to 042 (document 25), every case D-892 changed or added

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RAYSCC-001 frame 0: The settings as they start: intensity 100, centre 50, 50, the point (8, 5), radius 50, warp softness 50, Round, Color from Source on, Allow Brightening on, Transfer Mode None. The source covers the whole drawing, so all of it is light: each pixel is the drawing averaged along the line from the centre out to it, turned 5 degrees both ways, laid over the drawing. The empty columns and rows round the drawing take the patches' rays; the patches show through where the rays are thin. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-002 frame 0: Intensity 0: the drawing, untouched. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-003 frame 0: Radius 3, warp softness 0: only the pixels within 3 of the centre give light, the yellow's columns 5 to 7 and the brown's column 10, their four corner pixels only in part (the source's soft edge); the yellow's half edge in column 4, the brown's column 11 and the purple, outside, give none. Their rays streak outward, the yellow's to the left edge, the brown's to the right, laid over the drawing. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-004 frame 0: FX-RAYSCC-003 with a Square source: the square reaches 3 each way and into its corners, so the four corner pixels the round source takes only in part give all their light. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-005 frame 0: The square turned 45 degrees: a diamond, reaching 4.2 along the row and the column through the centre, so the brown's column 11 gives part of its light, and less to the corners. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-006 frame 0: Round with direction 45: direction does nothing for a round source; FX-RAYSCC-003. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-007 frame 0: Warp softness 300: FX-RAYSCC-003's rays turned 30 degrees both ways, so they spread round the centre and melt together. | largest difference 2.1e-7 | yes |
| FX-RAYSCC-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-008 frame 0: Color from Source off, colour white: the light is white at the source's covering, so the yellow and the brown give white rays. | largest difference 8.7e-8 | yes |
| FX-RAYSCC-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-009 frame 0: Color from Source off, colour #ff8000, orange: orange rays, red as much as the rays' covering, a fifth as much green, no blue. | largest difference 8.7e-8 | yes |
| FX-RAYSCC-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-010 frame 0: Color from Source on with colour #ff8000: the colour does not count; FX-RAYSCC-003. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-011 frame 0: FX-RAYSCC-009 with its colour written in capitals, #FF8000: the same. | largest difference 8.7e-8 | yes |
| FX-RAYSCC-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-012 frame 0: Transfer Mode Add: the rays added onto the drawing, D-124's way, so the lit patches brighten rather than being covered. | largest difference 3.0e-7 | yes |
| FX-RAYSCC-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-013 frame 0: Transfer Mode Screen: brighter than the drawing everywhere the rays fall, but less than Add where both are bright. | largest difference 1.3e-7 | yes |
| FX-RAYSCC-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-014 frame 0: Transfer Mode Lighten: each channel the larger of the drawing's and the rays'. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-015 frame 0: Intensity 300, Add: three times FX-RAYSCC-012's rays, past white, not cut off; the covering stops at full. | largest difference 4.8e-7 | yes |
| FX-RAYSCC-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-016 frame 0: Intensity 300, Add, Allow Brightening off: held at 100; FX-RAYSCC-012. | largest difference 3.0e-7 | yes |
| FX-RAYSCC-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-017 frame 0: Intensity 50, Allow Brightening off: below 100 nothing is held; the same as with it on. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-018 frame 0: Intensity 50, Allow Brightening on, the pair of FX-RAYSCC-017. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-019 frame 0: Radius 0: the centre sits on a pixel corner, every pixel centre at least 0.7 from it, so nothing is light; the drawing, untouched. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-020 frame 0: Centre 25, 50, the point (4, 5), radius 3: the yellow alone is the source, its rays streaking left to the edge and right across the gap and over the brown and the purple. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-021 frame 0: Radius keyed from 0 at frame 0 to 6 at frame 4, linear: frame 0 is FX-RAYSCC-019, frame 2 is FX-RAYSCC-003, frame 4 lights all but the purple. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-021 frame 2: Radius keyed from 0 at frame 0 to 6 at frame 4, linear: frame 0 is FX-RAYSCC-019, frame 2 is FX-RAYSCC-003, frame 4 lights all but the purple. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-021 frame 4: Radius keyed from 0 at frame 0 to 6 at frame 4, linear: frame 0 is FX-RAYSCC-019, frame 2 is FX-RAYSCC-003, frame 4 lights all but the purple. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-022 frame 0: A Square source with direction keyed from 0 at frame 0 to 90 at frame 4, linear: frame 2 is FX-RAYSCC-005, and frame 4, a quarter turn of a square, is frame 0 again. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-022 frame 2: A Square source with direction keyed from 0 at frame 0 to 90 at frame 4, linear: frame 2 is FX-RAYSCC-005, and frame 4, a quarter turn of a square, is frame 0 again. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-022 frame 4: A Square source with direction keyed from 0 at frame 0 to 90 at frame 4, linear: frame 2 is FX-RAYSCC-005, and frame 4, a quarter turn of a square, is frame 0 again. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-023 frame 0: Intensity eased from 0 at frame 0 to 2000 at frame 4 on a curve that overshoots, Add: at frame 2 it would pass 2000, is held at 2000, as frame 4 is. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-023 frame 2: Intensity eased from 0 at frame 0 to 2000 at frame 4 on a curve that overshoots, Add: at frame 2 it would pass 2000, is held at 2000, as frame 4 is. | largest difference 2.4e-6 | yes |
| FX-RAYSCC-023 frame 4: Intensity eased from 0 at frame 0 to 2000 at frame 4 on a curve that overshoots, Add: at frame 2 it would pass 2000, is held at 2000, as frame 4 is. | largest difference 2.4e-6 | yes |
| FX-RAYSCC-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-024 frame 0: FX-RAYSCC-003 moved three pixels right: the rays are drawn on the drawing before it is moved, so it is FX-RAYSCC-003 moved, and the three columns left of the drawing stay empty, as the layer does not grow. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-024 frame 3: FX-RAYSCC-003 moved three pixels right: the rays are drawn on the drawing before it is moved, so it is FX-RAYSCC-003 moved, and the three columns left of the drawing stay empty, as the layer does not grow. | largest difference 1.5e-7 | yes |
| FX-RAYSCC-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-025 frame 0: Intensity 250, None: the rays' colour runs past their covering, laid over the drawing; their covering stops at full. | largest difference 2.7e-7 | yes |
| FX-RAYSCC-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-026 frame 0: Square 4, direction 30, warp 120, Color from Source off #40c0ff, intensity 180, Screen, centre 40, 60: the controls together. | largest difference 1.7e-7 | yes |
| FX-RAYSCC-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-040 frame 0: 8 bpc, FX-RAYSCC-012's settings (radius 3, Add): the rays worked and added in display values, then held to 8 bits, so a faint ray lifts a dark pixel no more than it lifts a light one. | largest difference 2.4e-7 | yes |
| FX-RAYSCC-040: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-041 frame 0: 8 bpc, FX-RAYSCC-009's orange rays at intensity 90: #ff8000 used as written, so its green is half its red in display values, not a fifth (at 100 a pixel lands on a tie between two 8-bit levels). | largest difference 1.8e-7 | yes |
| FX-RAYSCC-041: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYSCC-042 frame 0: 32 bpc (After Effects), FX-RAYSCC-015's intensity 300 with Add: in display values and nothing cut off at white. | largest difference 3.8e-6 | yes |
| FX-RAYSCC-042: what opening it warns of, and what frame 4 warns of | [] and [] | yes |

## Older projects

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

## D-124's form (FX-RAYS-001 to 024), drawn as before

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RAYS-001 frame 0: The settings as they start: centre 50, 50, the point (8, 5), length 50, threshold 70, intensity 1, white. Only the yellow, 98 %, is bright enough; its light streaks outward, away from the centre: left across its soft edge into the empty columns 0 to 3, and up and down into the empty rows above and below it, fading as it goes, and it brightens the yellow itself. The brown, 60 %, and the purple give no light, and columns 9 to 15 are untouched; nothing is drawn past the layer's edge. | largest difference 3.1e-7 | yes |
| FX-RAYS-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-002 frame 0: Intensity 0: the drawing, untouched. | largest difference 1.5e-7 | yes |
| FX-RAYS-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-003 frame 0: Length 0: no streaks, but the light is still added onto itself: each yellow pixel twice as bright, and the half-covering edge, whose light is its own, doubled to full covering. | largest difference 3.1e-7 | yes |
| FX-RAYS-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-004 frame 0: Threshold 100: nothing is that bright, so the drawing is untouched. | largest difference 1.5e-7 | yes |
| FX-RAYS-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-005 frame 0: Threshold 60: the brown, at exactly 60 %, is lit too, and its light streaks right, over the purple line and on to the drawing's right edge. | largest difference 3.1e-7 | yes |
| FX-RAYS-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-006 frame 0: Length 100, the most: each pixel gathers the light all the way from the centre out to itself, so the streaks reach further than FX-RAYS-001's; and as the yellow touches the centre, every pixel of the drawing takes some of its light, right of the centre too, faintest at the right edge. | largest difference 3.1e-7 | yes |
| FX-RAYS-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-007 frame 0: Intensity 2.5: two and a half times FX-RAYS-001's rays, and where they add past white they are not cut off; the covering stops at full. | largest difference 6.2e-7 | yes |
| FX-RAYS-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-008 frame 0: Colour #ff8000, orange: FX-RAYS-001's rays tinted, their red as before, their green a fifth and their blue gone; the covering as FX-RAYS-001's. | largest difference 3.1e-7 | yes |
| FX-RAYS-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-009 frame 0: Colour #000000, black: the rays add no colour but still add covering, so they fall as a dark shadow on the empty pixels; the solid yellow is unchanged, and its half-covering edge gains covering but no colour, so it darkens. | largest difference 1.5e-7 | yes |
| FX-RAYS-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-010 frame 0: FX-RAYS-008 with its colour written in capitals, #FF8000: the same. | largest difference 3.1e-7 | yes |
| FX-RAYS-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-011 frame 0: Centre 0, 0, the top left corner: the streaks run down and right, away from the corner, onto the empty pixels right of the yellow and below it; the corner itself stays empty. | largest difference 3.1e-7 | yes |
| FX-RAYS-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-012 frame 0: Centre 50, -100, above the drawing: the streaks run straight down from the yellow to the bottom edge, and nothing lights the rows above it. | largest difference 2.9e-7 | yes |
| FX-RAYS-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-013 frame 0: Length keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-RAYS-003, frame 2 is FX-RAYS-001, frame 4 is FX-RAYS-006. | largest difference 3.1e-7 | yes |
| FX-RAYS-013 frame 2: Length keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-RAYS-003, frame 2 is FX-RAYS-001, frame 4 is FX-RAYS-006. | largest difference 3.1e-7 | yes |
| FX-RAYS-013 frame 4: Length keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-RAYS-003, frame 2 is FX-RAYS-001, frame 4 is FX-RAYS-006. | largest difference 3.1e-7 | yes |
| FX-RAYS-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-014 frame 0: Threshold keyed from 100 at frame 0 to 60 at frame 4, linear: frame 0 is the drawing, frame 2, at 80, lights the yellow alone and is FX-RAYS-001, frame 4 is FX-RAYS-005. | largest difference 1.5e-7 | yes |
| FX-RAYS-014 frame 2: Threshold keyed from 100 at frame 0 to 60 at frame 4, linear: frame 0 is the drawing, frame 2, at 80, lights the yellow alone and is FX-RAYS-001, frame 4 is FX-RAYS-005. | largest difference 3.1e-7 | yes |
| FX-RAYS-014 frame 4: Threshold keyed from 100 at frame 0 to 60 at frame 4, linear: frame 0 is the drawing, frame 2, at 80, lights the yellow alone and is FX-RAYS-001, frame 4 is FX-RAYS-005. | largest difference 3.1e-7 | yes |
| FX-RAYS-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-015 frame 0: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-RAYS-001, frame 2 streams from 25, 25, frame 4 is FX-RAYS-011. | largest difference 3.1e-7 | yes |
| FX-RAYS-015 frame 2: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-RAYS-001, frame 2 streams from 25, 25, frame 4 is FX-RAYS-011. | largest difference 3.7e-7 | yes |
| FX-RAYS-015 frame 4: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-RAYS-001, frame 2 streams from 25, 25, frame 4 is FX-RAYS-011. | largest difference 3.1e-7 | yes |
| FX-RAYS-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-016 frame 0: Intensity eased from 0 at frame 0 to 10 at frame 4 on a curve that overshoots: at frame 2 it would pass 10, is held at 10, and is intensity 10 plain, as frame 4 is. | largest difference 1.5e-7 | yes |
| FX-RAYS-016 frame 2: Intensity eased from 0 at frame 0 to 10 at frame 4 on a curve that overshoots: at frame 2 it would pass 10, is held at 10, and is intensity 10 plain, as frame 4 is. | largest difference 2.0e-6 | yes |
| FX-RAYS-016 frame 4: Intensity eased from 0 at frame 0 to 10 at frame 4 on a curve that overshoots: at frame 2 it would pass 10, is held at 10, and is intensity 10 plain, as frame 4 is. | largest difference 2.0e-6 | yes |
| FX-RAYS-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-017 frame 0: FX-RAYS-001 moved three pixels right: the rays are drawn on the drawing before it is moved, so it is FX-RAYS-001 moved, and the three columns left of the drawing stay empty, as the layer does not grow. | largest difference 3.1e-7 | yes |
| FX-RAYS-017 frame 3: FX-RAYS-001 moved three pixels right: the rays are drawn on the drawing before it is moved, so it is FX-RAYS-001 moved, and the three columns left of the drawing stay empty, as the layer does not grow. | largest difference 3.1e-7 | yes |
| FX-RAYS-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-018 frame 0: Length 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-018 frame 4: Length 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYS-019 frame 0: Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-019 frame 4: Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYS-020 frame 0: Intensity 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-020 frame 4: Intensity 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYS-021 frame 0: Intensity keyed to 20 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-021 frame 4: Intensity keyed to 20 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYS-022 frame 0: Centre 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-022 frame 4: Centre 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYS-023 frame 0: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-023 frame 4: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYS-024 frame 0: A colour written "orange", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-024 frame 4: A colour written "orange", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## On the card against the processor, the reference shot

| Check | The build's answer | Matches |
| --- | --- | --- |
| 8 bpc, as it starts, frame 0, Full | largest difference 1 of 255, 35 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| 8 bpc, as it starts, frame 100, Full | largest difference 1 of 255, 35 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| 8 bpc, as it starts, frame 0, Draft | largest difference 1 of 255, 5 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| 8 bpc, as it starts, frame 100, Draft | largest difference 1 of 255, 6 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| 8 bpc, radius 400, warp 300, Add, frame 0, Full | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| 8 bpc, radius 400, warp 300, Add, frame 100, Full | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| 8 bpc, radius 400, warp 300, Add, frame 0, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| 8 bpc, radius 400, warp 300, Add, frame 100, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| 8 bpc, Color from Source off #ff8000, intensity 250, Screen, frame 0, Full | largest difference 1 of 255, 234 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| 8 bpc, Color from Source off #ff8000, intensity 250, Screen, frame 100, Full | largest difference 1 of 255, 218 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| 8 bpc, Color from Source off #ff8000, intensity 250, Screen, frame 0, Draft | largest difference 1 of 255, 250 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| 8 bpc, Color from Source off #ff8000, intensity 250, Screen, frame 100, Draft | largest difference 1 of 255, 247 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| 32 bpc (After Effects), as it starts, frame 0, Full | largest difference 1 of 255, 31 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| 32 bpc (After Effects), as it starts, frame 100, Full | largest difference 1 of 255, 25 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| 32 bpc (After Effects), as it starts, frame 0, Draft | largest difference 1 of 255, 2 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| 32 bpc (After Effects), as it starts, frame 100, Draft | largest difference 1 of 255, 2 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| 32 bpc (After Effects), radius 400, warp 300, Add, frame 0, Full | largest difference 1 of 255, 14 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| 32 bpc (After Effects), radius 400, warp 300, Add, frame 100, Full | largest difference 1 of 255, 31 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| 32 bpc (After Effects), radius 400, warp 300, Add, frame 0, Draft | largest difference 1 of 255, 2 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| 32 bpc (After Effects), radius 400, warp 300, Add, frame 100, Draft | largest difference 1 of 255, 3 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| 32 bpc (After Effects), Color from Source off #ff8000, intensity 250, Screen, frame 0, Full | largest difference 1 of 255, 50 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| 32 bpc (After Effects), Color from Source off #ff8000, intensity 250, Screen, frame 100, Full | largest difference 1 of 255, 22 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| 32 bpc (After Effects), Color from Source off #ff8000, intensity 250, Screen, frame 0, Draft | largest difference 1 of 255, 4 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| 32 bpc (After Effects), Color from Source off #ff8000, intensity 250, Screen, frame 100, Draft | largest difference 1 of 255, 1 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |

## Result

178 of 178 checks pass.
