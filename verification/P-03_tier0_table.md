# P-03: the tier-zero items, before and after

Document 15's P-03 is a list of six small changes, each of which must show a before and after from
P-01's timer and prove that no pixel moved. This file is the table, and it gains a section per
item as each one lands. **Items (c) and (a) are done. (b), (d), (e) and (f) are not started.**

The order is `verification/P-01_frame_trace.md`'s measured order and not either research
document's estimate: (c), (a), (b), then (d), (e), (f).

## Machine, build and configuration

The same machine as P-01, because a before and after measured on two machines is neither.

- AMD Ryzen 9 9900X, 24 threads.
- Windows 11 Education, 10.0.26200.
- `cargo test --release`, `opt-level = 3`, debug assertions off.
- The harness is `tests/p01_frame_trace.rs`, unchanged, run as
  `cargo test --release --test p01_frame_trace -- --ignored`.

The "before" column of item (c)'s tables is the artifact committed with P-01. Each item after it
is measured against the build the one before it left, so item (a)'s "before" column is item (c)'s
"after" column. **P-01's artifact is not overwritten by this file.** It is the dated baseline
every later entry is measured against, and a baseline that moves is not one.

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


---

# Item (a): the cel the cache stopped copying

Document 15's P-03, item (a). `verification/P-01_frame_trace.md` measured two stages it named
"cache lookup and its copy" and "cache admit and its copy" at up to **55.6% of a warm frame**, and
the word in both names is *copy*: `CelCache::decoded` handed every caller its own owned
`WorkingBuffer`, 1920 x 1080 with four `f32` channels a pixel, **33,177,600 bytes**, every layer,
every frame. Almost no caller needed it. A layer with no mask and no effects only ever reads its
source.

**One line: the two cache stages fall from 14.0-50.3 ms a frame to 0.001-0.033 ms, the reference
shot's warm draft frame falls from 26.929 ms to 6.310 ms, and not one of 4,230,144,000 encoded
bytes moved.**

That warm draft row is **0.15x the 41.667 ms a 24 fps clock allows** - the first row in this
project's measured history to land comfortably inside the budget rather than over it.

## What changed

`src/cache.rs` holds `Arc<WorkingBuffer>` instead of `WorkingBuffer` and `decoded` returns a clone
of the `Arc`, which is a pointer. `LayerDraw.source`, `MatteDraw.source` and `ResolvedLayer.source`
follow it. The two places in `src/compose.rs` that actually write on a cel - the mask and the
effect stack - say so with `Arc::make_mut`, which copies exactly then and never otherwise.

Both of those sites are **guarded**, and the guards are the whole of this item rather than a
detail:

- The mask site is guarded on `mask.is_renderable()`, which is `mask::apply`'s own first line
  called here rather than restated. A mask that cannot be drawn writes nothing, and a copy taken
  in order to write nothing is precisely the cost this item removes.
- The effect site is guarded on `layer.effects.is_empty()`. An empty stack is skipped rather than
  called with nothing in it.

The second guard was not in the first version of this change, and **the first version made warm
frames worse**: the declared fixture's warm draft frame went from 187.154 ms to 240.721 ms, and
the reference shot - which has no effect instances on any layer - reported an effect stage of
**15.541 ms a warm frame**. That is impossible for a fixture with no effects, and it is what
pointed at the cause: `Arc::make_mut` was being evaluated as an argument to `apply_stack`, so the
copy happened before the empty-stack check inside it. The copy had not been removed, it had moved
from the cache stage into the effect stage. Both guards are in the measurement below.

## The stages this item aimed at

Both cache stages, p50, in milliseconds. "Before" is the build with item (c) in it, which is the
commit this one sits on; "after" is this change. Same machine, same harness, same session.

| Workload | Quality | Cache state | Lookup before | Admit before | Lookup after | Admit after |
|---|---|---|---|---|---|---|
| the reference shot | Draft | cold, files first read | 0.001 | 15.761 | 0.001 | **0.002** |
| the reference shot | Draft | cold, OS file cache warm | 0.001 | 16.385 | 0.001 | **0.003** |
| the reference shot | Draft | everything warm | 14.543 | 0.000 | **0.002** | 0.000 |
| the reference shot | Full | cold, files first read | 0.001 | 17.057 | 0.001 | **0.003** |
| the reference shot | Full | cold, OS file cache warm | 0.001 | 16.427 | 0.001 | **0.003** |
| the reference shot | Full | everything warm | 14.003 | 0.000 | **0.003** | 0.000 |
| the declared ten-layer fixture | Draft | cold, files first read | 0.004 | 50.309 | 0.005 | **0.009** |
| the declared ten-layer fixture | Draft | cold, OS file cache warm | 0.004 | 49.166 | 0.005 | **0.010** |
| the declared ten-layer fixture | Draft | everything warm | 40.202 | 0.000 | **0.021** | 0.000 |
| the declared ten-layer fixture | Full | cold, files first read | 0.004 | 48.705 | 0.004 | **0.009** |
| the declared ten-layer fixture | Full | cold, OS file cache warm | 0.004 | 47.782 | 0.004 | **0.009** |
| the declared ten-layer fixture | Full | everything warm | 37.788 | 0.000 | **0.023** | 0.000 |

