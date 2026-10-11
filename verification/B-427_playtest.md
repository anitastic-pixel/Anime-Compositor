# B-427: the card's maths audit

Built on 2026-10-11 as D-547. Not a new effect: a fix where the graphics card drew a different
picture from the processor.

**What was wrong.** This machine's graphics card gets the remainder of a negative whole number
wrong (it answers -7 divided by 6 leaves 3, not -1). The one place that met this was the noise's
loop: **Noise Alpha** with Cycle Noise on, or **Fractal Noise** with Cycle Evolution on, drew a
different noise in the viewer than in an export whenever Noise Phase or Evolution was negative and
the cycle was not 2, 4 or 8.

**The check.** `verification/D-547_card_remainder_table.md` (56 of 56 checks): the reference shot with
each noise on its first three layers, at frames 0, 100 and 239, Full and Draft, the card's picture
against the processor's, within 1 level of 255. Before the fix, 24 of the 56 checks failed:
`verification/D-547_card_remainder_before_table.md`.

## What to check

1. **Before and after.** In `verification/D-547 pictures/`, `noise_alpha_processor.png` is the
   right picture (Noise Alpha, phase -100, cycle 3). `noise_alpha_card_before.png` is what the
   card drew before the fix: a different pattern of holes. `noise_alpha_card.png` is the card now:
   the same as the processor's.
2. **Cycle 7.** The same with `noise_alpha_7_processor.png`, `noise_alpha_7_card_before.png` and
   `noise_alpha_7_card.png` (phase -1000, cycle 7).
3. **In the app.** Add Noise Alpha to a layer, turn Cycle Noise on with Cycle 3, and set Noise
   Phase to -100. The viewer and an exported frame should show the same holes.
4. **Fractal Noise.** Add Fractal Noise, turn Cycle Evolution on with Cycle 5, and set Evolution
   to -500. Again the viewer and an export should match.

Nothing else should look different: every other effect draws exactly as before.

## Speed

On the graphics card, measured while other builds were running (so provisional), the reference shot played again took 63.0 ms a frame with Fractal Noise cycling a negative evolution against 56.7 without the cycle, and 36.4 against 32.6 for Noise Alpha: the fix costs nothing measurable beyond the noise between runs.
