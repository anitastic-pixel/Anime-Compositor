# ADR-003: Rust for the core

Status: ACCEPTED
Date: 2026-09-04
Deciders: Andrew (owner)

## Context

The document model, evaluation and renderer need a language. Version 0.2 proposed C++20 with Rust as a secondary candidate, to be decided by a spike comparing toolchain friction, debugger quality, test ergonomics and dependency fit.

That framing assumed a human would be reading, debugging and reviewing the resulting code. During version 0.3 planning the owner disclosed no programming background, which invalidates the comparison as originally posed.

## Decision

Rust for the core.

## Rationale

The deciding factor is the failure mode, not performance. With no human reading the code, the dangerous defect is not one that crashes loudly but one that produces plausible-looking wrong behavior intermittently.

C++ specializes in exactly that class. Memory corruption and undefined behavior surface as occasional wrong pixels and irreproducible crashes. A non-programmer owner cannot diagnose those, and an agent cannot reliably reproduce them. Rust converts most of that class into compile-time refusal, which is visible to the agent immediately and never reaches the owner at all.

Supporting reasons. Cargo eliminates the build-system and dependency-manifest problem rather than specifying a solution to it, which deletes most of what ADR-005 would otherwise need to contain. Rayon makes the tile parallelism of ADR-011 straightforward. Serde makes the JSON persistence of ADR-008 nearly free. The output is a self-contained binary, which matters because the owner cannot repair a broken runtime installation.

One further point worth recording: the architecture in document 06 was written before this decision and specifies immutable snapshots, document revisions, no shared mutable state and cancellation tokens. That design is unusually well matched to Rust ownership semantics, which lowers the friction normally cited against the language.

## Alternatives considered

C++20 with Qt, the version 0.2 proposal. Rejected on failure mode above. Also the heaviest toolchain for a solo developer, and open-source distribution with Qt means permanent LGPL dynamic-linking discipline.

C-sharp with Avalonia. A genuine contender: fastest iteration of the three, excellent tooling, and garbage collection removes a whole class of defects. Rejected on the weaker story for float32 pixel throughput and self-contained native packaging. This was the closest call and is the first fallback if Rust proves to slow delivery materially.

## Consequences

Compile times are slower than the alternatives. Accepted.

Less directly copyable prior art for compositing specifically. Accepted, since the math is specified in document 21 independently of any implementation.

If a G0 spike shows Rust materially slowing delivery, reopen this through document 14 rather than drifting away from it silently.
