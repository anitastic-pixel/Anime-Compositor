# B-231: Stretch Levels, Stretch Contrast, Stretch Color and Spread Tones

Built on 2026-10-08 under your effects loop request, decided as D-351. This is P0-15,
"Whole-frame statistics": the program can now count the colours of a layer's whole picture (its
histogram and average colour), and four new effects use those counts. They are ours, named after
After Effects' **Auto Levels**, **Auto Contrast**, **Auto Color** and **Equalize**, and all four
are in the **Color Correction** group of the effects list:

- **Stretch Levels** (Auto Levels): stretches each colour channel to its own darkest and lightest,
  so a dull picture fills the range and a colour cast lessens.
- **Stretch Contrast** (Auto Contrast): one stretch for the three channels together, so the colours
  keep their balance and only the contrast changes.
- **Stretch Color** (Auto Color): takes the darkest pixels to black and the lightest to white,
  taking out a cast; **Snap Neutral Midtones** On also takes the average colour to grey.
- **Spread Tones** (Equalize): spreads the tones evenly over the whole range. **Equalize** RGB,
  Brightness or Photoshop Style, and **Amount to Equalize**.

The first three have **Temporal Smoothing** (seconds each side whose pictures are counted with this
one, to steady a flickering shot), **Scene Detect** (smoothing stops at a cut), **Black Clip** and
**White Clip** (per cent of the darkest and lightest pixels let go to pure black and white; 0.1
when added). After Effects' **Blend With Original** is the **Mix** every effect already has: Blend
With Original 30 is Mix 70.

They are worked by the processor; whatever comes after them still runs on the graphics card. The
check, `verification/D-351_auto_tone_table.md`, 146 of 146, holds every pixel to numbers worked
out by a separate program before the code existed. The pictures are in `verification/D-351
pictures/`: `before.png` is a dull, warm street, and the other four are it after each effect.

## What to check

Import a dull or tinted picture or clip (an old photo, a shot under orange street lights) and add
the effects to it from **Color Correction**.

1. **Stretch Levels as added.** The picture fills the range from black to white and a tint
   lessens (`stretch_levels.png` against `before.png`).
2. **The clips.** Raise **Black Clip** to 5: more of the dark parts go pure black. Raise
   **White Clip** to 5: more of the light parts go pure white. Back to 0.1.
3. **Contrast against Levels.** Swap to **Stretch Contrast**: the picture is stretched but the
   tint stays, because the three channels move together (`stretch_contrast.png`).
4. **Stretch Color with Snap.** Swap to **Stretch Color** and turn **Snap Neutral Midtones** On:
   the warm cast is taken out, the sky blue again and the road near black
   (`stretch_color_snap.png`).
5. **Spread Tones.** Add **Spread Tones**: the tones spread evenly, harsher than a stretch
   (`spread_tones.png`). Try Equalize Brightness and Photoshop Style, and **Amount to Equalize**
   50 for half way.
6. **Blend With Original.** On any of them set **Mix** to 50: half the change.
7. **A flickering shot.** On a clip whose brightness flickers (a time-lapse, old film), add
   Stretch Levels and play: it may flicker more, as each frame is stretched alone. Set
   **Temporal Smoothing** to 0.5: the flicker steadies. Turn **Scene Detect** On on a clip with
   a cut: the frames either side of the cut are not mixed. Smoothing is slow to play (about 8
   frames a second on three 1080p layers); export is fine.
8. **On an adjustment layer.** Put Stretch Levels with Temporal Smoothing on an adjustment layer:
   each frame is stretched alone and a warning says smoothing cannot be done there. With
   Temporal Smoothing 0 it works as normal.
9. **Save and open.** Save, close and reopen: the settings and keys should be as you left them. A
   project saved before this build opens exactly as it did.
10. **Same as an export.** Export a frame and put it next to the viewer at Full. They should look
    the same. At Draft they can differ a little, see below.

## Not done, on purpose or for later

- **Temporal Smoothing on an adjustment layer** is not drawn (step 8). Put the effect on the
  footage layer, or precompose the layers beneath and put it on the composition layer.
- **Draft** counts its own quarter-size picture, so a small bright window can average away and
  the stretch differs a little from Full's (up to 20 levels on a white point on the test street).
  Export is always Full.
- **Smoothing is slow** because each frame counts its neighbours again every time it is shown.
  Keeping each frame's counts would fix it; not done yet.
- After Effects' **Color Link** and **Color Stabilizer** read these counts too; they come later.
- Adobe does not publish how its four work, so how each stretch is worked out is ours; the
  results will be close to After Effects' but not identical. Scene Detect's threshold is fixed.
- All four run on the processor; a graphics-card version is for later.

## If something is wrong

Say which step. The most likely fault would be in step 7 (smoothing near the start or end of a
clip, or Scene Detect cutting in the wrong place) or step 4 (Snap turning the picture a colour
instead of neutral).
