# B-134a: Light Sweep, a shine that runs across a drawing (D-199)

Written on 2026-09-29, before any code. The ninth of the After Effects picks, A9, accepted with the rest by your "take everything". It does what After Effects' CC Light Sweep does, by a rule of our own.

## What you will see

A new effect, **Light Sweep**, in the **Generate** group. It lays a band of light across the drawing along a line, brightest on the line and fading out to both sides, and lights the drawing's outer edges more than its middle. Key the band's centre to move and the shine runs across a sword, a badge, a logo, a car or a window pane. Only what is drawn is lit: the empty parts round the drawing stay empty. Its card, in After Effects' words:

- **Center**, a point: where the band's middle line passes, across and down in per cent of the drawing's own width and height; (50, 50) when added. Keyable; this is the one to key for the sweep.
- **Direction**, in degrees clockwise from straight up: the way the band's line runs. -30 when added, leaning a little left at the top. Keyable.
- **Shape**: **Linear**, fading evenly to its sides; **Smooth**, fading softly, the default; or **Sharp**, a hard-edged bar.
- **Width**, in pixels, 0 to 10000: how wide the band is; 50 when added. 0 is no band. Keyable.
- **Sweep Intensity**, 0 to 100: how bright the band is over the drawing; 50 when added. Keyable.
- **Edge Intensity**, 0 to 100: how much brighter the band makes the drawing's outer edges; 100 when added. Keyable.
- **Edge Thickness**, 1 to 50 pixels: how deep into the drawing an edge counts; 1 when added. Keyable.
- **Light Color**: white when added.
- **Light Reception**: **Add**, the light added on top, the default; **Composite**, the band painting the drawing toward the light's colour, never brighter than it; or **Cutout**, the drawing cut away and only the light left, to use on its own or as a matte.

The card's picture is a box standing for the drawing with the band's centre as a point to drag.

`light_sweep.png` in this folder is worked by the rule itself: a badge before; as it starts; a sharp band keyed to one side; the edges alone; the band composited in orange; and the cutout.

## How it differs from what we already have

**Rim Light** lights the edge that faces a light, all along the drawing. **Kira-kira** scatters sparkles. **Gradient** lays a wash of colour over the whole layer. Light Sweep is one band that crosses the drawing and can be moved across it.

## Known limits

- An edge is where the drawing meets empty space. A line drawn inside the drawing, the star's outline in the picture, is not an edge.
- Edge Thickness counts whole pixels: 2.7 is 2.
- There is no handle for the band on the picture in the viewer yet: move it by the card's point or its numbers.
- It runs on the processor; the graphics card version is its own later unit.
- It is modelled on After Effects' CC Light Sweep and is not claimed to match it.

## How you will check it

The build's test draws the nineteen fixture cases and the seven wrong settings and compares every pixel with the numbers `tools/light_sweep_reference.py` worked out, and writes `verification/B-134_light_sweep_table.md`, with pictures. A playtest sheet walks you through sweeping a shine across a badge.
