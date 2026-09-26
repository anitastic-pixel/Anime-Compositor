# B-53: Radial Blur's zoom smears outward only

Built on 2026-09-26 at the owner's "improve the zoom/radial blur to behave like AE", against the B-53 entry in document 15. The owner named the direction of the streaks as what differs. D-110, proposed, is the rule. The new values of the fixtures it changes were written and committed before this code.

## What changed

**Before (D-95):** a zoom smeared each part of the picture both ways along the line through the centre, half toward the centre and half away from it.

**Now (D-110):** a zoom smears only outward, away from the centre, as After Effects' zoom does. It looks like a camera zooming in while the shutter is open.

These stay the same:

- **Amount** keeps its meaning: the length of the smear, in per cent of the distance from the centre.
- **Spin** is unchanged.

**A side effect:** a zoom about a centre inside the picture no longer fades at the picture's edges, even with Edges at Transparent. Everything it smears in comes from nearer the centre, inside the picture.

## How to judge it: the pictures

`verification/B-53 pictures/` holds frame 0 of the reference shot, drawn by the CPU as an export is:

| Picture | What it is |
|---|---|
| `plain.png` | the shot as it is |
| `zoom_before.png` | a zoom of 30 about the middle of the background, under the old rule (D-95) |
| `zoom_after.png` | the same zoom under the new rule (D-110) |

**What you should see:**

- In `zoom_after.png` everything streaks away from the middle of the frame.
- The water spreads down over the grass at the bottom.
- The clouds and snow streak up and outward.
- Nothing is pulled in toward the middle.
- In `zoom_before.png` the streaks run both ways, so the picture looks smeared in place rather than zooming.
- In both, the yellow and blue squares, the red dot and the green square are separate layers without the blur, and stay sharp.

`zoom_before.png` was drawn once by `tests/b53_zoom.rs` with the old rule, and is not redrawn. The other two are redrawn on every test run.

## The fixtures

The Python reference (`tools/radial_blur_reference.py`) sets the expected values. The change moved four existing fixtures, and document 25 records by how much:

| Fixture | What it now pins | Result |
|---|---|---|
| FX-RADIAL-002 | a zoom of 30 about the middle smears the block outward, and the column just inside the line stays empty | pass, within 2.5e-7 |
| FX-RADIAL-005 | about a point between a line and a block, both smear away from it, and the three columns between them stay empty | pass, within 2.5e-7 |
| FX-RADIAL-007 | a zoom of 100 reaches a pixel four past the block that a zoom of 30 does not | pass, within 2.5e-7 |
| FX-EDGES-008 | a zoom of 40 about the top left corner, edges repeated, stays fully covered | pass, within 1.1e-7 |

All the other radial and edge fixtures are unchanged and still pass:

- `verification/B-39_radial_blur_table.md` covers FX-RADIAL-001 to 018.
- `verification/B-52_edges_table.md` covers FX-EDGES-001 to 012.

## On the graphics card

The card takes the zoom's sample positions from the same place the CPU does, so it needed no change of its own. The results:

- `verification/B-46_gpu_radial_table.md`: 188 of 188 pass, within D-103's 1 level of 255.
- `verification/B-52_gpu_edges_table.md`: 108 of 108 pass.

## What else moved

- **B-52's picture of Radial Blur is now a spin of 30.** The zoom it used no longer fades at the edges, so it could no longer show what Repeat Edge Pixels fixes. With the spin, 413,548 pixels are see-through with Transparent edges and 0 with Repeat Edge Pixels.
- **Projects saved with a zoom** open with the new direction of streaks. Nothing else in them changes.
