# B-160: faster export, by hand

Built on 2026-09-29 under G7, which you approved that day ("sounds great! let's do 1 through 7 to your discretion."). Never performed.

The generated half is `verification/B-160_faster_export_table.md`. It shows that every file an export writes is the same file, byte for byte, as before this change, and it holds the numbers for the graphics card's MP4. This sheet covers what a table cannot: how the export feels, the new Preferences tick, and what the status line says.

There are two changes:

- **Several frames at once (D-231, decided within the discretion you gave).** An export now draws several frames side by side and writes them in order. Nothing about the files changes; only the time it takes.
- **Hardware video encoding (D-230, proposed, for you to decide).** A new tick in Preferences lets the graphics card encode an MP4. It is off unless you tick it. The card's MP4 is a slightly different file from the one this program has always written, which is why it is your decision.

## Before you start

Use the release build. It opens on the reference shot. Have an empty folder ready to export into, and a video player (the Windows Media Player or Films & TV app is fine).

## What to check

1. **An export is quicker.** Export the whole reference shot as a PNG sequence into the empty folder. It finishes, and the status line says "Exported 240 frames into ...". It should take noticeably less time than it used to. The progress count now climbs in jumps of about 15, as each batch of frames finishes together, rather than one by one.
2. **The frames are the same frames.** Open a few of the PNG files (for example `0000`, `0100` and `0239`). They look exactly as they did before this change.
3. **Stop halfway.** Export again into another empty folder and stop it partway through. The status line says how many frames finished, and the folder holds exactly that many complete frames, numbered from the start with no gap.
4. **The tick is off.** Open **Preferences…**. Under **Export** there is a box **Hardware video encoding**, not ticked, with a sentence saying what it does.
5. **An MP4 with the tick off.** Choose **MP4** in the export list and export the shot. The status line says it was exported as one MP4 and says nothing about encoders. Play it: it looks as MP4s always have.
6. **An MP4 with the tick on.** Tick **Hardware video encoding**, close Preferences and export the MP4 again into another folder. The status line now ends with a sentence saying the graphics card encoded the film, and names the card's encoder. Play it.
7. **Compare the two MP4s.** Play the two films side by side, or one after the other, and pause on a few frames with fine lines and flat colour. They should look the same to your eye. The table gives the measured difference; say whether you can see one.
8. **The tick is remembered.** Close the app and open it again. Preferences still shows the box ticked. Untick it before you finish, unless you decide to keep it.
9. **The tick only affects MP4.** With the tick on, export a PNG sequence or a GIF. Nothing about encoders is said, and the files are as in step 1.

## What to report

Anything that reads wrong, looks wrong or is in the way, and the number of the step. For D-230, the decision is yours: accept it (the tick stays, off unless ticked), reject it (the tick goes), or ask for something different. If step 6 said the software encoder wrote the film instead of the card, report that sentence as it appeared.
