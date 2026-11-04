# B-931 playtest: Lightning's Core Opacity and Glow Opacity

What changed (D-1051): Lightning has a **Core Opacity** and a **Glow Opacity**, each 0 to 100,
below its Opacity, each with a key diamond. That is After Effects' Advanced Lightning, where the
tutorial (uGxruFKgt6U) sets Core Opacity 75 and Glow Opacity 50 for a softer retro bolt. Both at
100 is the bolt exactly as before.

1. Make a solid layer, dark, and add Lightning from the effect search. Set Glow to about 50 and
   the glow colour to pink.
   - Expect: the bolt as before, a white core in a strong pink glow. Core Opacity and Glow
     Opacity both read 100.
2. Set Core Opacity 75 and Glow Opacity 50.
   - Expect: the same bolt, same shape; the glow much fainter and the white core a little
     see-through. Nothing gets brighter.
3. Set Core Opacity 0, Glow Opacity 100.
   - Expect: no white core at all, only the pink glow along the bolt.
4. Set Glow Opacity 0, Core Opacity 100.
   - Expect: only the thin white core, as if Glow were 0.
5. Type 101 into Core Opacity.
   - Expect: it is refused with "Lightning Bolt's core opacity runs from 0 to 100, and this is
     101." and nothing changes.
6. Key Core Opacity 0 at the start and 100 a second later, and play.
   - Expect: the core fades in smoothly over the glow. Ctrl+Z takes the keys away again.
7. Save, close and open the project again.
   - Expect: both opacities and the keys are as you left them. A project saved before this
     change opens with the bolt looking exactly as it did.

Pass if every "Expect" holds. If anything looks different, say which step.

Checked by `tests/b931_lightning_opacity.rs`, `verification/B-931_lightning_opacity_table.md`
(52 of 52), against `verification/D-1051 pictures/`: `1_full.png` (both at 100, the old bolt),
`2_tutorial.png` (core 75, glow 50) and `3_glow_only.png` (core 0, the glow alone).
