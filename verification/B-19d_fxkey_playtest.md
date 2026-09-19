# B-19d: an effect's settings keyed in the window, by hand

Built on 2026-09-18. This is the owner's "adding animation/keyframing stopwatches to effects as
well", with two of the ten graph items: effect settings in the graph (item 9) and several
properties in the graph at once (item 6). The rule it follows is D-68, accepted on 2026-09-18.

The generated halves are `verification/B-19c_fxkey_table.md`, 47 of 47, which checks the pixels
of a keyed effect, and `verification/B-19d_panel_table.md`, 21 of 21, which sends what the
window sends and reads back what the page is given. This sheet covers what a table cannot
judge: what is drawn, and how it feels.

## Before you start

Open any project, or use **Save As...** and work on a copy. Choose a layer with a picture on
it and add a **Gaussian Blur** and a **Tint** from the effects panel. Nothing below changes the
fixtures.

## What to check

1. **The stopwatch.** Each setting in an effect's card now has the small key diamond beside its
   name, the same one Position has. With the playhead on frame 0, click the diamond beside the
   blur's one number. It fills in, and nothing in the viewer changes.
2. **A second key.** Go to a later frame and change the blur's number, by typing or by dragging
   it. The diamond is filled on this frame too. Play or scrub between the two frames: the blur
   grows smoothly.
3. **The row on the timeline.** Twirl the layer open. Under its own properties there is a row
   named after the effect and the setting, with a diamond on each keyed frame. Press U with the
   layer selected: the row is among the animated ones.
4. **Keys behave like keys.** Drag one of those diamonds along the timeline. Select it and
   press F9 to ease it. Select it and press Delete. Ctrl+Z takes each back in one step.
5. **Copy and paste.** Select the setting's keys, Ctrl+C, move the playhead, Ctrl+V. The keys
   arrive at the playhead.
6. **A colour.** Key the tint's colour on two frames with two different colours. Between them
   the colour blends.
7. **The last key removed.** Remove every key of a setting. Its row leaves the timeline and the
   setting is a plain number again, holding still.
8. **The range is kept.** Try to type a blur below 0 on a keyed setting. It is refused with a
   message, and no key is made.
9. **The setting in the graph.** Open the **Graph** tab. The list of properties now has the
   keyed effect settings in it. Pick the blur. Its curve is drawn with its keys, and everything
   from B-19a works on it: drag a value, the box, typing, Fit, snapping.
10. **All.** Put two Opacity keys on the same layer. In the graph, press **All**. The other
    keyed properties are drawn as dashed grey lines behind the one being edited. Click a grey
    dot: that property becomes the one being edited. Press **All** again and they go.
11. **Save and reopen.** Save, close, open. The keys, their eases and the picture are as they
    were.

## Known limits, on purpose

- A setting has a row on the timeline only once it has a key. The first key is made from the
  effect's card.
- Pasting an effect's keys onto a layer that does not have that same effect is refused with a
  message.
- With **All** on, the grey lines share the scale of the property being edited, so a line whose
  values are far away can sit off the top or bottom until Fit is pressed.
- Separate X and Y, and auto, continuous and roving keys, are D-69 and come next as B-19f and
  B-19g.

## Verdict

Works / does not work, with the number of any step that failed:

**Works.** The owner, 2026-09-19: "play test of 19a and 19d work".
