# B-636 playtest: Turbulent Displace gains After Effects' Offset and its other Displacement types (D-757)

## What to try

1. Add **Turbulent Displace** to a picture layer. Set **Amount** to 30 and **Size** to 40.
2. Change **Offset (Turbulence)** from 0, 0 to 40, 0. The warp keeps its shape and slides 40 pixels to the right. Key it from 0, 0 to 200, 0 over a second and play: the warp travels across the picture.
3. Open **Displacement**. It now lists Turbulent, Bulge, Twist, Turbulent Smoother, Bulge Smoother, Twist Smoother, Vertical Displacement, Horizontal Displacement and Cross Displacement, After Effects' order.
   - **Bulge:** the picture swells in round lumps, like looking through bubbled glass.
   - **Twist:** the picture swirls round the same lumps.
   - **Cross Displacement:** long straight streaks, some across and some down, crossing.
   - **The Smoother ones:** the same kinds with less fine grain. The difference shows best at Complexity 4 or more.
4. Open a project saved before this build that has a Turbulent Displace. It looks exactly as before and shows Offset 0, 0. Saving it writes no offset, so the file stays as it was.

Before this build there was no Offset and only Turbulent, Horizontal and Vertical. A tutorial using Bulge, Twist or Cross could not be copied. The old build kept such an effect but left it out of the frame, with a warning.

## What the check found

Fixture check `tests/b636_turbulent_types.rs`: 186 of 186 (see `B-636_turbulent_types_table.md`). FX-TURB-TY-001 to 018 are pixel-compared with `tools/turbulent_types_reference.py` (tolerance 2e-5). No existing fixture changed.

Other checks:
- Every older Turbulent Displace fixture saves as it was written. That is FX-TURB 001 to 026, FX-TURB-AE 001 to 012 and FX-SHIMMER 001 to 016. Each one draws frames 0 and 4 bit for bit the same as its twin with Offset 0, 0 written.
- These are refused with a sentence and change nothing:
  - Displacement "Bulge" with a capital, or "cross_displacement"
  - an offset past 100000
  - an offset keyed past -100000
- Offset is halved in the half-size draft preview, as a distance should be.
- The picture is the same however it is cut into tiles.
- On the graphics card, an offset is drawn there and matches the processor within 1 level of 255. The six new words are drawn by the processor, as Horizontal, Vertical and Pinning already were, with no warning.
- `b70`, `b191`, `b212`, `b289` and `b290` (the older Turbulent Displace checks) pass. The app's tests: 114 passed, 5 ignored.

## Against After Effects (Manuel does Motion, youtube.com/watch?v=501MUGcxYRk)

Each sheet shows three panels:
- **After Effects:** the still from the video.
- **Before:** the wave 1 build.
- **After:** this build.

Both of ours are drawn by the real app on the processor, on the same gradient, with the panel's values. After Effects' Complexity 10 is entered as 8, our top.

The measures:
- **Shift:** how far the warp moved from the 0:46 still, in composition pixels.
- **Streak half-length:** how far a pattern runs across and down before it changes, in composition pixels (`aniso.py`). Down divided by across is above 1 for up-and-down streaks and near 1 for round lumps.

| Still | Setting | After Effects | Before | After | Verdict |
| --- | --- | --- | --- | --- | --- |
| 0:46 | Turbulent, Offset at the middle (ours 0, 0) | shift 0 | shift 0 | shift 0 | unchanged, as it should be (a control) |
| 0:50 | Offset moved to 1119, 540 (ours 159, 0) | shift +175 | +0 (no offset) | +160 | **matches**: same way, 15 px apart, about 5 pixels of the still |
| 1:58 | Bulge, Size 50 | 24 by 21, round (0.9) | effect left out | 9 by 9, round (1.0) | **close**: round lumps like After Effects', but ours are finer (a known gap) |
| 2:28 | Cross, Size 50 | 18 by 184 (10.2 down) | effect left out | 9 by 15 (1.7) | **differs** in the number: ours shows up-and-down streaks, but much shorter |
| 2:34 | Cross, Size 5 | 6 by 15 (2.5) | effect left out | 6 by 6 (1.0) | **differs**: at Size 5 our fine grain hides the streaks |

Ours is drawn finer than After Effects' at the same Size. Wave 2 found about half the size, and it is kept on purpose so no saved project changes (an owner option in D-757). That fine grain also shortens the measured streaks. By eye, the 2:28 sheet shows ours making the same up-and-down streak pattern, much shorter.

The black notches at the edges of our pictures come from Pin All with Amount 350 on transparent edges. They are in the before picture too, so they are not from this change.

## Pictures (`B-636 pictures/`)

- `td_offset_159.png`: Offset. The old build ignores it; this build's warp slides right by the amount typed, as After Effects' does.
- `td_bulge.png`: Bulge. The old build leaves the effect out; this build's round lumps match After Effects' kind, though finer.
- `td_cross_s50.png`: Cross. The old build leaves the effect out; this build shows crossing streaks, shorter than After Effects'.
- `1_turbulent.png` to `6_turbulent_smoother.png` (written by the check): each new word on the street picture.

## How this differs from After Effects

- After Effects' Offset is a point that starts at the layer's middle (960, 540 on a 1920 by 1080 layer). Ours is the slide from there, starting at 0, 0, so subtract the layer's middle from After Effects' number.
- The new words follow this program's own rules (document 21), read off one creator's stills, not fitted to After Effects' pixels.
- Not built:
  - Resize Layer. Ours always grows the layer when edges are transparent.
  - Antialiasing for Best Quality.
  - Complexity above 8 or between whole numbers.
- Ours has round Horizontal and Vertical patterns. After Effects' Vertical streaks run about 17 times longer down than across. Changing ours would change saved projects, so D-757 offers it to the owner instead.
