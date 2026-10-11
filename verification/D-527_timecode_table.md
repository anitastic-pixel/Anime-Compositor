# B-407: Timecode

D-527: Timecode, after After Effects' Timecode (Text): the layer's or the composition's time, or a count of its own, burnt onto the picture as SMPTE timecode, a frame number or feet and frames of film, on a box. Every expected frame is a project in `Fixtures/timecode/same/`, written by `tools/timecode_reference.py` before this code existed and printed in document 25 as FX-TC-001 to 046: the box as a solid and the characters as text layers. The build's frame is compared sample by sample with that project's frame; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-TC-001 to 046 (document 25): each frame against the words it must show

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TC-001 frame 0: As added: the layer's own time in SMPTE on a black box, at frames 0 and 37 of a 24-frame composition: 00:00:00:00 and 00:00:01:13. Shows "00:00:00:00" | largest difference 0.0e0 | yes |
| FX-TC-001 frame 37: As added: the layer's own time in SMPTE on a black box, at frames 0 and 37 of a 24-frame composition: 00:00:00:00 and 00:00:01:13. Shows "00:00:01:13" | largest difference 0.0e0 | yes |
| FX-TC-001: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-002 frame 34: Layer source on a layer starting at frame 10: its source counts from there, 00:00:01:00 at frame 34. Shows "00:00:01:00" | largest difference 0.0e0 | yes |
| FX-TC-002: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-003 frame 25: Layer source on a composition layer stretched to 200 per cent: source frame 12.5 at frame 25, taken down to 00:00:00:12. Shows "00:00:00:12" | largest difference 0.0e0 | yes |
| FX-TC-003: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-004 frame 34: Composition source on a layer starting at frame 10: the composition's frame, 00:00:01:10 at frame 34. Shows "00:00:01:10" | largest difference 0.0e0 | yes |
| FX-TC-004: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-005 frame 86405: Composition source in a composition starting at frame 86,400: an hour at 24, 01:00:00:05 at its sixth frame. Shows "01:00:00:05" | largest difference 0.0e0 | yes |
| FX-TC-005: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-006 frame 1799: Composition source at 29.97: drop frame, 00:00:59;29 then 00:01:00;02, the two frame numbers of the new minute left out. Shows "00:00:59;29" | largest difference 0.0e0 | yes |
| FX-TC-006 frame 1800: Composition source at 29.97: drop frame, 00:00:59;29 then 00:01:00;02, the two frame numbers of the new minute left out. Shows "00:01:00;02" | largest difference 0.0e0 | yes |
| FX-TC-006: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-007 frame 17981: Composition source at 29.97 at the tenth minute, where nothing is left out: 00:09:59;29 then 00:10:00;00. Shows "00:09:59;29" | largest difference 0.0e0 | yes |
| FX-TC-007 frame 17982: Composition source at 29.97 at the tenth minute, where nothing is left out: 00:09:59;29 then 00:10:00;00. Shows "00:10:00;00" | largest difference 0.0e0 | yes |
| FX-TC-007: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-008 frame 30: Composition source at 25: 00:00:01:05 at frame 30. Shows "00:00:01:05" | largest difference 0.0e0 | yes |
| FX-TC-008: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-009 frame 30: Composition source at 23.976: counted non-drop at 24, 00:00:01:06 at frame 30. Shows "00:00:01:06" | largest difference 0.0e0 | yes |
| FX-TC-009: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-010 frame 12: Custom at 30 time units in a 24-frame composition: half a second, 00:00:00:15, at frame 12. Shows "00:00:00:15" | largest difference 0.0e0 | yes |
| FX-TC-010: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-011 frame 0: Custom, drop frame at 30, starting frame 107,892: the layer's first frame is 01:00:00;00. Shows "01:00:00;00" | largest difference 0.0e0 | yes |
| FX-TC-011: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-012 frame 0: Custom, drop frame at 60 time units, starting frame 3,600 (a minute of frames, the first minute dropping none): 00:01:00;04, four numbers left out. Shows "00:01:00;04" | largest difference 0.0e0 | yes |
| FX-TC-012: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-013 frame 24: Custom at 25 time units, starting frame -50: one second in, -00:00:01:00. Shows "-00:00:01:00" | largest difference 0.0e0 | yes |
| FX-TC-013: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-014 frame 6: Custom on a composition layer stretched to 50 per cent counts by its key time: 00:00:00:15 at frame 6. Shows "00:00:00:15" | largest difference 0.0e0 | yes |
| FX-TC-014: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-015 frame 24: Custom with time units 29.9, taken down to 29: one second in is 29 frames, 00:00:01:00, at frame 24. Shows "00:00:01:00" | largest difference 0.0e0 | yes |
| FX-TC-015: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-016 frame 37: Frames: 00037 at frame 37. Shows "00037" | largest difference 0.0e0 | yes |
| FX-TC-016: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-017 frame 0: Frames past five digits: 123456 as it is. Shows "123456" | largest difference 0.0e0 | yes |
| FX-TC-017: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-018 frame 37: Feet and frames of 35mm: 37 frames is 0002+05. Shows "0002+05" | largest difference 0.0e0 | yes |
| FX-TC-018: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-019 frame 0: Feet and frames of 16mm: frame 1,000 is 0025+00. Shows "0025+00" | largest difference 0.0e0 | yes |
| FX-TC-019: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-020 frame 37: No box: the words alone over the plate. Shows "00:00:01:13" | largest difference 0.0e0 | yes |
| FX-TC-020: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-021 frame 37: A blue box at 50 per cent under yellow words. Shows "00:00:01:13" | largest difference 6.0e-8 | yes |
| FX-TC-021: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-022 frame 37: Composite off: the box and the words alone on transparent. Shows "00:00:01:13" | largest difference 0.0e0 | yes |
| FX-TC-022: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-023 frame 37: Composite off and no box: the words alone on transparent. Shows "00:00:01:13" | largest difference 0.0e0 | yes |
| FX-TC-023: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-024 frame 37: Near the top left corner, size 28: the box runs off the picture's edges. Shows "00:00:01:13" | largest difference 0.0e0 | yes |
| FX-TC-024: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-025 frame 37: Size 80, in the middle. Shows "00:00:01:13" | largest difference 0.0e0 | yes |
| FX-TC-025: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-026 frame 37: Box opacity 0: only the words show. Shows "00:00:01:13" | largest difference 0.0e0 | yes |
| FX-TC-026: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-027 frame 10: Layer source on a composition layer starting at frame 10 with a source offset of 48: 00:00:02:00 at frame 10. Shows "00:00:02:00" | largest difference 0.0e0 | yes |
| FX-TC-027: what opening it warns of, and what its first frame warns of | [] and [] | yes |
| FX-TC-040 frame 0: Display format written "seconds". The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-TC-040: what opening it warns of, and what its first frame warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TC-041 frame 0: Time source written "clock". The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-TC-041: what opening it warns of, and what its first frame warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TC-042 frame 0: Time units 0. The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-TC-042: what opening it warns of, and what its first frame warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TC-043 frame 0: Drop frame at 25 time units. The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-TC-043: what opening it warns of, and what its first frame warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TC-044 frame 0: Text colour written "white". The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-TC-044: what opening it warns of, and what its first frame warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TC-045 frame 0: Opacity 150. The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-TC-045: what opening it warns of, and what its first frame warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TC-046 frame 0: Starting frame 20,000,000. The file is read, the effect is kept as written and left out of every frame, with a warning. Shows nothing | largest difference 0.0e0 | yes |
| FX-TC-046: what opening it warns of, and what its first frame warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## A layer with a file: its own starting timecode is not read, and that is said

