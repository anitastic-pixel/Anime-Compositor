# B-08 preview latency, measured on the production path

T-06 asks for measured seek latency. This is the half of it that exists: the cost of producing a preview frame, from a project file on disk to a finished buffer in memory. Produced by `tests/b08_preview.rs`, which is `#[ignore]`d in normal runs.

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`
- Workload: `verification/B-08a_project.json`, the four-layer reference shot, previewed at 24 frames spread across the whole 240-frame shot
- Each resolution is preceded by one untimed frame, so no measured frame pays for the first touch of freshly allocated pages
- Tile size: `compose::DEFAULT_TILE_SIZE`

Debug assertions in this build: false. A run with `true` there is a debug build, and its numbers say more about the compiler than about the renderer.

## Measurements

| Resolution | Extent | Median ms | p95 ms | Slowest ms | Of the median, decoding | Of the median, rendering | Frames per second at the median |
|---|---|---|---|---|---|---|---|
| Draft | 480x270 | 81.69 | 85.27 | 89.37 | 75.15 | 6.53 | 12.2 |
| Full | 1920x1080 | 99.90 | 113.67 | 120.45 | 78.59 | 21.32 | 10.0 |

## How to read this

The number that matters is the last column against 24, the frame rate document 08's fixture asks for. A resolution whose median frame costs more than 41.7 ms cannot be played at speed on this machine, and D-32 says what happens then: the clock is held and frames are dropped rather than the shot running slow.

**Neither resolution reaches 24 frames per second here, and draft is barely faster than full.** That is not the shape SP-05 found, and the two decoding and rendering columns say why: SP-05 measured moving an already-rendered frame, while this measures making one, and making one begins by reading four cels off the disk and decoding them. That cost is the same at both resolutions, because a drawing has to be decoded at its own size before anything can be scaled. Draft resolution makes the rendering column cheaper and cannot touch the decoding one.

This does not reopen D-33 - draft is still the faster of the two and still the right default - but it does say plainly that resolution alone will not buy real-time playback of this shot. What would is not decoding the same drawing again every time it is shown, which is document 27's cache. That is B-08b, PARKED under D-12, and this table is the first measurement in this project that argues for it from the production path rather than from a spike.

**This is not end-to-end playback.** It stops at a finished buffer in memory. Getting that buffer onto the screen is the transport, which does not exist yet; SP-05 measured about 39.5 ms per full-resolution frame and 3.3 ms per draft frame for it, so an end-to-end estimate is roughly the sum of the two columns. Those figures came from a quarantined spike against an already-rendered frame and are not evidence about this build.

A slowest column far above the median means some frames cost much more than others. Two things in this shot would do that: a frame whose drawing has not been read before, and the twenty frames where layer 3 asks for a drawing that is deliberately absent.
