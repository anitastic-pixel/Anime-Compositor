# P-04: is the editor answering while the renderer works?

Written by `a_command_is_answered_while_a_frame_is_being_made` in `app/src/main.rs`, on every build. One thread asks the window for frames of the reference shot as fast as it can answer; another presses a key over and over and times how long each press waits. A press that begins *and* ends between the start and the end of one frame cannot have waited for that frame.

Nothing on this page is a duration. How many presses fit inside a frame, and how long each one took, depend on the build and the machine the suite happened to run on, and a page rewritten with different numbers on every run is a page nobody can tell a change from. The durations are the measurement, and they are dated: `verification/P-04_responsiveness.md`.

| Question | Expected | Found | Verdict |
|---|---|---|---|
| were frames being made while the keys were pressed | 4 frames, and at least one press | yes | pass |
| presses answered inside a frame that was still being rendered | more than none | more than none | pass |
| the slowest press against the time one frame takes | shorter | shorter | pass |
