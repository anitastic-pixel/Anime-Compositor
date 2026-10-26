# B-744 playtest: an export reads a frame's drawings side by side, and writes the same files

## What changed for you

An export used to open and unpack each layer's drawing one after another before drawing the
frame. It now opens all of a frame's drawings at once, across the processor's threads, as the
viewer already did. Every file is the same as before, byte for byte. Nothing in the window
changed, and nothing in a saved project changed.

Be ready for little or no speed-up on a whole export. Export already drew up to 16 frames at once,
so the processor was already busy. One frame drawn alone is often faster (about 1.4 to 1.6 times
on small shots). A whole export changed by between 0.95 and 1.18 times, which is inside the
measuring noise. The proposal hoped for "2.8 s to about 1 s" per frame, and this did not reach it.
The timing page explains why.

Missing or unreadable drawings are reported exactly as before: the same messages, in the same
order, and "if a drawing is missing: stop" still stops and writes nothing.

## What to look at

- `verification/B-744_export_read_ahead_table.md`: six exports (the reference shot with its
  missing drawing, a ten-layer shot with drawings deleted and spoiled in the middle of the stack,
  and a cancelled export), each file's fingerprint and each message in order. The build before
  this one writes the very same page.
- `verification/B-744_export_timing.md`: before and after times, the machine, and the fingerprint
  of every exported file for eleven shots.

## Try it

1. Open a project with many drawn layers (twenty or more) and export it to a PNG sequence.
   It should finish in about the same time as with the build before this one, or a little sooner.
   It should never take clearly longer.
2. Compare the files with an export of the same range made earlier: they are identical.
3. Rename one drawing's file and export again with "if a drawing is missing: stop": it stops and
   names the frames, as before. With "render transparent" it writes the frames and lists the
   missing drawing in the same words as before.
4. Press Cancel during an export: it stops as before.
