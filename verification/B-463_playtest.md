# B-463: Contour Paint, Arrowheads

Built on 2026-10-12 as D-585, under your /loop request: Contour Paint's **Edge Map** view (Stylize)
now draws **arrows**, as After Effects' does. Each short cyan line that shows the flow now ends in a
small V-shaped head, so you can read which way the paint will be pushed.

- **Arrowheads**: On or Off. **On** when you add a new Contour Paint. A project saved before this
  change opens with them **Off**, so it looks exactly as it did.
- **Offset Flow Angles 180** turns every arrow round: the heads move to the other ends.
- The heads only appear in the Edge Map view; the painted Final Result does not change.

Ours, not Adobe's: After Effects always draws its arrows, so the On/Off switch is ours (it keeps
old projects unchanged), and Adobe does not say how big the heads are or how wide they open; ours
are a fifth of the 12-pixel cell long and open 30 degrees each side.

The check, `verification/D-585_contour_heads_table.md` (61 of 61), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture to
within 1 level of the processor's. The two older Edge Map checks (FX-CONTOUR-014 and 018) are
unchanged and still pass. The pictures are in `verification/D-585 pictures/`, the street from
before (`town.png`).

## What to check

1. **No heads.** `1_edge_map_no_heads.png`: the street's Edge Map as an old project shows it: grey
   edges with short cyan lines.
2. **Heads.** `2_edge_map_arrowheads.png`: the same, each cyan line now ending in a tiny V (zoom in;
   they are small at this size).
3. **Turned round.** `3_arrowheads_offset_180.png`: Offset Flow Angles 180, the heads at the other
   ends.
4. **Final Result unchanged.** `4_final_result_unchanged.png`: the painted street, the same with
   Arrowheads on or off.
5. **In the app.** Add Contour Paint and set View to Edge Map: the arrows have heads. Switch
   Arrowheads to Off: plain lines. Open a project saved before today with an Edge Map: Arrowheads
   shows Off and the picture is as it was.
6. **Undo, save and open.** Switch Arrowheads, Undo; save, close and open: the same setting and
   picture.

## Speed

In `verification/B-463_contour_heads_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a moving
Noise and a Contour Paint Edge Map on each of three layers, played again, 32.4 ms a frame on
the card with Arrowheads off and 30.5 with them on, against 21.0 for the Noise alone;
on the processor 175.0 and 240.9 against 69.1:
a busy machine cannot say whether the card is quicker than the processor for this effect.
