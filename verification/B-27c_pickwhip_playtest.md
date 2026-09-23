# B-27c: the pick whip in the window, by hand

Built on 2026-09-23 against D-83, which the owner accepted the same day ("proceed").

The generated half is `verification/B-27c_panel_table.md`, 22 of 22. It sends what the window
sends and reads back what the property was given: each of FX-WHIP-001 to 013 writes exactly
the text `Fixtures/pickwhip/expected_pickwhip.json` says, as one step that Undo names and takes
back; a property dropped on itself, a separated half and a made-up name are refused with a
sentence; and the parent whip sends the same request the Parent list sends. Whether the linked
values are right on each frame is B-27b's table. This sheet covers what neither can judge: the
line, the lit row and the label, and how it feels in the hand.

## Before you start

Open any project with at least two drawing layers, or use **Save As...** first and work on the
copy. Add a null (**New null**) so there is something to link to. Nothing below changes the
fixtures.

## What to check

1. **The parent spiral.** Every layer row in the layer list has a small blue **@** after its
   switches. A sound's row has a gap there instead. Hovering one says "Drag onto another
   layer's row to make it ...'s parent".
2. **Parenting by dragging.** Press on a drawing's **@** and, holding the button, move the
   mouse. A thin blue line runs from the **@** to the pointer and follows it, and a small label
   rides beside the pointer. Move over Null 1's row: the row lights blue and the label says
   "Parent: Null 1". Let go. The drawing does not move on the picture, and choosing it shows
   Null 1 in its Parent row. Drag Null 1 on the picture: the drawing goes with it. Ctrl+Z once
   takes the parent away.
3. **Rows that cannot take it.** Drag a drawing's **@** again, slowly, over: its own row (label
   "a layer cannot be its own parent", row stays dark); a sound's row, if there is one ("a sound
   cannot be a parent"); the Camera row or a property row ("the camera and properties are not
   layers"); and empty space ("drop on a layer's row"). Letting go on any of them does nothing
   at all.
4. **A loop.** Parent the drawing to Null 1 (step 2), then drag **Null 1's** **@** over the
   drawing's row. It stays dark and the label says the drawing rides on Null 1, so that would be
   a loop.
5. **Beside the Parent list.** Choose a drawing. In the inspector, the Parent row has the same
   **@** after its list. Dragging it onto a layer's row does what step 2 did, and the list then
   shows that layer. Press Ctrl+Z so the drawing has no parent again for what follows.
6. **Escape.** Start dragging any **@**, move over a row that lights, and press **Escape**
   before letting go. The line, the label and the light all go, and nothing has changed.
7. **The value spiral.** Twirl open the drawing's properties (the arrow on its row). Alt-click
   the stopwatch diamond of its **Position**. An Expression row appears under Position with
   `value` in it, and between the **=** switch and the word "Expression" there is a **@**. The
   inspector's Position row has one too, after its **=**. A property with no expression has no
   **@**.
8. **Linking by dragging.** Twirl open Null 1's properties as well. Drag the drawing's
   Position **@** over Null 1's Position row: it lights and the label says
   "Null 1 › Position". Let go. The drawing's expression box now reads
   `thisComp.layer("...").transform.position` with Null 1's identifier in the quotes, and the
   drawing jumps to where Null 1 is. Drag Null 1 on the picture: the drawing follows its
   position exactly (not its turn or size, unlike a parent). Ctrl+Z once takes the link back,
   leaving `value`.
9. **Mixed sizes.** Give the drawing's **Rotation** an expression (Alt-click its diamond) and drag
   its **@** onto Null 1's **Position** row. The text ends in `.position[0]`: the drawing turns by
   Null 1's across position, in degrees. Give **Scale** an expression and drag its **@** onto
   Null 1's **Rotation**: the text is two lines, `temp = ...rotation;` and `[temp, temp]`.
10. **The camera.** Twirl open the Camera row. Drag the drawing's Position **@** onto the Camera's
    **Zoom** row: label "Camera › Zoom", and the text is `thisComp.activeCamera.zoom` written as
    `temp = ...; [temp, temp]`. The camera's own properties, once given an expression, have a
    **@** too.
11. **Rows the value spiral cannot use.** Drag a Position **@** over: its own Position row ("a
    property cannot be linked to itself"); an effect setting's or a mask path's row, if one is
    open ("an expression cannot read ..."); and a layer's own row or empty space ("drop on a
    property's row").
    None lights, and letting go does nothing.
12. **A loop of links.** Link the drawing's Position to Null 1's Position, then give Null 1's
    Position an expression and drag its **@** onto the drawing's Position. Both are written.
    Both properties then show the EXPRESSION_CYCLE note under their boxes and stay on their own
    keys, rather than anything freezing. Ctrl+Z once takes the second link away and the note
    goes.
13. **Saved and reopened.** Save, then **Open** the same file. The links are
    still there. Rename Null 1: the links still follow it, since they name it by identifier.

## Known limits

- A layer's properties have to be twirled open before the drag starts. Hovering over a closed
  layer mid-drag does not open it.
- The value spiral only appears once a property has an expression (Alt-click the stopwatch
  first). After Effects also shows one in its Parent & Link column for every property; this
  window has no such column.
- Effect settings cannot be linked to or from: D-68 gives them keys and no expression.
- Nothing can be dropped on the picture itself, only on rows in the layer list.
- The pick whip is a mouse gesture. The same results are reached by typing the expression into
  its box, or by choosing from the Parent list.

## What to answer

"works", or which step number did something else and what it did.
