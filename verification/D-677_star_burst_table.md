# D-677: Star Burst

B-556, after CycoreFX's CC Star Burst (Simulation): the layer broken into a grid of stars, Grid Spacing 270ths of its height apart, each the picture's colour at its grid point; Scatter jumbles them across and through a field three times the eye's distance deep, and the field flies towards the viewer by the speed summed from the in point plus the phase, in picture heights, a star that passes 0.9 of the eye's distance in front coming round again behind. They are seen through the composition camera when the file has one, else through an eye 1.5 picture heights off, drawn as Particle Space's discs farthest first onto clear and mixed with the picture by Blend w. Original. Nothing is carried from frame to frame (document 20). CycoreFX publishes no formula and no settings when added, so the rule and those settings are ours, stated in `tools/star_burst_reference.py`. Every expected pixel is `Fixtures/star_burst/expected_star_burst.json`, written by the tool before this code existed and printed in document 25 as FX-SBURST-001 to 025. Tolerance 2e-5.

## FX-SBURST-001 to 025 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SBURST-001 frame 0: As added: a fine field of about 7000 faint star dots, the picture's colours, scattered through the depth and flying towards the viewer. | largest difference 5.7e-8 | yes |
| FX-SBURST-001 frame 12: As added: a fine field of about 7000 faint star dots, the picture's colours, scattered through the depth and flying towards the viewer. | largest difference 7.3e-8 | yes |
| FX-SBURST-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SBURST-002 frame 12: A close look: about 300 stars, nearer ones larger and further out from the middle. | largest difference 1.0e-7 | yes |
| FX-SBURST-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SBURST-003 frame 12: Scatter 0, Speed 0: the picture as a grid of dots on its own plane. | largest difference 1.2e-7 | yes |
| FX-SBURST-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SBURST-004 frame 12: Scatter 0, Speed 1 at frame 12: the plane of dots half a picture height nearer, so half as large again and spread from the middle. | largest difference 1.1e-7 | yes |
| FX-SBURST-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SBURST-005 frame 12: Scatter 0, Speed 0, Phase 0.5: the same as FX-SBURST-004. | largest difference 1.1e-7 | yes |
| FX-SBURST-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SBURST-006 frame 12: Scatter 0, Speed -1 at frame 12: the plane of dots further off, smaller and drawn in towards the middle. | largest difference 5.7e-8 | yes |
| FX-SBURST-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SBURST-007 frame 12: Size 300: the close look's stars three times as wide. | largest difference 1.5e-7 | yes |
| FX-SBURST-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SBURST-008 frame 12: Blend w. Original 50: the close look half over the picture. | largest difference 1.5e-7 | yes |
| FX-SBURST-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SBURST-009 frame 12: Blend w. Original 100: the picture alone. | largest difference 1.5e-7 | yes |
| FX-SBURST-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SBURST-010 frame 12: The picture with its right half clear: stars only from the left half. | largest difference 8.0e-8 | yes |
| FX-SBURST-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SBURST-011 frame 0: Speed keyed from 0 at frame 0 to 2 at frame 12: the field speeding up. | largest difference 1.4e-7 | yes |
| FX-SBURST-011 frame 12: Speed keyed from 0 at frame 0 to 2 at frame 12: the field speeding up. | largest difference 1.0e-7 | yes |
| FX-SBURST-011 frame 6: Speed keyed from 0 at frame 0 to 2 at frame 12: the field speeding up. | largest difference 1.7e-7 | yes |
| FX-SBURST-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SBURST-012 frame 12: FX-SBURST-002 moved 3 pixels right: the same, moved. | largest difference 8.8e-8 | yes |
| FX-SBURST-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SBURST-013 frame 12: The composition's camera in the middle, 48 pixels off, as the effect's own eye: the same as FX-SBURST-002. | largest difference 1.0e-7 | yes |
| FX-SBURST-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SBURST-014 frame 12: The camera moved to x 20: the field seen from the left of its middle. | largest difference 1.1e-7 | yes |
| FX-SBURST-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SBURST-015 frame 12: The camera dollied in to 24 pixels off: the near stars nearer, those behind it gone. | largest difference 1.3e-7 | yes |
| FX-SBURST-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SBURST-016 frame 12: In point at frame 6: at frame 12 the same as the close look at frame 6. | largest difference 1.1e-7 | yes |
| FX-SBURST-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SBURST-017 frame 12: Grid Spacing 100: a few large stars, 5 by 3. | largest difference 1.4e-7 | yes |
| FX-SBURST-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SBURST-018 frame 24: Scatter 0, Speed 3 at frame 24: the plane of dots has flown past the viewer and come round again behind, half its size. | largest difference 3.8e-8 | yes |
| FX-SBURST-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SBURST-019 frame 0: Scatter 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SBURST-019 frame 4: Scatter 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SBURST-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SBURST-020 frame 0: Speed 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SBURST-020 frame 4: Speed 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SBURST-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SBURST-021 frame 0: Phase 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SBURST-021 frame 4: Phase 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SBURST-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SBURST-022 frame 0: Grid Spacing 0.4, below 0.5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SBURST-022 frame 4: Grid Spacing 0.4, below 0.5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SBURST-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SBURST-023 frame 0: Size -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SBURST-023 frame 4: Size -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SBURST-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SBURST-024 frame 0: Blend w. Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SBURST-024 frame 4: Blend w. Original 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SBURST-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SBURST-025 frame 0: Speed keyed to 200 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SBURST-025 frame 4: Speed keyed to 200 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SBURST-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sburst_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sburst_007.json is saved with its 6 settings, and no clock, view or history | 6 settings; scatter 100, speed 1, phase 0, grid spacing 20, size 300, blend with original 0 | yes |
| fx_sburst_019.json is refused in a sentence naming scatter | Star Burst's scatter runs from 0 to 100, and this is 101. | yes |
| fx_sburst_020.json is refused in a sentence naming speed | Star Burst's speed runs from -100 to 100, and this is 101. | yes |
| fx_sburst_021.json is refused in a sentence naming phase | Star Burst's phase runs from -1000 to 1000, and this is 1001. | yes |
| fx_sburst_022.json is refused in a sentence naming grid spacing | Star Burst's grid spacing runs from 0.5 to 100, and this is 0.4. | yes |
| fx_sburst_023.json is refused in a sentence naming size | Star Burst's size runs from 0 to 1000, and this is -1. | yes |
| fx_sburst_024.json is refused in a sentence naming blend with original | Star Burst's blend with original runs from 0 to 100, and this is 101. | yes |
| fx_sburst_025.json is refused in a sentence naming speed | Star Burst's speed runs from -100 to 100, and this is 200. | yes |
| a file with a Star Burst whose scatter is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Star Burst whose grid spacing is a list is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Star Burst without its phase is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sburst_002.json frame 12 in tiles of 1 and of 64 | byte-identical | yes |
| fx_sburst_011.json frame 6 in tiles of 1 and of 64 | byte-identical | yes |
| fx_sburst_014.json frame 12 in tiles of 1 and of 64 | byte-identical | yes |
| fx_sburst_018.json frame 24 in tiles of 1 and of 64 | byte-identical | yes |

