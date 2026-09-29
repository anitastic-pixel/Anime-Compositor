# Core data model contract

Version 0.2 | 2026-09-04 | Proposed baseline

## Authority and principles

This document defines the canonical in-memory concepts for G1. `Schemas/project-v0.schema.json` defines the serialized shape. The implementation may use different internal types only when round-trip behavior is identical. Stable IDs are opaque UUID strings; display names are never identity.

Rules: ownership is explicit; references use IDs rather than object pointers in persistence; editable state is changed only through commands; render workers consume immutable snapshots; absent optional data is distinct from an explicit zero value.

## Project graph

Project owns: schema version, project ID, project settings, assets, compositions, color settings and application metadata. A project may contain multiple compositions even though G1 UI may focus on one at a time.

Composition owns: ID, name, width, height, pixel aspect ratio, frame-rate numerator/denominator, start frame, duration frames, work area, an ordered layer ID list and, by D-58 which the owner accepted on 2026-09-15 and B-13c built, an optional `camera` of three animatable properties - position, depth and zoom - and, by D-171 (proposed), an optional `parent`, a layer of the same composition that the camera rides. An absent camera means the default camera of document 21, not the absence of one. By D-188, which the owner accepted on 2026-09-28 with ADR-019, a composition may carry `motion_blur`, `{"enabled", "shutter_angle", "shutter_phase", "samples"}`: a switch, true or false; an angle in degrees from 0 to 720; a phase in degrees from -360 to 360; and a whole number of samples from 2 to 64. None is animated. Absent means `{false, 180, -90, 16}`, and it is written only when it differs from that. When written, all four are required; a field missing or unknown, a number out of range, samples that are not whole or a switch that is not true or false is `PROJECT_SCHEMA_INVALID` (FX-MB-030 to 038). G1 accepts square pixels only; other ratios produce an unsupported-feature diagnostic.

Asset records media identity and interpretation. G1 asset kinds are `still` and `image_sequence`. Sequence assets store a numeric pattern and a frame-number-to-file map so missing numbers remain missing rather than being silently compacted.

## Layer model

G1 layer kind is `raster`. D-66, accepted on 2026-09-17 and built in B-17b, adds `adjustment`: a layer with no drawing, so no `asset_id`, `exposure_spans` or `source_offset_frames`, and blend mode `normal` only; its effects apply to what is drawn beneath it (document 21). An adjustment layer carrying an asset or another blend mode is `PROJECT_SCHEMA_INVALID`. D-67, accepted on 2026-09-18 and built in B-18b, adds `composition`: a layer whose drawing is another composition of the same project, named by `composition_id`; it has no `asset_id` and no `exposure_spans`, keeps `source_offset_frames`, and has everything else a raster layer has, blend modes included. A composition layer carrying an asset or exposures, or naming no composition, is `PROJECT_SCHEMA_INVALID`; one naming a composition that is not in the project is kept and reported as `COMPOSITION_REFERENCE_MISSING`. D-74, accepted by the owner on 2026-09-19 ("accept D-74, proceed with B-23b") and built in B-23b, adds `solid`: a layer whose drawing is a rectangle of one colour, described by a `solid` record of `color` (three numbers from 0 to 1, linear working-space RGB, not animated) and `width` and `height` (whole pixels, 1 to 8192); it has no `asset_id`, `exposure_spans` or `source_offset_frames`, and everything else a raster layer has, blend modes included. A solid that breaks any of that, or another kind carrying a `solid` record, is `PROJECT_SCHEMA_INVALID`. D-78, accepted by the owner on 2026-09-20 ("accept D-78, proceed with B-25b") and built in B-25b, adds `shape`: a layer whose drawing is a `shapes` list it carries, in the order they are drawn, first to last. It has no `asset_id`, `exposure_spans` or `source_offset_frames`, and **no width or height of its own** - unlike a solid, a shape layer's space is its composition's, the same size and the same origin - and everything else a raster layer has, masks and blend modes included. Each shape is `{"name", "enabled", "closed", "path", "fill", "stroke"}`. The `path` is D-77's mask path unchanged, base and keyframes together, so a shape's path is drawn and moved by the rules a mask's is; `closed` false means the last point is not joined to the first. `fill` is `{"color": [r, g, b], "opacity"}` and `stroke` is `{"color": [r, g, b], "opacity", "width_px"}`, each colour three numbers from 0 to 1 in the working space, each opacity 0 to 1, the width above 0 and at most 8192; either may be absent or null, and a shape with neither is kept and draws nothing. A shape layer carrying an asset, exposures or a source offset; a `shapes` key that is not a list; a shape with no path; a `closed` that is not true or false; a point or handle that is not two numbers; a colour, an opacity or a width outside those ranges; a keyed path whose key holds a different number of points from its base; shapes on an audio layer; or any other kind of layer carrying a `shapes` key - each is `PROJECT_SCHEMA_INVALID`. A shape of fewer than two points is not refused: it is kept, draws nothing and raises `SHAPE_INVALID_OUTLINE` (document 28). FX-SHP-001 to 030 in document 25 are the cases. D-168, proposed on 2026-09-27, lets `fill` or `stroke` carry an optional `gradient`, `{"type": "linear" or "radial", "start", "end", "stops"}`: `start` and `end` are two-number properties, keyed as any property is but with no `spatial` and no `expression`; `stops` is 2 to 64 records of `offset` (0 to 1), `color` (three numbers 0 to 1) and `opacity` (0 to 1). Anything else in a gradient is `PROJECT_SCHEMA_INVALID` (FX-SHP-060 to 069). D-169, proposed on 2026-09-27, lets a shape carry an optional `trim`, `{"start", "end", "offset"}`, each a one-number property keyed as any property is but with no `expression`: `start` and `end` 0 to 100 at the base and at every key, `offset` in degrees and any number. Anything else in a trim is `PROJECT_SCHEMA_INVALID` (FX-SHP-090 to 095). D-170, proposed on 2026-09-27, lets a fill's `color` and `opacity` and a stroke's `color`, `opacity` and `width_px` each be either the plain value or a property record, `{"base", "keyframes"}`, keyed as any property is but with no `expression`, a colour's base and keys each three numbers, every one within D-78's range; and lets a stroke carry `join` (`miter`, `round` or `bevel`), `miter_limit` (1 to 100, 4 when not written) and `cap` (`butt`, `round` or `square`), round and round when not written. Anything else is `PROJECT_SCHEMA_INVALID` (FX-SHP-120 to 127). A raster layer stores ID, name, asset ID, enabled/locked state, in/out frames, source offset, transform, optional exposure map, a list of masks, optional matte reference, blend mode and ordered effect instances.

