# B-161 playtest: decoded drawings kept on disk

What changed: the first time the preview unpacks a drawing (a PNG from a sequence), it now also keeps the unpacked pixels in a folder on disk. The next time that drawing is needed, even after the program has been closed and opened again, the preview reads the kept copy instead of unpacking the PNG. The pixels are the same to the bit (`verification/B-161_decode_cache_table.md`, 23 of 23 checks). Export does not use the copies at all.

What to expect: on the reference shot, getting a frame's drawings ready took about 24 ms from the copies against about 35 ms unpacking the PNGs, roughly 30% faster (`verification/B-161_decode_cache_timing.md`, measured on 2026-09-30 with no other build running). You will mostly notice it on the **first** play of a shot after opening the program. Once a shot has played, its drawings are held in memory as before, so later plays in the same sitting were already fast and look the same.

## Steps

1. Open the program and open the reference shot project.
2. Open **Preferences**. Under **Decoded drawings on disk** you should see:
   - **Most on disk, in GB** showing **5.0**.
   - **Folder** empty, with the program's own folder shown faintly inside it. It ends in `decoded drawings`.
   - A line under them saying how much is held now, for example "Holding 0.0 of 5.0 GB now, in ...".
3. Close Preferences. Play the shot from the start to the end once.
4. Open Preferences again. The "Holding" number should have gone up. The reference shot takes about 0.4 GB.
5. Optional: open that folder in File Explorer. It holds files ending in `.cel`, one for each drawing.
6. Close the program completely, open it again and open the same project. Play from the start.
   - **Pass** if the first play is at least as smooth as the first play in step 3, and the picture looks exactly the same.
   - **Fail** if any frame looks different, or anything goes wrong.
7. Turn it off: in Preferences set **Most on disk** to **0**. The line should say "Off: every drawing is unpacked from its file each time." Play the shot. It should look exactly the same as before.
8. Set it back to empty or 5 to turn it on again.

## Safe to know

- The folder is only a copy of what the drawings already are. It is safe to delete, whole or in part, at any time; the preview then unpacks the PNGs again.
- The program only ever touches files ending in `.cel` or `.part` in that folder. Choosing a folder by mistake cannot delete anything else. It always makes its own `decoded drawings` folder inside whatever folder you choose.
- If a copy is ever damaged, the preview notices, deletes it, unpacks the drawing again and writes a new copy. With the session log switched on, the log gets one line starting `DECODE_CACHE_DISCARDED`. Nothing on screen changes.
- When the folder reaches its limit, the copies used longest ago are deleted first.

## What the owner decides

- Whether 5 GB is a good default limit. About six hundred 1920x1080 drawings fit in it.
- Whether this should be on by default. It is on now, because it never changes a pixel and only uses disk space.
