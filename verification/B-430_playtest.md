# B-430: Deband, quicker on the graphics card

Built on 2026-10-11 as D-550, under your /loop request. **Deband** (Noise & Grain) now does its
blur on the graphics card in single precision instead of double, as D-228 did for other effects.
This card does double-precision sums at 1/64 the speed of single ones, so the blur was the slow
part.

Nothing you can see should change. Exports are made on the processor, which is not changed.

The check, `verification/D-456_deband_table.md` (157 of 157), was run again: every graphics-card
picture is within 1 level of the processor's, the same as before.

## What to check

1. **Same look.** Open a project with a Deband on a banded gradient (or add one): it looks as it
   did, banding hidden by the fine dither.
2. **Quicker.** Play it back with Blur at 30: it should keep up better than before.
3. **Export.** Export a few frames: the same as before this change.

## Speed

`verification/B-430_deband_single_precision_timing_table.md`. Three Debands on a 1920 by 1080 shot, milliseconds a frame played again, double
then single precision, run in turn: as added 55.1 against 29.1; blur 30, 76.5 against 31.6 (the
shot alone 14.7 and 12.1). PROVISIONAL: the machine was busy with other work.
