# P-02: how much of the stack composites the same way twice

Document 15's P-02. It counts, on paper, and runs no compositor: `verification/derive_p02_prefix.py` reads `Fixtures/reference_shot/exposure_sheet.json` and `verification/B-08a_project.json` and reports what a cache of partial composites would remove. Re-run it with

    python verification/derive_p02_prefix.py

If this file and that script ever disagree, the script is right and this file was edited by hand.

**The question P-02 was asked.** Document 32 section 6.2 argues that a run of frames whose lower layers hold the same drawings, transforms, effects and mattes composites those layers identically every frame, so a deterministic renderer can cache that partial composite on a hash of its inputs. Document 33 agreed it was promising *if the stable prefix dominates cost*, and said neither document knew whether it does. This is the number.

**The answer, in one line: within the memory D-40 allows, a prefix cache removes 24.9% of the reference shot's layer composites and 10.0% of the declared fixture's, and in both cases it removes exactly one layer — the static background, which is the bottom of the stack and nothing more. Everything deeper needs 4 GiB to 40 GiB.**

## What "stable" means here, and the precondition that makes the count exact

A prefix of the stack composites identically on two frames when **every** input to those layers is the same: the drawing each shows, the transform, the opacity, the blend mode, the mask, every effect parameter, and, for a matted layer, the matte layer's pixels as well. That is document 27 line 29's list.

Neither fixture animates anything. The script asserts this rather than assuming it — it walks the whole project document and fails if it finds one non-empty `keyframes` array — because with a keyframe present the drawing alone would no longer decide stability and the count would silently come out too high. Both fixtures pass the assertion, so **the only input that moves frame to frame is which drawing each layer shows**, and the count below is exact rather than approximate.

Layer order is bottom of the stack first, which is `src/compose.rs` line 138's order.

## Why the stable prefix is one layer deep, in both fixtures

| | Layer, bottom first | Exposes | Distinct states | Changes on how many of 239 transitions |
|---|---|---|---|---|
| reference shot | 1. layer1 | layer1 | 1 | **0** |
| | 2. layer2 | layer2 | 24 | **239, every one** |
| | 3. layer3 | layer3 | 12 | 119 |
| | 4. layer4 | layer4 | 20 | 79 |
| declared fixture | 1. copy1 | layer1 | 1 | **0** |
| | 2. copy2 | layer2 | 24 | **239, every one** |
| | 3. copy3 | layer3 | 12 | 119 |
| | 4. copy4 | layer4 | 20 | 79 |
| | 5. copy5 | layer2 + matte | 180 | 239 |
| | 6. copy6 | layer3 | 12 | 119 |
| | 7. copy7 | layer4 | 20 | 79 |
| | 8. copy8 | layer2 + matte | 180 | 239 |
| | 9. copy9 | layer3 | 12 | 119 |
| | 10. copy10 | layer4 | 20 | 79 |

`copy5` and `copy8` have 180 states rather than 24 because each is matted by a layer that moves on threes, and a matted layer's pixels depend on the matte's, so their combined state changes when either does.

The consequence is arithmetic. **The layer that moves on ones sits second from the bottom in both fixtures.** So between any two consecutive frames the stable prefix is one layer deep, on all 239 transitions of both shots, with no exceptions:

| Prefix depth between one frame and the next | Reference shot | Declared fixture |
|---|---|---|
| 1 of the stack | 239 transitions, 100.0% | 239 transitions, 100.0% |

The "run of frames whose lower layers hold the same drawings" that document 32 section 6.2 describes does not occur in either fixture, because the layer on ones is at the bottom of the stack rather than the top. That is a property of how the reference shot was drawn (document 22), not of the code.

## What a cache keyed on the prefix's inputs would remove anyway

Consecutive frames are not the only source of reuse. The exposures are periodic — 24, 24 and 60 frames — so frame 120 shows exactly what frame 0 showed, and a cache keyed on the *inputs* rather than on adjacency finds that. The simulation walks one 240-frame playthrough, and for each frame finds the deepest prefix whose exact input tuple this playthrough has composited before.

