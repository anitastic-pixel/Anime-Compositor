# D-643: Compressor (B-522)

Each project is mixed by the core and held to the numbers `tools/compressor_reference.py` wrote before the code, by document 21's rule: the louder channel's level in dB, a knee curve centred on the threshold giving the wanted gain change, the change eased toward it per sample by exp(-1/t), t the attack when it must fall and the release when it may rise (a tenth of the release while Auto Release is on and it is more than 3 dB below), then Makeup Gain and an instant Output Limit on the louder channel. A keyed setting is read straight between its values on this frame and the next. The compressor works before the layer's level; effects run in order; one switched off or outside its ranges lets the sound through. Tolerance 2e-5. The test sounds are 1 kHz square waves, so the gain is read exactly off the mix: what the last compressor makes of what reached it. Pictures: `verification/D-643 pictures/`: waveforms (left channel above, right below, `_before` with the effects switched off) and gain curves (`_curve`: level in across, level out up, -60 to +12 dB, grey lines every 12 dB, the diagonal is no change, red the rule, blue the measured steps).

## fx_comp_001: the starting values on the staircase: the steady gain curve, the soft knee from -23.5 to -8.5 dB

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 150: 300000 samples | 300000 and 300000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 302 named places within 0.00002 | largest difference 2.78e-17 | yes |
| the steady curve: 61 steps within 0.01 dB of the rule (after the Output Limit) | largest difference 3.85e-8 dB | yes |

## fx_comp_002: Knee 0 and Ratio 4 on the staircase: a hard corner at -16 dB, a quarter of the rise above it

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 150: 300000 samples | 300000 and 300000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 302 named places within 0.00002 | largest difference 2.78e-17 | yes |
| the steady curve: 61 steps within 0.01 dB of the rule (after the Output Limit) | largest difference 4.33e-8 dB | yes |

## fx_comp_003: Ratio 0.4 (it expands above the threshold) and Release 1 ms: louder above -16 dB, held at full level by the Output Limit

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 150: 300000 samples | 300000 and 300000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 302 named places within 0.00002 | largest difference 1.11e-16 | yes |
| the steady curve: 61 steps within 0.01 dB of the rule (after the Output Limit) | largest difference 5.68e-14 dB | yes |

## fx_comp_004: a step from -40 to 0 dB and back, Knee 0, Ratio 4: the attack (6 ms) and the two-speed Auto Release

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 96: 192000 samples | 192000 and 192000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 204 named places within 0.00002 | largest difference 1.11e-16 | yes |

## fx_comp_005: the same step with Auto Release off: one slow release of 440 ms

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 96: 192000 samples | 192000 and 192000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 204 named places within 0.00002 | largest difference 1.11e-16 | yes |

## fx_comp_006: Attack 0: the cut is whole on the step's first sample

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 96: 192000 samples | 192000 and 192000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 199 named places within 0.00002 | largest difference 1.73e-18 | yes |

## fx_comp_007: Makeup Gain +12 dB and Output Limit -6 dB: louder, the loud part held at -6 dB

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 96: 192000 samples | 192000 and 192000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 204 named places within 0.00002 | largest difference 6.94e-18 | yes |

## fx_comp_008: Threshold keyed from -40 dB (frame 0) to -10 dB (frame 10) over a steady -20 dB: the cut eases off

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 24: 48000 samples | 48000 and 48000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 62 named places within 0.00002 | largest difference 1.39e-17 | yes |

## fx_comp_009: Ratio keyed from 1 (frame 0) to 20 (frame 12) at 29.97 over a steady -20 dB

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 24: 38438 samples | 38438 and 38438 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 52 named places within 0.00002 | largest difference 1.39e-17 | yes |

## fx_comp_010: frames 40 to 59 of fx_comp_004 alone: the same samples as the whole mix there

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 40 to 60: 40000 samples | 40000 and 40000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 42 named places within 0.00002 | largest difference 1.73e-18 | yes |

