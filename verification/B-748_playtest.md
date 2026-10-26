# B-748 playtest: the graphics card draws every layer blend mode (D-868)

## What to try

1. Make a composition with a few layers. Set one layer's mode to **Overlay**, another to **Lighten**, **Difference**, **Hue** or **Color**, and leave the viewer on the graphics card.
2. Play. The viewer no longer says "The CPU drew this frame: a layer has a blend mode the GPU does not draw." The picture looks the same as before.
3. Turn **Preserve Underlying Transparency** (the T switch) on for one of those layers. It still keeps the layers beneath where they are.
4. Set a layer to **Stencil Alpha** or **Stencil Luma**. Everything beneath shows only where the stencil is; it is drawn on the card too.
5. Scale a Stencil layer to 0%. The frame goes clear, as it did before. (The card leaves this case to the processor; when it does, the viewer says "The CPU drew this frame: a stencil layer has collapsed to nothing.")
6. Open Composition Settings, set **8 bpc (After Effects)**, and do steps 1 to 4 again. Same result: the modes mix in display values, as before.
7. At Full, a shot with 10 to 50 layers and some of these modes plays faster than before (on wave 8's 10-layer reference shot about 209 ms a frame before and 80 now; 50 layers 1064 before and 645 now). At **Draft** some shots are a little slower than before (25 layers: about 244 before, 312 now), because at Draft the card is already slower than the processor on these shots even with every layer Normal. That is written down as a gap in D-868.

## What the check found

- `tests/b748_card_blend_modes.rs` (see `B-748_card_blend_modes_table.md`), 113 of 113 checks:
  - The 10 fixtures saved with a mode past Add, as saved and in 8 bpc, Full and Draft: drawn on the card, the largest difference 1 level of 255. FX-LBLEND-009, a mode this build does not know, is still refused when it opens.
  - Each of the sixteen modes, Preserve Transparency off and on, linear and 8 bpc, on a 512 by 512 picture of mixed pixels: the largest difference is 1 level, and no pixel is more than 1 off.
  - 1718 fixture projects with their upper layers set to each of the twelve new modes, as saved and in 8 bpc: the largest difference is 1 level, and no pixel is more than 1 off. The 193 frames the card still refuses (for other reasons, such as an adjustment layer) are the processor's to the byte.
  - A Stencil Alpha or Stencil Luma scaled to nothing: the card refuses it and the processor's frame is clear.
- `tests/b627_layer_blend_modes.rs` and `tests/b186_blend_modes.rs` now also draw their cases on the card: within 1 level.
- The card checks b44, b65, b107, b155, b222, b642 and b688 still pass.
- Timings: `B-748_timing.md`.

## Pictures (`B-748 pictures/`)

- `frame_0030_cpu.png` and `frame_0030_card.png`: wave 8's 10-layer reference shot (Overlay, Lighten, Multiply and Screen among its layers), frame 30 at Full, drawn by the real program (`tools/app_snapshot.py`) on the processor and on the card. They are the same picture: on the full 1920 by 1080 frame the largest difference is 1 level of 255, the mean under 0.01. The card's frame says "drawn on GPU" and shows no "CPU drew this frame" note (`frame_0030_card.json`). Before B-748 the card refused this frame.

No three-way sheet: the picture does not change, only who draws it.
