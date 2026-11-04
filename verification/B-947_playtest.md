# B-947 playtest: Text Animator, Based On Lines and Ease from -100 to 100 (D-1067)

The tutorial (iOknL8pPZGg, 1:20 to 1:50) brings a two-line title in one line at a time, and pulls
Ease High and Ease Low below 0.

## Steps

1. Make a text layer with two lines: type `LIQUID`, press Enter, type `DESIGN`.
2. Add Text Animator to it. Set Position to 0, 120 and Opacity to 0.
3. Set Based On to **Lines**, Shape to Square, Start 0, End 50. LIQUID should be gone and DESIGN
   should stay.
4. Set Offset to 50. Now DESIGN is gone and LIQUID stays.
5. Key Offset from -50 at frame 0 to 100 at frame 24 and play. The lines go one after the other,
   each as a whole.
6. Set Based On to **Words** with Offset 0. LIQUID goes and DESIGN stays, as in step 3. Before this
   change both words went, because the line break did not end a word.
7. Type a single line `ABCDEFGH`. Set Based On to Characters and Shape to Ramp Up. Set Ease Low to
   -50: the letters near the start move further than with Ease Low 0. Set Ease Low to 50: they
   move less.
8. Type -150 into Ease Low. The animator is switched off and its card shows a message. Type -100
   and it works again.

## What you should see

`verification/B-947 pictures/1_lines_first_picked.png` shows DESIGN only.
`2_lines_offset_half.png` shows LIQUID only.

Wrong if: in step 3 single letters of DESIGN also go; in step 5 a line goes letter by letter; in
step 7 -50 and 50 look the same; or an older title animated by characters looks different.
