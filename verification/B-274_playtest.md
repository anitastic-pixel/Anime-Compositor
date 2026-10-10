# B-274: Arbitrary Map

Built on 2026-10-09 as D-395, under your /loop request. **Arbitrary Map** (in **Color
Correction**) is our version of After Effects' PS Arbitrary Map: it recolours a layer through a
Photoshop arbitrary map, the `.amp` file Photoshop's Curves dialog saves when you draw a curve
with the pencil. Phase cycles the curve, which is how the effect is usually used for shifting,
psychedelic colours.

The settings, with the values they start at:

- **Map File** (none): the `.amp` file, chosen with the **Choose .amp file…** button on the
  card. Until one is chosen the layer is left as it is. The file stays where it is on disk, as a
  Color Lookup's `.cube` does, and Collect Files copies it.
- **Phase** (0): levels, -255 to 255, keyable. Cycles every curve in the file to the right, the
  end wrapping round to the start.
- **Apply Phase Map To Alpha** (Off): when On and the file holds an alpha curve (a fifth table),
  the layer's covering goes through it too.

A file of one curve changes every colour channel; three curves are red, green and blue; four
are an overall curve then red, green and blue; five add alpha. A file that is empty, of a length
that is not whole curves, or of more than five curves is refused and the card's message says
why; a missing file is reported and kept so you can relink it. Nothing is ever dropped silently.

The check, `verification/D-395_arbitrary_map_table.md` (161 of 161), holds every pixel to numbers
worked out by a separate program before the code existed, every refused file's reason word for
word, and the graphics card's picture to within 1 level of the processor's (it came out
identical on every test file, and within 1 level on the reference shot). The pictures are in
`verification/D-395 pictures/`; the map files are in `Fixtures/arbitrary_map/maps/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **One curve.** `2_brighter.png`, `master_256.amp`: the whole street brighter, colours kept.
2. **Three curves.** `3_rgb.png`, `rgb_768.amp` (red inverted, green squeezed, blue on a sine):
   odd colours; the sky and the road change colour completely.
3. **Four curves.** `4_four_tables.png`, `master_rgb_1024.amp`: a magenta-to-blue sky, green
   road.
4. **Phase.** `5_phase_64.png`, the same file at Phase 64: the colours shift again (red sky,
   pale road); `6_phase_minus_128.png`, Phase -128, half way round.
5. **Alpha.** `7_alpha_on.png`, `alpha_1280.amp` with Apply Phase Map To Alpha on: the street is
   fully covered and its alpha curve keeps full covering full, so it looks like the four-curve
   picture.
6. **In the app.** Add Arbitrary Map to a layer, press **Choose .amp file…** and pick
   `Fixtures/arbitrary_map/maps/master_rgb_1024.amp`. Key Phase from 0 to 255 over a second and
   play: the colours cycle round and come back.
7. **A bad file.** Choose `Fixtures/arbitrary_map/maps/refused/odd_300.amp`: the status line
   says it was not used because it is 300 bytes long, and nothing changes.
8. **Saved and opened again.** Save, close and open the project: the same file, settings and
   picture.

## Speed

**Timing (provisional, the machine was busy).** In `verification/B-274_arbitrary_map_timing_table.md`, the reference shot with a moving Noise on three layers, played again: 12.4 ms a frame on the card and 43.4 on the processor without the effect, 15.6 and 58.0 with Arbitrary Map (a four-table file, Phase 40), about 1.1 ms a layer on the card.

## Not built

- Phase's unit (levels, -255 to 255) is ours; Adobe does not publish the range.
- A file with more than five curves (a CMYK or multichannel map) is refused rather than guessed.
- With Apply Phase Map To Alpha on and a file that has an alpha curve, that layer is drawn by the
  processor, as Curves' alpha curve is (D-302).
- No tutorial with numbers was found to reproduce.
