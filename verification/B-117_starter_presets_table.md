# B-117: starter presets

Written by `tests/b117_starter_presets.rs` from `Fixtures/starter_presets/`, against D-181. Each line is one check: what `tools/starter_presets_reference.py` says the build must have, and what it has. The pictures are frame 100 of the reference shot at Draft, in `verification/B-117 pictures/`; `before.png` is the shot with no preset.

| Preset | Check | Should | The build | Matches |
| --- | --- | --- | --- | --- |
| all | the build carries the fixture file | the same text | the same text | yes |
| all | reads as an import would | read | read | yes |
| all | how many | 9 (six to ten) | 9 (six to ten) | yes |
| none | the shot draws cleanly | no diagnostic | no diagnostic | yes |
| Soft bloom | name, in this place | Soft bloom | Soft bloom | yes |
| Soft bloom | effects, in order | core.bloom | core.bloom | yes |
| Soft bloom | the sentence the panel shows | A gentle glow off the brightest parts of the picture. Best on an adjustment layer. | A gentle glow off the brightest parts of the picture. Best on an adjustment layer. | yes |
| Soft bloom | draws on an adjustment layer | no diagnostic | no diagnostic | yes |
| Soft bloom | changes the picture: `Soft bloom.png`, 118342 of 129600 pixels | yes | yes | yes |
| Night | name, in this place | Night | Night | yes |
| Night | effects, in order | core.color_balance, core.vibrance, core.brightness_contrast, core.vignette | core.color_balance, core.vibrance, core.brightness_contrast, core.vignette | yes |
| Night | the sentence the panel shows | Day painted as night: cooler, darker, the corners fall away. Best on an adjustment layer. | Day painted as night: cooler, darker, the corners fall away. Best on an adjustment layer. | yes |
| Night | draws on an adjustment layer | no diagnostic | no diagnostic | yes |
| Night | changes the picture: `Night.png`, 129600 of 129600 pixels | yes | yes | yes |
| Sunset | name, in this place | Sunset | Sunset | yes |
| Sunset | effects, in order | core.color_balance, core.gradient | core.color_balance, core.gradient | yes |
| Sunset | the sentence the panel shows | Warm evening light falling from the top of the frame. Best on an adjustment layer. | Warm evening light falling from the top of the frame. Best on an adjustment layer. | yes |
| Sunset | draws on an adjustment layer | no diagnostic | no diagnostic | yes |
| Sunset | changes the picture: `Sunset.png`, 129600 of 129600 pixels | yes | yes | yes |
| Cel shadow | name, in this place | Cel shadow | Cel shadow | yes |
| Cel shadow | effects, in order | core.drop_shadow | core.drop_shadow | yes |
| Cel shadow | the sentence the panel shows | A hard, flat shadow down and to the right, in a deep violet rather than black. For a character layer. | A hard, flat shadow down and to the right, in a deep violet rather than black. For a character layer. | yes |
| Cel shadow | draws on layer-3 | no diagnostic | no diagnostic | yes |
| Cel shadow | changes the picture: `Cel shadow.png`, 744 of 129600 pixels | yes | yes | yes |
| Rim light | name, in this place | Rim light | Rim light | yes |
| Rim light | effects, in order | core.rim_light | core.rim_light | yes |
| Rim light | the sentence the panel shows | A warm light catching the upper right edge. For a character layer. | A warm light catching the upper right edge. For a character layer. | yes |
| Rim light | draws on layer-3 | no diagnostic | no diagnostic | yes |
| Rim light | changes the picture: `Rim light.png`, 372 of 129600 pixels | yes | yes | yes |
| Old film | name, in this place | Old film | Old film | yes |
| Old film | effects, in order | core.gradient_map, core.noise, core.vignette, core.exposure_flicker | core.gradient_map, core.noise, core.vignette, core.exposure_flicker | yes |
| Old film | the sentence the panel shows | Sepia, grain, dark corners and a slight flicker. Best on an adjustment layer. | Sepia, grain, dark corners and a slight flicker. Best on an adjustment layer. | yes |
| Old film | draws on an adjustment layer | no diagnostic | no diagnostic | yes |
| Old film | changes the picture: `Old film.png`, 129600 of 129600 pixels | yes | yes | yes |
| Impact | name, in this place | Impact | Impact | yes |
| Impact | effects, in order | core.cross_glare, core.chromatic_aberration, core.camera_shake | core.cross_glare, core.chromatic_aberration, core.camera_shake | yes |
| Impact | the sentence the panel shows | The frame of a hit: glints on the brights, colour fringes and a shake. Best on an adjustment layer, for a few frames. | The frame of a hit: glints on the brights, colour fringes and a shake. Best on an adjustment layer, for a few frames. | yes |
| Impact | draws on an adjustment layer | no diagnostic | no diagnostic | yes |
| Impact | changes the picture: `Impact.png`, 121242 of 129600 pixels | yes | yes | yes |
| Dream haze | name, in this place | Dream haze | Dream haze | yes |
| Dream haze | effects, in order | core.diffusion, core.vibrance | core.diffusion, core.vibrance | yes |
| Dream haze | the sentence the panel shows | A soft, bright haze with the colour lifted. Best on an adjustment layer. | A soft, bright haze with the colour lifted. Best on an adjustment layer. | yes |
| Dream haze | draws on an adjustment layer | no diagnostic | no diagnostic | yes |
| Dream haze | changes the picture: `Dream haze.png`, 129600 of 129600 pixels | yes | yes | yes |
| Speed lines | name, in this place | Speed lines | Speed lines | yes |
| Speed lines | effects, in order | core.speed_lines | core.speed_lines | yes |
| Speed lines | the sentence the panel shows | Focus lines rushing in to the middle of the frame. Best on an adjustment layer. | Focus lines rushing in to the middle of the frame. Best on an adjustment layer. | yes |
| Speed lines | draws on an adjustment layer | no diagnostic | no diagnostic | yes |
| Speed lines | changes the picture: `Speed lines.png`, 26694 of 129600 pixels | yes | yes | yes |

**B-117: 49 of 49 checks pass.**
