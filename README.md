# Anime Compositor

Working title. A cel exposure and finishing compositor for 2D animation, including anime.

**Status: planning. No code exists yet. No test has been run and no performance number has been measured.**

## What it is meant to be

A tool for finishing short animated shots: import numbered PNG cel sequences and a painted background, set exposure timing including deliberate holds, stack and transform layers, preview, and export a deterministic PNG sequence. Offline, no account, no subscription, open source.

It is deliberately not an After Effects replacement. It is the narrower thing that node-based free tools handle badly: drawing-timing-first compositing.

## Where to start

Read in this order:

1. `HANDOFF.md` — session context and what a fresh agent needs to know
2. `Markdown/00_Start_Here.md` — navigation and current gate
3. `Markdown/12_Development_Operating_Guide.md` — how work is verified here, which is unusual and load-bearing
4. `CONTEXT.md` — project vocabulary

## Planned stack

Rust core with rayon for tile-parallel CPU rendering, Tauri with an HTML and CSS interface, Windows 11 x64. Reasoning in `docs/adr/`.

## Repository layout

| Path | Contents |
|---|---|
| `Markdown/` | The 32 planning documents. The only source of truth. |
| `docs/adr/` | Full architecture decision records. |
| `Schemas/` | Draft project schema. |
| `Fixtures/` | Fixture data and expected values. Read-only to implementation work. |
| `design/` | Interface design work. |
| `AGENTS.md`, `CLAUDE.md` | Enforceable rules for coding agents. |

## A note on the name

It is a joke, it references a competitor trademark, and it will not survive to distribution. D-11 in document 14 tracks choosing a real one.
