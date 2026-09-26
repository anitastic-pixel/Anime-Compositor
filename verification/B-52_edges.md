# B-52: Repeat Edge Pixels

Built on 2026-09-26 at the owner's "proceed, add to other blurs if needed", against the B-52 entry in document 15. It follows the owner's question, "how do we get radial blur to not cause transparency from the edges". D-109, proposed, is the rule. Its fixtures, FX-EDGES-001 to 012, were written and committed before this code.

## What it does

Radial Blur, Blur (Gaussian) and Directional Blur each get a new setting, **Edges**, with two choices:

- **Transparent.** This is how the blurs have always worked. A blur that reaches past the edge of the layer finds nothing there, so a picture that fills the frame fades to see-through at its edges.
- **Repeat Edge Pixels.** A blur that reaches past the edge takes the edge pixel instead, so the picture stays solid right to its edges. After Effects uses the same name for the same thing.

A new blur starts at **Transparent**, and every existing project opens and saves exactly as before. A layer using Repeat Edge Pixels does not grow past its own edges, because nothing is drawn beyond them.

**What does not get the switch:**

- Bloom and Glow. They add light rather than smear the picture, so their light fading at the edge is correct.
- Selective Colour Blur. It only reads the chosen pixels inside the drawing.
- Every other effect, because none of them reads outside the drawing.

## How to judge it: the pictures

`verification/B-52 pictures/` holds frame 0 of the reference shot with one blur on the background, drawn by the CPU as an export is. Each blur is shown twice, first with **Transparent** and then with **Repeat Edge Pixels**. Each picture is laid over a grey checkerboard, so the checkerboard shows through wherever the picture is see-through.

| Blur | Before (Transparent) | After (Repeat Edge Pixels) |
|---|---|---|
| Radial Blur, spin 30 about the middle | `radial_transparent.png` | `radial_repeat.png` |
| Blur (Gaussian), sigma 10 | `gaussian_transparent.png` | `gaussian_repeat.png` |
| Directional Blur, 45 degrees, 60 long | `directional_transparent.png` | `directional_repeat.png` |

**What you should see:** in each "before" picture, the checkerboard shows through a faded band around the border. In each "after" picture there is no checkerboard at all, and the picture runs solid to every edge. The middle of the picture is the same in both.

The Radial Blur here is a spin since B-53. Under D-110 a zoom about a centre inside the picture no longer fades at its edges at all, so a zoom could no longer show the difference. The first pictures used a zoom of 20, which left 320,748 pixels see-through under D-95's rule.

The count behind the pictures, from `verification/B-52_pictures_table.md` (3 of 3 pass):

| Blur | See-through pixels, before | See-through pixels, after |
|---|---:|---:|
| Radial Blur | 413,548 | 0 |
| Blur (Gaussian) | 147,228 | 0 |
| Directional Blur | 120,384 | 0 |

## The fixtures

`verification/B-52_edges_table.md`: **60 of 60 checks pass.**

- **FX-EDGES-001 to 009**, every frame. The build's pixels match the Python reference, `tools/edges_reference.py`, to within about 0.0000002. The allowed difference is 0.00002.
- **FX-EDGES-010 to 012**, the misspelt settings (`"wrap"`, `"Repeat"` and an empty word). Each is kept in the file as written, and the blur is left out with the warning `EFFECT_PARAMETER_INVALID`, both when the project opens and at the frame.
- **How far each blur reaches.** With Repeat Edge Pixels the layer does not grow. With Transparent it grows exactly as before.
- **Saving.**
  - Every fixture saves and reopens unchanged.
  - A written `"transparent"` is not written back, so a file saved now matches one saved before D-109.
  - A setting written as a number, or as keyframes, is refused with a message.
- **Commands.** A misspelt word is refused with a message that names the blur. A correct one is taken, and it can be undone.
- **Tiles.** The frame is the same however it is cut into tiles.

## On the graphics card

`verification/B-52_gpu_edges_table.md`: **108 of 108 checks pass** on the NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.74, Vulkan).

- The rule is that no channel of any pixel may be more than 1 level of 255 away from the CPU's picture. This is the tolerance each of the three blurs already has on the card (D-103, D-106 and D-107).
- The checks cover:
  - FX-EDGES-001 to 009 at every frame, at Full and Draft. Most are the same bytes on both. FX-EDGES-005 differs by 1 level at 2 pixels.
  - The reference shot at frames 0 and 239, at Full and Draft, with each blur repeating its edges on the background.
- The worst is the reference shot with Directional Blur at frame 0, Full. It differs by 1 level at 21,541 of 2,073,600 pixels (about 1%).
- Its pictures are `card_cpu.png`, `card_gpu.png` and `card_difference.png`. In the difference picture, black means the two agree, and a white square marks each pixel where they differ.

## What it does not change

- **Exports and existing projects.** Every blur already in a project stays on Transparent, which is exactly the old behaviour. Every earlier fixture and table is unchanged: the full test run passes.
- **Bloom and Glow.** They use the same blur passes, but always with the edges transparent.

## Not run

- No timings were taken. The switch does no extra work that would change speed noticeably, but this was not measured.
- The owner's playtest, `verification/B-52_playtest.md`.
