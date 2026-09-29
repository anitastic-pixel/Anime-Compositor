# B-141a: Roughen Edges, a layer's edges eaten into by noise (D-206)

Written on 2026-09-29, before any code. B4 of the second list of After Effects picks, accepted with the rest by your "take everything". An effect that does what After Effects' Roughen Edges does, by a rule of our own.

## What you will see

A new effect in the **Stylize** group, where After Effects keeps Roughen Edges. It eats into the edges of whatever the layer shows, by a rough amount that changes along the edge, so a clean cut-out looks torn, burnt, eroded or hand-cut. Nothing is added outside the old edge: the layer only loses. With **Roughen Color**, a band just inside the new edge takes the edge colour, like scorched or rusted paper. Its card, in After Effects' words:

- **Edge Type**, **Roughen** or **Roughen Color**, Roughen when added.
- **Edge Color**, a rust brown, #8a3c14, when added: the band's colour under Roughen Color.
- **Border**, 0 to 500 pixels, 8 when added: how deep the bites go at most.
- **Scale**, 1 to 1000 pixels, 10 when added: how wide the bites are.
- **Complexity**, 1 to 10, whole numbers, 3 when added: how much fine detail rides on the broad bites.
- **Evolution**, -100000 to 100000 degrees, 0 when added: turning it changes the bites smoothly.
- **Evolution Speed**, -360 to 360 degrees a frame, 0 when added: our own, as on Turbulent Displace and Fractal Noise, so the edge can crawl without keys.
- **Random Seed**, 0 to 100000, whole numbers, 0 when added: another seed, other bites.

Every number can be keyed. The layer does not grow. In a draft, Border and Scale are scaled with the picture.

`roughen_edges.png` in this folder is worked by the rule itself on a small made-up drawing, a red card, a blue disc and a yellow bar: as it starts, border 16, complexity 6, scale 40, seed 3, and Roughen Color in rust and in blue.

## How the rule works, in words

At every pixel the noise sets a depth, from nothing up to the border. The pixel keeps only as much covering as the drawing has that far away across and down, either way, so near an edge it is eaten away and deep inside it is untouched. Under Roughen Color the band is the part that would go at twice the depth.

## How it differs from what we already have

**Turbulent Displace** moves the whole picture about, inside and out; **Simple Choker** pulls every edge in evenly. Neither of them leaves the inside alone and makes the edge ragged.

## Known limits

- The depth is read across and down only: a slanted edge is bitten a little less deep than an upright one.
- A narrow gap in the drawing, thinner than the depth, can be jumped: a sliver beside it may survive.
- Specks can break off near the edge, as they do in After Effects.
- After Effects' Roughen Edges has Edge Sharpness, Fractal Influence, Stretch Width or Height, Offset, a cycle for Evolution and the Cut, Spiky, Rusty and Photocopy edge types; ours has Roughen and Roughen Color.
- It runs on the processor; the graphics card version is its own later unit.
- It is modelled on After Effects' Roughen Edges and is not claimed to match it.

## How you will check it

The build's test draws the fifteen fixture cases and the nine wrong settings and compares every pixel with the numbers `tools/roughen_edges_reference.py` worked out, and writes `verification/B-141_roughen_edges_table.md`, with pictures. A playtest sheet walks you through putting it on a layer.
