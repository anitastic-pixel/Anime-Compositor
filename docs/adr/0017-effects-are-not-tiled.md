# ADR-017: The effect stack runs whole-layer, before the frame plan, so effects are never tiled

Status: ACCEPTED
Date: 2026-09-06
Deciders: Andrew (owner), delegated to the agent's recommendation
Relates to: ADR-011 (tile-based render), D-12 (unparking R-05), document 21 lines 89 and 119-121, document 03 R-05

## Context

Document 21 line 119 states the rule this decision is about:

> An operation whose output pixel depends on a neighborhood, such as blur, requires the tile to be evaluated with a margin of the operation's radius, then cropped. The margin is part of the operation's contract and must be declared by the operation rather than assumed by the renderer.

Line 121 then names the blur as the first operation that will exercise that machinery, and calls it "a direct reason R-05 sits in G1-rest". R-05 in document 03 was written to match, and asked B-07 to "show that a tiled render equals the full-frame reference math within tolerance, which is what document 21 line 89 asks of any region-of-interest optimisation".

When B-07 came to write that comparison it turned out there was nothing to compare. This record says why, and what was checked instead.

## Decision

**The effect stack is evaluated once over the whole layer buffer, while the frame plan is being built, and is finished before the renderer exists. Effects are therefore never tiled, and no effect declares a tile margin.**

`crate::effects::apply_stack` runs inside `compose::resolve_layer`, on the layer's own decoded pixels in layer space, at document 21's step 3. What it produces is a `LayerDraw` whose `source` is a complete buffer with the mask and the effects already in it. `render::render` then cuts tiles out of the *composition* and, for each tile, samples each layer's finished source through that layer's transform. By the time a tile exists, every blur in the frame is already a set of pixel values.

**The margin requirement of line 119 is therefore satisfied vacuously, and the ROI equivalence R-05 asked for cannot be demonstrated, because there is no ROI optimisation to demonstrate it about.**

**What B-07 proves instead is the property that would have failed had this been got wrong: the finished frame does not depend on the tile size.** `verification/B-07_effects_table.md` renders one blurred frame at six tile sizes — 1, 3, 7, 16, 32 and 64, three of which do not divide the frame — and requires six byte-identical results.

## Rationale

The alternative was the design document 21 line 119 anticipates: evaluate effects per tile, with each operation declaring a margin the renderer expands the tile by and then crops. That is the right design when effects are evaluated at output resolution inside the tile loop, which is what a compositor that keeps everything lazy does.

This build is not that. It decodes a cel into a layer buffer at source resolution and transforms it into the composition per tile; the effect stack belongs to the layer, in layer space, before the transform. Adding a per-tile effect path would mean evaluating the same blur once per tile that a layer touches, expanding each of them by the kernel radius, and reconciling the overlaps — more arithmetic, more code, and a new class of bug (the seam) in exchange for nothing this build needs. Document 21's own layer render order puts effects at step 3 and the transform at step 4, so running them whole-layer is also the literal reading of the pipeline.

The cost is memory: a layer carrying a large blur holds a buffer expanded by the kernel radius on all four sides for the life of the plan. At G1 sizes that is small — a 1920x1080 layer with a sigma of 10 grows to 1980x1140, about ten per cent — and it is bounded by the sigma, which is a stored parameter, not by anything the renderer chooses.

Line 89's "every effect declares input bounds expansion" is honoured regardless, and is not the same requirement as the tile margin: `Effect::bounds_expansion` returns the kernel radius for a blur and zero for exposure and tint, and `verification/B-07_effects_table.md` checks all four. That declaration is what the caller uses to grow the buffer and to shift the layer transform back by the same offset, so that expanding the bounds adds margin without moving the picture. The table checks that too, by the alpha centroid of the whole frame.

## Consequences

R-05's sentence in document 03 asking for a tiled-versus-full-frame comparison is superseded by this record and by D-44 in document 14. The requirement's substance — that tiling must not change the picture — is met and checked; the specific experiment it named does not apply to this architecture.

If a later stage moves effect evaluation into the tile loop — a GPU path, or a lazy graph, or preview-resolution effects — this record is void and document 21 line 119 applies in full: every operation must then declare a margin, and the tiled-equals-full-frame comparison R-05 asked for becomes the check that must be written. The tile-size-invariance rows in `verification/B-07_effects_table.md` are the ones that would catch such a change going wrong, and they should be kept whatever else replaces them.

What this record does not settle: an operation with unbounded support, which document 21 says "cannot be tiled without an explicit strategy and must not be added casually". None of the three G1 effects has one. That sentence stays open for whoever proposes the first one.
