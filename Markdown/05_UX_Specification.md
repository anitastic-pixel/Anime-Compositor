# Workspace and interaction specification

Version 0.3 | 2026-09-04 | Accepted for baseline

## Scope of this document in version 0.3

This specifies the interface for **G1-core only**. Panels for masks, effect stacks and cache state are removed from the design surface, because those features are parked under D-12 and designing interface for parked features is exactly the planning theater that version 0.3 exists to remove.

The interface is HTML and CSS rendered through Tauri, per ADR-004. That decision changes the status of design work in this project: a design is no longer a mockup that will be hand-translated later, it is the product. Design effort is therefore worth spending properly, once, on the screens that actually exist.

## Proposed workspace

Media bin, central composition viewer, property inspector, and a bottom timeline that is also the exposure sheet. A compact status area shows render progress and actionable warnings.

The timeline is the most important panel in this application, because exposure timing is the product. It represents layer order, exposure duration and drawing identity, and holds must be visually obvious rather than inferred from repeated cells.

The inspector edits the selected layer. Selection stays consistent across panels, and selected, locked and hidden states must be distinguishable at a glance.

Original icons, copy and visual styling throughout. Familiar terminology is fine; tracing proprietary screenshots or reproducing distinctive assets is not.

## Core interaction rules

Dragging a numeric control produces exactly one undoable operation on release. Escape cancels an in-progress edit. Typing a number exposes its unit, allows precision and validates range; an invalid value keeps the previous valid state and explains the problem.

Frame stepping always moves exactly one composition frame. Exposure editing changes the drawing mapping and never resamples artwork. Moving a keyframe does not change cel timing unless a command explicitly operates on both.

Layer reorder previews the resulting position. Effect bypass is separate from effect deletion, when effects exist.

## Viewer requirements

Fit, 100 percent zoom, pan, checkerboard and alpha-only view. Overlays are excluded from render output. The viewer identifies draft resolution and displays the current frame number.

Preview resolution starts at draft under D-33, and which resolution is showing is always visible rather than only visible when it differs; that indicator is R-06a's "visible indication when preview quality differs from final export" in its always-on form. Playback holds real time under D-32: when the machine cannot render a frame in its slot the frame is skipped rather than the clock stretched, and the count of skipped frames is shown after playback stops. A skipped frame that is never reported would be a silent fidelity fallback, which document 28 forbids; frame stepping, not playback, is how an individual drawing is inspected.

Color interpretation is visible next to asset and project settings rather than buried. A warning states both the problem and the next action: a missing numbered frame offers relink directly. Generic failure messages that send the user to a log are not acceptable, since the user cannot read a log usefully.

Under ADR-004 the viewer is the one place where the web layer touches correctness. SP-06 verifies that displayed pixels are byte-exact. If the viewer alters color, the artist is being lied to about their own work, which is worse than a slow preview.

---

## Keyboard and accessibility

Command registry before shortcuts, per document 24. Support keyboard selection, property entry, frame navigation, undo and redo, save and export. Shortcuts become editable after the baseline is stable, with an in-app reference.

Keep visible focus, sufficient contrast and text labels alongside color-coded status. Verify common dialogs and W-01 at 100, 150 and 200 percent scaling, which WebView2 handles well. Japanese filenames and Unicode labels must survive editing, save and relink, which the reference shot exercises directly.

Screen-reader behavior requires implementation-specific testing. Do not claim accessibility compliance from this document.

## States to design explicitly

These are the G1-core screens, and they are the full scope of the design handoff.

Empty project: show import and composition creation.

Normal editing: bin, viewer, timeline and inspector populated, a layer selected.

Exposure editing: the timeline in its primary role, with holds, a 1s layer, a 2s layer and an irregular 3s layer visible simultaneously, since the reference shot contains all three.

Missing media recovery: the layer preserved, the problem stated, relink offered inline.

Export: running with current frame, completed count and cancellation; and failed, with a readable cause and the output location.

Two further states need visual treatment but not full screens: dirty project, shown without nagging, and recovery available, distinguishing the recovery copy from the last manual save.

## Usability evaluation

W-01 and W-02 are the task scripts. Observe where the owner looks, how often panels are switched, accidental edits and moments of confusion. Record the actual starting configuration and input assets. Compare successive designs using the same tasks.

Acceptance: complete W-01 without assistance, with all required controls reachable by keyboard. Efficiency targets stay open until an observed baseline exists.

## Design deliverables

A styled design system and the five G1-core screens above, produced in Claude Design and delivered as HTML and CSS into `design/`. Dark theme, type scale, spacing scale, state colors and icon direction.

Sequencing note: this work should follow a spike proving the application launches and displays a composited frame. Designing five screens for an application that has never run risks designing against assumptions the first working build immediately contradicts.

Related documents: 02, 03, 11, 15, 22 and 24. Command IDs, shortcuts and drag transactions in 24; undo behavior in 26; diagnostics in 28.
