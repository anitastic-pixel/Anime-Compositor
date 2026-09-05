# Rendering math and sampling specification

Version 0.2 | 2026-09-04 | Proposed baseline

## Reference representation

The correctness reference buffer is RGBA float32 in linear light with premultiplied RGB. Alpha is a linear coverage/opacity scalar in 0..1. Intermediate math may exceed 0..1 where an effect requires it; final integer output conversion clamps only at the declared encoding step.

Transparent black is `(0,0,0,0)`. Whenever straight color must be recovered from a premultiplied pixel with alpha zero, define straight RGB as zero to avoid NaN/Inf propagation.

## Coordinate system

Composition and layer coordinates use origin at the top-left, +x to the right and +y downward. Pixel `(i,j)` has center `(i+0.5, j+0.5)`. Geometry is expressed in continuous pixel coordinates.

For a layer point p, the 2D transform is:

`p_comp = T(position) * R(rotation) * S(scale/100) * T(-anchor) * p_layer`

Position denotes where the anchor lands in composition coordinates. Positive rotation is clockwise in the screen-coordinate system. Renderer sampling uses the inverse transform from destination pixel center to source space.

## Resampling and outside bounds

G1 final-quality transform sampling is bilinear in premultiplied linear RGBA. Samples outside the source extent are transparent black. A future higher-order filter may be added only with independent edge fixtures and explicit bounds rules.

Bilinear weights are computed from source pixel-center coordinates. Sampling must not mix straight RGB across zero-alpha boundaries.

## PNG interpretation

For G1 PNG input, alpha is interpreted as straight/unassociated unless the decoder specification states otherwise. RGB tagged sRGB is converted to linear light before premultiplication. An untagged PNG defaults to sRGB for G1, with an explicit user override reserved for later UI. Embedded profiles/gamma metadata require decoder-specific tests before being claimed as supported.

PNG output converts the linear working RGB to the declared output encoding, then writes straight alpha unless the chosen encoder/format contract explicitly differs. Display transforms are never baked into export unless selected as the output transform.

## Layer render order

For each raster layer, the G1 order is:

1. Decode the selected source drawing into tagged linear premultiplied RGBA.
2. Apply the layer polygon mask in layer/source space.
3. Evaluate ordered layer effects in layer space.
4. Transform the resulting image into composition space.
5. Apply the referenced alpha matte in composition space.
6. Multiply by animated layer opacity.
7. Composite with the accumulated background using the layer blend mode.

The matte layer is evaluated through its own source, mask, effects and transform at the same frame. When marked matte-only for a dependent layer, it contributes its alpha to that dependency but is not separately composited into the final stack unless another explicit layer instance also displays it.

## Mask and matte math

Mask coverage `m` in 0..1 multiplies both premultiplied RGB and alpha. G1 polygon edges use the same deterministic rasterization rule in CPU reference and production backend; multisample details must be fixture-tested before claiming subpixel equivalence.

Alpha matte coverage is the matte layer's post-transform alpha sampled at the destination pixel. Apply `C'=C*m` and `A'=A*m`.

## Normal composite

For premultiplied source `(Cs, As)` over destination `(Cd, Ad)`:

`Co = Cs + Cd*(1-As)`

`Ao = As + Ad*(1-As)`

This is the authoritative B-02 reference formula.

## Blend modes

For multiply, screen and add, first recover straight colors `cs` and `cd` where alpha is nonzero. Define the blend function B component-wise:

- multiply: `B = cs * cd`
- screen: `B = cs + cd - cs*cd`
- add: `B = min(1, cs + cd)` for the bounded G1 display blend

Then use the premultiplied source-over blend equation:

`Co = (1-As)*Cd + (1-Ad)*Cs + As*Ad*B(cs,cd)`

`Ao = As + Ad - As*Ad`

Zero-alpha straight colors are zero. Independent fixtures in 25 must verify each mode before the GPU implementation is accepted.

## G1 effects

Exposure: parameter is stops `e`; linear premultiplied RGB is multiplied by `2^e`; alpha is unchanged.

Solid-color tint: parameter color is linear RGB and amount `t` in 0..1. Recover straight source RGB where alpha > 0, compute `mix(source_rgb, tint_rgb, t)`, then premultiply by original alpha. Alpha is unchanged.

Gaussian blur: parameter `sigma_px >= 0`. Use separable normalized Gaussian weights with kernel radius `ceil(3*sigma_px)`. Sigma zero is identity. Samples outside the image are transparent black. Blur operates on premultiplied RGB and alpha together to avoid dark/bright fringe artifacts. Bounds expand by the kernel radius.

## Evaluation bounds and ROI

Every effect declares input bounds expansion. Transform/mask/matte operations declare the region they can affect. Cache keys include all parameters that alter pixels or bounds. An optimization may skip pixels outside ROI only if output equals full-frame reference math within test tolerance.

## Numeric tolerance

CPU reference scalar tests should use exact/near-exact float comparisons appropriate to the operation. GPU comparison tolerance begins at absolute channel error <= 1e-5 for simple arithmetic fixtures and requires per-effect declared tolerances for filters. A tolerance increase requires evidence, not convenience.

## Display pipeline

Viewer checkerboard, alpha-only display, overlays, selection outlines and draft-resolution indicators are presentation layers. They must not alter cached final pixels or export results.

## Deferred rendering questions

G2 camera projection, intersecting transparent planes, motion blur, depth of field, HDR display transforms and higher-order resampling are separate contracts. They must not be implied by this G1 specification.

Related documents: 08, 18, 20, 25 and 27.

---

## Tile contract (added in version 0.3, ADR-011)

The unit of render work is a tile: a fixed-size rectangular region of the output frame, composited independently of every other tile in that frame. A frame evaluation divides the output extent into tiles and distributes them across worker threads. Tiles within one frame evaluation never depend on one another.

All math specified above is defined per pixel and is therefore unchanged by tiling. Tiling changes only the order and grouping of evaluation, and must not change results: a tiled render and a hypothetical whole-frame render of the same request must be byte-identical, and B-05a proves this rather than assuming it.

Tile size is a tunable measured on the reference machine, not a constant chosen in advance. Too small wastes scheduling overhead; too large starves threads at the frame edges.

### Spatial support and margins

An operation whose output pixel depends only on the corresponding input pixel is tile-safe without qualification. This covers transforms sampled per output pixel, blending, opacity and color conversion, which is the whole of G1-core.

An operation whose output pixel depends on a neighborhood, such as blur, requires the tile to be evaluated with a margin of the operation's radius, then cropped. The margin is part of the operation's contract and must be declared by the operation rather than assumed by the renderer. An operation with unbounded support cannot be tiled without an explicit strategy and must not be added casually.

This is a direct reason R-05 sits in G1-rest: the blur in that requirement is the first operation that will exercise margin handling, and it deserves its own fixtures rather than arriving inside a milestone that has none.

### Relationship to a future GPU path

The tile decomposition is also the decomposition a GPU compute dispatch needs. Committing to it now is what makes ADR-006, which defers GPU work entirely, a deferral rather than a dead end: adding a GPU path later becomes a port of tile execution, not a rewrite of the renderer.

This is the one piece of forward design in the pack that is not justified by present need, and it is accepted deliberately, because retrofitting tiling onto a whole-frame renderer would be substantially more expensive than adopting it from the start.

### Determinism

Tile results must not depend on the number of worker threads, the order of completion, or scheduling. Two renders of the same request on the same build produce identical bytes, which SP-04 establishes and B-10 re-verifies against the exported sequence.
