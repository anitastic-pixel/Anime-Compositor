# B-744 (D-864): export reads a frame's footage side by side

Written by `tests/b744_export_read_ahead.rs`. Each export below was run by the build before B-744 and by this one, and this page is the same, word for word, from both: the same files to the byte (FNV-1a), the same diagnostics in the same order, the same status and progress count. "Broken footage" is the ten-layer declared fixture with layer 3's and layer 8's drawings gone and layer 6's files replaced by text, so the missing and unreadable footage sits in the middle of the stack.

| Export | Row | Result |
|---|---|---|
| reference shot 10-17, render transparent | status | Completed, 8 of 8 files, progress 8 |
| reference shot 10-17, render transparent | f_0010.png | d54eff42a4b579e3 (1968076 bytes) |
| reference shot 10-17, render transparent | f_0011.png | 1ab443f28e154303 (1961154 bytes) |
| reference shot 10-17, render transparent | f_0012.png | 6356118e36dc5c8a (1979909 bytes) |
| reference shot 10-17, render transparent | f_0013.png | 8db3dc1d1ead9772 (1997058 bytes) |
| reference shot 10-17, render transparent | f_0014.png | 37e922a31bc8cd30 (2075278 bytes) |
| reference shot 10-17, render transparent | f_0015.png | f60b80a71443e9ff (2088461 bytes) |
| reference shot 10-17, render transparent | f_0016.png | 1e2ff2c697058e9e (2041833 bytes) |
| reference shot 10-17, render transparent | f_0017.png | bd679352d117b7be (2026291 bytes) |
| reference shot 10-17, render transparent | diagnostic 1 | MEDIA_SEQUENCE_GAP Warning: Frame 14 exposes drawing 7 of layer3_%03d.png, which is missing. |
| reference shot 10-17, render transparent | diagnostic 2 | MEDIA_SEQUENCE_GAP Warning: Frame 15 exposes drawing 7 of layer3_%03d.png, which is missing. |
| reference shot 0-7, block | status | Completed, 8 of 8 files, progress 8 |
| reference shot 0-7, block | f_0000.png | 0b71dc46705606bc (2003389 bytes) |
| reference shot 0-7, block | f_0001.png | 8baba83c57b0d9f7 (2008198 bytes) |
| reference shot 0-7, block | f_0002.png | 4b447f776af318aa (1989855 bytes) |
| reference shot 0-7, block | f_0003.png | be8e99cbba9c1489 (1987880 bytes) |
| reference shot 0-7, block | f_0004.png | 743b0e79270c44f0 (1996371 bytes) |
| reference shot 0-7, block | f_0005.png | d005be07e9839336 (2008299 bytes) |
| reference shot 0-7, block | f_0006.png | 7643356c6f549569 (2010446 bytes) |
| reference shot 0-7, block | f_0007.png | dd630c4b15b8eaac (2014722 bytes) |
| reference shot 10-17, block | status | Blocked, 0 of 8 files, progress 0 |
| reference shot 10-17, block | diagnostic 1 | EXPORT_BLOCKED_MISSING_MEDIA Error: 2 of the 8 frames asked for have a drawing that is missing, so nothing was exported. |
| reference shot 10-17, block | diagnostic 2 | MEDIA_SEQUENCE_GAP Warning: Frame 14 exposes drawing 7 of layer3_%03d.png, which is missing. |
| reference shot 10-17, block | diagnostic 3 | MEDIA_SEQUENCE_GAP Warning: Frame 15 exposes drawing 7 of layer3_%03d.png, which is missing. |
| broken footage 0-5, render transparent | status | Completed, 6 of 6 files, progress 6 |
| broken footage 0-5, render transparent | f_0000.png | f0f81f8be72e7677 (1937113 bytes) |
| broken footage 0-5, render transparent | f_0001.png | fa775731691ed6b4 (1941354 bytes) |
| broken footage 0-5, render transparent | f_0002.png | da4215359464d785 (1922779 bytes) |
| broken footage 0-5, render transparent | f_0003.png | 2d633c5d2fb54199 (1927234 bytes) |
| broken footage 0-5, render transparent | f_0004.png | 67fb1b54eb101874 (1935564 bytes) |
| broken footage 0-5, render transparent | f_0005.png | 0893b078e51d435a (1947194 bytes) |
| broken footage 0-5, render transparent | diagnostic 1 | MEDIA_MISSING Warning: copy3/layer3_000.png is not where the project says it is. |
| broken footage 0-5, render transparent | diagnostic 2 | MEDIA_DECODE_FAILED Error: layer3_000.png could not be read. |
| broken footage 0-5, render transparent | diagnostic 3 | MEDIA_MISSING Warning: copy8/layer2_000.png is not where the project says it is. |
| broken footage 0-5, render transparent | diagnostic 4 | MEDIA_MISSING Warning: copy3/layer3_000.png is not where the project says it is. |
| broken footage 0-5, render transparent | diagnostic 5 | MEDIA_DECODE_FAILED Error: layer3_000.png could not be read. |
| broken footage 0-5, render transparent | diagnostic 6 | MEDIA_MISSING Warning: copy8/layer2_001.png is not where the project says it is. |
| broken footage 0-5, render transparent | diagnostic 7 | MEDIA_MISSING Warning: copy3/layer3_001.png is not where the project says it is. |
| broken footage 0-5, render transparent | diagnostic 8 | MEDIA_DECODE_FAILED Error: layer3_001.png could not be read. |
| broken footage 0-5, render transparent | diagnostic 9 | MEDIA_MISSING Warning: copy8/layer2_002.png is not where the project says it is. |
| broken footage 0-5, render transparent | diagnostic 10 | MEDIA_MISSING Warning: copy3: 6 frames affected. The first 3 are logged in full above. |
| broken footage 0-5, render transparent | diagnostic 11 | MEDIA_DECODE_FAILED Error: copy6: 6 frames affected. The first 3 are logged in full above. |
| broken footage 0-5, render transparent | diagnostic 12 | MEDIA_MISSING Warning: copy8: 6 frames affected. The first 3 are logged in full above. |
| broken footage 0-5, block | status | Blocked, 0 of 6 files, progress 0 |
| broken footage 0-5, block | diagnostic 1 | EXPORT_BLOCKED_MISSING_MEDIA Error: 6 of the 6 frames asked for have a drawing that is missing, so nothing was exported. |
| broken footage 0-5, block | diagnostic 2 | MEDIA_MISSING Warning: copy3/layer3_000.png is not where the project says it is. |
| broken footage 0-5, block | diagnostic 3 | MEDIA_DECODE_FAILED Error: layer3_000.png could not be read. |
| broken footage 0-5, block | diagnostic 4 | MEDIA_MISSING Warning: copy8/layer2_000.png is not where the project says it is. |
| broken footage 0-5, block | diagnostic 5 | MEDIA_MISSING Warning: copy3/layer3_000.png is not where the project says it is. |
| broken footage 0-5, block | diagnostic 6 | MEDIA_DECODE_FAILED Error: layer3_000.png could not be read. |
| broken footage 0-5, block | diagnostic 7 | MEDIA_MISSING Warning: copy8/layer2_001.png is not where the project says it is. |
| broken footage 0-5, block | diagnostic 8 | MEDIA_MISSING Warning: copy3/layer3_001.png is not where the project says it is. |
| broken footage 0-5, block | diagnostic 9 | MEDIA_DECODE_FAILED Error: layer3_001.png could not be read. |
| broken footage 0-5, block | diagnostic 10 | MEDIA_MISSING Warning: copy8/layer2_002.png is not where the project says it is. |
| broken footage 0-5, block | diagnostic 11 | MEDIA_MISSING Warning: copy3: 6 frames affected. The first 3 are logged in full above. |
| broken footage 0-5, block | diagnostic 12 | MEDIA_DECODE_FAILED Error: copy6: 6 frames affected. The first 3 are logged in full above. |
| broken footage 0-5, block | diagnostic 13 | MEDIA_MISSING Warning: copy8: 6 frames affected. The first 3 are logged in full above. |
| reference shot 0-7, cancelled | status | Cancelled, 0 of 8 files, progress 0 |
| reference shot 0-7, cancelled | diagnostic 1 | EXPORT_CANCELLED Info: Export stopped at your request after 0 of 8 frames. |
