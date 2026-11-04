# B-986 playtest: Lightning's Core Opacity and Glow Opacity on the graphics card

What changed (D-1106): a bolt with Core Opacity or Glow Opacity below 100 is now drawn by the
graphics card, like every other bolt, instead of falling back to the processor. The picture is
meant to be the same as before; only the speed changes.

1. Make a dark solid layer, add Lightning, set Glow about 50 and a pink glow colour. Set Core
   Opacity 75 and Glow Opacity 50 (the tutorial's retro bolt).
   - Expect: the same faint-glow bolt as in B-931's playtest
     (`verification/D-1051 pictures/2_tutorial.png`).
2. Play it with the graphics card preview on.
   - Expect: it plays about as smoothly as a bolt at 100 and 100 does; before this change the
     opacity below 100 made it slower.
3. Key Core Opacity 0 at the start and 100 a second later, and play.
   - Expect: the core fades in smoothly, with no flicker or jump between frames.
4. Export a frame and compare it with the viewer.
   - Expect: they look the same.

Pass if every "Expect" holds. If anything looks different, say which step.

Checked by `tests/b931_lightning_opacity.rs`, `verification/B-931_lightning_opacity_table.md`
(62 of 62): every bolt is handed to the card, and FX-LOPAC-001 to 010 drawn on the card and on
the processor at Full and Draft, every frame, differ by 0 levels of 255 (the allowance is 1).
