# H-01 — the whole picture, not a sample of it

**7 of 7 checks passed.**

Produced by `tests/h01_whole_picture.rs`.

## Why this exists

Every other table in this build checks **named pixels**: this corner of that bar, the alpha at that coordinate, this colour to six places. That is precise and it is narrow. A fault that moved every layer one pixel across, dimmed the entire frame by a hair, or swapped red for blue would pass all of them, and pass the hardening report's 137 deliberate defects too, because nothing was looking at the picture as a whole. Your own eye on the reference shot is the only thing that was, and an eye does not see one part in a thousand.

So the reference shot is composited **twice**. Once by the renderer this project is building — tiled, multithreaded, the real one. Once by a second compositor written inside this test, from document 21, in the most obvious way possible: one loop over 2,073,600 pixels, bottom layer first, no tiles, no threads, and no code shared with the renderer at all. The cel files are decoded separately, the sRGB curve is written out longhand, and the composite is the formula as document 21 line 57 states it.

Then every channel of every pixel of four frames is compared. Not a tolerance, not a sample: **any difference at all is a failure.**

## What to look at

- **`H-01_renderer_frame.png`** and **`H-01_independent_frame.png`** — frame 100 of your shot, produced by the two compositors. They should be indistinguishable, and the table says they are identical to the last bit. Flip between them in an image viewer if you want to see it for yourself.

## What this still does not prove

Both compositors were written from the same document by the same agent, so a misreading of document 21 that infects both would go unnoticed here. What it rules out is the far likelier fault: an implementation slip in the fast, tiled, threaded renderer that the naive one does not share. The frames chosen are frame 0, frame 14 (where layer 3's drawing is deliberately missing), frame 100 and frame 239, not every frame of the shot.

The last two rows exist to prove the comparison can fail at all: two different frames are compared and must disagree.

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| frame 0: every one of the 2073600 pixels is what a second compositor, written from document 21, produces | `0 pixels differ` | `0 pixels differ` | pass |
| frame 14: every one of the 2073600 pixels is what a second compositor, written from document 21, produces | `0 pixels differ` | `0 pixels differ` | pass |
| frame 100: every one of the 2073600 pixels is what a second compositor, written from document 21, produces | `0 pixels differ` | `0 pixels differ` | pass |
| frame 239: every one of the 2073600 pixels is what a second compositor, written from document 21, produces | `0 pixels differ` | `0 pixels differ` | pass |
| the comparison is capable of failing: frame 0 against frame 100's picture | `they differ` | `they differ` | pass |
| layer 3 still has no drawing 7 in the fixture, so frame 14 has one fewer layer in it | `there is no file for layer 3 drawing 7` | `there is no file for layer 3 drawing 7` | pass |
| frame 14 and frame 12 are different pictures in the independent compositor too | `they differ` | `they differ` | pass |
