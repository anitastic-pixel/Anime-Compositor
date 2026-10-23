# B-640: a Radio Waves added from now on starts at After Effects' times (D-761)

Written by `b640_a_new_radio_waves_starts_at_after_effects_times` in `app/src/tests/editing.rs`. A Radio Waves added in the window lives 10 seconds and fades over its last 5, as After Effects' panel shows a fresh Radio Waves (Lifespan 10 s in 5 tutorials, Fade-out Time 5 s in 6, wave 3 lane n), worked in the composition's frames to the nearest frame and held inside 1 to 1000. A project saved before keeps the 96 and 48 its file holds. No picture of an older project changes, and no fixture is added or changed.

| Check | Expected | Actual | Result |
|---|---|---|---|
| 24 fps: add a Radio Waves | Add Radio Waves | Add Radio Waves | pass |
| 24 fps: the new Radio Waves' Lifespan and Fade-out Time, frames | 240, 120 | 240, 120 | pass |
| 29.97 fps: add a Radio Waves | Add Radio Waves | Add Radio Waves | pass |
| 29.97 fps: the new Radio Waves' Lifespan and Fade-out Time, frames | 300, 150 | 300, 150 | pass |
| 25 fps: add a Radio Waves | Add Radio Waves | Add Radio Waves | pass |
| 25 fps: the new Radio Waves' Lifespan and Fade-out Time, frames | 250, 125 | 250, 125 | pass |
| 120 fps: add a Radio Waves | Add Radio Waves | Add Radio Waves | pass |
| 120 fps: the new Radio Waves' Lifespan and Fade-out Time, frames | 1000, 600 | 1000, 600 | pass |
| Typed in the panel: Lifespan 96, Fade-out Time 48 | kept | kept | pass |
| An older project's Radio Waves opens at what it saved | 96, 48 | 96, 48 | pass |
| Saved, opened and saved again, the older project's file | the same bytes | the same bytes | pass |
| The older project's saved Lifespan and Fade-out Time | 96, 48 | 96, 48 | pass |

12 of 12 checks pass.
