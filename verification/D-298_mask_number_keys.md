# D-298 / B-183: A mask's feather, opacity and expansion take keys

Found by P-26, tutorials 1 (Video Copilot, Shockwave) and 4 (Chris Connor, lightsaber): both animate a mask's feather or opacity, so the edge softens or the glow fades over the shot. Here each was one number for the whole shot, set from a box with no stopwatch.

## What changed

- On the timeline, open a layer's Masks, then a mask: Mask feather, Mask opacity and Mask expansion each have a diamond. Press it to key the number on this frame; change the number on another frame to add a second key. The keys sit on the track, and ease, hold and the graph work on them as on any number.
- Every key is held to the ranges a mask always had: opacity 0 to 100%, feather 0 or more, expansion -8192 to 8192 px. A key outside them is refused with the reason.
- A number with no keys is saved exactly as before, so older files open and save unchanged.
- Found on the way: after stepping to another frame, the timeline's keyed numbers (on any row, not only masks) showed the frame before. They now show the frame on screen.

## Checks (cargo test)

`tests/b183_mask_number_keys.rs`, 4 of 4 pass. A white 8x8 solid masked to the square from (1,1) to (7,7). Worked by hand:

| Check | Expected | Got |
|---|---|---|
| Opacity keyed 1 at frame 0 and 0 at frame 10: the middle pixel at frames 0, 5, 10 | 1, 0.5, 0 | 1, 0.5, 0 |
| Feather keyed 0 to 8 and expansion -2 to 2: frame 5 against a plain feather 4, expansion 0 | same picture, pixel for pixel | same |
| Frame 0 against a plain feather 0, expansion -2 | same picture | same |
| The file saves the keys, and a number with no keys stays a plain number | kept | kept |
| A file with an opacity key of 1.5, a feather key of -1, an expansion key of 9000, or an expression | refused | refused, all four |
| The command with an opacity key of -0.1 | refused: "Mask "m" was given an opacity of -0.1, which is not from 0 to 1." | the same |

The app suite's mask walk (`verification/B-24c_panel_table.md`) now has three more rows, all passing: Mask 1's feather keyed 8 at frame 0 and 28 at frame 10 reads 18 at frame 5; the plain feather box is refused while it has keys and says where to set it; a feather key of -1 is refused with the reason. 89 app checks pass, 20 of 20 in that walk.

## Pictures (the test copy, never the owner's app)

A white solid with a rectangle mask; feather keyed 0 px at frame 0 to 60 px at frame 23, opacity keyed 100% to 40%.

| Frame | Picture | Timeline reads | Pass? |
|---|---|---|---|
| 0 | `D-298 pictures/1_frame0_sharp.png`: a sharp white bar | 0 px, 100% | pass |
| 12 | `D-298 pictures/2_frame12_halfway.png`: soft edges, a little dimmer | 31.3 px (60 x 12/23), 68.7% | pass |
| 23 | `D-298 pictures/3_frame23_soft.png`: a soft, dim glow | 60 px, 40%, diamonds filled | pass |

## For the owner to try

1. Make a white solid and draw a rectangle mask on it.
2. On the timeline, open the layer, then Masks, then Mask 1. Press the diamond beside Mask feather.
3. Go 20 frames on and drag Mask feather up to 60 px. A second diamond appears.
4. Play: the edge goes from sharp to soft. Do the same with Mask opacity to fade it.

Fixtures are unchanged.
