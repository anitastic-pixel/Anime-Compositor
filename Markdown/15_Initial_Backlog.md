# Implementation backlog and task briefs

Version 0.3 | 2026-09-04 | Accepted for baseline

## Backlog rules

All tasks are NOT STARTED. Sequence reflects dependencies, not calendar. Every task attaches a verification artifact per document 12 before it closes. A task without an artifact the owner has reviewed is not done, regardless of test status.

One task per session. The dependency chain is deliberately narrow so that data model and rendering errors surface before an interface hides them.

## G0

B-01 / G0 feasibility. Draw the reference shot per document 22. Run SP-01 save and reopen with interrupted write, SP-03 scrub latency, SP-04 render determinism, SP-05 frame transport into WebView2, SP-06 viewer color exactness. Record toolchain, OS, driver and dependency versions.

Exit: five spike reports with measured numbers, the reference shot in hand, and either confirmation of ADR-003, ADR-004 and ADR-006 or an explicit reopening of whichever they contradict.

Artifact: the spike report table, plus the frames SP-04 compared and the color values SP-06 measured.

## G1-core

B-02 / R-10 / Tagged image buffers, linear-light premultiplied float32 working space, and normal-over compositing math on the CPU. Depends on B-01. No interface.

Exit: T-04 numeric basics and T-09 color and alpha fixtures pass. Artifact: fixture table with expected versus actual to full precision.

B-03 / R-01 / PNG import and sequence manifest, including numeric pattern detection, gap reporting and Unicode paths. Depends on B-02.

Exit: T-01 including the deliberate missing frame and the Japanese filename from the reference shot. Artifact: fixture table plus the diagnostic text the user would see for the missing frame.

B-04 / R-02 / Rational time model and explicit exposure spans. Depends on B-03. This is the central requirement of the product.

Exit: T-02 matches every expected drawing ID and frame range, including the five-frame hold and the one-frame accent. Artifact: the complete 240-row frame-to-drawing table, which the owner checks against the shot as drawn.

B-05 / R-03, R-07 / Model commands, layer order, transforms, hold and linear interpolation, undo and redo. Depends on B-04. Minimal inspection surface only, no real interface.

Exit: T-03 passes and undo restores the model exactly. Artifact: fixture table plus a before, after and undone project JSON the owner can diff by eye.

B-05a / ADR-011 / Tile-based multithreaded render plan across rayon workers. Depends on B-05. Not a separate feature; the renderer is built tiled from the start.

Exit: identical output to single-threaded evaluation, plus measured scaling on the reference machine. Artifact: identical-output proof and a timing table by thread count.

B-05b / ADR-012 / Render trace mode writing intermediate layer buffers as tagged PNGs. Depends on B-05a.

Exit: a trace directory for one composited frame. Artifact: the trace images themselves. This task exists to make every later task diagnosable.

---

B-08 / R-06a / Viewer, frame stepping, work-area playback, resolution selection, and the transport chosen by SP-05. Depends on B-05a. No cache. Playback holds real time and drops the frames it cannot deliver, reporting how many it skipped, per D-32; the preview starts at draft resolution with an always-visible indicator of which resolution is showing, per D-33. DONE. The headless half is B-08a; the headless middle is `src/preview.rs`, which carries the resolution choice, the plan scale and a playback clock driven by an elapsed time the caller supplies, with `verification/B-08_preview_table.md` as its artifact. The window, the transport (D-36's custom URI scheme carrying raw sRGB samples), the wall-clock loop wired to that playback clock, frame stepping, the resolution indicator and the screenshots are in `app/`, with `verification/B-08_window_shell.md` and its three photographs as the artifact. What the viewer does not have is a way to open a project: it shows the reference shot from a hard-coded path, because there is no open command until B-09.

Exit: T-06 with measured latency, and preview matching export within declared tolerance. Artifact: screenshots plus a preview-versus-export pixel comparison.

B-09 / R-08 / Versioned persistence, relink, manual save, autosave and recovery. Depends on B-05. Uses atomic replacement verified by SP-01.

Exit: T-07 including interrupted write and disk-full. Artifact: the project files themselves, before and after, plus the recovered file from an interrupted save.

B-10 / R-09 / Immutable export snapshot and PNG sequence writer. Depends on B-08 and B-09.

Exit: T-08 and T-09 with correct range, naming, color, cancellation and error state. Artifact: the exported 240-frame sequence, plus a byte comparison of two consecutive exports proving determinism.

B-11 / R-11, Q-03, Q-04 / Package the offline application, keyboard and display-scaling checks, dependency and license record. Depends on B-08 through B-10.

Exit: T-10, T-15, T-16 and a clean-machine smoke test. Artifact: the installed application running on a clean environment, screenshotted at 100, 150 and 200 percent scaling.

B-12 / G1-core acceptance. The owner completes W-01 and W-02 on the reference shot. Fix blocking usability defects. Publish the supported envelope. Depends on B-11.

Exit: the owner finishes a shot unaided. Artifact: the finished shot, and the owner's written account of what was awkward.

## G1-rest and beyond

B-06 / R-04 masks and mattes, B-07 / R-05 effects, B-08b / R-06b bounded cache. Parked under D-12. Each begins only when its revisit trigger in document 23 fires, and B-07 must address tile margins under ADR-011.

B-13 / R-12 flat-plane camera and parenting. B-14 / R-13 bounded expressions, runtime undecided per D-10. B-15 / R-14 collect and package. B-16 / R-15 additional formats, one at a time.

A GPU render path is not a backlog item. It is trigger-gated on a stopwatch reading per ADR-006.

## Definition of ready and done

Ready: requirement, input fixture, expected behavior, prerequisites and dependency status are understood, and the expected values already exist independently of any implementation.

Done: behavior works, relevant fixtures pass, the verification artifact exists and the owner has reviewed it, documentation matches actual behavior, and the report states what was not run. A partial interface is not completion.

For the first coding session use B-01. Do not attempt several backlog items in one session because they look small.

Related documents: 03, 11, 12, 14, 22, 23 and 25.
