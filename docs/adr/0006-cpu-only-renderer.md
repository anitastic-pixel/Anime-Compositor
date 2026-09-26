# ADR-006: CPU-only rendering for G1-core, GPU trigger-gated

Status: ACCEPTED, amended 2026-09-25 by D-100 (see below)
Date: 2026-09-04
Deciders: Andrew (owner)

## Context

Version 0.2 specified a permanent CPU reference renderer for correctness alongside a production GPU path, with SP-02 proving equivalence within declared tolerances.

That is two implementations of every pixel operation, maintained indefinitely, in a project with one owner who has no programming background and no committed weekly capacity.

## Decision

G1-core has exactly one renderer: tile-based, multithreaded, CPU.

No GPU backend is built until a measured result on a real shot on the reference machine shows the CPU path is too slow for the workflow. The trigger is a recorded stopwatch reading, not an opinion and not an expectation.

SP-02 is removed from the spike list.

## Rationale

Correctness is established against independently derived expected values in document 25, not against a second renderer. That is the stronger arrangement regardless of capacity: two implementations written by the same author verify that they agree with each other, which is consistency, not correctness against the specification.

A Ryzen 9 9900X with twelve cores compositing 1080p PNG layers is fast enough to finish shots. A GPU path is a performance optimization for a workload nobody has run yet, which is the same pattern that parked the preview cache under D-12.

Halving the renderer surface directly serves ADR-013, since every line must be verified through fixtures rather than review, and fewer lines means less unverifiable surface.

## Consequences

Preview performance in G1-core will be modest. This is expected and is not a defect. Document 03 reflects the same reasoning by removing the bounded cache from the first milestone.

The tile contract in ADR-011 exists specifically so a later GPU path is a dispatch port rather than a renderer rewrite. This decision is a deferral, not a rejection.

The RTX 4070 Ti Super sits unused during G1-core. Owning capable hardware is not a reason to build a path for it.

## Reopening

Reopen when a real shot is measurably unpleasant to work with and the measurement is recorded. At that point the tile decomposition should make the port tractable, and the CPU path remains as the correctness reference it already is.

## Amendment, 2026-09-25 (D-100)

The owner accepted D-100 road (a): "accept D-100 (a)". A GPU preview tier may be built. The GPU draws only the viewer's picture, behind a switch, and a comparison table holds it to the CPU within declared tolerances: at most 1 level of 255 for compositing, and a measured tolerance declared per effect as each is ported. The CPU remains the only renderer for fixtures, export and Full-quality checking, byte-exact as before. A GPU export stays out of scope.

The trigger in "Reopening" was not met. The owner reopened on scope grounds instead: each effect built CPU-only is another to port later. The first unit is B-44 in document 15.
