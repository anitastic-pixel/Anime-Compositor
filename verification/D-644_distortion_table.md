# D-644: Distortion (B-523)

Each project is mixed by the core and held to the numbers `tools/distortion_reference.py` wrote before the code, by document 21's rule: each channel's sample times 10^(drive / 50) (Drive 0 to 100% is 0 to +40 dB) through the type's curve (Soft Clip tanh, Hard Clip held to -1..1, Saturation 1 u/(1+|u|), Saturation 2 (2/pi) atan(pi u/2), Tube a lopsided tanh, Fuzz a square root held to 1), then Resolution's 2^(bits-1) steps and Downsample's hold counted from the layer's first sample; that times gain / 25, mixed with the clean sound by Mix, then times volume / 11. A keyed setting is read straight between its values on this frame and the next. The effect works before the layer's level; effects run in order; one switched off or outside its ranges lets the sound through. No oversampling: a high Drive folds harmonics above 24 kHz back down (a logged ceiling). Tolerance 2e-5. The test sound is a 1 kHz sine at -6 dB for half a second, then a ramp from -1 to +1 over a second. Pictures: `verification/D-644 pictures/`: each type's curve (`_curve`: sample in across, -1 to +1, out up, -1.5 to +1.5, grey the diagonal, blue the measured ramp), two cycles of the sine (`_sine`: distorted above, clean below), and waveforms (left channel above, right below, `_before` with the effects switched off).

## fx_dist_001: Soft Clip at the starting values (Drive 25%, +10 dB into the curve): the sine's harmonics and the ramp's curve

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 203 named places within 0.00002 | largest difference 1.11e-16 | yes |
| the curve: 21 ramp samples within 0.00002 | largest difference 1.11e-16 | yes |

## fx_dist_002: Hard Clip at the starting values (Drive 25%, +10 dB into the curve): the sine's harmonics and the ramp's curve

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 203 named places within 0.00002 | largest difference 1.11e-16 | yes |
| the curve: 21 ramp samples within 0.00002 | largest difference 0.00e0 | yes |

## fx_dist_003: Saturation 1 at the starting values (Drive 25%, +10 dB into the curve): the sine's harmonics and the ramp's curve

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 203 named places within 0.00002 | largest difference 5.55e-17 | yes |
| the curve: 21 ramp samples within 0.00002 | largest difference 0.00e0 | yes |

## fx_dist_004: Saturation 2 at the starting values (Drive 25%, +10 dB into the curve): the sine's harmonics and the ramp's curve

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 203 named places within 0.00002 | largest difference 5.55e-17 | yes |
| the curve: 21 ramp samples within 0.00002 | largest difference 5.55e-17 | yes |

## fx_dist_005: Tube at the starting values (Drive 25%, +10 dB into the curve): the sine's harmonics and the ramp's curve

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 203 named places within 0.00002 | largest difference 2.22e-16 | yes |
| the curve: 21 ramp samples within 0.00002 | largest difference 1.11e-16 | yes |

## fx_dist_006: Fuzz at the starting values (Drive 25%, +10 dB into the curve): the sine's harmonics and the ramp's curve

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 203 named places within 0.00002 | largest difference 1.11e-16 | yes |
| the curve: 21 ramp samples within 0.00002 | largest difference 1.11e-16 | yes |

## fx_dist_007: Hard Clip, Drive 0: nothing above full level, so the sound is unchanged

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 108 named places within 0.00002 | largest difference 1.11e-16 | yes |

## fx_dist_008: Gain 100 (four times the curve), Mix 50%, Volume 5.5 (half)

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 108 named places within 0.00002 | largest difference 2.22e-16 | yes |

## fx_dist_009: Resolution 4 bits, Drive 0, Hard Clip: every sample on a step of 1/8

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 108 named places within 0.00002 | largest difference 0.00e0 | yes |
| Resolution 4: every sample on a step of 1/8 (exactly) | largest distance from a step 0.0e0 | yes |

## fx_dist_010: Downsample 4, Drive 0, Hard Clip: each sample held for four

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 108 named places within 0.00002 | largest difference 1.11e-16 | yes |
| Downsample 4: each run of four samples from the layer's first is one value (exactly) | held; 18000 of 23999 runs differ from the next | yes |

