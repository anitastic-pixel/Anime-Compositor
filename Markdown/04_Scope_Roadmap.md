# Scope, roadmap and anti-bloat policy

Version 0.3 | 2026-09-04 | Accepted for baseline

## Scope stages

Stage G0: feasibility. Run SP-01, SP-03, SP-04, SP-05 and SP-06 on the reference machine, draw the reference shot, and record results. The stack decision is already made; these spikes confirm or reopen it rather than choosing it.

Stage G1-core: a useful cel finishing tool. Deliver R-01, R-02, R-03, R-06a, R-07, R-08, R-09, R-10 and R-11. PNG in, exposure timing, layer stack, 2D transforms, undo, durable projects, PNG sequence out. Blend modes limited to normal, multiply, screen and add with documented math. No masks, no effects, no cache.

Stage G1-rest: deliver R-04 masks and mattes, R-05 the three effects, and R-06b the bounded cache, each only when its revisit trigger in document 23 actually fires.

Stage G2: deliver R-12 flat planes, camera and parenting, then R-13 bounded expressions. Requires G1 passing and measured memory headroom.

Stage G3: production conveniences. Precompositions, adjustment layers, richer masks, EXR, packaging, anime-specific processing. Prioritized by observed shot work, never by catalog size.

## Why G1 was split

Version 0.2 listed fifteen must-have features for G1. That list described a small After Effects, and it was not deliverable by one person without a coding background at an uncommitted weekly capacity.

The split follows the logic in document 30: the structural gap against Natron, Fusion and Blender is exposure-and-layer-first finishing. Masks and blur are not gaps, because those tools already do them well. So the first milestone contains only what makes this tool worth existing, and the features every competitor already has are deferred until a real shot proves they are needed here too.

The bounded preview cache is the clearest case. Document 27 is a complete specification for a system guarding a performance problem that has never been observed, on a machine with twelve cores and 64 GB of memory. It is parked with a measurement as its trigger.

---

## Explicit exclusions

Not in any stage: drawing engine, vector animation, skeletal rigging, motion capture, 3D scene renderer, video editing timeline, advanced tracking, roto automation, generative features, asset store, collaboration service, mobile port.

Not in any stage: AEP importer, AE binary plugin host, comprehensive AE expression compatibility, cloned commercial effect catalogs. These are separate products with independent technical and legal costs.

## Dependencies

Time and alpha contracts precede all rendering work. Serializable commands precede interface complexity. Save and recovery precede any use with artwork the owner cares about. Stable render evaluation precedes any performance optimization. The tile contract precedes the renderer, because retrofitting tiling is a rewrite.

G1-rest depends on G1-core passing its fixtures. G2 depends on G1 and on measured headroom. G3 depends on evidence of repeated manual effort.

## Feature admission rule

For every addition, write the affected workflow, expected frequency, smallest useful behavior, requirement ID, dependency cost, test fixture and revisit trigger. If it does not unblock a workflow or measurably remove repeated effort, it stays parked.

Compare value against implementation, testing, interface, documentation, migration and permanent maintenance cost. Do not justify an addition because a competitor has it, because a library exists, or because an agent can generate the code quickly. That last one is the trap specific to this project: cheap code generation makes scope expansion feel free, and it is not, because every feature must still be specified, fixtured, verified and maintained by one person who cannot read it.

One major subsystem in active development at a time. Do not open several partially working systems to make the roadmap look faster.

## Parking and removal procedure

Record the reason, affected requirements, user-visible consequences and the revisit trigger. Keep the specification; parked work stays fully written so promotion needs no re-planning. Remove abandoned interface entry points and stale documentation. Preserve unknown serialized data so deferral does not damage old projects.

Revisit parked features after G1-core acceptance and after each small batch of real finished shots.

## Release naming

Prototype, internal alpha and artist alpha until objective gates pass. No claim of being a replacement for anything. The long-term ambition can stay large while each build states precisely what it supports.

Related documents: 01, 03, 12, 13, 14, 23 and 30.
