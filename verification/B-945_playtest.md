# B-945 playtest: Mercury after After Effects (D-1065)

What changed: Mercury now takes After Effects' CC Mr. Mercury numbers, its drops show the
layer under them, a producer with a radius of 0 still makes drops, and all twelve of After
Effects' Animation choices are there. Projects saved before this keep their old look.

## Steps

1. Start the app and make a new composition, 1920 by 1080.
2. Add a text layer that says LIQUID DESIGN in big white letters on black.
3. Select the text layer and add Mercury from the Effects window (Simulation).
4. Look at Effect Controls. You should see Birth Rate 1, Radius X 5, Radius Y 5, Velocity 1,
   Gravity 1, Extra 1, Blob Birth Size 0.15, Blob Death Size 0.75, and Rule set to
   "After Effects (numbers and look)". These are After Effects' numbers when it is added.
5. Type the tutorial's numbers: Radius X 0, Radius Y 1000, Birth Rate 100, Gravity -0.8.
6. Play from the start. You should see the words break into white liquid drops that stream
   apart and drift up. The drops are white because the letters under them are white.
7. Open Animation. You should see twelve choices: Explosive, Fractal Explosive, Twirl, Twirly,
   Vortex, Fire, Direction, Direction Normalized, Bi-Directional, Bi-Directional Normalized,
   Jet, Jet Sideways, and our Fixed. Pick Fire: the drops should rise like flame. Pick Jet: a
   narrow stream in the Direction.
8. Open a project saved before this change that uses Mercury. It should look the same as it
   did, and its Rule shows "Before (D-674)".

## What you should see

`verification/B-945 pictures/mercury_sheet.png`: the tutorial on the left, ours on the right,
same numbers, about the same moment. Before this change ours was an empty black frame. Ours
uses a thinner font and wobbles less than the tutorial; the drop shape is our own, since
CycoreFX publishes none.

`verification/B-945 pictures/1_before_radius_0.png` (old rule, nothing drawn) beside
`2_after_effects_radius_0.png` (new rule, drops drawn), both with the middle of the picture
wiped away. The `4_*.png` pictures show each Animation choice.

Wrong if: the frame stays empty in step 6, the Animation list is shorter than twelve, or an old
project looks different.
