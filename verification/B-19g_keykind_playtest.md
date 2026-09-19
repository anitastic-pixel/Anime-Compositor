# B-19g: separate X and Y, and auto, continuous and roving keys, by hand

Built on 2026-09-18. These are the last two of the ten graph items: separate X and Y for
position (item 7) and auto bezier, continuous bezier and roving keys (item 10). The rule they
follow is D-69, accepted on 2026-09-18.

The generated halves are `verification/B-19f_keykind_table.md`, 36 of 37, which checks the
numbers against D-69's fixtures and explains the one fixture fault, and
`verification/B-19g_panel_table.md`, 24 of 24, which sends what the window sends and reads back
what the page is given. This sheet covers what a table cannot judge: what is drawn, and how it
feels.

## Before you start

Open any project, or use **Save As...** and work on a copy. Choose a layer with a picture on
it. Nothing below changes the fixtures.

## What to check

1. **The button.** Select the layer. Beside **Position** in the inspector there is a small
   two-headed arrow, dimmed. Hover it: it says Separate dimensions. Press it. Position becomes
   two rows, **X position** and **Y position**, each with its own diamond and one number, and
   the layer does not move.
2. **One step to undo.** Ctrl+Z: one Position row again. Ctrl+Shift+Z: two rows again.
3. **The timeline rows.** Select the layer and press P. The timeline opens X position and Y
   position under the layer, each with the same arrow button.
4. **Key one half only.** On frame 0 click X position's diamond. Go to a later frame and change
   X. Play: the layer slides sideways. Y position has no keys and stays a plain number.
5. **The viewer still moves it.** On the later frame, drag the layer in the viewer, and nudge it
   with the arrow keys. X's key on that frame takes the new X and Y takes the new Y. One Ctrl+Z
   takes the whole drag back.
6. **The graph.** Open the **Graph** tab. X position and Y position are in the list of
   properties. Pick X position: one curve, one value handle per key, and everything from B-19a
   works on it. This is the reason to separate: each half has its own speed curve.
7. **Joining.** Press the arrow button again. There is one Position row, with a key on every
   frame where X or Y had one.
8. **Auto bezier.** Put three Rotation keys on the layer with uneven values, for example 0, 10
   and 40 on frames 0, 10 and 20. Right-click the middle key on the timeline and choose **Auto
   bezier: smoothed for me**. The key becomes a round dot. In the graph the curve passes through
   it with no corner. Move the middle key's value in the graph: the handles follow by themselves
   and the curve stays smooth.
9. **Continuous bezier.** Right-click the middle key and choose **Continuous bezier**. The key
   goes back to its hourglass shape and its tooltip says continuous bezier. In the graph, pull
   one of its handles: the handle on the other side follows, so the speed in equals the speed
   out.
10. **An auto key's handle pulled by hand.** Make the key auto again, then pull one of its
    handles in the graph. It becomes continuous: the dot turns back into an hourglass.
11. **Bezier.** Right-click and choose **Bezier: each handle on its own**. Pull one handle: the
    other stays where it is, as before today.
12. **Rove across time.** Join the position if it is separated. Key Position on three frames so
    the layer travels a short way and then a long way, for example frames 0, 10 and 20.
    Right-click the middle key and choose **Rove across time, or stop**. The key becomes a small
    dot and moves to an earlier frame, so that the layer travels at one even speed through all
    three keys. Play it: no change of pace at the middle key.
13. **Roving follows its neighbours.** Drag the last key to a later frame. The small dot moves by
    itself to keep the speed even.
14. **Stopping.** Drag the small dot along the timeline: it stops roving and is an ordinary key
    where you dropped it. Or choose the menu line again to stop it where it is.
15. **What is refused.** Try Rove across time on the first or the last key, or on a Rotation
    key. Each is refused with a sentence in the status line and nothing changes.
16. **Save and reopen.** Save, close, open. The separated position, the round keys and the small
    dot are as they were, and the picture is the same.

## Known limits, on purpose

- While a position is separated there is no motion path drawn in the viewer, and Alt+Shift+P
  is refused with a message: key X position or Y position instead.
- Separating a position takes the motion path's curved handles off its keys and stops any key
  roving, as D-69 says. Ctrl+Z puts them back.
- The kinds and roving are for a layer's own properties. The camera's keys and an effect
  setting's keys keep plain bezier handles.
- Several keys moved at once next to roving keys can be refused half way where a single move
  would not be. Move them one at a time there.
- The one row that does not match in B-19f's table is a fault in the fixture FX-KIND-005, not
  in the build, and waits for the owner's decision. It is written up in document 15 under B-19f.

## Verdict

Works / does not work, with the number of any step that failed:

**Works.** The owner, 2026-09-19: "B-19g playtest works".
