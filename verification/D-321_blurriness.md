# D-321 / B-205: Gaussian Blur in After Effects' Blurriness

From D-308 (T4-1) and P-26. The same Gaussian Blur number gave a glow about three times wider here than in After Effects. The blur also stopped dead at three times its spread, so a strong brightening after it (Exposure +20 in tutorial 2) showed that stop as a hard box.

## Where the rule comes from (no After Effects needed)

Two programs that play After Effects' own files both convert Blurriness to a spread with the same number, 0.3:

- **lottie-web** (Airbnb), `player/js/elements/svgElements/effects/SVGGaussianBlurEffect.js`: `kBlurrinessToSigma = 0.3`, commented "Empirical value, matching AE's blur appearance".
- **Skia's Skottie** (Google), `modules/skottie/src/SkottiePriv.h`: `kBlurSizeToSigma = 0.3f`, commented "Close-enough to AE".

## What changed

- Gaussian Blur has a **Units** choice in Effect controls:
  - **Blurriness (After Effects)**: the number means what it means in After Effects. Its row is named **Blurriness**. The blur fades out over 6.5 times its spread, so even Exposure +20 after it finds no edge.
  - **Sigma (older projects)**: exactly the old rule. Its row is named **Radius σ (px)**.
- A Gaussian Blur you add from now on uses Blurriness. **Your existing projects do not change**: a file saved before this has no units, reads as Sigma, draws exactly as before and saves exactly as before.

## Checks (cargo test)

All in `verification/D-321_blurriness_table.md`, **45 of 45 pass**. Among them:

| Check | Result |
|---|---|
| FX-BLURRY-001 to 009: every pixel matches values written by `tools/blurriness_reference.py` before the code existed | largest difference 4.7e-6 or less (allowed 2e-5) |
| An old file (no units) draws exactly as before and is saved without units | pass |
| A Blurriness blur is saved as `units: "blurriness"`; a wrong word (`"pixels"`, `"Blurriness"`) is refused with a sentence | pass |
| The preview card draws it the same as the CPU | 0 levels of 255 apart, 4 files |
| Cut into tiles of 1 or 64 pixels, the frame is identical | pass |

The full crate and app suites were run afterwards (see the commit).

## Pictures

In `verification/D-321 pictures/`, a white square 24 pixels across, over black:

- `1_square.png`: no effect.
- `2_old_blur_20.png`: 20 in an older project (Sigma). Very soft; the middle falls to 125 of 255.
- `3_blurriness_20.png`: 20 in Blurriness, as After Effects reads it. The square is kept and its edges are softened; the middle stays at 245.
- `4_old_cut_exposure_20.png`: the old rule's blur at the same spread, then Exposure +20 in a Float composition. **A hard-edged box**: the problem P-26 saw.
- `5_blurriness_20_exposure_20.png`: Blurriness 20, then Exposure +20 in Float. **A round glow that fades out with no box.**

From the app's test copy (not your app):

- `6_app_new_blur_blurriness_20.png`: a 240-pixel white square with a newly added Gaussian Blur set to 20. Effect controls shows **Blurriness 20** and **Units: Blurriness (After Effects)**.
- `7_app_same_20_units_sigma.png`: the same with Units switched to **Sigma (older projects)**. The row reads **Radius σ (px)**, and the blur is far softer.

## For the owner to try

1. Add **Gaussian Blur** to a layer and set it to 20. The row should say **Blurriness**, and the softness should look like an After Effects Blurriness of 20 that you remember from tutorials.
2. Open an older project that has a blur. It should look exactly as it did, with Units set to **Sigma (older projects)**.
3. Optional: in a Float composition, put Exposure +20 after the blur. There should be no square box round the glow.

**Awaiting the owner's playtest.**
