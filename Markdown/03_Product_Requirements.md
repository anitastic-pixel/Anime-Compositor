# Product requirements and acceptance matrix

Version 0.3 | 2026-09-04 | Accepted for baseline

## Requirement contract

C denotes G1-core, the narrowed first milestone. R denotes G1-rest, parked but specified. N denotes the 2.5D stage. L denotes later work.

A requirement is complete only when its observable acceptance condition is demonstrated and its verification artifact is attached, per document 12. A passing test without an artifact the owner has actually looked at does not close a requirement.

Version 0.3 changes the tier of R-04, R-05 and part of R-06 from must-have to parked, under D-12. Nothing was deleted. The parked requirements remain fully specified so they can be promoted without re-planning.

## G1-core: media, time and editing

R-01 / C / Import: group a selected PNG sequence using an explicit numeric pattern; show dimensions, numbering gaps and frame interpretation. T-01; B-03.

R-02 / C / Exposure timing: map composition frames to drawing IDs with explicit holds. A 1-1-2-2-2 pattern must remain unchanged through save, preview and export. This is the central requirement of the product and the one that justifies its existence. T-02; B-04.

R-03 / C / Layers: add, remove, reorder, rename, lock and hide raster layers; animate 2D position, anchor, scale, rotation and opacity with hold and linear interpolation. T-03; B-05.

R-06a / C / Preview: frame stepping, work-area playback, resolution selection, and a visible indication when preview quality differs from final export. No bounded cache; render on demand and accept the cost. T-06; B-08.

R-07 / C / Editing safety: group each user action into a reversible command; undo and redo must restore values, references and dirty state correctly. T-03 and T-07; B-05 and B-09.

## G1-core: persistence and delivery

R-08 / C / Projects: versioned save, reopen, relative media references, explicit relink and recoverable autosave. An interrupted save must preserve the last successful project. T-07; B-09.

R-09 / C / Export: render a declared inclusive frame range to a PNG sequence with chosen bit depth, naming and alpha policy; report failure and support cancellation between frames. T-08; B-10.

R-10 / C / Color and alpha: explicit input interpretation and a documented working and output path; checkerboard and alpha-only inspection must not alter export. Numeric alpha and color fixtures pass. T-04 and T-09; B-02 and B-10.

R-11 / C / Offline workflow: create, edit, save and export the reference shot without authentication or network connectivity. No project content leaves the device. T-10; B-11.

---

## G1-rest: parked, specified, not being built

R-04 / R / Masks and mattes: one closed polygon mask per layer and an alpha matte referencing another eligible layer, with defined visibility and dependency behavior and cycle rejection. T-04; B-06. Parked under D-12. Revisit trigger: a real shot the owner cannot finish without a mask.

R-05 / R / Effects: an ordered stack containing exposure, Gaussian blur and solid-color tint, following document 09. T-05; B-07. Parked under D-12. Note that blur interacts with the tile margins of ADR-011, which is part of why it is not in the first milestone. Revisit trigger: repeated manual effort in finishing real shots.

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
