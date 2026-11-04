# B-926 playtest: three-part position expressions on flat layers

What changed (D-1046): expressions copied from After Effects tutorials that are written for 3D
positions now run on our flat layers, as After Effects runs them. A position (or anchor, scale)
expression may end on three numbers `[x, y, z]`; the third is dropped. `[2]` of a two-number
value, such as `wiggle(1, 50)[2]` or `value[2]` on a position, reads 0. `value.length` gives how
many numbers the value has (2 on a position). Nothing changes in saved files.

1. Put any layer in a composition. Alt-click the stopwatch of its Position and paste the
   tutorial's expression (nlgsV151IVU at 2:10), each statement on its own line:
   `x = wiggle(1, 500)[0];` `y = wiggle(1, 100)[1];` `z = wiggle(2, 0)[2];` `[x, y, z]`
   - Expect: no warning. Play: the layer shakes widely left and right and a little up and down,
     as in the tutorial.
2. Replace it with `[value[0], value[1], 300]`.
   - Expect: no warning; the layer stays where its keys put it (the 300 is dropped).
3. Replace it with `[value[0] + value.length * 100, value[1]]`.
   - Expect: no warning; the layer sits 200 pixels right of its keys.
4. Replace it with `[1, 2, 3, 4]`.
   - Expect: a warning, as before (four numbers are too many), and the layer at its keys.
5. On Rotation, type `value.length`.
   - Expect: a warning, as before (a rotation is one number, it has no length).
6. Save, close and reopen.
   - Expect: the expressions read as you typed them and the layer moves the same.

Pass if every "Expect" holds. If anything looks different, say which step.

Checked by `cargo test --test b926_three_part_expressions` (48 of 48,
`B-926_three_part_expressions_table.md`): twelve expressions at frames 0, 12 and 24 against
`tools/expression_reference.py --three-part`, six let through and six still refused. The older
expression checks still pass: `b14b_expressions` (FX-EXPR-001 to 016) and
`b646_expression_controls` (017 to 020), and the evaluator's own tests.

Not done: there is still no true depth (P0-12); the third number is dropped, not drawn, and
`[2]` does not read the layer's Depth plane (D-58).