## fx_dist_011: Drive keyed from 0% (frame 0) to 100% (frame 10) on the sine: it grows steadily harsher

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 24: 48000 samples | 48000 and 48000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 62 named places within 0.00002 | largest difference 1.11e-16 | yes |

## fx_dist_012: Downsample keyed from 1 (frame 0) to 32 (frame 12) at 29.97

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 24: 38438 samples | 38438 and 38438 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 52 named places within 0.00002 | largest difference 1.11e-16 | yes |

## fx_dist_013: frames 5 to 14 of fx_dist_010 alone: the same samples as the whole mix there (the hold counts from the layer's start)

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 5 to 15: 20000 samples | 20000 and 20000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 24 named places within 0.00002 | largest difference 1.11e-16 | yes |

## fx_dist_014: a layer starting at frame 7 with a level of -6 dB, Downsample 3: the hold counts from the layer's start, before the level

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 108 named places within 0.00002 | largest difference 5.55e-17 | yes |

## fx_dist_015: two in a row: Fuzz then Soft Clip

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 108 named places within 0.00002 | largest difference 1.11e-16 | yes |

## fx_dist_016: switched off: the sound unchanged

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 103 named places within 0.00002 | largest difference 1.11e-16 | yes |

## fx_dist_017: a Drive of 150, outside 0 to 100: kept, EFFECT_PARAMETER_INVALID, the sound unchanged

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: EFFECT_PARAMETER_INVALID | EFFECT_PARAMETER_INVALID | yes |
| mixing warns of: EFFECT_PARAMETER_INVALID | EFFECT_PARAMETER_INVALID | yes |
| 103 named places within 0.00002 | largest difference 1.11e-16 | yes |

## fx_dist_018: a type of "crunch", which it does not have: kept, EFFECT_PARAMETER_INVALID, the sound unchanged

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: EFFECT_PARAMETER_INVALID | EFFECT_PARAMETER_INVALID | yes |
| mixing warns of: EFFECT_PARAMETER_INVALID | EFFECT_PARAMETER_INVALID | yes |
| 103 named places within 0.00002 | largest difference 1.11e-16 | yes |

## The harmonic table: a 1 kHz sine at -6 dB through each type at the starting values, within 0.01 dB

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dist_001: 2nd none, 3rd -17.74 dB | none, -17.74 dB | yes |
| fx_dist_002: 2nd none, 3rd -15.51 dB | none, -15.51 dB | yes |
| fx_dist_003: 2nd none, 3rd -17.09 dB | none, -17.09 dB | yes |
| fx_dist_004: 2nd none, 3rd -16.35 dB | none, -16.35 dB | yes |
| fx_dist_005: 2nd -20.50 dB, 3rd -18.45 dB | -20.50 dB, -18.45 dB | yes |
| fx_dist_006: 2nd none, 3rd -12.57 dB | none, -12.57 dB | yes |

