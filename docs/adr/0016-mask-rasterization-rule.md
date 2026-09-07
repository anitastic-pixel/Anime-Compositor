# ADR-016: The mask rasterization rule is a 4x4 ordered grid, and the reference is a second implementation

Status: ACCEPTED
Date: 2026-09-06
Deciders: Andrew (owner), delegated to the agent's recommendation
Relates to: D-12 (unparking R-04), document 25 line 51, document 21 step 2

## Context

Document 25 line 51 left a hole open on purpose: "Mask rasterization fixtures must record the exact rasterizer/reference tool once selected in B-06; until then, polygon interior/exterior topology tests are authoritative but subpixel edge goldens are OPEN."

That sentence names two separate things. The first is the rule this build uses to turn a polygon into per-pixel coverage. The second is what the fixture checks that rule against. B-06 is the stage that was told to choose both, and until it did, no expected value along a mask edge could be written down, because there was nothing to derive it from.

## Decision

**The rule: a 4x4 ordered grid of sample points per pixel. Coverage is the number of samples inside the polygon divided by sixteen. Insideness is the even-odd rule.**

The sample for grid cell (i, j) of pixel (x, y) is at `(x + (i + 0.5)/4, y + (j + 0.5)/4)`. The sixteen offsets are symmetric about the pixel centre at `(x + 0.5, y + 0.5)`, which is document 21's convention.

**The tie-break: a sample lying exactly on the outline counts as inside.**

This was not in the first draft of this record, and the gap was found by breaking the code on
purpose. Two faults in the ray test -- counting an edge span closed at both ends instead of
half-open, and casting the ray in -x instead of +x -- looked at first like different spellings of
the same arithmetic, and a brute force over forty thousand random grid-aligned polygons proved
otherwise: they disagree, and **every** disagreement is at a sample point lying exactly on the
polygon outline. So the case is reachable, it is not rare on the axis-aligned rectangles this build
is mostly asked to draw, and until now nothing said what the answer should be.

Inside is the choice because it is what the shipped code already does and because it is the
forgiving one: a mask drawn to a pixel boundary covers that pixel rather than leaving a hairline of
the layer behind it showing. It is a convention, not a derivation -- the winding-number reference
subtends exactly half a turn at a boundary point and so calls it outside, which is a third
defensible answer -- and that is exactly why it has to be written down here instead of left to
whichever implementation is asked.

**The reference: a second implementation, written against document 19 rather than against `src/mask.rs`, living in the fixture.** It answers insideness by summing the signed angle each edge subtends at the sample point and asking whether the total is nearer a full turn than zero. The build under test casts a ray along +x and counts crossings under a half-open rule. The two share no code and no method.

## Rationale

Three properties made the grid the choice over an analytic area computation.

**Coverage is an exact rational.** Sixteen integer decisions, divided by sixteen. No floating-point area is accumulated, so the answer does not depend on vertex order, summation order, or the machine. An independent implementation that agrees on which side of each edge the sixteen points fall agrees exactly, not to a tolerance. That is what lets `verification/B-06_mask_table.md` state whole numbers a person can check by hand instead of a tolerance a person has to trust.

**The even-odd choice is not load-bearing.** Even-odd and nonzero winding differ only on self-intersecting polygons, and document 19 makes those unsupported and refused at the command boundary. On every polygon this build accepts the two rules give the same answer. Recording even-odd is therefore a statement of what the code does, not a decision that shapes any picture — and it is why the fixture's reference, which uses winding, is a fair check rather than a rule mismatch waiting to be discovered.

**The edge quantum is stated rather than implied.** Coverage lands on one of seventeen values, so an edge is accurate to one sixteenth. That is this build's honest resolution. Document 21 says "multisample details must be fixture-tested before claiming subpixel equivalence"; nothing here claims subpixel equivalence, and the fixture checks the quantum directly — an edge at x = 2.5 gives exactly 0.5, at x = 2.25 exactly 0.75, and a diagonal always a whole number of sixteenths.

The reference choice follows the discipline H-03 and H-04 already established on this project: check a build against something written a second time from the specification, not against a library. A polygon rasterizer from a drawing library would have brought its own fill rule and its own antialiasing, and any disagreement would have said something about that library rather than about this file.

## Consequences

`SAMPLES_PER_SIDE` in `src/mask.rs` is a specification constant, not a tuning knob. Raising it costs time in proportion to its square, buys a finer edge quantum, and changes every expected coverage value in `Fixtures/` and in `verification/B-06_mask_table.md`. Changing it is an amendment to this record.

Because the reference disagrees on boundary samples by design, the two rows that pin the tie-break in `verification/B-06_mask_table.md` deliberately do not go through it. They state the expected coverage directly. Everything else in that table is checked against the second implementation.

Document 25 line 51's "OPEN" is closed. Subpixel edge goldens can now be written, and B-06's table writes them.

What this record does not settle: whether one sixteenth is fine enough for a shipped picture. It is a number, it is stated, and it is checked. If a real shot shows a mask edge that reads as stepped, the answer is to raise the grid and re-derive the fixture values, and that is a new decision with a new table behind it — not a silent change to this one.
