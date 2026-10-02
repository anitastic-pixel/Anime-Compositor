# D-261: Sketch, kept and never drawn into a frame

Written by `tests/d261_sketch.rs`. The fixture hash was taken on e395d8b, before D-253.

**16 of 16 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| Two sketch layers and four strokes are taken | none refused | none refused | pass |
| A stroke with a point that is not a number is refused | refused | refused | pass |
| A tool other than brush, pencil and eraser is refused | refused | refused | pass |
| A stroke on a sketch layer that is not there is refused | refused | refused | pass |
| Before Clear: strokes per layer | 3 + 1 | 3 + 1 | pass |
| Clear on frame 11 takes that frame's stroke only | 2 + 1 | 2 + 1 | pass |
| Undo puts it back | 3 + 1 | 3 + 1 | pass |
| Undo again takes back the last stroke drawn | 3 + 0 | 3 + 0 | pass |
| Saved and opened again, the two layers are the same, every point | the same | the same | pass |
| Saved again, the text is the same, byte for byte | the same | the same | pass |
| Frames 8 to 12 exported without sketches | 5 files | 5 files | pass |
| With the sketches (on frames 10 and 11, and the whole cut, one layer hidden), the same files, every byte | the same | the same | pass |
| Sketches in a file, with a line on a layer and a stroke no build writes yet, are kept through open and save | kept as they were | kept as they were | pass |
| The reference shot without sketches saves no sketches line | no line | no line | pass |
| Fixture projects that open | 2297 | 2297 | pass |
| Their saved text, all of it, is byte for byte as before (SHA-256) | aeadb2e51777a56efb28e3113b652eaa65c03d10cf2ba66b8d4798f0c86dae2d | aeadb2e51777a56efb28e3113b652eaa65c03d10cf2ba66b8d4798f0c86dae2d | pass |
