# CLAUDE.md - Working instructions for Claude Code

Follow `AGENTS.md` first. Read `CONTEXT.md` for project vocabulary. The planning documents in `Markdown/` are the source of truth, especially requirements (03), scope tiers (04), architecture (06), the verification protocol (12), ADRs (18), data/time/render contracts (19-21), fixtures (25), and undo/cache/diagnostics (26-28).

The owner has no programming background and verifies this project through fixture results and visual artifacts, never by reading code. Write for that reality: every completed unit of work must end in something a non-programmer can judge as correct or incorrect. If a change cannot be demonstrated that way, say so rather than declaring it done.

`Fixtures/` expected values are read-only to implementation work. Proposing a change to one is a specification decision, never a step in making a build pass.

Prefer small reviewable diffs. Do not refactor unrelated systems while implementing a requirement. Do not add convenience abstractions until at least two concrete call sites justify them.

For rendering or persistence work, include exact fixture evidence in the completion summary. For performance work, record machine, build and configuration; never invent timing claims.

Unknown or unsupported project, media or effect data must be preserved or explicitly diagnosed per document 28. No silent fidelity fallback.

G0 passed on 2026-09-04 by the owner's decision, recorded in document 00. Current stage is production implementation, starting at B-02. The G0 spikes under `spikes/` remain quarantined and must not be reused.
