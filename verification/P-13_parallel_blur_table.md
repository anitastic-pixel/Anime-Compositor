# P-13: the blur on one thread and on every thread, sample by sample

Written by `p13_parallel_blur_matches_one_thread` in `tests/p13_parallel_blur.rs`, which runs on every build. It carries no duration and no thread count on purpose, so that it is the same bytes on every machine; what the spreading is worth in milliseconds is `verification/P-13_parallel_blur.md`, out of P-01's timer on the recorded machine.

One buffer of 1920x1080, the size of a cel of the fixture document 08 line 41 declares, blurred at that fixture's own sigma of 4. Once inside a pool built with a single thread, once on the pool this machine would use. Compared by bits and not by a tolerance, because the arithmetic did not move: a destination row reads the source and writes only itself, so a pixel is the same taps accumulated in the same order whichever thread carries its row.

| What was compared | Samples | Differing |
|---|---|---|
| the blurred buffer, f32 and bit for bit | 8584704 | 0 |

The buffer grew from 1920x1080 to 1944x1104 - the kernel radius on all four sides, which document 21 requires and which is also how this page knows a filter ran at all rather than two untouched copies being compared.
