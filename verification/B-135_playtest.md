# B-135: Radio Waves, by hand

Built on 2026-09-29 against D-200, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' Radio Waves does, by a rule of our own: it sends
rings out from a point, a new one every few frames, each growing, fading and dying as the layer
plays. The rings are polygons, 3 sides to 64, 64 looking round; they can turn as they grow and
drift as they go, and are drawn over the drawing and the empty space round it alike. A shockwave,
a sonar ping or a ripple of rings behind a character. The layer does not grow. It sits in the
**Generate** group after Light Sweep, and its settings carry After Effects' own names.

The generated halves are `verification/B-135_radio_waves_table.md`, 126 of 126 checks passing,
which renders every FX-RWAVE case against the numbers written before the code and draws the
pictures below, and `verification/B-12b_state_fields_table.md`, which checks the Radio Waves card
sends every setting the command reads. This sheet covers what the tables cannot: how it looks
and feels in the window.

## The pictures

`verification/B-135 pictures/`, a badge (a blue plate with a dark border, a red bar across and a
gold disc in the middle, nothing round it), three times enlarged, over a dark blue where nothing
is drawn:

- `before.png`: no effect.
- `as_added_frame_8.png`: as it is added, at frame 8: one white ring 40 pixels out from the
  middle, crossing the badge and the empty space above and below it.
- `as_added_frame_30.png`: frame 30: the first ring gone past the edges and the second 30
  pixels out.
- `shockwave.png`: a shockwave at frame 8: one ring 10 pixels wide thinning to 1 over 20
  frames, a soft Triangle profile, fading out.
- `turning_squares.png`: frame 20: squares, a new one every 6 frames, each turned 4 degrees
  more a frame, so the older ones are turned further.
- `drifting_triangles.png`: triangles sent out from the badge's left side at frame 24, drifting
  right 2 pixels a frame, so each older one sits further right.
- `orange_sine.png`: orange (#ffb040), a soft Sine profile, 8 wide, Opacity 80, at frame 30.

## Before you start

Make the composition 160 by 100 and import `plate.png` from `verification/B-135 pictures/`.
Press **Full resolution**.

## What to check

1. **Adding it.** On the plate, pick **Radio Waves** in **Add effect…**, under **Generate**,
   after Light Sweep; typing "radio waves", "shockwave", "sonar" or "ripple" in the search
   finds it too. The card shows **Producer Point** 50, 50, **Sides** 64, **Interval** 24,
   **Expansion** 5, **Orientation** 0, **Direction** 90, **Velocity** 0, **Spin** 0,
   **Lifespan** 96, **Opacity** 100, **Fade-in Time** 0, **Fade-out Time** 48, **Start Width**
   5, **End Width** 5, **Profile** Square and **Color** white, and above them a box standing for
   the drawing with a mark at the producer point and three rings round it. At frame 8 the
   picture is as `as_added_frame_8.png`, and at frame 30 as `as_added_frame_30.png`.
2. **Dragging.** Drag the mark in the box to the lower left: the rings in the box and in the
   picture follow it.
3. **Playing.** Play: a ring leaves the middle every 24 frames, one second, grows 5 pixels a
   frame and fades out over the second half of its life. At frame 0 a ring has just been born
   and is a dot.
4. **Sides and orientation.** Set Sides to 4: the rings are squares, a corner pointing up. Turn
   the Orientation dial to 45: a side faces up instead. Sides 3 is triangles; 6 is hexagons.
5. **Spin.** With Sides 4, Interval 6, Expansion 3, Spin 4, Lifespan 30 and both widths 2, go
   to frame 20: as `turning_squares.png`. Play: each square turns as it grows.
6. **Drift.** Set Producer Point to 25, 50, Sides 3, Interval 8, Expansion 2, Velocity 2 and
   Lifespan 30, and go to frame 24: as `drifting_triangles.png`. Turn the Direction dial to 180:
   they drift down instead.
7. **Profile.** Back to the settings as added, with both widths 20: Square is a hard band,
   Triangle is brightest down its middle and fades to both sides, Sine is softer still.
8. **Colour and opacity.** Set Color to orange (#ffb040), Profile Sine, Interval 8, Expansion 3,
   Lifespan 40, Opacity 80, Fade-out Time 20 and both widths 8, and go to frame 30: as
   `orange_sine.png`.
9. **Fades and widths.** Set Fade-in Time to 24: each ring appears slowly instead of at full
   strength. Set Start Width 10, End Width 1: each ring thins as it grows, as in
   `shockwave.png`.
10. **Out of range.** Type 2 in Sides, 0 in Interval, 101 in Opacity or 361 in Spin: it is
    refused with a sentence saying what it runs from and to, and the card keeps its old number.
11. **Draft.** Press **Draft**: the picture is smaller and the rings sit in the same places, as
    wide for the smaller picture.
12. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there.

## Known limits, on purpose

- The rings are polygons; there are no other shapes, and no rings from a mask or the drawing's
  outline.
- There are three profiles: Square, Triangle and Sine.
- Every ring alive follows the settings at the current frame: key Sides or Expansion and all the
  rings on screen change together, not only the ones born afterwards.
- Rings do not bounce off the layer's edges; they are cut off there.
- The rings are counted from the composition's frame 0, not from where the layer starts.
- There is no handle for the producer point in the viewer yet: move it by the card's box or its
  numbers.
- It is a picture effect: there is no card version yet; that comes later as its own unit.
- It is modelled on After Effects' Radio Waves and is not claimed to match it.
- It costs little: the reference shot's frames 100 and 101 at full size take 48 to 53 ms
  without it, 60 to 65 ms with the settings as added and Sine on all four layers, and 260 to 273
  ms with a ring born every frame, 20 pixels wide, on all four.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
