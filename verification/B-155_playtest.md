# B-155: a layer's whole run of effects drawn on the graphics card

Built on 2026-09-30 under your "sounds great! let's do 1 through 7 to your discretion." This is
item 4 of the GPU plan, D-224.

Until now, the card drew only the **last** effect of a layer's stack. Every effect above it was
worked on the processor, and the picture was then sent to the card. Now the card draws the
layer's **whole run** of effects it knows, from the last one it cannot draw down to the end of
the stack, one after another, without the picture going back and forth.

**Nothing you should see changes.** Where an effect sits in the stack still decides what it
does. The pictures may differ from the processor's by at most 1 level of 255 in a channel, the
tolerance you already accepted for the card. Exports and renders to file are untouched: they
always use the processor.

Two kinds of effect only **begin** a run, never sit in the middle or end of one:
- **Bloom, Glow, Paraffin and Kira-kira** look at the whole picture they are given before the
  card is asked. So when one of them is in the stack, the card starts with it.
- **HSV Key.** The colour of a nearly grey pixel swings with the smallest change. Given the
  card's picture after two other effects, it keyed out pixels the processor kept, a full 255
  levels apart. So it stays first in a run, where it gets exactly the processor's picture.

`verification/B-155_gpu_chain_table.md` is the check. Each of the 69 effects the card draws is
put on the reference shot's first three layers twice: last, after two other card effects, and
first, before two. One more case puts a Kaleidoscope, which the card does not draw (D-240), in
the middle of a stack, so the card must draw the part after it.

**834 of 834 checks pass** (564 before the build):
- no pixel more than 1 level apart, at Draft and Full, frames 0 and 100;
- the same warnings on both;
- at Full, the card really drew the whole run.

The pictures of the closest case (Halftone first) are in `verification/B-155 pictures/`: the
processor's, the card's, and the difference.

## Before you start

- Use the release build. It opens on the reference shot.
- To compare, switch between **Draw on: CPU** and **Draw on: GPU**. The picture must not change.

## What to check

1. **A run of three.** On layer 1 add, in this order, **Levels**, **Gaussian Blur** and
   **Hue/Saturation**, with some visible settings (a blur of 10, a hue shift of 60). Go to frame
   100 at **Full**. Switch between **Draw on: CPU** and **Draw on: GPU**. The picture looks the
   same.
2. **Order matters.** Drag the Gaussian Blur to the bottom of the stack, below Hue/Saturation.
   Switch between CPU and GPU again. It still looks the same on both.
3. **A split.** Add a **Kaleidoscope** between Levels and Gaussian Blur. Switch between CPU and
   GPU. The same picture on both. On GPU, the label above the viewer still ends "drawn on
   GPU", not "drawn on CPU".
4. **Glow first.** Take the Kaleidoscope away and put a **Glow** at the top of the stack. Switch
   between CPU and GPU. The same picture.
5. **Play.** Press play at **Full** on **Draw on: GPU** and let it loop. Nothing flickers, and no
   stripes, black patches or old frames show.

## How long a frame takes

Measured on this machine, release build, the median of three runs (`verification/B-155_timing_table.md`):

| Shot | Quality | Before ms | After ms |
|---|---|---:|---:|
| still effects on the first three layers | Draft | 10.5 | 10.1 |
| still effects on the first three layers | Full | 10.6 | 9.7 |
| the same, ending in a moving Noise | Draft | 13.8 | 12.2 |
| the same, ending in a moving Noise | Full | 13.8 | 14.9 |
| the same, beginning with a moving Noise | Draft | 10.6 | 14.2 |
| the same, beginning with a moving Noise | Full | 96.4 | 24.7 |

The gain is where an effect early in the stack changes every frame, at Full: four times
quicker. At Draft that same case is 3.6 ms slower, since the processor's work on the small draft
picture was already quick. The rest is about the same.

## What to report

"works", or the number of the step that did something else, what it did, and the frame number.
