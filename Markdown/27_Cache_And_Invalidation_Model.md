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

Effect parameter/order change: invalidate that effect stage and downstream results for affected frames. D-68, accepted on 2026-09-18: the effect parameters in a cache key are their values at the frame, not their keys, so an edit to one key affects only the frames whose value it changes.

Mask edit: invalidate mask-dependent layer/effect stages according to the render order in 21.

Matte edit or referenced layer change: invalidate dependent layer results and downstream composition frames transitively.

Any edit inside a composition another holds a layer of (D-67): a composition layer's source is the inner composition's frame, keyed as that frame is, so the edit invalidates the composition-frame results of every composition holding a layer of it, at any depth, for the affected frames.

Layer reorder/visibility: preserve source/layer caches where possible; invalidate composition-frame results for affected frames.

Color interpretation/working-space change: conservatively invalidate all pixel caches.

## Revision and cancellation

Preview requests carry document revision and cancellation token. Results from an obsolete revision may enter content-addressed lower-level caches only if their key proves independence, but the viewer must never display them as the current revision.

Cancel stale interactive work promptly between tiles/stages where practical. Export uses an immutable snapshot and is not invalidated by live editing.

## Memory policy

Define a configurable cache budget after measuring the reference machine. G1 must degrade by eviction rather than unbounded growth. Repeated playback loops must reach a stable memory range under T-06.

Do not page large raw image caches to the project directory. Temporary cache storage, if added later, must be separate, disposable and versioned.

Decoded drawings on disk (B-161, D-232): the one temporary cache storage so far. Scope: the viewer's decoded media cache only; export never reads or writes it (ADR-015). It keeps the decoder's 8-bit sRGB straight RGBA bytes, before any arithmetic, and the colour conversion runs on every read, so a read is bit-identical to a decode. EXR is not kept. Key: path, size, modified time, interpretation and a decoder version, per key material above; a same-size, same-mtime replacement is the accepted residual risk of interactive identity. Separate: its own folder ("decoded drawings" in the app's local data folder or one chosen in Preferences), and only `.cel`/`.part` files there are touched. Disposable: bounded by a Preferences cap (default 5 GB, 0 turns it off), least recently used deleted first; a copy failing its length, header or checksum is deleted, decoded fresh and noted in the session log (document 28, DECODE_CACHE_DISCARDED). Versioned: the decoder version is in every file name.

Finished frames in memory, the RAM preview (B-154, D-223): the second cache of the viewer's. Scope: the viewer's frames only, `FrameCache` in `src/cache.rs` held by the window; export and every render to file never read or write it (ADR-015), and the export code does not name it. It keeps the 8-bit sRGB straight bytes the page receives, with the frame's width, height, whether the card fell back to the CPU and the warnings the session log was given, so a frame from memory is the same bytes and the same diagnostics as one made fresh. Key: the whole project as shown (after solo) compared by value, so any edit, undo or solo is a different key and undo finds its frame again; the composition; the drawings' root folder; the preview quality; which processor drew it and at what card budget; and the frame number. Alpha-only and the checkerboard are applied after, by the page, and are not part of it. Files: every file the render looked at (`cache::looked_at`: drawings, colour tables, the missing-media checks) is recorded with its size and modified time while the frame is made, and all of them are looked at again before a remembered frame is sent; any change makes the frame again, and a frame that read a file with no modified time is not kept. A same-size, same-mtime replacement is the accepted residual risk of interactive identity, as for the decoded drawings. Budget: the Preferences memory setting is shared. The finished frames may hold at most half of it, and never so much that the cels have less than 1 GiB, so at a 1 GiB setting there are no finished frames and the viewer is what it was before. The cels (with their seven sixteenths for effect results) have the setting less what the finished frames hold, fitted each time the cels are locked to make a frame, so a first pass works with the whole setting. A fixed half for the cels starved a heavy shot's first pass: document 08's ten layers at Full dropped 85 of 240 frames, against 24 before. The setting is a most, not a reservation: between two fittings the two together can be over it by the frames stored since, a frame or two. Eviction: whole views least recently used first (at most 16), and within its own view a frame is not admitted once the budget is full, so a loop longer than the budget keeps its first part rather than churning. Render-ahead: while the shot plays a worker makes the frames after the one on screen, in playback order, into this cache, and stops when playback stops, the budget is full, or a frame fails.

The layers below an edit (B-159, D-241): the third cache of the viewer's, and the smallest. Scope: the viewer's processor drawing only, `render::Below` held by the viewer's `CelCache` (`CelCache::below` is `None` for any cache without an effect budget, which is every cache an export or a render to file uses), so export never reads or writes it (ADR-015); the card does not use it. It keeps one picture, the frame in the working space as it stood below one layer, and the plan last drawn. Key: the composition and frame number, then the frame's width and height, the tile size and the part drawn (B-158), then every layer below the kept one compared by value with the last plan's: the same drawing buffer (held, so it cannot be changed or its memory reused), map, opacity, matte and motion-blur moments to the bit, and the same blend mode, adjustment effects, Light Wrap and card effects. Anything that changes a layer's pixels changes one of those, since the plan is everything the drawing reads. Invalidation: another frame or composition forgets it; a layer that differs below it makes it unused, and a new picture is kept. Budget: outside the memory setting: one frame-sized picture and the last plan's drawings, which the cel cache holds anyway unless it is squeezed.

A held masked drawing (B-170, D-242): not a new cache. The layer/effect cache (P-11) now also holds a masked drawing whose effects are all left to the card or which has none, and is asked before the mask is drawn, so a hit skips the mask. Key unchanged: the drawing's identity and interpretation, the masks drawn at their shape at the frame, the processor's effects at the frame and the draft divisor. Scope and budget unchanged: the viewer's effect budget; export has none and never reads or writes it.

Composition frames (B-171, D-243): the fourth cache of the viewer's. Scope: the viewer's `CelCache` only (`inner_frame`/`store_inner` do nothing without an effect budget, which is every cache an export or a render to file uses), so export never reads or writes it (ADR-015). It keeps a composition layer's inner frame in the working space as `render::render` drew it on the processor, before the layer's own mask, effects and transform. Key, compared whole: the program's build (its file's path, length and modified time), the drawings' root, the preview quality, the inner frame number, every composition reachable from the inner one through composition layers and every asset, by value as text. Files: every file the inner frame looked at (`cache::looked_at`) with its size and modified time, looked at again before a kept frame is used, and noted for the outer frame so B-154 still sees them; any change draws the frame again, and a frame that read a file with no modified time is not kept. Not kept: a frame that raised any diagnostic, and a composition that reaches one it is drawn inside. Memory: under the effect budget with the effect results, least recently used of both let go first. Disk: B-161's folder and cap, `.frame` files (SHA-256 of the key; the whole key, the file list and the floats inside, with checksums; `.part` then rename), least recently used deleted first with the `.cel` copies; only a frame that took longer to draw than it would to read back at 2 GB/s is written. A copy failing a check is deleted and drawn fresh (session log FRAME_CACHE_DISCARDED; FRAME_CACHE_NOT_WRITTEN when it cannot be written). Versioned: the build is in every key. A same-size, same-mtime replacement of a file is the accepted residual risk of interactive identity, as for the decoded drawings.

## Correctness tests

For each command class, render before edit, after edit and after undo; verify only intended pixels/time ranges change. Force cache hits and misses and compare output. Replace one source frame on disk and verify dependent frames update without relaunch.

Related documents: 06, 21, 26 and 11.
