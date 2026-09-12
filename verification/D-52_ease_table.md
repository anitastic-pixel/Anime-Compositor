# D-52: the eased segment, against a solver that is not this one

Written by `d52_eased_segments_match_the_reference_solver` in `tests/d52_ease.rs`, which runs on every build. **46 of 46 checks pass.**

D-52 asked for the curve to be named and a fixture to pin one eased segment to exact values *before any code was written*. The curve is a cubic Bezier of the value against time, written as the two inner control points of a curve whose ends are pinned at (0,0) and (1,1) - the four numbers CSS writes as `cubic-bezier()` and After Effects draws as two handles in its value graph.

Every number in the **expected** column is transcribed from document 25, which got it from `tools/ease_reference.py`. That file solves the equation by bisection, in Python, written from document 20. This build solves it by Newton's method with a bisection fallback, in Rust. The two were written from the specification and not from each other, which is what makes an agreement between them worth having. The largest disagreement anywhere in this table is **9.893e-13**.

What each case is for:

- **FX-EASE-001** - the curve that is linear evaluates as linear.
- **FX-EASE-002** - easy ease on a scalar, and its exact midpoint.
- **FX-EASE-003** - one timing curve drives both components of a pair.
- **FX-EASE-004** - an asymmetric curve is not evaluated as a symmetric one.
- **FX-EASE-005** - a curve whose solve is not free, because x(t) is not t.

| Case | Frame | Expected | This build | Difference | Tolerance | Result |
|---|---|---|---|---|---|---|
| FX-EASE-001 | 0 | 0 | 0 | 0.000e0 | 1e-9 | pass |
| FX-EASE-001 | 1 | 4.999999999999999 | 5 | 8.882e-16 | 1e-9 | pass |
| FX-EASE-001 | 6 | 30 | 30 | 0.000e0 | 1e-9 | pass |
| FX-EASE-001 | 12 | 60 | 60 | 0.000e0 | 1e-9 | pass |
| FX-EASE-001 | 18 | 90 | 90 | 0.000e0 | 1e-9 | pass |
| FX-EASE-001 | 23 | 115 | 115.00000000000001 | 1.421e-14 | 1e-9 | pass |
| FX-EASE-001 | 24 | 120 | 120 | 0.000e0 | 1e-9 | pass |
| FX-EASE-002 | 0 | 0 | 0 | 0.000e0 | 1e-6 | pass |
| FX-EASE-002 | 3 | 4.296875 | 4.296875 | 0.000e0 | 1e-6 | pass |
| FX-EASE-002 | 6 | 15.625 | 15.625 | 0.000e0 | 1e-6 | pass |
| FX-EASE-002 | 12 | 50 | 50 | 0.000e0 | 1e-6 | pass |
| FX-EASE-002 | 18 | 84.375 | 84.375 | 0.000e0 | 1e-6 | pass |
| FX-EASE-002 | 21 | 95.703125 | 95.703125 | 0.000e0 | 1e-6 | pass |
| FX-EASE-002 | 24 | 100 | 100 | 0.000e0 | 1e-6 | pass |
| FX-EASE-003 | 0 | (0, 200) | (0, 200) | 0.000e0 | 1e-6 | pass |
| FX-EASE-003 | 3 | (46.875, 162.5) | (46.875, 162.5) | 0.000e0 | 1e-6 | pass |
| FX-EASE-003 | 6 | (150, 80) | (150, 80) | 0.000e0 | 1e-6 | pass |
| FX-EASE-003 | 9 | (253.125, -2.5) | (253.125, -2.5) | 0.000e0 | 1e-6 | pass |
| FX-EASE-003 | 12 | (300, -40) | (300, -40) | 0.000e0 | 1e-6 | pass |
| FX-EASE-004 | 10 | 0 | 0 | 0.000e0 | 1e-6 | pass |
| FX-EASE-004 | 16 | 0.37813813082510966 | 0.378138130826099 | 9.893e-13 | 1e-6 | pass |
| FX-EASE-004 | 22 | 0.6846431874274606 | 0.6846431874274608 | 2.220e-16 | 1e-6 | pass |
| FX-EASE-004 | 28 | 0.9065353492811752 | 0.9065353492811752 | 0.000e0 | 1e-6 | pass |
| FX-EASE-004 | 34 | 1 | 1 | 0.000e0 | 1e-6 | pass |
| FX-EASE-005 | 0 | 0 | 0 | 0.000e0 | 1e-6 | pass |
| FX-EASE-005 | 4 | 0.08165985626589747 | 0.08165985626589689 | 5.829e-16 | 1e-6 | pass |
| FX-EASE-005 | 8 | 0.3318838700976461 | 0.33188387009764625 | 1.665e-16 | 1e-6 | pass |
| FX-EASE-005 | 10 | 0.5 | 0.5 | 0.000e0 | 1e-6 | pass |
| FX-EASE-005 | 12 | 0.6681161299023539 | 0.6681161299023539 | 0.000e0 | 1e-6 | pass |
| FX-EASE-005 | 16 | 0.9183401437341024 | 0.9183401437341032 | 7.772e-16 | 1e-6 | pass |
| FX-EASE-005 | 20 | 1 | 1 | 0.000e0 | 1e-6 | pass |
| FX-EASE-003 | all | x and y the same fraction of their own journey | same to 1e-12 | - | 1e-12 | pass |

## How to read a failure here

A row that fails is not necessarily a broken solver. It is a disagreement between two solvers, and either one of them may be the one that moved. Running `python tools/ease_reference.py` prints the Python side's table; document 25 holds what it printed when the fixtures were set. If the Python and the document still agree and this page does not, the build moved.

## What this page does not show

A motion path. An eased position travels the straight line between its two keys, faster and slower along it - FX-EASE-003's extra row is that claim checked, not assumed. A curve *through* the keys in space is the third decision D-52 named, and it is still open.

It also shows nothing about the window. Whether an artist sets an ease by dragging a graph or by pressing one preset button is a separate unit; this page is about the numbers those controls would produce.
