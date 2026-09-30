# B-173: the card remembers the effects it drew for each frame

Built on 2026-09-30 under your "sounds great! let's do 1 through 7 to your discretion.", to fix
the slowdowns the quiet re-measure of 2026-09-30 found (D-246).

When a layer's effects change from frame to frame (Noise, Snowfall, Roughen Edges, Turbulent
Displace, or any effect with a keyed setting), the graphics card used to keep only the last frame's
result. Playing the shot a second time, every frame's effects were drawn again from nothing. Now the
card keeps the result for each frame it has drawn, as the processor already does, up to a quarter
of the memory the card may use, so a frame that comes round again is shown without drawing its
effects again. **The pictures are byte for byte the ones the card drew the first time**, and still
within 1 level of the processor's (`verification/B-173_draft_moving_table.md`: 34 of 34 frames
asked for again run no effect pass and give the very same picture; 68 of 68 frames drawn within 1
level). Exports never use the graphics card and are untouched.

Played again at Draft on the card, a frame now takes about 11.5 to 12 ms for every effect measured,
against up to 18.3 before (Bloom with a keyed intensity). Snowfall 13.4 to 11.6, Roughen Edges
12.7 to 11.9, B-155's moving Noise first 12.9 to 11.9. The first time through costs a little more,
about 0.3 to 0.5 ms a frame, because the card has to make room for the pictures it keeps
(`verification/B-173_timing_table.md`).

## What to check

Open the reference shot. Set **Draw on: GPU** and **Draft**. On one layer, add Snowfall; on
another, Roughen Edges with its Evolution keyed; on a third, a Noise followed by Levels.

1. **Play twice.** Play the whole shot, then play it again. The second time should be at least as
   smooth as the first. Nothing should look different between the two plays.
2. **Scrub back.** Stop, drag the time marker back over frames already played, then forward. Every
   frame should look as it did when it played.
3. **Change a setting, play again.** Change Snowfall's amount (or any setting) and play again. The
   new setting must show on every frame, not the old picture.
4. **Switch an effect off.** Turn Roughen Edges off with its checkbox and play; turn it back on and
   play. The picture should follow each time.
5. **Full.** Switch to **Full resolution** and repeat steps 1 and 3.

## If something is wrong

Say which step. The most likely fault would be an old picture shown after a change (steps 3 and 4),
because the card now keeps pictures from earlier frames. The check asks for frame 0 again after
frame 100 and compares every byte with the first picture; a change of setting gives the layer new
settings, which never match a kept picture.
