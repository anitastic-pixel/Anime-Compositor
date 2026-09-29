# B-129a: Gradient Wipe, a layer wiped away in the order of another layer's brightness (D-194)

Written on 2026-09-29, before any code. The fourth of the After Effects picks, A4, accepted with the rest by your "take everything". It does what After Effects' Gradient Wipe does, by a rule of our own, and it is the third effect to read another layer of the composition as a map (D-189), after Compound Blur and Displacement Map.

## What you will see

A new effect, **Gradient Wipe**, under **Transition**, after Linear Wipe. As its completion goes up, the layer is wiped away where the map is darkest first, then where it is lighter, until at 100% nothing is left: a map black in the middle and white at the edges opens a hole from the middle, a map of clouds eats the picture away in cloud shapes. Its card has these rows, in After Effects' words and order:

- **Transition Completion**, 0 to 100%, 0 when added: how much is wiped away. It can be keyed, and that is how the transition is made.
- **Transition Softness**, 0 to 100%, 0 when added: 0 is a hard edge; more makes the edge fade over more of the map's greys. It can be keyed.
- **Gradient Layer**, a list of this composition's layers, **None** when added. The map layer can be switched off; it still works as a map.
- **Gradient Placement**, **Stretch Gradient to Fit** when added, or **Center Gradient** or **Tile Gradient**, as Compound Blur's If Sizes Differ.
- **Invert Gradient**, Off or On: On wipes the lightest parts first.

The map's brightness is read as Compound Blur reads it, so a clear part of the map counts as black and goes first. At 0% the layer is exactly as it was, and at 100% it is all gone, whatever the softness. With no layer named, nothing is wiped, whatever the completion. The map is the other layer's own picture, with its masks and effects, but not where it is moved to: it lies on this layer corner to corner. The layer does not grow. On an adjustment layer it wipes away everything below it.

`gradient_wipe.png` in this folder is worked by the rule itself, at a quarter of 1920 by 1080, over a plum background that stands for the shot beneath: the street; a map black round a point and white further out; the street wiped by it at 30% with a hard edge, a hole opening from the point; at 60% with softness 25, the hole larger and its edge soft; at 30% inverted, closing in from the outside; a second map of soft clouds; and the street wiped by it at 40% and 75%, softness 10.

## The layer list, copies and circles

All as Compound Blur's (D-191): a layer missing from the composition is kept by name with a warning, the effect doing nothing until you pick another or undo the delete; a layer can name itself, and is then wiped in the order of its own brightness; two layers may not read each other, even through two different effects that read layers; Duplicate and duplicating a composition follow the copies; a preset keeps no map.

## How it differs from what we already have

**Linear Wipe**, **Radial Wipe**, **Iris Wipe** and **Venetian Blinds** wipe by a fixed shape: a straight edge, a clock hand, a widening shape, stripes. Gradient Wipe wipes by a picture you choose, so the transition can have any shape: a hand-painted smear, clouds, a burn, the brightness of the next shot, or the layer's own brightness so the shadows go first.

## Known limits

- A map with few greys wipes in steps: a map of pure black and white wipes the black at the first 1% and the white at the very end. Soft maps make smooth wipes.
- It runs on the processor. The graphics card learns it in a card unit of its own.

## How you will check it

The build's test draws the twenty-two fixture cases and the six wrong settings and compares every pixel with the numbers `tools/gradient_wipe_reference.py` worked out, opens the two circle files, and writes `verification/B-129_gradient_wipe_table.md`, with pictures. A playtest sheet walks you through adding it, picking a map and keying the completion.
