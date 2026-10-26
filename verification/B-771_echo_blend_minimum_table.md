# B-771: Echo's Blend and Minimum

D-891, accepted on 2026-10-26. Blend now draws as Add, the copies summed and held at 1; Minimum takes the darker straight colour of the copies that cover a pixel, at the largest covering. Every expected pixel is `Fixtures/echo/expected_echo.json`, written by `tools/echo_reference.py`; the answer is the largest difference against the catalogue's tolerance of 2e-5.

## FX-ECHO-008 and 012 (document 25), the two cases D-891 changed

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-ECHO-008 frame 3: One echo, Minimum (D-891): both drawings' covering, the smaller colour where they overlap. | largest difference 9.8e-8 | yes |
| FX-ECHO-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-012 frame 4: Three echoes, Blend (D-891): as Add, the four drawings summed and held at 1. | largest difference 1.7e-7 | yes |
| FX-ECHO-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |

## What the two operators draw, row 2

| Check | The build's answer | Matches |
| --- | --- | --- |
| Blend, frame 4: every pixel at least as light as the present drawing alone, channel by channel, as a sum is (an average of four was darker) | column 9 [1.0, 1.0, 0.0349313, 1.0] against the drawing's [0.79129803, 0.8713672, 0.00303527, 1.0] | yes |
| Minimum, frame 3: drawings 4 and 3 both drawn, columns 4 to 9 (before D-891 only the two columns where they overlapped, 6 and 7) | covered columns [4, 5, 6, 7, 8, 9] | yes |
| Minimum, frame 3, column 7, where both balls are whole: opaque, the darker green of drawing 3 and the darker blue of drawing 4 | [0.79129803, 0.3515327, 0.03189603, 1.0] | yes |

## A project saved before D-891

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_echo_008.json opens with no warning and saves holding what it held, operator word included | warnings [], saved the same | yes |
| fx_echo_012.json opens with no warning and saves holding what it held, operator word included | warnings [], saved the same | yes |

## Result

9 of 9 checks pass.
