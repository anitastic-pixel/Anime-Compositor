# B-891: Color Grade's Hue vs Saturation curve in the page

D-1011. Every case of `Fixtures/hue_curve_drag/expected_hue_curve_drags.json` (written by `tools/hue_curve_drag_reference.py` before the page code) run through the page's own `hueCurveAt`, `hueCurveValue`, `hueCurveAdd` and `hueCurveMove`, then the graph used with the mouse in a private, hidden copy of the app.

| Check | Got | Passes |
|---|---|---|
| HUECURVE-001: The top left: hue 0, saturation 200. | hue 0, saturation 200 | yes |
| HUECURVE-002: The middle: hue 180, saturation 100. | hue 180, saturation 100 | yes |
| HUECURVE-003: The bottom right: held to hue 359, saturation 0. | hue 359, saturation 0 | yes |
| HUECURVE-004: A third across, a quarter down: hue 120, saturation 150. | hue 120, saturation 150 | yes |
| HUECURVE-005: Outside the graph: held to 0 and 200. | hue 0, saturation 200 | yes |
| HUECURVE-006: Hue 0.4 of a step past 90 rounds down. | hue 90, saturation 100 | yes |
| HUECURVE-007: No points: 100 at every hue. | 100.000000 | yes |
| HUECURVE-008: One point: its saturation at every hue. | 40.000000 | yes |
| HUECURVE-009: Two points, on the first. | 100.000000 | yes |
| HUECURVE-010: Two points, half way: the easing's middle, 50. | 50.000000 | yes |
| HUECURVE-011: Two points, a quarter of the way: 100 - 100 x 0.15625 = 84.375. | 84.375000 | yes |
| HUECURVE-012: Past the last point, round the circle to the first: half way from 180 to 360, 50. | 50.000000 | yes |
| HUECURVE-013: Before the first point, through 360: half way from 300 to 420 (60). | 100.000000 | yes |
| HUECURVE-014: Into no points: on the flat line, 100. | {"points": [[200, 100]], "index": 0} | yes |
| HUECURVE-015: Between two: on the line at 90, 50. | {"points": [[0, 100], [90, 50], [180, 0]], "index": 1} | yes |
| HUECURVE-016: After the last, round the circle: at 270, 50. | {"points": [[0, 100], [180, 0], [270, 50]], "index": 2} | yes |
| HUECURVE-017: On a hue a point already has: nothing added. | null | yes |
| HUECURVE-018: A 17th point: refused. | null | yes |
| HUECURVE-019: A lone point dragged anywhere. | [[300, 20]] | yes |
| HUECURVE-020: Dragged past its right neighbour: held one below it. | [[0, 100], [199, 60], [200, 80]] | yes |
| HUECURVE-021: Dragged past its left neighbour: held one above it. | [[0, 100], [1, 60], [200, 80]] | yes |
| HUECURVE-022: The last point dragged past 359: held to 359. | [[0, 100], [359, 80]] | yes |
| HUECURVE-023: Dragged above 200 and below 0: held. | [[0, 200], [200, 80]] | yes |
| HUECURVE-024: Dragged below 0. | [[0, 100], [200, 0]] | yes |
| a press on the flat line at hue 120 adds a point there at 100, the picture unchanged | [[120, 100]] | yes |
| dragging that point to the bottom writes 120 0 (the greens greyed) | [[120, 0]] | yes |
| a drag up, then Escape before letting go: 120 0 put back | [[120, 0]] | yes |
| the typed box shows what the graph wrote | '120 0' | yes |
| a double-click on the point takes it away: no points, no change | [] | yes |
| two drags: the reds to 180, the blues to 20 | [[0, 180], [240, 20]] | yes |

## Result

30 of 30 checks pass.
