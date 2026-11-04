# D-1065: Mercury after After Effects

B-945. Mercury's new `rule` word: `after_effects` (as added) reads CC Mr. Mercury's numbers and every drop lives, showing the layer under it; a file without the word keeps D-674's rule. FX-MERCURY-001 to 038 are unchanged and pass in B-553 (`verification/D-674_mercury_table.md`).

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mercury_023.json, written before the rule, is saved as it was: 28 settings and no rule | 28 settings; rule null | yes |
| a Mercury as added is saved with its rule, after_effects, and CC Mr. Mercury's numbers | rule "after_effects", birth rate 1, radius 5 by 5, velocity 1, gravity 1, birth size 0.15, death size 0.75 | yes |
| a Mercury whose rule is "spin" is left out with a warning | EFFECT_PARAMETER_INVALID Layer art's core.mercury has a setting this build cannot use, so it is not drawn. | yes |

## A producer of radius 0 over a clear part of the layer (G4)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before_radius_0.png, frame 24, the middle wiped away: no drops, as D-674 drew it | 0 pixels differ from no drops; warnings [] | yes |
| 2_after_effects_radius_0.png, frame 24, the middle wiped away: drops drawn | 375 pixels differ from no drops; warnings [] | yes |
| 3_after_effects_line.png, frame 24, the middle wiped away: drops drawn | 429 pixels differ from no drops; warnings [] | yes |

## The animations (frame 24, After Effects rule)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 4_explosive.png: drawn without a warning, unlike the street and every animation before it | 129598 pixels changed; warnings []; the same as none | yes |
| 4_fractal_explosive.png: drawn without a warning, unlike the street and every animation before it | 129598 pixels changed; warnings []; the same as none | yes |
| 4_twirl.png: drawn without a warning, unlike the street and every animation before it | 129597 pixels changed; warnings []; the same as none | yes |
| 4_twirly.png: drawn without a warning, unlike the street and every animation before it | 129598 pixels changed; warnings []; the same as none | yes |
| 4_vortex.png: drawn without a warning, unlike the street and every animation before it | 129598 pixels changed; warnings []; the same as none | yes |
| 4_fire.png: drawn without a warning, unlike the street and every animation before it | 129600 pixels changed; warnings []; the same as none | yes |
| 4_direction.png: drawn without a warning, unlike the street and every animation before it | 129596 pixels changed; warnings []; the same as none | yes |
| 4_direction_normalized.png: drawn without a warning, unlike the street and every animation before it | 129600 pixels changed; warnings []; the same as none | yes |
| 4_bi_directional.png: drawn without a warning, unlike the street and every animation before it | 129599 pixels changed; warnings []; the same as none | yes |
| 4_bi_directional_normalized.png: drawn without a warning, unlike the street and every animation before it | 129594 pixels changed; warnings []; the same as none | yes |
| 4_jet.png: drawn without a warning, unlike the street and every animation before it | 129599 pixels changed; warnings []; the same as none | yes |
| 4_jet_sideways.png: drawn without a warning, unlike the street and every animation before it | 129598 pixels changed; warnings []; the same as none | yes |
| 4_fixed.png: drawn without a warning, unlike the street and every animation before it | 129600 pixels changed; warnings []; the same as none | yes |

## On the card

| Check | The build's answer | Matches |
| --- | --- | --- |
| the reference shot, Mercury as added on three layers, frame 100, Full | largest difference 1 of 255, 121 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mercury as added on three layers, frame 239, Full | largest difference 1 of 255, 186 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mercury fire on three layers, frame 100, Full | largest difference 1 of 255, 34 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mercury fire on three layers, frame 239, Full | largest difference 1 of 255, 300 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mercury jet sideways, a line producer on three layers, frame 100, Full | largest difference 1 of 255, 169 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mercury jet sideways, a line producer on three layers, frame 239, Full | largest difference 1 of 255, 91 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mercury the D-674 rule on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Mercury the D-674 rule on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Result

27 of 27 checks pass.
