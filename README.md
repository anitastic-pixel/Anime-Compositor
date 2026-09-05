# TotallyNotAfterEffects

Working title. A cel exposure and finishing compositor for 2D animation, including anime.

**Status: G1-core implementation. B-02 through B-05b and B-09 are complete; B-06, export, is next.** G0 passed on 2026-09-04. Planning is complete, the reference shot is drawn, the G0 spikes are measured and recorded, the colour and compositing core is implemented and passing its fixtures, PNG sequences import with gap, Unicode and format diagnostics, the reference shot evaluates to a drawing number per layer per frame at exact rational time, the project model takes every edit through a command interface with exact undo and redo, a tiled multithreaded renderer turns a layer's anchor, position, scale, rotation and opacity into pixels with output that does not depend on tile size or thread count, trace mode writes every intermediate layer buffer of a frame as a tagged PNG so a wrong picture can be diagnosed by looking rather than by reading code, a project can be saved, closed and reopened without losing anything, including the parts of a file this build does not understand, and a layer composites in any of document 21's four blend modes.

Numbers in `Markdown/` are still targets and estimates. Numbers in `spikes/B-01_G0_spike_report.md` and `verification/` are measurements. The two are not interchangeable.

## What it is meant to be

A tool for finishing short animated shots: import numbered PNG cel sequences and a painted background, set exposure timing including deliberate holds, stack and transform layers, preview, and export a deterministic PNG sequence. Offline, no account, no subscription, open source.

It is deliberately not an After Effects replacement. It is the narrower thing that node-based free tools handle badly: drawing-timing-first compositing.

## Where to start

Read in this order:

1. `HANDOFF.md` — session context and what a fresh agent needs to know
2. `Markdown/00_Start_Here.md` — navigation and current gate
3. `Markdown/12_Development_Operating_Guide.md` — how work is verified here, which is unusual and load-bearing
4. `CONTEXT.md` — project vocabulary

If you only want to know whether the thing works, skip all of that and read `verification/`. One
file per completed task, each a table of expected against actual values that can be judged without
reading a line of code. That is deliberate: the owner has no programming background, so evidence
the owner can personally check is the project's primary quality control rather than a nicety.

## Planned stack

Rust core with rayon for tile-parallel CPU rendering, Tauri with an HTML and CSS interface, Windows 11 x64. Reasoning in `docs/adr/`. The toolchain is pinned in `rust-toolchain.toml`; `cargo test` is the whole build.

## Repository layout

| Path | Contents |
|---|---|
| `src/`, `tests/` | The `anime_compositor` crate. Production code. |
| `verification/` | Artifacts the owner reviews, and the scripts that derive expected values independently of the code under test. |
| `Markdown/` | The 32 planning documents. The specification. |
| `docs/adr/` | Full architecture decision records. |
| `docs/DEPENDENCIES.md` | Every crate in the build with version and licence, generated from `Cargo.lock`. |
| `Schemas/` | Draft project schema. |
| `Fixtures/` | Fixture data and expected values. Read-only to implementation work. |
| `design/` | Interface design work. |
| `spikes/` | Quarantined G0 feasibility code. Excluded from the cargo workspace and discarded at integration; never reuse it. |
| `AGENTS.md`, `CLAUDE.md` | Enforceable rules for coding agents. |

## A note on the name

It is a joke, it references a competitor trademark, and it will not survive to distribution. D-11 in document 14 tracks choosing a real one.
