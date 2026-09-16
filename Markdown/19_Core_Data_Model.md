# Core data model contract

Version 0.2 | 2026-09-04 | Proposed baseline

## Authority and principles

This document defines the canonical in-memory concepts for G1. `Schemas/project-v0.schema.json` defines the serialized shape. The implementation may use different internal types only when round-trip behavior is identical. Stable IDs are opaque UUID strings; display names are never identity.

Rules: ownership is explicit; references use IDs rather than object pointers in persistence; editable state is changed only through commands; render workers consume immutable snapshots; absent optional data is distinct from an explicit zero value.

## Project graph

Project owns: schema version, project ID, project settings, assets, compositions, color settings and application metadata. A project may contain multiple compositions even though G1 UI may focus on one at a time.

Composition owns: ID, name, width, height, pixel aspect ratio, frame-rate numerator/denominator, start frame, duration frames, work area, an ordered layer ID list and, by D-58 which the owner accepted on 2026-09-15 and B-13c built, an optional `camera` of three animatable properties - position, depth and zoom. An absent camera means the default camera of document 21, not the absence of one. G1 accepts square pixels only; other ratios produce an unsupported-feature diagnostic.

Asset records media identity and interpretation. G1 asset kinds are `still` and `image_sequence`. Sequence assets store a numeric pattern and a frame-number-to-file map so missing numbers remain missing rather than being silently compacted.

## Layer model

G1 layer kind is `raster`. A raster layer stores ID, name, asset ID, enabled/locked state, in/out frames, source offset, transform, optional exposure map, optional polygon mask, optional matte reference, blend mode and ordered effect instances.

Accepted by D-57 on 2026-09-15: an optional `parent`, the ID of another layer in the same composition whose transform is applied after this layer's own (document 21). Absent or null means no parent, and the file carries the field only when it is set. A parent that names no layer is preserved and diagnosed as `PARENT_REFERENCE_MISSING`.

By D-58, accepted by the owner on 2026-09-15 and built by B-13c: an optional `depth`, a scalar property in pixels, absent meaning 0, giving the plane the layer sits on (document 21). It is written only when set, and when written it carries a base and keyframes like every other animatable property. A parented layer's depth is measured from its parent's plane and adds to it up the chain, as its position is a point in its parent's space.

Proposed by D-61 on 2026-09-16: an asset may carry `redistribute`, false when the artist may not pass its drawings on, absent otherwise. It changes nothing about how the asset is drawn; collecting a package lists such an asset's drawings without copying them.

Layer order is composition order. Index is not identity. Reordering must not rewrite layer IDs or references.

By D-59, accepted by the owner on 2026-09-16: any layer transform property, a layer's depth and any of the camera's three properties may carry an optional `expression`, an object of `text` and `enabled`, written only when present. A switched-off expression is preserved exactly and ignored. The language is document 09's.

Transform contains anchor, position, scale, rotation and opacity properties. Scale is percentage-like in UI but serialized as explicit numeric pairs. Position and anchor use pixels. Rotation uses degrees. Opacity uses normalized 0..1 in the model.

## Property and keyframe model

A Property<T> has a base value and zero or more keyframes. G1 serializes keyframe frame indices as signed integers and supports interpolation values `hold`, `linear` and `ease`. Duplicate keyframes for the same property at the same frame are invalid.

`ease` was added on 2026-09-12 by D-52 and is the only interpolation value added since version 0.3. A keyframe whose `interp` is `ease` carries one further field, `ease`, and a keyframe whose `interp` is anything else must not carry it:

```json
{ "frame": 12, "value": [300, -40], "interp": "ease", "ease": [0.3333333333333333, 0, 0.6666666666666666, 1] }
```

The four numbers are the two inner control points of a cubic Bezier, `[x1, y1, x2, y2]`, of a curve whose ends are pinned at (0,0) and (1,1). They are written in the file in that order and in no other. **`x1` and `x2` must each be within 0 and 1 inclusive**, because they are positions in time within the segment and a handle outside it would make the curve fold back on itself and give one frame two values; a file whose `x` is outside that range is invalid and is diagnosed rather than clamped. **`y1` and `y2` are not bounded**, and that is deliberate: a `y` outside 0 and 1 is an overshoot, the curve going past its destination and coming back, which is a thing an animator means to do. The pair `[1/3, 1/3, 2/3, 2/3]` is the curve that is exactly linear, and `[1/3, 0, 2/3, 1]` is After Effects' easy ease.

A keyframe's `ease` describes the segment that begins at that keyframe, which is what document 20 already says about `interp`. It therefore has no effect on the last keyframe of a property, where there is no next keyframe and no segment; the field is preserved there rather than dropped, because an artist who moves a key back into the middle of a run expects the ease they set to still be on it.

`spatial` was added on 2026-09-12 by D-53 and is the motion path. It is optional, it appears only on keyframes of `position`, and it is independent of `interp`: a segment may be held, linear or eased and curved or straight in any combination, because the two describe different things.

```json
{ "frame": 12, "value": [300, -40], "interp": "linear", "spatial": [-60, 0, 80, 25] }
```

The four numbers are the keyframe's two handles as offsets in composition pixels from the keyframe's own value, `[in_x, in_y, out_x, out_y]`, written in that order and in no other. The incoming handle belongs to the segment arriving at this keyframe and the outgoing handle to the segment leaving it, so one segment is drawn from the outgoing handle of the key it starts at and the incoming handle of the key it ends at. **None of the four is bounded**, unlike the `x` of an `ease`, and the difference is deliberate: a handle outside its own segment in *time* would give one frame two values, while a handle longer than its own segment in *space* makes the layer loop back on itself, which is a thing an animator means to do.

**An absent `spatial` is the straight line**, and the handles that produce the straight line are at the thirds - `out` is one third of the way to the next key, `in` is one third of the way back to the previous one - exactly as `[1/3, 1/3, 2/3, 2/3]` is the `ease` that is exactly linear. Handles of `[0, 0, 0, 0]` are a different segment and not the default: they travel the same straight line at an easy-ease speed. A file written before D-53 therefore reads as what it already was, and every fixture predating it holds unchanged.

A `spatial` on a property other than `position` is invalid and is diagnosed rather than dropped, exactly as an out-of-range `ease` is and under the same `PROJECT_SCHEMA_INVALID`; document 28 gains no new identifier, because this is a file that does not say a thing rather than a feature this build declines to draw. On the last keyframe of a property the outgoing handle has no segment and no effect, and on the first the incoming handle has none; both are preserved rather than dropped, for the reason given above about `ease`.

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
- Parent graph is acyclic, and a parent is a layer in the same composition (D-57).
- Camera `zoom` is greater than zero; layer `depth` and camera `depth` are finite (D-58).
- Effect parameter types match the registered effect schema.
- No serialized path is trusted without normalization and access checks.

## Versioning and migration

`schema_version` is a required integer. Version 0 is the first draft schema in this pack, not a public compatibility promise. A loader must distinguish: supported current version, supported older version requiring migration, and newer/unknown required semantics.

Migrations operate on serialized records before model construction and must be testable in isolation. A migration never depends on UI state. A failed migration leaves the source file unchanged.

## Machine-readable companion

`Schemas/project-v0.schema.json` is the executable validation companion. Example documents live under `Fixtures/projects/`. When this prose and the schema disagree, treat the mismatch as a specification defect and resolve it before implementation rather than choosing one silently.

Related documents: 07, 18, 20, 26 and 28.
