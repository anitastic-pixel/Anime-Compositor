# B-384: Linear Color Key

Built on 2026-10-10 as D-504, under your /loop request: Linear Color Key, in **Lines & Mattes**
(beside the other keyers), with After Effects' settings but Key Operation. It keys out the
colours close to a key colour: the closer a pixel is, the more see-through it becomes.

- **View** (Final Output): Final Output is the keyed layer; Source Only the layer untouched;
  Matte Only the see-through part as grey (white kept, black gone).
- **Key Color** (pure blue): the colour to key out. Set it to your screen's colour.
- **Match Colors** (Using RGB): how closeness is judged. Using RGB compares red, green and blue;
  Using Hue compares only the hue, so a darker or paler green still counts as green; Using Chroma
  compares the colour without its brightness, so the screen in shadow is still the screen.
- **Matching Tolerance** (10): how close counts as the key colour; 100 keys out everything.
- **Matching Softness** (10): how far beyond the tolerance colours fade out instead of
  stopping sharply; 0 is a hard edge.

Adobe publishes no formula and no settings when added, so the rule and those settings are ours,
written down in document 21. Not built: After Effects' **Key Operation: Keep Colors** (used with
a second Linear Color Key to bring back colours the first keyed out). It needs the colour under
pixels that are already fully keyed, which this program does not keep; whether to build that is
your decision. One known difference: at Matching Tolerance 0 we still key the exact key colour,
where Adobe says 0 leaves the whole picture solid.

The check, `verification/D-504_linear_color_key_table.md` (131 of 131), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-504 pictures/`.

## What to check

1. **The shot.** `1_before.png`: a green screen, a face whose edge has picked up green spill,
   and a blue shirt.
2. **Using RGB.** `2_rgb.png`: keyed on the screen's colour, tolerance 10, softness 10. The
   screen is gone (see-through); the face, its green-tinged edge and the shirt are whole.
3. **Using Hue.** `3_hue.png`: tolerance 20. The screen is gone, and the green-tinged edge of the
   face fades out a little, because its hue is close to the screen's.
4. **Matte Only.** `4_hue_matte.png`: the same as 3 as a matte: black screen, white face and
   shirt, with a thin grey rim at the face's edge.
5. **In the app.** Add Linear Color Key (Lines & Mattes) to green-screen footage, set Key Color to
   the screen, try the three Match Colors and the two sliders, and switch the View to Matte Only.
   Keyframe the tolerance and play it.
6. **Out of range.** Type 101 in Matching Tolerance: it is refused with a sentence saying the
   range.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: pure blue, Using RGB and 10 / 10 when added; that a grey
counts as unlike every colour under Using Hue; and how strong the softness's fade is, which you
can compare with After Effects' own Linear Color Key on the same shot.

## Speed

Measured on a quiet machine (`verification/B-384_linear_color_key_timing_table.md`). Three Linear Color Keys on a 1920 by
1080 shot, card / processor, milliseconds a frame played again: the shot after a moving Noise 12.8 / 40.6 ms a frame, as added (RGB) 18.8 / 48.5, hue, key orange, tolerance 30 18.9 / 49.4, chroma, the matte 18.8 / 49.9; about 2.0 ms a 1080p layer on the card, over the row's Target P1 (1 ms), as the other keyers are; on the processor about 2.6 to 3.1 ms a layer.
