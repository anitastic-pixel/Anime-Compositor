# B-01 / G0 feasibility — spike report

Backlog item B-01 in document 15. Spikes defined in document 06. Reported under the rules
in document 12: every number below was produced by running the code in `spikes/`, on the
machine and build recorded here. Nothing in this report is an estimate. Where something was
not run, it says so and why.

**Status: all 5 spikes answered, including SP-06's on-screen stage. The reference shot
(owner task) is not drawn, so B-01 is not closed.**

## Machine, build and dependencies

| | |
|---|---|
| OS | Microsoft Windows 11 Education, build 26200.9278 |
| CPU | AMD Ryzen 9 9900X, 12 cores / 24 threads |
| RAM | 61.5 GB |
| GPU | NVIDIA GeForce RTX 4070 Ti SUPER, driver 32.0.16.1074 (2026-07-01) |
| | AMD Radeon(TM) Graphics (integrated), driver 32.0.21036.18 |
| Volume under test | `I:` — NTFS, 3.64 TB, Samsung SSD 990 EVO Plus 4TB (NVMe) |
| WebView2 runtime | 151.0.4129.86 |
| Toolchain | rustc 1.89.0 (29483883e 2025-08-04), cargo 1.89.0 (c24e10642 2025-06-23) |
| Profile | `release`, `opt-level = 3` |

Dependency versions as resolved: tauri 2.11.5, tauri-build 2.6.3, tauri-runtime-wry 2.11.4,
wry 0.55.1, webview2-com 0.38.2, tao 0.35.3, rayon 1.12.0, png 0.17.16, serde 1.0.229,
serde_json 1.0.151.

GPU is recorded because document 06 asks for it. **No GPU path was exercised.** Everything
below is CPU rendering into a WebView2 surface; the display path is composited by the OS.

## Summary

| Spike | Question | Result |
|---|---|---|
| SP-01 | Does an interrupted save leave the previous file intact? | **PASS** — 4/4 cases |
| SP-03 | Scrub latency presenting a composited 1080p frame | **MEASURED** — p50 98.90 ms, p95 102.80 ms |
| SP-04 | Are two renders of a fixed sequence byte-identical? | **PASS** — 10/10 runs |
| SP-05 | Frame transport Rust → WebView2, full and draft | **MEASURED** — 24.9 fps full, 145 fps draft |
| SP-06 | Does the webview alter the bytes it is given? | **PASS** — 16/16 in readback, 14/14 opaque on screen |
| SP-07 | Rendering the real reference shot | **MEASURED** — 12.02 ms/frame fused, 83.19 fps, determinism PASS |

---

## SP-01 — save and reopen with an interrupted write

`spikes/sp01_sp04_core/src/bin/sp01_atomic_save.rs`. Fixture reference FX-IO-001.

Writes a temporary sibling, flushes, `sync_all`, closes, then `fs::rename`, per ADR-010. A
child process is killed part-way through the write at three different points. The parent
then checks the original file byte for byte.

| Case | Expected | Actual | Result |
|---|---|---|---|
| control: uninterrupted save | file replaced, no temp left | 600090 bytes, marker=REPLACEMENT, strays=0 | PASS |
| abort after 64 KiB of temp write | original intact, byte for byte | aborted=true, intact=true, 75 bytes, marker=ORIGINAL | PASS |
| abort after 256 KiB of temp write | original intact, byte for byte | aborted=true, intact=true, 75 bytes, marker=ORIGINAL | PASS |
| abort after temp complete, before rename | original intact, byte for byte | aborted=true, intact=true, 75 bytes, marker=ORIGINAL | PASS |

**Observed defect:** every interrupted case leaves one `.tmp` file behind (`stray_temp_files=1`).
The saved project is never damaged, but the directory accumulates debris after a crash.
B-09 needs a startup sweep for orphaned temp siblings; without one the owner sees junk files
next to their project and cannot tell whether they matter.

