# Project, media and time specification

Version 0.2 | 2026-09-04 | Proposed baseline

## Proposed project schema

Use a versioned, inspectable structured format for G1; JSON is the initial candidate, with a schema and migrations. Do not embed media or cache in the project file. Fields include schema_version, project_id, compositions, assets, layers, effects, color_settings and application_metadata.

A composition stores stable ID, pixel dimensions, pixel aspect ratio, frame-rate numerator/denominator, start frame and duration. G1 supports square pixels only and rejects unsupported ratios explicitly. A layer references an asset and stores order, timing, transforms, visibility, masks, matte references and effect instances.

An effect instance stores stable instance ID, effect type, contract version, enabled state and typed parameters. References must use IDs rather than display names. Unknown effect records are preserved and surfaced as unsupported.

## Exact time model

Represent frame rate as a rational, including 24/1 and a tested non-integer rate such as 24000/1001. Use integer frame indices for exposure spans and rational composition time for evaluation. Do not accumulate floating-point frame durations to find the next frame.

Exposure spans use half-open intervals: start inclusive, end exclusive. Export UI ranges are inclusive at both ends and convert explicitly to an internal interval. For example, export 0 through 239 contains 240 frames.

A drawing exposure references an asset frame ID. Holding one drawing for three frames is different from interpolating between images. Frame labels may start at 1 in imported filenames while composition indexing starts at 0; preserve both explicitly.

## Asset records

Store media type, normalized relative path where possible, original sequence pattern, explicit frame list, dimensions, input color interpretation and straight/premultiplied alpha. Keep a content fingerprint for invalidation; file timestamps alone are insufficient for exact identity.

---

## Save, load and recovery

Validate before writing. Write a temporary sibling file, flush as supported, then use the platform's tested replacement mechanism. Keep the last valid project or backup until the new save succeeds. Atomicity and durability behavior must be verified on the supported filesystem; do not assume every filesystem has identical guarantees.

Autosaves use separate recovery files and must not overwrite the last manual save. Proposed default: save a recovery snapshot after two minutes of dirty activity, retain five rotating snapshots and pause safely while another save commits. Make these settings explicit and measure write cost.

On load, parse with size/depth limits, validate IDs and references, then apply migrations in memory. Back up before writing a migrated project. Newer unsupported major versions open read-only or fail clearly; never silently downgrade them.

## Relinking and changed media

Search only user-selected locations. Show candidate sequence, dimensions, range and interpretation before relink. Preserve layer IDs and effects. A changed asset fingerprint invalidates dependent decoded frames and downstream render cache. A missing drawing does not shift subsequent exposure indices.

Default missing-frame behavior is an explicit error placeholder in preview and a blocked final export. Any hold-last or transparent-frame override must be a recorded project choice and appear in export warnings.

## Interchange contract

G1: PNG stills and sequences in/out, with explicit output color and alpha handling. Initial project handoff uses the project plus a preserved relative media tree. Automated collection, EXR, WAV and layered artwork are later features with separate fixture requirements.

A future AE importer or translator must retain an unsupported-feature report and never advertise a lossless conversion without evidence. Do not conflate native project portability with proprietary project compatibility.

## Format review gate

Before B-09, review example files covering a clean project, missing media, unknown effect, two compositions, Unicode paths and an older schema. Validate frame counts and cycles before any render. Related documents: 03, 06, 08, 10 and 11.

Machine-readable schema: `Schemas/project-v0.schema.json`. Canonical entity/invariant rules are in 19 and exact time semantics are in 20. Example project fixtures are under `Fixtures/projects/`.
