# B-516: Shatter's other ten patterns

Built on 2026-10-12 as D-636, the last part of Shatter in the P0-17 group (D-600, your plan:
"how AE treats them/are done"), under the coordinator's approval for your /loop request.
Shatter's **Pattern** menu gains After Effects' other ten shapes: **Herringbone 2**,
**Squares & Triangles**, **Stars & Triangles**, **Overlapping Squares**, **Chevrons**,
**Puzzle**, **Js**, **Eggs**, **Crescents** and **Carpenter's Wheel**. Shatter now has all of
After Effects' patterns.

Adobe only names these patterns and never shows their exact shapes, so the shapes are ours,
drawn to match the names. They will not line up piece for piece with After Effects'. Curved
parts (puzzle knobs, eggs, crescents, the wheel) have small stair-step edges, as every Shatter
piece does. Projects saved before this open unchanged.

The check, `verification/D-636_patterns_table.md` (50 of 50), holds every pixel to numbers
worked out by a separate program before the code existed. The pictures are in
`verification/D-636 pictures/`: pictures 1 to 10 show each pattern's piece outlines in white
over black (Wireframe Front View, nothing moving yet), and 11 and 12 show two of them blown
apart.

## What to check

1. **Herringbone 2.** `1_herringbone_2_outlines.png`: small bricks laid in a zig-zag
   herringbone, each brick twice as long as it is wide.
2. **Squares & Triangles.** `2_squares_triangles_outlines.png`: a chequerboard where every
   other square is cut corner to corner into four triangles.
3. **Stars & Triangles.** `3_stars_triangles_outlines.png`: six-pointed stars with small
   triangles filling the gaps between them.
4. **Overlapping Squares.** `4_overlapping_squares_outlines.png`: big squares and small squares
   fitted together, each small one at a big one's corner.
5. **Chevrons.** `5_chevrons_outlines.png`: columns of V shapes stacked one inside the next.
6. **Puzzle.** `6_puzzle_outlines.png`: jigsaw pieces, each edge with a round knob poking one
   way or the other.
7. **Js.** `7_js_outlines.png`: J-shaped pieces locked together in rows.
8. **Eggs.** `8_eggs_outlines.png`: an egg in every square, wider at the bottom, with the
   pieces between them.
9. **Crescents.** `9_crescents_outlines.png`: a crescent moon in every square, with a round
   piece inside each and the pieces between them.
10. **Carpenter's Wheel.** `10_carpenters_wheel_outlines.png`: a wheel in every square, a
    small hub and eight spokes, with the pieces between them.
11. **Puzzle blown apart.** `11_puzzle_burst.png`: the town picture with puzzle pieces flying
    out of the middle.
12. **Stars blown apart.** `12_stars_burst.png`: the same with stars and triangles.
13. **In the app.** On Shatter, pick each new pattern from the Pattern menu, set the View to
    Wireframe Front View, and change Repetitions and Direction: the shapes should grow, shrink
    and turn. Set the View back to Rendered and play: the pieces should fly apart.
14. **Saved and opened again.** Save, close and open the project: the same pattern.

Our own choices you may want to judge: every one of the ten shapes, and how big each part is
within its square.

## Speed

Not timed: a pattern only decides which pixels belong to which piece; the pieces then fly
exactly as with the other patterns (see `verification/B-513_shatter_timing_table.md`).
