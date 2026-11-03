# B-814 playtest: CC Glass's AE Lights and rim look

What changed (D-934): CC Glass has a new setting, **Edge Look**, and a new Light Type, **AE
Lights**. With Distant or Point light, **Rim** keeps the flat middle of the glass its own colour
and darkens every slope, on all sides, as After Effects' Effect Light does in the Liquid Glass
tutorial; **Lit** is the old look. **AE Lights** shows the bend alone, not lit. A CC Glass added
from the Effect menu starts on Rim; saved projects keep Lit and look exactly as before.

1. Draw two coloured blocks on a clear background. Add Effect > Stylize > CC Glass.
   - Expect: Edge Look **Rim**; the blocks bent outward, their middles their own colour, their
     soft edges darker all round, a small highlight toward the top left.
2. Set Light Intensity to 0.
   - Expect: no darkening and no highlight; the blocks still bent.
3. Set Light Intensity back to 100 and Light Type to **Point Light**.
   - Expect: the darkening stays all round; the highlight moves toward the light's position.
4. Set Light Type to **AE Lights**.
   - Expect: the blocks bent, colours untouched, no light at all.
5. With AE Lights, set Displacement to 0.
   - Expect: the drawing exactly as it was without the effect.
6. Set Light Type to Distant Light and Edge Look to **Lit**.
   - Expect: the old look: the side facing the light brighter, the far side darker.
7. Open a project saved before this change that has a CC Glass.
   - Expect: Edge Look shows **Lit** and the picture is exactly as before.
8. Save, close and reopen.
   - Expect: every setting reads as you left it and the picture is the same.

Pass if every "Expect" holds. If anything looks different, say which step.

Checked by `cargo test --test b814_glass_look` (33 of 33, `B-814_glass_look_table.md`): every
pixel of FX-GLASS-028 to 038 against `tools/glass_look_reference.py`, the wrong word refused,
round trips and tiles. The old cases still pass: `b441_glass`, `b198_unsharp_glass` (5 of 5) and
`b224_gpu_maps` (card against processor).

Tutorial sheet: `verification/B-814 pictures/glass_vs_tutorial.png`, the Liquid Glass tutorial
(n0g4HXxx03s, 2:00-3:20) beside ours, rendered by the release app (lane w12c's pill project,
processor, full size). Top row, Effect Light: the middle of the pill, mean level (R, G, B) in a
flat patch, is tutorial (67, 90, 154), Lit before (75, 107, 184), Rim after (67, 97, 167): the
middle now keeps the background's own colour as the tutorial's does. The bottom rim band is
tutorial (40, 60, 120), Lit (55, 85, 165), Rim (53, 82, 160): darker than before but still not
as dark as the tutorial's rim (differs; the rim's floor is noted as a gap in D-934). Bottom row,
AE Lights: the bend alone, the same as lane w12c's ambient-only stand-in, now one menu choice.
