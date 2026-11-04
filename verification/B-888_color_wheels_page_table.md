# B-888: Color Grade's colour wheels in the page

D-1008. Every case of `Fixtures/color_wheels/expected_wheel_drags.json` (written by `tools/color_wheels_reference.py` before the page code) run through the page's own `wheelAt` and `wheelDot`, then the wheels used with the mouse in a private, hidden copy of the app.

| Check | Got | Passes |
|---|---|---|
| WHEEL-001: The middle: no push, the hue kept. | hue 40, amount 0 | yes |
| WHEEL-002: A touch off the middle, under half a step: still no push, the hue kept. | hue 40, amount 0 | yes |
| WHEEL-003: Up and to the left at the edge: red, full. | hue 0, amount 100 | yes |
| WHEEL-004: Straight up, half way: hue 347, amount 50. | hue 347, amount 50 | yes |
| WHEEL-005: Straight down, a quarter: hue 167, amount 25. | hue 167, amount 25 | yes |
| WHEEL-006: To the right, 0.3: hue 257, amount 30. | hue 257, amount 30 | yes |
| WHEEL-007: To the left, 0.7: hue 77, amount 70. | hue 77, amount 70 | yes |
| WHEEL-008: Past the edge: held at 100. | hue 302, amount 100 | yes |
| WHEEL-009: A blue-ish point. | hue 221, amount 52 | yes |
| WHEEL-010: A hue that rounds across 0 to 359.5, so 0. | hue 0, amount 100 | yes |
| WHEEL-011: Hue 0, amount 100: up and to the left. | (-0.224951, 0.974370) | yes |
| WHEEL-012: Hue 120, amount 50. | (-0.365677, -0.340999) | yes |
| WHEEL-013: Hue 257, amount 30: to the right. | (0.300000, -0.000000) | yes |
| WHEEL-014: Amount 0: the middle. | (0.000000, 0.000000) | yes |
| dragging the Shadows dot half way to the right writes hue 257, amount 50 | hue 257, amount 50 | yes |
| a drag up, then Escape before letting go: the hue and amount put back | hue 257, amount 50 | yes |
| a double-click puts the amount back to 0 | hue 257, amount 0 | yes |
| the slider under Shadows writes Shadow Lightness -30 | lightness -30 | yes |
| a press on the Shadows wheel at WHEEL-009's point writes hue 221, amount 52 | hue 221, amount 52 | yes |

## Result

19 of 19 checks pass.
