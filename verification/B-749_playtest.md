# B-749 playtest: in 8 bpc the graphics card lays adjustment layers and Light Wrap (D-869)

## What to try

1. Open a composition with a few picture layers. In Composition Settings choose **8 bpc (After Effects)**. Leave the viewer on the graphics card.
2. Add an **adjustment layer** on top and give it **Gaussian Blur** and **Levels**. Play. The viewer no longer says "The CPU drew this frame: its composition works in 8 bpc (After Effects) and has an adjustment layer or a Light Wrap". The picture looks the same as before: everything under the adjustment layer is blurred and levelled.
3. Change the adjustment layer's opacity, or draw a mask on it. Only the masked part is changed, as before.
4. Delete the adjustment layer and give one layer a **Light Wrap** instead. Play. It is drawn on the card too, and looks the same as before.
5. Set the Light Wrap's **Mix** below 100. The viewer says the CPU drew the frame, as it did before at every depth (this case stays on the processor).
6. Export a few frames. Exports are drawn on the processor as before; nothing in an exported file changes.
7. Playing is faster: on wave 8's reference shot at 1920 by 1080 in 8 bpc with an adjustment layer, about 94 ms a frame before and 24 now at Full.

## What the check found

- `tests/b749_eight_adjust_wrap.rs` (see `B-749_eight_adjust_wrap_table.md`), 5 of 5 checks:
  - Every fixture project holding an adjustment layer (70 projects) set to 8 bpc, first and middle frames, Full and Draft: 280 frames drawn on the card, the largest difference 1 level of 255, no pixel more than 1 off, 268 frames byte for byte.
  - Every fixture project holding a Light Wrap (24): 96 frames on the card, every one byte for byte.
  - The 88 frames the card still refuses (a Light Wrap with Mix under 100, or an adjustment layer with an effect the card does not draw) are the processor's to the byte. None is refused for being 8 bpc.
- The card checks b44, b65, b107, b155, b222, b642, b688 and b748 still pass (run on the build with B-750 on top; see `B-750_playtest.md`).
- Timings: `B-749_timing.md`.

## Pictures (`B-749 pictures/`)

- `frame_0030_cpu.png` and `frame_0030_card.png`: wave 8's 4-layer reference shot in 8 bpc under an adjustment layer with Gaussian Blur (6 px) and Levels, frame 30 at Full, drawn by the real program (`tools/app_snapshot.py`) on the processor and on the card. They are the same picture: on the full 1920 by 1080 frame the largest difference is 1 level of 255, the mean under 0.01. The card's frame says "drawn on GPU" and shows no "CPU drew this frame" note (`frame_0030_card.json`). Before B-749 the card refused this frame.
- Exports: because this frame (and 12 of the 280 fixture frames) is 1 level off somewhere, the card's frames are not byte for byte the processor's, so exports stay on the processor.

No three-way sheet: the picture does not change, only who draws it.
