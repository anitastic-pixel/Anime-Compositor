# W-41b check first: an empty place gives the page its pixels back (D-250, proposed)

When the card paints the picture straight onto the screen (B-45), the page gets no pixels: each frame's answer is empty and says `x-on-screen: 1`. A turned or mirrored view, a snapshot and a compare all need the picture's pixels on the page. D-250 asks for one thing: when the page sends a place with nothing in it, the card stops painting until the next place, so the next frame's answer carries its pixels.

Checked in the running app on the reference shot at Draft (480×270, drawn on GPU), with `w41b_check.js`. "Bytes" is the size of the picture in the answer; 480 × 270 × 4 = 518,400.

| Row | What was done | Before the change | After the change |
|---|---|---|---|
| R1 | the card is painting; a frame is asked for | on screen 1, 0 bytes, 480×270, GPU | on screen 1, 0 bytes, 480×270, GPU |
| R2 | the page sends an empty place | answered, nothing painted | answered, nothing painted |
| R3 | the next frame is asked for | on screen 1, 0 bytes — **fail** | on screen 0, 518,400 bytes — **pass** |
| R4 | the same frame is asked for again | on screen 1, 0 bytes — **fail** | on screen 0, 518,400 bytes — **pass** |
| R5 | the page places the picture again | on screen 1, 0 bytes | on screen 1, 0 bytes — the card paints again |

Before the change, rows R3 and R4 failed: the card kept painting and the page never got the picture. After it, all five rows are as asked.

The change is four lines in `place()` in `app/src/main.rs`. It changes no picture, no export and no project file, and taking it out puts back the old behaviour.
