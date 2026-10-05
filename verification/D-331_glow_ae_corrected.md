# D-331 / B-209: Glow in After Effects' numbers, corrected

From P-26's tutorial replays after D-327. Corrects D-322 (2026-10-04), which made Glow in After Effects units far too bright.

## What was wrong

D-322 read the Creative COW thread [Glow Effect and transparent background mechanics](https://creativecow.net/forums/thread/glow-effect-and-transparent-background-mechanics/) as: the glow's colour is **GI × (GT/100) + GI × 16 × (1 − GT/100)** times the light (GI Glow Intensity, GT Glow Threshold), laid on with its covering untouched. At threshold 0 that made the colour 16 times the intensity.

Played against After Effects' own frames in two tutorials, that is far too bright:

- **Tutorial 3** (three Glows at threshold 5, intensity 0.2, 0.1 and 0.3, on an opaque copy of the text): After Effects shows a faint blue halo round the word. D-322 drew a white slab over it.
- **Tutorial 2** (gold Glows at threshold 0, intensity 0.1): After Effects shows a thin bolt. D-322 drew a haze of sparks.

In both, the glow After Effects shows matches the colour at *just* the intensity times the blurred light.

## Why both readings fit the thread

The same thread says "however bright the glow pixels are, their alphas are still very low" and "The resulting color will be calculated as glowPixelAlpha*glowPixelColor". So the measured 48 (GI 3, GT 0) is the glow's colour **read straight**, that is its colour divided by its covering. A colour of 3 over a covering 16 times thinner reads as 48, and adds only 3 times the light. That fits the thread and both tutorials.

## What changed

- **After Effects units:** the glow's colour is the intensity times the blurred light (as Classic). Its covering is divided by **t + 16 (1 − t)**, t being the threshold ÷ 100. The spread is unchanged: a Gaussian Blur at Blurriness = radius (D-321).
- Add and Screen are as before. Intensity 0 still changes nothing.
- **Classic (any file without units) is untouched.** Old projects draw and save exactly as before.
- A Glow saved in After Effects units since 4 October now draws dimmer, as After Effects does.
- FX-GLOW-AE-001, 004, 006, 007 and 008 have new expected values, written by `tools/glow_ae_reference.py` and committed before this code (a recorded specification decision, document 25).

## Pictures (`verification/D-331 pictures/`)

Our own app's frames, in the test copy, replaying the tutorials step by step:

| Picture | What to look for |
|---|---|
| `1_t3_f110_D-322.png` | Tutorial 3, frame 110, as D-322 drew it: the word buried in a white glow. |
| `2_t3_f110_D-331.png` | The same frame now: the letters stand clear with a soft halo, as After Effects shows. |
| `3_t2_f20_D-322.png` | Tutorial 2, frame 20, as D-322 drew it: the bolt lost in a bright haze. |
| `4_t2_f20_D-331.png` | The same frame now: a thin gold bolt with sparks round it, as After Effects shows. |

Still different from the tutorials, and not part of this fix: tutorial 3's background is grey where After Effects' is dark navy (we use stand-in textures), and tutorial 2's glowing block on the ground is D-330 (After Effects' 8 bpc rounding, proposed).

Also redrawn: `verification/D-322 pictures/3_ae_glow_defaults.png` and `5_ae_tutorial_glow.png`, a white square glowing over black. They now look like their Classic neighbours (2 and 4), not far brighter.

## Checks (cargo test)

All in `verification/D-322_glow_ae_table.md`, **49 of 49 pass**. Among them:

| Check | Result |
|---|---|
| FX-GLOW-AE-001 to 010: every pixel matches the revised values from `tools/glow_ae_reference.py` | pass, tolerance 2e-5 |
| An old file (no units) draws exactly as before (FX-GLOW-014) and is saved without units | pass |
| The preview card draws all six glows within 1 level of 255 of the CPU | pass, largest difference 0 |
| Over black, After Effects units glow about as bright as Classic next to the square | pass: defaults 51 against 62, tutorial 41 against 40 |

The full crate suite passes, apart from two scratch tests that failed before this change (`zz_scratch_g2`, and `zz_scratch_p25`, which needs a folder setting). The app suite has 91 passing and 5 ignored.

## For the owner to try

1. Open an older project with a Glow: it looks exactly as before.
2. Add a new Glow to white text over a dark solid. Set Threshold 0, Radius 40, Intensity 0.1: a soft, modest glow, not a white haze.
3. Raise Intensity to 1, then 3: the glow gets stronger step by step, never jumping to a white slab.
4. In Effect controls, switch Units to Classic and back: at these settings the two look alike over a dark background.

**Awaiting the owner's playtest.**
