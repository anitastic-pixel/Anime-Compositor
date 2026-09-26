# B-44: the GPU draws the viewer's picture

**B-44b, on 2026-09-26, changes the speed and fixes a fault described here: see `B-44b_gpu_uploads.md`.** The timing section below is B-44's own, kept as the before.

Accepted with D-100 (a) on 2026-09-25 ("accept D-100 (a)") and built the same day at the owner's "proceed". **Exports, every fixture and Full-quality checking are unchanged and still drawn on the CPU, byte for byte.** Only the picture in the viewer can come from the graphics card, and only when you switch it on.

## What changed

- **A new button beside Draft / Full resolution: Draw on GPU.** It is also in the command search (Ctrl+Shift+P, "Preview on the CPU or the graphics card"). It starts on CPU every time the window opens.
  - Pressed, it finds the graphics card and says which one it is using, for example "The preview is drawn on the graphics card: NVIDIA GeForce RTX 4070 Ti SUPER ... Exports are still drawn on the CPU."
  - If there is no usable card, it says so and the preview stays on the CPU.
  - Pressed again, it says "The preview is drawn on the CPU again."
- **The quality label under the viewer now ends with where the frame was drawn:** "drawn on CPU", "drawn on GPU", or "drawn on CPU, not GPU". The last one means the switch is on but the card did not draw this frame. The button's tooltip names the card.
- **What the card does:** the layering. That is each layer's position, rotation and scale, its opacity, its alpha matte, the four blend modes, and turning the result into the eight-bit picture the screen shows. Drawings are sent to the card once and kept there, up to 1 GiB, the least recently used going first.
- **What the CPU still does, even with the switch on:**
  - reading drawings and running every effect, as before
  - any frame with an adjustment layer, which is drawn wholly on the CPU
  - any frame the card fails on, or is too big for
- **None of these is silent.** Each such frame carries a new message, `GPU_PREVIEW_ON_CPU`, with the reason. It appears in the session log (P-19). D-101, proposed, is the decision that names it; document 28 had only messages for a GPU that refuses work, and this one refuses nothing.
- **The session log has two new stages:** "GPU: send drawings to the card" and "GPU: draw, encode and bring the picture back".

## How to judge it

- `B-44_gpu_preview_table.md`: **105 of 105 comparisons pass.** Each one sets the eight-bit picture the CPU draws beside the one the card draws. The rule is that no colour of any pixel is more than 1 level of 255 apart (D-100). The frames compared:
  - the four H-01 frames P-07 measured, at Full and at Draft
  - P-07's hard case: a layer turned 0.001 degrees about a point 8 million pixels off the canvas
  - 21 frames of the reference shot and 21 of the ten-layer fixture, every twelfth and the last, at Draft and at Full
  - the ten-layer fixture with every layer in each of the four blend modes, every other layer at 60% opacity
  - the reference shot at Draft with the card allowed to keep only one drawing at a time, so it must throw drawings away and send them again
  - the adjustment-layer fixtures, which must be byte-identical, because the CPU draws them
- The worst is P-07's hard case: 396 pixels differ, each by 1 level. `B-44 pictures/` has the CPU's picture, the GPU's, and a difference picture. The difference picture is black where the two agree, with a white square around every pixel where they do not, because a single pixel 1 level off could not be seen.
- `B-44_gpu_window_table.md`: **9 of 9.** The switch in the window, through the same function every frame on screen comes from:
  - it starts on CPU
  - pressing it names the card
  - frame 100 at Full and at Draft says GPU, and is within 1 level of the CPU's picture
  - pressing it again goes back and says so
  - the CPU's picture afterwards is identical to the one before
- `B-44_gpu_playtest.md`: for you, at the window.

## How fast it is: the honest answer

Measured, not assumed: `B-44_gpu_timing_table.md`, written by `cargo test --release --test b44_gpu_preview -- --ignored`.

- Machine: AMD Ryzen 9 9900X, 24 threads, Windows 11 Education 10.0.26200
- Card: NVIDIA GeForce RTX 4070 Ti SUPER, driver 610.74, through Vulkan
- Build: rustc 1.89.0, release profile

The table leaves out the work both paths share, reading the drawings and running the effects. It times only the part B-44 moves to the card. Medians in ms per frame:

| Shot | Quality | CPU | GPU, first pass | GPU, again |
|---|---|---|---|---|
| the reference shot | Draft | 2.48 | 10.77 | 11.45 |
| the reference shot | Full | 15.50 | 14.03 | 13.19 |
| the ten-layer fixture | Draft | 4.08 | 16.60 | 16.76 |
| the ten-layer fixture | Full | 28.74 | 36.21 | 35.83 |

**For layering alone, the card is not faster.** At Draft it is 3 to 4 times slower. At Full it is a little faster on the reference shot and slower on the ten-layer fixture. The reason is the trip to the card, not the drawing. The animation changes drawings on twos, so almost every frame brings about three drawings the card has not seen, 33 MB each at full size. A pass of 240 frames meets about 430 different drawings, more than the card keeps. At Draft the CPU's work is already tiny, so the trip is all cost.

That is why the switch starts on CPU, and it is why the next units are what they are.

## Why effects are next

The time that can be won is in the effects, and they are what B-44 hands back to the CPU. From D-99's measurements, on the same machine, at frame 100 of the reference shot with the effect on the whole-frame background at Full: reading the frame costs about 48 ms. On top of that, Radial Blur costs about 184 ms and Bloom's star about 140 ms. Full preview is where it shows, because D-99 already makes both cheap at Draft. An effect on the card runs on a drawing already there, and its result never has to come back until the frame is finished. So the order is the one document 15 already gives:

1. **Radial Blur**
2. **Bloom**
3. **Directional Blur**
4. **Gaussian Blur and Glow**

Each is its own unit, with its own tolerance measured and shown in pictures, and your acceptance, before it is used.

Two cheaper ways to make the trip smaller are known and not built:

- keeping drawings on the card at half precision, which halves every upload
- sending the eight-bit drawing and turning it into working colour on the card, a quarter of the bytes

Each changes what the card's picture is made from, so each would be measured the same way first.

## Limits

- **Measured on one machine and one card.** Another card or driver can differ by a level in other places. The 1-level rule is checked on this machine only.
- **Exports never use the card.** Rendering a file with the switch on gives the same bytes as with it off.
- **The card is found when you first press the button**, not at start-up, so a window that never uses it never touches the card.
- **The timings are for layering only.** A whole frame on screen also includes reading drawings and effects, the same on either path. The session log shows the whole frame.
