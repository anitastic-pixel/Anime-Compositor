# Test fixture catalog and tolerances

Version 0.2 | 2026-09-04 | Proposed baseline

## Fixture authority

This catalog binds test IDs to independent inputs, expected behavior and tolerances. The machine-readable subset is `Fixtures/fixture_manifest.json`. A production implementation may generate additional cases, but passing self-generated snapshots alone is insufficient.

## Numeric fixtures

| Fixture ID | Purpose | Input | Expected result / rule | Tolerance |
|---|---|---|---|---|
| FX-A-001 | transparent source-over | S=(0,0,0,0), D=(0.2,0.4,0.6,1) | D unchanged | 1e-7 CPU |
| FX-A-002 | opaque source-over | S=(0.8,0.1,0.2,1), any D | S | 1e-7 CPU |
| FX-A-003 | partial alpha over opaque | straight S=(1,0,0), As=.5; D=(0,0,1), Ad=1 | premul output=(.5,0,.5,1) | 1e-6 CPU |
| FX-A-004 | zero-alpha unpremultiply | C=(0,0,0), A=0 | straight RGB=(0,0,0), finite | exact |
| FX-B-001 | multiply opaque | red over 50% gray | B=(.5,0,0), A=1 | 1e-6 |
| FX-B-002 | screen opaque | .5 gray over .5 gray | .75 gray, A=1 | 1e-6 |
| FX-B-003 | add opaque | .7 + .6 gray | 1.0 clamped, A=1 | 1e-6 |
| FX-E-001 | exposure identity | e=0 | unchanged | 1e-7 |
| FX-E-002 | exposure +1 | RGB=.25, e=1 | RGB=.5; alpha unchanged | 1e-6 |
| FX-T-001 | tint zero | amount=0 | unchanged | 1e-7 |
| FX-T-002 | tint full | amount=1, alpha=.5 | straight RGB=tint; premultiplied by .5 | 1e-6 |

## Time fixtures

FX-TIME-001: exposure spans map composition frames 0..4 to drawing numbers `[1,1,2,2,2]` exactly through save, preview and export.

FX-TIME-002: sequence numbers `[1001,1002,1004]` with a requested 1003 produce `MEDIA_SEQUENCE_GAP`/missing drawing, not substitution.

FX-TIME-003: 24000/1001 remains the exact stored rate after round-trip; no replacement with decimal 23.976 authority.

FX-TIME-004: composition `start_frame=-12`, `duration=24` has valid frames -12 through 11 and exactly 24 export frames.

## Ease fixtures

Added on 2026-09-12 by D-52. Each case is one segment: two keyframes, the four numbers of the curve on the first of them, and the value expected at each of a handful of frames. The expected values are written out here rather than in the machine-readable manifest, because a number a person can read beside the case it belongs to is the point of this document, and because the build's own table prints the same rows for comparison.

**Every number below is produced by `tools/ease_reference.py`**, which solves document 20's equation by bisection, in another language, written from document 20 rather than from the build. Running that file prints this table. The build solves the same equation by Newton's method with a bisection fallback, so an agreement between the two is two independent answers and not one answer twice.

FX-EASE-001: the curve `[1/3, 1/3, 2/3, 2/3]` evaluates as linear, to 1e-9. An implementation that rounds, clamps or shortcuts the solve fails this one first. A scalar from 0 at frame 0 to 120 at frame 24.

| frame | 0 | 1 | 6 | 12 | 18 | 23 | 24 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| value | 0 | 4.999999999999999 | 30 | 60 | 90 | 115 | 120 |

Frame 1 is not exactly 5, and that is the tolerance earning its place: the true answer is 5, the bisection lands one unit in the last place below it, and 1e-9 is far wider than the gap. A build that printed exactly 5 would also pass.

FX-EASE-002: easy ease, `[1/3, 0, 2/3, 1]`, on a scalar from 0 at frame 0 to 100 at frame 24. Tolerance 1e-6. The midpoint is exactly 50, and the whole row can be checked by hand against `100 * (3u^2 - 2u^3)`, which is what document 20 says this curve reduces to.

| frame | 0 | 3 | 6 | 12 | 18 | 21 | 24 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| value | 0 | 4.296875 | 15.625 | 50 | 84.375 | 95.703125 | 100 |

The row is symmetric about its middle - 4.296875 from the start and 95.703125 from the end are the same distance - which is the shape of an ease that starts and stops at rest.

FX-EASE-003: the same curve on a pair, position from (0, 200) at frame 0 to (300, -40) at frame 12. Tolerance 1e-6. One timing curve drives both components; this is an ease in time, not a path in space.

