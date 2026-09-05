# B-01 / G0 feasibility — spike report

Backlog item B-01 in document 15. Spikes defined in document 06. Reported under the rules
in document 12: every number below was produced by running the code in `spikes/`, on the
machine and build recorded here. Nothing in this report is an estimate. Where something was
not run, it says so and why.

**Status: 4 of 5 spikes answered. SP-06 answered on its primary path; its optional on-screen
stage was not completed. The reference shot (owner task) is not drawn, so B-01 is not closed.**

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
| SP-06 | Does the webview alter the bytes it is given? | **PASS** — 16/16 exact, both transports |

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

**NOT COMPLETED: the on-screen stage.** The result above is a canvas readback via
`getImageData`, which happens before the OS composites the window to the display. Document 06
says "displayed pixels", so an additional stage was attempted: screenshot the window and
compare the probe strip against the same source bytes. It did not complete. A background
process cannot raise the spike window above the foreground terminal on Windows — neither
`SetForegroundWindow` nor `HWND_TOPMOST` succeeded — so all four captures photographed the
wrong window. **No conclusion about displayed colour can be drawn from those captures in
either direction, and none is drawn here.** The obstruction is capture tooling, not evidence
of a colour problem. Closing it needs the window in the foreground, which requires the owner
at the machine. Measured for whoever retries: device pixel ratio 1.5, probe drawn at 6×
integer scale, on-screen tile pitch 48 device pixels.

## What B-01 still needs

1. **The reference shot** per document 22, drawn by the owner. Not started. B-01 cannot close
   without it, and the spikes above deliberately use synthetic layers instead.
2. **SP-06 on-screen stage**, above.

## Quarantine

All code under `spikes/` is quarantined per document 06 and is not production code. The SP-03
compositor in `sp05_sp06_webview/src/render.rs` is a deliberate copy of SP-04's, not a shared
module: SP-04 has already been run and its PASS recorded here, and editing that file to
extract a library would invalidate the artifact this report rests on. Both copies are
discarded at integration.

Raw output is in `spike-output/` (gitignored): `sp01_report.txt`, `sp04_report.txt`,
`sp05_sp06_results.json` including every individual scrub sample, and `sp04/` reference PNGs.
