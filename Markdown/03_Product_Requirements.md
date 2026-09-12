# Product requirements and acceptance matrix

Version 0.3 | 2026-09-04 | Accepted for baseline

## Requirement contract

C denotes G1-core, the narrowed first milestone. R denotes G1-rest, parked but specified. N denotes the 2.5D stage. L denotes later work.

A requirement is complete only when its observable acceptance condition is demonstrated and its verification artifact is attached, per document 12. A passing test without an artifact the owner has actually looked at does not close a requirement.

Version 0.3 changes the tier of R-04, R-05 and part of R-06 from must-have to parked, under D-12. Nothing was deleted. The parked requirements remain fully specified so they can be promoted without re-planning.

## G1-core: media, time and editing

R-01 / C / Import: group a selected PNG sequence using an explicit numeric pattern; show dimensions, numbering gaps and frame interpretation. T-01; B-03.

R-02 / C / Exposure timing: map composition frames to drawing IDs with explicit holds. A 1-1-2-2-2 pattern must remain unchanged through save, preview and export. This is the central requirement of the product and the one that justifies its existence. T-02; B-04.

R-03 / C / Layers: add, remove, reorder, rename, lock and hide raster layers; animate 2D position, anchor, scale, rotation and opacity with hold, linear and eased interpolation. T-03; B-05. The third mode was added on 2026-09-12 by D-52, at the owner's request after the third sitting with the window, and it is the only widening this requirement has had. An ease is a cubic Bezier of value against time, written as the two inner control points of a curve whose ends are pinned at (0,0) and (1,1) - the shape CSS writes as `cubic-bezier(x1, y1, x2, y2)` and the shape After Effects draws as two handles in its value graph. It belongs to the segment that starts at the keyframe carrying it, exactly as hold and linear do, so nothing about document 20's keyframe model changes shape. **What it does not add is a motion path**: an eased position moves along a straight line between its keys, faster and slower, and a curve *through* the keys in space is a separate decision D-52 leaves open.

R-06a / C / Preview: frame stepping, work-area playback, resolution selection, and a visible indication when preview quality differs from final export. No bounded cache; render on demand and accept the cost. T-06; B-08.

R-07 / C / Editing safety: group each user action into a reversible command; undo and redo must restore values, references and dirty state correctly. T-03 and T-07; B-05 and B-09.

## G1-core: persistence and delivery

R-08 / C / Projects: versioned save, reopen, relative media references, explicit relink and recoverable autosave. An interrupted save must preserve the last successful project. T-07; B-09.

R-09 / C / Export: render a declared inclusive frame range to a PNG sequence with chosen bit depth, naming and alpha policy; report failure and support cancellation between frames. T-08; B-10.

R-10 / C / Color and alpha: explicit input interpretation and a documented working and output path; checkerboard and alpha-only inspection must not alter export. Numeric alpha and color fixtures pass. T-04 and T-09; B-02 and B-10.

R-11 / C / Offline workflow: create, edit, save and export the reference shot without authentication or network connectivity. No project content leaves the device. T-10; B-11.

---

## G1-rest: parked, specified, not being built

R-04 / R / Masks and mattes: one closed polygon mask per layer and an alpha matte referencing another eligible layer, with defined visibility and dependency behavior and cycle rejection. T-04; B-06. Parked under D-12 until 2026-09-06, when its revisit trigger - a real shot the owner cannot finish without a mask - fired and the owner unparked it. The shot is the reference shot, and what it cannot finish is W-01: B-12 / G1 acceptance is defined against W-01 completed on that shot, and W-01 calls for a matte. Document 21 scopes what was unparked: the polygon mask applies in layer/source space at step 2 of the pipeline and the alpha matte in composition space at step 5, both as `C prime = C * m` and `A prime = A * m` on premultiplied values, and the matte layer is evaluated through its own source, mask, effects and transform at the same frame. Document 19 fixes the data: a closed ordered vertex list with enabled and inverted flags, and a matte reference naming a layer in the same composition whose dependency graph must be acyclic. `FX-MATTE-001` in document 25 is the cycle case and must be rejected with `MATTE_CYCLE`. Document 25 line 51 was carried into B-06 rather than settled before it, and B-06 settled it: ADR-016 records the rasterization rule - a 4x4 ordered sample grid, even-odd, coverage exact to one sixteenth - and the reference tool, a second implementation written from document 19 by a different method, so subpixel edge goldens are no longer OPEN. **DELIVERED on 2026-09-06**, in `src/mask.rs` and the mask and matte steps of `src/compose.rs`, with `verification/B-06_mask_table.md` as the artifact: 41 checks, including agreement with that second implementation at every pixel of a 64-pixel field, a mask and its inverse summing back to the unmasked pixel, and the matte arithmetic of document 21 step 5. Two decisions came out of it, both in document 14: D-42 gives the matte reference a `matte_only` flag rather than reading the matte layer's own `enabled: false` that way, and D-43 gives a mask outline this build refuses to draw its own identifier, `MASK_INVALID_OUTLINE`, in document 28.

