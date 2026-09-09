# P-03: the tier-zero items, before and after

Document 15's P-03 is a list of six small changes, each of which must show a before and after from
P-01's timer and prove that no pixel moved. This file is the table, and it gains a section per
item as each one lands. **Items (c), (a), (b), (d) and (f) are done. (e) was built, measured and reverted. P-03 is
finished.**

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

---

# Item (b): the frame's cels decoded together

Document 15's P-03, item (b). A frame's cels are separate files and nothing about decoding one
depends on another, but they were decoded one at a time because the `&mut CelCache` threaded
through `plan_frame_cached`'s layer loop made that loop the only place a decode could happen.
After items (c) and (a), the read, the byte-to-float pass and the transfer function are what a
cold frame *is*: 77.5 ms of an 89.7 ms reference-shot draft frame.

**One line: opening a project and pressing play costs 39.3% less a frame on the reference shot and
34.4% less on the declared fixture, the first pass through the shot goes from 1.04x the 24 fps
budget to 0.63x, and the cache's hit, miss and eviction counts are unchanged to the digit.**

## What changed

`CelCache::prewarm` takes the list of cels a frame is about to ask for, drops the ones it already
holds, deduplicates the rest — a matte and the layer that uses it name the same file and decode
once — and decodes what is left in one rayon fan-out. The results are staged in a `pending` list.
Nothing is admitted there: `decoded` collects a staged cel, still counts it as a **miss**, because
it was decoded, and still admits it through `store` in composition order. `plan_frame_cached`
builds the list from exactly the layers the loop below it will resolve.

**The list is not a guess, and being wrong about it costs nothing.** A cel `prewarm` could not
work out, or failed to decode, is simply absent, and `resolve_layer` decodes it serially a moment
later and raises the diagnostic against the right layer the way it always has. That is why
P-03(b)'s requirement to collect a `FrameLog` per layer and merge it in composition order is not
implemented: **no diagnostic is ever raised on a worker thread**, so there is no order to restore.
The requirement was written against a design that decoded inside the layer loop in parallel; this
one decodes before it, and the requirement dissolves rather than being skipped.

**It does nothing without a budget.** `CelCache::none` is what export and every non-preview caller
hold, and a cache that may not keep what it decodes has nothing to decode ahead for. ADR-015 bound
3 — "export neither reads the cache nor writes it" — is untouched by this item, and deliberately:
letting the fan-out run on the no-budget path would speed up export too, and that is a change to
an accepted decision rather than a step in making a number look better.

## Why this item has its own artifact

`verification/P-01_frame_trace.md` **cannot measure this change**, and reads 0.000 ms for it in
all twelve of its rows. That is correct rather than disappointing: P-01's two cold rows hold
`CelCache::none`, which this item skips, and its warm row already holds every cel, so there is
nothing left to decode. The state P-01 has no row for is the one a person actually sits through —
a real budget, and every layer of every frame a miss, which is what opening a project and pressing
play is.

So item (b) adds a row rather than borrowing one: `verification/P-03b_first_playthrough.md`, from
`tests/p01_frame_trace.rs`'s `p03b_first_playthrough`, which reuses P-01's harness, machine,
frames and stage timers and writes its own file. P-01's artifact is not touched, because it is the
dated baseline the other five items are measured against and a baseline that grows a column is not
one. The budget is D-40's 1 GiB, the viewer's own, and not P-01's 6 GiB: this row is what the
window does, so it is measured with what the window is configured with, evictions and all.

## A first playthrough, before and after

| Workload | Quality | Frame before (p50 ms) | Frame after (p50 ms) | Change | Against 24 fps |
|---|---|---|---|---|---|
| the reference shot | Draft | 43.242 | **26.262** | **-39.3%** | 1.04x to **0.63x** |
| the reference shot | Full | 107.101 | 83.750 | **-21.8%** | 2.57x to 2.01x |
| the declared ten-layer fixture | Draft | 287.434 | 188.541 | **-34.4%** | 6.90x to 4.52x |
| the declared ten-layer fixture | Full | 349.002 | 257.287 | **-26.3%** | 8.38x to 6.17x |

And the decode itself, which is the three stages this item moved onto several threads. "Before" is
their sum on the calling thread; "after" is the wall-clock of the fan-out that replaced them.

