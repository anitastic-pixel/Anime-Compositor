# B-652 playtest: Turbulent Noise's grain size and contrast match After Effects (D-772)

## What changed for you

Turbulent Noise now draws its clouds at After Effects' size and with After Effects' contrast.
- **Size:** at Size 100 one cell is 66.7 pixels across, the same as Fractal Noise since D-756.
  Before, it was 100 pixels, so our Turbulent Noise looked about 1.4 times too coarse next to
  seven tutorials.
- **Contrast:** at the same Contrast the greys now spread 0.64 times as far from the middle grey.
  Before, ours looked about 1.5 times too contrasty.

**An older project changes.** Every Turbulent Noise you saved before today now draws finer and
flatter clouds. Nothing in the file changes and nothing warns. To get the old look back, multiply
Size by 1.5 and Contrast by 1.5625 (both up to 1000).

## What to look at

Each sheet in `verification/B-652 pictures/` has three pictures with the same settings and the
same crop:
1. After Effects' own frame from a tutorial.
2. Ours before this change.
3. Ours after it.

Above each picture are its measurements:
- **spread:** how far the greys stray from their average, in levels of 255;
- **mean:** the average grey;
- **grain:** the size of a cloud, in pixels.

| Sheet | After Effects | Ours before | Ours after | Verdict |
| --- | --- | --- | --- | --- |
| `opal_0044_def_3way.png` (Opal, as added) | spread 16.0, grain 28.4 | 28.7, 38.8 | 16.4, 28.3 | matches |
| `opal_0052_c80_bm9_3way.png` (Opal, Contrast 80, Brightness -9) | spread 12.9, mean 106, grain 29.5 | 23.0, 99, 38.8 | 13.2, 103, 28.3 | matches |
| `jake_0436_def_3way.png` (Jake In Motion, as added) | spread 16.8, grain 28.1 | 25.4, 38.6 | 15.9, 25.6 | close (grain 9% fine) |

Two more cases are kept outside the repository:
- ECAbrams (as added): 15.9 / 25.5, then 24.8 / 38.5 before, then 16.3 / 25.8 after. Matches.
- Mike Murphy (inverted, used as a matte): spread and grain 17.1 / 27.3, then 24.8 / 38.5 before,
  then 16.3 / 25.8 after. Close: his average grey is 108 and ours is 128, which comes from
  settings of his that the tutorial does not show, not from this change.

On each sheet, the right-hand picture's clouds should be about the same size and as flat as the
left-hand picture's. The middle picture's clouds should look visibly bigger and more contrasty.
The pattern itself is not the same as After Effects': the random values are our own, and only the
size and contrast are matched.

## Try it

1. Add Turbulent Noise to a 1920 by 1080 solid, as added.
   - The clouds should look soft and mid-grey, without deep blacks or bright whites.
   - Set Noise Type to Block and Complexity to 1: about 29 squares fit across the frame
     (1920 / 66.7). Before this change, about 19 did.
2. Add Fractal Noise beside it with the same Size.
   - The squares are the same size in both.
   - The Turbulent Noise is flatter: Fractal Noise at Contrast 64 looks the same as Turbulent Noise
     at Contrast 100.
3. Open a project saved before today that uses Turbulent Noise.
   - It opens with no warning.
   - Its noise is finer and flatter than it was.

## Checks

`verification/B-652_turbulent_size_contrast_table.md`: 194 of 194 checks pass. They cover:
- the cell widths;
- Turbulent Noise at Contrast C matching Fractal Noise at 0.64 C, pixel for pixel;
- every Turbulent Noise fixture at its new values;
- an old project, which opens, saves back unchanged and draws with no warning;
- the card against the processor, within 1 level.
