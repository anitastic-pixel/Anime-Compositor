# P-03: the tier-zero items, before and after

Document 15's P-03 is a list of six small changes, each of which must show a before and after from
P-01's timer and prove that no pixel moved. This file is the table, and it gains a section per
item as each one lands. **Item (c) is done. (a), (b), (d), (e) and (f) are not started.**

The order is `verification/P-01_frame_trace.md`'s measured order and not either research
document's estimate: (c), (a), (b), then (d), (e), (f).

## Machine, build and configuration

The same machine as P-01, because a before and after measured on two machines is neither.

- AMD Ryzen 9 9900X, 24 threads.
- Windows 11 Education, 10.0.26200.
- `cargo test --release`, `opt-level = 3`, debug assertions off.
- The harness is `tests/p01_frame_trace.rs`, unchanged, run as
  `cargo test --release --test p01_frame_trace -- --ignored`.

The "before" column of every table below is the artifact committed with P-01. **That artifact is
not overwritten by this file.** It is the dated baseline every later entry is measured against,
and a baseline that moves is not one.

---

# Item (c): the sRGB decode table

**One line: the transfer function and premultiply stage falls by 20.5% to 40.7% depending on the
row, the cold frame time falls by 10.3% to 23.5%, and not one of 4,230,144,000 encoded bytes
moved.**

## What changed, and how small it is

`src/color.rs`, and nothing else under `src/`. `srgb_to_linear` keeps its signature, its exactness
and every caller. Before the existing arithmetic it tries a lookup in a 256-entry table built from
the same arithmetic, and the lookup is *guarded*: the index is trusted only if the table's own
dequantised value has the same **bit pattern** as the argument. A value that did not come from an
eight-bit sample — an interpolated one, a negative zero, a NaN — misses the guard and takes the
slow path, unchanged.

The bit-pattern comparison is not fussiness. `-0.0 == 0.0` is true in floating point, so an
equality guard would have returned `+0.0` where the arithmetic returns `-0.0`, and that is one
byte moving. The unit test `values_outside_the_table_and_not_a_number_take_the_slow_path` is
where that seam is held.

What this item did **not** do, deliberately: no new field on `ImageBuffer`, no second path through
`decode_png -> retag -> into_working`, no API change, and no change to `linear_to_srgb`. The
encode direction has four billion possible inputs rather than 256, no table can be exact for it,
and it is P-10's entry.

## The stage this item aimed at

P-01 ranked "transfer function and premultiply" first in all eight of its cold rows, at 30.7% to
50.7% of a frame. Here it is, before and after, at the median of twenty frames:

| Workload | Quality | Cache state | Before (p50 ms) | After (p50 ms) | Change |
|---|---|---|---|---|---|
| the reference shot | Draft | cold, files first read by this process | 62.593 | 42.897 | **-31.5%** |
| the reference shot | Draft | cold, operating system file cache warm | 72.101 | 45.022 | **-37.6%** |
| the reference shot | Full | cold, files first read by this process | 74.813 | 44.983 | **-39.9%** |
| the reference shot | Full | cold, operating system file cache warm | 75.790 | 44.924 | **-40.7%** |
| the declared ten-layer fixture | Draft | cold, files first read by this process | 172.571 | 136.141 | **-21.1%** |
| the declared ten-layer fixture | Draft | cold, operating system file cache warm | 173.621 | 137.950 | **-20.5%** |
| the declared ten-layer fixture | Full | cold, files first read by this process | 175.316 | 137.149 | **-21.8%** |
| the declared ten-layer fixture | Full | cold, operating system file cache warm | 174.357 | 136.763 | **-21.6%** |

The four warm rows are absent from this table on purpose: the stage reads 0.000 ms in them both
before and after, because a cache hit does not decode a cel. **This item cannot help a warm frame
and does not claim to.**

## The frame someone actually waits for

