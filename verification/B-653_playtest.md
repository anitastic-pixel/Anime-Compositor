# B-653 playtest: Path Stroke's line is as wide as After Effects' (D-773)

## What changed for you

Path Stroke's Brush Size now works like After Effects': it is the brush's radius, so the line is
**twice Brush Size across**.
- Brush Size 10 draws a line 20 pixels wide. Before, it drew 10.
- The dabs follow: Spacing 100 puts one dab every 20 pixels at Brush Size 10. Before, every 10.
- Seven tutorials from seven creators measured After Effects' line at 1.64 to 2.2 times Brush Size
  (median 1.91). Ours was 1 times.

**An older project changes.** Every Path Stroke you saved before today now draws twice as wide.
Nothing in the file changes and nothing warns. To get the old look back, halve Brush Size.

Scribble, Write-on and the other effects that draw along paths are not changed.

## What to look at

Each sheet in `verification/B-653 pictures/` has three pictures, all at the same scale in
composition pixels:
1. After Effects' own frame from a tutorial.
2. Ours before this change.
3. Ours after it, with the same Brush Size and Hardness.

Above each picture is the line's width in pixels and as a multiple of Brush Size.

| Sheet | After Effects | Ours before | Ours after | Verdict |
| --- | --- | --- | --- | --- |
| `smertimba_0135_s25_3way.png` (Smertimba, Size 25, Hardness 100) | 47.7 px (1.91x) | 24.3 (0.97x) | 49.9 (1.99x) | matches |
| `p75_0300_s20_zoom_3way.png` (P75, Size 20, Hardness 79) | 33.6 px (1.68x) | 19.1 (0.95x) | 37.1 (1.86x) | close (ours 10% wide) |
| `nichan_0602_s8_3way.png` (NiChaN, Size 8, Hardness 79) | 13.2 px (1.64x) | 8.0 (1.00x) | 15.8 (1.97x) | close (ours 20% wide) |

Two more cases are kept outside the repository:
- Morris Barrier (Size 24.9, Hardness 100): 48.8 px in After Effects, 24.3 before, 49.9 after. Matches.
- P75 at 2:46, the same line unzoomed: 33.6, then 19.1, then 37.1. Close.

On each sheet the right-hand line should be about as thick as the left-hand one, and the middle
line about half as thick. At Hardness 100 they match. At After Effects' default Hardness 79 ours
is still 10 to 20% wider: a soft edge narrows After Effects' line more than ours. Only three
tutorials show that, too few to change it.

## Try it

1. Draw a box mask on a solid and add Stroke (Path Stroke). Set Brush Size 10, Hardness 100,
   Paint Style On Transparent.
   - The white line is 20 pixels across. Before this change it was 10.
2. Set Spacing 100.
   - The line breaks into round dots, each touching the next, one every 20 pixels.
3. Open a project saved before today that uses Path Stroke.
   - It opens with no warning.
   - Its line is twice as wide as it was. Halve Brush Size and it looks as it did.

## Checks

`verification/B-653_stroke_width_table.md`: 134 of 134 checks pass. They cover:
- the line's width at Brush Size 2, 10 and 25: 4, 20 and 50 pixels;
- the dab spacing at three settings;
- every Path Stroke fixture at its numbers;
- an old project, which opens, saves back unchanged and draws twice as wide with no warning;
- the card against the processor, within 1 level.
