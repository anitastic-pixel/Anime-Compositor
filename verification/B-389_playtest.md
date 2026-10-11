# B-389: Glass Melt

Built on 2026-10-11 as D-509, under your /loop request: Glass Melt, in **Transition**, our own
take on CycoreFX's CC Glass Wipe (a name of our own). The layer melts away like bending glass,
the darkest parts of a gradient going first, to show another layer underneath.

- **Completion** (0): how far the melt has gone; 0 the layer whole, 100 all gone.
- **Layer to Reveal** (none): the layer shown where the melt has gone, centred; with none the
  gone part is clear.
- **Gradient Layer** (none): the layer whose brightness sets the order, stretched to fit; with
  none the layer's own brightness is used. Dark parts go first.
- **Softness** (10): blurs the gradient, smoothing the melt's edge and its bend.
- **Displacement Amount** (50): how far the melting glass bends; a minus number bends it the
  other way.

Not built: a choice of how the layers fit (the reveal is always centred, the gradient always
stretched), a band-width control, and a choice of property other than brightness. The layer does
not grow, so a bend near its edge stops there.

CycoreFX publishes no formula and no settings when added, so the rule and those settings are
ours, written down in document 21.

The check, `verification/D-509_glass_melt_table.md` (214 of 214), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-509 pictures/`.

## What to check

1. **The street.** `1_street.png`: the shot without the effect.
2. **The reveal.** `2_reveal.png`: the layer that will show through.
3. **Half melted.** `3_half_melted.png`: the darker parts of the street gone to the reveal, the
   edges between them bent like glass.
4. **A strong bend.** `4_strong_bend.png`: the melting edges bent much further.
5. **Done.** `5_done.png`: completion 100, only the reveal left.
6. **In the app.** Add Glass Melt (Transition) to a layer, choose a Layer to Reveal, keyframe
   Completion from 0 to 100 and play: the layer melts away, dark parts first. Choose a Gradient
   Layer: the melt follows its brightness instead.
7. **Out of range.** Type 101 in Completion: it is refused with a sentence saying the range.
8. **A missing layer.** Delete the Layer to Reveal: the layer is drawn as it is and a warning
   names the missing layer.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when added; how wide the melting edge is;
how the glass bends.

## Speed

`verification/B-389_glass_melt_timing_table.md`. Three Glass Melts on a 1920 by 1080 shot, milliseconds a frame played
again: PROVISIONAL (machine not quiet, other lanes compiling): card 66.7 and 93.1 ms a frame against 20.3 for the Noise alone, processor 209.0 and 234.7 against 69.5; a quiet re-measure is owed.
