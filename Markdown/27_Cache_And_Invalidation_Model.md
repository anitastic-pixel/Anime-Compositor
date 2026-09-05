# Cache and invalidation model

> **PARKED in version 0.3 under D-12.** The bounded preview cache is not part of G1-core. This specification is retained in full and unchanged below so that it can be promoted without re-planning.
>
> Revisit trigger: measured preview latency on the reference shot, recorded with numbers, that makes editing unpleasant. Not an opinion, a measurement.
>
> Why it was parked: it is a performance optimization guarding a problem that has never been observed, on a 12-core machine with 64 GB of memory, in a project whose scarcest resource is owner verification time. See documents 04 and 23.


Version 0.2 | 2026-09-04 | Proposed baseline

## Goals

Caching may improve interaction but must never define correctness. A cold render and a fully warm render for the same immutable request must produce equivalent pixels and diagnostics. Cache keys are derived from explicit inputs rather than widget state or memory addresses.

## Cache layers

G1 may use four logical caches even if one implementation combines storage:

- decoded media cache: decoded/tagged source frame;
- layer/effect cache: layer-space result before composition transform where profitable;
- transformed-layer cache: composition-space layer result before final stack blend;
- composition-frame cache: final frame for a project revision/quality/output interpretation.

A bounded memory manager owns eviction across these caches. Eviction changes performance only, not result.

## Key material

A cache key includes the minimum complete set of values that can change the result: media content identity, interpretation metadata, project revision or normalized property hashes, composition/layer IDs, frame/sample time, effect type/version/parameters, mask/matte dependencies, transform, render scale, quality, working/output color interpretation and implementation shader/kernel version where relevant.

File path alone is not sufficient media identity. At minimum include observed size/mtime for interactive invalidation; content hashes are preferred for packaged/reproducibility workflows where cost is acceptable.

## Invalidation classes

Media bytes/interpretation change: invalidate decoded frame and all descendants that depend on that asset.

Exposure edit: invalidate affected layer and downstream composition frames only for the changed spans.

Transform/opacity/blend change: preserve decoded/source-effect cache where independent; invalidate transformed/composition results for affected frames.

Effect parameter/order change: invalidate that effect stage and downstream results for affected frames.

Mask edit: invalidate mask-dependent layer/effect stages according to the render order in 21.

Matte edit or referenced layer change: invalidate dependent layer results and downstream composition frames transitively.

Layer reorder/visibility: preserve source/layer caches where possible; invalidate composition-frame results for affected frames.

Color interpretation/working-space change: conservatively invalidate all pixel caches.

## Revision and cancellation

Preview requests carry document revision and cancellation token. Results from an obsolete revision may enter content-addressed lower-level caches only if their key proves independence, but the viewer must never display them as the current revision.

Cancel stale interactive work promptly between tiles/stages where practical. Export uses an immutable snapshot and is not invalidated by live editing.

## Memory policy

Define a configurable cache budget after measuring the reference machine. G1 must degrade by eviction rather than unbounded growth. Repeated playback loops must reach a stable memory range under T-06.

Do not page large raw image caches to the project directory. Temporary cache storage, if added later, must be separate, disposable and versioned.

## Correctness tests

For each command class, render before edit, after edit and after undo; verify only intended pixels/time ranges change. Force cache hits and misses and compare output. Replace one source frame on disk and verify dependent frames update without relaunch.

Related documents: 06, 21, 26 and 11.
