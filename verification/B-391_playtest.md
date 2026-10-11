# B-391: Teeth Wipe

Built on 2026-10-11 as D-511, under your /loop request: Teeth Wipe, in **Transition**, our own
take on CycoreFX's CC Jaws (a name of our own). The layer splits in two along a toothed cut,
and the two halves slide apart, like jaws opening, showing what is underneath.

- **Completion** (0): how far the jaws have opened; 0 the layer whole, 100 both halves gone.
- **Center** (50, 50): where the cut goes through, in per cent of the layer's width and height;
  the cut also turns about this point.
- **Direction** (0): turns the cut; 0 runs left to right, 90 top to bottom.
- **Height** (40): how tall the teeth are, in pixels; 0 is a straight cut.
- **Width** (40): how far apart the teeth are, in pixels.
- **Shape** (Spikes): Spikes (pointed), Trapezoids (flat-topped) or Blocks (square).

The halves slide straight apart; they do not swing open. The teeth's edges are hard. The layer
does not grow.

CycoreFX publishes no formula and no settings when added, so the rule and those settings are
ours, written down in document 21. The shape names are ours too: Trapezoids is CycoreFX's
"Robo Jaw", Blocks its "Block".

The check, `verification/D-511_teeth_wipe_table.md` (171 of 171), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-511 pictures/`; the checkerboard or white you see in a viewer is where the layer is
gone.

## What to check

1. **The street.** `0_the_street.png`: the shot without the effect.
2. **Spikes, parted.** `1_spikes_parted.png`: the top half moved up and the bottom half down, a
   zigzag gap between them, the points of one half facing the gaps of the other.
3. **Trapezoids, parted.** `2_trapezoids_parted.png`: the same with flat-topped teeth.
4. **Blocks, upright.** `3_blocks_upright.png`: direction 90, the cut running top to bottom with
   square teeth, the halves moved left and right.
5. **Tall narrow spikes.** `4_tall_narrow_spikes.png`: teeth 80 high and 20 apart.
6. **Done.** `5_done.png`: completion 100, nothing left.
7. **In the app.** Add Teeth Wipe (Transition) to a layer over another, keyframe Completion from 0
   to 100 and play: the jaws open and show the layer below. Try each Shape and turn Direction.
8. **Out of range.** Type 0 in Width: it is refused with a sentence saying the range.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when added; that the halves slide rather than
swing; how far they move.

## Speed

`verification/B-391_teeth_wipe_timing_table.md`. Three Teeth Wipes on a 1920 by 1080 shot, milliseconds a frame played
again: card 19.2 to 23.8 ms a frame against 15.9 for the Noise alone, processor 194.3 to 214.2 against 116.2, PROVISIONAL (another lane's card test was running; a quiet re-measure is owed).
