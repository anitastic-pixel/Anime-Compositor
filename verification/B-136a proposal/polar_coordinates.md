# B-136a: Polar Coordinates, a drawing bent round its middle (D-201)

Written on 2026-09-29, before any code. The eleventh of the After Effects picks, A11, accepted with the rest by your "take everything". It does what After Effects' Polar Coordinates does, by a rule of our own.

## What you will see

A new effect, **Polar Coordinates**, in the **Distort** group after Corner Pin. It bends the drawing round its middle: the drawing's top row is squeezed into the middle, its foot is laid round the rim, and its columns become spokes, so upright speed lines become focus lines rushing into the middle, a strip of flames becomes a ring of fire and a row of houses a little world. Or it does the reverse and unrolls the drawing, its rings into rows and its spokes into columns, so a target becomes stripes and a ring of anything becomes a band. Its card, in After Effects' words:

- **Interpolation**, 0 to 100: how far to go; 100 when added, all the way, and 0 the drawing untouched. Keyable, so the drawing can be bent round over a few frames. After Effects adds it at 0, where it shows nothing until the number is raised; ours shows the effect at once.
- **Type of Conversion**: **Rect to Polar**, bending the drawing round its middle, the default; or **Polar to Rect**, unrolling it. Each undoes the other.

The layer does not grow: the bent drawing fills the drawing's own rectangle.

`polar_coordinates.png` in this folder is worked by the rule itself: speed lines, bent into focus lines, half way there, and bent back again; and a target, unrolled into bands.

## How it differs from what we already have

**Twirl** turns the middle of the drawing round; **Bulge** swells it. Neither changes rows into rings. **Speed Lines** already draws focus lines of its own; Polar Coordinates bends whatever is drawn, so hand-drawn streaks, a pattern or a strip of scenery can be taken round.

## Known limits

- On a drawing that is not square the rings are ellipses touching the drawing's sides, not circles, as in After Effects. Put the effect on a square layer for round rings.
- Beyond that ellipse, in the drawing's corners, Rect to Polar reads below the drawing's foot, so the corners are clear unless an effect before it has grown the layer.
- Between 0 and 100 a tear runs straight up from the middle, where the drawing's left and right edges meet going round.
- Near the middle the drawing's top row is squeezed into a few pixels; far out its foot is stretched and soft. Detail is read at a few points, not averaged.
- It runs on the processor; the graphics card version is its own later unit.
- It is modelled on After Effects' Polar Coordinates and is not claimed to match it.

## How you will check it

The build's test draws the eleven fixture cases and the four wrong settings and compares every pixel with the numbers `tools/polar_coordinates_reference.py` worked out, and writes `verification/B-136_polar_coordinates_table.md`, with pictures. A playtest sheet walks you through bending speed lines into focus lines.
