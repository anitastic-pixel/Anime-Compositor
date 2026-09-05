# Reference shot

Specified in `Markdown/22_Reference_Shots_and_Fixtures.md`. 1920x1080, 24 fps, 240 frames,
exactly 10 seconds. PNG, straight alpha, 8bpc. Organized by layer, numbered by **drawing ID**,
not by composition frame.

| Layer | Cadence | Edge | Drawings | Source |
|-------|---------|------|----------|--------|
| 1 background | static, all 240 frames | fully opaque | 1 | owner-drawn painting |
| 2 | on 1s | soft antialiased | 24 | `generate_cels.py` |
| 3 | on 2s | hard aliased, binary alpha | 12 (one missing, see below) | `generate_cels.py` |
| 4 | on 3s, irregular | interior at exactly 50% alpha | 20 | `generate_cels.py` |

`exposure_sheet.json` maps every composition frame to a drawing ID per layer. It is the
authority on timing. Layer 4 runs 80 exposures: 78 on 3s, one five-frame hold at frames
60-64, and one one-frame accent at frame 152.

Layer 4 also contains one **out-of-order re-exposure**: exposure 55, at composition frames
165-167, goes back to drawing 11, so the drawing IDs run 12, 13, 14, 11, 16, 17. Real cel
work does this constantly, and doc 20's exposure model permits it - a span maps frames to a
drawing number with no monotonicity constraint. An implementation that assumes drawing IDs
only increase must fail on this shot.

## Deliberate defects - required, do not fix

- `layer3/layer3_007.png` does not exist. The numeric gap is intentional and drives the
  import gap diagnostic in T-01.
- `layer2/layer2_桜_013.png` carries a Japanese filename, which must survive import, save,
  reopen and relink unchanged.

`generate_cels.py` fails if either defect is repaired.

## Provenance

Layer 1 is drawn by the owner. Layers 2-4 are generated, because what the fixture needs
from them is exact alpha behaviour - a strictly binary edge, an exactly-50% interior - that
hand-drawing cannot guarantee. Doc 22's stated reason for owner-drawn art is rights
clearance, which generated geometry satisfies equally. Recorded as a specification decision.

## Integrity

Fixed once drawn. Changing the shot to make a test pass is the same offense as editing an
expected value (doc 22, doc 12, ADR-009). `generate_cels.py` is the record of how the cels
were made and a self-check, not a build step - the committed PNGs are the fixture.

Run `python generate_cels.py` to verify: it re-emits the cels and asserts opacity, the
antialiased ramp, binary alpha, the exact 128 interior, both defects, and the frame counts.
