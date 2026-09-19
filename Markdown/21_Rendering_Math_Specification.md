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

## Parenting

Accepted by D-57 on 2026-09-15. Call the transform above `M(L)`. A layer with a parent has the parent's whole transform applied after its own:

`M_world(L) = M_world(parent(L)) * M(L)`, and `M_world(L) = M(L)` for a layer with no parent.

A child's position is therefore a point in its parent's layer space, the space the parent's anchor is measured in. Step 4 of the layer render order below uses `M_world` in place of `M`, and so do the matte layer's own transform and every viewer overlay drawn on a layer. Opacity, masks, effects, mattes, blend mode and exposure are not inherited.

Keeping place, when a parent is set at frame `f`: with `W` the parent's `M_world` at `f`, the child's new position is `W^-1` applied to its position, its new rotation is its rotation less the sum of the rotations along the parent's chain at `f`, and its new scale is its scale divided, component by component, by the product of the scales along that chain at `f`. The anchor is unchanged. Every keyframe value of position, rotation and scale is converted the same way with `W` taken at `f`, and a spatial handle, being an offset, by the linear part of `W^-1` alone. Clearing a parent applies the reverse. This is exact, every point of the layer landing where it was, when every layer in the parent's chain has equal x and y scale, or when neither the child nor any layer in the chain is rotated. Otherwise the exact result is a skew this transform cannot hold: the anchor lands exactly where it was, the rest may shift, and the command reports it. A chain with a zero scale component at `f` has no inverse, and setting the parent is refused.

## Resampling and outside bounds

G1 final-quality transform sampling is bilinear in premultiplied linear RGBA. Samples outside the source extent are transparent black. A future higher-order filter may be added only with independent edge fixtures and explicit bounds rules.

Bilinear weights are computed from source pixel-center coordinates. Sampling must not mix straight RGB across zero-alpha boundaries.

## PNG interpretation

For G1 PNG input, alpha is interpreted as straight/unassociated unless the decoder specification states otherwise. RGB tagged sRGB is converted to linear light before premultiplication. An untagged PNG defaults to sRGB for G1, with an explicit user override reserved for later UI. Embedded profiles/gamma metadata require decoder-specific tests before being claimed as supported.

PNG output converts the linear working RGB to the declared output encoding, then writes straight alpha unless the chosen encoder/format contract explicitly differs. Display transforms are never baked into export unless selected as the output transform.

## EXR interpretation

D-62, accepted on 2026-09-17. EXR input is linear light and premultiplied, the working buffer's own convention, so samples are used without conversion. The drawing is the display window; data outside it is cut off, and display pixels the data window does not cover are transparent black. Colour below 0 and above 1 is kept. A non-finite sample becomes 0, then alpha is clamped to 0..1. Which channels are used, and what is reported, are D-62's.

EXR output writes the working buffer unchanged and premultiplied. Float output is exact. Half output clamps each sample to -65504..65504 and rounds it to the nearest half, ties to even; that is its declared encoding step. No tolerance applies in either direction: the `exr` crate was measured against OpenEXR's own library and matched to the bit (D-62).

## Layer render order

For each raster layer, the G1 order is:

1. Decode the selected source drawing into tagged linear premultiplied RGBA.
2. Apply the layer polygon mask in layer/source space.
3. Evaluate ordered layer effects in layer space.
4. Transform the resulting image into composition space, and project it to the screen through the composition camera (see Camera and depth below, D-58).
5. Apply the referenced alpha matte in composition space.
6. Multiply by animated layer opacity.
7. Composite with the accumulated background using the layer blend mode.

The matte layer is evaluated through its own source, mask, effects and transform at the same frame. When marked matte-only for a dependent layer, it contributes its alpha to that dependency but is not separately composited into the final stack unless another explicit layer instance also displays it.

## Camera and depth

