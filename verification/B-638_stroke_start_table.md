# B-638: a Stroke added from now on starts at After Effects' values (D-759)

Written by `b638_a_new_stroke_starts_at_after_effects_values` in `app/src/tests/editing.rs`. A Stroke added in the window starts at Brush Hardness 79 with Stroke Sequentially on, as After Effects' panel shows a fresh Stroke (5 of 5 tutorial panels, wave 3 lane J). A project saved before keeps the 75 and off its file holds. No picture of an older project changes, and no fixture is added or changed.

| Check | Expected | Actual | Result |
|---|---|---|---|
| Add a Stroke from the Effects panel | Add Path Stroke | Add Path Stroke | pass |
| The new Stroke's Brush Hardness | "brush_hardness":79 | "brush_hardness":79 | pass |
| The new Stroke's Stroke Sequentially | "stroke_sequentially":"on" | "stroke_sequentially":"on" | pass |
| Typed in the panel: Brush Hardness 75, Stroke Sequentially off | kept | kept | pass |
| An older project's Stroke opens at what it saved | 75 and off | 75 and off | pass |
| Saved, opened and saved again, the older project's file | the same bytes | the same bytes | pass |
| The older project's saved Brush Hardness and Stroke Sequentially | 75, "off" | 75, "off" | pass |

7 of 7 checks pass.
