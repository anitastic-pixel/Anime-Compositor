# B-147a: Extract, keying a layer out by brightness (D-212)

Written on 2026-09-29, before any code. B10 of the second list of After Effects picks, accepted with the rest by your "take everything". An effect that does what After Effects' Extract does, by a rule of our own.

## What you will see

A new effect in the **Lines & Mattes** group, after HSV Key, which is where this program keeps its keys (After Effects keeps Extract under Keying, a group this program does not have). It takes out of a layer every part too dark or too bright, by one of its channels: the white paper behind a scanned pencil drawing, or the black behind a flash of light or a fire filmed on black. Its card, in After Effects' words:

- **Channel**, Luminance, Red, Green, Blue or Alpha, Luminance when added: what is measured, 0 to 255.
- **Black Point**, 0 to 255, 0 when added: anything darker than this is taken out.
- **White Point**, 0 to 255, 255 when added: anything brighter than this is taken out.
- **Black Softness**, 0 to 255, 0 when added: instead of a hard cut at the black point, a fade over this many steps above it.
- **White Softness**, 0 to 255, 0 when added: the same below the white point.
- **Invert**, Off or On, Off when added: keep what would be taken out, and take out the rest.

The four numbers can be keyed. When added it keeps everything, so it changes nothing until you move a point. The layer does not grow, and a draft looks the same, only smaller.

`extract.png` in this folder is worked by the rule itself on a small made-up scan: a face in black line on cream paper, a red scarf, a patch of blue sky and a yellow star; the paper taken out at white point 220, then softly; the line taken out at black point 40, and inverted, the line alone; only the middle band kept; and the red channel's own brightness used.

## How the rule works, in words

Each pixel's value in the channel is measured, 0 to 255, luminance as Threshold measures it. Inside the two points it is kept whole, outside them it is taken out, and in a softness it fades in a straight line. A point set exactly on a pixel's value keeps it.

## How it differs from what we already have

**Threshold** turns a picture black and white by brightness; **Extract** keeps its colours and takes some of it away. **Color Key**, **Linear Color Key** and **HSV Key** take out one colour or hue; Extract takes out everything past a brightness, whatever its colour. **Luma Matte** uses another layer's brightness; Extract uses the layer's own.

## Known limits

- After Effects shows a histogram of the layer on the card, to set the points by; this card has the numbers only.
- It runs on the processor; the graphics card version is its own later unit.
- It is modelled on After Effects' Extract and is not claimed to match it.

## How you will check it

The build's test draws the seventeen fixture cases and the ten wrong settings and compares every pixel with the numbers `tools/extract_reference.py` worked out, and writes `verification/B-147_extract_table.md`, with pictures. A playtest sheet walks you through putting it on a layer.
