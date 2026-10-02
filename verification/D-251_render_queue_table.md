# D-251: the render queue

Written by `d251_the_queue_writes_what_one_export_writes` in `app/src/main.rs`. The reference shot, frames 0 to 11, with missing drawings written as transparent. "One Export" is the window's Export of today, into a folder of its own.

**19 of 19 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| one Export writes twelve PNG frames, one GIF and one MP4 | 12, 1, 1 | 12, 1, 1 | PASS |
| four rows added | 4 | 4 | PASS |
| before Render: three ticked, one not, all queued | [x] Queued, [x] Queued, [x] Queued, [ ] Queued | [x] Queued, [x] Queued, [x] Queued, [ ] Queued | PASS |
| Render writes the ticked rows in turn | Rendered 3 of 3 rows. | Rendered 3 of 3 rows. | PASS |
| row 1, PNG, is what one Export writes | 12 files, every byte the same | 12 files, every byte the same | PASS |
| row 2, GIF, is what one Export writes | 1 file, every byte the same | 1 file, every byte the same | PASS |
| row 3, MP4, is what one Export writes, but for the second each was written at | 1 file, every byte the same | 1 file, every byte the same | PASS |
| the unticked row writes nothing | empty | empty | PASS |
| after: the three are done and unticked, the fourth still waits | [ ] Done, [ ] Done, [ ] Done, [ ] Queued | [ ] Done, [ ] Done, [ ] Done, [ ] Queued | PASS |
| Stop during row 2 is said | Stopped during row 2 of 3. | Stopped during row 2 of 3. | PASS |
| row 1 is whole | 12 files, every byte the same | 12 files, every byte the same | PASS |
| row 2 keeps the frames that finished, fewer than twelve | some, fewer than 12 | some, fewer than 12 | PASS |
| row 3 is not started | 0 files | 0 files | PASS |
| and the rows say so | [ ] Done, [x] Stopped, [x] Not started | [ ] Done, [x] Stopped, [x] Not started | PASS |
| a row dragged to the top is first | 4 1 2 3 | 4 1 2 3 | PASS |
| a row removed with its × is gone | 3 | 3 | PASS |
| a row added before any edit is not marked | false | false | PASS |
| after an edit, the row is marked as added before it | true | true | PASS |
| Refresh takes the project again | false | false | PASS |
