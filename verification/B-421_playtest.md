# B-421: Linear Color Key, tolerance 0 and Keep Colors

Built on 2026-10-11 as D-541, on your word "for keylight, build our own, proceed with your
recommendations, even if we may need to rewrite/change things". Two fixes to Linear Color Key
(B-384, whose playtest is `verification/B-384_playtest.md`):

- **Matching Tolerance 0 keys nothing.** Adobe says "A value of 0 makes the entire image opaque".
  Before, 0 still took out the key colour itself. This changed two of D-504's checked pictures,
  FX-LCKEY-007 and the first frame of FX-LCKEY-011; both are now the layer as it was.
- **Key Operation** (Key Colors when added): a new menu. **Keep Colors** on a second Linear Color Key, below
  one that keys, brings back a colour the first took out. It brings the colour back from before
  the keys directly above it, so the colour is the original one.

The check, `verification/D-504_linear_color_key_table.md` (161 of 161), holds every pixel to
numbers worked out by a separate program before the code existed, and holds the graphics card's
picture to within 1 level of the processor's. The new picture is in `verification/D-504
pictures/`.

## What to check

1. **Brought back.** `5_keep_shirt.png`: the green-screen shot with a first Linear Color Key at
   tolerance 100 (everything gone), then a second set to Keep Colors on the shirt's blue: only the
   shirt is back, looking exactly as it did. Compare `1_before.png`.
2. **In the app, Keep Colors.** Add Linear Color Key to footage and key the background with a
   wide tolerance, so part of your subject goes too. Add a second Linear Color Key below it, set
   Key Operation to Keep Colors and Key Color to the lost part's colour: that part comes back.
   Switch the view of the second one to Matte Only to see what is now kept.
3. **In the app, tolerance 0.** Drag Matching Tolerance down to 0: the whole layer is back, even
   pixels exactly the key colour.
4. **Something in between.** Put an Exposure between the two Linear Color Keys: the Keep Colors
   now brings nothing back, because another effect ends the run of keys above it.
5. **Saved and opened again.** Save, close and open: Key Operation is still Keep Colors. A project
   saved before this change opens with Key Colors.

Our own choices you may want to judge: only Linear Color Keys directly above count (another
keyer, such as Color Key or Color Range, in between ends the run; Adobe does not say what After
Effects does there); and the rule for how much comes back is ours, since Adobe publishes none.

## Speed

Not measured again: the change adds no work to a stack without a Keep Colors. A stack with one
keeps one extra copy of the layer and draws its keys on the processor (B-384's figures, about 2.6
to 3.1 ms a 1080p layer per key).
