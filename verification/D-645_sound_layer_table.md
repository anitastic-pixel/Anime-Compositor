# D-645: a keyed level, pan, and sound through composition layers (B-524)

Each project is mixed by the core and held to the numbers `tools/sound_layer_reference.py` wrote before the code, by document 21's rules: an audio layer's level is a number or keys (-96 to +12 dB), its pan absent, a number or keys (-100 all left to +100 all right; left times min(1, 1 - p/100), right times min(1, 1 + p/100), so the centre is unchanged); a keyed one moves in a straight line across each frame's samples (the level in dB), after the layer's sound effects. A composition layer switched on is heard: frame f plays the inner composition's frame floor(source time) from its first sample, nothing outside the inner composition's frames. Tolerance 2e-5. Clicks are single samples of 0.5 twice a second, so where a click lands says where the sound landed (the click method). Pictures: `verification/D-645 pictures/`, waveforms with the left channel above and the right below.

## fx_slay_001: a level keyed from -24 dB (frame 0) to 0 dB (frame 24): it rises steadily in decibels, sample by sample

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 163 named places within 0.00002 | largest difference 2.78e-17 | yes |

## fx_slay_002: a level keyed from 0 dB (frame 12) to -96 dB (frame 36), after a Noise Gate at its starting values

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 113 named places within 0.00002 | largest difference 2.78e-17 | yes |

## fx_slay_003: pan -100: all left, the right channel silent, the left at unity

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 103 named places within 0.00002 | largest difference 2.78e-17 | yes |

## fx_slay_004: pan +50: the left at half, the right at unity

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 103 named places within 0.00002 | largest difference 2.78e-17 | yes |

## fx_slay_005: pan keyed from -100 (frame 0) to +100 (frame 24) at 29.97: it sweeps left to right

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 76876 samples | 76876 and 76876 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 91 named places within 0.00002 | largest difference 5.55e-17 | yes |

## fx_slay_006: a level of -6 dB and pan -30, both plain numbers

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 103 named places within 0.00002 | largest difference 2.78e-17 | yes |

## fx_slay_007: a composition layer showing a composition with the tone in it: the same sound as the tone on its own

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 103 named places within 0.00002 | largest difference 2.78e-17 | yes |

## fx_slay_008: a composition layer starting at frame 12: the inner composition's first click is heard at sample 24,000

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 113 named places within 0.00002 | largest difference 0.00e0 | yes |

## fx_slay_009: a composition layer with a source offset of 6 frames: the inner click at 24,000 is heard at sample 12,000

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 115 named places within 0.00002 | largest difference 0.00e0 | yes |

## fx_slay_010: the inner composition is 24 frames long and its sound layer runs on: nothing is heard after frame 24

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 103 named places within 0.00002 | largest difference 2.78e-17 | yes |

## fx_slay_011: two deep: a composition at frame 6 inside one at frame 6, its clicks at -6 dB: the first click at sample 24,000, at 0.2506

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 113 named places within 0.00002 | largest difference 0.00e0 | yes |

## fx_slay_012: a composition layer switched off: silence

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 98 named places within 0.00002 | largest difference 0.00e0 | yes |

## fx_slay_013: a 30 fps composition inside a 24 fps one: each outer frame plays its inner frame's samples onward (a logged gap)

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 110 named places within 0.00002 | largest difference 0.00e0 | yes |

## fx_slay_014: a composition layer at a stretch of 200: each inner frame is played for two outer frames (a logged gap)

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 108 named places within 0.00002 | largest difference 2.78e-17 | yes |

## fx_slay_015: frames 10 to 14 of fx_slay_001 alone: the same samples as the whole mix there

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 10 to 15: 10000 samples | 10000 and 10000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 17 named places within 0.00002 | largest difference 1.39e-17 | yes |

## fx_slay_016: frames 20 to 29 of fx_slay_011 alone: the same samples as the whole mix there

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 20 to 30: 20000 samples | 20000 and 20000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 27 named places within 0.00002 | largest difference 0.00e0 | yes |

## fx_slay_017: one composition shown twice, at frames 0 and 6: each click heard twice, 12,000 samples apart

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 115 named places within 0.00002 | largest difference 0.00e0 | yes |

## fx_slay_018: a composition layer naming a composition that is not there: silent, COMPOSITION_REFERENCE_MISSING on opening

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: COMPOSITION_REFERENCE_MISSING | COMPOSITION_REFERENCE_MISSING | yes |
| mixing warns of: none | none | yes |
| 103 named places within 0.00002 | largest difference 1.39e-17 | yes |

