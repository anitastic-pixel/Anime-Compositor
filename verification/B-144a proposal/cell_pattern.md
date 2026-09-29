# B-144a: Cell Pattern, bubbles, crystals and plates over a layer (D-209)

Written on 2026-09-29, before any code. B7 of the second list of After Effects picks, accepted with the rest by your "take everything". An effect that does what After Effects' Cell Pattern does, by a rule of our own.

## What you will see

A new effect in the **Generate** group, where After Effects keeps Cell Pattern. It covers the layer with a pattern of cells, in two colours, black and white when added: round bubbles, faceted crystals, flat plates with dark seams, or flat plates each its own grey, like stained glass. It is for textures, backgrounds, magic surfaces, scales, cracked ground and the like, and it paints only where the layer shows: an empty part stays empty and a soft edge stays soft. Its card, in After Effects' words where it has them:

- **Cell Pattern**, Bubbles, Crystals, Plates or Static Plates, Bubbles when added.
- **Invert**, Off or On, Off when added: swaps light and dark.
- **Contrast**, 0 to 1000, 100 when added: above 100 pushes the greys out to black and white, 0 makes every pixel the middle grey.
- **Disperse**, 0 to 1.5, 1 when added: 0 lines the cells up in a square grid; higher scatters them.
- **Size**, 1 to 1000 pixels a cell, 60 when added.
- **Evolution**, degrees, 0 when added: turning it moves each cell's middle round a small circle, so keying it makes the cells writhe; a full turn brings them back.
- **Random Seed**, 0 to 100000, 0 when added: another seed, another pattern.
- **Dark Colour** and **Light Colour**, black and white when added, **Opacity**, 0 to 100, and **Blend**, Normal, Multiply, Screen or Add: how the pattern is laid on, as Fractal Noise's are.

Every number can be keyed. The layer does not grow, and a draft looks the same, only smaller.

`cell_pattern.png` in this folder is worked by the rule itself on a small made-up cel: the four patterns, invert, contrast 300, disperse 0, evolution 90, and navy seams with gold bubbles laid on by multiply.

## How the rule works, in words

The layer is cut into square cells of the size, and each cell holds one point, placed at random inside it by the seed and the disperse. Every pixel finds the nearest point and the second nearest. Near its nearest point it is in the middle of a cell; where the two are about as near it is on a seam between two cells. Bubbles shade that like a ball, Crystals like a cone, Plates stay flat until close to the seam, and Static Plates give each cell one grey of its own. The grey is turned into a colour between the two colours and laid on the way Fractal Noise lays on its clouds.

## How it differs from what we already have

**Fractal Noise** makes soft clouds with no edges; **Cell Pattern** makes cells with seams. **Mosaic** turns the drawing into square blocks of its own colours; Cell Pattern draws a pattern of its own over it.

## Known limits

- After Effects has twelve patterns; this has four: Bubbles, Crystals, Plates and Static Plates. Crystallize, Pillow, Mixed Crystals, Tubular and the HQ versions are left out.
- After Effects' Overflow, Offset, Tiling Options and Cycle Evolution are left out: the pattern is always clipped at black and white, it is fixed to the drawing, and Evolution loops by itself every full turn.
- It runs on the processor; the graphics card version is its own later unit.
- It is modelled on After Effects' Cell Pattern and is not claimed to match it.

## How you will check it

The build's test draws the twenty-two fixture cases and the eleven wrong settings and compares every pixel with the numbers `tools/cell_pattern_reference.py` worked out, and writes `verification/B-144_cell_pattern_table.md`, with pictures. A playtest sheet walks you through putting it on a layer.
