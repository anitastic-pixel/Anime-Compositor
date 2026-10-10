# B-299: Audio Spectrum

Built on 2026-10-10 as D-420, under your /loop request: After Effects' Audio Spectrum, in
**Generate**. It listens to a sound layer of the same composition and draws how loud each pitch
is, as bars, a line or dots, along a line, round a point or along a mask, over the layer or alone.

- **Audio Layer** (none): the sound layer to listen to; the list shows the composition's sound layers.
- **Start Point**, **End Point** (10, 50 and 90, 50 per cent): the line the bars stand on.
- **Path** (none): a mask of the layer to stand the bars along instead.
- **Use Polar Path** (Off): On stands the bars round Start Point, from straight up, clockwise.
- **Start Frequency**, **End Frequency** (20 and 2000 Hz): the lowest and highest pitch shown.
- **Frequency Bands** (64): how many bars.
- **Maximum Height** (200 pixels): how tall the loudest bar can be.
- **Audio Duration** (90 ms): how much sound each frame listens to.
- **Audio Offset** (0 ms): listens earlier or later than the frame.
- **Thickness** (3 pixels), **Softness** (50): drawn as Beam draws its line.
- **Inside Color** (white), **Outside Color** (blue): the colour along the middle of each mark and
  at its edges.
- **Blend Overlapping Colors**, **Hue Interpolation**, **Dynamic Hue Phase**, **Color Symmetry**:
  how the colours mix and turn round the colour wheel from bar to bar.
- **Display Options** (Digital): Digital bars, Analog Lines (one line through the bars' tips) or
  Analog Dots (a dot at each tip).
- **Side Options** (Side A & B): bars up, down, or both ways.
- **Duration Averaging** (Off): On smooths the bars by averaging three listens.
- **Composite On Original** (Off): On draws over the layer; Off draws the spectrum alone, the rest
  see-through.

Adobe's manual says what the effect shows but not how it measures the pitches or how tall it
draws a level, so that rule is ours: a sound at full loudness at one pitch makes its bar the
whole Maximum Height. One difference from After Effects: it listens to the sound layer with its
volume (gain) applied, where Adobe's manual says After Effects ignores the layer's levels.

The check, `verification/D-420_audio_spectrum_table.md` (328 of 328), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-420 pictures/`
(`town.png` is the layer, the sound is ten seconds of music the test writes). In these pictures a
see-through part shows as white or as your viewer's checker.

## What to check

1. **Before.** `1_before.png`: the street with no effect.
2. **As added.** `2_as_added.png` (600 tall): white bars edged blue along the middle, both ways,
   some tall, most just dots, the rest see-through.
3. **Over the street.** `3_over_the_street.png`, two seconds in: bars standing up from the foot of
   the road, over the street.
4. **Neon line.** `4_neon_line.png`, four seconds in: an orange glowing line across the middle,
   with sharp peaks where the music is loud.
5. **Ring of dots.** `5_ring_of_dots.png`, six seconds in: dots of every colour scattered round
   the middle, nearer or further out by how loud each pitch is, over the street.
6. **In the app.** You need a WAV sound file. Import it, put it in the composition, and add Audio
   Spectrum (Generate) to a picture layer. Choose the sound layer in Audio Layer and play: the
   bars move with the music. Try Display Options (bars, a line, dots), Side Options (up, down,
   both), and Use Polar Path on (round Start Point). Draw a closed mask on the layer and choose it
   in Path: the bars stand along it. Turn Composite On Original on: the spectrum over the layer.
7. **Only WAV is heard.** A sound in another format draws as silence (every bar flat) and says
   MEDIA_AUDIO_UNREADABLE. A saved project whose Audio Layer names a layer with no sound draws
   nothing and says EFFECT_SOUND_MISSING.
8. **Out of range.** Type 0 in Frequency Bands: it is refused with a sentence saying it runs from
   1 to 4096.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-299_audio_spectrum_timing_table.md` (quiet): the reference shot with a moving
Noise and an Audio Spectrum on three layers, played again, 49.8 ms a frame on the card as added
against 27.9 for the Noise alone; on the processor 56.5 against 54.7. With many dots round the
middle the processor is quicker (66.7 against 75.7 on the card): at these settings the card is
not quicker for this effect.