## The curve table: each type's output on the ramp at 21 samples

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dist_001: in -1.000, out -0.99642 | -0.99642 | yes |
| fx_dist_001: in -0.900, out -0.99328 | -0.99328 | yes |
| fx_dist_001: in -0.800, out -0.98739 | -0.98739 | yes |
| fx_dist_001: in -0.700, out -0.97639 | -0.97639 | yes |
| fx_dist_001: in -0.600, out -0.95602 | -0.95602 | yes |
| fx_dist_001: in -0.500, out -0.91879 | -0.91879 | yes |
| fx_dist_001: in -0.400, out -0.85244 | -0.85244 | yes |
| fx_dist_001: in -0.300, out -0.73923 | -0.73923 | yes |
| fx_dist_001: in -0.200, out -0.55980 | -0.55980 | yes |
| fx_dist_001: in -0.100, out -0.30616 | -0.30616 | yes |
| fx_dist_001: in -0.000, out -0.00007 | -0.00007 | yes |
| fx_dist_001: in +0.100, out +0.30604 | +0.30604 | yes |
| fx_dist_001: in +0.200, out +0.55970 | +0.55970 | yes |
| fx_dist_001: in +0.300, out +0.73917 | +0.73917 | yes |
| fx_dist_001: in +0.400, out +0.85240 | +0.85240 | yes |
| fx_dist_001: in +0.500, out +0.91877 | +0.91877 | yes |
| fx_dist_001: in +0.600, out +0.95601 | +0.95601 | yes |
| fx_dist_001: in +0.700, out +0.97639 | +0.97639 | yes |
| fx_dist_001: in +0.800, out +0.98738 | +0.98738 | yes |
| fx_dist_001: in +0.900, out +0.99328 | +0.99328 | yes |
| fx_dist_001: in +1.000, out +0.99642 | +0.99642 | yes |
| fx_dist_002: in -1.000, out -1.00000 | -1.00000 | yes |
| fx_dist_002: in -0.900, out -1.00000 | -1.00000 | yes |
| fx_dist_002: in -0.800, out -1.00000 | -1.00000 | yes |
| fx_dist_002: in -0.700, out -1.00000 | -1.00000 | yes |
| fx_dist_002: in -0.600, out -1.00000 | -1.00000 | yes |
| fx_dist_002: in -0.500, out -1.00000 | -1.00000 | yes |
| fx_dist_002: in -0.400, out -1.00000 | -1.00000 | yes |
| fx_dist_002: in -0.300, out -0.94877 | -0.94877 | yes |
| fx_dist_002: in -0.200, out -0.63253 | -0.63253 | yes |
| fx_dist_002: in -0.100, out -0.31630 | -0.31630 | yes |
| fx_dist_002: in -0.000, out -0.00007 | -0.00007 | yes |
| fx_dist_002: in +0.100, out +0.31617 | +0.31617 | yes |
| fx_dist_002: in +0.200, out +0.63240 | +0.63240 | yes |
| fx_dist_002: in +0.300, out +0.94864 | +0.94864 | yes |
| fx_dist_002: in +0.400, out +1.00000 | +1.00000 | yes |
| fx_dist_002: in +0.500, out +1.00000 | +1.00000 | yes |
| fx_dist_002: in +0.600, out +1.00000 | +1.00000 | yes |
| fx_dist_002: in +0.700, out +1.00000 | +1.00000 | yes |
| fx_dist_002: in +0.800, out +1.00000 | +1.00000 | yes |
| fx_dist_002: in +0.900, out +1.00000 | +1.00000 | yes |
| fx_dist_002: in +1.000, out +1.00000 | +1.00000 | yes |
| fx_dist_003: in -1.000, out -0.75975 | -0.75975 | yes |
| fx_dist_003: in -0.900, out -0.74000 | -0.74000 | yes |
| fx_dist_003: in -0.800, out -0.71671 | -0.71671 | yes |
| fx_dist_003: in -0.700, out -0.68883 | -0.68883 | yes |
| fx_dist_003: in -0.600, out -0.65487 | -0.65487 | yes |
| fx_dist_003: in -0.500, out -0.61259 | -0.61259 | yes |
| fx_dist_003: in -0.400, out -0.55850 | -0.55850 | yes |
| fx_dist_003: in -0.300, out -0.48686 | -0.48686 | yes |
| fx_dist_003: in -0.200, out -0.38746 | -0.38746 | yes |
| fx_dist_003: in -0.100, out -0.24029 | -0.24029 | yes |
| fx_dist_003: in -0.000, out -0.00007 | -0.00007 | yes |
| fx_dist_003: in +0.100, out +0.24022 | +0.24022 | yes |
| fx_dist_003: in +0.200, out +0.38741 | +0.38741 | yes |
| fx_dist_003: in +0.300, out +0.48682 | +0.48682 | yes |
| fx_dist_003: in +0.400, out +0.55847 | +0.55847 | yes |
| fx_dist_003: in +0.500, out +0.61257 | +0.61257 | yes |
| fx_dist_003: in +0.600, out +0.65486 | +0.65486 | yes |
| fx_dist_003: in +0.700, out +0.68882 | +0.68882 | yes |
| fx_dist_003: in +0.800, out +0.71670 | +0.71670 | yes |
| fx_dist_003: in +0.900, out +0.73999 | +0.73999 | yes |
| fx_dist_003: in +1.000, out +0.75975 | +0.75975 | yes |
| fx_dist_004: in -1.000, out -0.87353 | -0.87353 | yes |
| fx_dist_004: in -0.900, out -0.85991 | -0.85991 | yes |
| fx_dist_004: in -0.800, out -0.84306 | -0.84306 | yes |
| fx_dist_004: in -0.700, out -0.82173 | -0.82173 | yes |
| fx_dist_004: in -0.600, out -0.79392 | -0.79392 | yes |
| fx_dist_004: in -0.500, out -0.75633 | -0.75633 | yes |
| fx_dist_004: in -0.400, out -0.70318 | -0.70318 | yes |
| fx_dist_004: in -0.300, out -0.62376 | -0.62376 | yes |
| fx_dist_004: in -0.200, out -0.49795 | -0.49795 | yes |
| fx_dist_004: in -0.100, out -0.29356 | -0.29356 | yes |
| fx_dist_004: in -0.000, out -0.00007 | -0.00007 | yes |
| fx_dist_004: in +0.100, out +0.29345 | +0.29345 | yes |
| fx_dist_004: in +0.200, out +0.49788 | +0.49788 | yes |
| fx_dist_004: in +0.300, out +0.62372 | +0.62372 | yes |
| fx_dist_004: in +0.400, out +0.70315 | +0.70315 | yes |
| fx_dist_004: in +0.500, out +0.75631 | +0.75631 | yes |
| fx_dist_004: in +0.600, out +0.79391 | +0.79391 | yes |
| fx_dist_004: in +0.700, out +0.82172 | +0.82172 | yes |
| fx_dist_004: in +0.800, out +0.84306 | +0.84306 | yes |
| fx_dist_004: in +0.900, out +0.85990 | +0.85990 | yes |
| fx_dist_004: in +1.000, out +0.87353 | +0.87353 | yes |
| fx_dist_005: in -1.000, out -1.31809 | -1.31809 | yes |
| fx_dist_005: in -0.900, out -1.31260 | -1.31260 | yes |
| fx_dist_005: in -0.800, out -1.30233 | -1.30233 | yes |
| fx_dist_005: in -0.700, out -1.28327 | -1.28327 | yes |
| fx_dist_005: in -0.600, out -1.24831 | -1.24831 | yes |
| fx_dist_005: in -0.500, out -1.18559 | -1.18559 | yes |
| fx_dist_005: in -0.400, out -1.07737 | -1.07737 | yes |
| fx_dist_005: in -0.300, out -0.90265 | -0.90265 | yes |
| fx_dist_005: in -0.200, out -0.64874 | -0.64874 | yes |
| fx_dist_005: in -0.100, out -0.33098 | -0.33098 | yes |
| fx_dist_005: in -0.000, out -0.00007 | -0.00007 | yes |
| fx_dist_005: in +0.100, out +0.28470 | +0.28470 | yes |
| fx_dist_005: in +0.200, out +0.49223 | +0.49223 | yes |
| fx_dist_005: in +0.300, out +0.62586 | +0.62586 | yes |
| fx_dist_005: in +0.400, out +0.70518 | +0.70518 | yes |
| fx_dist_005: in +0.500, out +0.75000 | +0.75000 | yes |
| fx_dist_005: in +0.600, out +0.77463 | +0.77463 | yes |
| fx_dist_005: in +0.700, out +0.78796 | +0.78796 | yes |
| fx_dist_005: in +0.800, out +0.79510 | +0.79510 | yes |
| fx_dist_005: in +0.900, out +0.79892 | +0.79892 | yes |
| fx_dist_005: in +1.000, out +0.80096 | +0.80096 | yes |
| fx_dist_006: in -1.000, out -1.00000 | -1.00000 | yes |
| fx_dist_006: in -0.900, out -1.00000 | -1.00000 | yes |
| fx_dist_006: in -0.800, out -1.00000 | -1.00000 | yes |
| fx_dist_006: in -0.700, out -1.00000 | -1.00000 | yes |
| fx_dist_006: in -0.600, out -1.00000 | -1.00000 | yes |
| fx_dist_006: in -0.500, out -1.00000 | -1.00000 | yes |
| fx_dist_006: in -0.400, out -1.00000 | -1.00000 | yes |
| fx_dist_006: in -0.300, out -0.97405 | -0.97405 | yes |
| fx_dist_006: in -0.200, out -0.79532 | -0.79532 | yes |
| fx_dist_006: in -0.100, out -0.56241 | -0.56241 | yes |
| fx_dist_006: in -0.000, out -0.00812 | -0.00812 | yes |
| fx_dist_006: in +0.100, out +0.56229 | +0.56229 | yes |
| fx_dist_006: in +0.200, out +0.79524 | +0.79524 | yes |
| fx_dist_006: in +0.300, out +0.97398 | +0.97398 | yes |
| fx_dist_006: in +0.400, out +1.00000 | +1.00000 | yes |
| fx_dist_006: in +0.500, out +1.00000 | +1.00000 | yes |
| fx_dist_006: in +0.600, out +1.00000 | +1.00000 | yes |
| fx_dist_006: in +0.700, out +1.00000 | +1.00000 | yes |
| fx_dist_006: in +0.800, out +1.00000 | +1.00000 | yes |
| fx_dist_006: in +0.900, out +1.00000 | +1.00000 | yes |
| fx_dist_006: in +1.000, out +1.00000 | +1.00000 | yes |

