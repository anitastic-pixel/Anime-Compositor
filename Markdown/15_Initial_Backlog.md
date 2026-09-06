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

B-08 / R-06a / Viewer, frame stepping, work-area playback, resolution selection, and the transport chosen by SP-05. Depends on B-05a. No cache. Playback holds real time and drops the frames it cannot deliver, reporting how many it skipped, per D-32; the preview starts at draft resolution with an always-visible indicator of which resolution is showing, per D-33. DONE. The headless half is B-08a; the headless middle is `src/preview.rs`, which carries the resolution choice, the plan scale and a playback clock driven by an elapsed time the caller supplies, with `verification/B-08_preview_table.md` as its artifact. The window, the transport (D-36's custom URI scheme carrying raw sRGB samples), the wall-clock loop wired to that playback clock, frame stepping, the resolution indicator and the screenshots are in `app/`, with `verification/B-08_window_shell.md` and its three photographs as the artifact. The viewer can now be given a project — named on the command line or dropped on the window — which is B-09's half of it; without one it still opens on the reference shot from a hard-coded path.

Exit: T-06 with measured latency, and preview matching export within declared tolerance. Artifact: screenshots plus a preview-versus-export pixel comparison.

B-09 / R-08 / Versioned persistence, relink, manual save, autosave and recovery. Depends on B-05. Uses atomic replacement verified by SP-01. DONE. The headless half is `src/persist.rs` with `verification/B-09_persistence_table.md` as its artifact, 92 of 92 checks. The viewer half is both directions. A project reaches the window four ways — command-line argument, dropped on the window, the Open dialog, or the recent-projects list — all through the same function, and what the window then says about it is `persist::load`'s own diagnostics: missing media, an effect this build does not have, a file that is not a project at all. It goes back out through `persist::save`, with the shell holding the `Preserved` part of the load and handing it back unread, so a save writes the parts this build does not model as well as the parts it does. The window shows document 26's dirty state and the file it is on, Save As reopens what it wrote, and Ctrl+O, Ctrl+S and Ctrl+Shift+S do what the buttons do. The artifacts are `verification/B-09_open_a_project.md` with five photographs and `verification/B-09_save_table.md`, 15 of 15 checks. Autosave and recovery have now reached the window as well: a timer thread asks every ten seconds whether the project has been unsaved for two minutes, writes a snapshot beside the project when it has, and the window offers the snapshots it finds at startup, newest first. Recovering opens the snapshot as unsaved work against the project on disk, so nothing is overwritten until the person saves. The artifact is `verification/B-09_recovery_table.md`, 21 of 21 checks. What is still unphotographed is the file dialogs, because a script cannot answer one, and the two minutes actually elapsing: the table supplies the clock rather than waiting on it.

Exit: T-07 including interrupted write and disk-full. Artifact: the project files themselves, before and after, plus the recovered file from an interrupted save.

B-10 / R-09 / Immutable export snapshot and PNG sequence writer. Depends on B-08 and B-09. DONE. The renderer half is `src/export.rs`, with `verification/T-08_export_table.md` as its artifact, 47 of 47 checks and six exported frames beside it, and the whole-shot half is `tests/b10_full_shot.rs`, which exports the reference shot twice and requires all 240 pairs to be byte-identical, with `verification/B-10_full_shot_table.md` and `verification/B-10_contact_sheet.png` as its artifacts. The window half is now in as well: an Export button asks where the frames should go and then writes the whole work area there, named for the project and the frame number, at full resolution whatever the preview was showing. The project is cloned when the export is asked for, which is this task's immutable snapshot — editing the open document while the job runs does not reach the job. The job runs off the window's thread, the window says it is exporting and offers to cancel, and cancelling is read between frames. Document 07's default holds: a frame whose drawing is missing blocks the whole export and nothing is written, and the checkbox beside the button is D-28's override, made deliberately in front of a sentence that says what it does. The artifact is `verification/B-10_export_table.md`, 20 of 20 checks. What is unphotographed is the folder dialog, which no script can answer. There is no progress bar, and R-09 does not ask for one.

Exit: T-08 and T-09 with correct range, naming, color, cancellation and error state. Artifact: the exported 240-frame sequence, plus a byte comparison of two consecutive exports proving determinism.

B-11 / R-11, Q-03, Q-04 / Package the offline application, keyboard and display-scaling checks, dependency and license record. Depends on B-08 through B-10. DONE, with two things named rather than quietly left out. The dependency and licence record is `docs/DEPENDENCIES.md` and `Licenses/`, generated from the resolved Windows build and checked in both directions on every CI run, with `verification/B-11_record_table.md` as its artifact. The offline claim has two artifacts because it has two halves: `verification/B-11_offline_table.md` reads the build, 9 of 9 checks, and `verification/B-11_offline_run.md` watches the program run — where the application's own process held no connection and Microsoft's web view held four, which is D-39 and the owner's decision, not a defect in this code. The keyboard, scaling and Unicode checks are five photographs and `verification/B-11_display_and_keyboard.md`; they found three real defects, all fixed. Packaging is `tools/package.ps1`, a portable folder and zip carrying the executable, both licence texts, the manifest and 271 crates' notices, with `docs/SUPPORTED_ENVELOPE.md` inside it as `READ ME FIRST.md` and `verification/B-11_package.md` as its artifact. **Not done: there is no installer** — `bundle.active` stays false because signing a package needs a certificate this project does not have — and the two halves that need a person rather than a script, running the shot with the network adapter actually disabled and a legal review of the licence record.

Exit: T-10, T-15, T-16 and a clean-machine smoke test. Artifact: the installed application running on a clean environment, screenshotted at 100, 150 and 200 percent scaling.

B-12 / G1-core acceptance. The owner completes W-01 and W-02 on the reference shot. Fix blocking usability defects. Publish the supported envelope. Depends on B-11.

Exit: the owner finishes a shot unaided. Artifact: the finished shot, and the owner's written account of what was awkward.

## G1-rest and beyond

B-06 / R-04 masks and mattes, B-07 / R-05 effects. Parked under D-12. Each begins only when its revisit trigger in document 23 fires, and B-07 must address tile margins under ADR-011.

B-08b / R-06b bounded preview cache. UNPARKED on 2026-09-05 by D-37, after `verification/B-08_preview_latency.md` fired its trigger. Specified by document 27, recorded by ADR-015, and bounded by D-37 to a cache of decoded source cels with a memory ceiling, reachable from the preview path only and never from export. Its exit artifact is T-06: the same reference-shot playback measured before and after, plus the repeated-loop memory behaviour document 08 asks for. DONE on 2026-09-05, in `src/cache.rs`, with both halves delivered: `verification/B-08b_cache_table.md` for what a cache must never change, `verification/B-08b_cache_budget.md` for what this one is worth. The window-level picture is in as well: `verification/B-08_window_shell.md` was rephotographed after the cache and reads 59 frames played and 15 dropped where the same photograph before the cache read 66 played and 79 dropped. Hardened on 2026-09-06 — nine deliberate breaks, six of which the first fixture missed, recorded in `verification/HARDENING_mutation_report.md`.

B-13 / R-12 flat-plane camera and parenting. B-14 / R-13 bounded expressions, runtime undecided per D-10. B-15 / R-14 collect and package. B-16 / R-15 additional formats, one at a time.

A GPU render path is not a backlog item. It is trigger-gated on a stopwatch reading per ADR-006.

## Definition of ready and done

Ready: requirement, input fixture, expected behavior, prerequisites and dependency status are understood, and the expected values already exist independently of any implementation.

Done: behavior works, relevant fixtures pass, the verification artifact exists and the owner has reviewed it, documentation matches actual behavior, and the report states what was not run. A partial interface is not completion.

For the first coding session use B-01. Do not attempt several backlog items in one session because they look small.

Related documents: 03, 11, 12, 14, 22, 23 and 25.
