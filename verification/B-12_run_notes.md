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

## The second sitting, 2026-09-11, in the owner's words

Transcribed at the owner's instruction from what they wrote after the second hands-on sitting,
with `target/release/anime_compositor_app.exe` rebuilt from `main` at `68d6039` (W-04 merged).
The first build they tried was the one from 2026-09-09, which held none of W-03 or W-04, and the
first thing they reported was that nothing had changed: the page is baked into the binary at
compile time, and a change to it is not in any binary built before it. Their words, unedited;
the agent added nothing to this block and took nothing out of it. Two screenshots came with it
and are described under the block, in the agent's words.

> I seem to try and scroll the layer across the timeline, but it instantly teleports the
> playhead, so limit the playhead clicking to the top bar maybe with the 0 - 25 - 50 - 75, how
> would you best tackle this, maybe have it like so, but still select the specific frame/layer
> piece to let me slide around the timeline, and ? the multi-selecting of layers works, along
> with positioning, I would like to see the anchorpoint as well like AE does; transforming works;
> hiding the exposures looks much better; any value box, I want the drag to manipulate the values
> as well, along with a color display to help select the proper color for the tint, maybe
> different color picker graphs, like a color wheel, however way AE/Davinci Resolve does it;
> opacity as a percentage is great; effects reordering is great as well, let's also have an
> option to drag it up and down smoothly; also we need to optimize the effects performance for
> gpu/cpu acceleration, along with allowing realtime updating for the effects, as I had to toggle
> visibility of the layer to get it to properly display the effect; but is getting a little
> better now. also allow me to select layers that are outside of the preview box [Image #6] as it
> isn't allowing me to click and drag it; click and select multiple layers maybe? looks great
> with tint and blur! [Image #7] ; we need a scale bar at the bottom, for fit to screen, crl +
> scroll for specific spot zoom with where the mouse points in the composition area, scroll wheel
> for basic centered zoom, crl + shift + scroll for slighty boosted dramatic zoom speed.

The first screenshot shows a layer dragged well off the left of the composition, its outline and
handles drawn in the dark of the stage outside the picture, and the picture itself to the right
of it. The second shows a layer with a tint of `0, 5, 0` at amount 0.5 and a blur of radius 27,
with the outline drawn a little turned, and the effect controls beside it.

### What each finding is, read against the build

Fourteen findings. Two are defects. Six are confirmations, which is the best kind of line on this
page. The rest are gestures the window does not offer yet, and one is a question about the
engine rather than the window.

| # | The finding | What it is | Where it lives |
| --- | --- | --- | --- |
| 1 | Dragging a layer bar along the timeline moves the playhead instead | **Defect** | The whole time half of `#sheet` scrubs, rows included. That was chosen so that the rows scrub like Premiere's, and it makes a bar something that cannot be dragged. Scrubbing belongs to the ruler alone. |
| 2 | Slide a layer or an exposure along the timeline, keep selecting them | New interaction, part core | Nothing in the core moves a layer in time: no command changes `in_frame`/`out_frame`. `exposure.set_span` can move a span. The rule for a span dragged into its neighbour is the decision the first sitting already raised. |
| 3 | Multi-select and positioning work | Confirmation | — |
| 4 | Show the anchor point, as After Effects does | Presentation | `anchorOnScreen` already computes where it is; nothing draws it. |
| 5 | Transforming works; the collapsed exposures look better; opacity in percent; effect reordering | Confirmation | — |
| 6 | Drag on any value box, not only the transform ones | Already in the core | `Document::update_drag` takes any command, `effect.set_parameters` included. The effect fields are plain inputs that commit on blur; the transform fields already drag. |
| 7 | A colour display and picker for the tint | New control | The colour is linear RGB and can exceed 1 (the screenshot has 5); a picker shows the clamped colour and the numbers stay beside it. |
| 8 | Drag an effect up and down the stack smoothly | New interaction | `ReorderEffect` takes an index; the window only routes one step up or down. |
| 9 | GPU or CPU acceleration for the effects | Decision | The blur is already separable and the effect result is already cached by value (P-11). Anything more is an ADR, and no timing claim is made here without a measurement. |
| 10 | The effect did not show until the layer's visibility was toggled | Not reproduced | The effect cache holds the parameters in its key by value, so a stale result cannot come from there. The likeliest cause is finding 6: a typed value is not sent until the field loses focus, and the toggle is what took the focus. Live fields remove the case either way. |
| 11 | Cannot select or drag a layer that lies outside the picture | **Defect** | The press handler is on the canvas, and a press in the dark around it never reaches the canvas. The outline is drawn there; the pointer is not heard there. |
| 12 | Click and drag to select several layers | New interaction | A marquee over the stage; the selection model already holds several. |
| 13 | Tint and blur look great | Confirmation | — |
| 14 | A scale bar with fit-to-screen; scroll to zoom about the centre, Ctrl+scroll about the pointer, Ctrl+Shift+scroll faster | New interaction | The canvas is fitted by CSS and has no zoom. Every screen-to-frame conversion in the page reads the canvas's rectangle on screen, so a zoom is a transform on the stage and the arithmetic holds. |

### The order agreed

Defects first, then the thing the owner asked how to tackle, then the rest in the order that
puts a visible change in front of them soonest.

1. **W-05, the timeline as a timeline.** Findings 1 and 2. Only the ruler and the playhead's own
   head scrub. Pressing the middle of a layer's bar slides the whole layer, exposures riding with
   it; pressing either end trims that end; pressing an exposure block slides that block, stopping
   where it meets its neighbour. One drag is one history entry, as on the picture. This needs one
   new core command for a layer's in and out frames.
2. **W-06, the stage.** Findings 11, 12, 4 and 14. The press handler moves to the stage; a press
   on nothing starts a marquee; the anchor is drawn and can be dragged; a zoom with a scale bar.
3. **W-07, the values.** Findings 6, 7, 8 and 10. Effect fields drag through the same
   transaction the transform fields use and send as they are typed; a colour swatch and picker
   on the tint; the effect cards drag to reorder.
4. **Finding 9** is written down as a decision and not started.

## Anything else
