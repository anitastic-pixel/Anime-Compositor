# B-671 playtest: effects that read their own layer, on an adjustment layer (D-791)

What changed: some effects first look at their own layer's masks, shapes, text or sound before
they draw. On an adjustment layer that step was skipped, so they were left out of the frame (or
drawn without part of themselves) and nothing was said.

- Audio Spectrum and Audio Waveform with Path 0 now draw on the frame beneath the adjustment
  layer, exactly as they do on the layer itself.
- The others draw as before, and every frame now says EFFECT_PATH_MISSING: "Adjustment layer
  adj's Path Stroke works from a mask, a shape or the text of its own layer, which an effect on an
  adjustment layer cannot use yet." The remedy offered is to put the effect on the layer itself,
  or precompose the layers beneath and put it on the composition layer. This applies to:
  - Path Stroke, Vegas and Scribble
  - Energy Stroke along masks, shapes or text, or with Alpha Mode on
  - Audio Spectrum and Waveform on a Path
  - Inner/Outer Key with a Foreground
  - Reshape and Mask Smear naming a mask
  - Liquify's Freeze Area Mask
  - Particle Playground's Wall Mask

## What the pictures show

Frame 0, 16 x 10 pixels shown 24 times larger, drawn by the real app.

- `B-671 pictures/spectrum_on_adjustment_before_after.png`: `Fixtures/audio_spectrum/fx_aspec_002`
  (Audio Spectrum hearing tone.wav) with the effect moved from the cel onto an adjustment layer
  above it.
  - before: only the cel shows, because the spectrum was left out.
  - after: the spectrum is drawn. Its Composite On Original is Off, so the bars replace the
    picture beneath, as they do on the cel itself (112 of 160 pixels change).
- `B-671 pictures/stroke_on_adjustment_before_after.png`: the same file with a Path Stroke on the
  adjustment layer's own Mask 1.
  - before and after are the same picture (0 pixels differ).
  - What changed is the warning, which before was missing.

The window does not yet list EFFECT_PATH_MISSING beside the frame. It lists only missing fonts and
failing expressions there, as with B-666's Fill warning. The warning is in the frame's log and is
checked in the table.

## Try it yourself (5 minutes)

1. New composition. Import a sound and put it in the composition. Add a solid, then an adjustment
   layer above it.
2. On the adjustment layer add Audio Spectrum, set its Audio Layer to the sound and leave Path at
   None.
   - Expected: the spectrum draws over the whole frame beneath. Before this change, the solid
     showed unchanged.
3. Turn the Audio Spectrum's Composite On Original on.
   - Expected: the bars sit on top of the solid.
4. Draw a mask on the adjustment layer and add Stroke (Generate) on Mask 1.
   - Expected: no stroke is drawn, the same as before this change.

Pass if every "Expected" holds. The numbers are in `B-671_adjustment_prep_table.md` (57 of 57).

## The effects' own tests, unchanged

Each effect on an ordinary layer draws as before. These named tests pass unchanged:

- b235_stroke, b236_stroke_shapes, b252_stroke_text
- b324_vegas, b322_scribble
- b388_energy_stroke, b445_energy_stroke_extras
- b299_audio_spectrum, b300_audio_waveform
- b382_inner_outer_key, b485_reshape, b572_reshape_points, b486_mask_smear
- b484_liquify, b570_liquify_brushes, b493_particle_playground
- b666_fill_adjustment, d253_d254_roundtrip
