# B-582: Particle Grid in a layer's shape, and strings

Built on 2026-10-13 as D-712 (P0-6), under your /loop request. Particle Grid (B-496) gains two
Base Forms and one layer setting:

- **Box Strings**: the grid's points, each row drawn as a line from point to point, so the grid
  becomes strings that ripple, twist and turn as the points do. Size sets the line's width.
  Particle Type has no say here (strings are lines).
- **Layer**: the grid's points, kept only where the new **Base Layer** is solid (half opaque or
  more). Choose any layer: a drawing, a shape layer, or a text layer to get points in the shape of
  its words. The Base Layer can be switched off and still be used. With no Base Layer chosen,
  Layer draws the plain grid.

Files saved before this change open with no Base Layer and save exactly as before.

The check, `verification/D-712_particle_grid_shape_table.md` (148 of 148), holds every pixel and
every kept point to numbers worked out by a separate program before the code existed, and the
graphics card's picture to the processor's. The pictures are in `verification/D-712 pictures/`.

## What to check

1. **Points in the word.** `1_points_in_the_word.png`: the word FORM spelled in small yellow dots
   over the street, no dots outside the letters.
2. **The word rippling and glowing.** `2_the_word_rippling_and_glowing.png`: the same word,
   turned a little, in pink glowing dots, rippling.
3. **Strings rippling.** `3_strings_rippling.png`: rows of pale blue lines tipped back into the
   distance, waving like a flag.
4. **Strings coloured by the street.** `4_strings_coloured_by_the_street.png`: two layers of
   twisted strings, each coloured by the street behind it.
5. **In the app.** Add a text layer with a word and switch it off. Add Particle Grid to another
   layer, raise Particles in X and Y to about 150 and 80, set Form Size to the composition's size,
   set Base Form to Layer and Base Layer to the text layer: dots in the shape of the word. Then
   set Base Form to Box Strings, raise Displace and play: waving lines.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge (Red Giant publishes no formula): a point is kept where the
Base Layer is at least half opaque; the shape is read where each point sits before the ripple and
turn, so the shape moves with the grid; each line takes its start point's size, opacity and
colour; strings run across (along Particles in X).

## Speed

`verification/B-582_particle_grid_shape_timing_table.md`. Provisional: other tests were running.
Strings and the layer shape each cost about as much as the plain grid.
