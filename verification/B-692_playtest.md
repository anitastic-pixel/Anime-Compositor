# B-692 playtest: an export is faster when nothing is missing, and writes the same files

## What changed for you

When you export with the default choice for a missing drawing (stop and write nothing), the
export no longer works out every frame twice. On the reference shot 14 frames took about 0.9 s
instead of 1.3 s; the longer and heavier the shot, the more it saves (up to about half on the
shots wave 8 measured). Every file is the same as before, byte for byte.

One thing looks different: the progress counter stays still for longer at the start, while the
export checks for missing drawings (it now draws and encodes during the check), and then runs
quickly through the frames.

An export that does find a missing drawing stops as before, in the same time, with the same
message.

## What to look at

`verification/B-692_export_once_table.md`: before and after times, the machine, and the check
that 8678 exports wrote the same bytes.

## Try it

1. Open a project with no missing drawings and export it to a PNG sequence, leaving "if a drawing
   is missing" at its default.
2. Export the same range with the build before this one, or compare with an export you made
   earlier: the files are identical, and the export finishes sooner.
3. Rename one drawing's file and export again: it stops and names the frames, as before.
