# Reference shot and fixture media

Version 0.3 | 2026-09-04 | Accepted for baseline

## Status

D-05 is closed. The owner draws the reference shot. This removes the last rights-clearance dependency from the pack: artwork drawn by the owner is cleared by construction, needs no review, and can be committed to the repository and published with it.

The shot is a test fixture, not a portfolio piece. Stick figures on flat color are correct and expected. Its value is entirely in its timing structure, its edge types and its deliberate defects. Drawing it well would waste time and would not improve a single fixture.

## Reference shot specification

Format: 1920 by 1080, 24 fps, 240 frames, exactly 10 seconds. PNG with straight alpha, 8 bits per channel.

Layer 1, background: one static painted image, fully opaque, exposed across all 240 frames. Flat color regions are fine. Include at least one large area of mid-grey, because a mid-grey field makes color misinterpretation visible immediately.

Layer 2, on 1s: 24 unique drawings, a new drawing every frame, cycling across the 240 frames. Give this layer a soft antialiased edge, for example a shape with a feathered outline. This is the layer that will reveal alpha and edge errors first.

Layer 3, on 2s: a new drawing every two composition frames. Give this layer a hard aliased edge with no antialiasing at all, a pure binary alpha. Hard edges make resampling errors obvious, because any interpolation introduces intermediate values that should not exist.

Layer 4, on 3s with deliberate irregularity: a new drawing every three frames, except for one drawing held for five consecutive frames and one drawing exposed for a single frame as an accent. Give this layer semi-transparent paint, an interior region at roughly 50 percent alpha. This layer is the primary exposure fixture: irregular timing is what the product exists to preserve, and a regular cadence would not prove anything.

## Deliberate defects

These are required, not accidental. Do not fix them.

One numbered frame is missing from one of the cel sequences, creating a gap in the numeric pattern. This drives the import gap diagnostic in T-01 and the relink workflow in document 05.

One file in one sequence carries a Japanese filename. This drives the Unicode path fixtures across import, save, reopen and relink, and it must survive the entire round trip unchanged.

## Why these specific choices

The mixed 1s, 2s and 3s cadence exercises the exposure model with three different rates simultaneously, which catches errors that a single rate would hide.

The five-frame hold and the one-frame accent are the actual product promise in miniature: intentional irregular timing that the tool must never quietly normalize. If a future change breaks held frames, this is where it shows.

Soft, hard and semi-transparent edges cover the three alpha regimes that behave differently under premultiplication. A shot with only soft edges would pass tests that a hard-edge shot fails.

The missing frame and the Japanese filename are the two most common real-world failure conditions in an image-sequence workflow, and they are cheap to include now and expensive to retrofit later.

---

## Synthetic numeric fixtures

The reference shot verifies workflow. It does not verify arithmetic, because a drawing cannot be checked to four decimal places by eye.

Numeric verification uses the synthetic fixtures in `Fixtures/` and document 25: small images with known pixel values, expected results computed independently, and stated tolerances. These are the authority for alpha, blending and color behavior. The reference shot is the authority for whether the tool is usable and whether timing survives.

Both are required. A build that passes the numeric fixtures but produces a wrong-looking shot has a specification error. A build that looks right but fails the numeric fixtures is accidentally correct and will break later.

## Fixture integrity

Expected values in `Fixtures/` and document 25 are read-only to implementation work, per document 12 and ADR-009. The reference shot is likewise fixed once drawn: changing it to make a test pass is the same offense as editing an expected value.

If the shot needs to change for a legitimate reason, that is a specification decision recorded in document 14, and every fixture that depends on it is re-derived deliberately.

## Storage

The reference shot lives in the repository under `Fixtures/reference_shot/`, organized by layer and numbered by drawing ID rather than by composition frame, since drawing ID is the identity the exposure model uses.

Keep it small. Flat color compresses well and there is no reason for this shot to be large.

Related documents: 05, 11, 12, 14, 20, 25 and 28.
