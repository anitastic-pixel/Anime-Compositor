# B-521: Noise Gate (the fourth step of the sound plan)

Built on 2026-10-13 as the fourth step (S-4) of the sound plan you accepted (D-638), decided as
D-642. Noise Gate is the first sound effect: it silences a sound while it is quieter than a
level you choose, so hiss and hum between words drop out. Sound layers can now hold sound
effects, and only sound effects; picture layers cannot hold them.

The check, `verification/D-642_noise_gate_table.md` (97 of 97 checks pass), runs fourteen small
test sounds through the gate and holds every result to numbers worked out before the code, down
to the sample where the gate opens and the sample where it is shut.

## What to look at

**The pictures** in `verification/D-642 pictures/`, each a waveform: left channel above, right
below. `_before` is the sound with the gate off, `_after` with it on.

1. `fx_gate_001`: a loud tone on a faint hiss. Before, the hiss runs the whole length (a thin
   line). After, the hiss is gone before the tone, and after the tone it fades out rather than
   stopping dead.
2. `fx_gate_013`: two tones, one quiet, one loud, through two gates, the second set to -20 dB.
   After, the quiet tone is gone and only the loud one is left.
3. `fx_gate_007`: a steady sound with the gate's threshold rising over the first ten frames.
   After, the sound stops part-way through, where the threshold climbs past it.

## What to check in the app

4. Select a sound layer. **Add effect** now works on it, and the Effects list offers only
   **Audio: Noise Gate**. Select a picture layer: Noise Gate is not offered.
5. Add Noise Gate to a sound with quiet hiss between louder parts (dialogue is ideal). Set
   **Threshold** to about -40 dB and play: the hiss between the loud parts is gone.
6. Raise **Release** to 2000 ms and play again: the sound now fades out slowly after each loud
   part instead of cutting off.
7. Set **Hold** to 0 and **Release** to 5 ms: the sound gets choppy, as a fast gate does.
8. Key **Threshold** (the stopwatch) from -60 dB to -20 dB over a few seconds and play: the
   quieter parts drop out more and more. There is no click where the keys change.
9. The gate's card has no **Mix** row (a sound has no picture to mix).
10. Export the composition as an MP4 and play it: the gate is heard in it exactly as in the
    window.
11. **Older projects.** Open a project that already has sound in it: it plays, draws and saves
    exactly as before.

## Not built (known limits)

- Noise Gate has no look-ahead, side chain or range (how far it closes); it closes all the way.
- The window mixes each stretch it plays from the start of the gated layer, so a long gated
  sound costs a little more to preview the later you start playing. Measured at a few
  milliseconds for two seconds; recorded, not yet a problem.
- Compressor and Distortion follow as S-5 and S-6.
