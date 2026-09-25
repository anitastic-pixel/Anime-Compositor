# B-41: Colour Key, by hand

Built on 2026-09-25 against D-97, which you accepted the same day as the last of the batch of
seven ("knowing the expectation, let's do a batch of 1 thru 7 in your recommended order").

The generated halves are `verification/B-41_color_key_table.md`, 90 of 90, which renders every
FX-KEY case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, 93 of 93, which checks the Colour Key card sends
every setting the command reads. This sheet covers what the tables cannot: how it looks and
feels in the window. The picture in `verification/B-41a proposal/` shows what to expect.

## Before you start

Use `verification/B-41a proposal/screen_drawing.png`, the face in front of a green screen
`#00b140`. Its shadow darkens the screen's green towards `#006424`, and green light has spilt on
the outline all round, making it dark green. Import it, make a layer from it and press **Full
resolution**. A colour layer of any other colour underneath makes the holes easy to see.

## What to check

1. **Adding it.** Pick **Colour Key** in **Add effect…**. It goes to the top of the stack, with
   the other effects that match exact colours, and the card shows **Tolerance** 20, **Softness**
   20, **Match By** Red, green and blue, and no colours. Nothing changes yet.
2. **The screen.** Add `#00b140` to its colours, as in Select Colour. The screen goes. The
   shadow's pale outer part goes too, its middle fades, and its dark centre stays, as in the
   picture's third panel. The face and its outline stay whole.
3. **Softness.** Set Softness to 0: the shadow now stops at a hard edge. Set it to 60: the fade
   spreads further into the shadow.
4. **Tolerance.** Softness 0 and Tolerance 80: all the shadow goes, even its dark centre, and the
   outline still stays.
5. **Hue.** Set **Match By** to Hue, Tolerance 10 and Softness 20: the whole shadow goes however
   dark, as it is the screen's own hue, and the outline goes too where green has spilt on it, as
   in the fourth panel. Set it back to Red, green and blue and the outline returns.
6. **Two colours.** Back on Red, green and blue, Tolerance 20 and Softness 0, add the skin,
   `#f6d6be`, as a second colour: the skin goes as well; the highlight, the cheek and the
   shading stay.
7. **Out of range.** Type 300 in Tolerance: it is refused with a sentence saying the tolerance
   runs from 0 to 255, and the card keeps its old number. Type -5 in Softness: refused, 0 to 255.
8. **Keyed.** Key Tolerance at 0 and at 100 at a later frame. Scrub between: the screen, then
   the shadow, goes smoothly as the tolerance grows, with no jumps.
9. **Draft.** Press **Draft**: the picture is smaller, but the same parts are taken out.
10. **Undo and saved.** Ctrl+Z steps back each change, the Match By word and the colours
    included. Save, close and open again: every setting and key is still there.

## Known limits, on purpose

- It takes the screen out; it does not take the screen's green light off what stays, so a
  spilt-on outline stays dark green (no spill suppression).
- With Hue, anything of the screen's hue goes however dark or pale, the spilt-on outline
  included. Red, green and blue is the safer choice for a figure with a line.
- A grey has no hue, so with Hue a grey colour takes nothing out until the tolerance is 255.

## What to answer

"works", or which step number did something else and what it did.
