# B-946 playtest: Wave Warp after After Effects (D-1066)

What changed: Wave Warp now reads After Effects' numbers. Width is half a wave, as in After
Effects, and Speed is waves a second. Noise and Smooth Noise are jagged streaks, like After
Effects', where ours used to be a gentle bend. Projects saved before this keep their old look.

## Steps

1. Start the app and make a new composition, 1280 by 720, 30 frames a second.
2. Add a solid and put 4-Color Gradient on it so it has colour.
3. Add Wave Warp from the Effects window (Distort).
4. Look at Effect Controls. You should see Width 40, Speed 1, and Rule set to
   "After Effects (Width, Speed, noise)".
5. Play. The wave should travel by exactly one wave each second: at 1 second the picture is
   the same as at 0 seconds.
6. Set Wave Type to Smooth Noise, Height 250, Width 263, Direction 45. You should see diagonal
   streaks with sharp zig-zag edges, like the tutorial at 6:40. They should not be one soft
   bend.
7. Set Rule to "Before (D-149)". Smooth Noise turns back into the old gentle bend, and the
   waves get half as long.
8. Open a project saved before this change that uses Wave Warp. It should look as it did, and
   its Rule shows "Before (D-149)".

## What you should see

`verification/B-946 pictures/liquid_sheet.jpg`: the tutorial on the left, ours on the right, at
frame 98 (3:08), using the Wave Warp settings the tutorial shows. They are not a match yet. The
tutorial's warp is jagged and streaky. Ours shows larger, softer swirls with the same colours.
The rest of the tutorial's look (its other effects and the exact noise pattern After Effects
uses) is not copied. Judge this sheet only on whether the colours bend in the same rough way.

`verification/B-946 pictures/1_before_smooth_noise.png` (old rule, a soft bend) beside
`2_after_effects_smooth_noise.png` (new rule, jagged streaks). The same goes for `noise`.

Wrong if: step 5's wave does not come back to where it started after one second, step 6 shows
one soft bend, or an old project looks different.
