# B-929 playtest: CC Composite

What changed (D-1049): a new effect, **CC Composite** (Effects panel, Color Correction, beside
Solid Composite). It puts the layer back, as it was before its effects, on top of (or behind)
what the effects above it made. That is how After Effects' long-shadow tutorial puts the logo
back on top of its shadow.

1. Make a text or shape layer, for example a white logo, on a dark background.
2. Add Gaussian Blur (Blurriness 30), then Fill in black.
   - Expect: a soft black blob where the logo was.
3. Add CC Composite below them (search "cc composite" or "blend with original").
   - Expect: its card shows Opacity 100, Composite Original "In Front", RGB Only "Off".
   - Expect: the sharp white logo sits on top of the soft black shadow.
4. Set Composite Original to "Behind".
   - Expect: the black blob covers the logo again (the logo is now underneath).
5. Back to "In Front", Opacity 50.
   - Expect: the logo half seen over the shadow.
6. Key Opacity from 0 to 100 over a second and play.
   - Expect: the logo fades in over the shadow.
7. Try Add, Multiply and Screen.
   - Expect: Add and Screen make the logo brighter over the shadow; Multiply darker.
8. Turn RGB Only on.
   - Expect: the outline is the shadow's (soft); only the colours inside it change.
9. Save, close and open the project again.
   - Expect: the CC Composite and its settings are as you left them.

Pass if every "Expect" holds. If anything looks different, say which step.

Checked by `tests/b929_cc_composite.rs`, `verification/D-1049_cc_composite_table.md` (95 of 95),
with pictures in `verification/D-1049 pictures/`: `3_blue_then_in_front.png` should look exactly
like `1_before.png`, and `4_blue_then_behind.png` exactly like `2_blue.png`.
