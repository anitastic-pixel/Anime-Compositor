# D-322 / B-206: Glow in After Effects' own numbers

> **Corrected by D-331 (2026-10-05).** Played against After Effects' own tutorial frames, the glow below was about fifteen times too bright. The brightness formula is the glow's colour *read straight*: the colour stays at the intensity, and the glow's covering is thinner. See `D-331_glow_ae_corrected.md`. The pictures and checks here are redrawn under D-331.

From D-308 (T4-1) and P-26. The tutorials set Glow to small intensities (0.1 to 0.3) with threshold 0 or 5. In After Effects those give a strong glow; here they gave a faint one.

## Where the rule comes from (no After Effects needed)

The Creative COW thread [Glow Effect and transparent background mechanics](https://creativecow.net/forums/thread/glow-effect-and-transparent-background-mechanics/) measured After Effects' Glow on white shapes:

- "Glow Radius pretty much creates layer's Gaussian blur and adds it to the layer. This is the only parameter which changes alpha of surrounding pixels." So the radius is a Gaussian Blur's Blurriness (D-321), and intensity leaves the glow's covering alone.
- The glow's brightness is **GI × (GT/100) + GI × 16 × (1 − GT/100)**, GI being Glow Intensity and GT Glow Threshold. At intensity 3 and threshold 0 the glow reads about 48, as that formula says.

So at threshold 0, intensity 0.1 is 1.6 times the light, and After Effects' defaults (threshold 60, intensity 1) are 7 times.

## What changed

- Glow has a **Units** choice in Effect controls:
  - **After Effects**: the numbers mean what they mean in After Effects.
  - **Classic (older projects)**: exactly the old rule.
- A Glow you add from now on uses After Effects units, with After Effects' defaults. **Your existing projects do not change**: a file saved before this has no units, reads as Classic, draws exactly as before and saves exactly as before.

## Checks (cargo test)

All in `verification/D-322_glow_ae_table.md`, **49 of 49 pass**. Among them:

| Check | Result |
|---|---|
| FX-GLOW-AE-001 to 010: every pixel matches values written by `tools/glow_ae_reference.py` before the code existed | pass, tolerance 2e-5 |
| An old file (no units) draws exactly as before (FX-GLOW-014) and is saved without units | pass |
| After Effects units are saved as `units: "after_effects"`; a wrong word (`"ae"`, `"After_Effects"`) is refused with a sentence | pass |
| The preview card draws it the same as the CPU | 0 levels of 255 apart, 6 files |
| Cut into tiles of 1 or 64 pixels, the frame is identical | pass |

The full crate and app suites were run afterwards (see the commit).

## Pictures

In `verification/D-322 pictures/`, a white square 24 pixels across, over black:

- `1_square.png`: no glow.
- `2_old_glow_defaults.png`: threshold 60, radius 10, intensity 1, in an older project (Classic). A soft rim.
- `3_ae_glow_defaults.png`: the same in After Effects units. Over black, about as bright as Classic and a little tighter (D-331).
- `4_old_tutorial_glow.png`: tutorial 2's first Glow (threshold 0, radius 39, intensity 0.1) in Float, Classic. Barely there.
- `5_ae_tutorial_glow.png`: the same in After Effects units. Over black, as faint as Classic, as tutorial 2's After Effects frames show (D-331).

From the app's test copy (not your app):

- `6_app_new_glow_after_effects.png`: a white square with a newly added Glow. Effect controls shows threshold 60, radius 10, intensity 1 and **Units: After Effects**.
- `7_app_same_units_classic.png`: the same with Units switched to **Classic (older projects)**.

## For the owner to try

1. Add **Glow** to a white layer. Units should say **After Effects** (D-331's strength).
2. Set threshold 0, radius 39, intensity 0.1 in a Float composition. You should see a faint soft halo, as in tutorial 2.
3. Open an older project that has a Glow. It should look exactly as it did, with Units set to **Classic (older projects)**.

**Awaiting the owner's playtest.**
