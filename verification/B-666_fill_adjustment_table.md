# D-786: Fill on an adjustment layer

B-666, after Ben Marriott's "2D Smoke & Explosions" (9:27 to 9:45), where Fill on the Effects adjustment layer turns the grey cloud strokes white: a Fill with Fill Mask 0 and All Masks off on an adjustment layer fills everything beneath it by D-418's rule, mixed back by D-65's coverage and opacity. Since D-830 one that names a mask fills inside that mask of the adjustment layer (FX-FILLADJ-007, 008, 010, 011); one naming a mask that is not there fills nothing and says EFFECT_PATH_MISSING (012). Before B-666 the frame was left as it was and nothing was said. Every expected pixel is `Fixtures/fill_adjustment/expected_fill_adjustment.json`, written by `tools/fill_adjustment_reference.py` before this code existed and printed in document 25 as FX-FILLADJ-001 to 012. Tolerance 2e-5.

## FX-FILLADJ-001 to 012 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FILLADJ-001 frame 0: Fill as it starts (Fill Mask 0, red, opacity 100) on an adjustment layer: everything beneath turns red, both rows. | largest difference 0.0e0 | yes |
| FX-FILLADJ-001: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-FILLADJ-002 frame 0: The same at Fill opacity 50: half way to red. | largest difference 1.5e-8 | yes |
| FX-FILLADJ-002: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-FILLADJ-003 frame 0: White on half-covered green: white where the green was, its half covering kept, nothing laid on the empty. | largest difference 3.0e-8 | yes |
| FX-FILLADJ-003: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-FILLADJ-004 frame 0: A mask on the adjustment layer: only the left three columns turn red. | largest difference 3.1e-8 | yes |
| FX-FILLADJ-004: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-FILLADJ-005 frame 0: The adjustment layer at opacity 50: half way to red. | largest difference 1.5e-8 | yes |
| FX-FILLADJ-005: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-FILLADJ-006 frame 0: Invert on with no mask: nothing is left to fill, the frame as it was. | largest difference 3.1e-8 | yes |
| FX-FILLADJ-006: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-FILLADJ-007 frame 0: Fill Mask 1 on an adjustment layer with a mask over the left three columns: those three turn red, as 004 (D-830). | largest difference 3.1e-8 | yes |
| FX-FILLADJ-007: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-FILLADJ-008 frame 0: All Masks on, the same: the left three columns red (D-830). | largest difference 3.1e-8 | yes |
| FX-FILLADJ-008: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-FILLADJ-009 frame 0: Blue at opacity 50, then red, the tutorial's chain in small: the last Fill wins, red. | largest difference 0.0e0 | yes |
| FX-FILLADJ-009: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-FILLADJ-010 frame 0: A mask of mode None over the left three columns: the adjustment covers the whole frame, and Fill Mask 1 fills only inside the mask (D-830). | largest difference 3.1e-8 | yes |
| FX-FILLADJ-010: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-FILLADJ-011 frame 0: The same adjustment layer moved 2 pixels right: its mask goes with it, columns 2 to 4 red, the two it no longer covers as they were (D-830). | largest difference 3.1e-8 | yes |
| FX-FILLADJ-011: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-FILLADJ-012 frame 0: Fill Mask 2 with one mask: nothing to fill, the frame as it was, and EFFECT_PATH_MISSING said every frame, as on any layer. | largest difference 3.1e-8 | yes |
| FX-FILLADJ-012: what opening it warns of, and what frame 0 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_filladj_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_filladj_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_filladj_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_filladj_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_filladj_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_filladj_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_filladj_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_filladj_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_filladj_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |

## An old project

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FILL-001 frame 0: The settings as they start: Fill Mask 0 (None), red, opacity 100: the whole cel red, its line, skin and shadow alike, the soft left column half red and half see-through as it was, the empty border still empty. | largest difference 3.0e-8 | yes |
| FX-FILL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |

## Result

35 of 35 checks pass.
