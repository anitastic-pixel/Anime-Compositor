# D-1066: Wave Warp after After Effects

B-946. Wave Warp's new `rule` word: `after_effects` (as added) reads Width as half a wave and Speed as waves a second, and draws Noise and Smooth Noise as jagged streaks along the wave; a file without the word keeps D-149's rule. FX-WAVE-001 to 047 are unchanged and pass in B-92, B-624 and B-634.

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_wave_001.json, written before the rule, is saved as it was: no rule | 7 settings; rule null | yes |
| a Wave Warp as added is saved with its rule, after_effects | rule "after_effects", width 40, speed 1 | yes |
| a Wave Warp whose rule is "spin" is left out with a warning | EFFECT_PARAMETER_INVALID Layer art's core.wave_warp has a setting this build cannot use, so it is not drawn. | yes |

## The units: After Effects' Width 40 and Speed 1 at 24 frames a second, frame 6

| Check | The build's answer | Matches |
| --- | --- | --- |
| sine: the same as D-149's rule at Width 80 and Speed 15 degrees a frame | largest difference 0 of 255, 0 pixels differ; warnings [] [] | yes |
| triangle: the same as D-149's rule at Width 80 and Speed 15 degrees a frame | largest difference 0 of 255, 0 pixels differ; warnings [] [] | yes |
| circle: the same as D-149's rule at Width 80 and Speed 15 degrees a frame | largest difference 0 of 255, 0 pixels differ; warnings [] [] | yes |
| square: the same as D-149's rule at Width 80 and Speed 15 degrees a frame | largest difference 0 of 255, 0 pixels differ; warnings [] [] | yes |
| sawtooth: the same as D-149's rule at Width 80 and Speed 15 degrees a frame | largest difference 0 of 255, 0 pixels differ; warnings [] [] | yes |
| semicircle: the same as D-149's rule at Width 80 and Speed 15 degrees a frame | largest difference 0 of 255, 0 pixels differ; warnings [] [] | yes |
| Speed 1: frame 24 (one second on) is frame 0 again, the wave moved one whole wave (within 1 level, the rounding of a whole turn) | largest difference 0 of 255, 0 pixels differ | yes |

## Noise and Smooth Noise (the tutorial's Height 500 over Width 526, scaled to Height 20 over 21)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 2_after_effects_noise.png: drawn, and not D-149's noise (1_before_noise.png) | 76524 pixels moved from the street, 84884 pixels differ from D-149's; warnings [] | yes |
| 2_after_effects_smooth_noise.png: drawn, and not D-149's smooth_noise (1_before_smooth_noise.png) | 75227 pixels moved from the street, 84317 pixels differ from D-149's; warnings [] | yes |

## On the card

| Check | The build's answer | Matches |
| --- | --- | --- |
| the reference shot, Wave Warp as added on three layers, frame 100, Full | largest difference 1 of 255, 756 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Wave Warp as added on three layers, frame 239, Full | largest difference 1 of 255, 952 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Wave Warp noise on three layers, frame 100, Full | largest difference 1 of 255, 960 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Wave Warp noise on three layers, frame 239, Full | largest difference 1 of 255, 839 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Wave Warp smooth noise on three layers, frame 100, Full | largest difference 1 of 255, 909 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Wave Warp smooth noise on three layers, frame 239, Full | largest difference 1 of 255, 776 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Wave Warp the D-149 rule, smooth noise on three layers, frame 100, Full | largest difference 1 of 255, 941 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Wave Warp the D-149 rule, smooth noise on three layers, frame 239, Full | largest difference 1 of 255, 902 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Result

20 of 20 checks pass.