D-77, accepted by the owner on 2026-09-19 and built in B-24b, is what a mask is. A layer holds `masks`, a list in the order they were drawn, first to last; the single `mask` key it held before is read as one mask and never written again, and a file holding both keys is `PROJECT_SCHEMA_INVALID` because there is no way to tell which of the two the person drew. Each mask holds a `name`, `enabled`, `inverted`, a `mode` of `add`, `subtract`, `intersect`, `difference` or `none`, an `opacity` from 0 to 1, a `feather_px` of 0 or more, an `expansion_px` from -8192 to 8192, and a `path`. The path is a property like any other - `{"base": {"points": [...]}, "keyframes": []}` - so that animating it in B-24d fitted a stopwatch to what was already there. A point is `{"point": [x, y], "in": [dx, dy], "out": [dx, dy]}`, the two handles offsets in pixels from the point as D-53 writes a path, and `[0, 0]` meaning no handle; a mask is closed by definition. A mask of fewer than three points, or one whose points cross, is kept in the file, takes no part in the picture and raises `MASK_INVALID_OUTLINE` (document 28). An unknown mode, an opacity or feather or expansion out of range, a point or handle that is not two numbers, a mask with no path, a `masks` key that is not a list, masks on an audio layer (D-71), or a keyed path whose key holds a different number of points from its base, is `PROJECT_SCHEMA_INVALID`. A key on a path is a frame, a whole outline and an interpolation, written `{"frame": n, "value": {"points": [...]}, "interp": "hold" | "linear" | "ease", "ease": [x1, y1, x2, y2]}`; B-24d, built on 2026-09-20, moves the path between those keys point by point, the point and both of its handles, by document 20's rules. Adding or removing a point on a path that has keys is a command's problem, not a file's: a file whose keys disagree with each other is refused, and a command that would make them disagree is `MASK_INVALID_OUTLINE`. FX-MSK-001 to 030 in document 25 are the cases that stand still, FX-MSK-031 to 035 the ones that move.

Accepted by D-57 on 2026-09-15: an optional `parent`, the ID of another layer in the same composition whose transform is applied after this layer's own (document 21). Absent or null means no parent, and the file carries the field only when it is set. A parent that names no layer is preserved and diagnosed as `PARENT_REFERENCE_MISSING`.

