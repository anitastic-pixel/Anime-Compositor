# Audit P2: card effects moved to single precision (after B-157)

Built on 2026-10-14 from the approved audit (P2, with P9 and the Hair part of P10). The graphics
card adds numbers in two precisions; the finer one runs about 64 times slower on this card. B-157
(D-228) moved five effects to the ordinary precision; this does the same, by B-157's rules, for
the card's Hair, Star Burst and Curl Noise. Each keeps the finer precision only where B-157 kept
it: where a picture's position is worked out (large numbers), and where a value decides a hard
cut-off that the processor makes exactly.

**The rule (the same as B-157's):** every picture the card draws is within 1 level of 255 of the
processor's. The tables named below are each effect's own test table, run again on this build.

| Effect | What is single precision now | Stays double, and why | Worst difference from the processor (of 255) | Table | Result |
|---|---|---|---:|---|---|
| Hair | each strand's shape, its coverage of the pixel, its direction and lighting | where the strand's root is read, the coverage test, and the final colour mix (they follow the picture exactly) | 1 | `D-673_hair_table.md` (50 checks) | PASS |
| Star Burst | each star's distance, size, its picture read and its own colour | where the picture is read from (large numbers) and the final mix with the picture | 1 | `D-677_star_burst_table.md` (43 checks) | PASS |
| Curl Noise (Source Internal, the noise) | the noise itself, and the flow lines' samples and their sums | each step's length and stride along a flow line, and the seed | 1 | `D-446_curl_noise_table.md`, `D-646_curl_transform_table.md` | PASS |
| Curl Noise (Source This Layer / Other Layer) | nothing | the flow lines follow the picture's own field, which parts on a hair (D-583); in single precision they were up to 7 levels off, so this stays as it was | 1 (unchanged) | `D-583_curl_sources_table.md` (42 checks, unchanged) | PASS |
| Curves, Levels and the other tone effects | nothing | one pass over each pixel, which costs about as little as Invert (B-157's table: 8 to 9 ms against Invert's 8 at Full), so there is nothing to gain; the keying modes must also stay exact (D-341) | not changed | B-157's tables | not changed |
| Polar Coordinates, Ripple and the other warps | nothing | one pass, within about 3 ms of Invert in B-157's table; the mesh warps decide which triangle a pixel is in by an exact test | not changed | B-157's tables | not changed |
| Fractal Noise and the other grades | nothing | one pass, within about 3 ms of Invert in B-157's table; the block and cell modes make hard cut-offs | not changed | B-157's tables | not changed |

## Curl Noise keeps its field (P9)

Curl Noise first works out which way its flow lines run, from its own noise, settings and the time,
never from the picture. The card now keeps that and works it out again only when a setting or the
time changes. `P9_curl_field_table.md` checks it: 12 of 12 checks pass, and every kept picture is
byte for byte the same as one drawn from scratch.

## Not measured here

No timings were taken in this build (other work was building on the machine at the same time).
To measure later, one at a time, release build, nothing else running:

- `b552_hair_timing` (tests/b552_hair.rs)
- `b556_star_burst_timing` (tests/b556_star_burst.rs)
- `b326_curl_noise_timing` (tests/b326_curl_noise.rs)
- `b461_curl_sources_timing` (tests/b461_curl_sources.rs)
- `b525_curl_transform_timing` (tests/b525_curl_transform.rs)
- `b157_card_effect_timing` (tests/b157_f64_audit.rs), every card effect