| Workload | Quality | Cels a frame | Serial decode (p50 ms) | Parallel decode (p50 ms) | Change |
|---|---|---|---|---|---|
| the reference shot | Draft | 4 | 35.897 | 19.419 | **-45.9%** |
| the reference shot | Full | 4 | 36.357 | 19.638 | **-46.0%** |
| the declared ten-layer fixture | Draft | 10 | 116.416 | 30.728 | **-73.6%** |
| the declared ten-layer fixture | Full | 10 | 116.689 | 30.071 | **-74.4%** |

Four cels across twenty-four threads is a speed-up of 1.85x and ten cels is 3.79x, not 4x and not
10x. Neither number is a disappointment and the artifact does not present them as one: a frame has
as many cels as it has layers, the files are read from one disk, and the widest thread is what the
frame waits for. **This item is bounded by the layer count, and it says so here rather than being
quoted as a thread count.**

## The counts that did not move

| | Before | After |
|---|---|---|
| the reference shot, either quality | 31 hits, 47 misses, 15 evictions | 31 hits, 47 misses, 15 evictions |
| the declared fixture, either quality | 86 hits, 148 misses, 116 evictions | 86 hits, 148 misses, 116 evictions |

This is the check that matters more than the timings. A cel decoded ahead is still counted as the
miss it is, and is still admitted in the order the layer loop asks for it, so the budget, the
eviction order and the numbers `verification/B-08b_cache_table.md` reports are what they were when
every decode was serial. That table is regenerated by `tests/b08b_cache.rs` on every run of the
suite and **did not change by one digit**, which is the same statement made by a test the owner
already has.

## The stage table stayed disjoint, which took work

`src/perf.rs` opens with a promise: the stages are disjoint, so the table can be summed and
subtracted from a frame time to leave a residual, and "a nested pair would double-count and the
residual would go negative, which is a bug that reads as a result". Three stages now run on
however many threads rayon hands them, while the frame pays only the wall-clock of the widest.
Left alone, they would have added their *core* time to counters compared against wall-clock, the
table would have summed to more than the frame, and `tests/p01_frame_trace.rs`'s own assertion
would have failed.

So the fan-out reports as one new stage, "decode this frame's cels in parallel", wall-clock from
fan-out to join, exactly as `TileLoop` already did; and `perf::untimed` suppresses the three
per-cel timers on whichever worker thread is inside it. A cel decoded anywhere else — an export, a
caller with no cache, a request the fan-out could not see coming — reports in the three stages as
before. The p95 columns in the after run show that happening: the reference shot's read stage has
a p50 of 0.000 ms and a p95 of 2.862 ms, which is the frames where a cel was not decoded ahead.

## The proof that no pixel moved

Two things, because this item needed a second one.

**The manifest**, as items (c) and (a) ran it: 960 frames, both fixtures, both qualities, one line
a frame, compared against the run from before this change. **960 of 960 lines identical**, over
4,230,144,000 bytes, and identical to the run item (a) left, so the three items so far share one
unbroken chain of manifests.

**A budgeted render against an unbudgeted one, every frame.** The manifest renders through
`CelCache::none`, which this item skips, so it could not on its own see anything this item did.
`tests/p03_byte_equality.rs` now renders each frame a second time through one cache with the
viewer's D-40 budget and compares the two encodes byte for byte. That is `tests/b08b_cache.rs`'s
rule — which already covers the reference shot at several budgets, and whose table did not move —
extended to the declared ten-layer fixture, and P-03(b) is the reason it is needed: with a budget
and without one are no longer the same schedule of the same work. **Every frame of both fixtures
at both qualities passed**, 960 pairs, in the same run that produced the manifest above.

---

# Item (d): the tile that stopped being copied into the frame

Document 15's P-03, item (d). A tile allocated a buffer of its own, accumulated the whole layer
stack into it, and that buffer was then copied — row by row, on one thread — into a separately
allocated, separately zeroed 33 MB frame. Three passes over 33 MB a frame, for a result the tiles
had already finished computing.

Document 15 predicted this item would not show a factor: P-01 measured the tile loop at 1.3% to
7.8% and the assembly at 0.6% to 11.7%, and the entry says the last three items are "kept because
they are bit-exact and small, not because P-01 argues for them". **The assembly half of that
prediction was low. Every one of P-01's twelve rows improved and the best improved 11.6%**, which
is what happens when a stage measured at 15.1% of a warm full-resolution frame becomes 0.2% of it.

