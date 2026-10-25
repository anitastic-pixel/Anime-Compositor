# D-820: expressions on an effect's colour and point, and reading any effect's setting

B-700. Each frame is the file with the expression; the expected frame is the same file with the value the expression should give written in, worked out by this test from After Effects' rules: a colour is four numbers, red, green, blue and alpha (in this build's linear numbers, FX-EXPR-017); a point is in the layer's pixels, which the file holds as per cent of the layer's width and height (bands is 16 by 10); a colour word is the sRGB code of the linear numbers. The file's own constant must draw differently, so no row passes by the expression doing nothing. The Color Control holds the linear colour (0.25, 0.5, 0.75).

## It runs: the frame is the file with the expected value written in, and nothing is said

| Check | The build's answer | Matches |
| --- | --- | --- |
| Fill's Color (a colour word) from a Color Control, `effect("Color Control")("Color")`: frame 2 | sample-identical to the expected value, unlike the file's constant; said [] | yes |
| Drop Shadow's Color, Smertimba's neon, as lane w7e typed it: frame 2 | sample-identical to the expected value, unlike the file's constant; said [] | yes |
| Drop Shadow's Color written as four numbers, `[1, 0, 0, 1]`: frame 2 | sample-identical to the expected value, unlike the file's constant; said [] | yes |
| Tint's Color (linear numbers) from a Color Control on another layer, `thisComp.layer("dot").effect("Color Control")("Color")`: frame 2 | sample-identical to the expected value, unlike the file's constant; said [] | yes |
| Twirl's Center (a point) from another layer's position, `thisComp.layer("dot").transform.position`: (4, 2) is 25 % and 20 %: frame 2 | sample-identical to the expected value, unlike the file's constant; said [] | yes |
| Twirl's Center from its own value, `value + [4, 0]`: 50 % of 16 is 8, and 12 is 75 %: frame 2 | sample-identical to the expected value, unlike the file's constant; said [] | yes |
| Gaussian Blur's sigma from a Slider Control, `effect("Slider Control")("Slider") / 2`: frame 2 | sample-identical to the expected value, unlike the file's constant; said [] | yes |
| Gaussian Blur's sigma from another layer's Directional Blur, `thisComp.layer("dot").effect("Directional Blur")("Length")`: frame 2 | sample-identical to the expected value, unlike the file's constant; said [] | yes |
| Gaussian Blur's sigma from the second effect's second setting, `effect(2)(2)`: frame 2 | sample-identical to the expected value, unlike the file's constant; said [] | yes |

## It fails: the frame is the file's constant, and the frame says why

| Check | The build's answer | Matches |
| --- | --- | --- |
| Two Gaussian Blurs each reading the other's sigma: frame 2 is the file's constant and says EXPRESSION_CYCLE | sample-identical; said ["EXPRESSION_CYCLE The expression on art's Gaussian Blur sigma_px does not work at frame 2.", "EXPRESSION_CYCLE The expression on art's Gaussian Blur sigma_px does not work at frame 2."] | yes |
| Drop Shadow's Color given three numbers, `[1, 0, 0]` (After Effects asks for four): frame 2 is the file's constant and says EXPRESSION_TYPE | sample-identical; said ["EXPRESSION_TYPE The expression on art's Drop Shadow color does not work at frame 2."] | yes |
| Twirl's Center reading another layer's Twirl Center, whose pixels need that layer's size: frame 2 is the file's constant and says EXPRESSION_TYPE | sample-identical; said ["EXPRESSION_TYPE The expression on art's Twirl center does not work at frame 2."] | yes |
| `effect("Gaussian Blur")("Blurriness")`, which this blur holds as a sigma: frame 2 is the file's constant and says EXPRESSION_REFERENCE_MISSING | sample-identical; said ["EXPRESSION_REFERENCE_MISSING The expression on art's Slider Control slider does not work at frame 2."] | yes |
| Fill's All Masks, a choice from a list, `1`: frame 2 is the file's constant and says EXPRESSION_TYPE | sample-identical; said ["EXPRESSION_TYPE The expression on art's Fill all_masks does not work at frame 2."] | yes |

## Saved as read, and old projects as they were

| Check | The build's answer | Matches |
| --- | --- | --- |
| Fill's Color with its expression: saved as read, expression text and all | {"base":"#ff0000","expression":{"enabled":true,"text":"effect(\"Color Control\")(\"Color\")"},"keyframes":[]} | yes |
| Fixtures/effect_mix/fx_mix_001.json, written before D-820, opens and saves holding what it held | the same | yes |
| Fixtures/projects/expression_controls_project.json, written before D-820, opens and saves holding what it held | the same | yes |

## Result

17 of 17 checks pass.
