# B-615: every key travels with its layer

Written by `tests/b615_layer_shift.rs` from `Fixtures/layer_shift/`, against D-740. Each line is one check: what `tools/layer_shift_reference.py` says, and what the build gives.

| Case | Check | Should | The build | Matches |
| --- | --- | --- | --- | --- |
| FX-LSHIFT-001 | sound0 moved to frame 10 | moved | moved | yes |
| FX-LSHIFT-001 | every key moved with it | 2 keys, each 10 frames on | 2 keys, each 10 frames on | yes |
| FX-LSHIFT-001 | nothing else changed | the in and out points and keys only | the in and out points and keys only | yes |
| FX-LSHIFT-001 | one undo | the project as before | the project as before | yes |
| FX-LSHIFT-002 | sound0 moved to frame 10 | moved | moved | yes |
| FX-LSHIFT-002 | every key moved with it | 2 keys, each 10 frames on | 2 keys, each 10 frames on | yes |
| FX-LSHIFT-002 | nothing else changed | the in and out points and keys only | the in and out points and keys only | yes |
| FX-LSHIFT-002 | one undo | the project as before | the project as before | yes |
| FX-LSHIFT-003 | art moved to frame 10 | moved | moved | yes |
| FX-LSHIFT-003 | every key moved with it | 2 keys, each 10 frames on | 2 keys, each 10 frames on | yes |
| FX-LSHIFT-003 | nothing else changed | the in and out points and keys only | the in and out points and keys only | yes |
| FX-LSHIFT-003 | one undo | the project as before | the project as before | yes |
| FX-LSHIFT-004 | layer-1 moved to frame 10 | moved | moved | yes |
| FX-LSHIFT-004 | every key moved with it | 15 keys, each 10 frames on | 15 keys, each 10 frames on | yes |
| FX-LSHIFT-004 | nothing else changed | the in and out points and keys only | the in and out points and keys only | yes |
| FX-LSHIFT-004 | one undo | the project as before | the project as before | yes |
| FX-LSHIFT-005 | shapes moved to frame 10 | moved | moved | yes |
| FX-LSHIFT-005 | every key moved with it | 2 keys, each 10 frames on | 2 keys, each 10 frames on | yes |
| FX-LSHIFT-005 | nothing else changed | the in and out points and keys only | the in and out points and keys only | yes |
| FX-LSHIFT-005 | one undo | the project as before | the project as before | yes |
| FX-LSHIFT-006 | shapes moved to frame 10 | moved | moved | yes |
| FX-LSHIFT-006 | every key moved with it | 2 keys, each 10 frames on | 2 keys, each 10 frames on | yes |
| FX-LSHIFT-006 | nothing else changed | the in and out points and keys only | the in and out points and keys only | yes |
| FX-LSHIFT-006 | one undo | the project as before | the project as before | yes |
| FX-LSHIFT-007 | shapes moved to frame 10 | moved | moved | yes |
| FX-LSHIFT-007 | every key moved with it | 4 keys, each 10 frames on | 4 keys, each 10 frames on | yes |
| FX-LSHIFT-007 | nothing else changed | the in and out points and keys only | the in and out points and keys only | yes |
| FX-LSHIFT-007 | one undo | the project as before | the project as before | yes |
| FX-LSHIFT-008 | shapes moved to frame 10 | moved | moved | yes |
| FX-LSHIFT-008 | every key moved with it | 4 keys, each 10 frames on | 4 keys, each 10 frames on | yes |
| FX-LSHIFT-008 | nothing else changed | the in and out points and keys only | the in and out points and keys only | yes |
| FX-LSHIFT-008 | one undo | the project as before | the project as before | yes |
| FX-LSHIFT-009 | shapes moved to frame 10 | moved | moved | yes |
| FX-LSHIFT-009 | every key moved with it | 4 keys, each 10 frames on | 4 keys, each 10 frames on | yes |
| FX-LSHIFT-009 | nothing else changed | the in and out points and keys only | the in and out points and keys only | yes |
| FX-LSHIFT-009 | one undo | the project as before | the project as before | yes |
| FX-LSHIFT-010 | shapes moved to frame 10 | moved | moved | yes |
| FX-LSHIFT-010 | every key moved with it | 2 keys, each 10 frames on | 2 keys, each 10 frames on | yes |
| FX-LSHIFT-010 | nothing else changed | the in and out points and keys only | the in and out points and keys only | yes |
| FX-LSHIFT-010 | one undo | the project as before | the project as before | yes |
| FX-LSHIFT-011 | shapes moved to frame 10 | moved | moved | yes |
| FX-LSHIFT-011 | every key moved with it | 2 keys, each 10 frames on | 2 keys, each 10 frames on | yes |
| FX-LSHIFT-011 | nothing else changed | the in and out points and keys only | the in and out points and keys only | yes |
| FX-LSHIFT-011 | one undo | the project as before | the project as before | yes |
| FX-LSHIFT-012 | shapes moved to frame 10 | moved | moved | yes |
| FX-LSHIFT-012 | every key moved with it | 4 keys, each 10 frames on | 4 keys, each 10 frames on | yes |
| FX-LSHIFT-012 | nothing else changed | the in and out points and keys only | the in and out points and keys only | yes |
| FX-LSHIFT-012 | one undo | the project as before | the project as before | yes |
| FX-LSHIFT-013 | shapes moved to frame 10 | moved | moved | yes |
| FX-LSHIFT-013 | every key moved with it | 4 keys, each 10 frames on | 4 keys, each 10 frames on | yes |
| FX-LSHIFT-013 | nothing else changed | the in and out points and keys only | the in and out points and keys only | yes |
| FX-LSHIFT-013 | one undo | the project as before | the project as before | yes |
| FX-LSHIFT-014 | shapes moved to frame -7 | moved | moved | yes |
| FX-LSHIFT-014 | every key moved with it | 6 keys, each -7 frames on | 6 keys, each -7 frames on | yes |
| FX-LSHIFT-014 | nothing else changed | the in and out points and keys only | the in and out points and keys only | yes |
| FX-LSHIFT-014 | one undo | the project as before | the project as before | yes |
| FX-LSHIFT-015 | bar moved to frame 10 | moved | moved | yes |
| FX-LSHIFT-015 | every key moved with it | 2 keys, each 10 frames on | 2 keys, each 10 frames on | yes |
| FX-LSHIFT-015 | nothing else changed | the in and out points and keys only | the in and out points and keys only | yes |
| FX-LSHIFT-015 | one undo | the project as before | the project as before | yes |
| FX-LSHIFT-016 | bg moved to frame 10 | moved | moved | yes |
| FX-LSHIFT-016 | every key moved with it | 4 keys, each 10 frames on | 4 keys, each 10 frames on | yes |
| FX-LSHIFT-016 | nothing else changed | the in and out points and keys only | the in and out points and keys only | yes |
| FX-LSHIFT-016 | one undo | the project as before | the project as before | yes |
| FX-LSHIFT-017 | shapes moved to frame 10 | moved | moved | yes |
| FX-LSHIFT-017 | every key moved with it | 4 keys, each 10 frames on | 4 keys, each 10 frames on | yes |
| FX-LSHIFT-017 | nothing else changed | the in and out points and keys only | the in and out points and keys only | yes |
| FX-LSHIFT-017 | one undo | the project as before | the project as before | yes |
| FX-LSHIFT-018 | shapes moved to frame 5 | moved | moved | yes |
| FX-LSHIFT-018 | every key moved with it | 18 keys, each 5 frames on | 18 keys, each 5 frames on | yes |
| FX-LSHIFT-018 | nothing else changed | the in and out points and keys only | the in and out points and keys only | yes |
| FX-LSHIFT-018 | one undo | the project as before | the project as before | yes |

B-615: 72 of 72 checks pass.
