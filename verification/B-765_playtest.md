# B-765 playtest: the boxes still follow the picture (D-885, PROPOSED, switched off)

This change is built but switched off, because it measured no faster. Nothing should feel
different. The check is only that nothing got worse.

1. Open a project and select a layer that has a mask.
2. Drag the layer around in the viewer, then scale it by a corner handle.
3. Drag one mask point.
4. Drag the playhead across the timeline with the layer still selected.
5. Switch between Draft and Full, then do steps 2 to 4 again.

What should happen: the layer's box, its handles and the mask points stay exactly on the
picture at every moment, as before. If a box or point ever trails behind or sits off the
picture, that is a fail.