The two stages trade places by cache state and always did: a cold frame pays the copy on admission
and a warm frame pays it on lookup. After this item neither of them pays it, and the stage that
P-01 ranked at 54.4% of the reference shot's warm draft frame reads 0.002 ms.

Unlike item (c), **this item helps warm frames most**, because a warm frame is exactly the one
whose whole remaining cost was handing out copies of buffers it already had.

## The frame someone actually waits for

| Workload | Quality | Cache state | Frame before (p50 ms) | Frame after (p50 ms) | Change |
|---|---|---|---|---|---|
| the reference shot | Draft | cold, files first read | 104.167 | 89.654 | **-13.9%** |
| the reference shot | Draft | cold, OS file cache warm | 109.762 | 92.382 | **-15.8%** |
| the reference shot | Draft | everything warm | 26.929 | **6.310** | **-76.6%** |
| the reference shot | Full | cold, files first read | 182.745 | 167.887 | **-8.1%** |
| the reference shot | Full | cold, OS file cache warm | 176.428 | 169.711 | **-3.8%** |
| the reference shot | Full | everything warm | 94.089 | 79.575 | **-15.4%** |
| the declared ten-layer fixture | Draft | cold, files first read | 433.580 | 398.554 | **-8.1%** |
| the declared ten-layer fixture | Draft | cold, OS file cache warm | 428.466 | 392.033 | **-8.5%** |
| the declared ten-layer fixture | Draft | everything warm | 187.154 | 164.417 | **-12.1%** |
| the declared ten-layer fixture | Full | cold, files first read | 502.716 | 454.045 | **-9.7%** |
| the declared ten-layer fixture | Full | cold, OS file cache warm | 502.876 | 463.122 | **-7.9%** |
| the declared ten-layer fixture | Full | everything warm | 265.346 | 238.990 | **-9.9%** |

Every one of the twelve rows improved. Against `verification/P-01_frame_trace.md`'s original
baseline - item (c) and item (a) together - the reference shot's warm draft frame has gone from
25.472 ms to 6.310 ms and its cold draft frame from 129.928 ms to 89.654 ms.

**D-47 stays open.** Eleven of these twelve rows are still over the 41.667 ms budget, the worst of
them at 11.11x, and one row inside it on a four-layer shot is not the decision D-47 asks for.

## Where the copy went when it did not disappear

The declared fixture has three effect instances, so three of its ten layers still take a copy. It
is now taken inside `apply_stack`, which is inside the effect stage's timer, and the effect stage
rose accordingly: on the warm draft row from 121.239 ms to 149.734 ms, and on the warm full row
from 128.982 ms to 148.676 ms.

That rise is real and this file does not net it away. It is also smaller than what left the cache
stage - the warm draft row lost 40.181 ms of cache stage and gained 28.495 ms of effect stage, and
the frame fell 22.737 ms - and it is measured against a stage whose run-to-run spread on this
harness is tens of milliseconds (its own p95 on that row is 173.726 ms). The claim this section
makes is the frame time, which is the only figure with a before and after on every row.

The reference shot, which has no effects at all, is the clean case: its cache stages went to zero
and nothing else went up.

## The proof that no pixel moved

The same harness item (c) introduced, run on the commit before this change and on this change:

    cargo test --release --test p03_byte_equality -- --ignored

**Both runs completed and `diff` reported no difference: 960 lines out of 960 identical, over
4,230,144,000 bytes of pixels.** The run took 264.70 s.

The manifest was also compared against the one from *before* item (c), the run
`verification/P-01_frame_trace.md`'s commit produces, and is identical to that as well. So the two
P-03 items landed so far have between them changed the transfer function and the ownership of
every decoded cel, and the three manifests - before (c), after (c), after (a) - are the same file.

`cargo test --workspace` passes as well, which is where the H-series whole-picture comparisons and
`tests/b10_full_shot.rs` live. One test changed with this item and it was made **stronger**:
`tests/b06_mask.rs`'s `cache_isolation` now takes the `Arc::make_mut` copy explicitly before it
writes, which is what `src/compose.rs` does, so the test proves the copy happens where the code
says it does rather than assuming an owned buffer arrived.

## One thing fixed on the way past

`src/perf.rs`'s `records_only_while_enabled` timed a call it expected to be measurable and
asserted the result was not zero. Windows steps its performance counter in hundreds of nanoseconds
and that call takes a few, so the test could fail on a machine that was quick enough - an
intermittent failure already merged with P-01, not one this item introduced. It now spins until
the clock has actually moved.

## What is left of P-03

| Item | What it is | State |
|---|---|---|
| (c) | the sRGB decode table | **DONE**, item (c) above |
| (a) | shared cache buffers | **DONE**, item (a) above |
| (b) | parallel decode | not started |
| (d) | tiles written into the frame | not started |
| (e) | hoisted per-pixel branches | not started |
| (f) | draft tile size | not started |

Each will add a section here with its own two tables and its own manifest diff. Each is
measured against the build the one before it left, so the "before" column of the next section
is the "after" column of this one, and `verification/P-01_frame_trace.md` stays the dated
baseline none of them overwrite.
