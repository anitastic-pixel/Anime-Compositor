# D-330 / B-210: 8 bpc (After Effects) working depth

From P-26. Tutorial 2 is drawn in After Effects' **8 bpc** until its step F. Its reflection keeps only the bolt's tips, blurs them wide with Fast Box Blur and lifts them 17 stops with Exposure. In 8 bpc After Effects keeps each pixel as a whole number from 0 to 255 between effects, so the blur's faint outer edge becomes 0 before Exposure sees it, and only a small glow lights. Here it lit a block about 600 pixels wide.

## Where the rule comes from (no After Effects needed)

- 8 bpc means 8 bits per channel: 256 steps from black to full. Anything fainter than half a step is 0.
- After Effects' manual: with no linear working space, 8 bpc and 16 bpc work in display colours, so a blur averages display values. Exposure is the exception: it always works in linear light (CS3 manual, p.402).
- Rounding alone was not enough. In linear light a blur's faint edge is brighter than in display colours, and it survives the rounding: 161,580 pixels stayed lit, against 16,572 when the blur averages display values. So both parts were built.

## What changed

- **Composition Settings, Working depth** has a third choice between Display and Float: **8 bpc (After Effects)**.
- In 8 bpc, after every effect, each pixel is rounded to 8 bits, and Gaussian Blur, Fast Box Blur, Directional Blur and Radial Blur average display colours.
- A layer with no effects is not touched.
- A composition inside another follows the outermost one, as Float does.
- A file is in one depth: 8 bpc and Float both at once is refused.
- Display stays the default. **No existing picture or project changes.** A project only gets an `eight_bpc` line when 8 bpc is chosen.

## Checks (cargo test)

Everything is in `verification/D-330_eight_bpc_table.md`, **37 of 37 pass**. Among them:

| Check | Result |
|---|---|
| FX-8BPC-001 to 005: every pixel matches values written by `tools/eight_bpc_reference.py` before the code existed | pass, tolerance 2e-5 |
| A file with both depths on, or with `eight_bpc` set to a word, is refused | pass |
| Set to 8 bpc, saved, opened again and undone | pass |
| A composition inside another follows the outermost one | pass, 3 ways |
| The preview draws the same picture | 0 of 255 apart |
| Cut into tiles, the frame is identical | pass |

The full crate and app suites were run afterwards (see the commit).

## Pictures

In `verification/D-330 pictures/`, tutorial 2's reflection drawn from three small blue tips (`tips.png`): Fast Box Blur horizontal then vertical, Solid Composite on black, Exposure +17.33, as the tutorial sets them.

- `built_1_display.png`: Display, as before. The blur's faint edge lights a wide glowing block (83,207 pixels).
- `built_2_float.png`: Float. The same wide block.
- `built_3_eight_bpc.png`: 8 bpc. The faint edge rounds to nothing: one small blue spot where the tips are (6,691 pixels), as in the tutorial.

## Known limit

An adjustment layer's effects are not rounded to 8 bits. The tutorials do not use one in 8 bpc.

Still open: after step F the tutorial works in 32 bpc and still shows a small glow, where both Float and this program's Display give a large one.

## For the owner to try

1. Open Composition Settings and set Working depth to **8 bpc (After Effects)**.
2. Add a small white solid, then Fast Box Blur radius 40 and Exposure +17 after it. Only a small glow should light.
3. Switch Working depth to Display. The glow should grow much wider.
4. Undo. It should go back to 8 bpc.

**Awaiting the owner's playtest.**
