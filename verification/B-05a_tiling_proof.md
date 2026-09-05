# B-05a tiled render, identical-output proof

ADR-011 and document 21's tile contract. Produced by `tests/b05a_transform.rs`. **32 of 32 checks pass.**

## What to check by eye

Document 21: "a tiled render and a hypothetical whole-frame render of the same request must be byte-identical, and B-05a proves this rather than assuming it". Each row below renders the same four-layer reference-shot frame at one tile size on one number of worker threads, and compares the whole float32 buffer against a single whole-frame render. Every row must say `identical`. A row that named a float index instead would mean the picture changes depending on how the machine happened to divide the work, which is the failure this task exists to rule out.

Thirty combinations are covered: six tile sizes, including one that is a single pixel and two that do not divide the frame evenly, across five thread counts from one to twenty-four.

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| the identical-output plan draws every reference-shot layer | `4` | `4` | PASS |
| the whole-frame render is not blank, so the comparison below has something to compare | `true` | `true` | PASS |
| tile 1px across 1 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 1px across 2 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 1px across 4 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 1px across 12 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 1px across 24 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 7px across 1 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 7px across 2 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 7px across 4 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 7px across 12 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 7px across 24 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 16px across 1 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 16px across 2 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 16px across 4 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 16px across 12 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 16px across 24 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 64px across 1 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 64px across 2 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 64px across 4 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 64px across 12 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 64px across 24 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 100px across 1 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 100px across 2 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 100px across 4 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 100px across 12 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 100px across 24 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 256px across 1 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 256px across 2 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 256px across 4 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 256px across 12 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |
| tile 256px across 24 threads is byte-identical to whole-frame | `identical` | `identical` | PASS |

## Notes

- The comparison is `==` on the raw float32 buffers, not a tolerance. 517789 of the 518400 float values in the frame are nonzero.


## Why this holds

A tile owns its own accumulator and reads only immutable source buffers, so no float addition ever changes its order with thread count, and the assembly step writes each tile to a position fixed before any thread started. Determinism here is a property of the structure rather than something the test coaxes out of it. The test exists because ADR-011 asks for proof, not because the structure is in doubt.

This is not the same claim as SP-04's render determinism across runs, which is about two separate invocations, or B-10's, which is about two exported sequences. Those remain their own tests.
