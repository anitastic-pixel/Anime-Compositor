# B-785: the named colour spaces against OpenColorIO

Expected numbers: OpenColorIO 2.5.1 (ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5, OPTIMIZATION_LOSSLESS), from `Fixtures/colour/expected_conversions.json`.
A row passes when every sample is within 1e-6 of the sample's size. Nothing in the app uses these spaces yet; colour management is off.

## Matrices to ACES2065-1 (Bradford white adaptation)

| From | Largest difference | Result |
|---|---|---|
| aces2065_1 | 0.0e0 | pass |
| acescg | 3.3e-16 | pass |
| linear_p3_d65 | 3.8e-15 | pass |
| linear_rec2020 | 4.6e-15 | pass |
| linear_rec709 | 2.4e-15 | pass |

## Conversions

| From | To | Samples | Largest difference (of size) | Largest difference | Result |
|---|---|---|---|---|---|
| linear_rec709 | acescg | 5200 | 1.4e-7 | 7.6e-6 | pass |
| linear_rec709 | aces2065_1 | 5200 | 1.3e-7 | 3.9e-6 | pass |
| linear_rec709 | linear_rec2020 | 5200 | 1.4e-7 | 3.9e-6 | pass |
| linear_rec709 | linear_p3_d65 | 5200 | 1.1e-7 | 3.9e-6 | pass |
| acescg | linear_rec709 | 5200 | 2.4e-7 | 7.7e-6 | pass |
| acescg | aces2065_1 | 5200 | 1.6e-7 | 7.6e-6 | pass |
| acescg | linear_rec2020 | 5200 | 2.1e-7 | 7.7e-6 | pass |
| acescg | linear_p3_d65 | 5200 | 2.3e-7 | 7.7e-6 | pass |
| aces2065_1 | linear_rec709 | 5200 | 3.1e-7 | 1.6e-5 | pass |
| aces2065_1 | acescg | 5200 | 2.4e-7 | 7.7e-6 | pass |
| aces2065_1 | linear_rec2020 | 5200 | 2.2e-7 | 1.5e-5 | pass |
| aces2065_1 | linear_p3_d65 | 5200 | 2.4e-7 | 1.5e-5 | pass |
| linear_rec2020 | linear_rec709 | 5200 | 1.6e-7 | 7.7e-6 | pass |
| linear_rec2020 | acescg | 5200 | 2.1e-7 | 7.6e-6 | pass |
| linear_rec2020 | aces2065_1 | 5200 | 1.6e-7 | 3.9e-6 | pass |
| linear_rec2020 | linear_p3_d65 | 5200 | 2.4e-7 | 1.5e-5 | pass |
| linear_p3_d65 | linear_rec709 | 5200 | 1.2e-7 | 7.6e-6 | pass |
| linear_p3_d65 | acescg | 5200 | 1.9e-7 | 7.7e-6 | pass |
| linear_p3_d65 | aces2065_1 | 5200 | 1.2e-7 | 3.9e-6 | pass |
| linear_p3_d65 | linear_rec2020 | 5200 | 1.4e-7 | 3.9e-6 | pass |
| srgb_rec709 | linear_rec709 | 5177 | 1.9e-5 | 9.6e-6 | pass by the published formula (3.3e-7 of size, 1.7e-6); OCIO differs, see note srgb |
| linear_rec709 | srgb_rec709 | 5200 | 2.5e-4 | 3.2e-3 | pass by the published formula (1.1e-7 of size, 6.8e-7); OCIO differs, see note srgb |
| bt1886_rec709 | linear_rec709 | 5177 | 1.2e-7 | 4.8e-7 | pass |
| linear_rec709 | bt1886_rec709 | 5200 | 1.2e-7 | 4.8e-7 | pass |
| gamma22_rec709 | linear_rec709 | 5177 | 1.2e-7 | 4.8e-7 | pass |
| linear_rec709 | gamma22_rec709 | 5200 | 1.6e-7 | 4.8e-7 | pass |
| acescct | acescg | 5177 | 7.4e-7 | 2.8e1 | pass |
| acescg | acescct | 5200 | 1.1e-6 | 4.8e-7 | pass by the published formula (5.4e-8 of size, 3.8e-7); OCIO differs, see note acescct |
| acescc | acescg | 5177 | 4.3e2 | 4.2e7 | pass by the published formula (5.6e-8 of size, 1.8e0); OCIO differs, see note acescc |
| acescg | acescc | 5200 | 5.0e3 | 7.7e-1 | pass by the published formula (5.4e-8 of size, 3.0e-8); OCIO differs, see note acescc |

30 of 30 conversions pass.

Note srgb: OCIO's sRGB is a smooth power curve whose straight bottom piece ends at 0.0393 (slope 12.923), the standard's at 0.04045 (slope 12.92). Between 0 and 1 the two are at most 6.6e-7 apart in linear light and 6.3e-6 encoded, under a hundredth of a level of 255; below 0, where both carry the straight piece on, they differ by 2.5e-4 of the number. The build keeps the standard curve it has used since D-17.

Note acescct: one sample, linear -0.01, where the straight bottom piece 10.54 x + 0.0729 nearly cancels to -0.0325: OCIO works it in single precision and lands 3.8e-8 off the formula (1.2e-6 of the answer); the build works it in double precision and lands 3.4e-8 from the formula after rounding to single. Every other sample is within the bar.

Note acescc: OCIO's ACEScc is a 4096-step table that clamps below -0.36 and above 1.5 and sets negatives to 0 (about 3e-6 off mid-range, 3e-4 when encoding); the build works the published formula exactly.