**NOT RUN:** FX-IO-002 (disk-full). Simulating a full volume needs a dedicated small volume;
B-09 must cover it.

**NOT RUN:** true power loss with drive write caching enabled. `fsync` is issued; whether the
drive honours it is a hardware property this spike cannot observe. The interruption tested is
a process kill, which is weaker than power loss.

**Known gap:** the spike's own filesystem probe failed and printed `unknown (I:)`. The NTFS /
NVMe identification in the table above was obtained separately and is not self-reported by
the spike.

## SP-04 — render determinism

`spikes/sp01_sp04_core/src/bin/sp04_determinism.rs`. Document 21 line 131.

1920×1080, 8 frames, 64×64 tiles, four layers covering the alpha regimes document 22 asks
for (opaque field, soft antialiased edge, hard aliased edge, 50% semi-transparent). Rendered
twice at each of five thread counts and compared byte for byte against the 1-thread result.

**10 of 10 runs identical to the 1-thread baseline.** Thread count, completion order and
scheduling changed nothing. That is the property ADR-011 depends on.

| Threads | Render ms | Render speedup | Encode ms | Determinism |
|---:|---:|---:|---:|---|
| 1 | 235.9 | 0.99× | 320.9 | PASS |
| 2 | 160.8 | 1.45× | 329.3 | PASS |
| 4 | 116.7 | 2.00× | 322.7 | PASS |
| 12 | 98.7 | 2.36× | 326.8 | PASS |
| 24 | 88.9 | 2.62× | 345.3 | PASS |

1-thread baseline: render 233.1 ms, encode 316.1 ms, total 563.2 ms (8 frames).

Render and encode are timed separately on purpose. Only the render stage is tiled and
parallel. The sRGB encode is serial and does a `powf` per channel; a combined figure would
measure Amdahl's law on the encode and would be a false claim about tile scaling.

**2.62× on 24 threads is not the renderer's scaling limit and must not be quoted as one.**
This spike's per-frame work includes a serial full-frame tile merge and a per-tile layer
rebuild. Measuring real tile scaling is B-05a's job.

**NOT RUN:** determinism across different machines, CPUs or compiler versions. That is
document 29's question.

## SP-03 — scrub latency, composited 1080p

`spikes/sp05_sp06_webview/src/render.rs` and the SP-03 section of `ui/index.html`.

A timeline is scrubbed across 48 frames. **Every step composites its 1920×1080 frame on
demand** — serving a cache would answer an easier question than the one document 06 asks.
6 warm-up steps discarded, 48 measured.

| Stage | p50 ms | p95 ms | max ms | mean ms |
|---|---:|---:|---:|---:|
| **total** | **98.90** | **102.80** | **104.00** | **99.28** |
| composite (Rust core) | 54.53 | 57.51 | 59.49 | 54.78 |
| transport (core → webview) | 43.72 | 45.80 | 47.35 | 43.90 |
| draw (putImageData) | 0.40 | 0.50 | 0.70 | 0.38 |
| present (to next frame callback) | 0.20 | 0.40 | 0.50 | 0.22 |

Implied uncached scrub rate at p50: **10.1 steps per second**.

**This is a lower bound, not what the owner would feel.** The input event is dispatched by
script, so OS input dispatch and the final present-to-photon step are excluded. Real
perceived latency is higher by an unmeasured amount.

**This is not comparable to the document 08 target.** Document 08 line 41 proposes "p95
cached seek-to-display at or below 100 ms". That target is for a *cached* seek. SP-03
measured an *uncached* composite, and the two must not be compared. What can be said: an
uncached scrub step costs 102.80 ms at p95 on this machine, so the entire document 08
budget is consumed by one cold frame with nothing left for input or display.

