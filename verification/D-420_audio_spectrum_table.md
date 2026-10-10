# D-420: Audio Spectrum

B-299, after After Effects' Audio Spectrum: the levels of a sound layer at Frequency Bands frequencies evenly spaced from Start to End Frequency, each 2 |sum x_k w_k e^(-2 pi i f k / rate)| / sum w_k over Audio Duration from the frame's first sample plus Audio Offset (Hann window, channels averaged, the layer's gain applied; Duration Averaging the mean over three windows half a window apart), drawn as bars (Digital), a line through their tips (Analog Lines) or dots (Analog Dots), Maximum Height times the level (at most 1) tall, along the line from Start to End Point, round Start Point (Use Polar Path) or round a mask (Path), each mark Beam's line (D-207). Every expected pixel is `Fixtures/audio_spectrum/expected_audio_spectrum.json`, written by `tools/audio_spectrum_reference.py` before this code existed and printed in document 25 as FX-ASPEC-001 to 053. Tolerance 2e-5. The levels are worked out on the processor and the marks drawn on the card.

## FX-ASPEC-001 to 053 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-ASPEC-001 frame 0: The settings as they start, no Audio Layer: silence, so 64 dots of Side A and Side B together, 3 thick, along the line through the middle, alone. | largest difference 3.0e-8 | yes |
| FX-ASPEC-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-002 frame 0: Four bands at 300, 600, 900 and 1200 Hz, 8 tall at most, Side A, 2 thick, sharp, alone: the 300 Hz bar half as tall as it can be (4), the 1200 Hz bar a quarter (2), the two between all but nothing. | largest difference 2.8e-8 | yes |
| FX-ASPEC-002 frame 2: Four bands at 300, 600, 900 and 1200 Hz, 8 tall at most, Side A, 2 thick, sharp, alone: the 300 Hz bar half as tall as it can be (4), the 1200 Hz bar a quarter (2), the two between all but nothing. | largest difference 2.7e-8 | yes |
| FX-ASPEC-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-003 frame 0: FX-ASPEC-002 with Composite On Original on: over the cel. | largest difference 1.9e-7 | yes |
| FX-ASPEC-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-004 frame 0: Side B: the bars hang down. | largest difference 2.7e-8 | yes |
| FX-ASPEC-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-005 frame 0: Side A & B: each bar both ways. | largest difference 2.8e-8 | yes |
| FX-ASPEC-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-006 frame 0: Analog Lines: one line through the bars' tips. | largest difference 2.9e-8 | yes |
| FX-ASPEC-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-007 frame 0: Analog Dots: a dot at each tip. | largest difference 2.8e-8 | yes |
| FX-ASPEC-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-008 frame 0: Thickness 3, softness 100. | largest difference 2.9e-8 | yes |
| FX-ASPEC-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-009 frame 0: Orange inside, violet outside, Hue Interpolation 180, thickness 3: each band's colours turned further round the hue. | largest difference 2.8e-8 | yes |
| FX-ASPEC-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-010 frame 0: FX-ASPEC-009 with Color Symmetry on: the first and last bands match. | largest difference 2.8e-8 | yes |
| FX-ASPEC-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-011 frame 0: FX-ASPEC-009 with Dynamic Hue Phase on: the turn starts at the loudest band, 300 Hz, the first, so the frame is FX-ASPEC-009's. | largest difference 2.8e-8 | yes |
| FX-ASPEC-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-012 frame 0: Eight bands at 300, 450 .. 1350 Hz, thickness 3, Side A & B, Blend Overlapping Colors on, orange inside: neighbouring bars overlap and their colours are blended. | largest difference 2.9e-8 | yes |
| FX-ASPEC-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-013 frame 0: FX-ASPEC-012 with Blend Overlapping Colors off: each bar laid over the last. | largest difference 2.9e-8 | yes |
| FX-ASPEC-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-014 frame 0: Duration Averaging on: three windows half a window apart, averaged. | largest difference 2.9e-8 | yes |
| FX-ASPEC-014 frame 2: Duration Averaging on: three windows half a window apart, averaged. | largest difference 2.8e-8 | yes |
| FX-ASPEC-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-015 frame 0: Audio Offset 50 ms: the window starts 240 samples later. | largest difference 2.9e-8 | yes |
| FX-ASPEC-015 frame 2: Audio Offset 50 ms: the window starts 240 samples later. | largest difference 2.9e-8 | yes |
| FX-ASPEC-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-016 frame 0: Audio Duration 20 ms: a window of 96 samples, too short to keep 300 Hz from 600 Hz, so the levels spread. | largest difference 2.8e-8 | yes |
| FX-ASPEC-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-017 frame 0: Use Polar Path on about the middle, eight bands at 300, 450 .. 1350 Hz, 4 tall at most: the bars stand out round the centre from straight up, clockwise. | largest difference 2.3e-8 | yes |
| FX-ASPEC-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-018 frame 0: FX-ASPEC-017 as Analog Lines: a closed line round the centre. | largest difference 2.3e-8 | yes |
| FX-ASPEC-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-019 frame 0: Path 1, a mask (mode None) round the box from (2, 2) to (14, 8), eight bands, 3 tall at most, Side A: the bars stand outward round the box. | largest difference 2.6e-8 | yes |
| FX-ASPEC-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-020 frame 0: Path 2 with only one mask: none to draw along, so nothing is drawn and EFFECT_PATH_MISSING is said; the cel as it was. | largest difference 1.9e-7 | yes |
| FX-ASPEC-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PATH_MISSING"] and ["EFFECT_PATH_MISSING"] | yes |
| FX-ASPEC-021 frame 0: Audio Layer "ghost", not in the composition: EFFECT_LAYER_MISSING is said and the cel is as it was. | largest difference 1.9e-7 | yes |
| FX-ASPEC-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-ASPEC-022 frame 0: Audio Layer "art", the cel itself, which holds no sound: EFFECT_SOUND_MISSING is said and the cel is as it was. | largest difference 1.9e-7 | yes |
| FX-ASPEC-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_SOUND_MISSING"] and ["EFFECT_SOUND_MISSING"] | yes |
| FX-ASPEC-023 frame 0: The stereo file, bands at 600 and 900 Hz among the four: its two channels averaged, 0.3 at 600 Hz and 0.15 at 900 Hz. | largest difference 2.9e-8 | yes |
| FX-ASPEC-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-024 frame 0: The 24-bit file: as FX-ASPEC-002 within the tolerance. | largest difference 3.0e-8 | yes |
| FX-ASPEC-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-025 frame 0: The 32-bit floating point file. | largest difference 2.8e-8 | yes |
| FX-ASPEC-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-026 frame 0: The 8-bit file. | largest difference 2.8e-8 | yes |
| FX-ASPEC-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-027 frame 0: The sound layer starting at frame 2: frames 0 and 1 silent, frame 4 its own frame 2. | largest difference 2.6e-8 | yes |
| FX-ASPEC-027 frame 4: The sound layer starting at frame 2: frames 0 and 1 silent, frame 4 its own frame 2. | largest difference 2.7e-8 | yes |
| FX-ASPEC-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-028 frame 0: The sound layer at -6.0206 dB, half as loud: the bars half as tall. | largest difference 2.7e-8 | yes |
| FX-ASPEC-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-029 frame 0: Maximum Height keyed from 0 at frame 0 to 8 at frame 4, linear. | largest difference 2.6e-8 | yes |
| FX-ASPEC-029 frame 2: Maximum Height keyed from 0 at frame 0 to 8 at frame 4, linear. | largest difference 3.0e-8 | yes |
| FX-ASPEC-029 frame 4: Maximum Height keyed from 0 at frame 0 to 8 at frame 4, linear. | largest difference 2.7e-8 | yes |
| FX-ASPEC-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-030 frame 0: FX-ASPEC-002 moved three pixels right: the bars move with the layer. | largest difference 2.8e-8 | yes |
| FX-ASPEC-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-031 frame 0: After a Motion Tile that grows the layer: the points are the drawing's own, so the frame is FX-ASPEC-002's. | largest difference 2.8e-8 | yes |
| FX-ASPEC-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-032 frame 0: One band, 150 to 450 Hz: one bar at 300 Hz in the middle of the line. | largest difference 2.1e-8 | yes |
| FX-ASPEC-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-033 frame 0: Start Frequency 1350, End Frequency 150: the bands in the other order, 1200 Hz first. | largest difference 2.8e-8 | yes |
| FX-ASPEC-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-034 frame 0: Thickness 0: nothing drawn, the cel as it was. | largest difference 1.9e-7 | yes |
| FX-ASPEC-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ASPEC-035 frame 0: Frequency Bands 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-035 frame 4: Frequency Bands 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ASPEC-036 frame 0: Frequency Bands 4097, above 4096. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-036 frame 4: Frequency Bands 4097, above 4096. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ASPEC-037 frame 0: Start Frequency 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-037 frame 4: Start Frequency 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ASPEC-038 frame 0: End Frequency 20001, above 20000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-038 frame 4: End Frequency 20001, above 20000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ASPEC-039 frame 0: Maximum Height -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-039 frame 4: Maximum Height -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ASPEC-040 frame 0: Audio Duration 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-040 frame 4: Audio Duration 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ASPEC-041 frame 0: Audio Offset 30001, above 30000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-041 frame 4: Audio Offset 30001, above 30000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ASPEC-042 frame 0: Thickness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-042 frame 4: Thickness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ASPEC-043 frame 0: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-043 frame 4: Softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-043: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ASPEC-044 frame 0: Hue Interpolation 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-044 frame 4: Hue Interpolation 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-044: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ASPEC-045 frame 0: Path -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-045 frame 4: Path -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-045: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ASPEC-046 frame 0: Start Point 1001 per cent across, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-046 frame 4: Start Point 1001 per cent across, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-046: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ASPEC-047 frame 0: Display Options "bars", not digital, analog_lines or analog_dots. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-047 frame 4: Display Options "bars", not digital, analog_lines or analog_dots. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-047: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ASPEC-048 frame 0: Side Options "side_c". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-048 frame 4: Side Options "side_c". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-048: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ASPEC-049 frame 0: Composite "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-049 frame 4: Composite "yes", not on or off. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-049: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ASPEC-050 frame 0: Inside colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-050 frame 4: Inside colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-050: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ASPEC-051 frame 0: Use Polar Path "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-051 frame 4: Use Polar Path "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-051: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ASPEC-052 frame 0: Audio Layer 5, a number, not a layer's name. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-052 frame 4: Audio Layer 5, a number, not a layer's name. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-052: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ASPEC-053 frame 0: Frequency Bands keyed to 4097 at frame 4, above 4096. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-053 frame 4: Frequency Bands keyed to 4097 at frame 4, above 4096. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-ASPEC-053: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_aspec_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_aspec_002.json is saved with every setting as written, the levels it heard not among them | {"audio_duration":90,"audio_layer":"sound","audio_offset":0,"blend_overlapping_colors":"off","color_symmetry":"off","composite":"off","display_options":"digital","duration_averaging":"off","dynamic_hue_phase":"off","end_frequency":1350,"end_point":[90,80],"frequency_bands":4,"hue_interpolation":0,"inside_color":"#ffffff","maximum_height":8,"outside_color":"#3c8cff","path":0,"side_options":"side_a","softness":0,"start_frequency":150,"start_point":[10,80],"thickness":2,"use_polar_path":"off"} | yes |
| fx_aspec_035.json is refused in a sentence | Audio Spectrum's frequency bands runs from 1 to 4096, and this is 0. | yes |
| fx_aspec_036.json is refused in a sentence | Audio Spectrum's frequency bands runs from 1 to 4096, and this is 4097. | yes |
| fx_aspec_037.json is refused in a sentence | Audio Spectrum's start frequency runs from 1 to 20000, and this is 0. | yes |
| fx_aspec_038.json is refused in a sentence | Audio Spectrum's end frequency runs from 1 to 20000, and this is 20001. | yes |
| fx_aspec_039.json is refused in a sentence | Audio Spectrum's maximum height runs from 0 to 10000, and this is -1. | yes |
| fx_aspec_040.json is refused in a sentence | Audio Spectrum's audio duration runs from 1 to 30000, and this is 0. | yes |
| fx_aspec_041.json is refused in a sentence | Audio Spectrum's audio offset runs from -30000 to 30000, and this is 30001. | yes |
| fx_aspec_042.json is refused in a sentence | Audio Spectrum's thickness runs from 0 to 10000, and this is -1. | yes |
| fx_aspec_043.json is refused in a sentence | Audio Spectrum's softness runs from 0 to 100, and this is 101. | yes |
| fx_aspec_044.json is refused in a sentence | Audio Spectrum's hue interpolation runs from -3600 to 3600, and this is 3601. | yes |
| fx_aspec_045.json is refused in a sentence | Audio Spectrum's path runs from 0 to 1000, and this is -1. | yes |
| fx_aspec_046.json is refused in a sentence | Audio Spectrum's start point runs from -1000 to 1000, and this is 1001. | yes |
| fx_aspec_047.json is refused in a sentence | Audio Spectrum's display options is "digital", "analog_lines" or "analog_dots", and this is "bars". | yes |
| fx_aspec_048.json is refused in a sentence | Audio Spectrum's side options is "side_a", "side_b" or "side_a_b", and this is "side_c". | yes |
| fx_aspec_049.json is refused in a sentence | Audio Spectrum's composite is "off" or "on", and this is "yes". | yes |
| fx_aspec_050.json is refused in a sentence | Audio Spectrum's inside colour is written #rrggbb, and this is "#12345". | yes |
| fx_aspec_051.json is refused in a sentence | Audio Spectrum's use polar path is "off" or "on", and this is "maybe". | yes |
| fx_aspec_052.json is refused in a sentence | Audio Spectrum's audio layer is the name of a layer of this composition, and this is 5. | yes |
| a file with an Audio Spectrum with no `frequency_bands` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Audio Spectrum with a thickness in words is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Audio Spectrum whose end point is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| frequency bands 0 is refused with a sentence, and nothing changes | Audio Spectrum's frequency bands runs from 1 to 4096, and this is 0. | yes |
| display options "bars" is refused with a sentence, and nothing changes | Audio Spectrum's display options is "digital", "analog_lines" or "analog_dots", and this is "bars". | yes |
| audio layer 5 is refused with a sentence, and nothing changes | Audio Spectrum's audio layer is the name of a layer of this composition, and this is 5. | yes |
| maximum height keyed to 10001 is refused with a sentence, and nothing changes | Audio Spectrum's maximum height runs from 0 to 10000, and this is 10001. | yes |
| 32 bands as analog lines round the middle, orange, blended is taken | taken | yes |
| start point keyed from 10, 80 to 20, 50 is taken | taken | yes |
| thickness keyed from 1 to 3 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_aspec_002.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_aspec_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_aspec_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_aspec_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_aspec_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_aspec_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_aspec_031.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_aspec_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_022.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_aspec_035.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_036.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_037.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_038.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_039.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_040.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_041.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_042.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_043.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_044.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_045.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_046.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_047.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_048.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_049.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_050.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_051.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_052.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_aspec_053.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Audio Spectrum as it starts, listening to the music (64 bars both ways along the middle, alone): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Audio Spectrum as it starts, listening to the music (64 bars both ways along the middle, alone) on three layers, frame 0, Full | largest difference 1 of 255, 4 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum as it starts, listening to the music (64 bars both ways along the middle, alone) on three layers, frame 100, Full | largest difference 1 of 255, 4 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum as it starts, listening to the music (64 bars both ways along the middle, alone) on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum as it starts, listening to the music (64 bars both ways along the middle, alone) on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum as it starts, listening to the music (64 bars both ways along the middle, alone) on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum as it starts, listening to the music (64 bars both ways along the middle, alone) on three layers, frame 239, Draft | largest difference 1 of 255, 4 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum in silence, no Audio Layer (64 dots along the middle), thickness 10: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Audio Spectrum in silence, no Audio Layer (64 dots along the middle), thickness 10 on three layers, frame 0, Full | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum in silence, no Audio Layer (64 dots along the middle), thickness 10 on three layers, frame 100, Full | largest difference 1 of 255, 66 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum in silence, no Audio Layer (64 dots along the middle), thickness 10 on three layers, frame 239, Full | largest difference 1 of 255, 66 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum in silence, no Audio Layer (64 dots along the middle), thickness 10 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum in silence, no Audio Layer (64 dots along the middle), thickness 10 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum in silence, no Audio Layer (64 dots along the middle), thickness 10 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum Analog Lines, Side A, 1500 tall, thickness 6, softness 80: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Audio Spectrum Analog Lines, Side A, 1500 tall, thickness 6, softness 80 on three layers, frame 0, Full | largest difference 1 of 255, 15 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum Analog Lines, Side A, 1500 tall, thickness 6, softness 80 on three layers, frame 100, Full | largest difference 1 of 255, 19 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum Analog Lines, Side A, 1500 tall, thickness 6, softness 80 on three layers, frame 239, Full | largest difference 1 of 255, 25 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum Analog Lines, Side A, 1500 tall, thickness 6, softness 80 on three layers, frame 0, Draft | largest difference 1 of 255, 8 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum Analog Lines, Side A, 1500 tall, thickness 6, softness 80 on three layers, frame 100, Draft | largest difference 1 of 255, 5 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum Analog Lines, Side A, 1500 tall, thickness 6, softness 80 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum Analog Dots round the middle, 128 bands, 800 tall, thickness 8: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Audio Spectrum Analog Dots round the middle, 128 bands, 800 tall, thickness 8 on three layers, frame 0, Full | largest difference 1 of 255, 24 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum Analog Dots round the middle, 128 bands, 800 tall, thickness 8 on three layers, frame 100, Full | largest difference 1 of 255, 3 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum Analog Dots round the middle, 128 bands, 800 tall, thickness 8 on three layers, frame 239, Full | largest difference 1 of 255, 6 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum Analog Dots round the middle, 128 bands, 800 tall, thickness 8 on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum Analog Dots round the middle, 128 bands, 800 tall, thickness 8 on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum Analog Dots round the middle, 128 bands, 800 tall, thickness 8 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum 32 bars blended, thickness 40, Hue Interpolation 360, Dynamic Hue Phase and Color Symmetry: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Audio Spectrum 32 bars blended, thickness 40, Hue Interpolation 360, Dynamic Hue Phase and Color Symmetry on three layers, frame 0, Full | largest difference 1 of 255, 132 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum 32 bars blended, thickness 40, Hue Interpolation 360, Dynamic Hue Phase and Color Symmetry on three layers, frame 100, Full | largest difference 1 of 255, 98 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum 32 bars blended, thickness 40, Hue Interpolation 360, Dynamic Hue Phase and Color Symmetry on three layers, frame 239, Full | largest difference 1 of 255, 178 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum 32 bars blended, thickness 40, Hue Interpolation 360, Dynamic Hue Phase and Color Symmetry on three layers, frame 0, Draft | largest difference 1 of 255, 16 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum 32 bars blended, thickness 40, Hue Interpolation 360, Dynamic Hue Phase and Color Symmetry on three layers, frame 100, Draft | largest difference 1 of 255, 20 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum 32 bars blended, thickness 40, Hue Interpolation 360, Dynamic Hue Phase and Color Symmetry on three layers, frame 239, Draft | largest difference 1 of 255, 4 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum Side B over the layer, Duration Averaging, 200 ms from 40 ms before: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1755 pixels changed | yes |
| the reference shot, Audio Spectrum Side B over the layer, Duration Averaging, 200 ms from 40 ms before on three layers, frame 0, Full | largest difference 1 of 255, 3779 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum Side B over the layer, Duration Averaging, 200 ms from 40 ms before on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum Side B over the layer, Duration Averaging, 200 ms from 40 ms before on three layers, frame 239, Full | largest difference 1 of 255, 1767 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum Side B over the layer, Duration Averaging, 200 ms from 40 ms before on three layers, frame 0, Draft | largest difference 1 of 255, 130 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum Side B over the layer, Duration Averaging, 200 ms from 40 ms before on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum Side B over the layer, Duration Averaging, 200 ms from 40 ms before on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum 1024 bands, hairlines 1 thick, softness 0, 2000 tall: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Audio Spectrum 1024 bands, hairlines 1 thick, softness 0, 2000 tall on three layers, frame 0, Full | largest difference 1 of 255, 8 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum 1024 bands, hairlines 1 thick, softness 0, 2000 tall on three layers, frame 100, Full | largest difference 1 of 255, 8 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum 1024 bands, hairlines 1 thick, softness 0, 2000 tall on three layers, frame 239, Full | largest difference 1 of 255, 11 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum 1024 bands, hairlines 1 thick, softness 0, 2000 tall on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum 1024 bands, hairlines 1 thick, softness 0, 2000 tall on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Audio Spectrum 1024 bands, hairlines 1 thick, softness 0, 2000 tall on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-420 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, the music as it starts, 600 tall: white bars edged blue both ways along the middle, alone; draws cleanly | [], 129600 pixels changed | yes |
| 3_over_the_street.png, bars standing up from the foot of the street, over it, two seconds in; draws cleanly | [], 3132 pixels changed | yes |
| 4_neon_line.png, an orange neon line over the street, four seconds in; draws cleanly | [], 6910 pixels changed | yes |
| 5_ring_of_dots.png, a rainbow ring of dots round the middle, over the street, six seconds in; draws cleanly | [], 1636 pixels changed | yes |

## Result

328 of 328 checks pass.
