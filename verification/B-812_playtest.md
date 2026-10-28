# B-812 playtest: Lens Blur closer to After Effects

What changed (D-932): Lens Blur gains **Diffraction Fringe** and **Highlight Saturation**, a Lens
Blur added from the Effect menu starts with the **Hexagon** iris, and the panel shows Threshold on
After Effects' 0 to 255 scale. Saved projects look exactly as before. This is the tutorial
jv4axtpnZps at 22:30 to 22:47.

1. Open a project with a dark picture with a few small bright dots. Select its layer and add
   Effect > Blur & Sharpen > Lens Blur.
   - Expect: Iris **Hexagon**, Diffraction Fringe **0**, Highlight Saturation **0**, and
     Threshold shown as a number from 0 to 255.
2. Set Radius about 9, Highlight Gain about 60, and lower Threshold until the dots light up.
   - Expect: each dot becomes a bright hexagon, evenly lit.
3. Raise Diffraction Fringe to 60.
   - Expect: each hexagon's edge is brighter than its middle, like a ring.
4. Fringe back to 0, Highlight Saturation to 100.
   - Expect: the coloured dots' hexagons are more strongly coloured; the white dot stays white.
5. Open an older project that already had a Lens Blur.
   - Expect: it looks exactly as before; its iris is not changed to Hexagon.
6. Save, close and reopen.
   - Expect: every setting reads as you left it and the picture is the same.

Pass if every "Expect" holds. If anything looks different, say which step.

Known limits:
- The timeline row of Threshold still shows the stored 0 to 100 value; only the effect panel shows
  0 to 255.
- With Diffraction Fringe or Highlight Saturation above 0 the blur is drawn on the processor, not
  the graphics card, so it can be slower.
- After Effects' "Linear Working Space" tickbox is not added (PROPOSED in D-932).

Checked by `cargo test --test b812_lens_parity` (66 of 66, `B-812_lens_parity_table.md`):
every pixel of FX-LENS-070 to 085 against `tools/lens_blur_parity_reference.py`. The old Lens
Blur cases are unchanged: B-59 74 of 74, B-64 101 of 101, D-359 148 of 148, B-775 99 of 99.

The sheet `verification/B-812 pictures/lens_blur_vs_tutorial.png` shows the tutorial's frame at
22:30 above the release app's Hexagon bokeh with Fringe 0 / 60 and Saturation 0 / 100. Pass if
the Fringe 60 tiles show ringed hexagons and the Saturation 100 tiles show stronger colour.
