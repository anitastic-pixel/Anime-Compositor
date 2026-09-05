# Architecture decision records

Version 0.3 | 2026-09-04 | Accepted for baseline

## Purpose and status

This document is the index and summary of architecture decisions. Full records live in `docs/adr/`, written for the decisions where the reasoning matters more than the outcome; the remainder are summarized here only. An ADR may be ACCEPTED, PROVISIONAL, REJECTED or SUPERSEDED. Implementation must not silently contradict an accepted ADR; a conflict is raised and decided, not resolved by whichever side is easier to code.

Version 0.2 left most of these PROVISIONAL pending spikes. Version 0.3 accepts them on reasoned grounds, because the deciding constraint turned out not to be measurable performance but the fact that the owner cannot read code. That constraint favors compilers that reject broken programs, toolchains with no configuration surface, and user interface technology an agent can inspect visually.

## ADR-001 - Initial platform

Status: ACCEPTED. Windows 11 x64 only. Reference machine: Ryzen 9 9900X, 64 GB RAM, RTX 4070 Ti Super 16 GB. No cross-platform support is claimed or designed for. Platform-specific code stays behind narrow interfaces to keep a future port possible, not because one is planned.

## ADR-002 - Product scope order

Status: ACCEPTED. Complete the focused 2D workflow before camera and expression work, and complete G1-core before G1-rest. Each stage must be useful on its own.

## ADR-003 - Core implementation language: Rust

Status: ACCEPTED. The document model, evaluation and renderer are Rust.

The deciding argument is not performance. It is that no human will read this code, so the failure mode to design against is a defect that produces plausible-looking wrong behavior rather than an obvious stop. C++ was rejected for exactly this reason: its characteristic failures are memory corruption and undefined behavior, which surface as intermittent wrong pixels and crashes that a non-programmer owner cannot diagnose and an agent cannot reliably reproduce. Rust converts most of that class into compile-time refusal, which an agent can act on immediately and which never reaches the owner at all.

Secondary reasons: cargo removes the entire build-system and dependency-manifest problem rather than specifying a solution for it; rayon makes the tile parallelism in ADR-011 straightforward; serde makes ADR-008 nearly free; the resulting binary is self-contained, which matters because the owner cannot repair a broken runtime installation.

Accepted cost: compile times, and a smaller pool of copyable prior art for compositing specifically.

## ADR-004 - User interface: Tauri with HTML and CSS

Status: ACCEPTED. The interface is HTML and CSS rendered in WebView2 through Tauri, with the Rust core behind it.

Three reasons decided this over a native Rust immediate-mode toolkit. Design work is transferable: an HTML and CSS design is the product rather than a picture of it, so design effort is not spent twice. The interface is machine-inspectable: an agent can open the running application in a browser context, screenshot it, read its console and inspect its elements, which is the only practical way for the owner to have visual defects investigated. And dense dockable panel layouts, which this application needs, are ordinary work in CSS and difficult in immediate-mode toolkits.

Accepted costs and their bounds: WebView2 is a system component updated outside this project. Frame transport across the Rust-to-webview boundary limits full-resolution playback. Browser color management may alter displayed pixels. The second and third are preview-side only and never touch exported output, because export never passes through the display path. Both are measured by SP-05 and SP-06 before implementation depends on them. If either fails, the fallback is a native rendering surface hosted inside the web shell, which preserves the design and inspection benefits.

## ADR-005 - Build system and dependency policy

Status: ACCEPTED. Cargo, with `Cargo.lock` committed. No separate build system is specified, because Rust does not need one. No network access during normal application use.

Distribution is open source, so dependencies must be license-compatible with it. Every dependency records purpose, version and license before a distributable build. Prefer few, well-maintained crates; a small amount of owned code is preferable to a dependency that must be understood, updated and license-reviewed forever.

## ADR-006 - Rendering backend: CPU only for now

Status: ACCEPTED. G1-core has exactly one renderer: a tile-based multithreaded CPU implementation. No GPU backend is built until a measured result on a real shot on the reference machine shows the CPU path is too slow for the workflow.

Version 0.2 specified a permanent CPU reference path alongside a production GPU path. That is two implementations of every pixel operation maintained indefinitely, and it was proposed for a project with one non-programmer owner. It is removed. Correctness is established against the independent expected values in document 25, not against a second renderer, which is the stronger arrangement anyway: a second implementation by the same author verifies consistency, not correctness.

The tile contract in ADR-011 exists so that adding a GPU path later is a dispatch port rather than a rewrite. Deferred is not rejected.

## ADR-007 - Image and color dependencies

