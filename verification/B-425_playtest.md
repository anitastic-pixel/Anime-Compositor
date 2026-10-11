# B-425: Lens Dirt

Built on 2026-10-11 as D-545, your pick #14 (Lens Dirt). It is not a new effect: **Soft Physical
Glow** and **Lens Flare** each gain a **Dirt Layer** and five settings with it, as
`docs/effects/PLUGINS.md` settles ("a 'dirt texture from a layer' setting on Soft Physical Glow and
Lens Flare"). The settings and how the dirt is lit are Deep Glow 2's, from its documentation.

A real lens has smudges and specks on it. You never see them, until a bright light shines
through: then they light up round the light. Lens Dirt does that: you pick a layer holding a
picture of dirt, and it shows only where the glow or the flare is bright.

- **Dirt Layer** (none as added): the layer whose picture is the dirt. Turn that layer off so
  it is not drawn itself; its picture is still read. The picture sits centred on the layer.
- **Dirt Opacity** (100 per cent): how much dirt shows.
- **Dirt Inherit Glow Colour** (0 per cent): at 0 the dirt keeps its own colours; at 100 it
  takes the light's colours, channel by channel.
- **Dirt Matte Exposure** (0 stops): raise it and less light is needed to show the dirt.
- **Dirt Matte Gamma** (1): above 1 the dirt reaches further into the faint edge of the light.
- **Dirt Repeat Edges** (on): a dirt picture smaller than the layer is mirrored to fill it; off
  shows it once, in the middle.

All the numbers can be keyed. Without a dirt layer both effects draw exactly as before.

Not built: Deep Glow 2's View menu showing the dirt's stages, moving or scaling the dirt picture
on its own, and dirt on the other glows.

The check, `verification/D-545_lens_dirt_table.md` (463 of 463 checks), holds every pixel to numbers worked
out by a separate program before the code existed, runs the earlier checks of Soft Physical Glow
(45, plus 22, 20 and Halation's 18) and Lens Flare (19) again unchanged, and holds the graphics
card's picture to within 1 level of the processor's. The pictures are in
`verification/D-545 pictures/`, with smudges of our own as the dirt.

## What to check

1. **No dirt.** `glow_plain.png`: the town with Soft Physical Glow, for comparison.
2. **Dirt.** `glow_dirt.png`: the same with the smudges as its dirt: soft grey smudges appear
   round the bright windows and the sky, and nowhere in the dark.
3. **Stronger.** `glow_dirt_strong.png`: Matte Exposure 3, Matte Gamma 2: the smudges show
   further out into the glow.
4. **In the glow's colour.** `glow_dirt_colour.png`: Inherit Glow Colour 100: the smudges take
   the glow's warm and cool colours instead of their own grey.
5. **Once.** `glow_dirt_once.png`: Repeat Edges off: the smudge picture once, in the middle;
   outside it, the plain glow.
6. **Flare.** `flare_plain.png` and `flare_dirt.png`: Lens Flare without and with the smudges:
   they light up round the flare's bright core and along its ghosts.
7. **In the app.** Put a picture of dirt or specks (any picture with a clear background) in the
   composition and turn its layer off. Add Soft Physical Glow to a layer with bright parts, and
   pick the dirt layer in Dirt Layer. Raise Dirt Matte Exposure and watch more dirt show. Do the
   same on a Lens Flare and drag the flare's centre: the dirt lights up where the flare passes.
8. **Missing layer.** Delete the dirt layer: the effect is left out and a warning says the layer
   is missing.
9. **Out of range.** Type 101 in Dirt Opacity: it is refused with a sentence saying the range.
10. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the dirt centred on the layer, and mirrored to fill it.

## Speed

Measured, but PROVISIONAL: other builds were running on this machine at the time, so the numbers are higher and noisier than a quiet machine gives; the glow-with-dirt shot was even slower played again (133.8 ms) than the first time (96.2), which only noise explains. On the graphics card, the reference shot with colour noise and then Soft Physical Glow (radius 40) on its three layers took 66.9 ms a frame without dirt and 133.8 with the fourth layer as its dirt; Lens Flare took 76.6 without and 109.1 with; the noise alone 20.9. The full table is `verification/B-425_lens_dirt_timing_table.md`. A quiet re-measure is owed. What to check by eye: whether scrubbing a shot with dirt on its glow still feels smooth enough to you.
