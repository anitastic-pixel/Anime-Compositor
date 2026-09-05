# Rendering, color and performance contracts

Version 0.2 | 2026-09-04 | Proposed baseline

## Proposed evaluation order

For each raster layer: resolve exposure, decode the source, interpret input color/alpha, apply source-space mask, evaluate ordered effects, transform to composition space, apply the referenced matte in composition space, apply layer opacity and blend into the stack. This deliberately defines native behavior; it is not an AE parity claim.

Effects declare bounds expansion and sampling requirements. Blur must request enough neighboring pixels to avoid seams. Matte dependencies use the target's evaluated alpha before final stack blending; a matte-only layer does not also appear in the visible stack unless explicitly enabled. Reject dependency cycles.

## Color and alpha

Use floating-point linear-light working RGB with premultiplied alpha for compositing; proposed initial working primaries are sRGB/Rec.709. Decode straight PNG color to linear before premultiplying. Input interpretation must be explicit when metadata is absent or conflicting.

For normal over, premultiplied output color is Cs + Cd × (1 − As), and alpha is As + Ad × (1 − As). Alpha is coverage and is not gamma-encoded. At zero alpha, guarded unpremultiplication returns zero RGB; do not divide by zero.

The viewer applies a display transform once. Export converts from working space to the chosen output encoding, with explicit straight-alpha PNG output. OpenColorIO documents configurable color roles and transforms [S-03]; adopting OCIO itself remains a dependency decision.

## Blend and look behavior

Define multiply, screen and add with independent fixtures, including partial alpha. Do not substitute opaque-only formulas. G1 uses the declared linear working-space behavior. A later display-referred artistic blend option must have a separate mode and saved semantics, not a silent global change.

---

## Precision and determinism

Use float32 in reference calculations; assess half precision only after image-difference tests. Reject or sanitize non-finite parameter values at the model boundary. Clamp only at documented operations and integer output conversion; retain valid over-range working values.

Time, effect versions, sampling mode and random seeds must be explicit. Compare decoded pixels for deterministic export; file metadata may differ. Bit-identical results across every GPU are not promised. Document numeric tolerances per operation before accepting optimized implementations.

## Cache and scheduling

Cache keys include asset content, exposure frame, upstream revisions, effect versions/parameters, working color configuration, scale, quality and time. Editing a mask or replacing media invalidates downstream results. Cached preview quality must never silently satisfy a final export request.

Use a bounded memory budget and explicit eviction. Cancel obsolete preview work without corrupting shared resources. Prefer responsive controls over completing frames that are no longer requested.

## Provisional performance envelope

Declare the actual test machine before measurements. Use Ani's previously mentioned Ryzen 9 9900X and RTX 4070 Ti Super only if current hardware is confirmed; installed RAM and storage remain OPEN.

Reference fixture: 1080p, 24 fps, 240 frames, ten raster layers, two alpha mattes and three simple effect instances. Proposed targets: warm-cache playback at 24 fps; p95 cached seek-to-display at or below 100 ms; no unbounded memory growth after ten repeated work-area loops. Report dropped frames, cold-render throughput and peak RAM/VRAM.

These are validation targets, not measured capabilities. A 1080p RGBA float32 frame alone is about 31.6 MiB; intermediates and caches multiply that cost. Set initial budgets from observed headroom rather than assuming the entire machine is available.

## G2 addition

Specify camera coordinates, transform order, depth ordering and shutter sampling before adding 2.5D. Keep camera motion sampling independent of held cel selection. Related documents: 06, 07, 09 and 11.

Exact rendering math, coordinate conventions, blend formulas, Gaussian definition and sampling rules are authoritative in 21. Independent numeric tolerances and fixtures are in 25.
