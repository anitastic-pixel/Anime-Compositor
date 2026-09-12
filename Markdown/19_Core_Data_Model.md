# Core data model contract

Version 0.2 | 2026-09-04 | Proposed baseline

## Authority and principles

This document defines the canonical in-memory concepts for G1. `Schemas/project-v0.schema.json` defines the serialized shape. The implementation may use different internal types only when round-trip behavior is identical. Stable IDs are opaque UUID strings; display names are never identity.

Rules: ownership is explicit; references use IDs rather than object pointers in persistence; editable state is changed only through commands; render workers consume immutable snapshots; absent optional data is distinct from an explicit zero value.

## Project graph

Project owns: schema version, project ID, project settings, assets, compositions, color settings and application metadata. A project may contain multiple compositions even though G1 UI may focus on one at a time.

Composition owns: ID, name, width, height, pixel aspect ratio, frame-rate numerator/denominator, start frame, duration frames, work area and an ordered layer ID list. G1 accepts square pixels only; other ratios produce an unsupported-feature diagnostic.

Asset records media identity and interpretation. G1 asset kinds are `still` and `image_sequence`. Sequence assets store a numeric pattern and a frame-number-to-file map so missing numbers remain missing rather than being silently compacted.

## Layer model

G1 layer kind is `raster`. A raster layer stores ID, name, asset ID, enabled/locked state, in/out frames, source offset, transform, optional exposure map, optional polygon mask, optional matte reference, blend mode and ordered effect instances.

Layer order is composition order. Index is not identity. Reordering must not rewrite layer IDs or references.

Transform contains anchor, position, scale, rotation and opacity properties. Scale is percentage-like in UI but serialized as explicit numeric pairs. Position and anchor use pixels. Rotation uses degrees. Opacity uses normalized 0..1 in the model.

## Property and keyframe model

A Property<T> has a base value and zero or more keyframes. G1 serializes keyframe frame indices as signed integers and supports interpolation values `hold`, `linear` and `ease`. Duplicate keyframes for the same property at the same frame are invalid.

`ease` was added on 2026-09-12 by D-52 and is the only interpolation value added since version 0.3. A keyframe whose `interp` is `ease` carries one further field, `ease`, and a keyframe whose `interp` is anything else must not carry it:

```json
{ "frame": 12, "value": [300, -40], "interp": "ease", "ease": [0.3333333333333333, 0, 0.6666666666666666, 1] }
```

The four numbers are the two inner control points of a cubic Bezier, `[x1, y1, x2, y2]`, of a curve whose ends are pinned at (0,0) and (1,1). They are written in the file in that order and in no other. **`x1` and `x2` must each be within 0 and 1 inclusive**, because they are positions in time within the segment and a handle outside it would make the curve fold back on itself and give one frame two values; a file whose `x` is outside that range is invalid and is diagnosed rather than clamped. **`y1` and `y2` are not bounded**, and that is deliberate: a `y` outside 0 and 1 is an overshoot, the curve going past its destination and coming back, which is a thing an animator means to do. The pair `[1/3, 1/3, 2/3, 2/3]` is the curve that is exactly linear, and `[1/3, 0, 2/3, 1]` is After Effects' easy ease.

A keyframe's `ease` describes the segment that begins at that keyframe, which is what document 20 already says about `interp`. It therefore has no effect on the last keyframe of a property, where there is no next keyframe and no segment; the field is preserved there rather than dropped, because an artist who moves a key back into the middle of a run expects the ease they set to still be on it.

Supported G1 property value types: scalar, vec2, color4 and boolean where appropriate. Color4 is linear RGBA in model/evaluation code; UI color pickers may present display-referred values but must convert explicitly.

Before the first keyframe, evaluation returns the first keyframe value. After the last, it returns the last. With no keyframes, evaluation returns the base value. Exact time rules are in 20.

## Exposure model

ExposureSpan stores `start_frame`, `end_frame_exclusive`, and a source `drawing_number`. Spans are sorted, non-overlapping and half-open. A gap is permitted and means no drawing is exposed. A hold is represented by one span covering multiple composition frames, never by duplicating media records.

Source offset does not rewrite exposure spans. Evaluation derives the requested source drawing from layer-local frame plus the explicit exposure map.

## Masks, mattes and effects

G1 PolygonMask stores an ordered list of vec2 vertices, closed by definition, plus enabled/inverted flags. Self-intersection behavior is unsupported in G1 and must be rejected or normalized only through an explicit command.

MatteReference stores another layer ID and mode `alpha`. A matte dependency must refer to a layer in the same composition and the dependency graph must be acyclic. Matte-only visibility behavior is defined in 21.

EffectInstance stores stable instance ID, effect type ID, enabled flag and a typed parameter map. Unknown effect records must survive project load/save where feasible but render as unsupported with an explicit warning; they may not be silently discarded.

## Validation invariants

- Every referenced ID exists or is retained as an unresolved reference with diagnostics.
- Composition dimensions and duration are positive and bounded by implementation safety limits.
- Frame-rate numerator and denominator are positive, reduced integers.
- Layer `in_frame < out_frame`.
- Exposure spans have `start < end`, are sorted and do not overlap.
- Matte graph is acyclic.
- Effect parameter types match the registered effect schema.
- No serialized path is trusted without normalization and access checks.

## Versioning and migration

`schema_version` is a required integer. Version 0 is the first draft schema in this pack, not a public compatibility promise. A loader must distinguish: supported current version, supported older version requiring migration, and newer/unknown required semantics.

Migrations operate on serialized records before model construction and must be testable in isolation. A migration never depends on UI state. A failed migration leaves the source file unchanged.

## Machine-readable companion

`Schemas/project-v0.schema.json` is the executable validation companion. Example documents live under `Fixtures/projects/`. When this prose and the schema disagree, treat the mismatch as a specification defect and resolve it before implementation rather than choosing one silently.

Related documents: 07, 18, 20, 26 and 28.
