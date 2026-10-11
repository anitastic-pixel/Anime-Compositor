# B-420: Screen Key

Built on 2026-10-11 as D-540, on your word "for keylight, build our own, proceed with your
recommendations, even if we may need to rewrite/change things": Screen Key, in **Lines & Mattes**
(beside the other keyers), our own green and blue screen keyer. It is not Keylight and copies
none of it; a Keylight user will recognise the controls. It makes a pixel see-through by how far
its screen channel (green for a green screen) stands above its other two, against how far the
screen colour's does.

- **View** (Final Result): Final Result is the keyed layer; Screen Matte the matte as grey
  (white kept, black gone); Status shows each pixel as all gone (black), all kept (white) or in
  between (grey).
- **Screen Colour** (pure green): pick the screen's colour. Green or blue is found from it.
- **Screen Gain** (100): above 100 keys more, below less.
- **Screen Balance** (50): how the other two channels are weighed; about 50 for green screens,
  95 for blue.
- **Clip Black / Clip White** (0 / 100): the matte at or below Clip Black made all gone, at or
  above Clip White all kept, stretched between.
- **Despill Bias** (50) and **Despill** (100): the screen's colour cast taken off what is kept.
  Higher bias removes more.
- **Shrink/Grow** (0), **Softness** (0) and **Edge Refine** (0): the kept part's edge pulled in
  or pushed out, softened, and fitted to the picture's colours (as Key Cleaner does).

Not built: a light wrap (put **Light Wrap** after Screen Key); Keylight's screen pre-blur, clip
rollback, despot, inside and outside masks, alpha bias, replace colour and edge colour
correction; a picked despill colour (the bias is a number); an eyedropper; warning colours in
Status. The views show the matte before Edge Refine, and Edge Refine can fill in a little of the
see-through part just outside an edge (Key Cleaner does the same).

The check, `verification/D-540_screen_key_table.md` (156 of 156), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-540 pictures/`.

## What to check

1. **The shot.** `1_before.png`: a green screen, a face whose edge has picked up green spill,
   and a blue shirt.
2. **As added.** `2_as_added.png`: screen colour pure green. The screen, duller than pure green,
   is half see-through; the face and the shirt are whole.
3. **Colour picked.** `3_picked.png`: screen colour picked from the screen. The screen is gone;
   the face and shirt are untouched; the face's spilled edge is partly see-through with its green
   cast taken out.
4. **Clipped.** `4_clipped.png`: Clip Black 30, Clip White 70. The screen is gone and the face's
   edge nearly solid.
5. **Screen Matte.** `5_screen_matte.png`: the same as a matte: black screen, white face and
   shirt.
6. **Status.** `6_status.png`: black screen, white face and shirt, the few in-between pixels at
   the face's edge grey.
7. **Edge.** `7_edge.png`: then Shrink/Grow -2, Softness 1.5, Edge Refine 3: the face's edge
   pulled in and softened.
8. **In the app.** Add Screen Key (Lines & Mattes) to green-screen footage, set Screen Colour to
   the screen, look at Screen Matte and Status while you raise Clip Black and lower Clip White
   until the screen is black and the subject white, then go back to Final Result and adjust
   Despill. Keyframe Screen Gain and play it.
9. **Out of range.** Type 201 in Screen Gain: it is refused with a sentence saying the range.
10. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when added; the rule itself (it is ours, not
proven equal to Keylight's, so the same footage keys differently from Keylight); how strong
Despill is at bias 50; and whether Edge Refine is worth its cost.

## Speed

PROVISIONAL: measured while other builds were running
(`verification/B-420_screen_key_timing_table.md`), so the card's numbers are not readable yet. On
the processor about 9.4 ms a 1080p layer as a plain key, and about 344 ms a layer with shrink,
softness and edge refine (mostly the edge refine, as Key Cleaner).
