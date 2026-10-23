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
| FX-LBLEND-009 | color_dodge (refused) | - | reason: layers/4/blend_mode: expected one of normal, darken, multiply, add, lighten, screen, overlay, soft_light, hard_light, difference, hue, saturation, color, luminosity, stencil_alpha, stencil_luma. | - | yes |

B-627: 9 of 9 fixtures pass.
