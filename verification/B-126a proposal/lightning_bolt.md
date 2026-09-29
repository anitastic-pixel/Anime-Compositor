# B-126a: Lightning Bolt, a forking bolt of light (D-190)

Written on 2026-09-28, before any code. The first of the After Effects picks, A1, accepted with the rest by your "take everything". It does what After Effects' Lightning and Advanced Lightning do, by a rule of our own; nothing is ported.

## What you will see

A new effect, **Lightning Bolt**, under **Light & Glow**. It draws a jagged bolt of light from one point of the layer to another: a white-hot core in a coloured glow, with thinner forks off it, and a new bolt every few frames. Its card has these rows:

- **Start** and **End**, two numbers each, per cent across and down the layer, (40, 0) and (60, 100) when added: where the bolt begins and where it lands. It always lands exactly there.
- **Jaggedness**, 0 to 100 %, 40: how far each kink is pushed aside. 0 is a straight line.
- **Detail**, 1 to 8, 6: how many times the line is halved. Each step doubles the kinks, finer and finer.
- **Branches**, 0 to 100 %, 30: the share of kinks that sprout a fork. Forks sprout forks, three deep at most.
- **Width**, 0 to 100 pixels, 3: how thick the core is where the bolt starts. The bolt keeps that width to its end; each fork starts at half its parent's and thins to nothing at its tip.
- **Glow**, 0 to 500 pixels, 24: how far the glow reaches round the core, less round the thin forks.
- **Colour**, white, and **Glow colour**, a blue #6e8cff, when added.
- **Opacity**, 0 to 100 %, 100.
- **Hold**, 1 to 100 frames, 2: how many frames each bolt stays before a new one strikes.
- **Seed**, 0 to 100000, 0: another seed, other bolts.

Every number can be keyed; keying Start and End moves the bolt, keying Opacity makes it flash. A draft scales Width and Glow with the picture, so a draft looks like the full picture, smaller.

The light is added: it brightens what is under it, and where the layer is clear it paints the bolt. The bolt is drawn inside the layer and never makes it bigger, as After Effects draws it; for a bolt across the whole frame, put it on a solid or an adjustment layer the size of the frame. On an adjustment layer it lights the picture below.

## How it differs from what we already have

Nothing we have draws a line of its own. **Light Rays**, **Glow** and **Cross Glare** spread the light a drawing already has; Lightning Bolt makes new light where there was none.

`lightning_bolt.png` in this folder is worked by the reference rule itself, at half of 1920 by 1080, with Width and Glow halved as a half-size draft does: a night scene with clouds and hills; the bolt as added, with Start and End moved to the cloud and the hill, at frame 0; at frame 2, a new bolt between the same two points; with Branches 80 and Detail 8; with Jaggedness 20, straighter; and a wide red and gold bolt, Width 8 and Glow 60.

## Known limits

- Each bolt appears whole on its first frame. After Effects' Advanced Lightning can draw a bolt growing from its start; that is left for later. Key Opacity for a flash.
- A bolt past the layer's edge is cut there, as above.
- The cost grows with the glow's reach and the number of pieces: Detail 8 with Branches 100 and a glow of hundreds of pixels is the slowest it gets. The playtest sheet will time it.
- It runs on the processor. The graphics card learns it with the other new effects, in a card unit of its own.

## How you will check it

The build's test draws the thirty-two fixture cases and compares every pixel with the numbers `tools/lightning_bolt_reference.py` worked out, and writes `verification/B-126_lightning_bolt_table.md`, with pictures. A playtest sheet walks you through adding it, playing it, and trying each setting.
