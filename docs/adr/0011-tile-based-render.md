# ADR-011: Tile-based render plan

Status: ACCEPTED
Date: 2026-09-04
Deciders: Andrew (owner)

## Context

The renderer needs a work decomposition. The owner wants strong CPU and memory utilization now, and a GPU path eventually, and asked that the design not foreclose the second.

## Decision

The unit of render work is a tile: a fixed-size rectangular region of the output frame, composited independently of every other tile in that frame and distributed across worker threads by rayon. Tiles within one frame evaluation never depend on one another. Tile size is a tunable measured on the reference machine, not a constant chosen in advance.

This is committed as a contract in document 21 rather than left as an implementation detail.

## Rationale

Tiling is the natural decomposition for CPU parallelism across twelve cores, and it is the same decomposition a GPU compute dispatch requires. It is therefore the single piece of forward design that converts the GPU deferral in ADR-006 from a dead end into a port.

Retrofitting tiling onto a whole-frame renderer later is a renderer rewrite. Adopting it from the start costs almost nothing, because the per-pixel math in document 21 is unchanged by it.

## Consequences

Operations with neighborhood dependence, such as blur, must declare a margin; the renderer evaluates the tile with that margin and crops. The margin is part of the operation contract and is declared by the operation, never assumed by the renderer. An operation with unbounded spatial support cannot be tiled without an explicit strategy and must not be added casually.

This is one concrete reason the R-05 effects sit in G1-rest rather than G1-core: the blur in that requirement is the first operation that exercises margin handling, and it deserves its own fixtures rather than arriving inside a milestone that has none.

Tiled output must be byte-identical to whole-frame evaluation, and must not depend on thread count, completion order or scheduling. B-05a proves this rather than assuming it.

## A noted exception

This is the one instance in this pack of building for a future need rather than a present one, and it sits in tension with the anti-bloat policy in document 04.

It is accepted deliberately, on the grounds that the cost now is near zero and the cost later is a rewrite. It is recorded as an exception rather than as a precedent, and should not be cited to justify other speculative design.