| frame | 0 | 3 | 6 | 9 | 12 |
| --- | --- | --- | --- | --- | --- |
| x | 0 | 46.875 | 150 | 253.125 | 300 |
| y | 200 | 162.5 | 80 | -2.5 | -40 |

Both rows are the same fractions of their own journey, which is the check that matters here: at frame 3 the layer is 15.625% of the way along in x and 15.625% of the way along in y, so it is still on the straight line between the two keys and only its speed along that line has changed.

FX-EASE-004: `[0, 0, 0.58, 1]`, which is not symmetric, so a solver that assumes symmetry has somewhere to break. A scalar from 0 at frame 10 to 1 at frame 34, which also checks that a segment need not start at frame 0. Tolerance 1e-6.

| frame | 10 | 16 | 22 | 28 | 34 |
| --- | --- | --- | --- | --- | --- |
| value | 0 | 0.37813813082510966 | 0.6846431874274606 | 0.9065353492811752 | 1 |

This one leaves its first key fast and arrives at its second slowly, so the value at the middle frame is above half rather than at it. A solver that mirrored the curve would put it below.

FX-EASE-005: `[0.42, 0, 0.58, 1]`, whose `x` handles are not a third and two thirds, so the solve is real work rather than free. A scalar from 0 at frame 0 to 1 at frame 20. Tolerance 1e-6. Its presence is deliberate: the first two cases are curves where `x(t) = t` exactly, and a fixture set made only of those would not notice a solver that never worked.

| frame | 0 | 4 | 8 | 10 | 12 | 16 | 20 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| value | 0 | 0.08165985626589747 | 0.3318838700976461 | 0.5 | 0.6681161299023539 | 0.9183401437341024 | 1 |

The curve is symmetric, so the row is too, and the exact 0.5 at the middle frame is the one number in it that can be checked without a calculator.

## Transform fixtures

FX-XF-001 identity preserves pixels and bounds. FX-XF-002 integer translation moves a 1x1 impulse exactly one pixel. FX-XF-003 half-pixel translation verifies bilinear weights. FX-XF-004 rotates around a nonzero anchor using the matrix order in 21.

## Persistence fixtures

`Fixtures/projects/minimal_project.json`: smallest valid project. `cel_holds_project.json`: explicit exposure spans. `unicode_paths_project.json`: non-ASCII display/path fields. `missing_media_project.json`: valid project with intentionally unavailable asset. `unknown_effect_project.json`: structurally valid unknown effect that must survive load/save with a warning.

## Failure fixtures

FX-IO-001 interrupted replacement retains last valid project. FX-IO-002 disk-full/write failure reports `PROJECT_SAVE_FAILED` and does not truncate the previous valid save. FX-MATTE-001 creates A->B and B->A matte references and must be rejected with `MATTE_CYCLE`.

## Image/filter fixtures

Gaussian blur uses a synthetic single-pixel impulse in a transparent image. Expected weights are independently generated from the normalized sigma/kernel definition in 21. Test symmetry, normalization, alpha behavior and expanded bounds.

Mask rasterization fixtures must record the exact rasterizer/reference tool once selected in B-06; until then, polygon interior/exterior topology tests are authoritative but subpixel edge goldens are OPEN.

**Selected in B-06 on 2026-09-06 and recorded in ADR-016.** The rasterizer is a 4x4 ordered grid of sample points per pixel, coverage being the count of samples inside the polygon over sixteen, with insideness by the even-odd rule; the sample for grid cell (i, j) of pixel (x, y) is at `(x + (i + 0.5)/4, y + (j + 0.5)/4)`. The reference tool is a second implementation written from document 19 by a different method - summing the signed angle each edge subtends at the sample point - living in `tests/b06_mask.rs` rather than a drawing library, on the same discipline H-03 and H-04 use. Coverage is therefore an exact rational and the edge quantum is one sixteenth, which this sentence's OPEN now closes: subpixel edge goldens are written, in `verification/B-06_mask_table.md`. Raising the grid changes every one of them and is an amendment to ADR-016.

## CPU/GPU comparison

Simple arithmetic: absolute per-channel error <= 1e-5. Filter operations: tolerance is declared per fixture after the independent CPU reference is implemented; default target <= 2e-5 for float output. Exported integer PNG tests compare decoded integer samples exactly where deterministic quantization is specified.

A backend that exceeds tolerance requires diagnosis. Do not loosen tolerances globally to hide a backend-specific error.

## Workflow fixtures

RSH-01 and RSH-02 are specified in 22. They are manually reviewed in addition to automated tests. Their visual acceptance cannot replace the numeric fixtures above.

Related documents: 11, 20, 21, 22 and 28.
