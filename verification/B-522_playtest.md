# B-522: Compressor (the fifth step of the sound plan)

Built on 2026-10-13 as the fifth step (S-5) of the sound plan you accepted (D-638), decided as
D-643. Compressor is the second sound effect: it turns down the parts of a sound louder than a
level you choose, so a voice or a piece of music sits at a more even level. Like Noise Gate, it
goes on sound layers only.

The check, `verification/D-643_compressor_table.md` (131 of 131 checks pass), runs sixteen small
test sounds through it and holds every result to numbers worked out before the code.

## What to look at

**The gain curves** in `verification/D-643 pictures/`, the files ending `_curve`. Across is how
loud the sound goes in, up is how loud it comes out, both from -60 dB (bottom left) to +12 dB;
the thin diagonal is "unchanged". Red is the rule, blue dots are what the build measured; the
dots should sit on the red line.

1. `fx_comp_001_curve` (the starting values): on the diagonal up to about -24 dB, then bending
   gently away from it (the knee), then flatter: loud sound comes out quieter.
2. `fx_comp_002_curve` (Knee 0, Ratio 4): a sharp corner at -16 dB; above it a quarter as steep.
3. `fx_comp_003_curve` (Ratio 0.4): above the knee it bends upwards (louder gets louder) until
   it hits the Output Limit at 0 dB and runs flat along it.

**The waveforms** (left channel above, right below; `_before` with the effect off):

4. `fx_comp_004`: a loud block. After, the start of the block has a short spike (the 6 ms the
   compressor takes to react), then it is turned down to a quarter of its height.
5. `fx_comp_007`: the same block turned up by Makeup Gain but held flat at the Output Limit.
6. `fx_comp_016`: the loud block is only in the right channel, but the quiet left channel is
   turned down with it at the same moments.

**The timing table** in the check: the turn-down reaches 63% of the way in 286 samples (6 ms, the
Attack), and with Auto Release on the sound comes back 63% of the way in 44 ms but takes 464 ms
to come back 90%; with Auto Release off, 438 ms and 1,013 ms.

## What to check in the app

7. Select a sound layer. The Effects list offers **Audio: Compressor** and **Audio: Noise
   Gate**; a picture layer is offered neither.
8. Add Compressor to dialogue or music with loud and quiet parts. Set **Threshold** to -30 dB and
   **Ratio** to 6 and play: the loud parts are turned down, so loud and quiet sit closer.
9. Raise **Makeup Gain** to +8 dB: the whole thing gets louder, the loud parts still held.
10. Set **Output Limit** to -6 dB: nothing comes out louder than that.
11. Set **Attack** to 0 and **Release** to 10 ms with Auto Release off: the sound gets a fast,
    pumping, squashed feel, as a fast compressor does.
12. Key **Ratio** (the stopwatch) from 1 to 10 over a few seconds and play: the squeeze grows
    smoothly, with no clicks where the keys change.
13. The card has no **Mix** row. Export the composition as an MP4: it sounds as in the window.
14. **Older projects.** Open a project with sound in it: it plays, draws and saves as before.

## Not built (known limits)

- No side chain (one sound driving another's compression) and no look-ahead; the level is each
  sample's peak rather than an average, so with very short Attack and Release a low note can
  make the level ripple.
- The Output Limit scales the loudest sample down to the limit at once; it is not a look-ahead
  limiter.
- Distortion follows as S-6, then keyed level, pan and sound through composition layers as S-7.
