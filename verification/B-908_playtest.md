# B-908 playtest: Gradient's Ramp Scatter and Blend With Original

What changed (D-1029): the Gradient effect has two new settings at the bottom of its panel, as
After Effects' Gradient Ramp has.
- **Ramp Scatter** (0 to 256) sprinkles a fine grain along the ramp, so a soft sky shows no
  stripes (banding).
- **Blend With Original** (0 to 100) mixes the layer as it was back in; 100 hides the gradient.

Both start at 0, and an older project draws exactly as before.

1. Add a solid, then Gradient from the effects list. Set Blend to normal, both opacities to 100,
   the start colour to a very dark blue (#0a1430) and the end colour to a mid blue (#3c6eb4).
   - Expect: a dark-to-light blue sky. Zoomed in to 400%, you can see faint horizontal stripes.
2. Set Ramp Scatter to 64.
   - Expect: the stripes are gone, replaced by a fine even grain. Compare
     `verification/B-908 pictures/1_scatter_0_zoom.png` (stripes) with `2_scatter_64_zoom.png`
     (grain).
3. Play or scrub the timeline.
   - Expect: the grain stays still; it does not crawl from frame to frame.
4. Move the layer.
   - Expect: the grain moves with the layer.
5. Set Ramp Scatter back to 0 and Blend With Original to 50.
   - Expect: the solid's own colour shows through half (`4_blend_with_original_50.png` shows the
     same over the street picture). At 100 the gradient is gone entirely.
6. Click Ramp Scatter's stopwatch at frame 0 with the value 0, go to frame 24 and set 256.
   - Expect: the grain grows from nothing to strong as you play.
7. Type 300 into Ramp Scatter.
   - Expect: it is refused (the range is 0 to 256).
8. Save, close, and open the project again.
   - Expect: both settings come back as you left them.

Note: how strong After Effects' own scatter is per unit could not be measured from the tutorial
video, so our 64 may look stronger or weaker than After Effects' 64. If it looks wrong next to
After Effects, say which value matched, and that becomes a new decision.

The numbers are checked in `verification/B-908_gradient_scatter_table.md` (123 of 123).