## What changed

The frame is allocated once, at zero, and then carved into one disjoint set of row slices per
tile — **before any thread starts**. That is exactly the property `tests/b05a_transform.rs` proves
for ADR-011 and the reason the old code was safe; it is now enforced by the borrow checker rather
than by an index computed after the fact, because two tiles cannot be handed the same `&mut`.

Each tile then accumulates into the frame's own pixels. It used to accumulate into a private
buffer initialised to zero; the frame arrives at the same zero, so every pixel sees the same
layers in the same order doing the same arithmetic. What stops happening is one 33 MB allocation,
one 33 MB zero-fill and one 33 MB copy a frame.

The tile decomposition is untouched. `tiles()` is not modified, `DEFAULT_TILE_SIZE` is not
modified, and `verification/B-07_effects_table.md` still renders one frame at six tile sizes and
still gets six identical results.

`Stage::FrameAssembly` was not removed, because the carve-up is what is left of assembly and it is
honest to keep measuring it. It is serial on purpose: it hands out borrows and touches no pixel.

## Where the copy went, since some of it did not disappear

| Workload | Quality | Cache state | Assembly before | Assembly after |
|---|---|---|---|---|
| the reference shot | Full | cold, first read | 11.595 | **0.113** |
| the reference shot | Full | cold, OS cache warm | 11.792 | **0.107** |
| the reference shot | Full | everything warm | 11.684 | **0.122** |
| the reference shot | Draft | everything warm | 0.699 | **0.010** |
| the declared fixture | Full | cold, first read | 9.119 | **0.120** |
| the declared fixture | Full | cold, OS cache warm | 8.642 | **0.075** |
| the declared fixture | Full | everything warm | 9.318 | **0.079** |
| the declared fixture | Draft | everything warm | 0.575 | **0.007** |

But the tile loop **rose**, and this artifact reports that rather than netting it away:

| Workload | Quality | Cache state | Tile loop before | Tile loop after |
|---|---|---|---|---|
| the reference shot | Full | everything warm | 6.437 | 7.352 |
| the reference shot | Full | cold, OS cache warm | 6.474 | 7.696 |
| the declared fixture | Full | everything warm | 16.756 | 18.599 |
| the declared fixture | Full | cold, OS cache warm | 17.553 | 19.624 |
| the reference shot | Draft | everything warm | 1.745 | 1.732 |
| the declared fixture | Draft | everything warm | 4.625 | 4.639 |

A tile writing into a small private buffer writes into memory that fits in a core's cache. A tile
writing into its rows of a 33 MB frame writes into memory that does not, and pays for it, at full
resolution, about 1 to 2 ms a frame. **Some of the copy therefore moved into the tile loop rather
than disappearing.** The draft rows, where a tile's rows are small either way, do not show it.

The trade is still strongly worth making — 11.7 ms of serial copy against 1.9 ms spread over
twenty-four threads — but the honest statement is that this item removed most of a stage, not all
of it, and made a second stage slightly more expensive.

## The frame, before and after

All twelve of P-01's rows, same machine, build and twenty frames.

| Workload | Quality | Cache state | Before (p50 ms) | After (p50 ms) | Change |
|---|---|---|---|---|---|
| the reference shot | Draft | cold, first read | 87.915 | 86.530 | -1.6% |
| the reference shot | Draft | cold, OS cache warm | 88.099 | 87.642 | -0.5% |
| the reference shot | Draft | everything warm | 6.580 | **5.822** | **-11.5%** |
| the reference shot | Full | cold, first read | 161.386 | 151.234 | -6.3% |
| the reference shot | Full | cold, OS cache warm | 161.522 | 151.536 | -6.2% |
| the reference shot | Full | everything warm | 74.535 | **65.890** | **-11.6%** |
| the declared fixture | Draft | cold, first read | 374.902 | 372.493 | -0.6% |
| the declared fixture | Draft | cold, OS cache warm | 370.175 | 364.841 | -1.4% |
| the declared fixture | Draft | everything warm | 145.753 | 140.620 | -3.5% |
| the declared fixture | Full | cold, first read | 440.878 | 434.517 | -1.4% |
| the declared fixture | Full | cold, OS cache warm | 442.634 | 438.049 | -1.0% |
| the declared fixture | Full | everything warm | 216.688 | 213.914 | -1.3% |

