# D-314: Time-Reverse Keyframes and Freeze Frame

Found by P-26. The lightsaber tutorials (4, 5) play an ignition backwards with After Effects' **Time-Reverse Keyframes**. The Colorful Glitch tutorial (3) holds one frame with **Freeze Frame**. Neither existed here.

## What changed

- **Time-reverse keyframes** is on the timeline's key menu (right-click a key) and in the command palette. It needs two or more keys chosen.
  - The chosen keys play backwards inside the span from the first to the last.
  - Each key keeps its value, and each ease is turned round to match.
  - Keys that are not chosen stay where they are.
  - It is one entry to undo.
- **Freeze frame** is on the layer's right-click menu and in the command palette.
  - On a drawn layer, the drawing on the playhead is held for the layer's whole length.
  - It is one entry to undo.
  - It does not work on a composition layer yet, because that needs Time Remapping (D-308, not approved). It says so in a sentence.

## Checks (cargo test, app)

`d314_chosen_keys_reverse_and_a_drawing_freezes`: passes. All app tests pass.

| Check | Expected | Got |
|---|---|---|
| Keys at 0, 4, 10 (the one at 4 eased), all three reversed | 400 at 0 with the ease turned round, 100 at 6, 0 at 10 | so |
| The same, as undo sees it | one entry | so |
| Reversed again | as it was before | so |
| Undo twice | as it was before | so |
| A key not chosen where a reversed one would land (frame 6) | nothing changes; a sentence names frame 6 | so |
| Only one key chosen | nothing changes; a sentence asks for two or more | so |
| Freeze on frame 3 of a layer showing drawing 1 on frames 0-1 and drawing 2 on frames 2-4 | drawing 2 held from frame 0 to frame 4 | so |
| Freeze, as undo sees it | one entry; undo restores both exposures | so |

## Pictures (the test copy, never the owner's app)

These use the fixture's red (drawing 1) and green (drawing 2) cels, enlarged, with position keys at frame 0 (left), frame 1 and frame 4 (right).

| Picture | Look for | Pass? |
|---|---|---|
| `D-314 pictures/1_before_frame_0.png` | frame 0: the red drawing at the left of its motion path | pass |
| `D-314 pictures/2_reversed_frame_0.png` | after Time-reverse keyframes, frame 0: the red drawing at the right end of the path; the timeline's keys are now on 0, 3 and 4 | pass |
| `D-314 pictures/3_frame_3_before_freeze.png` | frame 3: the green drawing (drawing 2) | pass |
| `D-314 pictures/4_frozen_frame_0.png` | after Freeze frame on frame 3, frame 0 shows the green drawing too; the layer bar is one held exposure | pass |

## For the owner to try

1. Key a layer's position at two or three frames. Choose all the keys, right-click one, then pick **Time-reverse keyframes**. Play it: the move runs backwards.
2. Choose a drawn layer, put the playhead on a drawing, then right-click the layer and pick **Freeze frame**. Every frame now shows that drawing. Ctrl+Z puts the exposures back.