## On the card (D-677)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sburst_001.json, frames [0, 12] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_002.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_003.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_004.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_005.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_006.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_007.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_008.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_009.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_010.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_011.json, frames [0, 12, 6] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_012.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_013.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_014.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_015.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_016.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_017.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_018.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_019.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_020.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_021.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_022.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_023.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_024.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_sburst_025.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Star Burst as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2069316 pixels changed | yes |
| the reference shot, Star Burst as added on three layers, frame 0, Full | largest difference 1 of 255, 527 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Star Burst as added on three layers, frame 100, Full | largest difference 1 of 255, 419 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Star Burst as added on three layers, frame 239, Full | largest difference 1 of 255, 546 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Star Burst as added on three layers, frame 0, Draft | largest difference 1 of 255, 75 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Star Burst as added on three layers, frame 100, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Star Burst as added on three layers, frame 239, Draft | largest difference 1 of 255, 80 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Star Burst Grid Spacing 20, Size 300, flying away at Speed -3: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2045575 pixels changed | yes |
| the reference shot, Star Burst Grid Spacing 20, Size 300, flying away at Speed -3 on three layers, frame 0, Full | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Star Burst Grid Spacing 20, Size 300, flying away at Speed -3 on three layers, frame 100, Full | largest difference 1 of 255, 258 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Star Burst Grid Spacing 20, Size 300, flying away at Speed -3 on three layers, frame 239, Full | largest difference 1 of 255, 27681 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Star Burst Grid Spacing 20, Size 300, flying away at Speed -3 on three layers, frame 0, Draft | largest difference 1 of 255, 12 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Star Burst Grid Spacing 20, Size 300, flying away at Speed -3 on three layers, frame 100, Draft | largest difference 1 of 255, 26 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Star Burst Grid Spacing 20, Size 300, flying away at Speed -3 on three layers, frame 239, Draft | largest difference 1 of 255, 12 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Star Burst Speed keyed 0 to 5, Blend w. Original 30: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2069518 pixels changed | yes |
| the reference shot, Star Burst Speed keyed 0 to 5, Blend w. Original 30 on three layers, frame 0, Full | largest difference 1 of 255, 1674 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Star Burst Speed keyed 0 to 5, Blend w. Original 30 on three layers, frame 100, Full | largest difference 1 of 255, 1895 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Star Burst Speed keyed 0 to 5, Blend w. Original 30 on three layers, frame 239, Full | largest difference 1 of 255, 1800 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Star Burst Speed keyed 0 to 5, Blend w. Original 30 on three layers, frame 0, Draft | largest difference 1 of 255, 560 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Star Burst Speed keyed 0 to 5, Blend w. Original 30 on three layers, frame 100, Draft | largest difference 1 of 255, 489 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Star Burst Speed keyed 0 to 5, Blend w. Original 30 on three layers, frame 239, Draft | largest difference 1 of 255, 571 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-677 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added_frame_24.png, frame 24: drawn without a warning, the street changed | 129031 pixels changed; warnings [] | yes |
| 2_close_look_grid_20_frame_24.png, frame 24: drawn without a warning, the street changed | 128349 pixels changed; warnings [] | yes |
| 3_picture_as_dots.png, frame 24: drawn without a warning, the street changed | 122448 pixels changed; warnings [] | yes |
| 4_dots_half_a_height_nearer.png, frame 24: drawn without a warning, the street changed | 127380 pixels changed; warnings [] | yes |
| 5_big_stars_size_400_frame_48.png, frame 48: drawn without a warning, the street changed | 120463 pixels changed; warnings [] | yes |
| 6_blend_with_original_50_frame_24.png, frame 24: drawn without a warning, the street changed | 127980 pixels changed; warnings [] | yes |
| 7_composition_camera_from_the_left_frame_24.png, frame 24: drawn without a warning, the street changed | 128827 pixels changed; warnings [] | yes |

## Result

153 of 153 checks pass.
