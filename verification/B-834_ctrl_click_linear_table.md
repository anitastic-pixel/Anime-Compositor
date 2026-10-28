# B-834: Ctrl+click back to linear straightens both sides, as one entry (D-954)

Written by `b834_ctrl_click_back_to_linear` in `app/src/tests/w12r.rs`.

Found by B-825's check in a private copy of the app: the second Ctrl+click left the side coming into the key eased, and each click was two or three entries to undo. The rows send the page's requests to the window's own command handler on the reference shot's keyed Rotation; `intoKey` is the page's own code run under Node. The private-app check is `verification/B-825_private_app_table.md`.

**7 of 7 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| Ctrl+click on the plain middle key: auto, easing both sides, one entry | true, true, 1 | true, true, 1 | pass |
| the side coming in is sent as the key before's curve, straightened | /keyframe.set_interp?layer=layer-1&prop=rotation&frame=0&drag=1&mode=linear | /keyframe.set_interp?layer=layer-1&prop=rotation&frame=0&drag=1&mode=linear | pass |
| nothing to send for the first key, or for a side already straight | null, null | null, null | pass |
| Ctrl+click again: every Rotation key as it started, plain and linear, one entry | true, 1 | true, 1 | pass |
| one Ctrl+Z: the auto key again | true | true | pass |
| and one more: as it started | true | true | pass |
| the second Ctrl+click sends the side coming in | true | true | pass |
