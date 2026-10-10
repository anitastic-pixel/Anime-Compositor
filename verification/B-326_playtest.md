# B-326: Curl Noise

Built on 2026-10-10 as D-446, under your /loop request: **Curl Noise**, in **Noise & Grain**, new
in After Effects 26.3. It draws grey flow lines that swirl like smoke, ink or water, and keep
drifting and changing as the frames pass, without flickering.

After Effects can also swirl a picture (**Source**: This Layer or Other Layer). That is not built:
a project that asks for it keeps the setting, draws the layer without the effect, and says why in a
sentence. Logged as a gap; the row in EFFECTS.md is marked partial.

- **Speed** (10) and **Direction** (0, up): how fast and which way the whole pattern drifts.
- **Size** (100 pixels) and **Offset**: how big the swirls are and where they sit.
- **Evolution** and **Turbulence Speed** (20): how the swirls change over time.
- **Swirl** (45): how much the flow curls round on itself. **Density** (0): smaller, busier swirls
  above 0, bigger below. **Smoothness** (50): softer, rounder swirls when higher.
- **Vertical Bias** (50): towards 0 the flow runs across, towards 100 up and down.
- **Sample Count** (12) and **Sample Radius** (30 pixels): how long the streaks are; 0 radius, no
  streaks.
- **Flow Softness** (20), **Edge Definition** (50) and **Flow Falloff** (0): softer streaks, crisper
  streaks, and streaks fading where the noise is dark.
- **View**: **Final Render** (the streaks), **Input Noise** (the soft noise beneath) or **Curl
  Generation** (the flow as red and green, the noise as blue).
- **Contrast** (100) and **Brightness** (0); **Clip HDR Results** (on).
- **Channel**: **RGB** (the grey replaces the picture), **Red**, **Green** or **Blue** (only that
  colour), or **Alpha** (the picture shows through the light parts).

The controls are After Effects' own; how they draw is our own rule (Adobe publishes none), not
matched to After Effects pixel for pixel.

The check, `verification/D-446_curl_noise_table.md` (261 of 261), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-446 pictures/`, over the
street (`town.png`; `1_before.png` without the effect).

## What to check

1. **As added.** `2_as_added.png`: soft swirling grey flow over the whole street.
2. **Fine lines.** `3_fine_lines.png` (size 30, radius 20, 16 samples): fine streaked flow lines,
   like brushed hair or wood grain.
3. **Curl Generation.** `4_curl_generation.png`: the flow as reds and greens, the noise as blue.
4. **Alpha.** `5_alpha.png`: the street shows through the light parts, with holes in the dark.
5. **Moving.** `6_frame_24.png`, the same settings as 1 at frame 24: the flow has drifted up and
   changed shape.
6. **In the app.** Add Curl Noise (Noise & Grain) to a picture and play: the swirls drift and
   change smoothly, no flicker. Try Swirl, Size, Vertical Bias, the three Views and Channel Alpha.
7. **This Layer.** Not in the Source list. A project that asks for it draws the layer without the
   effect and says the source is not built yet.
8. **Out of range.** Type 25 in Sample Count: it is refused with a sentence giving the range.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-326_curl_noise_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a moving
Noise and a Curl Noise on three layers, played again, 66.6 ms a frame on the card as added
against 420.0 for the Noise alone; on the processor 271.7 against 46.1:
a busy machine cannot say whether the card is quicker than the processor for this effect.