Status: ACCEPTED for G1-core. PNG decoding and encoding via a maintained permissive Rust crate that preserves the metadata required by document 21. OpenColorIO, OpenImageIO and OpenEXR are not adopted; their format coverage is not needed by G1-core and their dependency weight is not justified. Revisit only when a specific requirement, such as EXR handoff, actually arrives.

The internal contract is linear-light premultiplied float32 as specified in document 21, independent of any library choice.

## ADR-008 - Project persistence

Status: ACCEPTED. A versioned, human-inspectable JSON project document validated against `Schemas/project-v0.schema.json`. Media and caches stay external. Writes go to a temporary sibling file, are flushed and closed, validated where practical, then atomically replaced. SP-01 verifies this on the target filesystem.

Inspectable JSON carries additional weight under this project's verification model: it is one of the few places the owner can check behavior directly by opening the file. Unknown additive data is preserved. Unsupported required semantics produce a diagnostic, never a silent reinterpretation.

## ADR-009 - Testing baseline

Status: ACCEPTED, and promoted to the project's primary control. Tests are built from independent fixtures and observable contracts, never from snapshots produced by the implementation under test. Expected values in `Fixtures/` and document 25 are specification and are read-only to implementation work.

Because human code review does not exist here, this ADR is not a quality preference but the mechanism by which the owner knows anything at all. Document 12 specifies the protocol; root `AGENTS.md` makes it enforceable.

## ADR-010 - Distribution model

Status: ACCEPTED. Open source. Decided early rather than late, specifically to remove ongoing license review from a project with no legal reviewer. Dependency choices must preserve compatibility with it.

## ADR-011 - Tile-based render plan

Status: ACCEPTED. New in version 0.3. The unit of render work is a tile: a fixed-size rectangular region composited independently, with the frame divided into tiles distributed across worker threads. Tile results are assembled into the frame; tiles never depend on each other within a single frame evaluation.

This is committed now rather than left as an implementation detail for two reasons. It is the natural shape for CPU parallelism on a 12-core machine, and it is the same decomposition a GPU compute dispatch needs, so it is what makes ADR-006 a deferral rather than a dead end. Retrofitting tiling onto a whole-frame renderer later is a rewrite of the renderer.

Consequences: effects with unbounded spatial support, such as large-radius blur, need explicit tile-margin handling, which is one reason those effects are parked rather than in G1-core. Tile size is a tunable, not a constant, and the correct value is measured on the reference machine rather than assumed.

## ADR-012 - Render trace diagnostics

Status: ACCEPTED. New in version 0.3. The renderer supports a trace mode that writes each intermediate layer buffer to a PNG, tagged with layer ID, frame and stage.

Justification is the verification model: when a composite is wrong, the owner cannot read code and an agent cannot see the screen by default. A directory of intermediate images turns a debugging problem into a looking problem, and can be attached directly to a verification artifact. This is a diagnostic facility, not a performance feature, and must never be on by default.

## ADR-013 - Verification by fixtures and artifacts, not code review

Status: ACCEPTED. New in version 0.3. The owner cannot read code, so human code review does not exist on this project and is never assumed as a backstop. Correctness is established instead by independent fixtures with pre-derived expected values, and by verification artifacts a non-programmer can judge.

The load-bearing control is that expected values in `Fixtures/` and document 25 are read-only to implementation work. Changing one to make a build pass is the single failure this model has no other defense against.

Consequences: full record in `docs/adr/0013-verification-without-code-review.md`, and the operating detail in document 12. The limits are stated there rather than hidden: this model catches wrong results, and does not catch bad architecture, unnecessary complexity or security defects.

## ADR-014 - Narrow G1 to G1-core

Status: ACCEPTED. New in version 0.3. The first milestone was cut from fifteen Must requirements to nine. Masks and mattes (R-04), effects (R-05) and the bounded preview cache (R-06b) move to G1-rest and are parked with written, measurement-based revisit triggers rather than deleted.

Justification: the bottleneck is owner verification time, not code generation speed. A milestone whose scope exceeds the rate at which the owner can actually check results is a milestone that will be accepted unchecked, which is worse than a smaller one finished honestly.

Consequences: parked features stay fully specified, and building one before its trigger fires is forbidden in `AGENTS.md`. Full record in `docs/adr/0014-g1-core-narrowing.md`, triggers in document 23.

## Decision gate

Before B-02 begins, ADR-001 through ADR-014 stand as accepted, and SP-01, SP-03, SP-05 and SP-06 must be recorded. SP-02 is removed with ADR-006. A spike that contradicts an accepted ADR reopens it explicitly through document 14.

Related documents: 06, 10, 12, 14, 21, 25 and 29. Full records in `docs/adr/`.
