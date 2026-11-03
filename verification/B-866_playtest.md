# B-866 playtest: Glow's A & B colours the right way round, and Glow Based On Alpha Channel (D-986)

What changed:

- With Glow Colors set to A & B Colors, the two Sawtooth choices are now the right way round, as in After Effects. With Sawtooth B>A, the brightest light takes Color A and the faintest takes Color B. Before this change it was the other way round.
- Glow Based On has a third choice, Alpha Channel. With it, every part of the layer that is solid enough glows, whatever its colour.

Start with the picture `verification/B-866 pictures/glow_tutorial_before_after.png`. It shows the neon title from the RIS86giOG-Q tutorial, built one step at a time. Each row has three pictures: After Effects on the left, ours before this change in the middle, and ours now on the right.

- In the middle row (Glow 1), the letters now glow blue with a thin blue rim, as After Effects' do. Before, they were white with a dark blue haze.
- The bottom row is still too hazy, so it is not right yet. See "Not right yet" below.

## Steps

1. New composition, 1920 by 1080. Add a black solid.
2. Add a text layer that says GLOW, white, large and thin.
3. Add Effect > Stylize > Glow to the text. Set:
   - Glow Colors: A & B Colors;
   - Color Looping: Sawtooth B>A;
   - Color A: blue (#1e78ff);
   - Color B: black;
   - Glow Intensity: 2.6;
   - Glow Radius: 10.

   You should see blue light on the letters and a thin blue rim. You should not see a dark, muddy halo.
4. Change Color Looping to Sawtooth A>B. The letters now light up black (no glow) and the faint blue moves to the edges. This is the opposite of step 3.
5. Put Color Looping back to Sawtooth B>A. Set Glow Based On to Alpha Channel and Glow Threshold to 60. The whole of every letter glows, the same as before or a little more. Now set the text colour to dark grey: it still glows, because Alpha Channel looks at how solid the letters are, not at their colour. With Glow Based On set to Bright parts instead, the dark grey letters would not glow.
6. Ctrl+Z steps back through each change.

## Not right yet

When a second Glow with radius 153 is added (bottom row of the picture), ours still fills the space around the letters with blue haze, and After Effects' does not. The likely cause is that our glow treats the soft blue drop shadows under the letters as bright. That needs its own decision, so this change does not fix it.

The glow is drawn on the processor (CPU) when Alpha Channel or A & B Colors is on, as A & B Colors already was.

The computer's checks are in four tables:

- `verification/B-625_glow_strength_table.md` (50 of 50);
- `verification/B-626_glow_controls_table.md`;
- `verification/D-337_glow_display_table.md`;
- `verification/B-866_private_app_table.md` (5 of 5; the choice in the real app).
