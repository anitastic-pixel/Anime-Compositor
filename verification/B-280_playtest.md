# B-280: Tint

Built on 2026-10-09 as D-401, under your /loop request. **Tint** (in **Color Correction**) now
has After Effects' settings: it turns each pixel into a colour between two you choose, the dark
parts toward one and the light parts toward the other.

The settings, with the values they start at:

- **Map Black To** (black, #000000): the colour the darkest parts become.
- **Map White To** (white, #ffffff): the colour the lightest parts become.
- **Amount to Tint** (100): how much of the tinted picture is shown, 0 to 100; 0 leaves the layer
  exactly as it is.
- **Swap Colors**: a button that trades the two colours, as After Effects' checkbox does.

As added, black to white at 100, it turns the layer into greys. The two colours are typed as
#rrggbb, like Gradient Map's, and are not keyable; Amount to Tint is keyable.

Adobe does not publish its exact rule. Ours is Gradient Map's with its two ends only: each
pixel's brightness picks a point between the two colours.

**Tints in older projects.** A Tint saved before this change (one colour and an amount from 0 to
1) still opens with its old settings and draws exactly as before; its card shows the old Colour
and Amount fields. Only a Tint added from now on has the new settings.

The check, `verification/D-401_tint_table.md` (145 of 145), holds every pixel to numbers worked
out by a separate program before the code existed, reruns the older Tint's own checks from before
this change unchanged, and holds the graphics card's picture to within 1 level of the processor's
(identical on every test file). The pictures are in `verification/D-401 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **As added.** `2_as_added.png`: the street in greys, no colour left.
2. **Two colours.** `3_navy_to_gold.png`, Map Black To navy (#1a2a6c), Map White To gold
   (#fdbb2d): the shadows navy, the bright parts gold, the rest in between.
3. **Swapped.** `4_swapped.png`, the same two colours swapped: the shadows gold and the bright
   parts navy, like a negative of 3.
4. **Amount.** `5_amount_30.png`, navy to gold at Amount to Tint 30: the street with a light wash
   of the two colours.
5. **An older Tint.** `6_older_tint.png` is drawn from a Tint saved the old way (a purple, amount
   0.3). It should look like a light purple wash over the street.
6. **In the app.** Add Tint to a layer: the card shows Map Black To, Map White To, Amount to
   Tint and a **Swap Colors** button. The layer turns grey at once. Pick two colours: the
   layer takes them. Press Swap Colors: the two colours trade places, and so does the picture.
   Set Amount to Tint to 0: the layer is as it was.
7. **An older project.** Open a project saved before today that has a Tint: it looks the same
   as before, and its card shows Colour and Amount, as before.
8. **Out of range.** Type 101 in Amount to Tint: it is refused with a sentence saying it runs
   from 0 to 100, and the card keeps its old number.
9. **Saved and opened again.** Save, close and open the project: the same colours, amount and
   picture.

## Speed

**Timing.** In `verification/B-280_tint_timing_table.md`, the reference shot with a moving Noise on three layers, played again: 12.4 ms a frame on the card without the effect, 14.2 with Tint as added and 13.7 navy to gold at 30; about half a millisecond a layer on the card, inside the 1 ms aimed at. On the processor 41.8 without and about 51.5 with it (this round PROVISIONAL: the other lane started building while it ran).

## Not built

- After Effects' exact brightness rule is not published; ours is Gradient Map's.
- The two colours are not keyable (as Gradient Map's are not).
- No tutorial with numbers was found to reproduce.
