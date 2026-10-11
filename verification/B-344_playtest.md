# B-344: Repeat Tile

Built on 2026-10-11 as D-464, under your /loop request and your go-ahead: **Repeat Tile**, in
**Stylize**, after After Effects' CC RepeTile. The layer grows outwards by as many pixels as you
like on each side, and the new room is filled with copies of the picture: repeated, mirrored,
turned, laid as bricks, scattered at random or drawn out from the picture's edges. Good for
stretching a texture or background past the edge of the frame.

- **Expand Right**, **Expand Left**, **Expand Down** and **Expand Up** (0, 0 to 4000 pixels): how
  far the layer grows each way.
- **Tiling** (Unfold): how the copies are laid. Repeat (side by side), Checker Flip H and V,
  Unfold (mirrored about each edge, so the copies meet without a jump), Checker 180, Checker Flip
  45, Checker 90 CW and CCW, Rosette, Random, None (no copies), Turn CW, Turn CCW, Twist, Slide
  (the edge pixels drawn outwards) and Brick (every other row pushed half across).
- **Blend Borders** (0, 0 to 100%): blends the seams between copies away. Unfold, None and Slide
  ignore it.

The controls are After Effects' own; how it draws is our own rule beyond what the manual says,
and the starting values are ours (After Effects' are not published), so it is not matched to
After Effects pixel for pixel. A copy turned a quarter round is squeezed to the picture's shape,
and the layer grows the same both ways (the larger of Right and Left, of Down and Up), the side
you did not ask for left clear.

The check, `verification/D-464_repeat_tile_table.md` (187 of 187), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-464 pictures/`: the
street (`town.png`) in the middle of a composition twice its size, so the copies have room
(`1_before.png` without the effect).

## What to check

1. **Repeat.** `2_repeat.png`: the composition filled with copies of the street side by side,
   the street itself still in the middle.
2. **Unfold.** `3_unfold.png`: the copies mirrored about the street's edges, so each one meets
   its neighbour without a jump.
3. **Brick.** `4_brick.png`: like Repeat, with the rows above and below pushed half a street
   across.
4. **Blend.** `5_repeat_blend_100.png`: Repeat with Blend Borders 100; the hard seams of
   `2_repeat.png` softened away.
5. **One side.** `6_unfold_right_only.png`: grown to the right only, a mirrored half street beside
   the street and nothing on the left.
6. **Random and Twist.** `7_random.png` and `8_twist.png`: copies flipped and turned, at random
   in 7 and in a turning pattern in 8.
7. **In the app.** Add Repeat Tile (Stylize) to a picture smaller than the composition, raise the
   four expansions, step through the Tiling choices, and try Blend Borders with Repeat.
8. **Out of range.** Type 4001 in Expand Right: it is refused with a sentence giving the range.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-344_repeat_tile_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a
moving Noise and a Repeat Tile growing each of three layers 200 pixels each way, played again,
55.7 ms a frame on the card against 43.8 for the Noise alone; on the processor
154.6 against 93.4: a busy machine cannot say whether the card is quicker than the processor for this effect.
