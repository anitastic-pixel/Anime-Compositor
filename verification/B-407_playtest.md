# B-407: Timecode

Built on 2026-10-11 as D-527, under your /loop request: Timecode, in the **Text** group. It burns
a time onto the picture on a box, the way an editor's review copy shows it.

- **Display Format** (SMPTE): SMPTE (hours:minutes:seconds:frames), Frames (00037), Feet + Frames
  35mm (0002+05) or 16mm.
- **Time Source** (Layer Source): the layer's own time; Composition, the composition's frame;
  Custom, a count set by the next three.
- **Time Units** (30), **Drop Frame** (Off), **Starting Frame** (0): read with Custom only. Drop
  frame needs 30, 60 and so on.
- **Text Position** (50, 85: low in the middle, in per cent), **Text Size** (36), **Text Color**
  (white).
- **Show Box** (On), **Box Color** (black), **Box Opacity** (100).
- **Composite On Original** (On): Off shows the box and numbers alone on transparent.

A 29.97 composition counts drop frame by itself, with a semicolon before the frames. A layer made
from a file counts from the file's first frame, 0, and says so (as information) every frame:
a starting timecode written in the file is not read.

The check, `verification/D-527_timecode_table.md` (220 of 220), holds every frame to a
picture of the same box and numbers made from a solid and text layers by a separate program
before the code existed, and holds the card to the processor within 1 level. The pictures are in
`verification/D-527 pictures/`.

## What to check

1. **As added.** `as_added.png`: 00:00:01:13 in white on a black box, low in the middle.
2. **Drop frame.** `drop_frame.png`: 00:01:00;02 (frames ;00 and ;01 are skipped at the new
   minute).
3. **An hour in.** `custom_hour.png`: 01:00:00;00.
4. **Before zero.** `negative.png`: -00:00:01:00.
5. **Frame number.** `frames.png`: 00037.
6. **Film feet.** `feet_35mm.png`: 0002+05.
7. **No box.** `no_box.png`: white numbers straight on the picture.
8. **Colours.** `blue_box.png`: yellow numbers on a half-see-through blue box.
9. **Alone.** `composite_off.png`: only the numbers show (the black box on black).
10. **Corner.** `corner.png`: smaller, top left, the box cut off at the edges.
11. **Big.** `size_80.png`: big numbers on a big box.
12. **In the app.** Add Timecode (Text) to a layer and play: the numbers count with the frames.
    Set Time Source to Composition in a 29.97 composition: drop-frame labels with a semicolon.
13. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Your decision: a file's own starting timecode is not read (it would need a reader for the files'
timecode tracks). Is counting from 0, with the note, all right for now?

## Speed

On a 1920 by 1080 still at Full, a frame Timecode has not drawn before took about 230 to 290 milliseconds, against about 12 without it; a frame drawn before took about 4 to 6 (PROVISIONAL: the machine was busy). Playing new frames is slow for now (`verification/B-407_timecode_timing_table.md`)
