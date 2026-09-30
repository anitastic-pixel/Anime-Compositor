# B-160: faster export

Written by `cargo test --release --test b160_faster_export -- --ignored b160_table`, after `b160_every_export_hashed` and `b160_timing` have run for the old build (`B160_LABEL=before`, compiled from the commit before B-160) and for this one.

**D-231 (bit-exact):** an export now draws several frames at once and writes them in order. Every file it writes must be the file the old build wrote, byte for byte, and every report the same report. **D-230 (REJECTED by the owner on 2026-09-29):** the graphics card's video encoder was no faster on this machine and wrote bigger files, so B-160b removed it; every MP4 comes from the software encoder, as before B-160.

## Checks: 18 of 18 pass

| Check | Expected | Actual | Result |
|---|---|---|---|
| lines hashed: every file and every report, old build and new | 28685 | 28685 | pass |
| the same files, named the same, in the same order | the same | the same | pass |
| files and reports whose bytes changed (the MP4 compared with its dates set aside) | 0 of 28684 | 0 of 28684 | pass |
| the same build, the same MP4 twice, a second apart: whole files, then dates set aside | different, then the same | different, then the same | pass |
| reference shot PNG 8-bit, frames 0 to 23: drawn 5 at once, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot PNG 8-bit, frames 0 to 23: drawn as many as the export picks, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot PNG 16-bit premultiplied, frames 0 to 23: drawn 5 at once, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot PNG 16-bit premultiplied, frames 0 to 23: drawn as many as the export picks, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot EXR half, frames 0 to 23: drawn 5 at once, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot EXR half, frames 0 to 23: drawn as many as the export picks, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot EXR float, frames 0 to 23: drawn 5 at once, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot EXR float, frames 0 to 23: drawn as many as the export picks, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot GIF, frames 0 to 23: drawn 5 at once, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot GIF, frames 0 to 23: drawn as many as the export picks, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot animated PNG, frames 0 to 23: drawn 5 at once, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot animated PNG, frames 0 to 23: drawn as many as the export picks, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot MP4, frames 0 to 23: drawn 5 at once, against one at a time | the same bytes and report | the same bytes and report | pass |
| reference shot MP4, frames 0 to 23: drawn as many as the export picks, against one at a time | the same bytes and report | the same bytes and report | pass |

## Measurements (not pass or fail)

| What | Measured |
|---|---|
| fixture projects exported, every composition, as PNG and EXR half | 2296 |

## Speed, median of 7 (PROVISIONAL)

Machine: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads; release build (opt-level 3). Other builds were running on the machine at the same time, so these are provisional until a quiet re-measure.

| Export of the whole reference shot, 240 frames at 1920x1080 | Median |
|---|---|
| reference shot PNG 8-bit, old build, one frame at a time | 79980 ms |
| reference shot MP4, old build, one frame at a time | 20946 ms |
| reference shot PNG 8-bit, new build, several frames at once | 11998 ms |
| reference shot MP4, new build, several frames at once | 13476 ms |
| reference shot PNG 8-bit, new build told to draw one frame at a time | 93661 ms |
