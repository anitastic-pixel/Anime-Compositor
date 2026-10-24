# B-643: a Curl Noise added from now on starts at After Effects' values (D-764)

Written by `b643_a_new_curl_noise_starts_at_after_effects_values` in `app/src/tests/editing.rs`. A Curl Noise added in the window starts at Speed 0, Turbulence Speed 10, Swirl 180, Sample Count 3, Sample Radius 80 and Flow Softness 25, as After Effects' panel shows a fresh Curl Noise (Softoni 0:48, wave 1 lane A; the settings D-749 was fitted at). Since D-749 ours reads each of the six in After Effects' own units and under the same name, so each number is carried over as it is. A project saved before keeps the 10, 20, 45, 12, 30 and 20 its file holds. No picture of an older project changes, and no fixture is added or changed.

| Check | Expected | Actual | Result |
|---|---|---|---|
| Add a Curl Noise from the Effects panel | Add Curl Noise | Add Curl Noise | pass |
| The new Curl Noise's Speed, Turbulence Speed, Swirl, Sample Count, Sample Radius, Flow Softness | 0, 10, 180, 3, 80, 25 | 0, 10, 180, 3, 80, 25 | pass |
| Typed in the panel: 10, 20, 45, 12, 30, 20 | kept | kept | pass |
| An older project's Curl Noise opens at what it saved | 10, 20, 45, 12, 30, 20 | 10, 20, 45, 12, 30, 20 | pass |
| Saved, opened and saved again, the older project's file | the same bytes | the same bytes | pass |
| The older project's saved six settings | 10, 20, 45, 12, 30, 20 | 10, 20, 45, 12, 30, 20 | pass |

6 of 6 checks pass.
