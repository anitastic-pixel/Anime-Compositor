# ADR-012: Narrow G1 to G1-core

Status: ACCEPTED
Date: 2026-09-04
Deciders: Andrew (owner)

## Context

Version 0.2 listed fifteen must-have features for the first milestone: import, exposure, layers, transforms, masks, alpha mattes, three effects, four blend modes, viewer, bounded preview cache, undo, persistence with recovery, export, color handling and offline operation.

That is a small After Effects. It was scoped before the owner capacity and background were known, and it is not deliverable by one person without a coding background at an uncommitted weekly capacity.

## Decision

Split the first milestone.

G1-core: R-01 import, R-02 exposure timing, R-03 layers and transforms, R-06a preview without a cache, R-07 undo, R-08 save and recovery, R-09 export, R-10 color and alpha, R-11 offline.

G1-rest, parked with revisit triggers: R-04 masks and mattes, R-05 the three effects, R-06b the bounded preview cache.

Parked work stays fully specified. Nothing is deleted.

## Rationale

Document 30 identified exposure-and-layer-first finishing as the one structural gap against Natron, Fusion, Blender and OpenToonz. Everything else on the version 0.2 must-have list is something those tools already do well. So the first milestone should contain only what makes this tool worth existing, and the features every competitor already has should wait until a real shot proves they are needed here too.

The bounded preview cache is the clearest case and worth stating plainly. Document 27 is a complete specification for a system that guards a performance problem which has never been observed, on a machine with twelve cores and 64 GB of memory, in a project whose scarcest resource is owner verification time. It scored highest complexity and risk in document 23 and was still a must-have. Its revisit trigger is now a measurement.

## Consequences

The first release will visibly lack masks and effects. This is a deliberate statement about what the tool is for, not an admission of incompleteness, and the charter now says so.

Every parked item carries a written revisit trigger in document 23, so promotion is an owner decision based on evidence rather than a judgment call made inside an implementation task.

Implementation agents may not build parked work. This is enforced in root `AGENTS.md`.

If capacity proves insufficient even for G1-core, narrow again rather than extending silently. Version 0.3 establishes that narrowing is a normal move, not a failure.
