# B-138a: Median and Smart Blur, specks and grain cleaned away (D-203)

Written on 2026-09-29, before any code. The first of the second list of After Effects picks, B1, accepted with the rest by your "take everything". Two effects that do what After Effects' Median and Smart Blur do, by rules of our own.

## What you will see

Two new effects in the **Blur & Sharpen** group, after the blurs already there.

**Median** cleans specks away. Each pixel looks at the pixels round it and takes the middle colour of them, the one with as many darker as lighter, so a lone speck of dust, a stray dot or a pinhole is outvoted and gone, while an edge, which has as many pixels on each side, stays sharp. Scanned drawings and noisy footage come out clean and flat. Its card, in After Effects' words:

- **Radius**, 0 to 10 pixels, 2 when added: how far round each pixel it looks. 0 is the drawing untouched. Keyable.
- **Operate on Alpha**, off when added. Off, the drawing's outline is kept exactly: a pixel that was clear stays clear and a soft edge keeps its softness, and only the colours are cleaned. On, the outline is cleaned too: pinholes in the drawing fill, stray dots outside it go, and sharp corners are rounded off.

**Smart Blur** smooths without blurring the lines. Each pixel is mixed only with the pixels round it whose colour is close to its own, so flat colour and fine grain are smoothed while a line, a shadow's edge or anything else more than the threshold away is left sharp. It is the skin-smoothing and grain-cleaning blur. Its card:

- **Radius**, 0 to 10 pixels, 3 when added: how far round each pixel it mixes. Keyable.
- **Threshold**, 0 to 255, 64 when added: how close a colour must be to be mixed in, in the 8-bit steps of Recolor's Tolerance, each of red, green, blue and covering. 0 mixes only the very same colour, which changes nothing; 255 mixes everything, a plain blur. After Effects measures it 0 to 100; our 64 is about its 25, where it starts. Keyable.

Neither effect grows the layer, and Smart Blur never spreads the drawing into empty space: a pixel that is clear stays clear.

`median_smart_blur.png` in this folder is worked by the rules themselves: a speckled, grainy face, cleaned by Median with Operate on Alpha off and on, and smoothed by Smart Blur at three thresholds.

## How it differs from what we already have

**Gaussian Blur** softens everything, lines included. **Selective Color Blur** blurs only the colours you pick. **Line Blur** softens only the lines. Smart Blur needs no colours picked: it keeps whatever edge is sharper than its threshold. Median is the only one that removes specks rather than smearing them.

## Known limits

- The radius stops at 10 pixels, for speed: Median sorts every pixel's neighbours afresh.
- Median takes away any line thinner than about its radius, as it does the one-pixel strand of hair in the picture, and rounds sharp corners, as it does the square eyes. Keep the radius small on line art.
- After Effects' Smart Blur has a Mode that draws only the edges it keeps, or lays them over the picture; that is left out. **Find Edges** draws edges.
- Smart Blur's mix is a flat average over its circle, not weighted toward the middle.
- They run on the processor; the graphics card version is its own later unit.
- They are modelled on After Effects' Median and Smart Blur and are not claimed to match them.

## How you will check it

The build's test draws the fifteen fixture cases and the eight wrong settings and compares every pixel with the numbers `tools/median_smart_blur_reference.py` worked out, and writes `verification/B-138_median_smart_blur_table.md`, with pictures. A playtest sheet walks you through cleaning a speckled drawing and smoothing grainy skin.
