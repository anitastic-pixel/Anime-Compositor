# B-145a: Optics Compensation, a lens's curve put in or taken out (D-210)

Written on 2026-09-29, before any code. B8 of the second list of After Effects picks, accepted with the rest by your "take everything". An effect that does what After Effects' Optics Compensation does, by a rule of our own.

## What you will see

A new effect in the **Distort** group, where After Effects keeps Optics Compensation, after Polar Coordinates. It bends the layer as a wide lens would: the middle keeps its size and the picture is squeezed in more and more toward the edges, so straight lines bow outward round the middle, a fisheye look for a peephole, a security camera, a dream or a big impact. With **Reverse Lens Distortion** on it does the opposite, stretching the edges out, which takes out a fisheye that is already there, or gives a stretched, looming look. Its card, in After Effects' words:

- **Field of View**, 0 to 180 degrees, 0 when added: how wide the lens is; 0 leaves the layer as it is, and the wider, the stronger the bend.
- **Reverse Lens Distortion**, Off or On, Off when added.
- **FOV Orientation**, Horizontal, Vertical or Diagonal, Horizontal when added: whether the field of view spans the drawing's width, its height or its diagonal. Vertical bends a wide drawing harder; Diagonal more gently.
- **View Center**, the lens's middle, per cent of the drawing's width and height, 50, 50 when added.

Field of View and View Center can be keyed. The layer does not grow: what is squeezed in leaves empty space round the edges, and what is stretched out past the edges is lost. A draft looks the same, only smaller.

`optics_compensation.png` in this folder is worked by the rule itself on a small made-up cel with a ruled card, so the bend shows: fields of view 60, 90, 150 and 180, reverse on, vertical and diagonal, the centre moved left, and the fisheye put in and then taken out again, which gives back the drawing, a little softer.

## How the rule works, in words

The lens sees the field of view across the drawing's width (or height, or diagonal). A pixel's distance from the centre is turned into an angle through the lens; with Reverse off it reads the picture from where a flat camera would have put that angle, which is farther out, more so the farther out it is; past a quarter turn nothing is there, and it is left empty. With Reverse on it reads from the other way round, nearer in. The two are exact opposites.

## How it differs from what we already have

**Bulge** swells or pinches a round patch and leaves the rest alone; **Optics Compensation** bends the whole layer, as a lens does. **Polar Coordinates** turns the picture into a circle; Optics Compensation keeps it the same picture, bowed.

## Known limits

- After Effects' **Optimal Pixels** and **Resize** are left out: the layer keeps its size, so a stretched picture loses its edges and a squeezed one leaves empty space.
- It runs on the processor; the graphics card version is its own later unit.
- It is modelled on After Effects' Optics Compensation and is not claimed to match it.

## How you will check it

The build's test draws the fourteen fixture cases and the six wrong settings and compares every pixel with the numbers `tools/optics_compensation_reference.py` worked out, and writes `verification/B-145_optics_compensation_table.md`, with pictures. A playtest sheet walks you through putting it on a layer.
