# B-635 playtest: Fractal Noise's cell is After Effects' size (D-756)

## What changed for you

Fractal Noise now draws its clouds and blocks at After Effects' size. At Scale (Size) 100 one cell
is 66.7 pixels across, as in After Effects; before, it was 100 pixels, so our noise looked about
1.5 times too coarse next to every tutorial.

**An older project changes.** Every Fractal Noise you saved before today now draws its cells at two
thirds of the size they had: finer, with about 2.25 times as many cells on the same layer. The
greys, the contrast and the movement are the same. Nothing in the file changes and nothing warns.
To get the old look back, multiply Size (or Scale) by 1.5, up to 1000.

**Turbulent Noise has not changed.** Its size is being checked against After Effects separately.

## What to look at

In `verification/B-635 pictures/`, each sheet shows After Effects' own frame from a tutorial,
then ours before this change, then ours after it. All three use the same settings and the same
crop. Under each picture is the cell size measured on that picture.

| Sheet | After Effects | Ours before | Ours after |
| --- | --- | --- | --- |
| `B-635_ak_block2.png` (Akshay Morey, Block, Complexity 2) | 66 px | 99 px | 66 px |
| `B-635_mason_ts_block.jpg` (Mason Rippel, Turbulent Smooth, Block, 4K) | 66 px | 98 px | 66 px |
| `B-635_cow_block1.png` (Creative COW, Block, Complexity 1) | 68 px | 100 px | 68 px |

Two more cases were measured the same way, with sheets kept outside the repository: School of
Motion (Block, Contrast 168) gave 68 / 99 / 68 and Ben Marriott (Block, Complexity 6) gave
66 / 99 / 66.

On each sheet, the right-hand picture's squares should be the same size as the left-hand
picture's. The middle picture's squares should look visibly bigger. The pattern itself is not the
same as After Effects': the random values are our own, and only the size is matched.

## Try it

1. Add Fractal Noise to a 1920 by 1080 solid and set Noise Type to Block and Complexity to 1.
   About 29 squares fit across the frame (1920 / 66.7). Before this change, about 19 did.
2. Set Size to 150. The squares are now exactly 100 pixels, the size Size 100 used to give.
3. Add Turbulent Noise to another solid with Block, Complexity 1 and Size 100. Its squares are
   still 100 pixels, as before.
4. Open a project you saved before today that has Fractal Noise. It opens without a warning, and
   its noise is finer than it was.

## The checks

`verification/B-635_fractal_size_table.md`, 104 of 104 checks pass. It covers:

- the cell counted in pixels (66 or 67 at size 100, 100 at size 150, 20 at size 30);
- Turbulent Noise still at 100;
- every Fractal Noise fixture at its new reference numbers;
- an old project that opens, saves back unchanged and draws with no warning;
- the card against the processor, within 1 level.

## Known differences from After Effects (not fixed here)

- Turbulent Noise's size is checked separately (wave 3).
- The Dynamic type, Strings, Turbulent Smooth's levels and Terrain still differ from After
  Effects. This change fixes only the cell size.