R-05 / R / Effects: an ordered stack containing exposure, Gaussian blur and solid-color tint, following document 09. T-05; B-07. Parked under D-12 until 2026-09-06, when its revisit trigger - repeated manual effort in finishing real shots - fired and the owner unparked it, at the same time as R-04 and for the same reason: W-01 calls for a blur and a colour operation, and B-12 / G1 acceptance is defined against W-01 completed on the reference shot. The ADR-011 caution is not lifted with the park - blur interacts with the tile margins, which is part of why it was not in the first milestone. **DELIVERED on 2026-09-06**, in `src/effects.rs` and step 3 of `src/compose.rs`, with `verification/B-07_effects_table.md` as the artifact: 60 checks, covering document 25's FX-E-001, FX-E-002, FX-T-001 and FX-T-002, the Gaussian kernel checked against a second implementation written from document 21 by a different method and pinned besides by its sum, its symmetry and its ratio to the centre, and document 25 line 49's single-pixel impulse checked channel by channel across all 1156 channels of the expanded 17x17 result. The comparison this requirement asked for is not the one that was run, and ADR-017 records why: the effect stack is evaluated once over the whole layer buffer while the frame plan is being built, before any tile exists, so no effect declares a tile margin and there is no region-of-interest optimisation for a tiled-versus-full-frame comparison to be about. What was proved instead is the property that would have failed had this been got wrong - the finished frame does not depend on the tile size, checked at six sizes, three of which do not divide the frame. Document 21 line 89's separate requirement, that each effect declare its input bounds expansion, is honoured and checked, including by the alpha centroid of the whole frame, which must not move when a blur enlarges the layer. Three decisions came out of it, all in document 14: D-44 records that effects are not tiled and what replaces the comparison, D-45 registers the fourth effect command document 24 did not list, and D-46 gives `EFFECT_PARAMETER_INVALID` two severities.

R-06b / R / Bounded preview cache: cache reusable results with correct invalidation and a memory ceiling, per document 27. T-06; B-08b. Parked under D-12 until 2026-09-05, when its revisit trigger - measured preview latency on the reference shot that makes editing unpleasant, recorded with numbers rather than asserted - fired in `verification/B-08_preview_latency.md` and the owner unparked it. D-37 and ADR-015 scope what was unparked: the reusable result is a **decoded source cel in the working space**, not a finished frame, because the measurement put decoding at 75.15 ms of an 81.69 ms draft frame and rendering at 6.53 ms; and the cache is confined to the preview path, so no exported sample can depend on it.

## Next-stage requirements

R-12 / N / 2.5D: one perspective camera, flat planes, parent transforms and documented transparency ordering. A reference parallax shot reproduces after reopening. T-11; B-13.

R-13 / N / Expressions: a documented native property-expression subset with deterministic time and seeded randomness, bounded evaluation and cycle errors. T-12; B-14. Runtime undecided per D-10.

R-14 / L / Packaging: collect permitted media with hashes and usage information, and verify reopening from a new path. T-13; B-15.

R-15 / L / Handoff: evaluate EXR and WAV first, then other formats, based on actual need. Each format needs its own conformance fixtures. T-14; B-16.

## Quality requirements

Q-01: no known reproducible project corruption in the release candidate; T-07.

Q-02: preview and export performance measured and reported on the declared reference machine, with no extrapolation to other hardware; T-06.

Q-03: keyboard access documented and tested for the complete W-01 workflow, with readable focus and correct behavior at 100, 150 and 200 percent display scaling; T-15.

Q-04: distributable builds carry a dependency and license record compatible with open-source distribution; T-16.

Q-05, new in version 0.3: every closed requirement has a verification artifact the owner has reviewed, per document 12. A requirement without one is not closed regardless of test status.

Unknown native files, unsupported effects and missing media must produce explicit diagnostics, never a false fidelity claim.

Related documents: 04, 07 through 12, 15 and 23.
