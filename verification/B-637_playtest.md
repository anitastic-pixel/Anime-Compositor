# B-637 playtest: 4-Color Gradient gains After Effects' Jitter (D-758)

## What to try

1. Add **4-Color Gradient** to a picture layer. It looks as it always did. **Jitter** sits under **Blend** and starts at 0.
2. Set **Jitter** to 50. Seen whole, the gradient looks the same. Zoom the viewer to 400% on a soft part of it: there is now a fine, even grain in the colours, a few levels deep. It breaks up the stepped bands a smooth gradient can show.
3. Set **Jitter** to 100, the top. The grain is about twice as strong; it is still fine, never blotchy.
4. Scrub or play. The grain stays put on the drawing; it does not crawl from frame to frame.
5. Key Jitter from 0 to 100 over a second and play: the grain fades in.
6. Open a project saved before this build that has a 4-Color Gradient. It looks exactly as before and shows Jitter 0. Saving it writes no jitter, so the file stays as it was.

Before this build there was no Jitter. A tutorial that set it could not be copied.

## What the check found

Fixture check `tests/b637_four_color_jitter.rs`: 133 of 133 (see `B-637_four_color_jitter_table.md`). FX-4CG-027 to 037 are pixel-compared with `tools/four_color_gradient_reference.py` (tolerance 2e-5; the largest difference found is 4.2e-8). No existing fixture changed.

Other checks:
- Every older 4-Color Gradient fixture (FX-4CG-001 to 026) saves as it was written and draws frames 0 and 4 bit for bit the same as its twin with Jitter 0 written. FX-4CG-010's capital colour letters come back lowercase, as they already did before this build.
- Jitter 101, Jitter -1, Jitter written as a word, Jitter 100.5 and Jitter keyed to 150 are each refused with a sentence naming jitter and change nothing.
- The half-size draft preview keeps Jitter 50 (it is not a distance). Jitter never grows the layer.
- The picture is the same however it is cut into tiles.
- On the graphics card: a 4-Color Gradient with Jitter 0 is drawn there as before and matches the processor within 1 level of 255. One with Jitter above 0 is drawn by the processor, with no warning, so those frames are exactly the processor's.
- `b143` (the older 4-Color Gradient check) passes; `b225` (the card's generators) 2386 of 2386; the app's tests 114 passed, 5 ignored.

## Before and after (no After Effects panel)

These sheets are before/after only. Wave 2 found no tutorial still where After Effects' Jitter can be measured: the grain is a few levels deep and 720p video compression wipes it out (the closest case, Jake in Motion's, differed by a standard deviation of 0.21 levels, below the video's own noise).

Both builds are the real app drawing on the processor, a 1920 by 1080 frame with the panel's default 4-Color Gradient. Measured against each build's own Jitter 0 frame. "Grain" is the standard deviation, in levels, of what is left after a 3-pixel blur is taken away.

| Setting | Before (wave 1 build) | After (B-637) | Verdict |
| --- | --- | --- | --- |
| Jitter 0 | grain 0.08 | grain 0.08; 0 pixels differ from the before build | **unchanged**, as it should be (a control) |
| Jitter 50 | 0 pixels change (no jitter there); grain 0.08 | 1,611,611 of 2,073,600 pixels change, by up to 7 levels; grain 0.37 | **works**: fine grain, a few levels |
| Jitter 100 | 0 pixels change; grain 0.08 | 1,807,087 pixels change, by up to 13 levels; grain 0.73 | **works**: about twice Jitter 50's grain |

The fixture check's street pictures agree: Jitter 50 changes 100,611 pixels by up to 6 levels, and Jitter 100 changes 112,931 pixels by up to 12 levels.

## Pictures (`B-637 pictures/`)

- `cg_jitter.png`: the sheet. Each row shows before, after, then the middle enlarged 3 times for before and after. The grain shows in the last panel of the Jitter 100 row.
- `1_jitter_0.png`, `2_jitter_50.png`, `3_jitter_100.png` and their `_zoom` corners (written by the check): the street under the gradient at each Jitter.

## How this differs from After Effects

- After Effects does not publish its Jitter rule. Ours nudges each pixel's four mixing weights by up to Jitter / 1000 of themselves with a seeded number fixed to the drawing (document 21). The strength is our own choice, not fitted to After Effects' pixels.
- Blend keeps this program's look. After Effects mixes by 1 / (distance squared plus softness squared); ours is kept so no saved project changes (recorded as a gap in D-758).
