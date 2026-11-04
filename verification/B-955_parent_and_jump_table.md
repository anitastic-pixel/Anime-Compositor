# B-955: Shift + parent whip, parent and jump (D-1075)

Written by `b955_parent_and_jump` in `app/src/tests/w12x.rs`.

After Effects' Shift-drag of the parent pick whip (4GeGLgK2BBk, about 3:20). `jumpRequests` is the page's own code run under Node; for the window rows its requests go to the window's own command handler on the reference shot.

**9 of 9 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| a child at (900, 500), a parent whose anchor is (40, 25): the child's Position becomes (40, 25) | /property.set_base?layer=c&prop=position&value=40%2C25 | /property.set_base?layer=c&prop=position&value=40%2C25 | pass |
| a keyed Position is not moved | 0 | 0 | pass |
| a separated Position is not moved | 0 | 0 | pass |
| no transform written: the anchor is (0, 0) | /property.set_base?layer=c&prop=position&value=0%2C0 | /property.set_base?layer=c&prop=position&value=0%2C0 | pass |
| the reference shot: layer-1 parented and jumped to layer-2: its anchor on the frame is the parent's (700.00, 300.00) | true | true | pass |
| entries to undo | 1 | 1 | pass |
| Shift on letting go picks parent and jump; the label says so | true | true | pass |
| Shift is read when letting go, too | true | true | pass |
| every chosen layer, as one entry to undo | true | true | pass |
