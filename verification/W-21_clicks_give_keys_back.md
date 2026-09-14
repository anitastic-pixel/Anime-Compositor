# W-21: a mouse click no longer holds on to the keyboard

Asked for on 2026-09-14: "I click the graph button, mess with keyframes, but when pressing play, I am still selecting the graph button for some reason, but I should be selected with the timeline still".

The cause: clicking a button gave it the keyboard, and a button with the keyboard answers Space itself. So Space pressed Graph again instead of playing. Clicking a layer row did the same. Now a mouse click on a button, a tick box or a list row gives the keyboard back to the window. Moving with Tab still works as before: a button reached with Tab keeps Space.

No photograph: this is a click followed by a key press, and the capture script can do neither.

## What to check by hand

Open `verification/W-16_key_shapes_project.json`:

1. **Graph, then Space.** Click the Graph tab, move a key dot, then press Space. The shot plays and the tab does not change. Space again stops it.
2. **Layer row, then Space.** Click layer4 in the list, then press Space. The shot plays.
3. **Other keys after a click.** Click any button (for example Graph), then press `K` and the arrow keys. They jump between keys and step frames as usual.
4. **Tab still works.** Press Tab until a button shows a focus ring, then press Space. That button is pressed and playback does not start.
5. **Typing is unchanged.** Double-click a layer name to rename it and type a space. The space goes into the name.

## What checks it by machine

- The existing app tests still pass (the keyboard table in `verification/B-12c_keyboard_table.md` is unchanged).
- **Not** checked by a test: where the keyboard goes after a click. Steps 1 to 5 are the check.
