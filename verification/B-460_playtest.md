# B-460: Card Wipe

Built on 2026-10-11 as D-580, under your /loop request: Card Wipe, under **Transition**, our
version of After Effects' effect of that name. It cuts the layer into a grid of cards that flip
over, one after another, to show another layer on their backs. It draws its own 3D look: the
composition's camera and lights are not used, the same way CC Cylinder and CC Sphere work.

- **Transition Completion** (0): how far the wipe has got; 100 has every card turned over.
- **Transition Width** (50): how wide the band of cards that are turning is; 0 flips each card
  at once.
- **Back Layer**: the layer seen on the cards' backs; with none, the backs are clear.
- **Rows & Columns, Rows, Columns** (8 by 8): the grid; Columns Follows Rows uses the rows for both.
- **Card Scale** (1): below 1 leaves gaps between the cards, above 1 makes them overlap.
- **Flip Axis, Flip Direction**: turn about the across (X) or upright (Y) axis, one way or the
  other, or Random for each card.
- **Flip Order**: which side goes first, or Gradient, where the dark parts of the **Gradient
  Layer** flip first.
- **Timing Randomness, Random Seed**: shuffle the order.
- **Camera** (X, Y, Z Rotation, X, Y Position, Z Position, Focal Length): look at the cards
  from another angle. As added it looks straight on, so nothing moves until the cards turn.
- **Light** (Type, Intensity, Color, Position, Depth, Ambient) and **material** (Diffuse,
  Specular, Highlight Sharpness): how the cards are lit.

Adobe does not say exactly how it draws, so when each card turns, the camera, the light and the
settings when added are ours. Not built yet: the composition's camera and corner pins as the
camera, the first composition light, Position Jitter and Rotation Jitter, and the choice of
Transform Order. The cards' edges are not smoothed.

The check, `verification/D-580_card_wipe_table.md` (300 of 300), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the card to the processor
within 1 level. The pictures are in `verification/D-580 pictures/`, drawn on the street
pictures: the left street is the layer, the right street the back.

## What to check

1. **Halfway.** `1_halfway_left_to_right.png`: the left columns turned over to the right
   street, the right columns still the left street, and a band in between seen edge-on as they
   turn.
2. **Camera turned, lit.** `2_halfway_turned_camera_lit.png`: the same, seen from a little
   to the side and above, with a shine on the cards facing the light.
3. **Gradient order.** `3_gradient_order.png`: the cards under the dark part of the ramp (top
   right) turned first.
4. **Shuffled.** `4_shuffled_random_flips.png`: cards turned in a scattered order, some about
   the upright axis and some about the across axis.
5. **Small cards, no back.** `5_small_cards_no_back.png`: a finer grid with gaps between the
   cards; the turned cards are clear (see-through), which a picture viewer shows as white or a checkerboard.
6. **Blinds.** `6_venetian_blinds.png`: ten strips the full width, all turning together like
   blinds, caught exactly half turned, so each is seen edge-on as a thin line with clear
   between them.
7. **All turned over.** `7_all_turned_over.png`: the right street whole, upright and not
   mirrored.
8. **In the app.** Add Card Wipe (Transition) to a layer, choose another layer as its Back
   Layer, and key Transition Completion from 0 to 100 over a second or two: the cards sweep
   over from left to right.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: how the band of turning cards looks as added, the light
as added, and how deep the camera makes it look when turned.

## Speed

Measured on a quiet machine: on the card the reference shot took about 71 to 83 ms a frame with Card Wipe on three layers against about 12.5 ms without it, so the effect costs about 20 to 23 ms a frame for each layer at 1920 by 1080, and a frame with it is slower than real time; on the processor it costs more, about 78 to 94 ms a frame (`verification/B-460_card_wipe_timing_table.md`). Making the card pass faster would be a separate piece of work.
