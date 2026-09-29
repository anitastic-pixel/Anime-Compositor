# B-120a: HSV Key, taking out a colour by hue, saturation and value (D-184)

Written on 2026-09-28, before any code. Accepted with items 1 to 9 of the list of effects and presets left to build. It is OpenToonz's HSV Key, ported under OpenToonz's BSD licence with its origin recorded, and four changes of ours, each marked.

## What you will see

A new effect, **HSV Key**, beside Colour Key in the Effects panel. Its card has OpenToonz's seven rows, in OpenToonz's order:

- **Hue**, 0 to 360 degrees, 120 when added, a green.
- **Saturation**, 0 to 100 %, 60 when added.
- **Value**, 0 to 100 %, 60 when added: how bright.
- **Hue Range**, 0 to 180 degrees, 40 when added.
- **Saturation Range** and **Value Range**, 0 to 100 %, 40 each when added.
- **Invert**, Off when added.

What it does: a pixel whose hue is within Hue Range of Hue, whose saturation is within Saturation Range of Saturation, and whose value is within Value Range of Value, all three at once, is taken out, made fully transparent. Everything else stays exactly as it was. With Invert On it is the other way round: the pixels inside the three windows stay and everything else goes. As added, it takes out a green screen, the screen's shadow and green light spilt on a figure, and leaves skin, lines, other colours and greys. All six numbers can be keyed.

## How it differs from Colour Key

Colour Key takes out colours near chosen swatches, with a soft band. HSV Key takes out a region of colour described by three windows, so one setting can take a screen from bright to dark (a wide Value Range) while leaving a figure's green shirt of a different hue. It has no soft band: a pixel goes or stays, as in OpenToonz.

## Our four changes from OpenToonz

1. It reads each pixel's colour as a drawing program stores it, not multiplied by its covering, so a half-covered edge of the screen is taken with the screen. OpenToonz reads the edge as a darker green and can leave a fringe.
2. The hue window goes round the circle: Hue 350 within 40 reaches 30, past 360. OpenToonz stops the window at 0 and 360, so a red key misses reds on the other side. That is why Hue Range stops at 180, which is everything.
3. Saturation and value are shown in percent, 0 to 100, where OpenToonz shows 0 to 1.
4. It is added as a green key, where OpenToonz starts at 0 everywhere, which takes out only pure black.

`hsv_key.png` in this folder is worked by the reference rule itself, three times enlarged, over a grey checkerboard where pixels are taken out: the figure before; as added; Hue 142 within 5, Saturation 100 within 10, Value 50 within 50, where the spill on the figure's left side stays; and the same inverted.

## Known limits

- A hard key: edges step, and a pixel half screen and half figure goes or stays whole. Colour Key's softness or Line Smoothing after it soften the edge.
- A grey has no hue; it counts as hue 0, red, so a red key with a wide Saturation Range reaching 0 takes greys too.
- It runs on the processor. The graphics card learns it in item 9, with the other new effects.

## How you will check it

The build's test draws the ten fixture cases and compares every pixel with the numbers `tools/hsv_key_reference.py` worked out, and writes `verification/B-120_hsv_key_table.md`, with pictures. A playtest sheet walks you through keying a green screen, narrowing the windows and inverting.
