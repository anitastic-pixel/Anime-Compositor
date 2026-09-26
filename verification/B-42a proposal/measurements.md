# D-98 proposal: what a running sum would change, and what it would save

**Proposed, not accepted. Nothing is built.** D-92 and D-96's fixtures are unchanged, and so is every picture the program makes today.

## First, a correction

When this change was offered, the agent said a running sum "adds numbers in a different order, so pixels shift by invisible amounts (about 1 part in 10 million)". **That was wrong.** D-92 takes its samples `length / ceil(length)` apart. At any direction except the four quarter turns, each pixel's samples fall between pixels in a different place, so no two pixels share a sample, and there is nothing to keep a running total of. A running sum needs a rule whose samples pixels do share. That is a small change to the rule itself, and it moves some pixels by more than rounding does. Below is how much.

## How to judge it

Look at the four pictures in this folder. Each shows the rule now, the proposed rule, and a black panel that is white wherever the two differ, made four times brighter than the real difference so it can be seen. If you cannot tell the first two panels apart, the change is safe to accept on looks.

- `directional_blur_1.png`: direction 30, length 10 (the length the speed table uses)
- `directional_blur_2.png`: direction 45, length 30, the direction that differs most
- `directional_blur_3.png`: direction 0, length 30
- `bloom_streaks.png`: Bloom's star of streaks, angle 20, length 60, on glow's flame

## How far pixels move

Screen levels out of 255, over the pictures' grey background, on the face drawing from D-92's proposal. "Largest" is the single pixel that moves most; the other column is the share of all pixels that move by more than 3 levels.

| Direction | Length | Largest | Pixels moving more than 3 |
|---|---|---|---|
| 0 | 1 | 0 | 0% |
| 0 | 4 | 24 | 1.2% |
| 0 | 30 | 3 | 0% |
| 0 | 100 | 1 | 0% |
| 20 | 1 | 52 | 3.5% |
| 20 | 4 | 55 | 2.7% |
| 20 | 30 | 33 | 0.3% |
| 20 | 100 | 10 | 0.1% |
| 30 | 10 | 39 | 0.9% |
| 45 | 1 | 99 | 4.4% |
| 45 | 4 | 87 | 3.9% |
| 45 | 30 | 60 | 0.4% |
| 45 | 100 | 19 | 0.3% |
| Bloom star, angle 20 | 60 | 4 | 0% |

**Where they move:** on one-pixel lines and hard edges, most near the diagonals and in short blurs. The proposed rule is a little crisper there. D-92's samples fall between pixels, which softens a line across the direction of the blur as well as along it. The proposed rule samples on each column's own centre instead. Straight across or straight down, odd whole lengths (1, 3, 5, …) give D-92's numbers exactly. Long blurs barely change. Bloom's streaks barely change, because the light they spread is already soft.

## The fixtures

The new expected values come from the same two Python references, run with `--d98`. They are written beside the old ones, which are left alone:

- `Fixtures/directional_blur/expected_directional_blur_d98.json`: 9 of the 15 cases move, the largest by 0.065 in working value (0 to 1). 005, 006 and the four invalid cases, 012 to 015, are unchanged.
- `Fixtures/bloom/expected_bloom_d98.json`: 5 of the 28 cases move, 007, 008, 011, 016 and 019, the largest by 0.041. These are the cases with streaks off the quarter turns, or of a length that is not a whole number. The other 23 are unchanged.

Each reference still checks its cases' claims on the new numbers. Three of Directional Blur's claims are restated with the new rule's figures: at length 4, row 4 is 3.875/5 covered instead of 4/5; at length 2.5, two columns out gets 1/15 instead of 1/16; and the covering is still all kept, spread over columns 2 to 12 instead of 3 to 11. Bloom's claims all hold unchanged. If D-98 is accepted, the `_d98` files replace the current ones and the old values are retired, as D-73a's were.

## What it would save

Timed with a throwaway prototype, `timing_prototype.rs` in this folder. It is not the build's code, and it runs both rules the plain way on **one thread**.

- Machine: AMD Ryzen 9 9900X, Windows 11 Education 10.0.26200
- Build: rustc 1.89.0, `-C opt-level=3`, one thread
- Workload: a 1920x1080 opaque plate, direction 30; the median of 5 runs

| Length | Rule now | Proposed | Times faster |
|---|---|---|---|
| 10 | 114.8 ms | 66.8 ms | 1.7x |
| 100 | 1123.6 ms | 76.7 ms | 14.6x |

The proposed rule's cost barely grows with the length; the rule now grows in step with it. The program itself runs on all 24 threads, so its times are shorter than both columns. Its own before and after would come with the build, in P-17's harness.

Bloom's streaks were not timed. A star is four such lines, so its streak part should gain in the same way. **Bloom's halo is untouched**, and it takes about 90 ms on its own (P-17's "Bloom 20" rows, no streaks). Bloom with a star of length 60 on an opaque plate, 340 ms now, cannot fall below that.

## What happens next

- **Accept:** B-42 builds it in the core. The `_d98` values replace the current fixtures, the B-36 and B-40 tables are run again, and P-17's harness gives the real before and after.
- **Decline:** nothing changes. The speeds stay as P-17 left them, and the `_d98` files and this folder are removed.
