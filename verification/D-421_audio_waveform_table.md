# D-421: Audio Waveform

B-300, after After Effects' Audio Waveform: the samples of a sound layer over Audio Duration from the frame's first sample plus Audio Offset (the layer's gain applied, held to -1..1; Mono the channels' mean, Left channel 0, Right channel 1), cut into Displayed Samples shares, each share's least and greatest drawn as a stroke from one to the other (Digital), or one of them, picked by Random Seed, joined in a line (Analog Lines) or dotted (Analog Dots), Maximum Height times the level from the line from Start to End Point or round a mask (Path), each mark Beam's line (D-207), drawn as Audio Spectrum's (D-420). Every expected pixel is `Fixtures/audio_waveform/expected_audio_waveform.json`, written by `tools/audio_waveform_reference.py` before this code existed and printed in document 25 as FX-AWAVE-001 to 049. Tolerance 2e-5. The least and greatest are worked out on the processor and the marks drawn on the card, on Audio Spectrum's pass.

## FX-AWAVE-001 to 049 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-AWAVE-001 frame 0: The settings as they start, no Audio Layer: silence, so a flat Analog Line 2 thick along the line through the middle, alone. | largest difference 2.1e-8 | yes |
| FX-AWAVE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-002 frame 0: Eight displayed samples of a 10 ms window, 4 tall each way at most, Analog Lines, 2 thick, sharp, alone: a zigzag across the middle, each point the least or greatest of its six source samples as Random Seed 1 picks. | largest difference 3.0e-8 | yes |
| FX-AWAVE-002 frame 2: Eight displayed samples of a 10 ms window, 4 tall each way at most, Analog Lines, 2 thick, sharp, alone: a zigzag across the middle, each point the least or greatest of its six source samples as Random Seed 1 picks. | largest difference 3.0e-8 | yes |
| FX-AWAVE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-003 frame 0: FX-AWAVE-002 with Composite On Original on: added to the cel. | largest difference 2.4e-7 | yes |
| FX-AWAVE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-004 frame 0: Digital: each displayed sample a stroke from its least to its greatest source sample. | largest difference 3.0e-8 | yes |
| FX-AWAVE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-005 frame 0: Analog Dots: a dot at each picked point. | largest difference 2.9e-8 | yes |
| FX-AWAVE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-006 frame 0: Random Seed 2: other picks of least or greatest. | largest difference 3.0e-8 | yes |
| FX-AWAVE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-007 frame 0: Random Seed 2.9: its whole part counted, so FX-AWAVE-006's frame. | largest difference 3.0e-8 | yes |
| FX-AWAVE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-008 frame 0: The stereo file, Waveform Options Left: the 600 Hz channel alone. | largest difference 3.0e-8 | yes |
| FX-AWAVE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-009 frame 0: The stereo file, Right: the 900 Hz channel alone. | largest difference 2.8e-8 | yes |
| FX-AWAVE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-010 frame 0: The stereo file, Mono: the two channels' mean. | largest difference 2.8e-8 | yes |
| FX-AWAVE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-011 frame 0: The one-channel file with Right: it plays as Mono, so FX-AWAVE-002's frame. | largest difference 3.0e-8 | yes |
| FX-AWAVE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-012 frame 0: Audio Offset 5 ms: the window starts 24 samples later. | largest difference 2.9e-8 | yes |
| FX-AWAVE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-013 frame 0: Audio Duration 40 ms: 192 samples, 24 to each displayed sample, so every least and greatest is near the tone's own. | largest difference 2.8e-8 | yes |
| FX-AWAVE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-014 frame 0: Displayed Samples 1: one point in the middle of the line, a dot. | largest difference 2.0e-8 | yes |
| FX-AWAVE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-015 frame 0: Displayed Samples 48: one source sample each, the wave itself. | largest difference 2.9e-8 | yes |
| FX-AWAVE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-016 frame 0: Digital with Displayed Samples 60, more than the window's 48: some displayed samples share a source sample. | largest difference 2.9e-8 | yes |
| FX-AWAVE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-017 frame 0: Path 1, a mask (mode None) round the box from (2, 2) to (14, 8), 16 displayed samples, 2 tall at most: a closed line round the box. | largest difference 2.9e-8 | yes |
| FX-AWAVE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-018 frame 0: Path 2 with only one mask: none to draw along, so nothing is drawn and EFFECT_PATH_MISSING is said; the cel as it was. | largest difference 1.9e-7 | yes |
| FX-AWAVE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PATH_MISSING"] and ["EFFECT_PATH_MISSING"] | yes |
| FX-AWAVE-019 frame 0: Audio Layer "ghost", not in the composition: EFFECT_LAYER_MISSING is said and the cel is as it was. | largest difference 1.9e-7 | yes |
| FX-AWAVE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-AWAVE-020 frame 0: Audio Layer "art", the cel itself, which holds no sound: EFFECT_SOUND_MISSING is said and the cel is as it was. | largest difference 1.9e-7 | yes |
| FX-AWAVE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_SOUND_MISSING"] and ["EFFECT_SOUND_MISSING"] | yes |
| FX-AWAVE-021 frame 0: The 24-bit file: as FX-AWAVE-002 within a level. | largest difference 2.9e-8 | yes |
| FX-AWAVE-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-022 frame 0: The 32-bit floating point file. | largest difference 2.9e-8 | yes |
| FX-AWAVE-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-023 frame 0: The 8-bit file. | largest difference 3.0e-8 | yes |
| FX-AWAVE-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-024 frame 0: The sound layer starting at frame 2: frames 0 and 1 silent (a flat line), frame 4 its own frame 2. | largest difference 2.1e-8 | yes |
| FX-AWAVE-024 frame 4: The sound layer starting at frame 2: frames 0 and 1 silent (a flat line), frame 4 its own frame 2. | largest difference 3.0e-8 | yes |
| FX-AWAVE-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-025 frame 0: The sound layer at -6.0206 dB, half as loud: the wave half as tall. | largest difference 3.0e-8 | yes |
| FX-AWAVE-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-026 frame 0: The sound layer at +12 dB: four times as loud, held to Maximum Height. | largest difference 2.9e-8 | yes |
| FX-AWAVE-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-027 frame 0: Maximum Height keyed from 0 at frame 0 to 4 at frame 4, linear. | largest difference 2.1e-8 | yes |
| FX-AWAVE-027 frame 2: Maximum Height keyed from 0 at frame 0 to 4 at frame 4, linear. | largest difference 3.0e-8 | yes |
| FX-AWAVE-027 frame 4: Maximum Height keyed from 0 at frame 0 to 4 at frame 4, linear. | largest difference 3.0e-8 | yes |
| FX-AWAVE-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-028 frame 0: FX-AWAVE-002 moved three pixels right: the wave moves with the layer. | largest difference 3.0e-8 | yes |
| FX-AWAVE-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-029 frame 0: After a Motion Tile that grows the layer: the points are the drawing's own, so the frame is FX-AWAVE-002's. | largest difference 3.0e-8 | yes |
| FX-AWAVE-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-030 frame 0: Thickness 0: nothing drawn, the cel as it was. | largest difference 1.9e-7 | yes |
| FX-AWAVE-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-031 frame 0: Thickness 3, softness 100, orange inside, violet outside. | largest difference 3.0e-8 | yes |
| FX-AWAVE-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-032 frame 0: Start Point (90, 50), End Point (10, 50): drawn right to left, so the wave faces down. | largest difference 3.0e-8 | yes |
| FX-AWAVE-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-033 frame 0: A slant from (10, 20) to (90, 80). | largest difference 2.9e-8 | yes |
| FX-AWAVE-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-AWAVE-034 frame 0: Displayed Samples 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-034 frame 4: Displayed Samples 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AWAVE-035 frame 0: Displayed Samples 4097, above 4096. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-035 frame 4: Displayed Samples 4097, above 4096. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AWAVE-036 frame 0: Maximum Height -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-036 frame 4: Maximum Height -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AWAVE-037 frame 0: Audio Duration 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-037 frame 4: Audio Duration 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AWAVE-038 frame 0: Audio Offset -30001, below -30000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-038 frame 4: Audio Offset -30001, below -30000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AWAVE-039 frame 0: Thickness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-039 frame 4: Thickness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AWAVE-040 frame 0: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-040 frame 4: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AWAVE-041 frame 0: Random Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-041 frame 4: Random Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AWAVE-042 frame 0: Path 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-042 frame 4: Path 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AWAVE-043 frame 0: End Point -1001 per cent down, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-043 frame 4: End Point -1001 per cent down, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-043: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AWAVE-044 frame 0: Waveform Options "stereo", not mono, left or right. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-044 frame 4: Waveform Options "stereo", not mono, left or right. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-044: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AWAVE-045 frame 0: Display Options "bars", not digital, analog_lines or analog_dots. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-045 frame 4: Display Options "bars", not digital, analog_lines or analog_dots. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-045: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AWAVE-046 frame 0: Composite "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-046 frame 4: Composite "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-046: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AWAVE-047 frame 0: Outside colour "blue", not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-047 frame 4: Outside colour "blue", not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-047: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AWAVE-048 frame 0: Audio Layer 5, a number, not a layer's name. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-048 frame 4: Audio Layer 5, a number, not a layer's name. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-048: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-AWAVE-049 frame 0: Displayed Samples keyed to 4097 at frame 4, above 4096. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-049 frame 4: Displayed Samples keyed to 4097 at frame 4, above 4096. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-AWAVE-049: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_awave_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_awave_002.json is saved with every setting as written, the samples it heard not among them | {"audio_duration":10,"audio_layer":"sound","audio_offset":0,"composite":"off","display_options":"analog_lines","displayed_samples":8,"end_point":[90,50],"inside_color":"#ffffff","maximum_height":4,"outside_color":"#3c8cff","path":0,"random_seed":1,"softness":0,"start_point":[10,50],"thickness":2,"waveform_options":"mono"} | yes |
| fx_awave_034.json is refused in a sentence | Audio Waveform's displayed samples runs from 1 to 4096, and this is 0. | yes |
| fx_awave_035.json is refused in a sentence | Audio Waveform's displayed samples runs from 1 to 4096, and this is 4097. | yes |
| fx_awave_036.json is refused in a sentence | Audio Waveform's maximum height runs from 0 to 10000, and this is -1. | yes |
| fx_awave_037.json is refused in a sentence | Audio Waveform's audio duration runs from 1 to 30000, and this is 0. | yes |
| fx_awave_038.json is refused in a sentence | Audio Waveform's audio offset runs from -30000 to 30000, and this is -30001. | yes |
| fx_awave_039.json is refused in a sentence | Audio Waveform's thickness runs from 0 to 10000, and this is -1. | yes |
| fx_awave_040.json is refused in a sentence | Audio Waveform's softness runs from 0 to 100, and this is 101. | yes |
| fx_awave_041.json is refused in a sentence | Audio Waveform's random seed runs from 0 to 100000, and this is -1. | yes |
| fx_awave_042.json is refused in a sentence | Audio Waveform's path runs from 0 to 1000, and this is 1001. | yes |
| fx_awave_043.json is refused in a sentence | Audio Waveform's end point runs from -1000 to 1000, and this is -1001. | yes |
| fx_awave_044.json is refused in a sentence | Audio Waveform's waveform options is "mono", "left" or "right", and this is "stereo". | yes |
| fx_awave_045.json is refused in a sentence | Audio Waveform's display options is "digital", "analog_lines" or "analog_dots", and this is "bars". | yes |
| fx_awave_046.json is refused in a sentence | Audio Waveform's composite is "off" or "on", and this is "yes". | yes |
| fx_awave_047.json is refused in a sentence | Audio Waveform's outside colour is written #rrggbb, and this is "blue". | yes |
| fx_awave_048.json is refused in a sentence | Audio Waveform's audio layer is the name of a layer of this composition, and this is 5. | yes |
| a file with an Audio Waveform with no `displayed_samples` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Audio Waveform with a random seed in words is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Audio Waveform whose start point is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| displayed samples 0 is refused with a sentence, and nothing changes | Audio Waveform's displayed samples runs from 1 to 4096, and this is 0. | yes |
| waveform options "stereo" is refused with a sentence, and nothing changes | Audio Waveform's waveform options is "mono", "left" or "right", and this is "stereo". | yes |
| audio layer 5 is refused with a sentence, and nothing changes | Audio Waveform's audio layer is the name of a layer of this composition, and this is 5. | yes |
| random seed keyed to 100001 is refused with a sentence, and nothing changes | Audio Waveform's random seed runs from 0 to 100000, and this is 100001. | yes |
| 16 samples as digital strokes, left channel, orange, over the layer is taken | taken | yes |
| end point keyed from 90, 50 to 80, 20 is taken | taken | yes |
| random seed keyed from 1 to 9 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_awave_002.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_awave_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_awave_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_awave_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_awave_028.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_awave_029.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_awave_031.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_awave_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_awave_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_awave_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_awave_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_awave_034.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_awave_035.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_awave_036.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_awave_037.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_awave_038.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_awave_039.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_awave_040.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_awave_041.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_awave_042.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_awave_043.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_awave_044.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_awave_045.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_awave_046.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_awave_047.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_awave_048.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_awave_049.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Audio Waveform listening to the music, otherwise as it starts (32 points, an analog line along the middle, alone): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Audio Waveform listening to the music, otherwise as it starts (32 points, an analog line along the middle, alone) on three layers, frame 0, Full | largest difference 1 of 255, 9 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform listening to the music, otherwise as it starts (32 points, an analog line along the middle, alone) on three layers, frame 100, Full | largest difference 1 of 255, 10 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform listening to the music, otherwise as it starts (32 points, an analog line along the middle, alone) on three layers, frame 239, Full | largest difference 1 of 255, 5 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform listening to the music, otherwise as it starts (32 points, an analog line along the middle, alone) on three layers, frame 0, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform listening to the music, otherwise as it starts (32 points, an analog line along the middle, alone) on three layers, frame 100, Draft | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform listening to the music, otherwise as it starts (32 points, an analog line along the middle, alone) on three layers, frame 239, Draft | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform in silence, no Audio Layer (a flat line), thickness 10: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Audio Waveform in silence, no Audio Layer (a flat line), thickness 10 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform in silence, no Audio Layer (a flat line), thickness 10 on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform in silence, no Audio Layer (a flat line), thickness 10 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform in silence, no Audio Layer (a flat line), thickness 10 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform in silence, no Audio Layer (a flat line), thickness 10 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform in silence, no Audio Layer (a flat line), thickness 10 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform Digital, 512 samples over 200 ms, 600 tall, hairlines, softness 0: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Audio Waveform Digital, 512 samples over 200 ms, 600 tall, hairlines, softness 0 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform Digital, 512 samples over 200 ms, 600 tall, hairlines, softness 0 on three layers, frame 100, Full | largest difference 1 of 255, 4 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform Digital, 512 samples over 200 ms, 600 tall, hairlines, softness 0 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform Digital, 512 samples over 200 ms, 600 tall, hairlines, softness 0 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform Digital, 512 samples over 200 ms, 600 tall, hairlines, softness 0 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform Digital, 512 samples over 200 ms, 600 tall, hairlines, softness 0 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform Analog Dots, 128 samples, 800 tall, thickness 8, seed 9: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Audio Waveform Analog Dots, 128 samples, 800 tall, thickness 8, seed 9 on three layers, frame 0, Full | largest difference 1 of 255, 16 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform Analog Dots, 128 samples, 800 tall, thickness 8, seed 9 on three layers, frame 100, Full | largest difference 1 of 255, 15 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform Analog Dots, 128 samples, 800 tall, thickness 8, seed 9 on three layers, frame 239, Full | largest difference 1 of 255, 11 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform Analog Dots, 128 samples, 800 tall, thickness 8, seed 9 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform Analog Dots, 128 samples, 800 tall, thickness 8, seed 9 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform Analog Dots, 128 samples, 800 tall, thickness 8, seed 9 on three layers, frame 239, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform over the layer on a slant, 64 samples, thickness 30, orange to violet, from 40 ms before: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 477896 pixels changed | yes |
| the reference shot, Audio Waveform over the layer on a slant, 64 samples, thickness 30, orange to violet, from 40 ms before on three layers, frame 0, Full | largest difference 1 of 255, 1817 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform over the layer on a slant, 64 samples, thickness 30, orange to violet, from 40 ms before on three layers, frame 100, Full | largest difference 1 of 255, 1464 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform over the layer on a slant, 64 samples, thickness 30, orange to violet, from 40 ms before on three layers, frame 239, Full | largest difference 1 of 255, 1739 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform over the layer on a slant, 64 samples, thickness 30, orange to violet, from 40 ms before on three layers, frame 0, Draft | largest difference 1 of 255, 57 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform over the layer on a slant, 64 samples, thickness 30, orange to violet, from 40 ms before on three layers, frame 100, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform over the layer on a slant, 64 samples, thickness 30, orange to violet, from 40 ms before on three layers, frame 239, Draft | largest difference 1 of 255, 74 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform 4096 samples over 1000 ms, an analog line 2000 tall, softness 0: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Audio Waveform 4096 samples over 1000 ms, an analog line 2000 tall, softness 0 on three layers, frame 0, Full | largest difference 1 of 255, 363 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform 4096 samples over 1000 ms, an analog line 2000 tall, softness 0 on three layers, frame 100, Full | largest difference 1 of 255, 408 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform 4096 samples over 1000 ms, an analog line 2000 tall, softness 0 on three layers, frame 239, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform 4096 samples over 1000 ms, an analog line 2000 tall, softness 0 on three layers, frame 0, Draft | largest difference 1 of 255, 43 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform 4096 samples over 1000 ms, an analog line 2000 tall, softness 0 on three layers, frame 100, Draft | largest difference 1 of 255, 40 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Waveform 4096 samples over 1000 ms, an analog line 2000 tall, softness 0 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-421 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, the music as it starts, 900 tall: a white line edged blue along the middle, alone; draws cleanly | [], 129600 pixels changed | yes |
| 3_over_the_street.png, digital strokes along the foot of the street, over it, two seconds in; draws cleanly | [], 88076 pixels changed | yes |
| 4_neon_line.png, an orange neon line over the street, four seconds in; draws cleanly | [], 86706 pixels changed | yes |
| 5_slanted_dots.png, red dots on a slant up the street, over it, six seconds in; draws cleanly | [], 2251 pixels changed | yes |

## Result

297 of 297 checks pass.
