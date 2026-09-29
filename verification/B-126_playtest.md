# B-126: Lightning Bolt, by hand

Built on 2026-09-28 against D-190, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' Lightning does, by a rule of our own: a jagged
bolt of light from one point of the layer to another, a white-hot core in a coloured glow, with
thinner forks off it, and a new bolt every few frames. The line is halved again and again, each
middle pushed a little aside, so the bolt always starts and lands exactly on its two points.

The generated halves are `verification/B-126_lightning_bolt_table.md`, 138 of 138 checks
passing, which renders every FX-BOLT case against the numbers written before the code and draws
the pictures below, and `verification/B-12b_state_fields_table.md`, which checks the Lightning
Bolt card sends every setting the command reads. This sheet covers what the tables cannot: how
it looks and feels in the window.

## The pictures

`verification/B-126 pictures/`, the proposal's night scene at a quarter of 1920 by 1080. A
quarter-size picture takes a quarter of the width and the glow, as a quarter-size draft does, so
each picture shows what the full frame looks like, smaller. Start is at (30, 14) and End at
(64, 76), the cloud and the hill:

- `before.png`: a night sky, a band of cloud, a dark hill with three lit windows.
- `as_added_frame_0.png`: the bolt at frame 0, white in a blue glow, with a few thin forks.
- `as_added_frame_2.png`: frame 2, a new bolt between the same two points.
- `branches_80_detail_8.png`: Branches 80 and Detail 8, finer kinks and many more forks.
- `jaggedness_20.png`: Jaggedness 20, straighter.
- `red_and_gold.png`: Width 8, Glow 60, a gold core #fff2c0 in a red glow #ff4a1a.

Both ends are lit in every one, and the corners never change.

## Before you start

Import `verification/B-126 pictures/night.png`, make a layer from it, make the composition at
least 24 frames long and press **Full resolution**. The picture is a quarter of full size, so
for steps 1 to 6 set **Width** 0.75 and **Glow** 6, a quarter of theirs, to see what the
pictures show.

## What to check

1. **Adding it.** Pick **Lightning Bolt** in **Add effect…**, under **Light & Glow**, after
   Kira-kira; typing "lightning" or "electric" in the search finds it too. The card shows
   **Start Point** (40, 0), **End Point** (60, 100), **Jaggedness** 40, **Detail** 6,
   **Branches** 30, **Width** 3, **Glow** 24, **Opacity** 100, **Hold** 2, **Seed** 0,
   **Colour** #ffffff and **Glow Colour** #6e8cff. A bolt runs down the middle of the layer.
2. **The points.** Drag the Start and End markers on the picture to the cloud and the hill, or
   type (30, 14) and (64, 76). The bolt always begins and lands exactly on them. Frame 0 looks
   like `as_added_frame_0.png`.
3. **New bolts.** Play it, or step frame by frame. Frames 0 and 1 are one bolt, frame 2 a new
   one, like `as_added_frame_2.png`, and so on every two frames. Set Hold 1: a new bolt every
   frame. Set Hold 12: one every half second.
4. **Shape.** Set Jaggedness 20: straighter, like `jaggedness_20.png`; 0 is a straight line.
   Set Branches 80 and Detail 8: like `branches_80_detail_8.png`. Detail 1 is two straight
   pieces meeting at one bend.
5. **Look.** Set Width 2, Glow 15, Colour #fff2c0 and Glow Colour #ff4a1a (a quarter of the
   picture's 8 and 60): it looks like `red_and_gold.png`. The forks start at half the bolt's
   width and thin to nothing at their tips.
6. **Seed.** Change Seed: other bolts between the same points.
7. **Nothing.** Set Opacity 0, or Width 0 and Glow 0: the picture is exactly as imported.
8. **Out of range.** Type 9 in Detail: it is refused with a sentence saying detail runs from 1
   to 8, and the card keeps its old number. Type 0 in Hold: refused, it runs from 1.
9. **Keyed.** Key Opacity at 0 at the first frame and at 100 two frames later, then back to 0
   two frames after that: the bolt flashes. Key the End Point across the layer: the bolt
   follows it.
10. **Inside the layer.** Move the End Point past the layer's edge, to (60, 150): the bolt is cut
    at the edge; the layer never grows. Move the layer: the bolt moves with it.
11. **Draft.** Press **Draft**: the picture is smaller and the bolt thinner with it, the same
    bolt in the same place.
12. **Undo and saved.** Ctrl+Z steps back each change, the colours included. Save, close and
    open again: every setting and key is still there.

## Known limits, on purpose

- Each bolt appears whole on its first frame; it does not grow from its start. Key Opacity for a
  flash.
- A bolt past the layer's edge is cut there. For a bolt across the whole frame, put it on a solid
  or an adjustment layer the size of the frame.
- A colour typed in capitals is kept in small letters when saved, as Rain does.
- It is drawn on the processor; the graphics card learns it in a card unit of its own. On the
  reference shot, 1920 by 1080 at frame 100, with the bolt on the bottom layer, three runs each:
  50 to 53 ms without it; 52 to 55 ms as added (detail 6, branches 30, width 3, glow 24); 63 to
  64 ms at detail 8, branches 100, width 6, glow 100; and 299 to 316 ms at detail 8, branches
  100, width 100, glow 500, the slowest it gets, which is slow on purpose.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
