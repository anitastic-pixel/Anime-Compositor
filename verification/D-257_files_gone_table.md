# D-257: files gone from the disk

Written by `d257_a_deleted_drawing_is_named_and_a_restored_one_is_not` in `app/src/main.rs`. The reference shot copied to `target/d257_files_gone`, and `files_gone`, the answer to the page's `/files-gone`.

**6 of 6 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| the copied reference shot: nothing is gone | none | none | PASS |
| drawing 7 of layer3, which never had a file, is a gap and not a gone file | none | none | PASS |
| layer2_005.png deleted: it is named, with its footage item | layer2: layer2/layer2_005.png | layer2: layer2/layer2_005.png | PASS |
| asking changes nothing: no undo step, no new note | 0 undo steps, notes as before | 0 undo steps, notes as before | PASS |
| the answer names the footage item to relink | asset-layer2 | asset-layer2 | PASS |
| put back: it is no longer named | none | none | PASS |
