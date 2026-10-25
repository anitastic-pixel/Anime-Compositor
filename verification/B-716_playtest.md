# B-716 playtest: Bend It turns 1.8 degrees a unit of Bend (D-836)

What changed: CC Bend It's Bend now curls like After Effects. Bend 100 bends the strip from
Start to End through half a turn (180 degrees), and Bend 200 through a whole circle. Before this
change, Bend 100 gave only 100 degrees. A freshly added Bend It now starts at Bend 25, which is the
same 45-degree curl it had before.

## What to look at

The pictures in `verification/B-716 pictures/` each show three panels side by side:

1. After Effects, from a tutorial.
2. Our app before the change.
3. Our app after the change.

The turn shown on our two panels is measured from the picture by fitting a circle to the red
stripe down the bar.

| Picture | After Effects | Before | After |
|---|---|---|---|
| bendit_1_morris_bend-75 | 137.5 | 75.0 | 134.9 (matches) |
| bendit_2_moveshapes_bend100: Bend 100 with Mirror closes a ring | 180.4 | 100.0, an open C | 179.9, a closed ring (matches) |
| bendit_3_3dpixel_bend26.4 | 48.1 | 26.4 | 47.5 (matches) |
| bendit_4_jake_bend-24 | 45.0 | 24.0 | 43.1 (close: this tutorial reads 1.85-1.90 a unit, the high end of the seven) |
| bendit_5_jake_bend-6 | 11.0 | 6.1 | 10.8 (matches) |
| bendit_6_morris_bend-56 | 102.4 | 56.0 | 100.8 (matches) |

All turns are in degrees. Before is wrong in every case, by 41 to 45 percent. After is within
2 percent in five cases and within 5 percent in the sixth.

## Try it in the app

1. Make a 1920x1080 composition. Put a white solid on it 40 wide and 600 tall, standing up the
   middle.
2. Add Distort > CC Bend It. It starts at Bend 25, and the bar leans over about an eighth of a
   turn, as it did before.
3. Set Bend to 100. The bar should curl into a half circle, its top end pointing straight down.
   Before this change, it stopped short at about 100 degrees.
4. Set Render Prestart to Mirror and move Start to the middle of the bar. At Bend 100 the two
   halves should meet in a closed ring, as in picture 2.
5. Open a project saved before this change that uses Bend It. It opens with no warning and keeps
   the Bend number it was saved with, but now curls 1.8 times as far. Bend 90 now gives
   162 degrees instead of 90. This is intended (D-836, "Saved projects").

## Tests

- `tests/b716_bend_it_turn.rs` measures the turn on a bent bar for Bend 25, 50, 100, -50 and 150,
  each within 1 degree of Bend x 1.8. It also checks the fixtures, an old project, and that the
  graphics card agrees with the processor within 1 level. Its results are in
  `verification/B-716_bend_it_turn_table.md`.
- `tests/b256_bend_it_bender_blobbylize.rs`, which also rewrites
  `verification/D-377_bend_it_table.md`.
