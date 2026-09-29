# B-146a: Radial Shadow, a shadow cast from a point of light (D-211)

Written on 2026-09-29, before any code. B9 of the second list of After Effects picks, accepted with the rest by your "take everything". An effect that does what After Effects' Radial Shadow does, by a rule of our own.

## What you will see

A new effect in the **Stylize** group, after Drop Shadow, which is where this program keeps its shadows (After Effects keeps both under Perspective, a group this program does not have). Where Drop Shadow slides a copy of the drawing's shape behind it, Radial Shadow casts it from a lamp at a point you place: the shadow spreads away from the light and grows the farther the wall behind is, as a character's shadow does under a street lamp or in front of a torch. Its card, in After Effects' words:

- **Shadow Color**, black when added.
- **Opacity**, 0 to 100, 50 when added.
- **Light Source**, a point in per cent of the drawing's width and height, 50, 0 when added: the top of the middle.
- **Projection Distance**, 0 to 1000, 10 when added: how far the wall lies behind; the shadow is the drawing made bigger about the light by one part in a hundred for each step, so 10 is a tenth bigger and 100 twice the size. At 0 it hides exactly under the drawing.
- **Softening**, 0 to 500 pixels, 0 when added: how blurred the shadow's edge is.
- **Render**, Regular or Glass Edge, Regular when added: Glass Edge takes the shadow's colours from the drawing itself, as light through stained glass.
- **Color Influence**, 0 to 100, 100 when added: with Glass Edge, how much of the drawing's own colour is in the shadow; the rest is the Shadow Color.
- **Shadow Only**, Off or On, Off when added: On leaves the shadow and takes the drawing away.

Shadow Color, Opacity, Light Source, Projection Distance, Softening and Color Influence can be keyed. The layer grows to hold the shadow, by at most its own width across and its own height down on each side; a shadow thrown farther than that is cut there. A draft looks the same, only smaller.

`radial_shadow.png` in this folder is worked by the rule itself on a small made-up cel: the shadow as it starts, the light at the top-left, in the middle and low on the right, soft, blue, Glass Edge at full and half influence, and Shadow Only.

## How the rule works, in words

Each pixel of the shadow looks back toward the light, the projection distance's share of the way, and takes whatever of the drawing it finds there; so the drawing's shape is scaled up about the light. Then it is blurred by the softening, coloured, and laid behind the drawing.

## How it differs from what we already have

**Drop Shadow** moves the shape a set distance in one direction, the same size, as from a far sun; **Radial Shadow** casts it from a near lamp, so it spreads and grows, differently on each side of the light. **Light Rays** paint light streaming out of a picture; Radial Shadow paints the dark behind it.

## Known limits

- After Effects' **Resize Layer** switch is left out: the layer always grows, up to its own size on each side.
- It runs on the processor; the graphics card version is its own later unit.
- It is modelled on After Effects' Radial Shadow and is not claimed to match it.

## How you will check it

The build's test draws the fifteen fixture cases and the ten wrong settings and compares every pixel with the numbers `tools/radial_shadow_reference.py` worked out, and writes `verification/B-146_radial_shadow_table.md`, with pictures. A playtest sheet walks you through putting it on a layer.
