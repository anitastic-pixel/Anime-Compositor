# B-627: a layer's eight new blend modes against the reference

Written by `tests/b627_layer_blend_modes.rs`. Each case is `Fixtures/layer_blend_modes/`'s, worked by `tools/layer_blend_modes_reference.py`: a 10 by 8 frame, four backdrop strips under four strips in the mode, the last at 50%. Largest difference over every channel of every pixel, against the tolerance.

| Fixture | Mode | Pixels | Largest difference | Tolerance | Pass |
| --- | --- | --- | --- | --- | --- |
| FX-LBLEND-001 | hard_light | 80 | 5.28e-8 | 2e-5 | yes |
| FX-LBLEND-002 | darken | 80 | 2.33e-8 | 2e-5 | yes |
| FX-LBLEND-003 | lighten | 80 | 2.33e-8 | 2e-5 | yes |
| FX-LBLEND-004 | difference | 80 | 2.69e-8 | 2e-5 | yes |
| FX-LBLEND-005 | hue | 80 | 2.90e-8 | 2e-5 | yes |
| FX-LBLEND-006 | saturation | 80 | 2.51e-8 | 2e-5 | yes |
| FX-LBLEND-007 | color | 80 | 2.57e-8 | 2e-5 | yes |
| FX-LBLEND-008 | luminosity | 80 | 3.80e-8 | 2e-5 | yes |
| FX-LBLEND-010 | color_burn | 80 | 2.33e-8 | 2e-5 | yes |
| FX-LBLEND-011 | classic_color_burn | 80 | 2.33e-8 | 2e-5 | yes |
| FX-LBLEND-012 | linear_burn | 80 | 2.33e-8 | 2e-5 | yes |
| FX-LBLEND-013 | darker_color | 80 | 2.33e-8 | 2e-5 | yes |
| FX-LBLEND-014 | color_dodge | 80 | 8.71e-8 | 2e-5 | yes |
| FX-LBLEND-015 | classic_color_dodge | 80 | 8.71e-8 | 2e-5 | yes |
| FX-LBLEND-016 | lighter_color | 80 | 2.33e-8 | 2e-5 | yes |
| FX-LBLEND-017 | linear_light | 80 | 6.23e-8 | 2e-5 | yes |
| FX-LBLEND-018 | vivid_light | 80 | 1.44e-7 | 2e-5 | yes |
| FX-LBLEND-019 | pin_light | 80 | 3.81e-8 | 2e-5 | yes |
| FX-LBLEND-020 | hard_mix | 80 | 2.84e-8 | 2e-5 | yes |
| FX-LBLEND-021 | exclusion | 80 | 2.33e-8 | 2e-5 | yes |
| FX-LBLEND-022 | subtract | 80 | 3.18e-8 | 2e-5 | yes |
| FX-LBLEND-023 | divide | 80 | 3.14e-8 | 2e-5 | yes |
| FX-LBLEND-009 | dissolve (refused) | - | reason: layers/4/blend_mode: expected one of normal, darken, multiply, color_burn, classic_color_burn, linear_burn, darker_color, add, lighten, screen, color_dodge, classic_color_dodge, lighter_color, overlay, soft_light, hard_light, linear_light, vivid_light, pin_light, hard_mix, difference, exclusion, subtract, divide, hue, saturation, color, luminosity, stencil_alpha, stencil_luma. | - | yes |

B-627: 23 of 23 fixtures pass.
