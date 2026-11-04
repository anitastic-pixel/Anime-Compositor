# B-958 playtest: Noise, Clip Result Values (D-1078)

After Effects' Noise has a Clip Result Values box. On, a grain that would pass white or black stops there. Off, it wraps round: grain on white turns into dark dots, and grain on black into light ones. This is the look the tutorial uses (lHQvi94dpvM about 4:20).

What to look at first: `verification/B-958 pictures/clip.png`. It is a black-to-white ramp with strong Noise: Clip On above, Clip Off below. The middle of the two strips is the same; at the white end the lower strip has dark speckle, and at the black end it has light speckle.

## In the app

1. Make a white solid. Add Effect > Noise & Grain > Noise. Set Amount of Noise to 40.
   - Expected: Effect Controls shows a new row, Clip Result Values, set to On. The grain on white is faint: it can only darken.
2. Set Clip Result Values to Off.
   - Expected: the white fills with dark dots, as if the grain wrapped round from white to black.
3. Change the solid to black.
   - Expected: with Off, light dots on black; with On, the grain only lightens.
4. Change the solid to mid grey and switch between On and Off.
   - Expected: no difference at all at Amount 40 or less on mid grey.
5. Save the project with Clip On, then open an older project that has Noise in it.
   - Expected: both look exactly as before. Clip On is never written to the file, so an older file and a new file with Clip On are the same.
6. Set Off, save, close and open the project again.
   - Expected: Clip Result Values is still Off and the dots are still there.

Judge it correct if steps 1 to 6 match. Tell me which step looked wrong if any did.
