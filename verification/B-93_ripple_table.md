# B-93: ripple

D-150, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the seventeenth of the third batch. Every expected pixel is `Fixtures/ripple/expected_ripple.json`, written by `tools/ripple_reference.py` before this code existed and printed in document 25 as FX-RIPPLE-001 to 026. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-RIPPLE-001 to 026 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RIPPLE-001 frame 0: The settings as they start: centre 50, 50, the point (8, 5), amplitude 5, wavelength 30, speed 20, phase 0, fade 0. Each pixel reads from up to five pixels along its line through the centre. On frame 0 every pixel reads from further out, so the drawing is drawn in toward the centre, a small patch with empty all round it. The rings move outward: the still ring, where nothing moves, is at the centre on frame 0, 3.33 pixels out on frame 2 and 6.67 on frame 4, and inside it each pixel reads from nearer the centre, so the middle of the band is magnified while the rest is still drawn in; the three frames differ. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-001 frame 2: The settings as they start: centre 50, 50, the point (8, 5), amplitude 5, wavelength 30, speed 20, phase 0, fade 0. Each pixel reads from up to five pixels along its line through the centre. On frame 0 every pixel reads from further out, so the drawing is drawn in toward the centre, a small patch with empty all round it. The rings move outward: the still ring, where nothing moves, is at the centre on frame 0, 3.33 pixels out on frame 2 and 6.67 on frame 4, and inside it each pixel reads from nearer the centre, so the middle of the band is magnified while the rest is still drawn in; the three frames differ. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-001 frame 4: The settings as they start: centre 50, 50, the point (8, 5), amplitude 5, wavelength 30, speed 20, phase 0, fade 0. Each pixel reads from up to five pixels along its line through the centre. On frame 0 every pixel reads from further out, so the drawing is drawn in toward the centre, a small patch with empty all round it. The rings move outward: the still ring, where nothing moves, is at the centre on frame 0, 3.33 pixels out on frame 2 and 6.67 on frame 4, and inside it each pixel reads from nearer the centre, so the middle of the band is magnified while the rest is still drawn in; the three frames differ. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIPPLE-002 frame 0: Amplitude 0: the drawing, untouched, on every frame. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-002 frame 4: Amplitude 0: the drawing, untouched, on every frame. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIPPLE-003 frame 0: Amplitude 1, wavelength 4, speed 0: a ring every four pixels, each pixel reading from at most one pixel away, so the stripes wobble in rings about the centre; with speed 0 the rings hold still, the same on frames 0, 2 and 4. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-003 frame 2: Amplitude 1, wavelength 4, speed 0: a ring every four pixels, each pixel reading from at most one pixel away, so the stripes wobble in rings about the centre; with speed 0 the rings hold still, the same on frames 0, 2 and 4. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-003 frame 4: Amplitude 1, wavelength 4, speed 0: a ring every four pixels, each pixel reading from at most one pixel away, so the stripes wobble in rings about the centre; with speed 0 the rings hold still, the same on frames 0, 2 and 4. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIPPLE-004 frame 0: FX-RIPPLE-003 with speed 90: frame 0 is FX-RIPPLE-003; frame 2, half a turn on, is phase 180, FX-RIPPLE-005; and frame 4, a full turn on, is FX-RIPPLE-003 again, to within rounding. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-004 frame 2: FX-RIPPLE-003 with speed 90: frame 0 is FX-RIPPLE-003; frame 2, half a turn on, is phase 180, FX-RIPPLE-005; and frame 4, a full turn on, is FX-RIPPLE-003 again, to within rounding. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-004 frame 4: FX-RIPPLE-003 with speed 90: frame 0 is FX-RIPPLE-003; frame 2, half a turn on, is phase 180, FX-RIPPLE-005; and frame 4, a full turn on, is FX-RIPPLE-003 again, to within rounding. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIPPLE-005 frame 0: FX-RIPPLE-003 with phase 180: half a turn, so every pixel reads from the same distance the other way, out where FX-RIPPLE-003 reads in. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIPPLE-006 frame 0: FX-RIPPLE-003 with wavelength 8: the rings twice as far apart, a gentler wobble. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIPPLE-007 frame 1: FX-RIPPLE-003 with speed -90: the rings move inward. Frame 1, a quarter turn back, differs from speed 90's frame 1, a quarter turn on; frame 2, half a turn back, is FX-RIPPLE-004's frame 2, half a turn on, to within rounding. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-007 frame 2: FX-RIPPLE-003 with speed -90: the rings move inward. Frame 1, a quarter turn back, differs from speed 90's frame 1, a quarter turn on; frame 2, half a turn back, is FX-RIPPLE-004's frame 2, half a turn on, to within rounding. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIPPLE-008 frame 0: Speed 0, fade 6: the reach dies away with the distance from the centre, to nothing at six pixels, so every pixel six or more from (8, 5) is the drawing's own, and every nearer pixel reads from less far than it would with fade 0. | largest difference 2.0e-7 | yes |
| FX-RIPPLE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIPPLE-009 frame 0: FX-RIPPLE-003 with the centre at 53.125, 55, the middle of pixel (8, 5): that pixel, at no distance from the centre, is its own; the rest ripple about it. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIPPLE-010 frame 0: The centre at 53.125, 55 and fade 0.5, the rest as they start: pixel (8, 5) is at the centre and every other pixel at least a pixel away, past the fade, so nothing moves and the drawing is untouched on every frame. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-010 frame 2: The centre at 53.125, 55 and fade 0.5, the rest as they start: pixel (8, 5) is at the centre and every other pixel at least a pixel away, past the fade, so nothing moves and the drawing is untouched on every frame. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIPPLE-011 frame 0: FX-RIPPLE-003 with the centre 0, 0, the top left corner: the rings are quarter circles about the corner. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIPPLE-012 frame 0: FX-RIPPLE-003 with the centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-RIPPLE-003, frame 2 rings about 25, 25, the point (4, 2.5), and frame 4 is FX-RIPPLE-011. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-012 frame 2: FX-RIPPLE-003 with the centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-RIPPLE-003, frame 2 rings about 25, 25, the point (4, 2.5), and frame 4 is FX-RIPPLE-011. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-012 frame 4: FX-RIPPLE-003 with the centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-RIPPLE-003, frame 2 rings about 25, 25, the point (4, 2.5), and frame 4 is FX-RIPPLE-011. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIPPLE-013 frame 0: Wavelength 4, speed 0, amplitude keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frame 2 is amplitude 1, FX-RIPPLE-003, and frame 4 amplitude 2. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-013 frame 2: Wavelength 4, speed 0, amplitude keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frame 2 is amplitude 1, FX-RIPPLE-003, and frame 4 amplitude 2. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-013 frame 4: Wavelength 4, speed 0, amplitude keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frame 2 is amplitude 1, FX-RIPPLE-003, and frame 4 amplitude 2. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIPPLE-014 frame 0: Fade keyed from 20 at frame 0 to 0 at frame 4, eased past its end (-6.5 at frame 2), the rest as they start: frame 0 is fade 20; frame 2 is held at 0, no fade, and is FX-RIPPLE-001's frame 2; frame 4 is FX-RIPPLE-001's frame 4. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-014 frame 2: Fade keyed from 20 at frame 0 to 0 at frame 4, eased past its end (-6.5 at frame 2), the rest as they start: frame 0 is fade 20; frame 2 is held at 0, no fade, and is FX-RIPPLE-001's frame 2; frame 4 is FX-RIPPLE-001's frame 4. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-014 frame 4: Fade keyed from 20 at frame 0 to 0 at frame 4, eased past its end (-6.5 at frame 2), the rest as they start: frame 0 is fade 20; frame 2 is held at 0, no fade, and is FX-RIPPLE-001's frame 2; frame 4 is FX-RIPPLE-001's frame 4. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIPPLE-015 frame 0: FX-RIPPLE-001 moved three pixels right: the rings are worked in the drawing's own space, so they move with it, and the three columns left of the drawing stay empty, as the layer does not grow. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-015 frame 2: FX-RIPPLE-001 moved three pixels right: the rings are worked in the drawing's own space, so they move with it, and the three columns left of the drawing stay empty, as the layer does not grow. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIPPLE-016 frame 0: Amplitude 1, wavelength 1, speed 0: a ring every pixel, so neighbouring pixels read from unalike places and the stripes break up. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIPPLE-017 frame 0: Speed 360, the rest as they start: a full turn every frame, so the ripple seems to stand still, every frame FX-RIPPLE-001's frame 0 to within rounding. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-017 frame 2: Speed 360, the rest as they start: a full turn every frame, so the ripple seems to stand still, every frame FX-RIPPLE-001's frame 0 to within rounding. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-017 frame 4: Speed 360, the rest as they start: a full turn every frame, so the ripple seems to stand still, every frame FX-RIPPLE-001's frame 0 to within rounding. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIPPLE-018 frame 0: Speed 0, phase 40: FX-RIPPLE-001's frame 2, which is phase 0 plus two frames of 20 degrees. | largest difference 2.5e-7 | yes |
| FX-RIPPLE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIPPLE-019 frame 0: Amplitude 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-019 frame 4: Amplitude 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RIPPLE-020 frame 0: Amplitude -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-020 frame 4: Amplitude -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RIPPLE-021 frame 0: Wavelength 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-021 frame 4: Wavelength 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RIPPLE-022 frame 0: Speed 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-022 frame 4: Speed 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RIPPLE-023 frame 0: Phase -100001, below -100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-023 frame 4: Phase -100001, below -100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RIPPLE-024 frame 0: Fade -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-024 frame 4: Fade -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RIPPLE-025 frame 0: Centre 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-025 frame 4: Centre 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RIPPLE-026 frame 0: Amplitude keyed to 1500 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-026 frame 4: Amplitude keyed to 1500 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIPPLE-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview halves the amplitude, the wavelength and the fade, and nothing else | Ripple { center: [50.0, 50.0], amplitude: 2.5, wavelength: 15.0, speed: 20.0, phase: 40.0, fade: 30.0, frame: 0 } | yes |
| a half-size draft of wavelength 1 holds the wavelength at 1, its range's bottom, rather than leaving the effect out | Ripple { center: [50.0, 50.0], amplitude: 2.5, wavelength: 1.0, speed: 20.0, phase: 0.0, fade: 0.0, frame: 0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ripple_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ripple_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ripple_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ripple_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ripple_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ripple_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ripple_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ripple_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ripple_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ripple_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ripple_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ripple_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ripple_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `fade` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a centre of one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amplitude 1001 is refused with a sentence, and nothing changes | Ripple's amplitude runs from 0 to 1000, and this is 1001. | yes |
| amplitude -1 is refused with a sentence, and nothing changes | Ripple's amplitude runs from 0 to 1000, and this is -1. | yes |
| wavelength 0 is refused with a sentence, and nothing changes | Ripple's wavelength runs from 1 to 10000, and this is 0. | yes |
| speed 361 is refused with a sentence, and nothing changes | Ripple's speed runs from -360 to 360, and this is 361. | yes |
| phase -100001 is refused with a sentence, and nothing changes | Ripple's phase runs from -100000 to 100000, and this is -100001. | yes |
| fade -1 is refused with a sentence, and nothing changes | Ripple's fade runs from 0 to 100000, and this is -1. | yes |
| centre 50, 1001 is refused with a sentence, and nothing changes | Ripple's center runs from -1000 to 1000, and this is 1001. | yes |
| amplitude keyed to 1500 is refused with a sentence, and nothing changes | Ripple's amplitude runs from 0 to 1000, and this is 1500. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| the centre keyed from 50, 50 to 0, 0 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ripple_001.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ripple_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ripple_015.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

111 of 111 checks pass.
