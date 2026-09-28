# B-115: Motion Tile on the card, by hand

Built on 2026-09-28 against the B-115 entry in document 15 (D-178). **Awaiting the owner's playtest.**

`verification/B-115_gpu_motion_tile_table.md` is the comparison: every fixture frame of Motion Tile, and the reference shot with it, drawn by the CPU and by the card, 238 of 238 within 1 level of 255. The three pictures of the worst frame are in `verification/B-115 pictures/`. No timing was taken: the release build could not be made during this session, and none is claimed. Motion Tile costs almost nothing on either side (P-22 measured 0.0 ms on the CPU).

This replaces item 7 of `verification/B-107_playtest.md`: Motion Tile no longer stays on the CPU.

## Before you start

- Use the release build. It opens on the reference shot.
- Leave **Draw on** at **Auto**.

## What to check

1. **The same picture.** Select a layer and add **Motion Tile**. Set Output Width to 200 and Output Height to 150. Switch **Draw on** between **CPU** and **GPU** a few times: the picture should not change in any way you can see.
2. **Mirror.** Turn Mirror on and off on **GPU**. With it on, every other copy is turned over, so the copies meet as reflections; with it off they repeat side by side. Each should look the same on CPU.
3. **Width and height apart.** Set Output Width to 300 and leave Output Height at 100. The layer should grow sideways only, on GPU exactly as on CPU. Then the other way round.
4. **Left of the drawing.** Look at the copies to the left of and above the drawing. These were the ones the card first put two columns out; they should line up with the drawing's edge on GPU as on CPU.
5. **Dragging.** On **GPU**, drag Output Width slowly from 100 to 400. The picture should follow your hand, with no old size showing.
6. **Draft.** Switch to **Draft** and repeat step 1. CPU and GPU should still match.
7. **Export is untouched.** An export is the same file whatever **Draw on** says.

## What to report

- Anything that looks different between CPU and GPU, with the settings and the frame number.
- Any copy that sits one or more pixels away from where the CPU puts it.