## fx_comp_011: a layer starting at frame 6 with a level of -6 dB: the compressor counts from the layer's start, before the level

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 96: 192000 samples | 192000 and 192000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 199 named places within 0.00002 | largest difference 5.55e-17 | yes |

## fx_comp_012: the compressor switched off: the sound unchanged

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 103 named places within 0.00002 | largest difference 0.00e0 | yes |

## fx_comp_013: a Noise Gate (Threshold -30 dB) then a compressor: the gate shuts the quiet part first

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 96: 192000 samples | 192000 and 192000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 204 named places within 0.00002 | largest difference 0.00e0 | yes |

## fx_comp_014: a Ratio of 50, outside 0.4 to 30: kept, EFFECT_PARAMETER_INVALID, the sound unchanged

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: EFFECT_PARAMETER_INVALID | EFFECT_PARAMETER_INVALID | yes |
| mixing warns of: EFFECT_PARAMETER_INVALID | EFFECT_PARAMETER_INVALID | yes |
| 103 named places within 0.00002 | largest difference 0.00e0 | yes |

## fx_comp_015: an Auto Release of "sometimes": kept, EFFECT_PARAMETER_INVALID, the sound unchanged

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: EFFECT_PARAMETER_INVALID | EFFECT_PARAMETER_INVALID | yes |
| mixing warns of: EFFECT_PARAMETER_INVALID | EFFECT_PARAMETER_INVALID | yes |
| 103 named places within 0.00002 | largest difference 0.00e0 | yes |

## fx_comp_016: the right channel's step drives both: the quiet left is turned down with it

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 96: 192000 samples | 192000 and 192000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 204 named places within 0.00002 | largest difference 1.11e-16 | yes |

## The timing table: how many samples a step takes to cover 63% and 90% of its change

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_comp_004: step at sample 24000, 0.00 to -12.00 dB: 63% after 286 samples (6.0 ms), 90% after 663 samples (13.8 ms) | 286 samples (6.0 ms), 663 samples (13.8 ms) | yes |
| fx_comp_004: step at sample 72000, -12.00 to 0.00 dB: 63% after 2099 samples (43.7 ms), 90% after 22278 samples (464.1 ms) | 2099 samples (43.7 ms), 22278 samples (464.1 ms) | yes |
| fx_comp_005: step at sample 24000, 0.00 to -12.00 dB: 63% after 286 samples (6.0 ms), 90% after 663 samples (13.8 ms) | 286 samples (6.0 ms), 663 samples (13.8 ms) | yes |
| fx_comp_005: step at sample 72000, -12.00 to 0.00 dB: 63% after 20998 samples (437.5 ms), 90% after 48630 samples (1013.1 ms) | 20998 samples (437.5 ms), 48630 samples (1013.1 ms) | yes |
| fx_comp_006: step at sample 24000, 0.00 to -12.00 dB: 63% after 0 samples (0.0 ms), 90% after 0 samples (0.0 ms) | 0 samples (0.0 ms), 0 samples (0.0 ms) | yes |
| fx_comp_006: step at sample 72000, -12.00 to 0.00 dB: 63% after 2099 samples (43.7 ms), 90% after 22278 samples (464.1 ms) | 2099 samples (43.7 ms), 22278 samples (464.1 ms) | yes |

