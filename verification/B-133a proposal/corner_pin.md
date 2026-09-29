# B-133a: Corner Pin, a drawing pinned by its four corners (D-198)

Written on 2026-09-29, before any code. The eighth of the After Effects picks, A8, accepted with the rest by your "take everything". It does what After Effects' Corner Pin does, by a rule of our own.

## What you will see

A new effect, **Corner Pin**, in the **Distort** group after Motion Tile. It takes the drawing's four corners and pins each one somewhere new, and the drawing is stretched between them in perspective: a poster put onto a wall that runs away from the camera, a picture on a phone or television screen, a sign on the side of a bus, a card lying on a table. The part further away is drawn smaller, as a camera would see it. Its card, in After Effects' words:

- **Upper Left**, **Upper Right**, **Lower Left** and **Lower Right**, each a point: across and down, in per cent of the drawing's own width and height. When added they are at the drawing's own corners, (0, 0), (100, 0), (0, 100) and (100, 100), and the drawing is as it was. Each runs from -400 to 500 either way, so a corner can be pulled well outside the drawing. They can be keyed, so a sign can swing or a screen follow a moving phone.

The card's picture shows the four corners as points in the drawing's box, joined into the shape the drawing is pinned to; drag a point to move its corner.

When a corner is pulled outside the drawing, the layer grows so that nothing is cut off. If the four corners make no shape that can be drawn, crossed like a bow tie, one bent in past the others, or three in a line, nothing is drawn until they are moved back; the drawing is not left half-drawn.

`corner_pin.png` in this folder is worked by the rule itself: a poster the size of the frame; the poster pinned onto a wall that runs away to the left, its far side smaller; a keystone, its top narrowed; and its lower right corner pulled in.

## How it differs from what we already have

The layer's own **Scale** and **Rotation** keep a rectangle a rectangle. Corner Pin moves each corner on its own, so the drawing can lean away in perspective. **Mirror** and **Offset** turn over or slide the picture inside the layer; Corner Pin reshapes it.

## Known limits

- A drawing shrunk a long way is read at a few points, not averaged: fine lines can break up. Pin a drawing that is not much larger than it will be shown.
- A glow, shadow or tile added before the Corner Pin is carried along, but what of it falls far outside the four corners' box may be cut. Put those effects after the Corner Pin.
- There are no handles on the picture in the viewer yet: move the corners by the card's points or its numbers.
- It runs on the processor; the graphics card version is its own later unit.

## How you will check it

The build's test draws the fifteen fixture cases and the three wrong settings and compares every pixel with the numbers `tools/corner_pin_reference.py` worked out, and writes `verification/B-133_corner_pin_table.md`, with pictures. A playtest sheet walks you through pinning a poster onto a wall.
