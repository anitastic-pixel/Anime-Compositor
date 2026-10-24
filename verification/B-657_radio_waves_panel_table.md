# B-657: Radio Waves' Parameters Are Set At in the window (D-777)

Written by `b657_a_new_radio_waves_is_set_at_birth` in `app/src/tests/editing.rs`.

| Check | Expected | Actual | Result |
|---|---|---|---|
| A new Radio Waves' Parameters Are Set At | "birth" | "birth" | pass |
| Typed in the panel: each_frame, 128 sides | each_frame, not written, 128 | each_frame, not written, 128 | pass |
| Typed in the panel: birth, 100 sides | "birth", 100 | "birth", 100 | pass |
| Typed in the panel: "death" is refused | refused | refused | pass |
| An older project's Radio Waves opens at | each_frame, not written | each_frame, not written | pass |
| Saved, the older project's file holds no parameters_at | false | false | pass |

6 of 6 checks pass.
