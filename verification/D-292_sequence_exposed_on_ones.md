# D-292 / B-177: a picture sequence lays in on ones

Found by P-26: in tutorials 2 and 3 an imported picture sequence made a layer that drew nothing until its sheet was filled by hand. In After Effects, footage lays in frame by frame from the layer's start.

## What changed

- `app/src/main.rs`, the `layer.create` route: a layer made from a picture sequence gets one exposure per picture, in number order, starting at the layer's first frame. Past the last picture the layer is transparent.
- The layer still spans the composition. Still pictures, solids, text, shapes and every other kind of layer are unchanged. A sheet that someone edits afterwards is left as they edit it.

## Checks (cargo test)

App check, in the layer-adding test: a sequence asset with pictures 1 and 2 makes a layer whose sheet reads `[(0, 1, 1), (1, 2, 2)]` (frame 0 shows picture 1, frame 1 shows picture 2). Result: pass (app suite run on 2026-10-04: 89 passed, 5 ignored as always, none failed).

## For the owner to try

1. Import a numbered picture sequence (for example three PNGs named 0001, 0002, 0003) and drag it onto the timeline.
2. Step frame by frame with the arrow keys from the layer's start. The pictures follow one another, one frame each.
3. Step past the last picture: the layer shows nothing there.

Pixels, exports, saved files and fixtures for existing projects are unchanged.
