# B-617 playtest: the boxes of a stretched layer show the value the picture is drawn from (D-742)

## What to try

1. Open a project with a drawing or composition layer whose position, rotation or an effect setting is keyed (or open `Fixtures/stretched_values/host.json`).
2. Set the layer's **Time stretch** to 200 % in Effect Controls. Its keys now play twice as slowly, so the timeline's key marks spread out.
3. Put the playhead between two keys. The boxes in Effect Controls and on the timeline show the number the picture is drawn with at that frame: halfway between the two key marks, halfway between their values.

Before this build, a stretched layer's boxes showed the value from a different moment: on a layer stretched to 200 % starting at frame 4, frame 14 showed what the picture reaches at frame 24. A mask's feather, opacity and expansion and the Time Remap already showed the right value.

## What the check found (private copy, `Fixtures/stretched_values/host.json`): 7 of 7 pass

- The layer `nest` starts at frame 4 and is stretched to 200 %. Its keys at 4 and 24 play at frames 4 and 44.
- On frames 2, 4, 11, 14, 24 and 43, every box (position, rotation, depth, the mask's path and feather, Fractal Noise's Brightness and Offset) shows the value at the layer's key time, the same number the picture uses. At frame 14 that is rotation 22.5, position 80, 70, Brightness −60, Offset 25, −12.5.
- No page errors.

Fixture check `app/src/tests/stretched_values.rs`: 42 of 42 (see `B-617_stretched_values_table.md`). FX-SVAL-001 to 006 ask the window for its boxes on six frames and compare each of seven boxes with `tools/stretched_values_reference.py`. Before the fix, 18 of those 42 passed: only frames 2 and 4, where both times give the same value, and the mask's feather, which was already right. The app's other tests still pass.

## Pictures (`B-617 pictures/`)

- `window_f04.png`, `window_f14.png`, `window_f24.png`: the whole window with the layer chosen, at frames 4, 14 and 24. Look at Brightness and Offset Turbulence in Effect Controls: −80 and 0, 0 at frame 4; −60 and 25, −12.5 at frame 14; −40 and 50, −25 at frame 24.

## How this differs from After Effects

- None known. After Effects shows a stretched layer's values at the time they play.
