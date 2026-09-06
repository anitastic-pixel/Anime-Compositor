# B-08b: the bounded cache of decoded cels, and what it must not change

D-37 unparked this cache on 2026-09-05 because the window was delivering a frame every 92 ms and 75 of those milliseconds were reading and decoding cels the same second of playback had already read. ADR-015 bounded what was unparked. This is the part of it a reader can check: **the cache is not allowed to change the picture or the warnings, ever, under any budget.** Produced by `tests/b08b_cache.rs` from `verification/B-08a_project.json` and the reference shot.

The rows to read first are the three that compare pixels. A warm render, a render from a cache bounded so tightly it throws away every cel it holds, and a render with no cache at all are the same picture sample for sample. If a cache ever starts serving the wrong cel, those rows are where it shows up as a number rather than as a picture somebody has to notice looks wrong.

The speed this bought is not here. It is a measurement, and it is in `verification/B-08b_cache_budget.md`, on a named machine.

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| A warm draft render is the cold render, sample for sample | 3110400 | 3110400 | pass |
| A warm draft render reports the same diagnostics, word for word | identical, all 1 of them | identical, all 1 of them | pass |
| The gap frame is still reported when the drawing either side of it came from memory | true | true | pass |
| The cold pass decoded, the warm pass did not | cold > 0, warm = 0 | cold > 0, warm = 0 | pass |
| Nothing held exceeds the budget | held <= 663552000 | held <= 663552000 | pass |
| A warm full-resolution frame 100 is the cold one, sample for sample | 8294400 | 8294400 | pass |
| A cache bounded to one cel renders what an unbounded one renders | 3110400 | 3110400 | pass |
| A cache bounded to one cel reports what an unbounded one reports | identical | identical | pass |
| It stayed inside one cel by evicting, not by growing | held <= 33177600, evictions > 0 | held <= 33177600, evictions > 0 | pass |
| The cache export uses holds nothing after a full pass | 0 cels, 0 bytes, 0 hits | 0 cels, 0 bytes, 0 hits | pass |
| The uncached path renders what the cached one renders | 3110400 | 3110400 | pass |
| The same unchanged file is answered from memory the second time | 1 decode, 1 hit | 1 decode, 1 hit | pass |
| and the remembered answer is the decoded one, sample for sample | 8294400 | 8294400 | pass |
| A file that changed under the same name is decoded again | 2 decodes | 2 decodes | pass |
| and the answer is the new file, not the remembered one | different from the first | different from the first | pass |
| and it is exactly what an uncached decode of the new file gives | 8294400 | 8294400 | pass |
| Ten loops of the shot hold what one loop held | 497664000 then nine identical | 497664000 then nine identical | pass |
| and the nine loops after the first decode nothing at all | 0 decodes | 0 decodes | pass |

**18 of 18 checks pass.**

## What one cel costs to hold

A cel of this composition is 1920 by 1080. On disk, and in `verification/D-37_decode_cost.md`, that is 8,294,400 bytes. In memory it is 33177600 bytes, four times as much, because the renderer samples a buffer of `f32` in the working space rather than the bytes that were on disk. ADR-015 records the correction; it matters because a budget written against the smaller figure would have held four times the memory it promised.

The viewer's default budget is 134217728 bytes, which is 4 cels of this size, and it was chosen by the measurement in `verification/B-08b_cache_budget.md` rather than by a guess ahead of it. The checks above deliberately do not use that number: they use a budget with room for every drawing the six frames touch, because a check about whether remembering changes the answer should not also depend on how much is remembered. Ten loops of six frames ended holding 497664000 bytes in 15 cels, which is every distinct drawing those frames use and no copy of any of them.
