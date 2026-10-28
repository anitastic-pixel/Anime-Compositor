# B-813 playtest: Light Sweep's rim

What changed (D-933): Light Sweep has a new setting, **Edge Look**. **Rim** lights a soft, thin
edge all round the drawing, brightest on the side facing the Direction, whatever the band is
doing; **Band** is the old look, where only the edges inside the band light up. A Light Sweep
added from the Effect menu starts on Rim; saved projects keep Band and look exactly as before.
This is the tutorial n0g4HXxx03s's glass pill edge.

1. Draw a rounded rectangle (or open a drawing on a clear background). Add Effect > Generate >
   Light Sweep.
   - Expect: Edge Look **Rim**, and a thin white edge all round the shape, brightest at the top
     left (Direction -30), as well as the soft band across the middle.
2. Set Sweep Intensity to 0.
   - Expect: the band goes; the rim stays all round.
3. Set Width to 0.
   - Expect: the rim still stays.
4. Set Direction to 180.
   - Expect: the rim is now brightest along the bottom.
5. Set Edge Thickness to 4, Edge Intensity 60, Light Reception **Cutout**.
   - Expect: only a soft rim of white light is left, as in the tutorial.
6. Switch Edge Look to **Band**.
   - Expect: the old look: only the parts of the edge inside the band are lit (with Sweep
     Intensity 0 and Width 100, two short bright bars where the band crosses the edge).
7. Open a project saved before this change that has a Light Sweep.
   - Expect: Edge Look shows **Band** and the picture is exactly as before.
8. Save, close and reopen.
   - Expect: every setting reads as you left it and the picture is the same.

Pass if every "Expect" holds. If anything looks different, say which step.

Known limit: with Rim and Edge Intensity above 0 the sweep is drawn on the processor, not the
graphics card, so it can be slower on big frames.

Checked by `cargo test --test b813_light_sweep_rim` (33 of 33, `B-813_light_sweep_rim_table.md`):
every pixel of FX-SWEEP-027 to 036 against `tools/light_sweep_rim_reference.py`, the wrong word
refused, round trips and tiles. The old cases are unchanged: `b134_light_sweep` 106 of 106.

The sheet `verification/B-813 pictures/light_sweep_rim_vs_tutorial.png` puts the tutorial's glass
pill (After Effects) above ours before (Band: two bars) and now (Rim), the same settings. Pass if
ours now shows a thin rim all round the pill, brightest at the upper left, as the tutorial's does.
