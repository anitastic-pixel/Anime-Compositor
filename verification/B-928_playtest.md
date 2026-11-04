# B-928 playtest: "Effect Opacity" in the effect search

What changed (D-1048): After Effects keeps an effect's strength under Compositing Options >
Effect Opacity. Ours is **Mix**, the last row of every effect card. Typing After Effects' name for
it now tells you where Mix is, instead of finding nothing.

1. Select any layer and press Ctrl+Space. Type `effect opacity`.
   - Expect: a line at the top of the results: "Effect Opacity is called Mix here: the last row of
     every effect card, 0 to 100 per cent, keyable." No "No effect or preset is called..." line.
2. Clear it and type `compositing options`, then `blend with original`.
   - Expect: the same line each time.
3. Type `glow`.
   - Expect: Glow and the other matches as before, and no Mix line.
4. Press Escape. Open the Effects panel and type `Effect Opacity` in its search box.
   - Expect: the same line at the top of the list, in place of "No effect matches".
5. Type `opacity` alone.
   - Expect: no Mix line (a layer's own Opacity is a different thing).
6. Add any effect, for example Gaussian Blur, and look at the bottom of its card.
   - Expect: the Mix row the line pointed to.

Pass if every "Expect" holds. If anything looks different, say which step.

Checked by the app test `d1048_effect_search_names_mix_for_after_effects_words`, which pins the
rule in the page and works it on the 11 rows of FX-MIXNAME-001 in document 25 (11/11).
