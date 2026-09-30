# B-171b: composition frames written to disk by a worker

Written by `tests/b171b_background.rs`. A 1920 by 1080 composition frame is kept by the viewer as B-171 keeps a slow one, and the disk folder looked at straight away.

| Check | Result |
|---|---|
| Keeping a frame returns before its copy is on disk | FAIL |
| A new viewer asking at once gets the frame back bit for bit | pass |
| One copy on disk, laid out byte for byte as B-171 wrote it | pass |

**2 of 3 pass.**
