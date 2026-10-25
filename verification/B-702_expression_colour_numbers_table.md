# D-822: an expression's colours are After Effects' numbers

B-702. A colour in an expression is four numbers 0 to 1, After Effects' sRGB numbers (Display, 8 bpc and AE 32 bpc, which are After Effects with Linearize Working Space off), and this build's linear numbers only in Float without AE 32 bpc (Linearize Working Space on). Each frame is the file with the expression; the expected frame is the file with the value the expression should give written in. The file's own constant, and D-820's linear reading, must draw differently. The Color Control holds the linear colour (0.25, 0.5, 0.75), which After Effects shows as about (0.537, 0.735, 0.881).

## It runs: the frame is the file with the expected value written in, and nothing is said

| Check | The build's answer | Matches |
| --- | --- | --- |
| The tutorial's typed orange on Fill's Color, `[1, 0.5, 0, 1]`, in Display: #ff8000 (D-820 gave #ffbc00): frame 2 | sample-identical to the expected value, unlike the other reading; said [] | yes |
| The same in 8 bpc: #ff8000: frame 2 | sample-identical to the expected value, unlike the other reading; said [] | yes |
| The same in AE 32 bpc: #ff8000: frame 2 | sample-identical to the expected value, unlike the other reading; said [] | yes |
| The same in Float, Linearize Working Space on: linear numbers, #ffbc00: frame 2 | sample-identical to the expected value, unlike the other reading; said [] | yes |
| Tint's Color (stored linear) given `[1, 0.5, 0, 1]` in Display: green 0.5 is linear 0.214041: frame 2 | sample-identical to the expected value, unlike the other reading; said [] | yes |
| A Color Control read, `effect("Color Control")("Color")[1] * 4` on Gaussian Blur's sigma: green 0.5 reads 0.735357: frame 2 | sample-identical to the expected value, unlike the other reading; said [] | yes |
| A Fill colour read, `effect("Fill")("Color")[1] * 4` with Fill at #ff8000: 128 / 255 = 0.501961: frame 2 | sample-identical to the expected value, unlike the other reading; said [] | yes |
| A Color Control linked into Tint, `effect("Color Control")("Color")`: the same colour as before, out and back: frame 2 | sample-identical to the expected value, unlike the other reading; said [] | yes |

## The orange on screen

| Check | The build's answer | Matches |
| --- | --- | --- |
| `[1, 0.5, 0, 1]` on Fill's Color in Display, pixel (12, 8) in sRGB codes: After Effects' orange 255, 128, 0 | 255, 128, 0 at alpha 1 | yes |

## Old projects as they were

| Check | The build's answer | Matches |
| --- | --- | --- |
| Fixtures/effect_mix/fx_mix_001.json, written before D-822, opens and saves holding what it held | the same | yes |
| Fixtures/projects/expression_controls_project.json, written before D-822, opens and saves holding what it held | the same | yes |

## Result

11 of 11 checks pass.
