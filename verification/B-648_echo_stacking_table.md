# B-648: Echo's Composite In Back and In Front

D-768, accepted on 2026-10-24. Composite In Back now puts the present copy at the back and each echo farther forward; Composite In Front puts the present copy in front. Every expected pixel is `Fixtures/echo/expected_echo.json`, written by `tools/echo_reference.py`; the answer is the largest difference against the catalogue's tolerance of 2e-5.

## FX-ECHO-010 and 011 (document 25), the two cases D-768 changed

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-ECHO-010 frame 4: Three echoes, decay 0.7, Composite In Back: the present ball at the back, the furthest echo on top. | largest difference 1.2e-7 | yes |
| FX-ECHO-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ECHO-011 frame 4: Three echoes, decay 0.7, Composite In Front: the present ball in front. | largest difference 1.2e-7 | yes |
| FX-ECHO-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |

## What each operator puts on top, frame 4, row 2

| Check | The build's answer | Matches |
| --- | --- | --- |
| In Front: the present ball's opaque column 9 is the drawing itself, whole on top | [0.79129803, 0.8713672, 0.00303527, 1.0] against the drawing's [0.79129803, 0.8713672, 0.00303527, 1.0] | yes |
| In Back: the same column is covered by the echoes, so it differs from the drawing | [0.79129803, 0.6657165, 0.0232378, 1.0] against the drawing's [0.79129803, 0.8713672, 0.00303527, 1.0] | yes |

## A project saved before D-768

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_echo_010.json opens with no warning and saves holding what it held, operator word included | warnings [], saved the same | yes |
| fx_echo_011.json opens with no warning and saves holding what it held, operator word included | warnings [], saved the same | yes |

## Result

8 of 8 checks pass.
