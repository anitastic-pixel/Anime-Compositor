# B-670 playtest: an expression on a colour, a point or a word (D-790)

What changed: a project with an expression on an effect's colour, point or word setting (for
example Drop Shadow's Shadow Color reading a Color Control) used to refuse to open
(PROJECT_SCHEMA_INVALID). Now it opens. The expression is kept and saved exactly as written. The
setting is drawn at its keyed or typed value, and every frame says EXPRESSION_TYPE naming the
setting. This build does not run such expressions yet.

## What the picture shows

`B-670 pictures/t4_neon_opens_before_after.png`: lane w7e's replay of Smertimba's soft neon glow
(seven Drop Shadows whose Shadow Color reads a Color Control), frame 0.

- before (build 94262cb3): the project is refused, "This project file cannot be opened, because
  part of it does not match the project format." Nothing is drawn.
- after: the project opens and the magenta glow draws at its typed colour. Beside the frame, the
  window says EXPRESSION_TYPE: "The expression on NEON's Drop Shadow color does not work at
  frame 0. Correct the expression, or switch it off."

Still different, not part of this fix: the two Drop Shadows at softness 1000 and 2000 are bypassed
(EFFECT_PARAMETER_INVALID, the range ends at 500). That is another lane's item.

## Try it yourself (5 minutes)

The window has no control yet for writing an expression on a colour, so this needs a project
that already carries one, such as the neon replay above
(`wave7/w7e/proj/t4_before_8_ae_ff00f0.json` in this session's scratch folder).

1. Open the project.
   - Expected: it opens. Before this change it was refused.
2. Look at the error details beside the frame.
   - Expected: EXPRESSION_TYPE naming NEON's Drop Shadow color, on every frame you visit.
3. Save the project under a new name, close it and open the copy.
   - Expected: it opens the same way. The expression text is still in the file.
4. Try to export it.
   - Expected: the export is refused and names the expression, as it already is for any other
     expression that does not work (B-670 table, "An export meets it").

Pass if every "Expected" holds. The numbers are in `B-670_held_expressions_table.md` (23 of 23).
