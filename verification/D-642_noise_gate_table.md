# D-642: Noise Gate (B-521)

Each project is mixed by the core and held to the numbers `tools/noise_gate_reference.py` wrote before the code, by document 21's rule: a gain from 0 (shut) at the layer's first sample rises by 1/attack while the louder channel is at or above the threshold or the hold is counting down, and falls by 1/release after; the times are whole samples at 48,000 a second; a keyed setting is read straight between its values on this frame and the next. The gate works before the layer's level; gates run in order; one switched off or outside its ranges lets the sound through. Tolerance 2e-5. The timing is read off the mix: the last gate's gain is what the mix keeps of what reached that gate (shut where nothing reached it). Pictures: `verification/D-642 pictures/` (left channel above, right below, full level at the edges), `_before` with the gates switched off.

## fx_gate_001: the starting values on a burst at sample 12000: opens over 480 samples, holds 1920, shuts over 10560

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 122 named places within 0.00002 | largest difference 5.55e-17 | yes |
| timing: rise 12000, full 12479, last_open 23999, closing 25920, shut 36479 | rise 12000, full 12479, last_open 23999, closing 25920, shut 36479 | yes |

## fx_gate_002: Attack 0: fully open on the burst's first sample

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 107 named places within 0.00002 | largest difference 5.55e-17 | yes |
| timing: rise 12000, full 12000, last_open 23999, closing 25920, shut 36479 | rise 12000, full 12000, last_open 23999, closing 25920, shut 36479 | yes |

## fx_gate_003: Hold 500 ms: open 24000 samples after the burst ends

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 113 named places within 0.00002 | largest difference 5.55e-17 | yes |
| timing: rise 12000, full 12479, last_open 23999, closing 48000, shut 58559 | rise 12000, full 12479, last_open 23999, closing 48000, shut 58559 | yes |

## fx_gate_004: Hold 0 and Release 5 ms: it dips at the burst's quiet zero crossings, and is shut 240 samples after the burst

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 108 named places within 0.00002 | largest difference 5.55e-17 | yes |
| timing: rise 12000, full 12548, last_open 12563, closing 12564, shut 24239 | rise 12000, full 12548, last_open 12563, closing 12564, shut 24239 | yes |

## fx_gate_005: Threshold -20 dB: the quiet burst (-26 dB) stays shut, the loud one (-6 dB) opens it

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 113 named places within 0.00002 | largest difference 5.55e-17 | yes |
| timing: rise 48000, full 48479, last_open 59999, closing 61920, shut 72479 | rise 48000, full 48479, last_open 59999, closing 61920, shut 72479 | yes |

## fx_gate_006: Threshold -60 dB: both bursts open it

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 108 named places within 0.00002 | largest difference 5.55e-17 | yes |
| timing: rise 12000, full 12479, last_open 23999, closing 25920, shut 36479 | rise 12000, full 12479, last_open 23999, closing 25920, shut 36479 | yes |

## fx_gate_007: Threshold keyed from -40 dB (frame 0) to -20 dB (frame 10) over a steady -28 dB: it shuts part-way through frame 6

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 24: 48000 samples | 48000 and 48000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 62 named places within 0.00002 | largest difference 1.73e-18 | yes |
| timing: rise 0, full 479, last_open 12041, closing 13962, shut 24521 | rise 0, full 479, last_open 12041, closing 13962, shut 24521 | yes |

## fx_gate_008: the right channel's burst opens the gate for both: the quiet left is heard while it is open

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 107 named places within 0.00002 | largest difference 2.78e-17 | yes |
| timing: rise 12000, full 12479, last_open 23999, closing 25920, shut 36479 | rise 12000, full 12479, last_open 23999, closing 25920, shut 36479 | yes |

## fx_gate_009: frames 10 to 19 of fx_gate_001 alone: the same samples as the whole mix there

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 10 to 20: 20000 samples | 20000 and 20000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 32 named places within 0.00002 | largest difference 5.55e-17 | yes |
| timing: rise 12000, full 12479, last_open 23999, closing 25920, shut 36479 | rise 12000, full 12479, last_open 23999, closing 25920, shut 36479 | yes |

## fx_gate_010: a layer starting at frame 6 with a level of -6 dB: the gate counts from the layer's start, before the level

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 108 named places within 0.00002 | largest difference 2.78e-17 | yes |
| timing: rise 24000, full 24479, last_open 35999, closing 37920, shut 48479 | rise 24000, full 24479, last_open 35999, closing 37920, shut 48479 | yes |

## fx_gate_011: the gate switched off: the sound unchanged

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 103 named places within 0.00002 | largest difference 5.55e-17 | yes |
| timing: no gate runs | no gate runs | yes |

