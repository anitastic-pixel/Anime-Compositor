# B-12: what happened during the run

The second of the three things B-12 owes — the finished shot is the first, and the decisions the
run forces are the third. This is a form to write in, not a page written by anything. It is
blank on purpose, and it is nobody's job to fill in but the owner's.

`verification/B-12_acceptance_run.md` is the sheet to follow. This is where what happened to you
goes.

**Awkwardness is the finding.** A step that took four minutes because a control was not where it
looked like it should be is worth more here than a step that worked. A step that could not be
completed at all is the most valuable line on the page. Nothing written here is a complaint about
the build; all of it is the build's report card.

Date of the run:
Build: `target/release/anime_compositor_app.exe`, from commit
Display scale used:

## The thirteen steps

For each: how long it took, what you had to hunt for, what you expected and did not find, and
anything you did by accident.

| # | W-01 step | How long | What happened |
| --- | --- | --- | --- |
| 1 | Import media | | |
| 2 | Review grouping and missing-frame warnings | | |
| 3 | Create a composition | | |
| 4 | Assign exposures | | |
| 5 | Stack layers | | |
| 6 | Adjust anchors and transforms | | |
| 7 | Apply a matte | | |
| 8 | Add one blur | | |
| 9 | Add one colour operation | | |
| 10 | Inspect alpha | | |
| 11 | Preview the work area | | |
| 12 | Save, close, reopen | | |
| 13 | Export a PNG sequence | | |

## W-02, the second walk

## Anything that behaved differently from the sheet

The sheet was written from the build of 2026-09-08 and can go stale silently. A step where the
page and the window disagreed is a defect in one of them, and which one is worth knowing.

## Blocking defects

Something that stopped the run, as against something that made it slower. These are what B-12
asks be fixed before the gate closes.

## The first sitting, 2026-09-10, in the owner's words

Transcribed at the owner's instruction from what they wrote after the first hands-on sitting with
`target/release/anime_compositor_app.exe`. Their words, unedited; the agent added nothing to this
block and took nothing out of it. The reading of each finding is in the table below it and is the
agent's, so that the two can be told apart.

> so far, the visibility works. the exposures essentially details what I am seeing on the
> timeline, but I think we should hide the right detailing of said exposures for now, maybe we can
> preview it better later as it looks overwhelming on the side. I can't select/edit the
> timeline/layers directly like after effects, I would like to have the common controls/
> manipulations that After Effects has on layers like moving layers behind or over another, slide
> over the timeline, select frames and drag them to certain areas. effects control is nice,
> opacity in percentage instead of 0 to 1. reorganizing effects over one another, to get a certain
> effect. mouse click drag on values. when trying to play back from the start manually through the
> arrow keys, it freezes, then plays where I continued off from. can't select a play for the
> playhead to start from, have to use the arrows. I would like to be able to select the layers on
> the preview area/shot, select one with their respective bounding box/layerbox, then drag to move
> around, transform, rotate, scale, move with arrow key, etc, maybe select multiple layers at once
> with shift-click.

And, asked what a timeline ought to feel like:

> I do want the timeline to behave/interact like any other timeline, so far, I really love the
> timelines from after effects, premiere pro, and davinci resolve.

### What each finding is, read against the build

Eleven findings. One is a defect. Most of the rest are a window that does not offer what the core
already does, which is the pleasant kind of gap: the commands exist and have no control attached.

| # | The finding | What it is | Where it lives |
| --- | --- | --- | --- |
| 1 | Layer visibility works | Confirmation | — |
| 2 | The exposure sheet on the right is overwhelming | Presentation | `#exposures` in the right-hand column. Collapse rather than remove: typing frame numbers there is the only way an exposure is set, because the timeline bars are read-only by design. |
| 3 | Cannot select or edit layers directly in the timeline | New interaction | The rows select; the bars do not. |
| 4 | Move a layer in front of or behind another | Already in the core | `layer.move_up`, `layer.move_down` |
| 5 | Slide an exposure along the timeline, drag frames to a place | Part core, part new | `exposure.set_span` can express the result. No drag produces it, and dragging one span across another needs a collision rule, which is a decision and not a coding task. |
| 6 | Opacity as a percentage, not 0 to 1 | Presentation only | Document 19 holds opacity 0..1 in the file. The panel prints it and can print percent without the model moving. |
| 7 | Reorder effects within a layer | New capability | No `effect.move` command exists. ADR-017 makes the order matter, so the picture really does depend on it. |
| 8 | Click-drag on a number to change it | Already in the core | `property.drag_update`, `drag_end`, `drag_cancel`, built for this and never given a mouse. |
| 9 | Stepping from the start with the arrow keys freezes, then catches up | **Defect** | Manual stepping queues a render per keypress; playback deliberately drops frames instead. The two disagree, and the held key outruns the renderer. |
| 10 | Cannot click the timeline to put the playhead somewhere | New interaction | The ruler has no click handler. `/frame/N` already exists to answer one. |
| 11 | Select a layer in the preview with a bounding box; drag, rotate, scale, nudge; shift-click for several | Mostly already in the core | `property.set_base` and the drag commands already cover anchor, position, scale, rotation. Multi-select is the exception: every command takes one layer, so selecting several is a new idea in the window rather than a new button. |

Finding 9 is the only one that stopped anything, and it does not stop it for long — the queue
drains and the window comes back. It is recorded here rather than under blocking defects for that
reason.

## The decisions this run forces

Three are already open and waiting in `verification/B-12c_owner_brief.md`. Two more come from
B-12d:

- **Whether `composition.create` stays.** It is a new capability, not a fix. Nothing depends on
  it and cutting it is a revert.
- **The three size limits** — 16384 a side, 67108864 pixels, 10000 frames. Document 19 said
  "bounded by implementation safety limits" and never gave numbers; these are the build's, not
  the specification's.

## Anything else
