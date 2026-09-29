# B-128a: Displacement Map, a layer moved by another layer's colours (D-193)

Written on 2026-09-29, before any code. The third of the After Effects picks, A3, accepted with the rest by your "take everything". It does what After Effects' Displacement Map does, by a rule of our own, and it is the second effect to read another layer of the composition as a map (D-189), after Compound Blur.

## What you will see

A new effect, **Displacement Map**, under **Distort**, after Ripple. It moves each pixel of the layer across and up or down by how bright the map is there: white moves the picture the most one way, black the most the other way, mid grey not at all. Its card has these rows, in After Effects' order:

- **Displacement Map Layer**, a list of this composition's layers, **None** when added. The map layer can be switched off; it still works as a map.
- **Use For Horizontal Displacement**, **Red** when added: which part of the map's colour moves pixels across. One of Red, Green, Blue, Alpha, Luminance, Hue, Lightness, Saturation, Full or Off.
- **Max Horizontal Displacement**, -1000 to 1000 pixels, 5 when added. It can be keyed. A negative number turns the direction round.
- **Use For Vertical Displacement**, **Green** when added, the same list, for up and down.
- **Max Vertical Displacement**, as the horizontal one.
- **Displacement Map Behavior**, **Stretch Map to Fit** when added, or **Center Map** or **Tile Map**, as Compound Blur's If Sizes Differ.
- **Wrap Pixels Around**, Off or On: On brings what leaves one edge back in at the other, for a pattern that repeats.

White in the chosen part moves the picture left (across) and up (down): each pixel shows what lies the maximum to its right and below. Black moves it right and down. A part of the map that is clear moves nothing, so a map drawn only where you want the picture moved leaves the rest alone. **Full** moves every pixel the maximum whatever the map, and **Off** moves nothing that way.

The map is the other layer's own picture, with its masks and effects, but not where it is moved to: it lies on this layer corner to corner. The layer does not grow. A draft moves pixels less with the smaller picture, so it looks like the full picture, smaller. On an adjustment layer it moves everything below it.

`displacement_map.png` in this folder is worked by the rule itself, at a quarter of 1920 by 1080: the street from Compound Blur; a map of red bands and green waves; the street moved by it, at most 3, as if seen through rippling water; the same at 10 with Wrap on; a second map, a ball whose colours point back to its middle, clear outside it; and the street moved by that one, a magnifying glass, the rest untouched.

## Two things of After Effects' left out

- **Half** in the channel lists moves nothing, exactly as Off does, so it is left out.
- **Expand Output**: the layer keeps its size, as Twirl and Bulge do. What moves in from beyond the layer's edge is clear, unless Wrap Pixels Around is on.

## The layer list, copies and circles

All as Compound Blur's (D-191): a layer missing from the composition is kept by name with a warning, the effect doing nothing until you pick another or undo the delete; a layer can name itself; two layers may not read each other, even through two different effects, one a Displacement Map and the other a Compound Blur; Duplicate and duplicating a composition follow the copies; a preset keeps no map.

## How it differs from what we already have

**Twirl**, **Bulge**, **Ripple** and **Turbulent Displace** move pixels by a rule: a swirl, a dome, rings, noise. Displacement Map moves them by a picture you choose, so any layer can be the rule: a hand-drawn heat haze, a glass shape, water reflections, text that makes a dent in the picture, a moving shape that pushes the picture as it passes.

## Known limits

- A map with hard edges tears the picture at them, as After Effects' does; soften the map for a smooth bend.
- It runs on the processor. The graphics card learns it in a card unit of its own.

## How you will check it

The build's test draws the twenty-three fixture cases and the eight wrong settings and compares every pixel with the numbers `tools/displacement_map_reference.py` worked out, opens the two circle files, and writes `verification/B-128_displacement_map_table.md`, with pictures. A playtest sheet walks you through adding it, picking a map and trying each setting.
