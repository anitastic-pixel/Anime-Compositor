# B-233: Soft Physical Glow

Built on 2026-10-08 under your effects loop request, decided as D-353. This is P0-21, the
"shared soft-glow engine", proved by pick #1 from `docs/effects/PLUGINS.md`, **Soft Physical
Glow**: a glow with a bright core and a long, soft tail, the way real light spreads, after the
Deep Glow plugin's main mode. It is a new mode of our Glow, so it sits in the **Light & Glow**
group of the effects list as its own entry, **Soft Physical Glow**, just below Glow. A Glow you
already have is not changed.

Its settings, with the plugin's names:

- **Radius** (how far the tail reaches, up to 2,000 pixels; 500 when added) and **Exposure**
  (how bright the glow is; 1 when added).
- **Threshold Mode** (Chroma tests red, green and blue each on its own; Luminance tests one
  brightness), **Threshold** (0 when added, so everything glows), **Threshold Smooth** (how soft
  the cut is) and **Saturation Bias** (positive glows the strong colours more, negative the
  greys).
- **Aspect Ratio** and **Aspect Angle**: stretch the glow into an oval at any angle (above 1
  wider, below 1 taller).
- **Blend Mode**: Screen (adds, but never past white) or Add; **Source Opacity** (the layer
  itself on top of its glow); **Unmult** (On: the glow is see-through where it is dark; Off: it
  sits on solid black).

It runs on the graphics card. One case stays on the processor: a Threshold above 0 with
Threshold Smooth 0, a hard cut, because the card and the processor could disagree on a pixel
right at the cut. The check, `verification/D-353_soft_glow_table.md`, 238 of 238, holds every
pixel to numbers worked out by a separate program before the code existed, and the card's
picture to within 1 level of the processor's. The pictures are in `verification/D-353 pictures/`.

## What to check

Use a shot with some bright parts (the reference shot's town works) and add **Soft Physical
Glow** from **Light & Glow**.

1. **As added.** The whole layer glows into a wide, soft haze that reaches far past its edges
   (`soft_glow_as_added.png`). Nothing is cut off at the layer's edge: the glow keeps going.
2. **Against the old Glow.** Set **Threshold** to 60, **Threshold Smooth** to 50 and **Radius**
   to 60. Only the bright parts glow, with a bright core close in and a long faint tail
   (`soft_glow_threshold_60.png`), where the old Glow gives one short, even halo
   (`classic_glow.png`).
3. **Radius.** Key Radius from 0 to 500 over a second and play it. The glow should grow smoothly,
   with no jumps or pops.
4. **Threshold Mode.** With Threshold 30, switch Chroma and Luminance: Chroma lets a strong
   colour's bright channel glow on its own; Luminance glows only what is bright overall.
5. **Saturation Bias.** At Threshold 50, set it to 100: the strong colours glow and greys do not.
   Set it to -100: the other way round.
6. **Stretch.** Set **Aspect Ratio** to 1.8: the glow spreads sideways like an anamorphic streak
   (`soft_glow_stretched.png`). Turn **Aspect Angle**: the streak turns with it.
7. **Blend Mode, Exposure, Unmult.** Screen never goes past white; Add can blow out. Raise
   Exposure to brighten the glow. With Unmult Off the area round the layer turns black.
8. **Save and open.** Save, close and reopen: the settings and keys should be as you left them,
   and it should still say Soft Physical Glow. A project saved before this build opens exactly as
   it did.
9. **Same as an export.** Export a frame and put it next to the viewer at Full. They should look
   the same.

## Not done, on purpose or for later

- **Speed**: with one on each of three 1080p layers a frame takes about 50 to 60 ms at radius 60
  to 500 (about 12 ms without), so playback drops below 24 frames a second, and about 0.14 s at
  radius 2,000 (`verification/B-233_soft_glow_timing_table.md`). A hard threshold (Smooth 0)
  runs on the processor at about 0.35 s a frame. Export is unaffected in result.
- **Tint** (one colour for the core, another for the edge) is pick #6, and a separate size per
  colour channel (colour fringes) is pick #10. Both come next on this engine; neither is here yet.
- **An input mask** (glow only from part of the layer), **tone mapping**, **lens dirt**, **Lens
  Iris** and the **Gamma Correction** amount are not there.
- **No quality setting**: one fixed method, as PLUGINS.md settled. A strongly stretched glow is a
  little softer than a true oval at its widest sizes.
- With Unmult Off, the whole area the glow grows into is solid black, not only the layer's own
  rectangle.
- The plugin's maker does not publish how it is worked out, so ours is our own; it will look like
  Deep Glow, not be identical to it.

## If something is wrong

Say which step. The most likely fault would be in step 3 (a visible jump while Radius is keyed)
or step 6 (the streak at a wrong angle).
