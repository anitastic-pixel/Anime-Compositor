# B-139a: Snowfall, soft flakes falling in three depths (D-204)

Written on 2026-09-29, before any code. B2 of the second list of After Effects picks, accepted with the rest by your "take everything". An effect that does what After Effects' CC Snowfall does, by a rule of our own.

## What you will see

A new effect in the **Generate** group, after Rain. Put it on a solid, or on any drawing, and snow falls across it: soft round flakes, some near and large, some far and small, all drifting down, swaying a little from side to side, and carried by the wind. Like Rain, it draws only where the layer shows, so snow on a solid covers the frame and snow on a drawing stays inside the drawing. Its card, in After Effects' words where it has them:

- **Flakes**, 0 to 100 per cent, 50 when added: how many of the places a flake could be hold one.
- **Spacing**, 2 to 1000 pixels, 32 when added: how far apart the near flakes' places are.
- **Size**, 0 to 100 pixels, 6 when added: the widest a near flake is; each flake is its own width between half that and the whole of it.
- **Scene Depth**, 0 to 100, 50 when added: the snow lies in three layers of depth, near, middle and far. At 0 all three are near; at 100 the far one is a third the near one's size, so far flakes are smaller, closer together and slower.
- **Speed**, 0 to 1000 pixels a frame, 2 when added: how fast the near flakes fall.
- **Wind**, -1000 to 1000 pixels a frame, 0.5 when added: how fast they drift right, or left when negative.
- **Wiggle Amount**, 0 to 100 pixels, 3 when added, and **Wiggle Period**, 1 to 1000 frames, 48 when added: each flake sways from side to side by up to that much, once in that many frames, each at its own point in the sway.
- **Color**, white when added; **Opacity**, 0 to 100, 100 when added; **Random Seed**, 0 to 100000, whole numbers, 0 when added: another seed is another snowfall.

Every number setting can be keyed. The flakes are fixed to the drawing, so the snow moves with the layer, and it falls with the composition's frame, so it plays the same every time.

`snowfall.png` in this folder is worked by the rule itself: the snow as it starts at frame 0 and frame 12, with more and bigger flakes, with scene depth 0 and 100, and fine snow.

## How it differs from what we already have

**Rain** draws streaks that fall fast along a slant. Snowfall draws round flakes, in depth, swaying. **Particle** effects are not in this program yet; Snowfall is a pattern, not particles, as After Effects' own CC Snowfall is.

## Known limits

- Three layers of depth, not a continuous one.
- A flake is a soft dot; it does not tumble, blur with its motion or catch the light.
- After Effects' CC Snowfall has a Flakes count, Variation settings, a Background Illumination, an Extra and a Composite With Original switch; ours has a density in per cent and no background lighting. Its snow can also fall off the layer's edge; ours stays inside the layer's covering.
- It runs on the processor; the graphics card version is its own later unit.
- It is modelled on After Effects' CC Snowfall and is not claimed to match it.

## How you will check it

The build's test draws the twenty fixture cases and the nine wrong settings and compares every pixel with the numbers `tools/snowfall_reference.py` worked out, and writes `verification/B-139_snowfall_table.md`, with pictures. A playtest sheet walks you through putting snow on a solid and playing it.
