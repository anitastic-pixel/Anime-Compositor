# P-10: the encode for the page

Document 15's P-10. `WorkingBuffer::to_srgb8_straight` unpremultiplies each pixel, applies the
output transfer function to its three colour channels and quantises the four samples to bytes.
For a full-resolution frame that is 2,073,600 pixels, and it ran in **one serial pass on the
thread that asked for the frame** while the other twenty-three sat idle.

`verification/P-01_frame_trace.md` measured that pass at 52 to 57 ms of every full-resolution
frame — after P-03 landed it was the **largest single named stage** of a warm full frame, larger
than the tile loop, larger than the effect stack, larger than the decode.

**One pixel in, four samples out, and nothing a pixel does depends on any other pixel. The pass
is now one `rayon` map across the same thread pool the renderer already owns. The arithmetic per
pixel is unchanged and the manifest of 960 frames is identical to the byte.**

## What this entry was allowed to do, and what it was not

Document 15's entry is explicit, and it is worth quoting because the boundary is the whole
design: the encode may be split across threads, and it may **not** "approximate the transfer
function, table it, or change its arithmetic; that is the direction P-03(c) excluded and it stays
excluded."

So P-03(c)'s 256-entry table is on the **input** side only and is untouched here. This item calls
the same `composite::unpremultiply`, the same `color::linear_to_srgb` on the same three channels
and the same `color::quantise_u8` / `quantise_u16` as before, on the same pixels, in the same
order. What changed is which thread runs which pixel — and because the answer to "what colour is
this pixel" may not depend on which thread asked, that is a change with no room in it for a
result to drift.

The destination is allocated once and split with `par_chunks_mut` into one disjoint four-sample
piece per pixel, so **no two threads are ever handed the same byte** — the same property item
P-03(d) established for tiles writing into the frame, and enforced the same way, by the borrow
checker rather than by an index computed after the fact.

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: cargo release profile, `opt-level = 3`, debug assertions off
- Threads rayon was given: 24
- Harness: `tests/p01_frame_trace.rs`, both of its tests, unmodified — 20 frames a row, stepping
  by 97 through each fixture's 240-frame work area, p50 by nearest rank
- Before: the build item P-03(f) left. After: that build with this one change to `src/lib.rs`
- **P-01 was run twice on the after build**, and both runs are reported below. One row disagreed
  between them and that is the reason both columns are here rather than one
- Frame budget at 24 fps: 41.667 ms

## The stage this item exists to move

P-01's twelve rows, the encode stage alone, p50 in milliseconds.

| Workload | Quality | Cache state | Before | After, run 1 | After, run 2 | Change |
|---|---|---|---|---|---|---|
| the reference shot | Draft | cold, first read | 2.883 | 0.582 | 0.586 | **-79.7%** |
| the reference shot | Draft | cold, OS cache warm | 2.847 | 0.615 | 0.615 | **-78.4%** |
| the reference shot | Draft | everything warm | 3.618 | 0.450 | 0.431 | **-88.1%** |
| the reference shot | Full | cold, first read | 57.075 | 3.915 | 3.928 | **-93.1%** |
| the reference shot | Full | cold, OS cache warm | 54.961 | 3.983 | 3.938 | **-92.8%** |
| the reference shot | Full | everything warm | 56.201 | 4.051 | 3.880 | **-93.1%** |
| the declared fixture | Draft | cold, first read | 3.489 | 0.707 | 0.685 | **-80.4%** |
| the declared fixture | Draft | cold, OS cache warm | 3.986 | 0.701 | 0.686 | **-82.8%** |
| the declared fixture | Draft | everything warm | 4.055 | 0.611 | 0.609 | **-85.0%** |
| the declared fixture | Full | cold, first read | 54.678 | 3.913 | 3.977 | **-92.7%** |
| the declared fixture | Full | cold, OS cache warm | 53.333 | 3.899 | 3.848 | **-92.8%** |
| the declared fixture | Full | everything warm | 52.604 | 4.022 | 3.969 | **-92.5%** |

**Twelve rows, twelve improvements, and the two after-runs agree with each other to within a
tenth of a millisecond in every row.** The full-resolution rows fall by a factor of about
fourteen and the draft rows by about five.

Neither figure is twenty-four, and this artifact does not present the gap as a disappointment.
A full frame is 33 MB of floats read and 8 MB of bytes written, so past a certain thread count
the pass is waiting on memory rather than on arithmetic; and a draft frame is a sixteenth of the
work, small enough that the fan-out and join are a visible share of what is left. **The stage is
bounded by memory bandwidth and by the frame's size, not by the thread count**, and the two
resolutions falling by different factors is exactly what that explanation predicts.

## What it did to a frame

The same twelve rows, frame time, p50 in milliseconds. Both after-runs again.

