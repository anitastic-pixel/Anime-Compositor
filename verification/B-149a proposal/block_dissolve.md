# B-149a: Block Dissolve, a layer vanishing in random blocks (D-214)

Written on 2026-09-29, before any code. B12 of the second list of After Effects picks, accepted with the rest by your "take everything". It is After Effects' **Block Dissolve** in purpose and names, by a rule of our own.

## What you will see

A new effect in the **Transition** group, beside Venetian Blinds and the wipes. It makes a layer vanish in blocks: the drawing is cut into a grid of rectangles, and as Transition Completion goes from 0 to 100 the blocks go one by one, in a scattered order, until at 100 nothing is left. Keyed from 100 down to 0 it is the other way: the layer appears in blocks. A block once gone stays gone as the completion rises, so the transition never flickers.

Its card, in After Effects' words:

- **Transition Completion**, 0 to 100, 0 when added: how much of the layer has gone.
- **Block Width** and **Block Height**, 1 to 10000 pixels, 1 when added: the size of the blocks, so as added it dissolves pixel by pixel.
- **Feather**, 0 to 10000 pixels, 0 when added: softens the blocks' edges over that many pixels.

All four can be keyed. The blocks are laid from the drawing's own top left corner, so they move and scale with the layer. The colour never changes, only how much the layer covers. The layer does not grow. A draft looks the same, only smaller.

`block_dissolve.png` in this folder is worked by the rule itself on a made-up title card: as it is, a quarter, half and three quarters gone in blocks 10 by 10, half gone in wide flat blocks, half gone pixel by pixel, and half gone with a feather of 8.

## How the rule works, in words

Every block is given a number from 0 up to 1 by chance, the same way Noise picks its grain, and the same number every time. A block is gone once the completion, as a share of 100, has passed its number. With no feather each pixel is kept whole or gone with its block. With a feather each pixel keeps the share of a square that many pixels across, centred on it, that lies in kept blocks, so the blocks' edges fade over the feather.

## How it differs from what we already have

**Venetian Blinds**, **Linear Wipe**, **Radial Wipe** and **Iris Wipe** take the layer away in one tidy shape; Block Dissolve takes it away in scattered pieces. **Noise** puts grain on the picture and changes its colour; Block Dissolve only takes covering away.

## Known limits

- After Effects' **Soft Edges** switch is left out: the feather is always the soft one.
- There is no random seed: two Block Dissolves with the same block sizes go in the same order.
- It runs on the processor; the graphics card version is its own later unit.
- It is modelled on After Effects' Block Dissolve and is not claimed to match it; its blocks go in a different order.

## How you will check it

The build's test draws the thirteen fixture cases and the six wrong settings and compares every pixel with the numbers `tools/block_dissolve_reference.py` worked out, and writes `verification/B-149_block_dissolve_table.md`, with pictures. A playtest sheet walks you through putting it on a layer.