And item (b)'s first-playthrough row, which is the one a person sits through:

| Workload | Quality | Before (p50 ms) | After (p50 ms) | Change | Against 24 fps |
|---|---|---|---|---|---|
| the reference shot | Draft | 26.262 | **25.426** | -3.2% | **0.61x** |
| the reference shot | Full | 83.750 | **74.120** | **-11.5%** | 1.78x |
| the declared fixture | Draft | 188.541 | 181.441 | -3.8% | 4.35x |
| the declared fixture | Full | 257.287 | 244.059 | -5.1% | 5.86x |

**Sixteen rows measured, sixteen improved.** The reference shot's warm draft frame stays the one
row inside the budget, now at 0.61x.

## The proof that no pixel moved

`tests/p03_byte_equality.rs`, unchanged from item (b): 960 frames across both fixtures at both
qualities, each rendered with no cache and again with the viewer's own budget, every pair equal,
and the manifest of all 960 lines **identical to the one item (b) left**, over 4,230,144,000
bytes. `verification/B-07_effects_table.md`, the six-tile-size table this item's safety argument
rests on, did not move, and neither did any other artifact under `verification/` — the whole suite
of 31 test binaries passed with `git diff --exit-code -- verification/` clean.

---

# Item (e): the hoisted branches, built twice, measured, and reverted

Document 15's P-03, item (e). Three things are fixed for a whole layer — its blend mode, whether
its opacity is worth multiplying by, and whether it has a matte — and `render_tile` tested all
three at every pixel of every tile. The entry proposed hoisting them out of the pixel loop.

**It was built, measured, and it made the tile loop slower. It is reverted, and this section is
what it produced instead.** No file under `src/` differs from what item (d) left.

## Why it looked worth doing

For the three separable blend modes, the `match` on the mode ran not once per pixel but **once per
colour channel inside a loop of three**, so a multiply-blended full-resolution layer paid it
6,220,800 times a frame. The opacity and matte tests are only once a pixel, but they are tests of
values that cannot change for the duration of the loop. Nothing about the proposal was unsound.

## What was built

Two versions, because the first one's result had an obvious suspect.

**First: a function pointer.** `composite::blender(mode)` returned the mode's own function, chosen
once per layer, and `render::draw::<OPACITY, MATTE>` compiled the other two tests away as const
generics. This was **worse in eleven of P-01's twelve rows** — the tile loop rose 6% to 15%, and
the declared fixture's warm full frame rose 12.4%. The suspect was the pointer: an indirect call
at every pixel is a call the optimiser cannot see into, where the `match` it replaced was inlined.

**Second: full monomorphisation.** `composite::blend_fixed::<MODE>` with the mode as a const
generic and the renderer instantiating `draw::<BLEND, OPACITY, MATTE>` — sixteen compiled copies
of the pixel loop, every blend call direct and inlinable, no pointer anywhere. This recovered the
reference shot. It did not recover the declared fixture.

## What it measured

The frame times were too noisy to decide on, so the decision was made on the stage the item
targets. Two paired runs of item (b)'s first-playthrough harness per build, same machine, same
twenty frames, alternating so that neither build got the quiet half of the run.

| Workload | Quality | Tile loop, item (d) | Tile loop, item (e) | Change |
|---|---|---|---|---|
| the reference shot | Draft | 1.881, 2.046 | 1.918, 1.968 | -1% (a wash) |
| the reference shot | Full | 7.766, 7.724 | 7.862, 7.839 | **+1.3%** |
| the declared ten-layer fixture | Draft | 4.541, 4.504 | 4.779, 4.844 | **+6.4%** |
| the declared ten-layer fixture | Full | 18.428, 17.656 | 19.276, 19.021 | **+6.1%** |

**Seven of the eight paired measurements are slower with the hoist, and none is faster.** The
frame-time columns overlap between builds and are not quoted as a result here, which is the point
of measuring the stage instead: on the declared fixture the two runs of one build differ by more
than the two builds differ, so a frame-time table would have supported whichever answer was
wanted.

## Why it lost