## The click table: every sample louder than 0.1, exactly where the reference has it

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_slay_008: 24000 (0.5000), 48000 (0.5000), 72000 (0.5000) | the same samples | yes |
| fx_slay_009: 12000 (0.5000), 36000 (0.5000), 60000 (0.5000) | the same samples | yes |
| fx_slay_011: 24000 (0.2506), 48000 (0.2506), 72000 (0.2506) | the same samples | yes |
| fx_slay_013: 0 (0.5000), 29600 (0.5000), 30000 (0.5000), 59600 (0.5000), 60000 (0.5000), 89600 (0.5000), 90000 (0.5000) | the same samples | yes |
| fx_slay_016: 48000 (0.2506) | the same samples | yes |
| fx_slay_017: 0 (0.5000), 12000 (0.5000), 24000 (0.5000), 36000 (0.5000), 48000 (0.5000), 60000 (0.5000), 72000 (0.5000), 84000 (0.5000) | the same samples | yes |

## The level table: each channel's loudest sample, within 0.01 dB

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_slay_001: left -6.00 dB, right -6.00 dB | -6.00 dB, -6.00 dB | yes |
| fx_slay_002: left -6.00 dB, right -6.00 dB | -6.00 dB, -6.00 dB | yes |
| fx_slay_003: left -6.00 dB, right silent | -6.00 dB, silent | yes |
| fx_slay_004: left -12.02 dB, right -6.00 dB | -12.02 dB, -6.00 dB | yes |
| fx_slay_005: left -6.00 dB, right -6.00 dB | -6.00 dB, -6.00 dB | yes |
| fx_slay_006: left -12.00 dB, right -15.10 dB | -12.00 dB, -15.10 dB | yes |
| fx_slay_007: left -6.00 dB, right -6.00 dB | -6.00 dB, -6.00 dB | yes |
| fx_slay_008: left -6.02 dB, right -6.02 dB | -6.02 dB, -6.02 dB | yes |
| fx_slay_009: left -6.02 dB, right -6.02 dB | -6.02 dB, -6.02 dB | yes |
| fx_slay_010: left -6.00 dB, right -6.00 dB | -6.00 dB, -6.00 dB | yes |
| fx_slay_011: left -12.02 dB, right -12.02 dB | -12.02 dB, -12.02 dB | yes |
| fx_slay_012: left silent, right silent | silent, silent | yes |
| fx_slay_013: left -6.02 dB, right -6.02 dB | -6.02 dB, -6.02 dB | yes |
| fx_slay_014: left -6.00 dB, right -6.00 dB | -6.00 dB, -6.00 dB | yes |
| fx_slay_015: left -15.01 dB, right -15.01 dB | -15.01 dB, -15.01 dB | yes |
| fx_slay_016: left -12.02 dB, right -12.02 dB | -12.02 dB, -12.02 dB | yes |
| fx_slay_017: left -6.02 dB, right -6.02 dB | -6.02 dB, -6.02 dB | yes |
| fx_slay_018: left -18.00 dB, right -18.00 dB | -18.00 dB, -18.00 dB | yes |

## Refused projects

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_slay_101.json: a keyed level with a key at +20 dB, above +12 | PROJECT_SCHEMA_INVALID | yes |
| fx_slay_102.json: pan 150, outside -100 to +100 | PROJECT_SCHEMA_INVALID | yes |
| fx_slay_103.json: an expression on the level: a level takes keys only | PROJECT_SCHEMA_INVALID | yes |
| fx_slay_104.json: pan keyed to -120 | PROJECT_SCHEMA_INVALID | yes |
| fx_slay_105.json: pan written as a word | PROJECT_SCHEMA_INVALID | yes |

## Saved again: every level and pan as written

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_slay_001.json | as written | yes |
| fx_slay_002.json | as written | yes |
| fx_slay_003.json | as written | yes |
| fx_slay_004.json | as written | yes |
| fx_slay_005.json | as written | yes |
| fx_slay_006.json | as written | yes |
| fx_slay_007.json | as written | yes |
| fx_slay_008.json | as written | yes |
| fx_slay_009.json | as written | yes |
| fx_slay_010.json | as written | yes |
| fx_slay_011.json | as written | yes |
| fx_slay_012.json | as written | yes |
| fx_slay_013.json | as written | yes |
| fx_slay_014.json | as written | yes |
| fx_slay_015.json | as written | yes |
| fx_slay_016.json | as written | yes |
| fx_slay_017.json | as written | yes |
| fx_slay_018.json | as written | yes |

## The level command on a keyed level

| Check | The build's answer | Matches |
| --- | --- | --- |
| set to -3 dB: the number and the keys' base are -3, the two keys stay | set; number -3, base Some(-3.0), 2 keys | yes |
| and the sound is the keys' sound, unchanged | unchanged | yes |

## Which compositions have sound (an MP4 gets a sound track when one does)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_slay_007.json: has sound | has sound | yes |
| fx_slay_011.json: has sound | has sound | yes |
| fx_slay_012.json: has none | has none | yes |
| fx_slay_018.json: has sound | has sound | yes |

## Speed (release build, this machine)

| Check | The build's answer | Matches |
| --- | --- | --- |
| two seconds mixed: a plain level and pan, a keyed level, two compositions deep (milliseconds, recorded only) | 2.6 ms, 1.9 ms, 1.2 ms | yes |

## Result

126 of 126 checks pass.