| Check | The build's answer | Matches |
| --- | --- | --- |
| a still with Timecode, layer source: said, as information | INFO Layer plate's Timecode counts from the file's first frame, 0: a starting timecode written in the file is not read. | yes |
| a still with Timecode, composition source: nothing said | nothing | yes |
| a still with Timecode, custom: nothing said | nothing | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tc_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tc_001.json: the count worked each frame is never saved | {"box_color":"#000000","composite_on_original":"on","display_format":"smpte","drop_frame":"off","opacity":100,"show_box":"on","starting_frame":0,"text_color":"#ffffff","text_position":[50,85],"text_size":36,"time_source":"layer_source","time_units":30} | yes |
| fx_tc_011.json: the count worked each frame is never saved | {"box_color":"#000000","composite_on_original":"on","display_format":"smpte","drop_frame":"on","opacity":100,"show_box":"on","starting_frame":107892,"text_color":"#ffffff","text_position":[50,85],"text_size":36,"time_source":"custom","time_units":30} | yes |
| fx_tc_040.json is refused in a sentence naming it | Timecode's display format is one of smpte, frames, feet_frames_35mm, feet_frames_16mm, and this is "seconds". | yes |
| fx_tc_041.json is refused in a sentence naming it | Timecode's time source is one of layer_source, composition, custom, and this is "clock". | yes |
| fx_tc_042.json is refused in a sentence naming it | Timecode's time units runs from 1 to 999, and this is 0. | yes |
| fx_tc_043.json is refused in a sentence naming it | Timecode's drop frame counts only at time units that are a multiple of 30, and this is 25. | yes |
| fx_tc_044.json is refused in a sentence naming it | Timecode's text colour is written #rrggbb, and this is "white". | yes |
| fx_tc_045.json is refused in a sentence naming it | Timecode's opacity runs from 0 to 100, and this is 150. | yes |
| fx_tc_046.json is refused in a sentence naming it | Timecode's starting frame runs from -10000000 to 10000000, and this is 20000000. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Text size 0 is refused with a sentence, and nothing changes | Timecode's text size runs from 1 to 2000, and this is 0. | yes |
| Display Frames, Composition source, is taken | taken | yes |
| Custom, drop frame at 60 time units, a blue box at half opacity, is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tc_001.json frame 37 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tc_006.json frame 1800 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tc_021.json frame 37 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tc_024.json frame 37 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tc_027.json frame 10 in tiles of 1 and of 64 | byte-identical | yes |

