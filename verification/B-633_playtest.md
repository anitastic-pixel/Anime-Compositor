# B-633 playtest: Mosaic gains Horizontal and Vertical Blocks and Sharp Colors (D-754)

## What to try

1. Add a new **Mosaic** to a picture. It starts with Mode **Count**, **Horizontal Blocks 10**, **Vertical Blocks 10** and **Sharp Colors** off: the layer is cut into 10 by 10 even blocks, whatever its size, as in After Effects.
2. Change Horizontal to 20 and Vertical to 5. The blocks become narrow and tall, and they still fill the layer exactly, with no part-block at the edge.
3. Turn **Sharp Colors** on. Each block takes the colour of its middle pixel instead of the average, so the blocks look crisper and more varied.
4. Set Mode to **Size**. The old square-block Mosaic comes back, Size in pixels, exactly as before.
5. Open a project saved before this build that uses Mosaic. It looks exactly the same as before: it opens in Size mode with its own Size, and Sharp Colors off.
6. Try typing 0 or 1001 blocks. The panel refuses them (1 to 1000).

## What the check found

- Fixture check `tests/b633_mosaic_counts.rs`: 166 of 166 (see `B-633_mosaic_counts_table.md`). FX-MOSAIC-021 to 037 are new; 001 to 020 are unchanged, and their old pictures still match (`tests/b87_mosaic.rs`, `B-87_mosaic_table.md`).
- An old file (FX-MOSAIC-001) opens as Size mode with Sharp Colors off, and saving it writes only Size, as before.
- The graphics card's pictures match the CPU's: B-107 table, 7432 of 7432 checks, Mosaic 376 of 376.
- App tests 114 passed, including `d754_new_mosaic_counts` (a new Mosaic starts at 10 by 10).

## Against the tutorials (three stills)

All panels were drawn by the real app on the CPU. The measure is the mean difference from the After Effects still over the frame, in levels from 0 to 255; lower is closer. "Before" is the closest setting the old Size-only Mosaic had.

| Still | Before (old Size) | After (10 by 10) | Verdict |
| --- | --- | --- | --- |
| Jake In Motion 0:18 | Size 64: 5.40 | 4.08 | closer; the grid now matches After Effects' 10 by 10 |
| Gabianco 0:18 | Size 38.3: 7.08 | 4.27 | closer; the blocks now fit the layer evenly, with no cut-short row |
| Aleksander Vlad 0:24 | Size 33.3: 10.81 | 10.81 | the same: the grid already matched; the difference is the block colour (below) |

With Sharp Colors on, Jake reads 9.09. That is expected: the tutorial had Sharp Colors off, and the sheet shows what the switch does.

**A gap kept on purpose.** Mosaic averages each block in linear light, where After Effects averages the displayed values. That makes our blocks brighter where a block mixes light and dark: the Vlad ring's blocks come out light grey where After Effects' are mid grey. The brief said not to change the colour averaging, because Mosaic is an inspired effect (D-144) and changing it would change every old project. It is listed under Gaps in the effects list.

## Pictures (`B-633 pictures/`)

Each sheet shows After Effects, the picture fed to ours, before and after.
- `mosaic_jake.png`: also shows 10 by 10 with Sharp Colors on.
- `mosaic_gab.png`
- `mosaic_vlad.png`
