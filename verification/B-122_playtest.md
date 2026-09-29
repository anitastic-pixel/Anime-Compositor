# B-122: Kira-kira, by hand

Built on 2026-09-28 against D-186, which you accepted as the eighth of the nine ("1, 2,3,5,6,7,8,9",
then "include 4 as well, just 1 thru 9"). It is anime finishing's kira-kira, written from scratch:
small stars set on a drawing's near-white highlights, an eye's glint, a blade's edge, a sparkle on
the sea, each growing and fading on its own beat. The frame is cut into square cells; a cell with
a highlight in it may get one star, at the middle of its highlights, so a big white area gets a
few stars, not a smear. Only near-white counts: a pale skin, a cream moon or a gold is left alone.

The generated halves are `verification/B-122_kira_kira_table.md`, 122 of 122 checks passing,
which renders every FX-KIRA case against the numbers written before the code and draws the
pictures below, and `verification/B-12b_state_fields_table.md`, which checks the Kira-kira card
sends every setting the command reads. This sheet covers what the tables cannot: how it looks and
feels in the window.

## The pictures

`verification/B-122 pictures/`, the proposal's night scene at a quarter of 1920 by 1080. A
quarter-size picture takes a quarter of every distance, as a quarter-size draft does, so each
picture shows what the full frame looks like, smaller:

- `before.png`: a face whose eyes each hold two white glints, a cream moon, a pale collar, a sword
  with a white edge and a white point on its gem, and a sea with a moon path of white dashes.
- `as_added_frame_0.png`: Kira-kira as it is added, at frame 0. A few stars on the sea and the
  blade; the face, the collar and the moon have none.
- `as_added_frame_12.png`: the same half a second later. Different stars are at their brightest;
  some that were small are large, and the other way about.
- `density_100.png`: every cell with a highlight gets a star, the eyes' glints included.
- `small_and_close.png`: Spacing 32 and Size 24, more and smaller stars.
- `warm_cross.png`: a warm four-armed cross turned 45 degrees, an X.

The moon, the cheek, the collar and the sky never change in any of them.

## Before you start

Import `verification/B-122 pictures/night.png`, make a layer from it, make the composition at
least 24 frames long and press **Full resolution**. The picture is a quarter of full size, so
for steps 1 to 5 set **Spacing** 16 and **Size** 10, a quarter of theirs, to see what the
pictures show.

## What to check

1. **Adding it.** Pick **Kira-kira** in **Add effect…**, under **Light & Glow**, after Cross
   Glare. The card shows **Threshold** 95, **Spacing** 64, **Density** 60, **Size** 40,
   **Angle** 0 on a dial, **Twinkle** 100, **Period** 24, **Seed** 0, **Opacity** 100,
   **Shape** Star and **Colour** #ffffff. With Spacing 16 and Size 10, frame 0 looks like
   `as_added_frame_0.png`.
2. **Twinkling.** Play it, or scrub from frame 0 to frame 24. Each star grows, peaks and fades on
   its own beat, and frame 24 is frame 0 again. Frame 12 looks like `as_added_frame_12.png`.
3. **Steady.** Set Twinkle 0: the stars stop twinkling and all show at their full size, still
   each a little different in size. Set Twinkle 50: they breathe between half and full.
4. **More stars.** Set Density 100: it looks like `density_100.png`. Change Seed: the same cells,
   different sizes and beats; at Density 60, a different choice of cells.
5. **Shape, angle and colour.** Set Shape Cross, Angle 45 and Colour #ffe0a0: it looks like
   `warm_cross.png`. Drag the Angle dial round: the arms turn.
6. **Only near white.** Lower Threshold slowly from 95. At 90 the collar starts to sparkle, at
   82 the moon, and at 74 the skin: each one's smallest channel, as a per cent of 255. Put it
   back to 95.
7. **Nothing.** Set Size 0, Density 0 or Opacity 0: the picture is exactly as imported.
8. **Out of range.** Type 1 in Spacing: it is refused with a sentence saying spacing runs from 2
   to 1000, and the card keeps its old number. Type 0 in Period: refused, it runs from 1.
9. **Keyed.** Key Size at 0 at the first frame and at 10 twelve frames later. Scrub between: the
   stars grow out of nothing.
10. **Moving the layer.** Move the layer across the frame: the stars move with the drawing and
    stay on the same highlights.
11. **Draft.** Press **Draft**: the picture is smaller and its stars are smaller with it, in the
    same places.
12. **Undo and saved.** Ctrl+Z steps back each change, the Shape choice and the Colour included.
    Save, close and open again: every setting and key is still there.

## Known limits, on purpose

- A star sits at the middle of its cell's highlights, not on the brightest pixel. A long white
  edge crossing a cell gets one star at that stretch's middle; lower Spacing for more along it.
- The stars are added light: on white they cannot show, so an eye's glint shows its star on the
  pixels round it, not on the glint itself.
- The layer grows by the size on every side so the arms can reach past the drawing's edge.
- A colour typed in capitals is kept in small letters when saved, as Rain does.
- It is drawn on the processor. On the reference shot, 1920 by 1080 at frame 100, three runs
  each: 52 to 58 ms without it; 75 to 76 ms as added (spacing 64, density 60, size 40,
  threshold 95); 84 to 87 ms at spacing 16, density 100, size 80, threshold 80; and 910 to
  928 ms at spacing 4, density 100, size 200, threshold 60, hundreds of big stars, which is
  slow on purpose. Item 9 of the batch moves it to the graphics card.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
