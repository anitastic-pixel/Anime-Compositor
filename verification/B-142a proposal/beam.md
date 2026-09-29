# B-142a: Beam, a laser or energy shot drawn from one point to another (D-207)

Written on 2026-09-29, before any code. B5 of the second list of After Effects picks, accepted with the rest by your "take everything". An effect that does what After Effects' Beam does, by a rule of our own.

## What you will see

A new effect in the **Generate** group, where After Effects keeps Beam. It draws a straight beam of light along the line from a start point to an end point: white along its middle, turning to the outside colour at its edges, like a laser, a ray gun shot or an energy blast. Only a stretch of the line is lit, as long as Length says, and Time moves that stretch from the start to the end, so keying Time from 0 to 100 fires the shot across. Its card, in After Effects' words:

- **Start Point** and **End Point**, per cent of the layer across and down, -1000 to 1000, (10, 50) and (90, 50) when added: the line, left to right through the middle.
- **Length**, 0 to 100 per cent of the line, 25 when added: how much of it is lit.
- **Time**, 0 to 100 per cent, 0 when added: where along the line the lit stretch is, from touching the start to touching the end.
- **Starting Thickness** and **Ending Thickness**, 0 to 500 pixels, 8 and 8 when added: the beam widens or narrows between them along the line.
- **Softness**, 0 to 100 per cent, 50 when added: 0 a hard edge, 100 solid only along the middle and fading out to twice the thickness.
- **Inside Color**, white, #ffffff, and **Outside Color**, a laser blue, #3c8cff, when added: the middle's colour and the edges'.
- **Composite On Original**, on when added: the beam over the layer; off, the beam alone and the layer gone, for a layer that only carries the beam.

Every number can be keyed. The ends are round. The layer does not grow: the beam is drawn inside it. In a draft, the two thicknesses are scaled with the picture.

`beam.png` in this folder is worked by the rule itself on a small made-up night: as it starts, time 50 and 100, length 100, thickness 2 to 20, softness 0 and 100, a yellow beam in red slanted across, and composite off.

## How the rule works, in words

Every pixel measures how far it is from the lit stretch of the line and how thick the beam is at the nearest point. Inside the thickness it is covered, and the covering fades over one pixel at softness 0, or over the whole thickness at softness 100. Its colour runs from the inside colour on the line to the outside colour at the edge.

## How it differs from what we already have

**Lightning Bolt** draws a jagged, forking bolt that is redrawn every few frames, and glows; **Beam** is straight and steady, and travels. **Line Width** thickens lines already drawn.

## Known limits

- After Effects' Beam has a 3D Perspective switch, which makes the ends thicken as if the beam came towards you; ours is flat, the thickness changing evenly along the line.
- The beam does not glow past its softness; add Glow after it for a halo.
- A Length of 0 leaves a round dot the beam's thickness, not nothing.
- It runs on the processor; the graphics card version is its own later unit.
- It is modelled on After Effects' Beam and is not claimed to match it.

## How you will check it

The build's test draws the nineteen fixture cases and the ten wrong settings and compares every pixel with the numbers `tools/beam_reference.py` worked out, and writes `verification/B-142_beam_table.md`, with pictures. A playtest sheet walks you through putting it on a layer.
