# B-846 playtest: Remove Grain's further settings (D-966)

What changed: Remove Grain now has the rest of After Effects' groups, apart from the noise
sample boxes. A new Remove Grain starts with Chroma Suppression 1 and Texture 0.1, as After
Effects adds it. Projects you already saved look exactly as they did.

## Steps

1. Open the program and a composition with a grainy picture or video on a layer.
2. Select the layer and add Effect > Noise & Grain > Remove Grain.
3. Look at the Effect Controls.
4. Set Viewing Mode to Preview. Change Preview Region Width and Height to about half the
   picture. Then set Show Box to Off.
5. Set Viewing Mode to Noise Samples, then to Blending Matte, then back to Final Output.
6. Move Texture from 0.1 up to 1.
7. Set Red Noise Reduction to 0.
8. Set Temporal Filtering to On, on a video (grain that changes every frame). Play a few frames.
9. Open a project saved before this change that has a Remove Grain.

## What to see

- Step 3: Viewing Mode Final Output, Chroma Suppression 1, Texture 0.1, Noise Size Bias 0,
  Clean Solid Areas 0, Red/Green/Blue Noise Reduction 1, Temporal Filtering Off, Amount 100,
  Motion Sensitivity 0.9. Coloured speckles in the grain are calmer than light-and-dark ones.
- Step 4: only a box in the middle is smoothed; outside it the grain is untouched. With Show Box
  On the box has a thin white edge; with it Off the edge is gone.
- Step 5: Noise Samples shows the picture untouched with a thin white line round its edge (ours
  samples the whole layer). Blending Matte is plain white where the layer shows.
- Step 6: the grain comes back little by little; at 1 the layer looks as if there were no
  Remove Grain.
- Step 7: the red grain stays as it was; green and blue are still smoothed.
- Step 8: still areas look smoother than with Temporal Filtering Off; moving edges do not smear.
- Step 9: the older Remove Grain looks exactly as it did and shows the new settings at their
  quiet values (Chroma Suppression 0, Texture 0).

## Picture check

`B-846 pictures/remove_grain_sheet.jpg` puts the tutorial's grainy wall beside ours, drawn by the
real program. Grain measured on the same flat patch (lower is smoother): no effect 2.53, ours
before this change 2.22, ours now as added 1.73, After Effects' result 1.45. Ours now removes
about a third of the grain where After Effects removes about two fifths: closer, not yet equal.

If any of these is not so, B-846 is wrong.