### The reference shot, 4 layers, 960 layer composites in a playthrough

| Prefix composites held | Memory those buffers cost | Layer composites removed | Share |
|---|---|---|---|
| 1 | 0.03 GiB | 0 | 0.0% |
| 4 | 0.12 GiB | 239 | 24.9% |
| 8 | 0.25 GiB | 239 | 24.9% |
| 16 | 0.49 GiB | 239 | 24.9% |
| **32 (the 1 GiB D-40 sets)** | **0.99 GiB** | **239** | **24.9%** |
| 64 | 1.98 GiB | 239 | 24.9% |
| 128 | 3.96 GiB | 675 | 70.3% |
| 256 | 7.91 GiB | 731 | 76.1% |
| unbounded (229 held) | 7.08 GiB | 731 | 76.1% |

### The declared ten-layer fixture, 2,400 layer composites in a playthrough

| Prefix composites held | Memory those buffers cost | Layer composites removed | Share |
|---|---|---|---|
| 1 | 0.03 GiB | 0 | 0.0% |
| 8 | 0.25 GiB | 0 | 0.0% |
| 16 | 0.49 GiB | 239 | 10.0% |
| **32 (the 1 GiB D-40 sets)** | **0.99 GiB** | **239** | **10.0%** |
| 64 | 1.98 GiB | 239 | 10.0% |
| 128 | 3.96 GiB | 239 | 10.0% |
| 256 | 7.91 GiB | 671 | 28.0% |
| 512 | 15.82 GiB | 692 | 28.8% |
| unbounded (1,309 held) | 40.45 GiB | 1,091 | 45.5% |

One prefix composite is a whole composition-sized working buffer, 1920 × 1080 × four f32 channels = **33,177,600 bytes**, which is the same 33 MB a decoded cel costs. A prefix cache does not add memory to the budget; it competes with the cel cache for the 1 GiB D-40 set, twice, on the same evidence.

**The 239 removed composites in every affordable row are the same 239: the static background layer, depth 1, on every frame after the first.** The shape of both tables — flat at depth 1 until the cache can hold a whole period of the shot, then a jump — is the honest reading. Prefix reuse in these fixtures is not adjacency, it is the loop coming round again, and capturing the loop costs 4 GiB on a four-layer shot and 40 GiB on a ten-layer one.

## What this decides

**P-08, the stable-prefix composite cache, dies here, and document 15 says so.** Its exit condition was "a number. If it is small, P-08 dies here and this section says so." The number is 10.0% of layer composites on the fixture that misses the budget by 13.9×, and the one layer it removes is a static background whose decoded pixels `src/cache.rs` already holds. Against that: a second 33 MB-per-entry cache, a key that must include every input document 27 line 29 names or it is wrong in a way no test would catch, and a competition for the same 1 GiB.

Two things this does **not** say.

- **It does not say prefix caching is a bad idea in general.** It says the two fixtures this project verifies against put the fastest-moving layer at the bottom of the stack, which defeats it. A shot drawn with a moving foreground over a still background — which is the usual case, and is what document 32 had in mind — would rank differently. If the owner ever adds such a fixture, re-run the script; it takes a project file and an exposure sheet and nothing else.
- **It does not say the reuse is imaginary.** 45.5% is real, and it is periodicity. What it says is that reaching it costs 40 GiB, which document 08 line 43 declines to assume and D-40 declined twice.

## What P-02 did not do

- **It ran no compositor and measured no time.** Every figure here is arithmetic over two fixture files. Nothing in it is a performance measurement, and the layer-composite counts are not milliseconds: `verification/P-01_frame_trace.md` is where the milliseconds are, and it puts the tile loop where layer composites happen at 1.3% to 7.8% of a frame.
- **It read no expected value.** `Fixtures/reference_shot/exposure_sheet.json` is read as the specification of the shot, the way `verification/derive_d37_reuse.py` reads it. Nothing under `Fixtures/` was written.
- **It did not model the cost of the prefix cache itself** — the hash of the input tuple, the copy of a 33 MB buffer in and out. Those are costs on the side that is already losing.
