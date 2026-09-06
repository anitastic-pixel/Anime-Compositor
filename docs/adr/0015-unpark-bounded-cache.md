# ADR-015: Unpark the bounded cache, as a cache of decoded cels

Status: ACCEPTED
Date: 2026-09-05
Deciders: Andrew (owner), delegated to the agent's recommendation
Supersedes in part: ADR-014, for R-06b only

## Context

ADR-014 parked three features and gave each a revisit trigger. Two of the triggers describe something happening in a real shot. The cache's trigger was deliberately different: "measured preview latency on the reference shot that makes editing unpleasant, recorded with numbers rather than asserted." It was written that way because the cache is the entry most likely to be built out of enthusiasm, and a number is the only thing enthusiasm cannot produce.

The number now exists, and it was taken on the production preview path rather than a spike. `verification/B-08_preview_latency.md` records a median of 81.69 ms per draft preview frame and 99.90 ms per full one, which is 12.2 and 10.0 frames per second against a 24 fps target. Of the draft median, 75.15 ms is reading and decoding the four cels the frame needs and 6.53 ms is rendering. Decoding costs the same at both resolutions, because a drawing is decoded at its own size before anything scales it, which is why D-33's draft default cannot reach it.

`verification/B-08_window_shell.md` shows what that is in a window: the viewer played 66 frames and dropped 79 across about six seconds, roughly 92 ms of wall clock per frame delivered against the 41.7 ms a 24 fps shot allows.

Nothing there is a broken requirement. R-06a asks for a preview with no cache and got one; D-32 already decided that playback drops frames rather than stretching the clock, and the count is on screen. What changed is that there is now a measurement of how often that happens, and it is most of the time.

## Decision

Unpark R-06b. B-08b enters G1, specified by document 27.

Four bounds are part of the decision, not implementation detail:

1. **The cached unit is a decoded source cel, not a finished frame.** Decoding is 75.15 ms of an 81.69 ms frame; rendering is 6.53 ms. A frame cache would chase eight percent of the cost. A frame cache is not forbidden forever, it is simply not this - it waits for its own measurement, on the same discipline that produced this record.

   The unit held is the cel after document 21's step 1, which is decoded, tagged and converted into the working space, rather than the bytes that came off the disk. The conversion is inside what the measurement timed, and stopping short of it would leave the larger half of the cost being paid on every hit. Its consequence is that the asset's interpretation joins the key, which is what document 27's key material asks for anyway: "media content identity, interpretation metadata".
2. **The cache has a memory ceiling and evicts**, per document 27, and the ceiling is counted in bytes held rather than in entries. `verification/D-37_decode_cost.md` measured what it buys on the reference shot: one entry saves nothing at all, because the four layers are asked for in rotation; four entries save 54%; all 57 distinct drawings save 94%. The single best entry is the background, the most expensive drawing in the shot and the one that never changes.

   That table's 473 MB for 57 drawings needs one correction before it is used as a budget, and it is the kind that would otherwise be found by a machine running out of memory. It counts a cel as 1920x1080x4 bytes, which is what a cel is on disk and on the wire. What this build holds is the buffer the renderer samples: f32 in the working space, four times that, so 57 drawings are about 1.9 GB rather than 473 MB. The reuse curve is unchanged - it counts requests, not bytes - but the price of every point on it is four times what that sentence says.
3. **Export neither reads the cache nor writes it.** A full-resolution preview and an export of the same frame currently differ in 0 of 8,294,400 samples, and T-08's tables are byte-exact. A cache that can change one exported sample is the only failure mode here that would matter, and keeping it out of the export path costs less than proving it harmless inside one.
4. **Masks (R-04) and effects (R-05) stay parked.** Their triggers have not fired. This record moves one row.

## Rationale

The competing reading is that a cache is an optimization and optimizations are optional. That stops being true when the measured cost is more than double the frame budget and the dominant term is one no other lever in G1-core can reach: the draft default cannot, because decoding does not scale with preview extent; D-32 cannot, because dropping frames is how the cost is being paid rather than reduced.

The alternatives are all larger than a cache. A GPU path reopens ADR-006. A faster decoder is a new dependency and a licence review. Decoding fewer cels means changing what the renderer asks for, which is a change to the thing the fixtures pin.

The delegation is recorded rather than hidden: the owner asked the agent to decide this one. The four bounds above are the reason that is safe. An unbounded "yes, build the cache" would have been an agent choosing the shape of a system from a document that describes several; a yes limited to what the measurement actually names is a smaller claim than the evidence supports.

## Consequences

The preview path gains a lookup between "which cel does this frame need" and "read and decode it". The export path gains nothing.

T-06 becomes live again, and D-37 fixes what it has to show: the same reference-shot playback measured before and after the cache exists, in the same build configuration, on a named machine. A table asserting that a cache exists is not the artifact. Document 08's repeated-loop memory behaviour is part of the same test and has never been measured; the memory ceiling is what makes it worth measuring.

ADR-006 is not reopened. The renderer stays CPU-only.

**Built on 2026-09-05, the same day.** `src/cache.rs` is the cache; `verification/B-08b_cache_table.md` is the correctness half, eighteen checks of the one rule that matters, and `verification/B-08b_cache_budget.md` is the measurement this record asked for. On the reference machine a draft preview frame went from 99.93 ms to 42.54 ms, and the shot from 10.0 to 23.5 frames per second, at a default budget of 128 MB. The budget was chosen from that table rather than ahead of it: 512 MB, four times the memory, was within a millisecond of 128 MB, because playback is sequential and what has to fit is the reuse distance rather than the shot.

Two things the measurement said that the reasoning above did not. A cache bounded to a single cel is worth nothing at all — it evicts what it is about to need and finishes no faster than no cache — so the ceiling is not a dial that can be turned down arbitrarily. And the correction in the Context section held: a budget written against the on-disk figure would have held four times the memory it promised.

What is still owed is the window-level half. `verification/B-08_window_shell.md` counts playback in dropped frames by photographing a running window, and that picture has not been retaken: on 2026-09-05 the capture script's synthetic keystroke stopped reaching the webview, so three attempts produced a picture of an idle viewer rather than a playing one. The engine number moved and is recorded; the number a person sees has not been re-measured, and this record does not claim it has.

If the owner reverses this, B-08b returns to G1-rest and the preview path loses a lookup. Nothing else in the build depends on it, which is a property worth keeping as the cache is written.
