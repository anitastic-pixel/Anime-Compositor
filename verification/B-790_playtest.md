# B-790 playtest: "Faster on the graphics card (not byte-exact)" in the export window (D-911)

You said "do what you recommend for the decisions". So the card export measured in D-907 is now a tick in the export window. It is off unless you tick it, and the processor stays the default.

Picture of the window with the tick: `verification/B-790 pictures/export_tick.png`.

## 1. The tick is there and off (1 minute)

1. Open the app and open any project.
2. Open the Composition menu and look under Export..., below the format list.

**Pass:** a tick "Faster on the graphics card (not byte-exact)" is there and is not ticked. Hold the mouse over it: the hint says it helps heavy projects and may be slower on light ones, and that its files are not byte-exact.

## 2. An export without the tick is as before (2 minutes)

1. Leave the tick off. Export a few frames as a PNG sequence.
2. Read the report when it finishes.

**Pass:** the report says nothing about the graphics card, and the frames look as they always have.

## 3. An export with the tick (3 minutes)

1. Tick "Faster on the graphics card (not byte-exact)".
2. Export the same frames into another folder.

**Pass:**
- The status line says "... on the graphics card (not byte-exact), into ...".
- The report ends with "Exported on the graphics card (D-911): the card drew N of M frames", and names which frames the card drew.
- If a frame was drawn by the processor instead, the report says which frame and why ("Frame 12: ...").
- Put a frame from step 2 beside the same frame from step 3: you should not be able to see a difference (each pixel is at most one step in 255 away).

On a heavy project (many layers) this export should finish noticeably faster; on a light one it can be the same or a little slower (D-907 measured 2.46x faster on 25 layers, 0.94x on the 4-layer reference shot).

## 4. The render queue (1 minute)

1. With the tick on, click "Add to Render queue..." and pick a folder.

**Pass:** the row's format says "PNG sequence, on the graphics card".

## 5. Off again next time (1 minute)

1. Close the app and open it again.

**Pass:** the tick is off again.

**Fail anywhere:** the tick is ticked when the app opens, an unticked export's report mentions the graphics card, or the ticked export's report does not say what the card drew.