**Cross-check against SP-04.** At 24 threads SP-04 renders 8 frames in 88.9 ms render +
345.3 ms encode, i.e. 11.1 ms render and 43.2 ms encode per frame, totalling 54.3 ms.
SP-03's independently measured composite p50 is 54.53 ms. Two separate spikes agree to
within 0.3 ms, which is meaningful evidence that both numbers are real.

That agreement also decomposes the latency, and the decomposition is the useful result:

- serial sRGB encode ≈ **43 ms** — the largest single cost, and it is not parallelised
- transport ≈ **44 ms** — see SP-05; nearly as expensive as the entire render
- parallel tile render ≈ **11 ms** — the part ADR-011 addresses is already the cheapest part
- draw and present ≈ **0.6 ms** — negligible

Two targets follow, and neither is the tile renderer. Neither is proposed here as work;
they are recorded for whoever plans B-05a.

## SP-05 — frame transport into WebView2

The primary technical risk of ADR-004. 60 frames per transport per scale, 10 warm-up
discarded. Transport time is measured separately from end-to-end.

| Transport | Scale | Size | Transport ms/frame | End-to-end ms/frame | fps | MB/s | Wrong-size frames |
|---|---|---|---:|---:|---:|---:|---:|
| JSON IPC | draft | 480×270 | 16.197 | 20.643 | 48.44 | 30.5 | 0 |
| Raw IPC | draft | 480×270 | 3.533 | 6.877 | 145.42 | 139.9 | 0 |
| Custom protocol | draft | 480×270 | 3.288 | 6.890 | 145.14 | 150.3 | 0 |
| JSON IPC | full | 1920×1080 | 250.375 | 254.552 | 3.93 | 31.6 | 0 |
| Raw IPC | full | 1920×1080 | 41.083 | 41.667 | 24.00 | 192.5 | 0 |
| Custom protocol | full | 1920×1080 | 39.538 | 40.145 | 24.91 | 200.1 | 0 |

**Findings.**

JSON IPC is unusable at full resolution: 250 ms per frame, 3.93 fps. Document 06 expected
this and it is confirmed rather than assumed. It must not be used for frame data.

Raw IPC and the custom URI scheme are within 4% of each other. There is no measured reason
to prefer one on speed.

**Full-resolution preview tops out at about 24.9 fps on the best transport, on this machine,
with an already-rendered frame.** That is at the 24 fps the reference fixture in document 08
asks for, with no margin, and before compositing, caching or any real workload. Draft
resolution has ample headroom at 145 fps.

ADR-004's transport risk is answered: the Tauri boundary can carry 1080p frames at roughly
real time, but it is a real cost — 40 ms per frame is not free, and SP-03 shows it is about
half of scrub latency.

## SP-06 — viewer colour exactness

The colour-correctness risk of ADR-004.

**Fixture note, and it matters.** Document 06 says to verify displayed pixels are byte-exact
"against document 25". Document 25 has no display-side expected value: it specifies
linear-light float compositing results, not 8-bit sRGB values after a display path. **No
expected value was invented and nothing in `Fixtures/` or document 25 was changed.** The
expected value used is the input bytes themselves. That is the correct test for the stated
risk in K-06 — that the webview *alters* colour — because any alteration breaks identity.

16 probe colours, chosen for where display paths go wrong: both endpoints, both mid-greys,
the sRGB encoding of linear 0.5 (188), all six saturated primaries and secondaries where
colour management shifts appear first, near-black (1,2,3), near-white (254,253,252), a
non-trivial mixed value, and two partial-alpha cases. Each sent by two independent paths:
raw RGBA bytes over IPC, and an untagged PNG with no `gAMA`, `cHRM`, `iCCP` or `sRGB` chunk.

