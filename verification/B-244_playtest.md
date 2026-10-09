# B-244: Broadcast Safe

Built on 2026-10-08 under your effects loop request, decided as D-365. **Broadcast Safe** (in
**Color Correction**) is our version of After Effects' Broadcast Colors: it finds colours too
bright or too saturated for a television signal and darkens them, greys them, or keys them out,
or keeps only them so you can see where they are.

The settings, with the values they start at:

- **Locale** (NTSC): NTSC puts black at 7.5 IRE, PAL at 0.
- **How to Make Colour Safe** (Reduce Luminance): Reduce Luminance darkens the unsafe colours;
  Reduce Saturation greys them at the same brightness; Key Out Unsafe clears them; Key Out Safe
  clears everything else, leaving only the unsafe pixels.
- **Maximum Signal** (110): 90 to 120 IRE. Brightness and colour together above this are unsafe.

The check, `verification/D-365_broadcast_safe_table.md` (125 of 125), holds every pixel to
numbers worked out by a separate program before the code existed, and the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-365 pictures/`.

## What to check

**`town.png`** is a street with lit yellow windows and white road markings. **`1_before.png`**
is the street with no effect.

1. **As added.** `2_as_added.png` (NTSC, Reduce Luminance, 110): every colour in the street is
   safe, so it should look exactly like the before picture.
2. **Darkened.** `3_reduce_luminance_100.png`, limit 100: the lit windows (about 107 IRE) should
   be a little darker and still yellow; the white road markings (about 98) unchanged.
3. **Greyed.** `4_reduce_saturation_100.png`: the windows paler and less yellow instead.
4. **Where they are.** `5_key_out_safe_100.png`: only the lit windows are left.
5. **In the app.** Put it on a layer with pure yellow or cyan and pick Key Out Safe: the colours a
   television would object to are the ones left.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU, Broadcast Safe costs under 1 ms a 1080p layer, about 4 ms on the
processor (`verification/B-244_colour_timing_table.md`).

## Not built

- No waveform or vectorscope to see the signal; Key Out Safe is the way to see unsafe pixels.
- The signal is measured on the picture's sRGB colours, as if they were video's, and only its top
  is checked, not a dip below black.
