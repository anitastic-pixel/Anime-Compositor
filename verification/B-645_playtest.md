# B-645 playtest: layer markers (D-765)

## What changed for you

A layer now has markers of its own, as in After Effects: a little orange pennant on the top edge of
the layer's bar, on its frame, with its name beside it. A marker can have a length, drawn as an
orange strip along the top of the bar to where it ends. Composition markers on the ruler can have a
length too, drawn the same way.

- **`*`** (Numpad * or Shift+8) with one or more layers selected puts a marker on each of them at the
  playhead. With no layer selected it puts a marker on the ruler, as before. One Ctrl+Z takes it away.
- On a layer's bar: **drag** a pennant to move it, **Alt+drag** it to pull out its length,
  **double-click** it to name it, **Ctrl+click** it to remove it, **click** it to put the playhead on it.
- **Shift** while dragging the playhead now snaps to every marker, the ruler's and the layers'.
- **J and K** now stop at markers as well as keys: the ruler's, and the selected layers' (every
  layer's when none is selected).
- Sliding a layer along the timeline takes its markers with it.

A project saved before this change looks exactly as it did. A project with layer markers in it
(from a file written elsewhere) now shows them. If a layer's markers in a file cannot be read, the
project still opens, a note says which layer, and saving keeps them exactly as they were.

Nothing a render draws changes, so the proof is the timeline itself.

## What to look at

- `verification/B-645 pictures/1_layer_markers_on_the_bars.png`: three layers. Hero has
  "anticipation" at frame 6 and "hit" at 14 with a 10-frame strip; Impact FX has "fade" at 30; the
  ruler has "start" at 0 and "beat" at 24 with an 8-frame strip.
- `verification/B-645 pictures/2_star_adds_a_layer_marker_at_40.png`: the same after selecting
  Impact FX, putting the playhead on frame 40 and pressing `*`: a new pennant at 40 on Impact FX.
- `verification/B-645_layer_markers_table.md`: the file check, 9 of 9 cases match the reference.

## Try it

1. Select a layer, put the playhead somewhere and press `*`. A pennant appears on that layer's bar
   at the playhead. Press Ctrl+Z: it goes.
2. Double-click the pennant, type "hit", press Enter. The name shows beside it.
3. Alt+drag the pennant to the right: an orange strip grows from it. Let go; Ctrl+Z puts it back.
4. Drag the layer's bar along the timeline: the pennant moves with it.
5. Deselect all layers, press J and K: the playhead stops at the pennants and the ruler's markers.
6. Hold Shift and drag the playhead near a pennant: it snaps onto it.
7. Save, close and open the project again: the markers are still there.

## Known differences from After Effects (not fixed here)

- Marker expressions (`marker.key(1).time` and the like) are not built yet; that is a separate
  proposal in D-765.
- Stretching a layer's time does not stretch its markers (After Effects does).
- Markers have no chapter, web link, cue point or colour, and a ruler marker's length cannot be
  dragged on the ruler yet (only on a layer's bar).
