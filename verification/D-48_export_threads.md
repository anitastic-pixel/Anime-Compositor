# D-48: the export frame does not depend on how many threads rendered it

Written by `d48_export_is_the_same_frame_however_many_threads_render_it` in `tests/d48_export_threads.rs`, which runs on every build. **8 of 8 checks pass.**

ADR-015 bound 3 once said the export path "gains nothing and must not", and two units in a row could not tell from that sentence whether it forbids a *cache* or forbids *threads*. **D-48 settled it on 2026-09-12**: the bound now reads that the export path neither reads nor writes the cel cache or the effect cache, because a thread count is not a cache. This page is the check that made that safe to say, and it keeps running so that it stays true.

Each row is one frame of the export path - `compose::render_frame`, the function `crate::export` calls - rendered twice: once in a rayon pool of **1 thread** and once in a pool of **24**. Both hold `CelCache::none()`, which is what export holds, so nothing here is cached and this page has no opinion on bound 3's actual subject.

The two thread counts are written into the test rather than read from the machine, so that this artifact is the same on every runner. A pool may hold more threads than the machine has cores.

| Fixture | Frame | Bytes | Digest at 1 thread | Digest at 24 threads | Result |
|---|---|---|---|---|---|
| reference shot | 0 | 8294400 | `ea45b64e05dbd758` | `ea45b64e05dbd758` | pass |
| reference shot | 14 | 8294400 | `74f74dedc0076159` | `74f74dedc0076159` | pass |
| reference shot | 100 | 8294400 | `d2adadeec0e856af` | `d2adadeec0e856af` | pass |
| reference shot | 239 | 8294400 | `fd558eff0538714a` | `fd558eff0538714a` | pass |
| declared fixture | 0 | 8294400 | `854e828f29864871` | `854e828f29864871` | pass |
| declared fixture | 14 | 8294400 | `34e7849275997629` | `34e7849275997629` | pass |
| declared fixture | 100 | 8294400 | `8e90daf37c09f3f8` | `8e90daf37c09f3f8` | pass |
| declared fixture | 239 | 8294400 | `e381ae4e7c39a8ef` | `e381ae4e7c39a8ef` | pass |

## What this settles, and what it does not

It settles one reading of bound 3, and that reading is now retired. Threads have run on the export path since B-05a, because ADR-011 renders every frame tiled across the rayon pool and export renders through the same function the viewer does. Whatever bound 3 forbids, it has never been read as forbidding that, and the frames above are identical either way.

What it does **not** do is give export a parallel decode. The parallel decode this build has is `CelCache::prewarm`, which returns early at a zero budget, and export holds a zero budget. Building one would be a unit of its own with byte-equality evidence of its own. The amended bound removes the reason not to; it does not do the work.

It settles nothing about caching, which is what bound 3 is for. A full-resolution preview matching an export in 0 of 8,294,400 samples is a property this build has, and a cache reachable from export is the obvious way to lose it. That property is guarded by `tests/p03_byte_equality.rs` and is not touched here.