By D-58, accepted by the owner on 2026-09-15 and built by B-13c: an optional `depth`, a scalar property in pixels, absent meaning 0, giving the plane the layer sits on (document 21). It is written only when set, and when written it carries a base and keyframes like every other animatable property. A parented layer's depth is measured from its parent's plane and adds to it up the chain, as its position is a point in its parent's space.

By D-188, accepted on 2026-09-28 with ADR-019: an optional `motion_blur` on a raster, solid, shape or composition layer, `true` when the layer's motion-blur switch is on. It is written only when true, and `false` reads as absent. A value that is not true or false, or the key on a null, adjustment or audio layer, is `PROJECT_SCHEMA_INVALID` (FX-MB-039 to 041).

D-61, accepted on 2026-09-16: an asset may carry `redistribute`, false when the artist may not pass its drawings on, absent otherwise. It changes nothing about how the asset is drawn; collecting a package lists such an asset's drawings without copying them.

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

D-68, accepted on 2026-09-18: a value in an effect's parameter map is either a plain number (for a colour, three), which is constant, or a property record `{"base", "keyframes"}` as a transform property is written above, its keys carrying `frame`, `value`, `interp` and `ease` and never `spatial`. A setting with no keys is written plain, so older files are unchanged. A `base` or key `value` outside the setting's range (`sigma_px` 0 or more, `amount` within 0 and 1) is kept as written and the effect bypassed with `EFFECT_PARAMETER_INVALID`, as D-46 has it for a plain number. An unknown effect's parameter map is kept as written.

D-69, accepted on 2026-09-18: a layer's `position` may be written `{"x": property, "y": property}`, two properties of one number each, in place of `{"base", "keyframes"}`; no key inside it carries `spatial`. Any key may carry `"kind": "continuous"` or `"auto"`, and a position key that is neither first nor last may carry `"roving": true`. Neither changes how the file is evaluated: the `ease` and `frame` written are the ones used, and the two fields say what an edit keeps true of them.

D-189, accepted on 2026-09-28: an effect may have a layer setting. In its parameter map it is a word, the identifier of a layer in the same composition as the layer the effect is on, or `""` for none; it is never keyed and never a property record. Each effect that has one names a fit setting beside it, the word `center`, `stretch` or `tile`. A layer setting naming no layer of that composition, a deleted layer's or another composition's, is kept as written and reported with `EFFECT_LAYER_MISSING`, and the effect is skipped (document 28). Layer settings that lead round in a circle are rejected with `EFFECT_LAYER_CYCLE`; an effect naming its own layer is not one, an edge into an adjustment layer ends there, and every effect counts, switched on or off. D-191, accepted on 2026-09-28, for copies: duplicating a layer renames a layer setting that names the layer itself to the copy's identifier and keeps the others; duplicating a composition renames each layer setting naming one of its layers to that layer's copy; a pasted effect keeps its layer setting as written; a preset saved or exported writes every layer setting as `""`. Compound Blur (`core.compound_blur`) is the first effect with one, `layer`, beside the fit `fit`. Displacement Map (`core.displacement_map`, D-193) is the second, with the same two.

## Validation invariants

- Every referenced ID exists or is retained as an unresolved reference with diagnostics.
- Composition dimensions and duration are positive and bounded by implementation safety limits.
- Frame-rate numerator and denominator are positive, reduced integers.
- Layer `in_frame < out_frame`.
- Exposure spans have `start < end`, are sorted and do not overlap.
- Matte graph is acyclic.
- Parent graph is acyclic, and a parent is a layer in the same composition (D-57).
- Composition graph is acyclic: no composition holds, at any depth, a layer of itself (D-67).
- Camera `zoom` is greater than zero; layer `depth` and camera `depth` are finite (D-58).
- A camera's `parent` is a layer's identifier and not an audio layer's; one not in the composition is kept, with `PARENT_REFERENCE_MISSING` (D-171, proposed).
- A composition's `motion_blur` holds its four fields in range, and a layer's is true or false on a layer that draws (D-188).
- Effect parameter types match the registered effect schema.
- An effect's layer setting names a layer of its own composition, or none, or is kept with `EFFECT_LAYER_MISSING`; layer settings do not lead round in a circle (D-189).
- No serialized path is trusted without normalization and access checks.

## Versioning and migration

