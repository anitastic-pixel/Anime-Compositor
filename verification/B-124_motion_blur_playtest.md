# B-124: motion blur, by hand

Built on 2026-09-28 against D-188 and ADR-019, which you accepted the same day ("I approve of
motion blur plan"). A layer that moves can now be blurred along its path, the way After Effects
does it: each drawing layer has its own motion-blur switch, the composition has one switch over
the timeline that turns them all on or off, and Composition Settings holds the shutter.

`verification/B-124b_motion_blur_table.md` is the check: every fixture of FX-MB-001 to 050, the
times the shutter samples, the pictures bit for bit, the corners of the layers within a
ten-thousand-millionth of a pixel, and every file and setting that must be refused.
**75 of 75 checks pass.** Its last six rows are the pictures in `verification/B-124 pictures/`,
frame 5 of a small shot: an orange card crossing the frame and turning, and a white bar spinning
in place.

- `sharp.png`: the composition's switch off. Both are sharp.
- `as_added.png`: switched on as it comes, a shutter of 180 degrees with 16 samples. The card
  smears along its path and the bar fans out.
- `shutter_360.png`: a shutter of 360 degrees, a whole frame of travel. The smears are twice as
  long.
- `four_samples.png`: only 4 samples. The four copies can be counted; this is what too few
  samples looks like.
- `bar_switch_off.png`: the bar's own switch off. The card blurs; the bar is sharp.
- `draft.png`: `as_added.png` at Draft, a quarter of the size each way.

## Before you start

- Use the release build. It opens on the reference shot.
- Leave **Draw on** at **Auto**.
- On the reference shot nothing moves, so there is nothing to blur yet. Step 2 makes a layer
  move.

## What to check

1. **The switches are there.** Each drawing layer in the timeline has a new switch after Shy: a
   dot with three speed lines behind it. A null, an adjustment layer and a sound layer have a
   blank there, because they have no picture of their own to blur. Over the timeline, beside
   **Hide shy**, is a **Motion blur** button for the whole composition.
2. **Nothing blurs until both are on.** Go to frame 90 and key layer 4's **Position** where it
   is. Go to frame 110 and add 400 to its first number, so it slides 20 pixels a frame. Go to
   frame 100. Turn on layer 4's switch: nothing changes. Turn on **Motion blur** over the
   timeline: layer 4 smears along its path, sharpest in the middle, fading at both ends. Turn the
   button off: sharp again.
3. **Only switched layers blur.** Key layer 3 the same way and leave its switch off. With the
   button on, layer 3 slides sharp while layer 4 blurs.
4. **A layer that holds still is untouched.** Go to frame 50, before the keys. Turning the button
   on and off changes nothing you can see, and the frame costs no more time: a layer whose place
   is the same all through the shutter is drawn once, exactly as without blur.
5. **The shutter.** Press **Ctrl+K**. Under **Motion blur**, set the shutter angle to 360: the
   smear doubles in length. Set samples to 4: four copies show, as in `four_samples.png`. Set
   them back to 16. Set the phase to 0: the smear now lies ahead of the layer, towards where it
   is going, instead of round it. Each change is one step of undo.
6. **Undo.** **Ctrl+Z** takes back each switch and each shutter change, one at a time.
7. **Draft.** Switch to **Draft**. The same smear, smaller and much faster (the table below).
8. **The card.** With **Draw on** at **Auto** or **GPU**, a frame with motion blur is drawn by the
   processor, because the card does not draw motion blur yet. Open **Session log…**, tick the
   box and play a few frames round frame 100: those frames carry `GPU_PREVIEW_ON_CPU` with "The
   CPU drew this frame: it has motion blur, which the GPU does not draw yet." Turn the button off
   and the card draws again.
9. **Save and reopen.** Save, close, open again: the switches and the shutter are as you left
   them. A project saved before today opens with every switch off.
10. **Export.** Export frames 95 to 105. The smear is in the file, the same as the viewer at Full.

## How long a frame takes

Timed on 2026-09-28 by a throwaway test, not kept: the reference shot at 1920 by 1080, frame 100,
with layers 3 and 4 sliding 20 pixels a frame as in steps 2 and 3, both switches on, drawn by the
processor through the viewer's path with the drawings already read. The median of five after one
to warm up. AMD Ryzen 9 9900X with 24 threads; Windows 11; release build.

| The composition's shutter | Full | Draft |
| --- | --- | --- |
| off | 11 ms | 2 ms |
| on, but nothing moves (the still shot) | 11 ms | |
| 180 degrees, 4 samples | 97 ms | |
| 180 degrees, 16 samples, as added | 296 ms | 27 ms |
| 360 degrees, 32 samples | 624 ms | |
| 180 degrees, 64 samples | 1133 ms | |

Each sample draws each blurred layer once more, so the cost grows with the samples and with how
many layers move: about 9 ms for one more 1920 by 1080 layer at Full. For playing back at speed,
use Draft until the card version is built.

## Known limits, on purpose

- The card does not draw motion blur yet. A frame with any blurred layer or matte is drawn by the
  processor and says so. A card version is a unit of its own, after this one.
- The blur follows a layer's movement, turning and scaling, and a camera's, not what moves
  inside a drawing from one cel to the next. That is D-188's rule, as it is After Effects'.

## Found along the way, not motion blur

**A layer with a matte disappears at Draft.** A throwaway check, not kept: a white 960 by 540
layer matted by a 100 by 100 square. At Full the square of white shows, 10,000 pixels of it; at
Draft, nothing does. The cause is in the Draft step, which shrinks each layer's placement to the
smaller frame but not its matte's, so the matte lands in the wrong place. It is older than motion
blur, it is the same on the card, and exports are untouched, because they are never Draft. So
that it does not look like a motion blur fault, leave mattes out of step 7.

The fix is one line and a table check, as a small unit of its own. Say if you want it, and when.

## What to report

"works", or which step number did something else and what it did, with the settings and the frame
number; and whether to fix the Draft matte.
