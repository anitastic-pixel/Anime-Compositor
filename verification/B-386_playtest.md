# B-386: 3D Glasses

Built on 2026-10-11 as D-506, under your /loop request: 3D Glasses, in **Camera & Lens** (the
panel has no Perspective group), with After Effects' settings. It makes one stereo picture from a
left-eye and a right-eye layer: an anaglyph to watch with red-cyan (or red-green) glasses, or the
two side by side, one over the other, or on alternate rows for a 3D screen.

- **Left View** and **Right View** (none): the two layers, chosen like Difference Matte's plate;
  none means the layer the effect is on. A layer that is not in the composition leaves the
  picture as it is, with a warning.
- **Scene Convergence** (0): slides the two views apart sideways, the left view half of it to
  the right and the right view half to the left, which sets what seems in front of or behind
  the screen.
- **Vertical Alignment** (0): slides them apart up and down, to fix views shot slightly askew.
- **Units** (Pixels): the two distances in pixels, or in per cent of the layer's width (and
  height).
- **Swap Left-Right** (Off): exchanges the two views.
- **3D View** (Balanced Colored Red Blue): Stereo Pair (Side by Side), Over Under, Interlace
  Upper L Lower R, Red Green LR, Red Blue LR, Balanced Red Green LR, Balanced Red Blue LR and
  Balanced Colored Red Blue.
- **Balance** (0.75): for the three balanced views, how far apart the two eyes are kept: 0
  makes them one (no depth), 1 keeps them as they are, higher pushes them apart (stronger
  colour, more ghosting).

Adobe publishes no formula and no settings when added, so the rule and those settings are ours,
written down in document 21. As added both views are the layer itself, so nothing changes until
you choose a view or a convergence.

The check, `verification/D-506_three_d_glasses_table.md` (238 of 238), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-506 pictures/`.

## What to check

1. **The two eyes.** `1_left.png` is a street; `2_right.png` is the same street as the right
   eye sees it, 6 pixels further left.
2. **Red Blue.** `3_red_blue.png`: the street in grey with red and cyan fringes on the edges of
   the buildings. With red-cyan glasses on, the street should stand out from the screen.
3. **Lined up.** `4_lined_up.png`: the same at Scene Convergence -6, which slides the two eyes
   back together: the picture is plain grey, no fringes, except a thin strip at each side that
   only one eye sees.
4. **Balanced Colored Red Blue.** `5_balanced_colored.png`: the street in its own colours, with
   red and cyan fringes. Through glasses the colours should mostly survive.
5. **Side by side.** `6_side_by_side.png`: the left eye squeezed into the left half and the
   right eye into the right half.
6. **In the app.** Put two layers of a stereo pair in a composition, add 3D Glasses (Camera &
   Lens) to one, choose the other as Right View, and try each 3D View, Swap Left-Right,
   Scene Convergence and Balance. Keyframe Scene Convergence and play it: the depth should
   change smoothly.
7. **Out of range.** Type 1001 in Scene Convergence: it is refused with a sentence saying the
   range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when added; what Balance does (Adobe says
only that it reduces ghosting); the red-blue views being built from each eye's brightness; and
a view layer of another size being centred.

## Speed

Measured on a quiet machine (`verification/B-386_three_d_glasses_timing_table.md`). Three 3D Glasses on a 1920 by 1080 shot,
card / processor, milliseconds a frame played again: the shot after a moving Noise 12.8 / 41.3 ms a frame, against layer 4, balanced colored red blue 46.6 / 78.2, the layer itself, red blue, convergence 7 17.3 / 65.9; with the layer itself about 1.5 ms a 1080p layer on the card, over the row's Target P1 (1 ms), and 8.2 on the processor; with another layer as a view about 11.3 ms a layer on the card and 12.3 on the processor, that layer drawn on the processor first (D-189's path, as Difference Matte's plate).
