# P-09: how often the effect stack does work it has already done

Document 15's P-09, the entry `verification/P-01_frame_trace.md` created. It counts, on paper, and runs no compositor: `verification/derive_p09_effect_reuse.py` reads `Fixtures/reference_shot/exposure_sheet.json` and the fixture definition in `tests/common/mod.rs`. Re-run it with

    python verification/derive_p09_effect_reuse.py

If this file and that script ever disagree, the script is right and this file was edited by hand.

**Why this entry exists.** P-01 measured the effect stack at 23.2% of a cold draft frame, **65.2% of a warm draft frame** and 46.2% of a warm full frame on the declared fixture — first or second in every row of that workload, and ranked by neither research document. ADR-017 runs the stack whole-layer, before the frame plan, on the cel's own pixels, so one evaluation's entire input is *which drawing the layer shows* and *the resolved stack*. Nobody had counted how often that pair repeats.

**The answer in one line: 93.3% of the declared fixture's effect evaluations are repeats of one already done, and the most expensive instance in it can be made to stop repeating for 0.37 GiB.**

## The count

The reference shot carries no effect instances at all, which is why P-01 measures its effect stack at 0.0% of a frame. Everything here is the declared ten-layer fixture, whose three instances — one of each kind this build has — are declared in `tests/common/mod.rs`.

Both fixtures' properties are static; `verification/P-02_prefix_reuse.md` asserts that rather than assuming it, by walking the project document for a non-empty `keyframes` array. So an evaluation's whole input really is the drawing, and this count is exact rather than approximate.

| Layer | Instance | Kind | Exposes | Evaluations | Distinct inputs | Repeats | Memory to hold every distinct result |
|---|---|---|---|---|---|---|---|
| copy2 | fx-exposure | `core.exposure` | layer2 | 240 | 24 | 216 (90.0%) | 0.74 GiB |
| copy6 | fx-blur | `core.gaussian_blur` | layer3 | 240 | 12 | 228 (95.0%) | 0.37 GiB |
| copy9 | fx-tint | `core.tint` | layer3 | 240 | 12 | 228 (95.0%) | 0.37 GiB |
| **all three** | | | | **720** | **48** | **672 (93.3%)** | **1.48 GiB** |

One held result is a whole source-sized buffer, 1920 × 1080 with four f32 channels a pixel: **33,177,600 bytes**, the same as a decoded cel. Like P-08's prefix cache, this would compete with the cel cache for the 1 GiB D-40 set rather than add to it.

## One least-recently-used cache shared by all three does not work, and the table says why

| Results held | Memory | Evaluations removed | Share of 720 |
|---|---|---|---|
| 4 | 0.12 GiB | 240 | 33.3% |
| 8 | 0.25 GiB | 240 | 33.3% |
| 16 | 0.49 GiB | 240 | 33.3% |
| **32 (the 1 GiB D-40 sets)** | **0.99 GiB** | **240** | **33.3%** |
| **48** | **1.48 GiB** | **672** | **93.3%** |
| unbounded (48 held) | 1.48 GiB | 672 | 93.3% |

Flat, then a cliff at 48. That is the textbook behaviour of least-recently-used against a request stream that cycles: the working set is 48 distinct results and the loop returns to a key only after touching all the others, so a cache holding fewer than 48 evicts every entry exactly before it is wanted. The 240 it does remove at every size below the cliff are the adjacent-frame hits — the blur and the tint sit on a layer that moves on twos, so every second frame asks for what the frame before asked for, three requests ago.

**The cliff is at 1.48 GiB and the budget is 1 GiB.** A shared cache is therefore the wrong shape for this, and the table is the argument rather than an opinion about caches.

## What does work, and it is the cheapest thing on this list

Split by instance and buy only the expensive one.

| | Evaluations | Repeats | Memory |
|---|---|---|---|
| the blur alone, cached on its own | 240 | **228, 95.0%** | **0.37 GiB** |

`core.gaussian_blur` is the only one of the three whose output pixel reads more than its own input pixel. `core.exposure` and `core.tint` are per-sample arithmetic on a buffer that has to be walked anyway; the blur runs a kernel over neighbours. **This is a structural argument and not a measurement** — P-01's timer covers the stack as one stage and does not split it by instance — but it is the reason to spend the 0.37 GiB on the blur first and to measure the other two before spending anything on them.

## What this would be worth, stated as what it is

P-01 measures the declared fixture's effect stack at a p50 of 141.617 ms inside a 292.281 ms warm full frame, and at 65.2% of a warm draft frame. If 93.3% of the evaluations stopped happening, and if the time went with the count, that stage would be near-empty. **That "if" is not a result and this file does not report it as one.** Nothing here was timed; the count is a count. What P-09's exit condition asks for is a recommendation of which road to offer the owner, and the count is enough for that.

## The recommendation: which road

Two roads were open, and the evidence now points at one of them.

**Road A, evaluate the stack on a draft-resolution source.** This is what would make the stage cheap rather than skippable, and it is 16× less memory a held buffer as well: at 480 × 270 the whole 48-result working set is 0.095 GiB, comfortably inside the budget. But downsampling then transforming is not transforming then downsampling, so **it changes preview pixels**, which makes it a specification question. It belongs to P-06 and to the owner's answer in document 14 about whether D-33's "the viewer says when the preview differs from export" covers this difference. **P-09 does not take that decision and cannot.**

**Road B, cache the evaluated result, bit-exact.** Nothing about any frame changes; the same buffer is computed once instead of 240 times. The key is short — the cel key `src/cache.rs` already builds, plus a hash of the resolved stack — and document 33's caveat on P-08 applies but is much easier to satisfy here than for a whole prefix, because ADR-017 already fixes the input to a single layer's pixels before the frame plan exists.

**Recommend B, and recommend it scoped to one instance rather than to the stack.** Concretely: a cache of evaluated effect results, sized in bytes like the cel cache and sharing D-40's budget, and the first thing to measure is the blur at 0.37 GiB removing 228 of 240 evaluations. If that lands, the other two are measured and bought or not bought on their own numbers. Road A stays open and unprejudiced: it is a different kind of win, it is the owner's to allow, and B does not foreclose it.

**Neither road is proposed as a backlog entry by this file.** P-09's scope was the count, and the count is here.

## What P-09 did not do

- **It changed no code under `src/`**, which is what its scope said.
- **It timed nothing.** Every millisecond quoted above comes from `verification/P-01_frame_trace.md` and is labelled where it does.
- **It did not split the effect stack by instance in the timer.** That would need a per-instance stage in `src/perf.rs`, and P-09 does not add one. The blur being the expensive instance is an argument from what the three effects do, not a measurement of them.
- **It read no expected value.** Nothing under `Fixtures/` was written; the exposure sheet is read as the specification of the shot, the way `verification/derive_d37_reuse.py` reads it.
- **It counted one playthrough of one fixture.** The reference shot has no effects to count, and a project with an animated effect parameter would count differently — the script would refuse it, because `verification/P-02_prefix_reuse.md`'s assertion is what makes the drawing the whole key.
