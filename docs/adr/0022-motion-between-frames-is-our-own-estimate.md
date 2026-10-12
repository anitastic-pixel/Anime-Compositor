# ADR-022: Motion between frames is our own deterministic estimate; effects that use it read other frames whole

Status: ACCEPTED on 2026-10-12 under the owner's /loop request, within the owner's D-600 ("Blur +
retiming only"), with D-650, built as B-530
Date: 2026-10-12
Deciders: Andrew (owner), through D-600 and the /loop request
Relates to: D-600 (P0-13, motion from footage), D-650 (the estimate and Pixel Motion Blur), D-651
(a card version, proposed), ADR-020 and D-216 (frame blending refuses Pixel Motion), D-528 (frames
kept for the next frame), D-610 (no state from frame to frame), document 20 "Extension boundary"

## Context

Document 20's extension boundary puts optical flow outside G1 unless an ADR and new fixtures add
it. The owner's D-600 asks for motion from footage, "Blur + retiming only": Pixel Motion Blur and
Timewarp on our own motion estimate, then Warp Stabilizer if the estimate holds up. After Effects
publishes no formula for its vectors, so the estimate has to be ours.

## Decision

**1. Our own estimate.** Given pictures A and B of the same size, linear and premultiplied, the
estimate finds for each pixel p a vector u(p) with A(p) close to B(p + u(p)). The rule is dense
pyramidal Lucas-Kanade with a ridge term, in double precision:

- two channels per picture: 0.3 R + 0.6 G + 0.1 B of the premultiplied colour, and alpha, so a
  drawing over nothing moves the estimate by its edges (footage, being opaque, is not changed by
  the second channel);
- a pyramid of 2 by 2 means, at most 6 levels above the picture, the top at least 8 pixels;
- Vector Detail picks the finest level estimated (100 is a vector for every pixel, 20 one every
  4); the field is enlarged to full size afterwards;
- a fixed count of steps on each level, each step solving a 2 by 2 system with a ridge term, then
  a local smoothing.

There is no search and no choice made on the data, so the same two pictures always give the same
field, and tiny differences of rounding move the field by tiny amounts, never by a jump. Document
21 holds the rule; `tools/flow_reference.py` works it a second way.

**2. Effects read other frames whole.** An effect that uses the estimate reads the holder's own
drawings at other whole composition frames, each drawn alone with the effects before it, by the
layer's own timing. A frame outside the layer's in and out points stands in as the present frame.
Nothing is carried from frame to frame: a frame drawn alone, on any tile, in any order, gives the
picture the same frame gives when played. The viewer's cache may keep those drawings, and the
cache key holds their content, so a kept drawing can never change a picture.

**3. On an adjustment layer nothing changes.** The frames beneath at other times are not to hand
there; the frame says TEMPORAL_SMOOTHING_SKIPPED (document 28).

**4. Processor first.** The estimate runs on the processor. A card version is D-651, proposed:
the present frame and its neighbours must meet on one side, and the card version must agree
within 1 level before it is used.

## Consequences

- Files are unchanged unless an effect that uses the estimate is added; the neighbour frames are
  never written.
- Pixel Motion Blur (D-650) is the first user; Timewarp is the next, Warp Stabilizer after it if
  the estimate holds up.
- Each frame costs the same wherever it sits in a shot: two extra drawings and two estimates a
  frame for Pixel Motion Blur.
- FX-PMB-001 to 024 in document 25 pin the estimate and the blur.

## Alternatives not taken

- **A third-party flow library, or After Effects' own vectors.** The vectors are closed and not
  ours to copy; a library would add a dependency for a rule of a few hundred lines that we can
  check ourselves.
- **A search (block matching).** Its best-match choices can flip on tiny differences between the
  processor and the card, which would break the 1-level rule.
- **Pixel Motion in a layer's frame blending.** ADR-020 refuses it for anime, since it tears flat
  colour; that stays refused.
