# B-724: a visual comparison toolkit for tutorial checks (tools/visual_compare.py)

Wave 8, lane w8h, under the owner's instruction of 2026-10-25: "also, review deeply the tutorial results for everything, the glow from motion nations, make sure the behaviors are more 1 to 1 than before. use better visual analytical techniques to improve at visual analysis of these behaviors."

Earlier waves judged an After Effects frame against ours mostly by one number (grey RMS or a median level difference). This tool measures the same pair several ways, so a verdict says *what* differs:

- colour: CIEDE2000 percentiles (perceptual colour difference; under 1 is not visible, about 2 is just visible), the hue and chroma of added light, a tone curve;
- structure: SSIM and multi-scale SSIM, the correlation of edges (gradient) and an edge-match score;
- spread: the shape of a glow, blur, shadow or feather, fitted as one Gaussian, an exponential and a sum of Gaussians (so a long tail shows up as a tail, not as "a bit off");
- motion: optical flow between two frames of each (how far and which way pixels moved);
- registration: the scale and offset of one picture inside another;
- the recording's own noise floor, so a gap smaller than the video's noise is called "can't tell";
- a sheet: After Effects / ours / a difference heat map / a fall-off plot, under 400 KB.

It changes nothing in the app. Its dependencies are numpy, SciPy, Pillow and OpenCV (all permissive), already installed in the user site packages.

## The named test: `python -I tools/visual_compare.py selftest`

Each check makes a picture whose answer is known, runs the measure, and compares. Run on 2026-10-25 (Windows 11, Python 3.13), exit code 0:

| Check | Known answer | The tool's answer | Passes |
| --- | --- | --- | --- |
| A picture against itself: largest CIEDE2000 | 0 | 0.0 | yes |
| ... SSIM / MS-SSIM | 1 / 1 | 1.0 / 1.0 | yes |
| ... edge correlation / edge match | 1 / 1 | 1.0 / 1.0 | yes |
| ... tone curve gap (levels) | 0 | 0.0 | yes |
| ... spectrum ratio | 1 | 1.0 | yes |
| ... optical flow, 90th percentile (px) | 0 | 8e-06 | yes |
| ... registration (scale, dx, dy) | 1, 0, 0 | 1.0, 1e-07, 2e-07 | yes |
| A Gaussian blur of sigma 3 recovered | 3.0 (amplitude 1) | 2.999998 (1.0) | yes |
| ... by the sum of Gaussians, effective sigma | 3.0 | 3.000001 | yes |
| 0.6 G(2) + 0.4 G(8): weights at sigma 1, 2, 4, 8, 16 | 0, 0.6, 0, 0.4, 0 | 0.0, 0.6, 0.0, 0.4, 0.0 | yes |
| ... one Gaussian fits it worse (rms ratio) | more than 10 | 1.2e10 | yes |
| Sigma 3 through 8-bit rounding and 2-level noise | 3.0 within 0.05 | 2.989541 | yes |
| Sigma 3 through JPEG quality 50 | 3.0 within 0.1 | 2.98588 | yes |
| An exponential kernel, lambda 4 | 4.0 | 3.999561 | yes |
| ... one Gaussian fits it worse (rms ratio) | more than 5 | 1069 | yes |
| Noise floor of 2-level noise (levels) | 2 within 0.25 | 1.954309 | yes |
| A half-size copy moved by (13.3, -7.2) | 0.5, 13.3, -7.2 | 0.50014, 13.333, -7.311 | yes |
| Edge profile of a sigma-3 half plane against erfc | error under 0.02 | 0.004591 | yes |
| Sheet size (bytes) | under 400 000 | 51 396 | yes |
| CIEDE2000, Sharma's published pair 1 | 2.0425 | 2.04246 | yes |

`selftest passed (23 of 23)`

## What it found on real tutorial frames (wave 8 re-review, pictures in `verification/B-724 pictures/`)

| Sheet | What it shows |
| --- | --- |
| glow_motionnations_i1_measured.jpg | Motion Nations (https://youtu.be/Yxa3j02AyW8 at 1:51), Glow Threshold 60, Radius 40, Intensity 1, re-rendered on the 1080p frame: the spread's sigma is 0.374 x Radius in After Effects and 0.391 in ours; the added light is 1.02 x ours. Matches. |
| glow_motionnations_i0.2_measured.jpg | The same at Intensity 0.2 (1:54): the spread still matches (0.367 against 0.391) but After Effects' added light is 0.63 x ours. Ours is too strong below Intensity 1 here; one source, so not changed (see the wave 8 report). |
| twirl_dope_a58_measured.png | Twirl (Dope Motions, Angle 58): the angle each ring turns was read by aligning rings of the untwirled and twirled pictures. After D-744, ours is within 5 degrees of After Effects' median (before: 16.5). |
| gaussian_bullo_measured.png | Gaussian Blur 300 (Bullo): fitted blur sigma 141.3 px in After Effects, 141.8 in ours (ratio 0.996). The earlier "After Effects is about 13% narrower" reading is not borne out by a kernel fit. |