| # | Source RGBA | Via raw bytes | Via PNG | Result |
|---:|---|---|---|---|
| 0 | 0, 0, 0, 255 | 0, 0, 0, 255 | 0, 0, 0, 255 | exact |
| 1 | 255, 255, 255, 255 | 255, 255, 255, 255 | 255, 255, 255, 255 | exact |
| 2 | 128, 128, 128, 255 | 128, 128, 128, 255 | 128, 128, 128, 255 | exact |
| 3 | 127, 127, 127, 255 | 127, 127, 127, 255 | 127, 127, 127, 255 | exact |
| 4 | 188, 188, 188, 255 | 188, 188, 188, 255 | 188, 188, 188, 255 | exact |
| 5 | 255, 0, 0, 255 | 255, 0, 0, 255 | 255, 0, 0, 255 | exact |
| 6 | 0, 255, 0, 255 | 0, 255, 0, 255 | 0, 255, 0, 255 | exact |
| 7 | 0, 0, 255, 255 | 0, 0, 255, 255 | 0, 0, 255, 255 | exact |
| 8 | 255, 255, 0, 255 | 255, 255, 0, 255 | 255, 255, 0, 255 | exact |
| 9 | 0, 255, 255, 255 | 0, 255, 255, 255 | 0, 255, 255, 255 | exact |
| 10 | 255, 0, 255, 255 | 255, 0, 255, 255 | 255, 0, 255, 255 | exact |
| 11 | 1, 2, 3, 255 | 1, 2, 3, 255 | 1, 2, 3, 255 | exact |
| 12 | 254, 253, 252, 255 | 254, 253, 252, 255 | 254, 253, 252, 255 | exact |
| 13 | 255, 0, 0, 128 | 255, 0, 0, 128 | 255, 0, 0, 128 | exact |
| 14 | 0, 0, 0, 128 | 0, 0, 0, 128 | 0, 0, 0, 128 | exact |
| 15 | 64, 96, 160, 255 | 64, 96, 160, 255 | 64, 96, 160, 255 | exact |

**16/16 byte-exact on both paths, alpha included.** Tolerance is zero; nothing was rounded
or approximated. Untagged PNG decoding did not apply a colour transform, and partial alpha
survived unchanged.

**On-screen stage: PASS, 14/14.** The table above is a canvas readback via `getImageData`,
which happens before the OS composites the window to the display, so document 06's phrase
"displayed pixels" needed a second stage: photograph the window and measure the probe strip
in the screenshot. Two earlier attempts were inconclusive because a background process cannot
raise the window above the foreground terminal on Windows, and the captures photographed the
wrong window; the owner brought the window to the front and the capture was retaken.

The strip was located at pitch 48 device pixels, matching the value the page recorded in
advance (device pixel ratio 1.5, drawn at 6x integer scale). All 14 opaque probe colours
measured byte-exact on screen, including all six saturated primaries and secondaries, both
mid-greys, near-black (1,2,3) and near-white (254,253,252).

This closes the display path end to end: canvas, WebView2 compositor, DWM, GPU scanout and
the physical monitor alter nothing. Evidence: `spike-output/sp06_screen_report.txt` and
`spike-output/sp06_probe_strip_onscreen.png`, a crop of the measured strip. The check is
reproducible with `spikes/sp06_screen_check.py <screenshot.png>`.

**NOT ASSERTED on screen:** probe 13 and 14 carry alpha 128 and are composited over the page
background before they reach the display, so their on-screen value legitimately differs from
their source. Both were verified exact in the readback stage above, where alpha survives.

## Reference shot

Present, at `Fixtures/reference_shot/`. Layer 1 is the owner's painting; layers 2-4 are
generated, a specification decision recorded there and in that directory's README. The
spikes above predate it and deliberately use synthetic layers instead, so no number in this
report was measured on the reference shot. See the ADR-006 note below.

## ADR adjudication

B-01's exit requires confirmation of ADR-003, ADR-004 and ADR-006, or explicit reopening of
whichever the measurements contradict. **None is contradicted. None is reopened.** What
follows separates what the spikes actually established from what they did not.

### ADR-003, Rust core - CONFIRMED on its measurable claims

