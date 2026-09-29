# B-140a: Kaleidoscope, one wedge of the drawing turned round a centre (D-205)

Written on 2026-09-29, before any code. B3 of the second list of After Effects picks, accepted with the rest by your "take everything". An effect that does what After Effects' CC Kaleida does, by a rule of our own.

## What you will see

A new effect in the **Stylize** group, where After Effects keeps CC Kaleida. It takes one wedge of the drawing, like a slice of cake from a centre, and repeats it round that centre until the circle is full, the way the mirrors of a toy kaleidoscope do. With **Mirror**, every other slice is flipped, so the slices meet without a seam and the pattern looks like a snowflake or a flower; with **Repeat**, every slice is the same way round, like a pinwheel. Where the pattern reaches past the drawing's edge it reads the drawing mirrored back at that edge, so a full-frame drawing gives a full-frame pattern with no clear corners. Its card, in After Effects' words where it has them:

- **Center**, two numbers, per cent of the drawing's width and height, -1000 to 1000, 50 and 50 when added: the point the slices turn round.
- **Size**, 10 to 1000 per cent, 100 when added: how large the pattern is. At 100 the first slice is the drawing itself; at 200 the pattern is twice as large, read from half as far out.
- **Segments**, 2 to 32, whole numbers, 6 when added: how many slices make the circle.
- **Mirroring**, **Mirror** or **Repeat**, Mirror when added.
- **Rotation**, -3600 to 3600 degrees, 0 when added: turns the slice that is taken, so keying it spins the pattern.

Every number can be keyed. The layer does not grow, and nothing is scaled in a draft: every setting is a share of the drawing, not a count of pixels.

`kaleidoscope.png` in this folder is worked by the rule itself on a small made-up drawing: as it starts, Repeat, 12 segments, rotation 30, size 50 and 200, the centre moved to the left, and 3 segments.

## How it differs from what we already have

**Mirror** folds the drawing once, along one line. **Motion Tile** repeats the whole drawing side by side. **Polar Coordinates** bends rows into rings. None of them turns a slice round a centre.

## Known limits

- With an odd number of segments under Mirror, the last slice meets the first unflipped, so one seam shows, straight down from the centre when the rotation is 0. It cannot be otherwise: an odd number of slices cannot all be flipped by turns. Use an even number for no seam.
- Under Repeat the slices meet with a hard edge, as a pinwheel's do.
- Far from the centre at a small size, the drawing is read from far away and mirrored many times over, and fine detail is read at a few points, not averaged.
- After Effects' CC Kaleida has Mirroring shapes named Flower, Starlish, Wheel and more, and a Floating Center; ours has a count of segments and two ways of joining them.
- It runs on the processor; the graphics card version is its own later unit.
- It is modelled on After Effects' CC Kaleida and is not claimed to match it.

## How you will check it

The build's test draws the thirteen fixture cases and the seven wrong settings and compares every pixel with the numbers `tools/kaleidoscope_reference.py` worked out, and writes `verification/B-140_kaleidoscope_table.md`, with pictures. A playtest sheet walks you through putting it on a layer and spinning it.
