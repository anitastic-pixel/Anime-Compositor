# B-951 playtest: Radio Waves, Curve Size, Star, Star Depth and Frequency (D-1071)

The tutorial (f80Aj131YJs, 1:40 to 3:40) puts Radio Waves on an orange solid. It sets
Sides 6, Curve Size 0.780 and Frequency 1.90, then 4.40, and it shows the Star check box and
Star Depth.

## Steps

1. Make a 1920 by 1080, 30 fps composition with an orange solid, and add Radio Waves with a white
   colour. Rings come out of the middle.
2. Find the four new settings in the panel: **Curve Size** (0), **Star Depth** (-0.3),
   **Frequency** (0) and the **Star** choice (Off).
3. Set Sides to 6 and Frequency to 1.9. A new wave comes every 16 frames or so, about two a
   second; changing Interval now does nothing.
4. Set Frequency to 4.4. The waves come more than twice as often. Set it back to 0, and
   Interval rules again.
5. Set Curve Size to 0.78. Each hexagon's corners turn round, and the middles of its sides stay
   where they were. At 1 the waves are circles; at 0 the hexagons are sharp again.
6. Set Sides to 5 and Star to On. The waves become five-point stars. Drag Star Depth: towards -1
   the points get thinner, at 0 the waves are pentagons again, and above 0 the stars turn
   blunt.
7. Turn the graphics card on. The waves look the same with Curve Size and Star as without the
   card.
8. Save, close and open the project. All four settings come back.

## What you should see

`verification/B-951 pictures/shapes.png` holds three shots of white waves on orange, left to
right: sharp hexagons; hexagons at Curve Size 0.78; and five-point stars at Star Depth -0.5.

Wrong if:
- in step 3 the waves still come at the Interval;
- in step 5 the hexagons grow or shrink as a whole instead of rounding at the corners;
- in step 6 the points are between the corners instead of at them;
- a project saved before this change looks different.

Not the same as After Effects:
- How After Effects rounds a corner and shapes a star was not measured. Our Curve Size 0.78
  looks rounder than the tutorial's, which keeps more of each straight side.
- Curvyness, a rounded star, and the Image Contours and Mask wave types are not built.
- The profiles Sawtooth Out, Sawtooth In, Gaussian and Bell are not built.
- Lifespan and the fade times stay in frames, where After Effects uses seconds.
