# B-139: Snowfall, by hand

Built on 2026-09-29 against D-204, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' CC Snowfall does, by a rule of our own: soft
round flakes in three depths, near, middle and far, falling, drifting with the wind and swaying,
drawn wherever the layer shows. The far flakes are smaller, closer together and slower. It does
not grow the layer. It sits in the **Generate** group after Rain, with After Effects' own names
for its settings.

The generated halves are `verification/B-139_snowfall_table.md`, 125 of 125 checks passing,
which renders every FX-SNOW case against the numbers written before the code and draws the
pictures below, and `verification/B-12b_state_fields_table.md`, which checks the card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and moves in
the window.

## The pictures

`verification/B-139 pictures/`, three times enlarged, over a grey check where nothing is drawn:

- `sky.png`: the plate, a dark night sky with a clear margin round it.
- `start_frame_0.png` and `start_frame_24.png`: Snowfall as it starts, a second apart: big near
  flakes, smaller far ones, all moved on.
- `depth_0.png`: scene depth 0: every flake near and large.
- `depth_100.png`: scene depth 100: the far flakes tiny specks.
- `blizzard.png`: flakes 100, spacing 12, size 3, wind 4, speed 6: thick snow.

In every one the margin stays clear: the snow is only where the sky is.

## Before you start

Make the composition 160 by 90, 48 frames or more, and import `sky.png` from
`verification/B-139 pictures/`. Press **Full resolution**.

## What to check

1. **Adding it.** On the sky, pick **Snowfall** in **Add effect…**, under **Generate**, after
   Rain; typing "snow", "flakes", "winter" or "blizzard" in the search finds it too. The card
   shows Flakes 50, Spacing 32, Size 6, Scene Depth 50, Speed 2, Wind 0.5, Wiggle Amount 3,
   Wiggle Period 48, Random Seed 0, Opacity 100 and a white Color, and the picture is as
   `start_frame_0.png` at once.
2. **Playing.** Press play: the snow falls gently and drifts a little right, the near flakes
   faster than the far ones, each swaying from side to side. It plays the same every time.
3. **Flakes and Size.** Drag Flakes down: flakes vanish one by one, none at 0. Drag Size up: the
   flakes grow; at 0 there is no snow.
4. **Scene Depth.** At 0 every flake is near and large, as `depth_0.png`; at 100 the far ones are
   tiny and slow, as `depth_100.png`.
5. **Speed and Wind.** Speed 0 with Wind 0 and Wiggle Amount 0: the snow hangs still. Wind
   negative: it drifts left.
6. **Wiggle.** Wiggle Amount 10, Wiggle Period 12: the flakes sway widely, once every half
   second.
7. **Color, Opacity and Seed.** Pick a pale blue: the snow turns blue. Opacity 50: the flakes
   half show. Another seed: other flakes in other places.
8. **Keyed.** Key Flakes from 0 at the first frame to 100 at frame 48 and play: the snow thickens.
9. **Moved.** Move the layer: the snow moves with it, and never falls outside the sky.
10. **Out of range.** Type 101 in Flakes, or 0 in Wiggle Period: it is refused with a sentence
    saying what it runs to, and the card keeps its old number.
11. **Draft.** Press **Draft**: the picture is smaller and snows the same way.
12. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: the settings
    and their keys are still there.

## Known limits, on purpose

- Three planes of depth, not a continuous one.
- A flake is a soft dot: it does not tumble, blur with its motion or catch the light.
- After Effects' CC Snowfall has Variation settings, a Background Illumination, an Extra and a
  Composite With Original switch; ours has none of these. Its Flakes is a count; ours is a per
  cent of the places a flake could be.
- The snow is drawn only where the layer shows: put it on a solid to cover the frame.
- It is a picture effect: there is no card version yet; that comes later as its own unit.
- It is modelled on After Effects' CC Snowfall and is not claimed to match it.
- The cost on the reference shot's frames 100 and 101 at full size, the effect on all four
  layers: 48 to 54 ms without it; Snowfall as it starts, 69 to 71 ms; a blizzard (flakes 100,
  spacing 12, size 3), 84 to 94 ms. Big flakes packed close are dear: flakes 100, spacing 4,
  size 20 took 554 to 570 ms, because each pixel looks at every place a flake could reach it
  from. **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