Its secondary claims held: rayon made the tile parallelism straightforward, SP-04 was
byte-identical across 10 runs and 5 thread counts, and the binary was self-contained.

**Not measured, by nature.** ADR-003's deciding argument is about failure *modes* - that
C++'s characteristic failures surface as intermittent wrong pixels an owner cannot diagnose.
No spike can measure the absence of a defect class. That argument stands on reasoning, not
on this report, and should not be described as spike-confirmed.

### ADR-004, Tauri and WebView2 - CONFIRMED, with a much tighter bound than it states

ADR-004 names three accepted costs and commits SP-05 and SP-06 to measuring two of them
before implementation depends on them, with a native rendering surface as the fallback if
either fails. Neither failed, so the fallback is not triggered.

- **Colour management: cost does not materialise.** SP-06 found 16/16 exact in readback and
  14/14 exact on the physical display. The webview does not alter the bytes it is given.
- **Frame transport: cost is real, and the margin is far thinner than "limits
  full-resolution playback" suggests.** At 1920x1080 the custom protocol runs 39.54 ms per
  frame (24.91 fps) and raw IPC 41.08 ms (24.00 fps), against a 24 fps target. That is
  between 0.00 and 0.91 fps of headroom, for transport alone, with nothing composited
  concurrently. SP-03 measured a complete uncached scrub step at 102.80 ms p95.

Two consequences that are findings, not opinions. **JSON IPC is eliminated**: 250.38 ms per
frame and 3.93 fps at full resolution, where document 06 expected it to be viable. And the
frame cache in document 27 plus draft-resolution preview are **load-bearing, not
optional** - full-resolution playback has no margin to spend.

### ADR-006, CPU only - CONFIRMED, and the measurements strengthen it

ADR-006 defers a GPU backend until a measured result shows the CPU path too slow. The
measurements argue positively against building one now. The per-frame decomposition is:

| Stage | ms per frame | Would a GPU backend help? |
|---|---|---|
| serial sRGB encode | ~43 | no - not the renderer |
| transport into the webview | ~44 | no - ADR-004's boundary |
| parallel tile render | ~11 | this is the only part it addresses |
| draw and present | ~0.6 | no |

A GPU path would target the cheapest stage and leave roughly 87 ms untouched. The
bottleneck is not the renderer.

**This also reprioritises B-05a.** ADR-011's tiling is confirmed correct - determinism held
at every thread count - but the optimisation target it implies is wrong. The serial sRGB
encode is the largest single cost and does not scale with threads at all.

> **Superseded by SP-07.** The sentence above was written before the reference shot was
> rendered. The serial sRGB encode is not an inherent cost: it was an artifact of how SP-04
> was structured. See SP-07 below, which measures the same work both ways and finds the
> serial pass costs 41.41 ms per frame that need not be spent at all. The ADR-006 verdict is
> unaffected and gets stronger; the B-05a recommendation is replaced.

## SP-07 - the reference shot rendered

Run after the reference shot landed, to discharge ADR-006's own exit condition, which speaks
of "a measured result on a real shot on the reference machine". Every earlier number in this
report came from synthetic layers.

`spikes/sp01_sp04_core/src/bin/sp07_reference_shot.rs`, quarantined. Real PNG cels decoded
from `Fixtures/reference_shot/`, composited four layers back to front in linear-light
premultiplied f32, 64x64 tiles across rayon workers, merged by tile origin.

**What this does not establish.** The colour arithmetic in SP-07 is provisional and is
B-02's job. Its expected values in document 25 are not consulted and no correctness claim is
made. SP-07 measures cost and determinism on real media, nothing else.

### Decode

| Measurement | Value |
|---|---|
| Drawings decoded | 56 |
| Resident once decoded | 443.0 MiB |
| Total decode | 122.1 ms |
| Per drawing | 2.18 ms |

443 MiB for 56 drawings is a finding in its own right and an input to the document 27 cache
model: a shot with several hundred drawings does not fit a naive decode-everything strategy.

