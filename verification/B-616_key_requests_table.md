# B-616: chosen keys of masks', shapes' and groups' numbers

Written by `app/src/tests/key_requests.rs` from `Fixtures/key_requests/`, against D-741. Each case sends the timeline's request with the keys chosen, as the page sends it. Each line is one check: what `tools/key_requests_reference.py` says, and what the build gives.

| Case | Check | Should | The build | Matches |
| --- | --- | --- | --- | --- |
| FX-KREQ-001 | 2 keys moved by 3 | done | done | yes |
| FX-KREQ-001 | every key of the layer, frame, value and ease | 2 keys in 1 place | 2 keys in 1 place | yes |
| FX-KREQ-001 | nothing else changed | the keys only | the keys only | yes |
| FX-KREQ-001 | one undo | the project as before | the project as before | yes |
| FX-KREQ-002 | 2 keys deleted | done | done | yes |
| FX-KREQ-002 | every key of the layer, frame, value and ease | 0 keys in 0 places | 0 keys in 0 places | yes |
| FX-KREQ-002 | /shapes/0/trim/end with no keys left | holds 0 | holds 0 | yes |
| FX-KREQ-002 | nothing else changed | the keys only | the keys only | yes |
| FX-KREQ-002 | one undo | the project as before | the project as before | yes |
| FX-KREQ-003 | 2 keys reversed | done | done | yes |
| FX-KREQ-003 | every key of the layer, frame, value and ease | 2 keys in 1 place | 2 keys in 1 place | yes |
| FX-KREQ-003 | nothing else changed | the keys only | the keys only | yes |
| FX-KREQ-003 | one undo | the project as before | the project as before | yes |
| FX-KREQ-004 | 1 key moved by 4 | refused | refused: This number would have a second key at frame 4, where a key that is not chosen is. Choose it too, or move it first. | yes |
| FX-KREQ-004 | every key of the layer, frame, value and ease | 2 keys in 1 place | 2 keys in 1 place | yes |
| FX-KREQ-004 | nothing else changed | the keys only | the keys only | yes |
| FX-KREQ-005 | 6 keys moved by 2 | done | done | yes |
| FX-KREQ-005 | every key of the layer, frame, value and ease | 21 keys in 10 places | 21 keys in 10 places | yes |
| FX-KREQ-005 | nothing else changed | the keys only | the keys only | yes |
| FX-KREQ-005 | one undo | the project as before | the project as before | yes |
| FX-KREQ-006 | 2 keys deleted | done | done | yes |
| FX-KREQ-006 | every key of the layer, frame, value and ease | 19 keys in 10 places | 19 keys in 10 places | yes |
| FX-KREQ-006 | nothing else changed | the keys only | the keys only | yes |
| FX-KREQ-006 | one undo | the project as before | the project as before | yes |
| FX-KREQ-007 | 4 keys reversed | done | done | yes |
| FX-KREQ-007 | every key of the layer, frame, value and ease | 21 keys in 10 places | 21 keys in 10 places | yes |
| FX-KREQ-007 | nothing else changed | the keys only | the keys only | yes |
| FX-KREQ-007 | one undo | the project as before | the project as before | yes |
| FX-KREQ-008 | 4 keys moved by 5 | done | done | yes |
| FX-KREQ-008 | every key of the layer, frame, value and ease | 4 keys in 2 places | 4 keys in 2 places | yes |
| FX-KREQ-008 | nothing else changed | the keys only | the keys only | yes |
| FX-KREQ-008 | one undo | the project as before | the project as before | yes |
| FX-KREQ-009 | 2 keys deleted | done | done | yes |
| FX-KREQ-009 | every key of the layer, frame, value and ease | 2 keys in 2 places | 2 keys in 2 places | yes |
| FX-KREQ-009 | nothing else changed | the keys only | the keys only | yes |
| FX-KREQ-009 | one undo | the project as before | the project as before | yes |
| FX-KREQ-010 | 4 keys reversed | done | done | yes |
| FX-KREQ-010 | every key of the layer, frame, value and ease | 4 keys in 2 places | 4 keys in 2 places | yes |
| FX-KREQ-010 | nothing else changed | the keys only | the keys only | yes |
| FX-KREQ-010 | one undo | the project as before | the project as before | yes |
| FX-KREQ-011 | 4 keys moved by 4 | done | done | yes |
| FX-KREQ-011 | every key of the layer, frame, value and ease | 4 keys in 2 places | 4 keys in 2 places | yes |
| FX-KREQ-011 | nothing else changed | the keys only | the keys only | yes |
| FX-KREQ-011 | one undo | the project as before | the project as before | yes |
| FX-KREQ-012 | 4 keys reversed | done | done | yes |
| FX-KREQ-012 | every key of the layer, frame, value and ease | 4 keys in 2 places | 4 keys in 2 places | yes |
| FX-KREQ-012 | nothing else changed | the keys only | the keys only | yes |
| FX-KREQ-012 | one undo | the project as before | the project as before | yes |
| FX-KREQ-013 | 5 keys moved by 6 | done | done | yes |
| FX-KREQ-013 | every key of the layer, frame, value and ease | 6 keys in 3 places | 6 keys in 3 places | yes |
| FX-KREQ-013 | nothing else changed | the keys only | the keys only | yes |
| FX-KREQ-013 | one undo | the project as before | the project as before | yes |
| FX-KREQ-014 | 2 keys deleted | done | done | yes |
| FX-KREQ-014 | every key of the layer, frame, value and ease | 4 keys in 2 places | 4 keys in 2 places | yes |
| FX-KREQ-014 | /groups/1/operations/0/amount with no keys left | holds -60 | holds -60 | yes |
| FX-KREQ-014 | nothing else changed | the keys only | the keys only | yes |
| FX-KREQ-014 | one undo | the project as before | the project as before | yes |
| FX-KREQ-015 | 2 keys moved by 2 | done | done | yes |
| FX-KREQ-015 | every key of the layer, frame, value and ease | 2 keys in 1 place | 2 keys in 1 place | yes |
| FX-KREQ-015 | nothing else changed | the keys only | the keys only | yes |
| FX-KREQ-015 | one undo | the project as before | the project as before | yes |
| FX-KREQ-016 | 2 keys reversed | done | done | yes |
| FX-KREQ-016 | every key of the layer, frame, value and ease | 2 keys in 1 place | 2 keys in 1 place | yes |
| FX-KREQ-016 | nothing else changed | the keys only | the keys only | yes |
| FX-KREQ-016 | one undo | the project as before | the project as before | yes |

B-616: 65 of 65 checks pass.
