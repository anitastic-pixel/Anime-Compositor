# B-398: Warp Blend

Built on 2026-10-11 as D-518, under your /loop request: Warp Blend, in **Transition**, our own
take on CycoreFX's CC WarpoMatic (a name of our own). The layer and the Layer to Reveal are
pulled out of shape along what differs between them while one blends into the other, a little
like a morph.

- **Completion** (0): how far the transition has gone; at 100 only the revealed layer is left.
- **Layer to Reveal** (none): the layer it turns into. Choose the layer itself to warp it alone.
- **Reactor** (Brightness): what steers the pull: the two layers' brightness, how far apart
  their brightness is, how far apart their contrast is, or the biggest difference in any colour.
- **Smoothness** (10): blurs what steers the pull, so small details stop mattering.
- **Warp Amount** (50): how far things are pulled; below 0 they are pulled the other way.
- **Warp Direction** (Joint): Joint pulls both layers along the same flow, Opposing pulls them
  against each other, Twisting swirls them round.
- **Blend Span** (50): how much of the transition the two layers spend mixed; at 0 the switch is
  sudden at the halfway point.

The layer does not grow.

CycoreFX publishes no formula and no settings when added, so the rule and those settings are
ours, written down in document 21.

The check, `verification/D-518_warp_blend_table.md` (248 of 248), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-518 pictures/`.

## What to check

1. **The street and what it turns into.** `1_street.png` is the shot; `2_reveal.png`, the same
   street in its opposite colours, is the layer to reveal.
2. **Begun.** `3_begun.png`: completion 25: still only the street, but bent along its edges.
3. **Halfway.** `4_halfway.png`: completion 50: half each, both bent along the same flow.
4. **Opposing.** `5_opposing.png`: the same, the two bent against each other.
5. **Twisting.** `6_twisting.png`: completion 50, more pull: the edges swirled round.
6. **Done.** `7_done.png`: completion 100: the opposite colours, flat again.
7. **In the app.** Add Warp Blend (Transition) to a layer over another, choose a Layer to
   Reveal, keyframe Completion from 0 to 100, and play. Try each Reactor and Direction.
8. **Out of range.** Type 101 in Completion: it is refused with a sentence saying the range.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when added; what each Reactor measures; one
blend across the whole layer rather than one that travels with the pull.

## Speed

`verification/B-398_warp_blend_timing_table.md`. Three Warp Blends on a 1920 by 1080 shot, milliseconds a frame played
again: PROVISIONAL (machine not quiet, other lanes' cargo runs sharing the machine), card 219.2, 226.2 and 86.0 ms a frame (completion 50, revealing layer 4; local differences, twisting, smoothness 30, revealing layer 4; smoothness 0, revealing nothing) against 47.7 for the Noise alone, processor 254.9, 464.4 and 195.5 against 79.8; a quiet re-measure is owed.