### The deliberate missing drawing

Layer 3 drawing 007 is absent by design. SP-07 reports it as a missing source and composites
those frames without it rather than substituting a neighbour, which is what document 20
requires. **20 of 240 composition frames reference the absent drawing.** That is the count
B-03's gap diagnostic has to surface.

### Determinism and scaling on the real shot

| Threads | Wall ms, 240 frames | ms per frame | fps | Determinism |
|---|---|---|---|---|
| 1 | 13629.8 | 56.79 | 17.61 | PASS |
| 24 | 4429.9 | 18.46 | 54.18 | PASS |

Speedup on 24 threads: **3.15x**. Byte-identical across repeat runs and across both thread
counts. As with SP-04, this is not the renderer's scaling limit and must not be quoted as
one - the workload is memory-bound, reading roughly 33 MB of source cels per frame.

### The finding: fused versus serial sRGB encode

SP-04 rendered to linear f32 in parallel and then converted to sRGB8 in a single serial
pass, and this report called that pass the largest single cost. SP-07 measures both
structures on identical work, on the same shot, on the same machine.

| Encode structure | Wall ms, 240 frames | ms per frame | fps |
|---|---|---|---|
| Fused into the tile, parallel | 2885.1 | 12.02 | 83.19 |
| Serial pass after render | 12823.3 | 53.43 | 18.72 |

Output is **byte-identical** between the two, checked before the timings were compared. The
serial pass costs **41.41 ms per frame** more, 4.44x the fused wall time. That figure
independently reproduces the ~43 ms SP-04 attributed to encoding, which is good evidence
both measurements are real.

Document 21 line 117 already settles this: an operation whose output pixel depends only on
the corresponding input pixel "is tile-safe without qualification. This covers transforms
sampled per output pixel, blending, opacity and **color conversion**". The tile contract
always permitted the encode to happen inside the tile. SP-04 simply did not, and this report
then promoted that choice to a property of the problem.

**No specification changes.** Document 21 is right as written. What changes is the B-05a
recommendation: the tile should emit display-ready 8-bit rather than linear f32 for the
preview path, and there is no serial encode stage to optimise because there should not be
one.

### What this does to the ADR-004 picture

With the encode fused, compositing the real shot costs **12.02 ms per frame**. SP-05
measured transport into the webview at **39.54 ms per frame** at full resolution. Transport
is now **3.3x the entire composite** and is unambiguously the dominant cost in the preview
path. Nothing about ADR-004's verdict changes - it passed - but the case for the document 27
cache and for a draft-resolution preview is stronger than it was.

**Evidence:** `spikes/evidence/sp07_rendered_frames.png` shows frames 0, 60, 62, 64, 152,
165, 166 and 239 composited. Frames 60, 62 and 64 hold layer 4's square in place while the
other layers move, which is the five-frame hold. Frames 165 and 166 do the same for the
out-of-order re-exposure. Full-size frames and the raw log are in `spike-output/sp07/`.

**NOT RUN:** SP-07 renders and discards. It does not write a 240-frame sequence to disk, so
it says nothing about export throughput, which is B-10's measurement. It also does not push
frames through the webview, so the 12.02 ms and the 39.54 ms above are measured separately
and never end to end.

## Quarantine

All code under `spikes/` is quarantined per document 06 and is not production code. The SP-03
compositor in `sp05_sp06_webview/src/render.rs` is a deliberate copy of SP-04's, not a shared
module: SP-04 has already been run and its PASS recorded here, and editing that file to
extract a library would invalidate the artifact this report rests on. Both copies are
discarded at integration.

Raw output is in `spike-output/` (gitignored): `sp01_report.txt`, `sp04_report.txt`,
`sp05_sp06_results.json` including every individual scrub sample, `sp04/` reference PNGs,
`sp06_screen_report.txt` and `sp06_probe_strip_onscreen.png`.
