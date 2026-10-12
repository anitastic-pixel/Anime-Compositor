# D-563: Shadow/Highlight's Midtone Contrast and Black and White Clip

B-443, after After Effects' Shadow/Highlight: after D-400's lift, Midtone Contrast is Color Grade's contrast curve on the lightness; Black Clip and White Clip stretch the lightness from the clip bins of the layer's own count to its darkest and lightest bins in use; the chroma follows each change toward black or white. The rule is this program's own (Adobe does not publish one). Every expected pixel is `Fixtures/shadow_highlight_clip/expected_shadow_highlight_clip.json`, written by `tools/shadow_highlight_clip_reference.py` before this code existed and printed in document 25 as FX-SHCLIP-001 to 021. Tolerance 2e-5.

## FX-SHCLIP-001 to 021 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHCLIP-001 frame 0: A file written before D-563, without the three settings, at Shadow/Highlight's starting settings: drawn exactly as FX-SHHI-001. | largest difference 2.1e-7 | yes |
| FX-SHCLIP-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHCLIP-002 frame 0: The three written at 0: the same as FX-SHCLIP-001. | largest difference 2.1e-7 | yes |
| FX-SHCLIP-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHCLIP-003 frame 0: Midtone contrast 50 after the starting shadow lift: lightnesses above the middle lighter, below it darker; black and white stay. | largest difference 2.6e-7 | yes |
| FX-SHCLIP-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHCLIP-004 frame 0: Midtone contrast 100: further than FX-SHCLIP-003. | largest difference 3.1e-7 | yes |
| FX-SHCLIP-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHCLIP-005 frame 0: Midtone contrast -50: the middle tones drawn together toward the middle. | largest difference 2.1e-7 | yes |
| FX-SHCLIP-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHCLIP-006 frame 0: Midtone contrast -100: further than FX-SHCLIP-005. | largest difference 1.7e-7 | yes |
| FX-SHCLIP-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHCLIP-007 frame 0: Midtone contrast 60 with both amounts 0: it works alone, with no lift. | largest difference 1.8e-7 | yes |
| FX-SHCLIP-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHCLIP-008 frame 0: Black clip 20: the darkest fifth of the picture pushed to black. | largest difference 2.0e-7 | yes |
| FX-SHCLIP-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHCLIP-009 frame 0: White clip 10: the lightest tenth pushed to white. | largest difference 5.2e-7 | yes |
| FX-SHCLIP-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHCLIP-010 frame 0: Black clip 15 and white clip 15 together: both ends. | largest difference 5.5e-7 | yes |
| FX-SHCLIP-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHCLIP-011 frame 0: Both clips 12 with both amounts 0: the drawing stretched by its own count alone. | largest difference 7.0e-7 | yes |
| FX-SHCLIP-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHCLIP-012 frame 0: Both clips 0.01, After Effects' starting numbers as recalled: the drawing's own black and white are more than 0.01 per cent of it, so nothing is clipped and it is drawn as FX-SHCLIP-001. | largest difference 2.1e-7 | yes |
| FX-SHCLIP-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHCLIP-013 frame 0: Shadow 70, highlight 30, midtone contrast 40, both clips 10: every step in turn. | largest difference 5.3e-7 | yes |
| FX-SHCLIP-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHCLIP-014 frame 0: Midtone contrast keyed from -100 at frame 0 to 100 at frame 4, linear: frame 0 as FX-SHCLIP-006, frame 2 as FX-SHCLIP-002, frame 4 as FX-SHCLIP-004. | largest difference 1.7e-7 | yes |
| FX-SHCLIP-014 frame 2: Midtone contrast keyed from -100 at frame 0 to 100 at frame 4, linear: frame 0 as FX-SHCLIP-006, frame 2 as FX-SHCLIP-002, frame 4 as FX-SHCLIP-004. | largest difference 2.1e-7 | yes |
| FX-SHCLIP-014 frame 4: Midtone contrast keyed from -100 at frame 0 to 100 at frame 4, linear: frame 0 as FX-SHCLIP-006, frame 2 as FX-SHCLIP-002, frame 4 as FX-SHCLIP-004. | largest difference 3.1e-7 | yes |
| FX-SHCLIP-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHCLIP-015 frame 0: Black clip keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 as FX-SHCLIP-002, frame 2 as FX-SHCLIP-008, frame 4 further. | largest difference 2.1e-7 | yes |
| FX-SHCLIP-015 frame 2: Black clip keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 as FX-SHCLIP-002, frame 2 as FX-SHCLIP-008, frame 4 further. | largest difference 2.0e-7 | yes |
| FX-SHCLIP-015 frame 4: Black clip keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 as FX-SHCLIP-002, frame 2 as FX-SHCLIP-008, frame 4 further. | largest difference 2.4e-7 | yes |
| FX-SHCLIP-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHCLIP-016 frame 0: FX-SHCLIP-013 moved three pixels right: the count is of the whole layer, so the same, moved. | largest difference 5.3e-7 | yes |
| FX-SHCLIP-016 frame 3: FX-SHCLIP-013 moved three pixels right: the count is of the whole layer, so the same, moved. | largest difference 5.3e-7 | yes |
| FX-SHCLIP-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHCLIP-017 frame 0: Both clips 50: the black bin is at or above the white bin, so the clips do nothing and it is drawn as FX-SHCLIP-001. | largest difference 2.1e-7 | yes |
| FX-SHCLIP-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHCLIP-018 frame 0: Midtone contrast 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHCLIP-018 frame 4: Midtone contrast 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHCLIP-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHCLIP-019 frame 0: Midtone contrast -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHCLIP-019 frame 4: Midtone contrast -101, below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHCLIP-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHCLIP-020 frame 0: Black clip -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHCLIP-020 frame 4: Black clip -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHCLIP-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHCLIP-021 frame 0: White clip 51, above 50. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHCLIP-021 frame 4: White clip 51, above 50. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHCLIP-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shclip_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shclip_001.json, written without the three settings, is saved without them (D-304's way: only if moved, keyed or already in the file) | {"color_correction":20,"highlight_amount":0,"highlight_radius":30,"highlight_tonal_width":50,"shadow_amount":50,"shadow_radius":30,"shadow_tonal_width":50} | yes |
| fx_shclip_013.json is saved with its ten settings | {"black_clip":10,"color_correction":20,"highlight_amount":30,"highlight_radius":30,"highlight_tonal_width":50,"midtone_contrast":40,"shadow_amount":70,"shadow_radius":30,"shadow_tonal_width":50,"white_clip":10} | yes |
| fx_shclip_018.json is refused in a sentence naming its midtone contrast | Shadow/Highlight's midtone contrast runs from -100 to 100, and this is 101. | yes |
| fx_shclip_019.json is refused in a sentence naming its midtone contrast | Shadow/Highlight's midtone contrast runs from -100 to 100, and this is -101. | yes |
| fx_shclip_020.json is refused in a sentence naming its black clip | Shadow/Highlight's black clip runs from 0 to 50, and this is -1. | yes |
| fx_shclip_021.json is refused in a sentence naming its white clip | Shadow/Highlight's white clip runs from 0 to 50, and this is 51. | yes |
| a file with a Shadow/Highlight whose midtone contrast is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Shadow/Highlight whose black clip is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| midtone contrast 101 is refused with a sentence, and nothing changes | Shadow/Highlight's midtone contrast runs from -100 to 100, and this is 101. | yes |
| white clip 51 is refused with a sentence, and nothing changes | Shadow/Highlight's white clip runs from 0 to 50, and this is 51. | yes |
| black clip keyed to -5 is refused with a sentence, and nothing changes | Shadow/Highlight's black clip runs from 0 to 50, and this is -5. | yes |
| midtone contrast 30, both clips 5 is taken | taken | yes |
| midtone contrast keyed from -100 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shclip_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shclip_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shclip_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shclip_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shclip_015.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shclip_016.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shclip_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_008.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_009.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_012.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 2 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shclip_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Shadow/Highlight as added with midtone contrast 60: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2052768 pixels changed | yes |
| the reference shot, Shadow/Highlight as added with midtone contrast 60 on three layers, frame 0, Full | largest difference 1 of 255, 814 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight as added with midtone contrast 60 on three layers, frame 100, Full | largest difference 1 of 255, 1014 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight as added with midtone contrast 60 on three layers, frame 239, Full | largest difference 1 of 255, 632 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight as added with midtone contrast 60 on three layers, frame 0, Draft | largest difference 1 of 255, 40 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight as added with midtone contrast 60 on three layers, frame 100, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight as added with midtone contrast 60 on three layers, frame 239, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight shadow 0, midtone contrast -80: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2066579 pixels changed | yes |
| the reference shot, Shadow/Highlight shadow 0, midtone contrast -80 on three layers, frame 0, Full | largest difference 1 of 255, 1048 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight shadow 0, midtone contrast -80 on three layers, frame 100, Full | largest difference 1 of 255, 731 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight shadow 0, midtone contrast -80 on three layers, frame 239, Full | largest difference 1 of 255, 1024 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight shadow 0, midtone contrast -80 on three layers, frame 0, Draft | largest difference 1 of 255, 79 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight shadow 0, midtone contrast -80 on three layers, frame 100, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight shadow 0, midtone contrast -80 on three layers, frame 239, Draft | largest difference 1 of 255, 62 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight black clip 2 and white clip 2, on the processor: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2011664 pixels changed | yes |
| the reference shot, Shadow/Highlight black clip 2 and white clip 2, on the processor on three layers, frame 0, Full | largest difference 1 of 255, 58059 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight black clip 2 and white clip 2, on the processor on three layers, frame 100, Full | largest difference 1 of 255, 59224 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight black clip 2 and white clip 2, on the processor on three layers, frame 239, Full | largest difference 1 of 255, 57630 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight black clip 2 and white clip 2, on the processor on three layers, frame 0, Draft | largest difference 1 of 255, 4224 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight black clip 2 and white clip 2, on the processor on three layers, frame 100, Draft | largest difference 1 of 255, 4136 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shadow/Highlight black clip 2 and white clip 2, on the processor on three layers, frame 239, Draft | largest difference 1 of 255, 4138 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-563 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, Shadow/Highlight as added (the three new settings 0); draws cleanly | [] | yes |
| 3_midtone_80.png and 4_midtone_minus_80.png, as added with midtone contrast 80 and -80: the street's brightness spread wider at 80 and drawn in at -80 than as added; draw cleanly | [] [], spread of brightness 64.02, 49.96 as added, 36.70 | yes |
| 5_clips_5.png, as added with both clips 5: more of the street at or below the as-added darkest and more white than as added; draws cleanly | [], pixels at or below the as-added darkest 4241 against 1440, white 5136 against 0 | yes |
| 6_no_lift.png, both amounts 0, midtone contrast 50, both clips 1: the new settings work without the lift; draws cleanly | [], 129600 pixels changed | yes |

## Result

139 of 139 checks pass.
