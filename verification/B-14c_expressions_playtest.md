# B-14c: expressions in the window, by hand

Built on 2026-09-16 against D-59. The owner accepted B-14b on 2026-09-16 ("works") and asked, in
the same message, to be able to add and edit expressions "when alt-clicking a stopwatch/diamond
like in after effects". This sheet walks that.

The generated half is `verification/B-14c_panel_table.md`, 18 of 18: it sends the same requests
the page sends and checks what comes back. This sheet is for what a table cannot judge: whether
the box is where you expect it, whether typing into it feels right, and whether the red line
says something you can act on.

## What changed that you already know

**Alt-click on a diamond no longer removes every key.** W-23 made it do that; it now adds or
removes an expression, as After Effects' stopwatch does. To take every key off a property, press
the property's name on the timeline (which chooses all its keys), then press Delete.

## What to check by hand

Open any shot with a layer in it. `verification/B-14b playtest/expressions.json` works; so does
`Fixtures/reference_shot`.

1. **Alt-click adds an expression.** Select a layer. In the inspector, hold Alt and click the
   diamond beside Rotation. A box appears under Rotation with `value` in it, the text already
   selected so that typing replaces it, and a small orange `=` appears beside the word
   Rotation. The picture does not change: `value` is the rotation the layer already had.
2. **Typing applies when you leave the box.** Type `time * 90` and click anywhere else. The
   layer turns as you play: a quarter turn each second. The Rotation number in the inspector
   follows the playhead, as a keyed number does.
3. **Ctrl+Enter applies too, and Escape takes back what you typed.** Click into the box, change
   90 to 180, and press Ctrl+Enter: the layer now turns twice as fast. Click in again, type
   something, and press Escape: the box goes back to `time * 180` and nothing changes.
4. **Keys typed in the box are text, not shortcuts.** In the box, type `wiggle(2, 30)` with a
   space in it. Space does not start playback, and the letters do not open property rows on
   the timeline. Ctrl+Z after leaving the box undoes the change to the expression.
5. **A half-typed expression is kept, with its error beside it.** Replace the text with
   `time *` and click away. The text stays as you typed it. A red line under the box reads
   something like "⚠ EXPRESSION_SYNTAX: … The rotation is drawn at its keyed value." The layer
   stops turning and sits at its own rotation. Finish it (`time * 90`) and the red line goes.
6. **The switch.** Click the `=` beside Rotation. It becomes a grey `≠`, the text greys out, the
   layer goes back to its own rotation, and the red line (if any) is gone. Click it again and
   the expression is back. Each click is one Ctrl+Z.
7. **Alt-click again removes it.** Alt-click the Rotation diamond. The box and the `=` are gone.
   Ctrl+Z brings them back, text included.
8. **Emptying the box removes it too.** Delete all the text in the box and click away.
9. **On the timeline.** Press R to open the layer's Rotation row. Alt-click its diamond there.
   An Expression row appears under Rotation with the same box across the bar, and the keyboard
   goes to that box, not the inspector's. What you type in either place shows in the other once
   you click away.
10. **A position takes a pair.** Alt-click Position and type `[value[0] + 100, value[1]]`. The
    layer sits 100 pixels to the right of where it was. Type `100` alone and click away: the
    red line says the type is wrong (EXPRESSION_TYPE), because a position is two numbers.
11. **The camera.** In the Camera panel, Alt-click the diamond beside Lens and type
    `2000 + time * 500`. Play: the lens lengthens and the parallax flattens. The Camera group
    at the top of the timeline shows the same Expression row under Lens when it is open.
12. **Without the mouse.** Tab to a diamond (the inspector's or the Camera panel's) and press
    Alt+Shift+=. That adds the expression, as After Effects' shortcut does; pressed again on the
    same diamond, it removes it.
13. **A locked layer refuses.** Lock the layer and Alt-click a diamond. Nothing is added, and
    the status line says the layer is locked.
14. **Save keeps it.** Save, close the window, open the file again. The expressions are there,
    a switched-off one is still switched off, and a broken one still shows its red line.

## Known limits

- The box has no colours, no autocomplete, and no pick whip. It is plain text.
- `thisComp.layer("…")` takes the layer's ID, not its name (B-14b's rule). The layer list does
  not show IDs yet.
- Only a layer's six transform properties and the camera's three can carry an expression, as
  D-59 says. Effect settings cannot.

## What to answer

"works", or which step number did something else and what it did.
