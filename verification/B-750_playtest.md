# B-750 playtest: in 8 bpc the graphics card draws Exposure, Drop Shadow, Fill, Solid Composite and Bloom (D-870)

## What to try

1. Open a composition with a few picture layers, choose **8 bpc (After Effects)** in Composition Settings, and leave the viewer on the graphics card.
2. Give one layer a **Drop Shadow** (softness 20), another a **Bloom**, and another **Exposure** (+0.5 stops) and **Solid Composite** (a blue at 30%). Play. The picture looks the same as before; the shadow, bloom and colours are where they were.
3. Change the Drop Shadow's or Solid Composite's colour, or add **Fill** to a layer. The colours look as they did before in 8 bpc.
4. Turn on Exposure's **Bypass Linear Light Conversion**. Still the same picture. Then set its **Mix** to 50: still the same picture; that one Exposure is handed to the processor, as before, and the rest stays on the card.
5. Add a **Glow** to a layer. It looks as before; in 8 bpc the Glow itself is still drawn on the processor (deferred, D-870), the rest of the frame on the card.
6. Export a few frames. Exports are drawn on the processor as before; nothing in an exported file changes.
7. Playing is faster: the shot in step 2 at 1920 by 1080 took about 162 ms a frame on the card at Full before and 31 now.

## What the check found

- `tests/b750_eight_card_effects.rs` (see `B-750_eight_card_effects_table.md`), 15 of 15 checks: every fixture project holding one of the five (Exposure 128, Drop Shadow 45, Fill 56, Solid Composite 31, Bloom 28), set to 8 bpc, first and middle frames, Full and Draft, as saved, with each of the five at Mix 50, and with every Exposure's Bypass on: the largest difference is 1 level of 255, no pixel more than 1 off. The frames the card still refuses are the processor's to the byte.
- W7's audit settings in 8 bpc at 1920 by 1080 (Bloom, Drop Shadow, Fill and Solid Composite as added and heavy): on the card, 9 of 9 within 1 level.
- Glow, tried on the card in 8 bpc: up to 255 levels off on the Glow fixtures, so it stays on the processor.
- The card checks b44, b65, b107, b155, b186, b222, b627, b642, b688, b748 and b749 still pass on this build.
- Timings: `B-749_timing.md`.

## Pictures (`B-750 pictures/`)

- `frame_0030_cpu.png` and `frame_0030_card.png`: wave 8's 4-layer reference shot in 8 bpc with Drop Shadow, Bloom, Exposure and Solid Composite (step 2), frame 30 at Full, drawn by the real program (`tools/app_snapshot.py`) on the processor and on the card. They are the same picture to the byte on the full 1920 by 1080 frame. The card's frame says "drawn on GPU" and shows no "CPU drew this frame" note (`frame_0030_card.json`).
- Exports: some fixture frames are 1 level off somewhere (Exposure 8 of 512, Fill 29 of 224, Solid Composite 6 of 124), so exports stay on the processor.

No three-way sheet: the picture does not change, only who draws it.
