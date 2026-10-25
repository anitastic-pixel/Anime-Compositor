# B-702 playtest: an expression's colours are After Effects' numbers (D-822)

What changed: a colour typed in an expression, such as `[1, 0.5, 0, 1]`, used to be read as this
build's linear numbers (B-700), so it came out a yellow-orange, #ffbc00, where After Effects
shows orange, #ff8000. Now a colour in an expression is After Effects' 0 to 1 numbers, the ones
its colour picker shows. Reading a colour (a Color Control's, a Fill's) gives those numbers too:
a Color Control set to the build's (0.25, 0.5, 0.75) now reads about (0.537, 0.735, 0.881), as
After Effects would show it. Only in Float without AE 32 bpc (After Effects' Linearize Working
Space) do expressions keep linear numbers, as After Effects does there.

A Color Control linked into a Fill or Drop Shadow still gives exactly the colour it held before:
the numbers go out and come back the same.

## What the pictures show

`B-702 pictures/`, frame 0 of a full-frame Fill:

- `q1_typed_orange_display.png`: left, what B-700 drew for `[1, 0.5, 0, 1]` (#ffbc00). Middle,
  this build with the expression. Right, After Effects' orange #ff8000 typed in as a colour. The
  middle and right must be the same orange (centre pixel 255, 128, 0 in both).
- `q2_typed_float.png`: the same expression in a Float composition, where After Effects with
  Linearize Working Space on gives the linear reading, #ffbc00.

`B-700 pictures/p2_circle_before_after.png` and `neon_after.png` are re-made: the earlier
sheets dropped the picture's see-through parts, which made the Circle look like a hard square
block. Nothing in the frames changed; only the sheet maker.

## Try it yourself (3 minutes)

The projects are in this session's scratch folder (`wave8/w8e/proofs/q1_typed_display.json` and
`q2_typed_float.json`).

1. Open `q1_typed_display.json`.
   - Expected: the whole frame is orange, the same orange as After Effects' #ff8000, and nothing
     is said beside the frame.
2. In After Effects (if you have it), put a Fill on a solid and give its Color the expression
   `[1, 0.5, 0, 1]`.
   - Expected: the same orange.
3. Open `q2_typed_float.json` (a Float composition).
   - Expected: a lighter yellow-orange, as After Effects gives with Linearize Working Space on.

Pass if steps 1 and 2 show the same orange and step 3 the lighter one.
