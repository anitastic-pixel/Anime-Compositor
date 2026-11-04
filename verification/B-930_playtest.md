# B-930 playtest: dashed strokes on shapes

What changed (D-1050): a shape's stroke can be dashed. In the shape's panel, below Trim paths, a
**Dashes** switch turns them on, with **Dash**, **Gap** and **Offset** rows, all in pixels and
each with a key diamond. That is After Effects' Stroke > Dashes, used in the tutorials for dotted
guide lines and dashes that march along a line.

1. Make a shape layer and draw an open line with the pen, or a rectangle. Give it a stroke about
   8 pixels wide.
2. In its panel, turn Dashes on.
   - Expect: the line becomes dashes 10 pixels long with 10-pixel gaps (Dash 10, Gap 10,
     Offset 0), starting with a dash where the line starts.
3. Set Dash 30, Gap 15.
   - Expect: longer dashes, shorter gaps.
4. Set the stroke's cap to Butt, then Round.
   - Expect: Butt gives flat dash ends; Round gives rounded ends that reach a little into the
     gaps.
5. Round caps, Dash 0, Gap 20.
   - Expect: a dotted line of round dots.
6. Set Gap 0.
   - Expect: a solid line again.
7. Back to Dash 30, Gap 15. Key Offset 0 at the start and 200 a second later, and play.
   - Expect: the dashes march back along the line, smoothly.
8. Turn Trim paths on as well, End 50.
   - Expect: only half the line is drawn, and it is dashed from where it starts.
9. Turn Dashes off.
   - Expect: the solid line. Undo (Ctrl+Z) brings the dashes back with their keys.
10. Save, close and open the project again.
    - Expect: the dashes and the Offset keys are as you left them.

Pass if every "Expect" holds. If anything looks different, say which step.

Checked by `tests/b930_shape_dashes.rs`, `verification/B-930_shape_dash_table.md` (24 of 24),
against `verification/D-1050 pictures/dash_cases.png` (each row a case: 131 shows two flat dashes,
135 round dots, 137 a rectangle's sides dashed) and the app test
`d1050_a_shape_is_dashed_from_the_window`.
