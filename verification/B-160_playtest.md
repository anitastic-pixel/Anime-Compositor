# B-160: faster export, by hand

Built on 2026-09-29 under G7, which you approved that day ("sounds great! let's do 1 through 7 to your discretion."). Never performed.

The generated half is `verification/B-160_faster_export_table.md`. It shows that every file an export writes is the same file, byte for byte, as before this change. This sheet covers what a table cannot: how the export feels and what the status line says.

There is one change:

- **Several frames at once (D-231, decided within the discretion you gave).** An export now draws several frames side by side and writes them in order. Nothing about the files changes; only the time it takes.

Hardware video encoding (D-230) was also built here, as a tick in Preferences. You rejected it on 2026-09-29 ("trust your recommendation to reject it"): the graphics card was no faster on this machine and wrote bigger files. B-160b removed it, so there is no **Export** heading in Preferences and every MP4 is written as it always was.

## Before you start

Use the release build. It opens on the reference shot. Have an empty folder ready to export into.

## What to check

1. **An export is quicker.** Export the whole reference shot as a PNG sequence into the empty folder. It finishes, and the status line says "Exported 240 frames into ...". It should take noticeably less time than it used to. The progress count now climbs in jumps of about 15, as each batch of frames finishes together, rather than one by one.
2. **The frames are the same frames.** Open a few of the PNG files (for example `0000`, `0100` and `0239`). They look exactly as they did before this change.
3. **Stop halfway.** Export again into another empty folder and stop it partway through. The status line says how many frames finished, and the folder holds exactly that many complete frames, numbered from the start with no gap.

## What to report

Anything that reads wrong, looks wrong or is in the way, and the number of the step.
