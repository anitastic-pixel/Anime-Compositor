# B-24l: the free transform box's limits, by hand

Built on 2026-09-23 against D-81a, which the owner asked for the same day ("works; work on
limits") after B-24k passed.

Three of B-24k's six limits are lifted: the box can hold several masks, it has keyboard keys, and
Undo brings the box back with the shape. Three are kept on purpose, because After Effects' own
mask box does not do them either: skew and perspective, a box round a whole layer (the layer's
own handles do that), and remembering the box in the saved project. The fourth kept limit, a box
with no width, is arithmetic: points in one straight line have nothing to scale across.

Building this found one fault of B-24g's that was not on any list. Dragging **several layers at
once** could leave all but one of them short of where the hand let go, because the drag queue
kept only the newest request of the whole drag, not the newest of each layer. Step 8 checks it.

The generated half is `verification/B-24h_panel_table.md`, now 26 of 26. Its three new rows check
that one drag changing two masks' paths is one thing to undo, lands both, and one Undo puts both
back. As with B-24k, the box's own arithmetic runs only in the window, so this sheet is its check.

## Before you start

Open any project with at least one layer, or use **Save As...** first and work on the copy.
Choose a layer and draw two rectangle masks on it with **Q**, a little apart, then press **V**.
Nothing below changes the fixtures.

## What to check

1. **Two masks in one box.** Double-click the outline of the first mask: the box goes round it.
   Now hold Shift and double-click the outline of the second mask. The box grows to hold both
   masks, and the status line says "Free transform round 2 masks". The cross-hair moves to the
   middle of the pair.
2. **Working on both.** Drag inside the box, drag a corner square, and turn from just outside it,
   as in B-24k. Both masks move, scale and turn together, as one picture. Each drag is one
   Ctrl+Z, and that one Ctrl+Z puts both masks back.
3. **Another mask on its own.** Put the box away with Enter. Double-click the second mask's
   outline, without Shift. That mask becomes the one worked on (its points show) and the box
   goes round it alone.
4. **Undo brings the box back too.** With the box round one mask, turn it about 30 degrees and
   let go. Press Ctrl+Z: the mask goes back **and the box stands upright again**, tight round it.
   Press Ctrl+Shift+Z: the mask turns again and the box is turned with it.
5. **Keys: moving.** With the box up, press the arrow keys. What the box holds moves a pixel a
   press, ten with Shift, and the cross-hair travels with it. The layer does not move.
6. **Keys: turning.** With the box up, press **+** on the numeric keypad: what the box holds
   turns one degree clockwise about the cross-hair, and the box turns with it. **-** on the
   keypad turns it back. With Shift they turn ten degrees.
7. **Keys: scaling.** Hold Alt and press **+** on the keypad: what the box holds grows by one
   percent about the cross-hair; Alt and **-** shrinks it; with Shift as well, ten percent.
   Each press of steps 5 to 7 is one Ctrl+Z, even with two masks in the box.
8. **Several layers dragged together.** Put the box away. Ctrl-click two or three layers in the
   layer list (or press Ctrl+A for all of them) and drag one of them a long way across the
   picture quickly, then let go. Every one
   of them lands the same distance from where it started; none stops short.

## Known limits

- The keypad keys need a numeric keypad (on many laptops, the keys under Num Lock or Fn). They
  are After Effects' own keys for turning and scaling a layer; After Effects' box has no keys.
- Shift and a double-click adds a mask; nothing takes one back out of the box except closing it
  and starting again.
- Only the mask being worked on shows its points; the others in the box show their outline.
- Skew and perspective, a box round a whole layer, and a box kept in the saved file are left out
  on purpose: After Effects' mask box has none of them.
- A box with no width (points in one line) still scales nothing across that line.

## What to answer

"works", or which step number did something else and what it did.
