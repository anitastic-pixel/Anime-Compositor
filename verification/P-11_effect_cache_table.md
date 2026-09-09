# P-11: the evaluated-effect cache

Document 15's P-11. Running an effect stack is the most expensive thing a frame of the declared fixture does - the Gaussian blur alone is 111 to 123 ms of it, measured by the split `src/perf.rs` gained for this unit - and a playthrough runs the same stack over the same drawing again every time that drawing comes back on screen. This unit remembers the result instead.

The timings are in `verification/P-11_effect_cache.md`. **This page is the other half: the checks that the cache changes nothing except how long a frame takes.** Document 27 line 29 puts it in one sentence - caching "may improve interaction but must never define correctness" - and there are exactly three ways it could break that.

**It could hand back the wrong picture.** Eighty frames of the declared ten-layer fixture, at both preview qualities, are rendered from the cache and again with no cache at all, and every pair has to be identical byte for byte. The comparison is against a cacheless render rather than against the cache's own first pass, because a cache that was wrong twice would agree with itself.

**It could fail to notice that something changed.** The thing a cache is remembered by is the picture it shows after an edit that it missed. Three checks move one input at a time - the blur's amount, a corner of the mask, and the mask's presence - and each has to come back as "not found".

**It could lose a warning.** An effect this build cannot draw is reported once per frame (document 28), and a frame answered from the cache never runs the stack. The last check renders the same frame twice, the second time from the cache, and both have to carry the warning.

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| 80 frames of the declared fixture rendered from the effect cache (72 hits, 152 evaluations) match the same frames rendered with no cache | 0 frames differ, and the cache was used | 0 frames differ, and the cache was used | pass |
| the same cel, mask and effect stack is found again | found | found | pass |
| a blur of 4.5 pixels is not served the result of a blur of 4.0 | not found | not found | pass |
| a mask with a corner moved is not served the old mask's result | not found | not found | pass |
| a layer with no mask is not served a masked layer's result | not found | not found | pass |
| an unsupported effect warns on the frame that evaluated the stack and on the frame served from the cache (3 effect-cache hits) | 1 warning, then 1 warning | 1 warning, then 1 warning | pass |

## What is deliberately not here

**The cel cache is untouched.** This is a second, separately budgeted list beside the decoded cels, and not a share of theirs, because `verification/P-09_effect_reuse.md` measured what one list would do: the request stream cycles, least-recently-used is at its worst against a cycle, and effect results competing with cels in one list would be dropped exactly before they are wanted.

**The gibibyte did not grow.** The effect budget is taken out of D-40's budget and not added to it, so the window holds what document 40 said it holds.

**Export still caches nothing.** ADR-015's third bound is that an export neither reads the cache nor writes it, and an export builds `CelCache::none`, which has no effect budget either. Every check above is a preview.