## The curve table: every sixth step of each staircase, the steady gain change

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_comp_001: in -60.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_001: in -54.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_001: in -48.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_001: in -42.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_001: in -36.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_001: in -30.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_001: in -24.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_001: in -18.0 dB, change -0.672 dB | -0.672 dB | yes |
| fx_comp_001: in -12.0 dB, change -2.939 dB | -2.939 dB | yes |
| fx_comp_001: in -6.0 dB, change -6.667 dB | -6.667 dB | yes |
| fx_comp_001: in 0.0 dB, change -10.667 dB | -10.667 dB | yes |
| fx_comp_002: in -60.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_002: in -54.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_002: in -48.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_002: in -42.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_002: in -36.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_002: in -30.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_002: in -24.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_002: in -18.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_002: in -12.0 dB, change -3.000 dB | -3.000 dB | yes |
| fx_comp_002: in -6.0 dB, change -7.500 dB | -7.500 dB | yes |
| fx_comp_002: in 0.0 dB, change -12.000 dB | -12.000 dB | yes |
| fx_comp_003: in -60.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_003: in -54.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_003: in -48.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_003: in -42.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_003: in -36.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_003: in -30.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_003: in -24.0 dB, change 0.000 dB | 0.000 dB | yes |
| fx_comp_003: in -18.0 dB, change 1.512 dB | 1.512 dB | yes |
| fx_comp_003: in -12.0 dB, change 6.612 dB | 6.612 dB | yes |
| fx_comp_003: in -6.0 dB, change 6.000 dB | 6.000 dB | yes |
| fx_comp_003: in 0.0 dB, change 0.000 dB | -0.000 dB | yes |

## The level table: each channel's loudest sample, within 0.01 dB

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_comp_001: left -10.00 dB, right -10.00 dB | -10.00 dB, -10.00 dB | yes |
| fx_comp_002: left -11.25 dB, right -11.25 dB | -11.25 dB, -11.25 dB | yes |
| fx_comp_003: left 0.00 dB, right 0.00 dB | 0.00 dB, 0.00 dB | yes |
| fx_comp_004: left -0.04 dB, right -0.04 dB | -0.04 dB, -0.04 dB | yes |
| fx_comp_005: left -0.04 dB, right -0.04 dB | -0.04 dB, -0.04 dB | yes |
| fx_comp_006: left -12.00 dB, right -12.00 dB | -12.00 dB, -12.00 dB | yes |
| fx_comp_007: left -6.00 dB, right -6.00 dB | -6.00 dB, -6.00 dB | yes |
| fx_comp_008: left -20.05 dB, right -20.05 dB | -20.05 dB, -20.05 dB | yes |
| fx_comp_009: left -20.00 dB, right -20.00 dB | -20.00 dB, -20.00 dB | yes |
| fx_comp_010: left -40.36 dB, right -40.36 dB | -40.36 dB, -40.36 dB | yes |
| fx_comp_011: left -6.04 dB, right -6.04 dB | -6.04 dB, -6.04 dB | yes |
| fx_comp_012: left 0.00 dB, right 0.00 dB | 0.00 dB, 0.00 dB | yes |
| fx_comp_013: left -6.73 dB, right -6.73 dB | -6.73 dB, -6.73 dB | yes |
| fx_comp_014: left 0.00 dB, right 0.00 dB | 0.00 dB, 0.00 dB | yes |
| fx_comp_015: left 0.00 dB, right 0.00 dB | 0.00 dB, 0.00 dB | yes |
| fx_comp_016: left -40.00 dB, right -0.04 dB | -40.00 dB, -0.04 dB | yes |

## Refused projects

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_comp_101.json: Compressor on a solid layer is PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID | yes |
| fx_comp_102.json: an expression on a Compressor number is PROJECT_SCHEMA_INVALID: sound effects take keys only | PROJECT_SCHEMA_INVALID | yes |
| fx_comp_103.json: a Mix on Compressor is PROJECT_SCHEMA_INVALID: a sound effect has no Mix | PROJECT_SCHEMA_INVALID | yes |

## Saved again

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_comp_009.json: the layer's effects as written | as written | yes |
| fx_comp_015.json: the layer's effects as written | as written | yes |
| fx_comp_013.json: the layer's effects as written | as written | yes |

## Decision 3: sound effects on sound layers only, through the commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Compressor added to a sound layer | added | yes |
| Compressor added to a solid | refused: "solid 0" is not an audio layer, so it takes no Compressor. | yes |

## Speed (release build, this machine)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_comp_004, four seconds mixed: without the compressor, with it (milliseconds, recorded only) | 1.7 ms, 10.7 ms | yes |

## Result

131 of 131 checks pass.
