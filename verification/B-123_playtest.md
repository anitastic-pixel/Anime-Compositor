# B-123: the batch's five on the card, by hand

Built on 2026-09-28 against D-187, which you accepted as the last of the nine ("1, 2,3,5,6,7,8,9",
then "include 4 as well, just 1 thru 9"). Color Lookup, Line Blur, HSV Key, Paraffin and Kira-kira
are now drawn by the graphics card in the viewer when one is a drawn layer's last effect, as the
other sixty-odd effects already are. Exports and renders to file are untouched: the processor
still draws those, byte for byte as before.

`verification/B-123_gpu_fx_table.md` is the comparison: every fixture frame of the five, and the
reference shot with each on three layers (one after a Drop Shadow), at Full and Draft, drawn by
the processor and by the card. **1010 of 1010 checks pass**, none more than 1 level of 255 apart,
with the same warnings on both. The three pictures of the worst frame, the reference shot with
Kira-kira, are in `verification/B-123 pictures/`: `cpu.png`, `gpu.png`, and `difference.png`,
black where the two agree and white round every pixel 1 level apart.

HSV Key first failed on the reference shot: about 1,400 pixels inside the green square were keyed
on one side and not the other. The card's own division can be one part in ten million billion
off, and that moved a colour sitting exactly on one of the key's edges across it. The card now
divides the way the processor does, and every pixel agrees.

## Before you start

- Use the release build. It opens on the reference shot.
- Leave **Draw on** at **Auto**.

## What to check

For each of the five, select a layer, add the effect, and switch **Draw on** between **CPU** and
**GPU** a few times. The picture should not change in any way you can see.

1. **Color Lookup.** Choose `Fixtures/cube_lut/luts/warm_17.cube` on its card. The warm look
   is the same on CPU and GPU. Choose another file: the picture changes at once on GPU.
2. **Line Blur.** Length 12, Strength 80. The lines soften along their own length the same on
   both. Drag Length from 0 to 50 on GPU: the picture follows your hand.
3. **HSV Key.** Hue 30, Hue Range 40, Saturation 50, Saturation Range 50, Value 60, Value Range
   40. The same pixels are cut out on both, the snow and the clouds' warm edges among them. Turn
   Invert on: the same again, the other way.
4. **Paraffin.** Colour #ffc890, Direction 45, Spread 60, Opacity 70, Blend Overlay. The warm
   wash comes from the top right the same on both. Try each Blend in turn.
5. **Kira-kira.** Threshold 60, Spacing 24, Density 80, Size 12. The stars sit on the same
   highlights, the same size, on both. Play it on GPU: they twinkle as on CPU.
6. **Kira-kira, heavy.** Spacing 4, Density 100, Size 200, Threshold 60. This is slow on both,
   about 0.9 of a second a frame on CPU and 0.7 on GPU (the table below). The stars are the same
   on both.
7. **Draft.** Switch to **Draft** and repeat steps 1, 3 and 5. CPU and GPU still match.
8. **Not the last effect.** Put a Gaussian Blur after any of the five. The picture is the same on
   both; the five then stay on the processor, as any effect before another does.
9. **Export is untouched.** An export is the same file whatever **Draw on** says.

## How long a frame takes

Timed on 2026-09-28 by a throwaway test, not kept: the reference shot at Full, the effect on all
four layers, frames 101 to 103, the drawings already read. AMD Ryzen 9 9900X with 24 threads;
Windows 11; release build. Milliseconds per frame, each of the three frames.

| On every layer | CPU | GPU |
| --- | --- | --- |
| nothing | 16, 16, 15 | 2, 2, 5 |
| Color Lookup, warm_17 | 42, 44, 43 | 22, 25, 19 |
| Line Blur, the settings of step 2 | 124, 116, 120 | 33, 38, 17 |
| HSV Key, the settings of step 3 | 38, 43, 41 | 15, 9, 5 |
| Paraffin, the settings of step 4 | 50, 49, 51 | 11, 11, 7 |
| Kira-kira as added (threshold 95, spacing 64, size 40) | 51, 52, 56 | 31, 26, 27 |
| Kira-kira, heavy, step 6 | 902, 872, 906 | 704, 672, 668 |

The heavy Kira-kira is the one the card barely helps: tens of thousands of stars, each lighting a
400-pixel square, is a great deal of light to lay down either way. A viewer that must play it
should use Draft, or fewer, smaller stars.

## Known limits, on purpose

- The card still asks the processor for two quick things: Paraffin's near and far side of the
  figure, and Kira-kira's stars, which highlights get one and how big. Both are one pass over the
  drawing. The light itself, every arm of every star, is the card's.
- Color Lookup's file is read by the processor, once, and kept until it changes on disk.
- A lookup file too big for the card to hold with the settings, or a frame so big that
  Kira-kira's light would not fit the card's largest buffer, is drawn by the processor, which says
  so.

## What to report

"works", or which step number did something else and what it did, with the settings and the frame
number.
