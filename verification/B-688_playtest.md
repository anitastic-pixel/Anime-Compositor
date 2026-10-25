# B-688 playtest: 8 bpc (After Effects) frames are drawn on the graphics card (D-808)

## What to try

1. Make a composition, open Composition Settings and set the working depth to **8 bpc (After Effects)**.
2. Put a picture or a drawing in it and add **Cartoon**, **Bilateral Blur**, **Find Edges** or **Matte Choker** to it.
3. Play. The viewer no longer says "The CPU drew this frame: its composition works in 8 bpc (After Effects)", and it plays much faster than before: on the reference shot at Full, Cartoon with Detail Radius 40 and 16 shading steps took about 390 ms a frame before and about 20 ms now; Bilateral Blur at radius 40 about 400 ms before and 19 ms now.
4. The picture looks the same as before. Switch the depth to Display and back; the 8 bpc look (D-763: colours mixed in display values) is kept.
5. Put an **adjustment layer** or a **Light Wrap** in the 8 bpc composition. The frame is drawn by the processor again and the viewer says so, as before.
6. Add **Glow**, **Drop Shadow**, **Exposure**, **Fill**, **Solid Composite**, **Echo** or **Bloom** in 8 bpc. These still run on the processor in 8 bpc (each reads the depth itself); the rest of the layer's effects run on the card around them.

## What the check found

- `tests/b688_eight_card.rs` (see `B-688_eight_card_table.md`): every fixture project (4711) set to 8 bpc, drawn by the processor and by the card at its first and middle frames, Full and Draft. 18822 frames on the card; 228 effects, 2,214,118 pixels compared: the largest difference is 1 level of 255, no pixel is 2 or more off, and 139 effects are the processor's to the byte. The 16 projects saved in 8 bpc pass. The 8 frames the card refuses are the processor's exactly. 245 of 245 checks pass.
- `B-688_eight_card_settings_table.md`: W7's audit settings in 8 bpc at 1920 by 1080, frames 30 and 120, Draft and Full: 369 of 369 rows within 1 level, no pixel 2 or more off.
- B-642 (52 of 52) and D-330 (B-210) still pass; their tables now show the card drawing their 8 bpc rows with largest difference 0.

## Pictures (`B-688 pictures/`)

- `cartoon_8bpc_frame30_cpu.png` and `cartoon_8bpc_frame30_card.png`: the reference shot in 8 bpc with Cartoon (16 shading steps) on the drawing layer, frame 30 at Full, drawn by the real program on the processor and on the card. They are the same picture: 0 levels of difference on every pixel of the full frame. The card's frame shows no "CPU drew this frame" note.

No three-way sheet: the picture does not change, only who draws it.
