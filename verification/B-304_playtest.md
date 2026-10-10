# B-304: Threads

Built on 2026-10-10 as D-425, under your /loop request: **Threads**, in **Generate**, our name
for CycoreFX's CC Threads. It weaves the layer into a cloth of crossing threads, each thread
taking the picture's colours where it lies, with clear gaps between the threads and shadows where
they cross.

- **Width** and **Height** (50 pixels): how far apart the up-and-down and the across threads are.
- **Overlaps** (1): how many threads each passes over and under. 1 is a plain weave like a
  basket; 2 or more makes diagonal lines, a twill.
- **Direction** (0 degrees): turns the whole cloth.
- **Center** (50, 50 per cent): where the cloth is laid and turned about.
- **Coverage** (90): how wide each thread is. 100 fills every gap; 0 leaves nothing.
- **Shadowing** (50): how dark the threads beneath are where the ones above cross them.
- **Texture** (0): shades each thread round, bright in its middle and dark at its sides.

CycoreFX's manual says what each setting does in words but gives no formula, ranges or starting
values, so those are ours, and the look is not matched pixel for pixel. All logged.

The check, `verification/D-425_threads_table.md` (191 of 191), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-425 pictures/`, each the
street at frame 0 (`town.png` is the layer).

## What to check

1. **Before.** `1_before.png`: the street with no effect.
2. **As added.** `2_as_added.png`: big threads 50 pixels apart crossing over the street, thin
   clear gaps between them, a faint shadow beside each crossing.
3. **Fine weave.** `3_fine_weave.png`: threads 8 apart, shadowing 80: a fine basket weave, the
   threads underneath clearly darker.
4. **Twill, turned.** `4_twill_turned.png`: 10 apart, Overlaps 3, turned 45 degrees, texture 50:
   diagonal stripes of rounded threads, the whole cloth tilted.
5. **Open cloth.** `5_open_cloth.png`: 16 apart, Coverage 50, Texture 100: round threads with big
   clear square holes between them.
6. **Ribbons.** `6_ribbons.png`: width 40, height 12: broad up-and-down ribbons over narrow
   across ones.
7. **In the app.** Add Threads (Generate) to a picture and drag Direction: the cloth turns about
   the centre. Raise Overlaps to 2: the weave becomes diagonal. Lower Coverage: the gaps grow.
8. **Out of range.** Type 0 in Width: it is refused with a sentence saying it runs from 1 to 1000.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-304_threads_timing_table.md` (quiet): the reference shot with a moving
Noise and a Threads on three layers, played again, 16.4 ms a frame on the card as added
against 12.2 for the Noise alone; on the processor 48.3 against 41.6:
the card is quicker than the processor for this effect.