`schema_version` is a required integer. Version 0 is the first draft schema in this pack, not a public compatibility promise. A loader must distinguish: supported current version, supported older version requiring migration, and newer/unknown required semantics.

Migrations operate on serialized records before model construction and must be testable in isolation. A migration never depends on UI state. A failed migration leaves the source file unchanged. A file written before D-188 has no `motion_blur` on its compositions or layers and reads as motion blur off; saved again, it still has none (FX-MB-014).

## Effect preset files

D-180. Effect presets are kept by the window, not the project (D-166); a preset file carries them between machines. It is JSON, an object with exactly `preset_file_version` (0, the only version there has been) and `presets`, a list of at least one preset. A preset is an object with exactly `name`, text not empty once its end spaces are taken off and not repeated in the file, and `effects`, a list of at least one effect, each written as a layer's `effects` are, keys and all. The effects are read by the rules pasted effects are read by: an effect type this build does not have, or a field or setting it would not write back, refuses the file. Anything else the rule does not name refuses it too. A file is read whole or refused whole; a refusal changes nothing. A version above 0 is refused, never guessed at. Settings outside their range are read and kept (D-46). Instance ids in a preset carry no meaning: applying a preset gives each effect a new one. Fixtures FX-PRE in document 25.

## Starter presets

D-181. The program comes with six to ten starter presets, written as one preset file of D-180's format, `Fixtures/starter_presets/starter.fxpreset`, which the build carries inside itself. It is read by the rule every preset file is read by, and must read; each effect's settings are in range. They are the window's too but are never stored by it: the window lists them first under Presets, marked built in, and keeps them whatever it stores. They cannot be removed, and a copy is an ordinary preset. No preset of the owner's shares a starter preset's name: saving under one is refused, an import gives the next free number, and a preset the window kept from before with such a name is given the next free number once and the owner told. Export presets writes the owner's presets only. Fixtures FX-STARTER in document 25.

## Colour lookup files

D-182. An asset of kind `lut` is a colour lookup file: `id`, `name`, `path`, stored as a drawing's is, relative to the project file, and `redistribute`, with no interpretation, which it has no use for. No layer shows one: a file with a layer naming one is `PROJECT_SCHEMA_INVALID`, and a command making one is refused. An effect names it: Color Lookup's `lut` is a `lut` asset's id, or empty. Collect Files copies the file as it copies a drawing, to `media/<asset id>/`, and the manifest lists it as kind `lut`, used by each layer whose effect names it. A missing file is `MEDIA_MISSING` on opening and at every frame, and is kept to be relinked; relinking a lookup asset takes one .cube file, read first. A file the rule below refuses is `MEDIA_DECODE_FAILED` at every frame, with the reason.

Reading a .cube file. It is UTF-8 text, else refused as not text; a leading byte-order mark is skipped. Lines end in LF or CRLF; each is trimmed, and blank lines and lines starting with `#` are skipped. A line starting with an ASCII letter is a keyword line, any other a table line. The keywords are `TITLE` (the rest of the line is ignored), `LUT_3D_SIZE N` (a whole number, digits only, 2 to 256), `LUT_1D_SIZE N` (2 to 65536), `DOMAIN_MIN r g b` and `DOMAIN_MAX r g b` (three finite numbers each; 0 0 0 and 1 1 1 when not given), and `LUT_1D_INPUT_RANGE lo hi` or `LUT_3D_INPUT_RANGE lo hi` (two finite numbers, the domain of every channel). Each keyword comes at most once, and before the first table line. A table line is exactly three finite numbers. A 3D table has N³ lines, red changing fastest, then green, then blue; a 1D table has N lines, one column a channel. The file is refused, whole, for: text that is not UTF-8; an unknown keyword; a keyword after the table has begun, or given twice; a size, domain or input range that is not its numbers; a table line that is not three numbers; both sizes, a 1D table before a 3D one, which some grading programs write (a known limit); no size; the domain given more than one way; a domain whose top is not above its bottom in each colour; and the wrong number of table lines. `read_cube` in `tools/cube_lut_reference.py` is the reference, and FX-LUT in document 25 lists each refusal with the reason the build must give, word for word.

## Machine-readable companion

`Schemas/project-v0.schema.json` is the executable validation companion. Example documents live under `Fixtures/projects/`. When this prose and the schema disagree, treat the mismatch as a specification defect and resolve it before implementation rather than choosing one silently.

Related documents: 07, 18, 20, 26 and 28.
