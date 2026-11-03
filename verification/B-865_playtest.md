# B-865 playtest: Lens Flare looks like After Effects' (D-985)

What changed: the three Lens Flare lenses (50-300mm Zoom, 35mm Prime, 105mm Prime) were measured again from After Effects' own flares in three tutorials. They are now soft blooms with faint ghosts and rings, and no rays.

Look first at `verification/B-865 pictures/lens_flare_tutorial_before_after.png`: each row is one lens, After Effects on the left, ours before in the middle, ours now on the right. The right column should look like the left one; the middle one should not.

## Steps

1. New composition, 1920 by 1080. Add a black solid. Add Effect > Generate > Lens Flare to it.
2. Set Flare Center to 40, 40 per cent and Lens Type to 105mm Prime. You should see a wide, soft white-blue glow with no rays, a few small bright dots on the line through the middle of the picture, and a faint large round shape down to the right.
3. Set Lens Type to 50-300mm Zoom. The centre turns white-hot inside a soft red glow with a thin red ring round it, and a row of faint coloured circles (blue, orange, yellow-green) runs through the middle, ending in a faint large ring.
4. Set Lens Type to 35mm Prime. A pink-red glow with a faint red ring, a yellow-green circle and a faint grey ring.
5. Drag Flare Center about: the ghosts slide along the line through the middle, as before. Set Flare Brightness to 200: everything brighter. Set Blend With Original to 50: half as strong.

## What you should not see

- Long straight rays or a star shape on any lens.
- A strong blue or orange halo round the centre.
- Bright, solid coloured discs much stronger than the glow.

Close, not pixel for pixel: over a picture rather than black, After Effects mixes the colours a little differently. The computer's checks are in `verification/D-426_lens_flare_table.md` (146 of 146) and `verification/D-545_lens_dirt_table.md` (463 of 463).