## fx_gate_012: Attack keyed from 0 ms (frame 0) to 100 ms (frame 10) at 29.97: the ramp lengthens sample by sample

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 76876 samples | 76876 and 76876 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 89 named places within 0.00002 | largest difference 5.55e-17 | yes |
| timing: rise 12000, full 16192, last_open 23999, closing 25920, shut 36479 | rise 12000, full 16192, last_open 23999, closing 25920, shut 36479 | yes |

## fx_gate_013: two gates in a row: the second opens on what the first lets through

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 48: 96000 samples | 96000 and 96000 | yes |
| opening warns of: none | none | yes |
| mixing warns of: none | none | yes |
| 108 named places within 0.00002 | largest difference 5.55e-17 | yes |
| timing: rise 48096, full 48096, last_open 59999, closing 61920, shut 72479 | rise 48096, full 48096, last_open 59999, closing 61920, shut 72479 | yes |

## fx_gate_014: a Threshold of +3 dB, outside -60 to 0: kept, EFFECT_PARAMETER_INVALID, the sound unchanged

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 24: 48000 samples | 48000 and 48000 | yes |
| opening warns of: EFFECT_PARAMETER_INVALID | EFFECT_PARAMETER_INVALID | yes |
| mixing warns of: EFFECT_PARAMETER_INVALID | EFFECT_PARAMETER_INVALID | yes |
| 55 named places within 0.00002 | largest difference 5.55e-17 | yes |
| timing: no gate runs | no gate runs | yes |

## The level table: each channel's loudest sample, within 0.01 dB

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gate_001: left -5.99 dB, right -5.99 dB | -5.99 dB, -5.99 dB | yes |
| fx_gate_002: left -5.99 dB, right -5.99 dB | -5.99 dB, -5.99 dB | yes |
| fx_gate_003: left -5.99 dB, right -5.99 dB | -5.99 dB, -5.99 dB | yes |
| fx_gate_004: left -5.99 dB, right -5.99 dB | -5.99 dB, -5.99 dB | yes |
| fx_gate_005: left -5.99 dB, right -5.99 dB | -5.99 dB, -5.99 dB | yes |
| fx_gate_006: left -5.99 dB, right -5.99 dB | -5.99 dB, -5.99 dB | yes |
| fx_gate_007: left -27.96 dB, right -27.96 dB | -27.96 dB, -27.96 dB | yes |
| fx_gate_008: left -70.00 dB, right -6.00 dB | -70.00 dB, -6.00 dB | yes |
| fx_gate_009: left -5.99 dB, right -5.99 dB | -5.99 dB, -5.99 dB | yes |
| fx_gate_010: left -11.99 dB, right -11.99 dB | -11.99 dB, -11.99 dB | yes |
| fx_gate_011: left -5.99 dB, right -5.99 dB | -5.99 dB, -5.99 dB | yes |
| fx_gate_012: left -5.99 dB, right -5.99 dB | -5.99 dB, -5.99 dB | yes |
| fx_gate_013: left -5.99 dB, right -5.99 dB | -5.99 dB, -5.99 dB | yes |
| fx_gate_014: left -5.99 dB, right -5.99 dB | -5.99 dB, -5.99 dB | yes |

## Refused projects

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gate_101.json: a picture effect (Exposure) on an audio layer is PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID | yes |
| fx_gate_102.json: Noise Gate on a solid layer is PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID | yes |
| fx_gate_103.json: an expression on a Noise Gate number is PROJECT_SCHEMA_INVALID: sound effects take keys only | PROJECT_SCHEMA_INVALID | yes |
| fx_gate_104.json: a Mix on Noise Gate is PROJECT_SCHEMA_INVALID: a sound effect has no Mix | PROJECT_SCHEMA_INVALID | yes |

## fx_gate_201.json: an effect this build does not know on an audio layer: kept and saved as written, EFFECT_UNSUPPORTED, the sound unchanged

| Check | The build's answer | Matches |
| --- | --- | --- |
| opening warns of EFFECT_UNSUPPORTED | EFFECT_UNSUPPORTED | yes |
| mixing warns of EFFECT_UNSUPPORTED | EFFECT_UNSUPPORTED | yes |
| the sound unchanged at the named places, within 0.00002 | largest difference 5.55e-17 | yes |
| saved again, the layer's effects are as written | as written | yes |
| fx_gate_012 saved again: its keyed gate as written | as written | yes |

## Decision 3: sound effects on sound layers only, through the commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Exposure added to a sound layer | refused: "sound 0" is an audio layer, which draws nothing, so it takes no Exposure. | yes |
| Noise Gate added to a sound layer | added | yes |
| Noise Gate added to a solid | refused: "solid 0" is not an audio layer, so it takes no Noise Gate. | yes |

## Speed (release build, this machine)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gate_001, two seconds mixed: without the gate, with it (milliseconds, recorded only) | 1.5 ms, 3.4 ms | yes |

## Result

97 of 97 checks pass.
