# D-326 / B-207: Fractal Noise's Brightness from -1000 to 1000

From P-26. Tutorial 3 (the glitch title) keys Fractal Noise's Brightness to -204. After Effects takes that when it is typed in, past its slider's -200. D-318 stopped at -200, so the replay had to use -200.

## What changed

- Fractal Noise's **Brightness** now runs from **-1000 to 1000**. No After Effects limit was found, so the range is this program's choice, beside Contrast's 0 to 1000.
- **Your existing projects do not change**: every value they could hold is still in range and draws as before.

## Checks (cargo test)

All in `verification/B-71_fractal_noise_table.md`, **125 of 125 pass**. Among them:

| Check | Result |
|---|---|
| FX-FRACTAL-034: contrast 1000, brightness -204, every pixel as written by `tools/fractal_noise_d326_reference.py` before the code took it | pass, largest difference 3e-8 |
| FX-FRACTAL-035, 036: brightness 1000 all light, -1000 all dark | pass |
| FX-FRACTAL-037, 038: -1001 and 1001 in a file are read, left out of the picture and warned of | pass |
| A command setting -1001 is refused with a sentence; 1000 and -1000 are taken | pass |
| FX-FRACTAL-033 (-201, refused before) now opens with no warning | pass (superseded) |
| The preview card draws FX-FRACTAL-034 to 038 the same as the CPU (`verification/B-76_gpu_fx_table.md`, 2560 of 2560) | 50 of 50 frames pass |

The full crate and app suites were run afterwards: everything passes except `zz_scratch_g2`, a scratch measurement that failed before P-26 began; the app suite has 91 passing and 5 set aside.

## Pictures

In `verification/D-326 pictures/`, from the app's test copy (not your app): a card with Fractal Noise, size 60, complexity 8, contrast 1000.

- `1_contrast_1000_brightness_minus_200.png`: Brightness -200, D-318's old bottom.
- `2_brightness_minus_204_taken.png`: Brightness **-204**, taken. Effect controls shows -204; the white specks are a little smaller.
- `3_brightness_minus_300_fewer_specks.png`: Brightness -300. Only a few specks are left.

Typing -1001 is refused with "Fractal Noise's brightness runs from -1000 to 1000, and this is -1001."

## For the owner to try

1. Add **Fractal Noise** to a solid and type Brightness **-204**. It should be taken.
2. Type **-1001**. It should be refused with a sentence, and the value should stay as it was.

**Awaiting the owner's playtest.**
