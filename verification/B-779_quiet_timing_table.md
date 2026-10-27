# B-779: size shown, quiet re-measure (D-899)

Re-measured on 2026-10-27 with nothing else building on the machine. RTX 4070 Ti SUPER on Vulkan, AMD64 Family 26 Model 68 with 24 threads, Windows 11, release builds, screen at 150%. Reference shot L25 (25 layers), Full chosen; the lag harness (`S/wave11/L23/lag/ab2.py`, tag w), 5 rounds of 2 repeats each, before (B-778) and after (B-779 as pushed) alternating round by round. Milliseconds, medians of the 10 readings. "Letting go to final" is the time from releasing the mouse to the sharp picture; "drag" is the delay while dragging. At fit the code asks exactly what it did before, so those rows show the machine's noise. The first run (shared machine) gave processor at 25%: mask 411 to 292, scale 394 to 277, move 335 to 271, number 381 to 311, bar 323 to 283.

| drawn by | zoom | gesture | letting go to final, before | after | drag, before | after |
|---|---|---|---:|---:|---:|---:|
| processor | 25% | mask | 348 | 284 | 49 | 49 |
| processor | 25% | scale | 331 | 244 | 43 | 41 |
| processor | 25% | move | 320 | 244 | 23 | 27 |
| processor | 25% | number | 264 | 251 | 41 | 48 |
| processor | 25% | bar | 234 | 231 | 41 | 47 |
| processor | fit | mask | 347 | 349 | 43 | 39 |
| processor | fit | scale | 339 | 333 | 41 | 39 |
| processor | fit | move | 338 | 326 | 24 | 25 |
| processor | fit | number | 409 | 372 | 48 | 45 |
| processor | fit | bar | 317 | 310 | 40 | 48 |
| processor | fit | scrub | 409 | 398 | 22 | 22 |
| card | 25% | mask | 236 | 248 | 66 | 68 |
| card | 25% | scale | 224 | 216 | 31 | 33 |
| card | 25% | move | 216 | 209 | 38 | 38 |
| card | 25% | number | 218 | 213 | 34 | 34 |
| card | 25% | bar | 205 | 202 | 32 | 25 |
| card | fit | mask | 243 | 257 | 60 | 54 |
| card | fit | scale | 214 | 218 | 26 | 25 |
| card | fit | move | 222 | 214 | 37 | 37 |
| card | fit | number | 232 | 220 | 40 | 33 |
| card | fit | bar | 216 | 218 | 27 | 23 |
| card | fit | scrub | 232 | 219 | 29 | 24 |

Result: on the processor at 25%, letting go to the final picture is quicker for mask (348 to 284), scale (331 to 244) and move (320 to 244); number and bar are about the same; scrub at 25% gives no readings (as before). Drag delays and every fit and card row stay within noise. D-899 stays ACCEPTED.