| Workload | Quality | Cache state | Before | After, run 1 | After, run 2 | Change (run 2) |
|---|---|---|---|---|---|---|
| the reference shot | Draft | cold, first read | 89.397 | 84.978 | 84.087 | -5.9% |
| the reference shot | Draft | cold, OS cache warm | 86.563 | 84.906 | 84.269 | -2.7% |
| the reference shot | Draft | everything warm | 5.621 | 2.308 | 2.254 | **-59.9%** |
| the reference shot | Full | cold, first read | 149.887 | 101.198 | 98.344 | **-34.4%** |
| the reference shot | Full | cold, OS cache warm | 148.114 | 100.347 | 99.017 | **-33.1%** |
| the reference shot | Full | everything warm | 64.431 | 13.126 | 13.061 | **-79.7%** |
| the declared fixture | Draft | cold, first read | 372.617 | 369.716 | 365.619 | -1.9% |
| the declared fixture | Draft | cold, OS cache warm | 374.229 | 369.055 | 369.866 | -1.2% |
| the declared fixture | Draft | everything warm | 144.310 | 162.701 | 143.491 | -0.6% |
| the declared fixture | Full | cold, first read | 434.646 | 391.132 | 385.400 | **-11.3%** |
| the declared fixture | Full | cold, OS cache warm | 432.664 | 395.089 | 388.201 | **-10.3%** |
| the declared fixture | Full | everything warm | 204.005 | 163.439 | 160.855 | **-21.2%** |

**One row disagrees between the two after-runs, and it is the reason there are two.** The
declared fixture's warm draft frame reads +12.7% in run 1 and -0.6% in run 2, against a stage
that fell by 3.4 ms in both. A row whose repeat runs differ by 19 ms cannot be read as a result
in either direction, and the honest statement is that **this item does not move that row**: its
encode is 4 ms of a 144 ms frame that is dominated by the effect stack, so there is not enough
of it there to move. Every other row agrees between the runs to within about 3%.

The reference shot's warm draft frame is **2.254 ms, 0.054x the 24 fps budget**, and its warm
full frame is **13.061 ms, 0.313x** — the first full-resolution row this project has measured
inside the budget.

## A first playthrough, which is the row a person sits through

Item P-03(b)'s harness: the viewer's own 1 GiB D-40 budget, every layer of every frame a miss,
which is what opening a project and pressing play is.

| Workload | Quality | Frame before | Frame after | Change | Against 24 fps |
|---|---|---|---|---|---|
| the reference shot | Draft | 25.414 | **22.960** | -9.7% | 0.61x to **0.55x** |
| the reference shot | Full | 72.998 | **32.988** | **-54.8%** | 1.75x to **0.79x** |
| the declared fixture | Draft | 183.990 | 172.498 | -6.2% | 4.42x to 4.14x |
| the declared fixture | Full | 258.900 | **206.926** | **-20.1%** | 6.21x to 4.97x |

And the stage inside those rows: 2.870 to 0.486, 44.726 to 3.920, 4.242 to 0.632, and 58.783 to
4.027 — between -83.1% and -93.1%, agreeing with P-01's table.

**A first pass through the reference shot at full resolution is now inside the frame budget**, at
0.79x, which is a row that read 2.57x before P-03 began. The declared ten-layer fixture is still
outside it in both qualities and D-47 stays open; what stands between it and the budget is now
the effect stack, which P-11 is the entry for.

## The proof that no pixel moved

`tests/p03_byte_equality.rs`: 960 frames across both fixtures at both qualities, each rendered
with no cache and again through one cache with the viewer's D-40 budget, every pair equal, and
the manifest of all 960 lines **identical to the one item P-03(f) left**, over 4,230,144,000
bytes.

That test is the right proof for this item in particular, because **the manifest is a hash of
exactly what this item computes**: every line of it is `to_srgb8_straight`'s output reduced to a
digest, so a single sample that quantised differently on a worker thread would change a line.
960 of 960 lines identical is 960 full encodes, 4.2 GB of samples, produced across twenty-four
threads and equal to the serial run byte for byte.

The whole test suite passed with `git diff --exit-code -- verification/` clean, so no other
artifact in this directory moved either.

## What this leaves for the next entry

P-01's ranking is now different from the one it opened with. On a warm full-resolution reference
shot the encode was 63.2% of the frame and is now about 30% of a frame a fifth the size; the
stage that is left largest across the declared fixture's rows is the effect stack, which is what
**P-11** proposes to cache. On the cold draft rows, which barely moved here, what remains is the
read and the decode, and those are bounded by one disk.

One thing this item did **not** do, and it is the same boundary item P-03(b) drew: it changes the
encode for every caller, including export, because the encode is not the cache and ADR-015 bound
3 says nothing about it. Export gets this saving on every frame it writes. The open question that
bound 3 does raise — whether the **parallel decode** may run on the no-budget path — is still the
owner's to answer and is not touched here.