Specified by D-58, accepted by the owner on 2026-09-15 and built by B-13c. Every layer sits on a plane parallel to the
screen at a depth `d` - a scalar property read at the frame being drawn, absent meaning 0 - and a
parented layer's depth adds to its parent's up the
chain: `world_depth(L) = depth(L) + world_depth(parent(L))`.

A composition is seen through a camera with a position, a depth of its own and a zoom, all three
animatable and all three read at the frame being drawn. A composition whose file carries no
camera has the default one: position at the centre of the frame, with depth and zoom both
`width * 50 / 36`, a 50 mm lens on a 36 mm film back (D-58).

For a point already carried into composition space by the transform above:

`s = zoom / (world_depth(L) - camera_depth)`

`p_screen = centre + (p_comp - camera_position) * s`

where `centre` is `(width/2, height/2)`. A plane at the camera's true-size distance has `s = 1`;
one twice as far has `s = 1/2`. The map is a uniform scale about a point followed by a shift, so
it composes with `M_world` into the one matrix step 4 samples through, and adds no second
resampling. **A projection that is the identity must be left out rather than applied**, so that a
composition with the default camera and no depths reproduces earlier results exactly rather than
within a tolerance.

`world_depth(L) - camera_depth` at or below zero means the layer is level with the camera or
behind it: it is not drawn and `CAMERA_PLANE_BEHIND` is reported. A `zoom` at or below zero is
invalid rather than clamped.

Layers are drawn from far to near, and layers at equal depth keep composition order. Depth
therefore overrides the layer stack wherever two layers differ in depth. The order is settled at
every frame rather than once, because a depth can be animated and two planes can cross. A matte layer is
projected at its own depth before its alpha is sampled, so a matte on another plane slides
against the layer it shapes.

## Adjustment layers

D-66, accepted on 2026-09-17 and built in B-17b. An adjustment layer has no drawing. Its shape is an opaque rectangle the size of the composition in its own layer space, taken through steps 2, 4 and 5 above - mask, transform with parent and camera, matte - so its coverage at a pixel is that rectangle's alpha there. Where it comes in the draw order, the frame drawn so far is one picture `B` the size of the frame. Its enabled effects run on `B` in order, as defined under G1 effects below, with samples outside the frame transparent black and anything grown past the frame cut off, giving `E(B)`. Then, with `c` the coverage times the layer's opacity:

`out = B + c*(E(B) - B)`, for all four premultiplied channels.

Blend mode is normal only. A layer with no enabled effects, outside its in and out frames or switched off leaves the frame exactly as it was, bit for bit. The draw order is unchanged: far to near, the stack breaking ties. The effect stack runs on the whole frame; the mix is per pixel. At draft preview size the blur's sigma is divided by the draft divisor. FX-ADJ-001 to 013 in document 25 are the cases.

## Precompositions

D-67, proposed on 2026-09-18 as B-18a. A composition layer's step 1 is not a decode: its source is the composition it names, rendered by this same order at the layer's local frame (document 20), at that composition's own width and height, through that composition's own camera, in linear premultiplied; transparent black outside that composition's own start and length. Steps 2 to 7 then run on that picture exactly as on a decoded drawing, the mask and effects in the inner composition's pixel space, so a blur on the layer blurs the inner picture as one (FX-PRE-002). Nesting is recursion: the inner composition's own composition layers render first, to any depth, and a cycle is `COMPOSITION_CYCLE` before any pixel is drawn. An adjustment layer inside the inner composition sees only what is drawn beneath it there (FX-PRE-009). The same composition rendered for two layers gives the same picture twice (FX-PRE-013). At draft preview size the inner composition is rendered at the draft divisor too. FX-PRE-001 to 015 in document 25 are the cases.

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

Intersecting transparent planes, motion blur, depth of field, HDR display transforms and higher-order resampling are separate contracts. They must not be implied by this G1 specification. G2 camera projection was one of them until D-58, which the owner accepted on 2026-09-15 and B-13c built; it is specified under Camera and depth above, and it deliberately specifies none of the rest - in particular a camera that tilts or turns, which would need a perspective transform rather than the affine one written there.

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
