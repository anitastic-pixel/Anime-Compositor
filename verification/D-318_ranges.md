# D-318 / B-199: wider ranges for Exposure and Fractal Noise

From D-308, approved by the owner on 2026-10-04, after P-26. The tutorials set values this build refused: Exposure past 20 stops, and Fractal Noise's Brightness past -100 or Complexity past 8.

## What changed

- **Exposure**: -40 to 40 stops, After Effects' range. It was -20 to 20.
- **Fractal Noise**:
  - Complexity 1 to 20. It was 1 to 8.
  - Brightness -200 to 200. It was -100 to 100.
- Turbulent Displace has its own Complexity, still 1 to 8.
- An old file draws exactly as before.
- A value past the new ends is still refused with a sentence. In a file, it is kept as written and left out of the picture, with a warning.
- Not built: refusing only the one bad value of a change. The panel already sends one changed value at a time, so only that value is refused.

## Checks (cargo test)

| Check | Expected | Got |
|---|---|---|
| FX-LIMIT-011 to 014: Exposure 40, -40, 21, and an ease to 40 | every pixel as `Fixtures/limits/expected_limits_d318.json`, within 1e-4 | so (`verification/B-34_limits_table.md`) |
| FX-LIMIT-015, 016: Exposure 41 and -41 in a file | read, left out of the picture, warned | so |
| FX-FRACTAL-029 to 031: Complexity 20; Contrast 600 with Brightness -154; Brightness 200 | every pixel as `Fixtures/fractal_noise/expected_fractal_noise_d318.json`, within 2e-5 | so (`verification/B-71_fractal_noise_table.md`) |
| FX-FRACTAL-032, 033: Complexity 21 and Brightness -201 in a file | read, left out of the picture, warned | so |
| Commands: Exposure 41, -41, 128; Complexity 21; Brightness -201 | refused with a sentence | refused |
| Commands: Exposure 21, 40, -40; Complexity 20; Brightness 200 and -200 | taken | taken |
| FX-LIMIT-004, 006, 007; FX-FRACTAL-022, 024 (the old ends) | superseded: open with no warning and are drawn | so |
| Every other FX-LIMIT and FX-FRACTAL case | unchanged | pass |

Also still passing: the whole core suite and the app suite.

## Pictures (the test copy, never the owner's app)

A 640 x 360 white card. Pictures 1 to 4 put Fractal Noise on it at Size 60. Pictures 5 and 6 replace it with a black-to-white Gradient (top to bottom) and Exposure.

| Picture | Look for | Pass? |
|---|---|---|
| `D-318 pictures/1_complexity_8_the_old_top.png` | grey clouds, the old top of Complexity | pass |
| `D-318 pictures/2_complexity_20.png` | the same clouds with finer grain on top; the panel shows Complexity 20 | pass |
| `D-318 pictures/3_contrast_600_brightness_minus_154.png` | mostly black, with only the brightest cloud peaks showing as white blobs | pass |
| `D-318 pictures/4_brightness_200_all_white.png` | all white | pass |
| `D-318 pictures/5_ramp_exposure_minus_12.png` | Exposure -12: the ramp almost all black | pass |
| `D-318 pictures/6_ramp_exposure_20_49_taken.png` | Exposure 20.49, past the old top, is taken: the ramp all white; the panel shows 20.49 | pass |

Then setting Exposure to 41 was refused with "Exposure runs from -40 to 40 stops, and this is 41. Choose a value inside the range.", and the panel kept 20.49.

## For the owner to try

1. Put **Exposure** on a layer and type 30. It is taken; before, it stopped at 20.
2. Put **Fractal Noise** on a solid. Set Complexity to 20, then Contrast to 600 and Brightness to -150. Only the brightest specks remain, as in the tutorials.
3. Type 41 into Exposure. It is refused with a sentence.
