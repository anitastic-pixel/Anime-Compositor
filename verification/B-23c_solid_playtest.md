# B-23c: solid layers in the window, by hand

Built on 2026-09-19 against D-74, which the owner accepted the same day ("accept D-74, proceed
with B-23b").

The generated half is `verification/B-23c_panel_table.md`, 18 of 18. It calls what the window
calls and checks what comes back: New solid needs no drawing chosen, is the composition's size
and mid grey, is named Solid 1, Solid 2 and so on, lands above the chosen layer, takes a colour
and a size from the panel, refuses a size or colour out of range with a sentence, and comes back
on Undo. Whether the pixels are right is B-23b's table, against `Fixtures/solid`. This sheet
covers what neither can judge: what you see in the viewer, on the timeline and in the panels.

## Before you start

Open any project with at least one layer, or use **Save As...** first and work on the copy.
Nothing below changes the fixtures.

## What to check

1. **The button.** Beside **New adjustment layer** is **New solid**. It is usable as soon as a
   composition is open, with nothing chosen under Drawings. Hovering it says what a solid is.
2. **Adding one.** Choose a layer in the layer list and click **New solid**. A layer named
   "Solid 1" appears just above the chosen one, marked "solid" at the end of its row. The viewer
   is covered edge to edge in mid grey, over the chosen layer and under anything above it. Its
   bar on the timeline is the same grey, runs the whole composition, and hovering it says
   "a solid, 1920 by 1080 pixels of one colour" (or your composition's size).
3. **The key.** With nothing chosen, press Ctrl+Y. "Solid 2" appears, at the front. Press
   Ctrl+Z twice: both are gone. Ctrl+Y is also listed in the command palette (Ctrl+Shift+P) and
   in the timeline's right-click menu as "New solid". Ctrl+Alt+Y still makes an adjustment layer.
4. **The inspector.** Add a solid and choose it. Drawing reads "none: a solid". Below it are
   **Colour**, a colour swatch showing the grey, and **Width** and **Height** showing the
   composition's size in pixels. The Exposures panel says "A solid has no drawings to expose."
   and **Add an exposure** is greyed out.
5. **The colour.** Click the swatch. The system's colour picker opens on the grey. Choose a pure
   red and close it. The viewer turns red where the solid is, the bar on the timeline turns red,
   and the swatch shows red. Ctrl+Z puts the grey back, in the viewer, the bar and the swatch.
6. **The size.** Drag across **Width** as you would any other number in this panel: the solid
   narrows and widens under the hand, and letting go is one thing to undo. Escape part way
   through puts it back. Pressing it without dragging opens a field to type into, and the arrow
   keys step it a pixel at a time.
   Type 960 in **Width** and press Enter. The solid is half as wide and still
   centred: the left and right quarters of the frame show what is beneath. The anchor in the
   inspector reads half what it did across. Type 0 in Width: the status line says the width must
   be from 1 to 8192, and nothing changes. One Ctrl+Z puts the full width back.
7. **Like any layer.** Move, scale and rotate the solid in the inspector, set its opacity to
   50%, and pick **Multiply** from its Blend list: each does what it does to a drawing. Hide it
   with its eye and it is gone from the viewer.
8. **As a matte.** Make a solid half the composition's width, move it to one side, and choose it
   as the matte of the layer beneath, in that layer's Matte row. Only the half of that layer under
   the solid shows.
9. **Saved and reopened.** Save, then **Open** the same file. The solid is there, with its
   colour, size and place in the order. It never shows as a missing drawing.
10. **Export.** Export a few frames as PNG. The exported frames show the solid as the viewer did.

## Known limits

- The window has no Layer menu of its own: New solid is a button beside New adjustment layer,
  a line in the timeline's right-click menu and the palette, and Ctrl+Y. That is where
  New adjustment layer already stands.
- The picker works in the screen's 256 levels. A colour written into a file by hand between two
  of them is kept and drawn exactly (D-74); the swatch shows the nearest level.
- Colour and size are not animated (D-74): they have no stopwatch, and Scale is what grows a
  solid over time. Whether they should be keyable is D-76, proposed and awaiting the owner.
  A new solid is always mid grey; After Effects remembers the last colour, this does not.
- A solid is drawn whole: one of 8192 by 8192 takes a gigabyte while it is drawn (D-74).

## Answered

"B-23c playtest works", 2026-09-19. The owner asked for Width and Height to be draggable,
which step 6 above now covers, and asked whether the colour and size could be keyed: that is
D-76, proposed and awaiting them.

## What to answer

"works", or which step number did something else and what it did.
