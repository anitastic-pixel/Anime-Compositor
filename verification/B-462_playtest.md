# B-462: Contour Paint, Match to Nearest Custom Color

Built on 2026-10-11 as D-584, under your /loop request: Contour Paint (Stylize) can now fill with
**colours you choose**. After Effects' beta calls this Reduction Method **Match to Nearest Custom
Color**, with a **Custom Color Set**. Every flat fill takes whichever of your colours is nearest the
pixel as the eye sees it, not by plain red, green and blue numbers. Without any colours nothing is
filled. Posterization Level is not used in this mode.

- **Reduction Method**: K-Means, Median or (new) Match to Nearest Custom Color.
- **Colours, up to 8** (none to start with): shown under Reduction Method only when Match to Nearest
  Custom Color is picked. Add one by typing `#rrggbb` beside "Add:" or picking it; ✕ removes one.
  The colours are kept when you switch back to K-Means, though they are not used then.

Not built: **Freehand Flow Adjustments** (it needs painting on the frame, which effects here do not
have). The **arrowheads** on the Edge Map view are waiting on your answer, because adding them
would change two existing checks (FX-CONTOUR-014 and 018). Adobe has not said how many custom
colours After Effects allows or whether they can be keyed, so ours stops at eight and they are
not keyable.

The check, `verification/D-584_contour_custom_table.md` (68 of 68), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-584 pictures/`: the street
(`town.png`, `1_before.png` without the effect).

## What to check

1. **For comparison.** `2_k_means_5_colours.png`: fills by K-Means in 5 colours the program picks
   from the street.
2. **Your five colours.** `3_custom_5_colours.png`: the same street filled only with a night navy,
   a blue, a skin tone, a red and an off-white. Every flat colour should be one of those five.
3. **Two colours.** `4_custom_2_colours.png`: only a dark blue and a cream, dark where the street
   is dark and cream where it is light.
4. **No colours.** `5_custom_no_colours.png`: nothing filled, just the smear, the same as Contour
   Paint without fills.
5. **In the app.** Add Contour Paint to a picture; set Draw Color Fills to Over Smeared Image and
   Reduction Method to Match to Nearest Custom Color; the panel says no colour is chosen. Add a
   dark and a light colour and watch the picture go two-tone; add a third; remove one with ✕.
6. **Refused.** Type `#12` as a colour: it is refused with a sentence. With eight colours the Add
   line goes away.
7. **Undo.** Add a colour, then Undo: it goes away.
8. **Saved and opened again.** Save, close and open the project: the same colours and picture.

## Speed

In `verification/B-462_contour_custom_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a moving
Noise and a Contour Paint filling Over Smeared Image matched to 2 custom colours on each of three
layers, played again, 101.5 ms a frame on the card against 12.1 for the Noise alone,
and 104.1 with 8 colours; on the processor 705.1 against 47.9:
a busy machine cannot say whether the card is quicker than the processor for this effect.
