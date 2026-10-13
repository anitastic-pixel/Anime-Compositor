# D-640: mixing a composition's sound (B-519)

Each project is mixed by the core for the frames its case names and held to the numbers `tools/sound_mix_reference.py` wrote before the code, by document 21's rule: each sample belongs to its frame (ADR-018), an audio layer that is on reads its working sound from its local frame's first sample, times its level; the layers are added and nothing is clipped. The window plays this mix. Tolerance 2e-5. Pictures: `verification/D-640 pictures/` (left channel above, right below, a grey line at each frame's first sample, drawn to the loudest sample).

## fx_smix_001: a 1 kHz sine at -6 dB through a level of -6 dB peaks at -12.00 dB

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 24: 48000 samples | 48000 and 48000 | yes |
| left out: none | none | yes |
| not silent | not silent | yes |
| 55 named places within 0.00002 | largest difference 2.78e-17 | yes |

## fx_smix_002: the same layer switched off: every sample 0

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 24: 48000 samples | 48000 and 48000 | yes |
| left out: none | none | yes |
| every sample 0 | every sample 0 | yes |
| 50 named places within 0.00002 | largest difference 0.00e0 | yes |

## fx_smix_003: two layers add: the sine and the clicks, each at its level

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 36: 72000 samples | 72000 and 72000 | yes |
| left out: none | none | yes |
| not silent | not silent | yes |
| 84 named places within 0.00002 | largest difference 2.78e-17 | yes |

## fx_smix_004: a layer starting at frame 5: silence to sample 10000, the file from there

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 30: 60000 samples | 60000 and 60000 | yes |
| left out: none | none | yes |
| not silent | not silent | yes |
| 67 named places within 0.00002 | largest difference 1.11e-16 | yes |

## fx_smix_005: a layer starting at frame 2 with its first 6 frames cut: sample 4000 is the file's 12000

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 24: 48000 samples | 48000 and 48000 | yes |
| left out: none | none | yes |
| not silent | not silent | yes |
| 60 named places within 0.00002 | largest difference 5.55e-17 | yes |

## fx_smix_006: a layer ending at frame 10: silence from sample 20000

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 24: 48000 samples | 48000 and 48000 | yes |
| left out: none | none | yes |
| not silent | not silent | yes |
| 55 named places within 0.00002 | largest difference 2.78e-17 | yes |

## fx_smix_007: a layer whose sound starts 3 frames late (offset -3): silence to sample 6000

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 24: 48000 samples | 48000 and 48000 | yes |
| left out: none | none | yes |
| not silent | not silent | yes |
| 55 named places within 0.00002 | largest difference 5.55e-17 | yes |

## fx_smix_008: 29.97 frames a second, a layer starting at frame 1: each frame read from its own first sample

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 20: 32032 samples | 32032 and 32032 | yes |
| left out: none | none | yes |
| not silent | not silent | yes |
| 54 named places within 0.00002 | largest difference 2.78e-17 | yes |

## fx_smix_009: a level of +12 dB: the sine 3.98 times louder, past 1, not clipped

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 12: 24000 samples | 24000 and 24000 | yes |
| left out: none | none | yes |
| not silent | not silent | yes |
| 31 named places within 0.00002 | largest difference 2.22e-16 | yes |

## fx_smix_010: a level of -96 dB: the sine 63,000 times quieter

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 12: 24000 samples | 24000 and 24000 | yes |
| left out: none | none | yes |
| not silent | not silent | yes |
| 31 named places within 0.00002 | largest difference 8.47e-22 | yes |

## fx_smix_011: an Opus layer is left out with its warning; the sine beside it is heard

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 12: 24000 samples | 24000 and 24000 | yes |
| left out: MEDIA_AUDIO_PREVIEW_ONLY | sound0 MEDIA_AUDIO_PREVIEW_ONLY | yes |
| not silent | not silent | yes |
| 31 named places within 0.00002 | largest difference 2.78e-17 | yes |

## fx_smix_012: a layer whose file is gone is left out with its warning; the clicks beside it are heard

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 24: 48000 samples | 48000 and 48000 | yes |
| left out: MEDIA_AUDIO_UNREADABLE | sound0 MEDIA_AUDIO_UNREADABLE | yes |
| not silent | not silent | yes |
| 55 named places within 0.00002 | largest difference 0.00e0 | yes |

## fx_smix_013: a 44,100 mono file: its working sound, in both channels

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 0 to 14: 28000 samples | 28000 and 28000 | yes |
| left out: none | none | yes |
| not silent | not silent | yes |
| 35 named places within 0.00002 | largest difference 2.22e-16 | yes |

## fx_smix_014: frames 10 to 14 of fx_smix_003 alone: the same samples as the whole mix there

| Check | The build's answer | Matches |
| --- | --- | --- |
| frames 10 to 14: 8000 samples | 8000 and 8000 | yes |
| left out: none | none | yes |
| not silent | not silent | yes |
| 15 named places within 0.00002 | largest difference 2.78e-17 | yes |

## A stretch alone is the whole mix there

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_smix_014 (frames 10 to 14) against fx_smix_003 at the same samples | every sample the same | yes |
| mixing again (the working sound kept) gives the same samples | the same | yes |

## The level table: each channel's loudest sample, within 0.01 dB

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_smix_001: left -12.00 dB, right -12.00 dB | -12.00 dB, -12.00 dB | yes |
| fx_smix_002: left silent, right silent | silent, silent | yes |
| fx_smix_003: left 0.60 dB, right -6.00 dB | 0.60 dB, -6.00 dB | yes |
| fx_smix_004: left -0.00 dB, right -0.00 dB | -0.00 dB, -0.00 dB | yes |
| fx_smix_005: left -0.00 dB, right -0.00 dB | -0.00 dB, -0.00 dB | yes |
| fx_smix_006: left -7.60 dB, right -7.60 dB | -7.60 dB, -7.60 dB | yes |
| fx_smix_007: left -1.16 dB, right -1.16 dB | -1.16 dB, -1.16 dB | yes |
| fx_smix_008: left -3.96 dB, right -3.96 dB | -3.96 dB, -3.96 dB | yes |
| fx_smix_009: left 6.00 dB, right 6.00 dB | 6.00 dB, 6.00 dB | yes |
| fx_smix_010: left -102.00 dB, right -102.00 dB | -102.00 dB, -102.00 dB | yes |
| fx_smix_011: left -6.00 dB, right -6.00 dB | -6.00 dB, -6.00 dB | yes |
| fx_smix_012: left -0.92 dB, right -6.02 dB | -0.92 dB, -6.02 dB | yes |
| fx_smix_013: left -7.96 dB, right -7.96 dB | -7.96 dB, -7.96 dB | yes |
| fx_smix_014: left -3.92 dB, right -6.00 dB | -3.92 dB, -6.00 dB | yes |

## Result

72 of 72 checks pass.
