# P-05 - the layer skipped for a tile it cannot reach

**21 of 21 checks passed.**

Produced by `tests/p05_culling.rs`, on every build.

## Why this exists

`render_tile` used to sample every layer at every pixel of every tile, with no test of whether the layer's transformed extent met the tile at all, and cel artwork is mostly transparent. P-05 gives every layer a box - its source rectangle grown by the one pixel bilinear sampling reaches for, mapped through the layer's own transform - and skips the layer for the tiles outside it.

A skip is only correct if the frame is the same frame without it. So every frame below is rendered **twice from one plan**: once by the renderer, which skips, and once by `render_without_culling`, which is the loop as it was, and every channel of every pixel is compared. Not a tolerance, not a sample: any difference a viewer could see is a failure.

The last three rows are the ones that make the rest mean anything. The first of them proves the comparison can fail at all; the second counts how many layer-and-tile pairs the box actually excludes, because a table of unchanged frames reads the same whether the skip works or never fires. The last is stricter than a viewer: it compares the f32 working buffer bit for bit, which is where the one wrinkle would show, since `0.0 + -0.0` is `+0.0` and a blend that is skipped can therefore differ from a blend performed against a zero source in the sign of a zero while agreeing on every pixel. It is reported rather than asserted.

Timings are not on this page. They are dated, and they are in `verification/P-05_culling.md`.

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| reference shot at Draft, frame 0: every one of 129600 pixels is what the frame was before any layer was skipped | `0 pixels differ` | `0 pixels differ` | pass |
| reference shot at Draft, frame 14: every one of 129600 pixels is what the frame was before any layer was skipped | `0 pixels differ` | `0 pixels differ` | pass |
| reference shot at Draft, frame 100: every one of 129600 pixels is what the frame was before any layer was skipped | `0 pixels differ` | `0 pixels differ` | pass |
| reference shot at Draft, frame 239: every one of 129600 pixels is what the frame was before any layer was skipped | `0 pixels differ` | `0 pixels differ` | pass |
| reference shot at Full, frame 0: every one of 2073600 pixels is what the frame was before any layer was skipped | `0 pixels differ` | `0 pixels differ` | pass |
| reference shot at Full, frame 14: every one of 2073600 pixels is what the frame was before any layer was skipped | `0 pixels differ` | `0 pixels differ` | pass |
| reference shot at Full, frame 100: every one of 2073600 pixels is what the frame was before any layer was skipped | `0 pixels differ` | `0 pixels differ` | pass |
| reference shot at Full, frame 239: every one of 2073600 pixels is what the frame was before any layer was skipped | `0 pixels differ` | `0 pixels differ` | pass |
| declared fixture at Draft, frame 0: every one of 129600 pixels is what the frame was before any layer was skipped | `0 pixels differ` | `0 pixels differ` | pass |
| declared fixture at Draft, frame 14: every one of 129600 pixels is what the frame was before any layer was skipped | `0 pixels differ` | `0 pixels differ` | pass |
| declared fixture at Draft, frame 100: every one of 129600 pixels is what the frame was before any layer was skipped | `0 pixels differ` | `0 pixels differ` | pass |
| declared fixture at Draft, frame 239: every one of 129600 pixels is what the frame was before any layer was skipped | `0 pixels differ` | `0 pixels differ` | pass |
| declared fixture at Full, frame 0: every one of 2073600 pixels is what the frame was before any layer was skipped | `0 pixels differ` | `0 pixels differ` | pass |
| declared fixture at Full, frame 14: every one of 2073600 pixels is what the frame was before any layer was skipped | `0 pixels differ` | `0 pixels differ` | pass |
| declared fixture at Full, frame 100: every one of 2073600 pixels is what the frame was before any layer was skipped | `0 pixels differ` | `0 pixels differ` | pass |
| declared fixture at Full, frame 239: every one of 2073600 pixels is what the frame was before any layer was skipped | `0 pixels differ` | `0 pixels differ` | pass |
| the comparison is capable of failing: the reference shot's frame 0 against its frame 100 | `they differ` | `they differ` | pass |
| how much the skip fires on the fixtures: layer-and-tile pairs the box excludes, over the eight full-resolution frames above | `stated, whatever it is` | `0 of 7020` | noted |
| the skip fires at all: a quarter-size layer placed in one corner of a 1920x1080 frame | `more than none` | `120 of 135` | pass |
| and that frame is still the frame it was without the skip, all 2073600 pixels | `0 pixels differ` | `0 pixels differ` | pass |
| the working buffer, f32 and bit for bit, across every check above | `identical` | `identical` | pass |
