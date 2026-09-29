# B-143a: 4-Color Gradient, four colours running into each other across a layer (D-208)

Written on 2026-09-29, before any code. B6 of the second list of After Effects picks, accepted with the rest by your "take everything". An effect that does what After Effects' 4-Color Gradient does, by a rule of our own.

## What you will see

A new effect in the **Generate** group, where After Effects keeps 4-Color Gradient. It pins four colours to four points of the layer; each colour is strongest at its own point and runs smoothly into the others, for a sky, a glow, a colour wash over a background or a shimmer across a cel. It paints only where the layer shows: an empty part of the layer stays empty, and a soft edge stays soft. Its card, in After Effects' words:

- **Point 1** to **Point 4**, per cent of the layer across and down, -1000 to 1000, at the four corners, (10, 10), (90, 10), (10, 90) and (90, 90), when added.
- **Color 1** to **Color 4**, yellow, green, magenta and blue when added, as After Effects starts them.
- **Blend**, 1 to 1000, 100 when added: how far each colour reaches into the others. 1 gives four hard-edged patches; 1000 mixes the four nearly evenly everywhere.
- **Opacity**, 0 to 100 per cent, 100 when added.
- **Blending Mode**, Normal, Multiply, Screen or Add, Normal when added: Normal paints the colours over the layer; Multiply tints and darkens it, keeping its lines; Screen and Add lighten it.

Every number and point can be keyed. The layer does not grow, and a draft looks the same, only smaller.

`four_color_gradient.png` in this folder is worked by the rule itself on a small made-up cel: as it starts, blend 1 and 1000, opacity 50, multiply, screen, add at 30, point 1 moved to the middle, and a sunset laid on by multiply.

## How the rule works, in words

Every pixel measures how far it is from each of the four points. The nearer a point, the more its colour counts, and Blend says how quickly that falls off: at 100, a point twice as far counts a quarter as much. The pixel's colour is the four colours mixed by those shares, and it is laid onto the pixel by the blending mode at the opacity, the way Gradient lays its colour on.

## How it differs from what we already have

**Gradient** runs between two colours along a line or in rings; **4-Color Gradient** mixes four colours across the whole layer. **Gradient Map** recolours by brightness, not by place.

## Known limits

- After Effects' 4-Color Gradient has Jitter, a fine noise against banding; our pictures are worked in floating point and do not band, so it is left out.
- Of After Effects' many blending modes, only Normal, Multiply, Screen and Add, as Gradient has.
- It runs on the processor; the graphics card version is its own later unit.
- It is modelled on After Effects' 4-Color Gradient and is not claimed to match it.

## How you will check it

The build's test draws the seventeen fixture cases and the nine wrong settings and compares every pixel with the numbers `tools/four_color_gradient_reference.py` worked out, and writes `verification/B-143_four_color_gradient_table.md`, with pictures. A playtest sheet walks you through putting it on a layer.