## The card against the processor, within 1 level of 255 (ADR-006, D-100)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tc_001.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_001.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_001.json frame 37, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_001.json frame 37, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_002.json frame 34, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_002.json frame 34, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_003.json frame 25, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_003.json frame 25, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_004.json frame 34, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_004.json frame 34, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_005.json frame 86405, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_005.json frame 86405, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_006.json frame 1799, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_006.json frame 1799, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_006.json frame 1800, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_006.json frame 1800, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_007.json frame 17981, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_007.json frame 17981, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_007.json frame 17982, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_007.json frame 17982, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_008.json frame 30, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_008.json frame 30, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_009.json frame 30, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_009.json frame 30, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_010.json frame 12, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_010.json frame 12, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_011.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_011.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_012.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_012.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_013.json frame 24, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_013.json frame 24, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_014.json frame 6, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_014.json frame 6, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_015.json frame 24, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_015.json frame 24, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_016.json frame 37, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_016.json frame 37, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_017.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_017.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_018.json frame 37, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_018.json frame 37, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_019.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_019.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_020.json frame 37, Full | largest 1 level(s), 59 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_020.json frame 37, Draft | largest 1 level(s), 3 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_021.json frame 37, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_021.json frame 37, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_022.json frame 37, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_022.json frame 37, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_023.json frame 37, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_023.json frame 37, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_024.json frame 37, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_024.json frame 37, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_025.json frame 37, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_025.json frame 37, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_026.json frame 37, Full | largest 1 level(s), 59 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_026.json frame 37, Draft | largest 1 level(s), 3 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_027.json frame 10, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_027.json frame 10, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "" | yes |
| fx_tc_040.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_tc_040.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_tc_041.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_tc_041.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_tc_042.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_tc_042.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_tc_043.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_tc_043.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_tc_044.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_tc_044.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_tc_045.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_tc_045.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_tc_046.json frame 0, Full | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| fx_tc_046.json frame 0, Draft | largest 0 level(s), 0 pixel(s) differ; 0 of 1 effect(s) on the card; warnings "EFFECT_PARAMETER_INVALID" | yes |
| a 480 by 160 picture, as added, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "PROJECT_FEATURE_UNSUPPORTED" | yes |
| a 480 by 160 picture, as added, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "PROJECT_FEATURE_UNSUPPORTED" | yes |
| a 480 by 160 picture, frames, a blue box at half opacity, composite off, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, frames, a blue box at half opacity, composite off, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, custom drop frame at 30, size 60, near the top, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, custom drop frame at 30, size 60, near the top, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, feet and frames of 16mm, no box, yellow, Full | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| a 480 by 160 picture, feet and frames of 16mm, no box, yellow, Draft | largest 0 level(s), 0 pixel(s) differ; 1 of 1 effect(s) on the card; warnings "" | yes |
| the largest difference over every frame above | 1 level(s) | yes |

## Pictures: in `verification/D-527 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| as_added.png, as added: 00:00:01:13 in white on a black box, low in the middle of the dark blue plate | warnings "" | yes |
| drop_frame.png, 29.97 drop frame at the new minute: 00:01:00;02, a semicolon before the frames | warnings "" | yes |
| custom_hour.png, custom, drop frame, starting frame 107,892: 01:00:00;00 | warnings "" | yes |
| negative.png, custom from -50 at 25 time units: -00:00:01:00 | warnings "" | yes |
| frames.png, the frame number, 00037 | warnings "" | yes |
| feet_35mm.png, feet and frames of 35mm: 0002+05 | warnings "" | yes |
| no_box.png, no box: the white numbers straight on the plate | warnings "" | yes |
| blue_box.png, yellow numbers on a blue box at half opacity, the plate showing through the box | warnings "" | yes |
| composite_off.png, composite off: the black box and white numbers on transparent (shown on black: only the numbers show) | warnings "" | yes |
| corner.png, smaller, near the top left corner, the box cut off by the picture's edges | warnings "" | yes |
| size_80.png, size 80: big numbers on a big box | warnings "" | yes |

## Result

220 of 220 checks pass.
