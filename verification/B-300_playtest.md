# B-300: Audio Waveform

Built on 2026-10-10 as D-421, under your /loop request: After Effects' Audio Waveform, in
**Generate**. It listens to a sound layer of the same composition and draws its wave, as short
strokes, a line or dots, along a line or along a mask, added over the layer or alone.

- **Audio Layer** (none): the sound layer to listen to; the list shows the composition's sound layers.
- **Start Point**, **End Point** (10, 50 and 90, 50 per cent): the line the wave runs along.
- **Path** (none): a mask of the layer to run the wave along instead.
- **Displayed Samples** (32): how many points the wave is drawn with.
- **Maximum Height** (300 pixels): how far the loudest sound reaches from the line.
- **Audio Duration** (90 ms): how much sound each frame shows.
- **Audio Offset** (0 ms): shows sound earlier or later than the frame.
- **Thickness** (2 pixels), **Softness** (50): drawn as Beam draws its line.
- **Random Seed (Analog)** (1): which points of the line or dots take the loudest or quietest
  moment of their share of the sound.
- **Inside Color** (white), **Outside Color** (blue): the colour along the middle of each mark and
  at its edges.
- **Waveform Options** (Mono): Mono mixes both channels; Left or Right shows one channel of a stereo
  file. A file with one channel plays as Mono.
- **Display Options** (Analog Lines): Digital (a short stroke from the quietest to the loudest
  moment of each share), Analog Lines (one line through the points) or Analog Dots (a dot at each).
- **Composite On Original** (Off): On adds the wave over the layer (brightening it); Off draws the
  wave alone, the rest see-through.

Adobe's manual says what each display shows but not how the points are taken from the sound, how
tall they stand or how the seed picks, so that rule is ours. One difference from After Effects:
it listens to the sound layer with its volume (gain) applied, as Audio Spectrum does.

The check, `verification/D-421_audio_waveform_table.md` (297 of 297), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-421 pictures/`
(`town.png` is the layer, the sound is the same ten seconds of music as Audio Spectrum's). In these
pictures a see-through part shows as white or as your viewer's checker.

## What to check

1. **Before.** `1_before.png`: the street with no effect.
2. **As added.** `2_as_added.png` (100 tall): a white line edged blue zigzagging along the middle,
   alone, the rest see-through.
3. **Over the street.** `3_over_the_street.png`, two seconds in: short digital strokes along the
   road, over the street.
4. **Neon line.** `4_neon_line.png`, four seconds in: an orange neon wave across the middle, a
   pale core with an orange glow, over the street.
5. **Slanted dots.** `5_slanted_dots.png`, six seconds in: red dots on a slant from the foot of
   the road up to the top right, turning pink where they are added over the sky.
6. **In the app.** You need a WAV sound file. Import it, put it in the composition, and add Audio
   Waveform (Generate) to a picture layer. Choose the sound layer in Audio Layer and play: the
   wave moves with the sound. Try Display Options (strokes, a line, dots). With a stereo file, try
   Waveform Options Left and Right: each shows one channel. Change Random Seed with Analog Lines
   or Dots: the line changes which points it picks. Draw a closed mask on the layer and choose it
   in Path: the wave runs round it. Turn Composite On Original on: the wave added over the layer.
7. **Only WAV is heard.** A sound in another format draws as silence (a flat line) and says
   MEDIA_AUDIO_UNREADABLE. A saved project whose Audio Layer names a layer with no sound draws
   nothing and says EFFECT_SOUND_MISSING.
8. **Out of range.** Type 0 in Displayed Samples: it is refused with a sentence saying it runs
   from 1 to 4096.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-300_audio_waveform_timing_table.md` (quiet): the reference shot with a moving
Noise and an Audio Waveform on three layers, played again, 15.9 ms a frame on the card as added
against 12.7 for the Noise alone; on the processor 49.2 against 43.4. With 1024 strokes the card
takes 25.1 against the processor's 70.7: the card is quicker for this effect.
