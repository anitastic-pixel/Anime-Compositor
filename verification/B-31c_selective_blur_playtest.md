# B-31c: Selective Colour Blur in the Effects panel, by hand

Built on 2026-09-25 against D-87, which you accepted the same day ("accept").

The generated halves are `verification/B-31b_selective_blur_table.md`, 69 of 69, which renders
every FX-SELBLUR case from document 25 and compares it with the reference numbers, and two
panel tables: `verification/B-12a_effects_table.md`, which adds Selective Colour Blur from the
window and checks it lands at the very top of the stack, above Line Smoothing, at blur 12 with
no colour chosen, and `verification/B-12b_state_fields_table.md`, which checks the settings the
panel sends are the ones the command reads. This sheet covers what the tables cannot: how it
looks on a drawing.

## Before you start

Open any project you are happy to change without saving. The test drawing is
`verification/B-31c_selective_blur_drawing.png`, 200 by 150, the same face as the proposal's
pictures: skin, a shadow with no line along its edge, a highlight, a blush, a black outline and
a black line across the shadow, all drawn without antialiasing. It has exactly five colours:

| Colour | Written |
| --- | --- |
| skin | `#f6d6be` |
| shadow | `#dba08e` |
| highlight | `#fff3e8` |
| blush | `#f09696` |
| line | `#1e1a24` |

Click **Import drawings...**, choose it, and make a layer from it. Press **Full resolution** if
it is not already on, and zoom the viewer in (Ctrl with the mouse wheel, or the Zoom slider)
until you can see the shadow's edge clearly.

Revised the same day: the first version of this sheet asked you to type the hex into the
colour window, which on Windows has no such box. The card now has a box of its own for each
colour.

## What to check

1. **In the list.** With the layer selected, open **Add effect...** under Effects. **Selective
   Colour Blur** is the last choice, after Line Smoothing.
2. **Adding it.** Choose it. A card named Selective Colour Blur appears with a **Blur, 0 to
   200** box at 12 and a **Colours** row that says no colour is chosen, and below it **Add:**, an empty box showing `#rrggbb`, the word "or" and a white swatch. The picture
   does not change yet.
3. **Choosing colours.** Click in the empty box, type `f6d6be` and press **Enter** (the `#` is
   optional). A swatch appears with a box holding `#f6d6be` beside it. Still no change in the
   picture: skin has nothing to mix with. Add a second colour the same way, `dba08e`. Now the
   edge between skin and shadow turns into a soft ramp, as in the proposal's first picture.
4. **Lines untouched.** The black outline and the black line across the shadow stay exactly as
   drawn, sharp, with no skin or shadow colour mixed into them. The softening stops at the black
   line: the two sides of it are soft only where skin meets shadow.
5. **The others untouched.** The highlight and the blush stay flat and sharp-edged. Add
   `fff3e8` as a third colour: now the highlight's edge goes soft into the skin as well.
6. **Changing and removing a colour.** Click a swatch and pick a different colour in the
   colour window, such as pure white, then OK: the edge it was softening goes back to sharp and
   the box beside the swatch shows `#ffffff`. Click the ✕ beside a swatch: it is
   removed and its edges go back to sharp.
7. **One step off does nothing.** Click in the skin colour's box, change it to `f6d6bf`, one
   step off, and press Enter. The skin is no longer softened. Change it back to `f6d6be`.
   Type `pink` instead: it is refused with a sentence saying a colour is written # and six hexadecimal digits.
8. **Blur.** Drag the Blur box down to 0: everything is exactly as drawn. Up to 40: the ramp
   is wider and softer than at 12.
9. **Top of the stack.** Add a **Line Smoothing** to the same layer. It goes directly below
   Selective Colour Blur, not above it. Add a second **Selective Colour Blur**: it goes to the
   very top. Delete the second one and the Line Smoothing.
10. **Out of range.** Type 300 in Blur. It is refused with a sentence saying blur runs from 0
    to 200, and the card keeps its old number. Add colours until there are eight: the **Add a
    colour** row then goes away.
11. **Keyed.** Click the key diamond beside Blur on frame 0 with Blur at 0. Go to a later frame
    and set it to 24. Scrub between: the softening grows from nothing.
12. **Bypass and undo.** Switch the effect off on its card: the edges are sharp again. Switch it
    on, then press **Ctrl+Z** a few times: each step goes back one change.
13. **Saved.** Save the project (**Ctrl+S**), close the window and open it again. The card is
    there with its colours, blur and keys, and the picture is the same.
14. **Your own drawing.** If you have a cel drawn without antialiasing, import it and try it
    on its skin and shadow colours.

## Known limits, on purpose

- A colour is chosen only when it matches exactly. A drawing that was antialiased, scanned or
  saved as JPEG has few exact colours, and gains little (FX-SELBLUR-008).
- A gap in a line lets the softening through it.
- The Windows colour window has no box for a hex value, which is why the card has its own.
  Typing the hex value is the sure way. Choosing a colour by clicking the picture
  is not in D-87.

## What to answer

"works", or which step number did something else and what it did.