The branches were never the cost. A test on a value that does not change across a loop is
predicted correctly by the hardware every time after the first, which makes it approximately free,
and the compiler was already specialising what it could. What the hoist added was real: sixteen
copies of the pixel loop, and the declared fixture — ten layers, several modes, several with
mattes — walks through more of those copies per frame than the reference shot's four layers do.
The instruction cache pays for that, and the two fixtures' results differ in exactly the direction
that explanation predicts.

## What this is worth

Document 15 said of (d), (e) and (f) that they are "kept because they are bit-exact and small, not
because P-01 argues for them", and P-01's ranking put the tile loop at 1.3% to 7.8% of a frame.
(d) then found a real saving in the stage next to it. (e) is the entry where the caution was
right.

It cost two builds and four measurement runs, and it closes a question rather than leaving it as a
plausible idea for someone to have again. The rejection is the deliverable: **the per-pixel
branches in `render_tile` are not worth hoisting, they have been hoisted twice to find that out,
and the numbers above are why.** If a later P-01 run shows the tile loop grown into a large
fraction of the frame — the same condition document 15 already sets for reopening SIMD — this can
be re-measured against that build, and the second version is the one to rebuild.

---

# Item (f): the draft frame cut into more pieces than the machine has threads

Document 15's P-03, item (f). `compose::DEFAULT_TILE_SIZE` is 128 pixels and it was measured —
`verification/B-05a_scaling_table.md` renders one frame at four tile sizes and six thread counts
and 128 wins. It was measured on a **1920x1080** frame. A draft preview is a quarter of that in
each direction, and 128 pixels cuts 480x270 into four columns of three: **twelve pieces of work
for twenty-four hardware threads**, so half the machine has nothing to do however fast each piece
is.

**`compose::DRAFT_TILE_SIZE` is 48 pixels, sixty tiles, and it is used only when the preview
quality is `Draft`. `DEFAULT_TILE_SIZE` is untouched, so an export still renders in 128px tiles.
The draft tile loop falls 6.3% to 16.2%, no frame time moves further than this machine's noise,
and the 960-line manifest is identical to the one item (d) left.**

## The measurement this constant is

Document 21: "Tile size is a tunable measured on the reference machine, not a constant chosen in
advance." So 48 is not chosen; it is the row `verification/P-03f_draft_tile_size.md` picks.
`tests/p03f_draft_tile_size.rs` builds the draft plans for twenty frames of each fixture once,
then times **`render::render` alone** — the decode, the transfer function and the encode are
outside the span on purpose, because this item moves none of them and at a first playthrough they
are large enough to hide it entirely — at seven sizes, sweeping the whole list twice so that no
size is measured twice in a row.

| Tile | Tiles of a 480x270 frame | Reference shot, two sweeps | Declared fixture, two sweeps |
|---|---|---|---|
| 128px | 12 | — | — |
| 96px | 15 | -10.7%, -5.5% | -11.3%, -16.6% |
| 64px | 40 | -11.2%, -2.3% | -10.1%, -17.6% |
| **48px** | **60** | **-16.8%, -6.1%** | **-13.9%, -24.4%** |
| 32px | 135 | -17.3%, +7.4% | -12.2%, -19.5% |
| 24px | 240 | -16.0%, +3.8% | -13.2%, -22.8% |
| 16px | 510 | -7.7%, +4.1% | -18.9%, -18.0% |

**128px is the worst size in all four columns.** Below 48 the table stops improving and starts
disagreeing with itself — the reference shot's second sweep is *slower* at 32, 24 and 16 than at
128 — which is per-tile overhead beginning to cost more than the extra parallelism buys. 48 is
best or within a sweep's noise of best in every column, so it is the row the constant is set to,
and the sizes below it are in the table to show why it is not one of them.

That is 560 renders — seven sizes, twenty frames, two sweeps, two fixtures — and **every one of
them is compared against the 128px render of the same frame byte for byte**. A tile size is a
schedule, never a picture; that is ADR-011, and the artifact re-checks it rather than citing it.

## What it did to a frame

All twelve of P-01's rows, same machine, build and twenty frames, measured against the build item
(d) left. Item (e) changed nothing under `src/`, so that is also the build before this one.

