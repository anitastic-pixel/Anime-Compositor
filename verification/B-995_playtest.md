# B-995 playtest: the project text for the window is written once, outside the lock (D-1115)

What changed: after every edit the window asks for the whole project. Before, the program wrote
the project as text, read that text back, and wrote it again, all while holding the lock that
every frame needs, so no frame could be shown meanwhile. Now it takes a copy under the lock
(0.2 ms at 200 layers), lets go, and writes the text once. What the window is sent is the same
project, every field, so nothing it shows changes.

## Before and after

Busy machine (other lanes' builds running): these timings are not final. AMD Ryzen 9 9900X,
RTX 4070 Ti SUPER, Windows 11 Education 10.0.26300; release build, `CARGO_TARGET_DIR=target/lanes/p27b`,
`CARGO_INCREMENTAL=0`, `-j 4`. Before: commit f8264a63 (p20) and 7ccfbdcd (p21, the audit's exe).
After: f8264a63 plus this change, with the B-996 autosave change in the same exe (p21 does not
wait the two minutes autosave needs, so it does not touch these numbers).

p20 (`tests/p20_edit_cost.rs`), medians, 15 to 51 runs:

| Step | 4 layers | 50 layers | 200 layers |
|---|---|---|---|
| Time frames are held back per edit, before (whole `/state` under the lock) | 1.22 ms | 49.7 ms | 212.6 ms |
| Time frames are held back per edit, after (copies under the lock) | 0.009 ms | 0.055 ms | **0.209 ms** |
| Writing the text, after (outside the lock, frames keep coming) | 0.58 ms | 30.1 ms | 124.7 ms |

p21 (`tools/p21_page_cost.py`, a background copy of the app with its own web view profile,
nothing on screen), 7 runs, medians:

| Measure | Reference shot before | after | 200 layers before | after |
|---|---|---|---|---|
| Text sent (bytes) | 32,952 | 32,952 | 3,437,027 | 3,437,027 |
| Page waits for it (ms) | 13.4 | 11.6 | 733.3 | 638.1 |
| One rename, start to finish on the page (ms) | 16 | 13.9 | 1,815 | 1,660 |

The page's own wait barely moves: it is mostly the transfer and the page's redraw, which
B-997 (D-1116) and B-998 (D-1117) address. What this change buys is the hold on frames: 213 ms
down to 0.2 ms at 200 layers.

## Try it

1. Open a big project (100 or more layers), press play or start a RAM preview, and while it
   plays rename a layer or type a new value into a field, several times.
   - Expect: playback keeps going while you edit; before, it stuttered on every edit.
2. Undo and redo a few times.
   - Expect: every panel shows the same as it did before this change.

Pass if every "Expect" holds. If anything looks different, say which step.

Checked by `app/src/tests/p27_speed.rs`, `d1115_state_is_the_same_json_as_before`: for the
reference shot and the 200-layer copy, before and after a rename, the new answer read as JSON
equals the old one, every field, and is no longer than it (32,954 and 3,437,029 bytes, the same
as before). The whole app suite passed (162 tests) on the first version of this change, and the 90 app
tests that read `/state` (editing, contract, blur outline, w12v, p27) passed on this one. No drawing code changed; the p03 manifest
(`tests/p03_byte_equality.rs`) is taken on this build as the "before" that B-999 and B-1000,
which do touch the project and the frame memory, are compared against.