## The level table: each channel's loudest sample, within 0.01 dB

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dist_001: left -0.03 dB, right -0.03 dB | -0.03 dB, -0.03 dB | yes |
| fx_dist_002: left 0.00 dB, right 0.00 dB | 0.00 dB, 0.00 dB | yes |
| fx_dist_003: left -2.39 dB, right -2.39 dB | -2.39 dB, -2.39 dB | yes |
| fx_dist_004: left -1.17 dB, right -1.17 dB | -1.17 dB, -1.17 dB | yes |
| fx_dist_005: left 2.40 dB, right 2.40 dB | 2.40 dB, 2.40 dB | yes |
| fx_dist_006: left 0.00 dB, right 0.00 dB | 0.00 dB, 0.00 dB | yes |
| fx_dist_007: left 0.00 dB, right 0.00 dB | 0.00 dB, 0.00 dB | yes |
| fx_dist_008: left 1.91 dB, right 1.91 dB | 1.91 dB, 1.91 dB | yes |
| fx_dist_009: left 0.00 dB, right 0.00 dB | 0.00 dB, 0.00 dB | yes |
| fx_dist_010: left 0.00 dB, right 0.00 dB | 0.00 dB, 0.00 dB | yes |
| fx_dist_011: left 0.00 dB, right 0.00 dB | 0.00 dB, 0.00 dB | yes |
| fx_dist_012: left -0.01 dB, right -0.01 dB | -0.01 dB, -0.01 dB | yes |
| fx_dist_013: left 0.00 dB, right 0.00 dB | 0.00 dB, 0.00 dB | yes |
| fx_dist_014: left -6.00 dB, right -6.00 dB | -6.00 dB, -6.00 dB | yes |
| fx_dist_015: left -0.03 dB, right -0.03 dB | -0.03 dB, -0.03 dB | yes |
| fx_dist_016: left 0.00 dB, right 0.00 dB | 0.00 dB, 0.00 dB | yes |
| fx_dist_017: left 0.00 dB, right 0.00 dB | 0.00 dB, 0.00 dB | yes |
| fx_dist_018: left 0.00 dB, right 0.00 dB | 0.00 dB, 0.00 dB | yes |

## Refused projects

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dist_101.json: Distortion on a solid layer is PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID | yes |
| fx_dist_102.json: an expression on a Distortion number is PROJECT_SCHEMA_INVALID: sound effects take keys only | PROJECT_SCHEMA_INVALID | yes |
| fx_dist_103.json: a picture Mix on Distortion is PROJECT_SCHEMA_INVALID: its wet share is its own Mix setting | PROJECT_SCHEMA_INVALID | yes |

## Saved again

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dist_008.json: the layer's effects as written | as written | yes |
| fx_dist_012.json: the layer's effects as written | as written | yes |
| fx_dist_017.json: the layer's effects as written | as written | yes |
| fx_dist_018.json: the layer's effects as written | as written | yes |

## Decision 3: sound effects on sound layers only, through the commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Distortion added to a sound layer | added | yes |
| Distortion added to a solid | refused: "solid 0" is not an audio layer, so it takes no Distortion. | yes |

## Speed (release build, this machine)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dist_001, two seconds mixed: without the distortion, with it (milliseconds, recorded only) | 0.9 ms, 5.7 ms | yes |

## Result

240 of 240 checks pass.