| Workload | Quality | Cache state | Tile loop before | Tile loop after | Change |
|---|---|---|---|---|---|
| the reference shot | Draft | cold, first read | 2.016 | 2.015 | -0.0% |
| the reference shot | Draft | cold, OS cache warm | 1.958 | 1.830 | **-6.5%** |
| the reference shot | Draft | everything warm | 1.732 | 1.585 | **-8.5%** |
| the declared fixture | Draft | cold, first read | 4.748 | 4.274 | **-10.0%** |
| the declared fixture | Draft | cold, OS cache warm | 4.844 | 4.325 | **-10.7%** |
| the declared fixture | Draft | everything warm | 4.639 | 3.886 | **-16.2%** |

And item (b)'s first-playthrough harness, which is the row a person actually sits through:

| Workload | Quality | Tile loop before | Tile loop after | Change |
|---|---|---|---|---|
| the reference shot | Draft | 1.881 | 1.763 | **-6.3%** |
| the declared fixture | Draft | 4.541 | 3.982 | **-12.3%** |

**The six full-resolution rows are the control.** A full-resolution preview still renders in
`DEFAULT_TILE_SIZE` tiles, so nothing about it changed, and its tile loop reads -3.1%, -2.1%,
-0.6%, -0.4%, +0.0% and -5.2% across the two harnesses — which is what this machine's noise looks
like on that stage, measured on a build where the code could not have moved. Every draft row is
outside that band and every one is in the same direction.

## What it did not do

**It did not move a frame time out of noise, and this section says so rather than quoting one.**
The tile loop is 1.6 to 4.8 ms of a draft frame that is 5.6 ms warm and 372 ms cold, so a sixth of
it is between 0.1 and 0.8 ms. The frame column across the twelve P-01 rows reads between -4.6% and
+3.3% with no relation to whether the row is draft or full, and the declared fixture's warm draft
frame — the row whose tile loop fell the furthest, 16.2% — reads **+2.6%**. That is the same
lesson item (e) was decided on: where the change is smaller than the run-to-run spread, the stage
is the measurement and the frame is not.

So the honest statement of this item is: it is a bit-exact change that makes the draft tile loop
about a tenth cheaper, costs nothing, and is not visible in a frame time. Document 15 said of (d),
(e) and (f) that they are "kept because they are bit-exact and small, not because P-01 argues for
them", and of the three, (d) found a real saving, (e) was a loss, and (f) is exactly the size the
entry predicted.

## The proof that no pixel moved

`tests/p03_byte_equality.rs`: 960 frames across both fixtures at both qualities, each rendered
with no cache and again with the viewer's own budget, every pair equal, and the manifest of all
960 lines **identical to the one item (d) left**, over 4,230,144,000 bytes. This is the strongest
run of that test so far, because half of those frames — every draft frame — were rendered in
48-pixel tiles and compared against a manifest produced in 128-pixel tiles. The whole test suite
passed with `git diff --exit-code -- verification/` clean.

One consequence worth stating plainly, because it looks like an inconsistency. Five artifacts
record "Tile size: `compose::DEFAULT_TILE_SIZE`" and were measured before this change:
`verification/P-01_frame_trace.md`, `verification/B-08b_cache_budget.md`,
`verification/B-08_preview_latency.md`, `verification/T-06_declared_fixture.md` and
`verification/T-06_performance_envelope.md`. They are **not** regenerated here — they are dated records of runs that really did use 128-pixel tiles
throughout, and a dated artifact that is rewritten to match today's build is not a record. Each of
their test sources now names the constant that row actually renders in, so the next deliberate run
of any of them will say what that run did.

## What is left of P-03

| Item | What it is | State |
|---|---|---|
| (c) | the sRGB decode table | **DONE**, item (c) above |
| (a) | shared cache buffers | **DONE**, item (a) above |
| (b) | parallel decode | **DONE**, item (b) above |
| (d) | tiles written into the frame | **DONE**, item (d) above |
| (e) | hoisted per-pixel branches | **CUT**, item (e) above: measured slower, twice |
| (f) | draft tile size | **DONE**, item (f) above |

Each section above has its own tables and its own manifest comparison, and each was measured
against the build the one before it left, so the "before" column of a section is the "after"
column of the one above it. `verification/P-01_frame_trace.md` is the dated baseline none of them
overwrote.

**Nothing in P-03 is outstanding.** Five items landed, one was cut with its numbers, and the
manifest that opened with item (c) closes with item (f) at the same 960 identical lines it started
with: 4,230,144,000 bytes of picture that six changes to how the picture is computed did not
move.
