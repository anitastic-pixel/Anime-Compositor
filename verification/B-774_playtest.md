# B-774 playtest: pixels nobody can see are not graded (D-894)

What changed: nothing in the program. Only the checks that hold the graphics card's picture
against the processor's picture changed. They no longer grade the colour of a pixel that is
fully transparent in both pictures, because that colour cannot be seen against any background.
Nothing you see, save or export is different.

## What to look at

1. Open `verification/B-774_unseen_pixels_table.md`.
   - **Expect:** 4 of 4 checks pass. The first row shows 13,312 pairs of fully transparent
     pixels with different hidden colours, and a difference of **0** over black and over white.
   - **Expect:** a pixel transparent on only one side is still graded (255).
2. Open `verification/B-748_card_blend_modes_table.md` and search for "D-894".
   - **Expect:** 20 rows, all with Preserve Transparency on. Each mentions 1 to 3 pixels that
     were not graded, ends "largest difference over black or white 0", and has **yes** in the last column.
     These rows passed before this change with the same numbers.
3. Open `verification/B-688_eight_card_table.md`, `B-749_eight_adjust_wrap_table.md` and
   `B-750_eight_card_effects_table.md`.
   - **Expect:** no row mentions D-894. On the fixture projects, no pixel was excused.

## If something looks wrong

Say which table and row. To undo the rule, make `seen` in `tests/effect_table/mod.rs` compare
all four channels again. That is one line.
