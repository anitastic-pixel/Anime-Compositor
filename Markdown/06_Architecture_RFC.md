# Architecture and technology

Version 0.3 | 2026-09-04 | Accepted for baseline

## Status

The stack is selected. Version 0.2 left it open pending spikes; version 0.3 closes it, because the deciding constraint turned out to be a property of the team rather than of the machine. Details and reasoning are in ADR-003, ADR-004 and ADR-006.

Rust core, rayon for tile parallelism, Tauri with an HTML and CSS interface, CPU-only rendering, offline desktop application on Windows 11 x64. The rendering core stays independent of the interface.

## Module boundaries

The document model owns compositions, assets, layer IDs, properties and serializable edits. The command layer validates changes and owns undo and redo. The interface reads model snapshots and issues commands; it never mutates rendering state or the model directly.

The evaluator turns a composition, a time and a quality request into an acyclic render plan. The renderer executes that plan tile by tile. The media service owns decoded frame access and interpretation. Persistence owns versioning and atomic project writes. The export service evaluates immutable job snapshots.

The same evaluator and the same effect contracts serve preview and export. Interactive scheduling may cancel stale preview requests, but exported frames use a fixed job specification at final quality.

## Dependency direction

The interface depends on commands and public model interfaces. Evaluation depends on the model. The renderer depends on evaluation requests and image buffers. Persistence translates schema records to the model and must not depend on interface code.

Use stable IDs across interfaces. Never pass mutable project references or unvalidated file paths into worker tasks.

---

## The Rust and web boundary

The Rust core is the whole application. The web layer is a view and an input surface, and holds no authoritative state: it renders what the core reports and sends user intent back as commands.

This split is deliberate under the verification model. Everything that determines correctness of output lives in Rust, is covered by fixtures, and never depends on the interface. A defect in the web layer can produce a confusing screen but cannot produce a wrong exported frame.

Frame delivery to the viewer is the one performance-sensitive crossing. Naive per-frame serialization is expected to be too slow at full resolution; SP-05 measures what is achievable and selects the transport. Draft-scale scrubbing and frame stepping are expected to be comfortable regardless.

## Concurrency and ownership

One controlled command path commits document edits. Workers consume immutable snapshots. Each preview request carries a document revision and a cancellation token, and the viewer discards results for obsolete revisions.

Within a frame, tiles are independent and composited in parallel across worker threads by rayon. Tile count and size are tunable and measured on the reference machine, not assumed.

Export runs against a captured revision and media manifest, independent of live editing. A read-only snapshot plus an export worker is sufficient; a separate process is not planned.

## Core request contracts

FrameRequest: composition ID, rational time, project revision, quality, render scale and output interpretation. FrameResult: image handle, extent, alpha mode, color-space tag, warnings and provenance key. An untagged buffer never crosses a module boundary.

CommandResult: accepted or rejected, new revision, reversible change and diagnostics. LoadResult: parsed version, model or recoverable errors, unsupported fields and missing media. ExportResult: job status, completed frame list and error details.

## G0 spikes

SP-01: save and reopen a minimal document; interrupt the save and verify the previous file survives intact on the target filesystem.

SP-03: present a composited 1080p frame while scrubbing a small timeline, and record the interaction latency.

SP-04: render a fixed sequence twice and compare decoded pixels byte for byte, establishing determinism.

SP-05, new in version 0.3: measure frame transport from the Rust core into WebView2 at full and draft resolution, and record achievable preview frame rate. This is the primary technical risk of ADR-004.

SP-06, new in version 0.3: display a known fixture value through the webview and verify the displayed pixels are byte-exact against document 25. This is the color-correctness risk of ADR-004, and it matters because the product promise is predictable pixels.

SP-02 is removed. It compared a CPU reference path against a GPU path; ADR-006 leaves only one path.

Record toolchain, OS, driver, dependency versions, timings and observed defects for every spike. Write the minimum code that answers the question and quarantine it before production integration.

## Decision rule for future changes

Reject any candidate that cannot satisfy data safety, color correctness, license compatibility with open-source distribution, or reproducible local builds. Among viable candidates, prefer the one that produces fewer failure modes the owner cannot diagnose. That criterion, not raw performance, selected the current stack and should decide future changes as well.

Related documents: 07 through 12, 14, 18, 21 and 29.
