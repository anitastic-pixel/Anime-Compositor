# P-04: what a keystroke waits for while a frame is being made

Document 15's P-04. This page is the measurement half of the unit; the property half — that a
command is answered *while* a frame is being rendered rather than after it — is
`verification/P-04_responsiveness_table.md`, which is checked on every build and carries no
duration at all, so that a run on a different machine does not rewrite it.

**A keystroke pressed while a frame of the declared ten-layer fixture is being made falls from
46.757 ms to 0.001 ms at p50, and from 181.882 ms to 0.004 ms at p95. At full resolution it falls
from 67.240 ms to 0.001 ms at p50 and from 193.009 ms to 0.005 ms at p95. Frame times did not
regress. Before the change, not one press out of thirteen was answered while a frame was still
being made; after it, 814 out of 815.**

## What was wrong

`serve` took the viewer mutex and held it through planning, rendering and the encode. Every
command in the window needs that same mutex, so every command queued behind the frame in flight:
on the declared fixture a keystroke waited most of a fifth of a second, and at p95 most of a
fifth of a second twice over. The page made it worse — its `inFlight` flag *dropped* any request
that arrived while one was out, which is neither a mailbox nor a cancellation, so a second press
during a slow frame simply did not happen.

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`
- Harness: `p04_responsiveness` in `app/src/main.rs`, `#[ignore]`d and run deliberately. One
  thread asks the window for twelve frames of the declared ten-layer fixture, scattered across
  its work area so the cache is not answering out of one entry; another presses
  `viewer.toggle_checkerboard` — a command document 24 lists — every millisecond and times how
  long the window took to answer each press.
- Before: the build P-11 left, with the harness applied to it and nothing else changed
- Date: 2026-09-09

## The measurement

| Quality | Frame p50 | Frame p95 | Press p50 | Press p95 | Worst press | Answered inside a frame |
|---|---|---|---|---|---|---|
| Draft, before | 47.732 | 182.893 | **46.757** | **181.882** | 181.882 | 0 of 13 |
| Draft, after | 44.438 | 175.016 | **0.001** | **0.004** | 0.011 | 814 of 815 |
| Full, before | 68.177 | 194.092 | **67.240** | **193.009** | 193.009 | 0 of 13 |
| Full, after | 67.453 | 213.088 | **0.001** | **0.005** | 0.010 | 877 of 884 |

Milliseconds, nearest-rank. A second after run read 46.415 / 0.001 / 0.005 Draft and 70.933 /
0.001 / 0.005 Full, so the frame column moves by a couple of milliseconds between runs and the
press column does not move at all.

The "answered inside a frame" column is the honest one. It counts presses that began *and* ended
between the start and the end of one frame that was still being rendered, and a press like that
cannot have waited for that frame. Before the change the count is zero **by construction** — the
render held the lock the command needed, so the only moment a command could be answered was one
where no frame was in flight — and thirteen presses in a whole pass is itself the finding: the
command thread pressed a key every millisecond and got thirteen answers, one per frame.

## What the change costs

A frame no longer borrows the project from the viewer; it copies it out under a short lock and
renders from the copy. That copy is layer and asset records, not pixels:

**p50 0.0114 ms, p95 0.0116 ms, worst 0.0174 ms over two hundred copies.** Against a frame of
44 ms that is 0.026%, and it is what the whole saving is bought with.

The decoded cels are not copied. They move behind a lock of their own, which the render holds for
its whole length and no command ever touches — the one thing a render can keep without a command
waiting on it.

## The photograph

`verification/P-04_window.png`, taken with:

```
cargo build -p anime_compositor_app --release
powershell -ExecutionPolicy Bypass -File tools/capture_window.ps1 -Name P-04_window -Keys "d{RIGHT}{RIGHT}" -Settle 4000
```

`d` puts the viewer into full resolution, where a frame of this composition is at its slowest,
and then two arrow presses step two frames. The readout says **frame 2, Full — 1920×1080, same as
export**: both presses landed. Before P-04 the second one arrived while the first one's refresh
was still out and the page threw it away, and the window would have read frame 1.

`tools/capture_window.ps1` grew the ability to press a key that has no printable character for
this. It sent `-Keys` one character at a time through `VkKeyScan`, with only Tab named, so
`"{RIGHT}"` went through as the seven characters it is spelled with — the first attempt at this
photograph read frame 0 and looked like a broken viewer. Named keys in braces now map to virtual
key codes, with the extended-key bit set for the arrows, and the picture above is the check that
they arrive.

## What P-04 does not do

Document 33 proposes p95 command acknowledgement at or below 16 ms and a useful drag preview at
or below 50 ms as candidate targets. **Neither is adopted here.** P-04 reports the measurement;
if the owner wants a target it goes into document 08 beside the ones already there. The
measurement clears both by a wide margin, which is a fact about this machine and this fixture and
not a promise.
