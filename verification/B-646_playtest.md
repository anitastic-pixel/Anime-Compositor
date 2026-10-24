# B-646 playtest: After Effects' Expression Controls, and if/else in expressions (D-766)

## What changed for you

The Effects list has a new group, **Expression Controls**, with After Effects' six: **Slider
Control**, **Checkbox Control**, **Angle Control**, **Point Control**, **Color Control** and **Layer
Control**. Each holds one setting you can key, and draws nothing. An expression reads it the way the
After Effects tutorials type it:

    wiggle(2, effect("Slider Control")("Slider"))

Expressions also understand `if` / `else`, comparisons (`==`, `!=`, `<`, `<=`, `>`, `>=`) and logic
(`&&`, `||`, `!`), so a tutorial line such as

    if (effect("Checkbox Control")("Checkbox") == 1) 100 else 0

works on Opacity. A mistake in one of these lines (a misspelled control name, a setting the control
does not have, an `if` with nothing after it) is refused with a sentence saying what is wrong, never
silently.

Before, ours had none of these effects (a project from After Effects that used one said "this build
cannot draw this effect") and an expression with `if` or `effect(...)` failed.

A project you saved before this change opens and saves exactly as it did.

## What to look at

`verification/B-646 pictures/1_slider_wiggle_ae_before_after.png`: a Slider Control keyed 0 to 100
drives a wiggle's amount. Left, what the tutorials say; middle, before (the dot never moves); right,
after (the trail starts still and shakes harder as the slider rises: 16.5 px in the first half
second, 77 px in the last).

`verification/B-646 pictures/2_checkbox_opacity_ae_before_after.png`: a Checkbox Control, off until
frame 24, switches a box on through `if ... else`. Before, the box shows on both frames; after, it is
hidden on frame 23 and shown on frame 24.

`verification/B-646 pictures/3_wiggle_amplitude_ae_before_after.png`: a Slider held at 100 now
reaches the wiggle (peak 0.876 of the amount in 2 seconds, matching our wiggle(2, 100) within
0.35 px). This picture also shows a difference that remains, see below.

`verification/B-646_expression_controls_table.md`: 125 of 125 numbers and diagnostics match
document 25 (FX-EXPR-017 to 020), and opening and saving both expression projects keeps every byte.

## Try it

1. Add a solid. In the Effects list, open Expression Controls and add **Slider Control**. Type 100 in
   its Slider.
2. Alt-click the solid's Position stopwatch and type `wiggle(2, effect("Slider Control")("Slider"))`.
   Play: the solid shakes. Set the Slider to 0: it stops.
3. Add a second Slider Control. Its row says **Slider Control 2**, and
   `effect("Slider Control 2")("Slider")` reads it.
4. Add a **Checkbox Control** and type `if (effect("Checkbox Control")("Checkbox") == 1) 100 else 0`
   on Opacity. Tick the checkbox (1) and untick it (0): the layer shows and hides.
5. Type `effect("Slidr Control")("Slider")`: the expression is refused with a sentence naming the
   missing effect.
6. Open a project you saved before today: it looks the same.

## Known differences from After Effects (not fixed here)

- The controls cannot be renamed. After Effects lets you rename "Slider Control" to, for example,
  "Amount"; ours keeps After Effects' own names (and "Slider Control 2" and so on for a second one).
- The pick whip does not reach effect settings yet: type the `effect(...)(...)` line by hand.
- An expression on a Point or Color Control's own setting is refused, as on any two- or
  three-number effect setting today.
- The Checkbox is typed as 1 (on) or 0 (off).
- The controls have no Mix (they draw nothing).
- Our wiggle never goes past 1.00 times its amount; the After Effects reports read by lane q put its
  peaks at 1.09 to 1.25 times. B-646 brings the slider to the wiggle; it does not change the wiggle.