| Workload | Quality | Cache state | Before (p50 ms) | After (p50 ms) | Change |
|---|---|---|---|---|---|
| the reference shot | Draft | cold, files first read by this process | 129.928 | 104.167 | **-19.8%** |
| the reference shot | Draft | cold, operating system file cache warm | 143.507 | 109.762 | **-23.5%** |
| the reference shot | Full | cold, files first read by this process | 217.258 | 182.745 | **-15.9%** |
| the reference shot | Full | cold, operating system file cache warm | 219.760 | 176.428 | **-19.7%** |
| the declared ten-layer fixture | Draft | cold, files first read by this process | 483.489 | 433.580 | **-10.3%** |
| the declared ten-layer fixture | Draft | cold, operating system file cache warm | 485.409 | 428.466 | **-11.7%** |
| the declared ten-layer fixture | Full | cold, files first read by this process | 579.398 | 502.716 | **-13.2%** |
| the declared ten-layer fixture | Full | cold, operating system file cache warm | 578.628 | 502.876 | **-13.1%** |

**D-47 is not closed and is not close to closed.** The best of these is still 2.50x the 41.667 ms
a 24 fps clock allows, and the declared fixture is still past ten times it. What this item bought
is a fifth of a cold frame, which is what a first item is for.

## The noise, stated rather than hidden

The warm rows moved between the two runs as well — the reference shot's warm Draft frame **up**
from 25.472 to 26.929 ms, the declared fixture's warm Draft frame **down** from 215.535 to
187.154 — and **none of that is this change**. The stage it touches reads zero in every warm row.
The clearest case is the declared fixture's warm Draft effect stack: 144.260 ms before, 121.239 ms
after, and there is no path from this diff to a stage that runs on cached pixels.

Those are two runs of one machine on two different minutes, and that is the size of this harness's
run-to-run variation. It is also the reason to read the cold percentages above as "about a fifth
to about two fifths" rather than to three decimal places. P-01 said the same thing about its own
residual and it is still true.

## The ranking moved, and document 15 is updated to say so

P-01's ranking table put the transfer function first in all eight cold rows. After this item it is
first in six of them, and in the reference shot's two Full cold rows **the encode has taken first
place**:

| Workload | Quality | Cache state | First before | First after |
|---|---|---|---|---|
| the reference shot | Full | cold, files first read by this process | transfer function — 34.0% | **encode for the page — 31.9%** |
| the reference shot | Full | cold, operating system file cache warm | transfer function — 34.4% | **encode for the page — 31.8%** |

That is not the encode getting slower. It is the row it was compared against getting smaller. P-10
is the entry that takes the encode, and it moves up.

## The proof that no pixel moved

P-03's entry is explicit: *"An item that moves one byte is reverted, not given a tolerance."* Two
things were run, and the second is the one the entry asks for.

**The suite.** `cargo test --workspace` passes with no failures. That includes
`tests/h01_whole_picture.rs`, `h02`, `h03`, `h04` and `tests/b10_full_shot.rs` — the whole-picture
byte comparisons against `Fixtures/` expected values, which are what would fail if a sample came
back different.

**The manifest**, new in `tests/p03_byte_equality.rs`. Every frame of both fixtures, at both
qualities, rendered and encoded the way `app/src/main.rs` encodes a frame for the page, reduced to
one line a frame: workload, quality, frame number, byte count, dimensions, and a hash of the
encoded bytes. 960 frames a run, 4,230,144,000 bytes of pixels.

`tests/b10_full_shot.rs` compares two exports *within one run*, which is determinism and cannot
answer a question about a code change, because nothing of its first pass survives the rebuild. A
manifest does survive it:

    # on the commit before the change
    cargo test --release --test p03_byte_equality -- --ignored
    cp target/p03_manifest.txt before.txt
    # on the change
    cargo test --release --test p03_byte_equality -- --ignored
    diff before.txt target/p03_manifest.txt

**Both runs completed and `diff` reported no difference: 960 lines out of 960 identical.** The
baseline run took 311.10 s and the run with the table 286.36 s, on the machine named above.

The hash is FNV-1a rather than a real digest, and the test says why in its own comment: the
question is whether two builds of this repository produced the same bytes, not whether someone
could forge a frame, and a cryptographic digest means a new dependency, a line in
`docs/DEPENDENCIES.md` and an entry in the licence archive for a question that does not need one.

## What is left of P-03

| Item | What it is | State |
|---|---|---|
| (c) | the sRGB decode table | **DONE**, this section |
| (a) | shared cache buffers | not started |
| (b) | parallel decode | not started |
| (d) | tiles written into the frame | not started |
| (e) | hoisted per-pixel branches | not started |
| (f) | draft tile size | not started |

Each will add a section here with its own two tables and its own manifest diff.
