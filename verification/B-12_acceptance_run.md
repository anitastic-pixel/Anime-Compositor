# B-12: the acceptance run, step by step

This is the sheet for the run itself. B-12 is the owner completing W-01 and W-02 on the reference
shot unaided; document 05 line 69 is the bar — *"complete W-01 without assistance, with all
required controls reachable by keyboard"* — and the artifact document 15 asks for is the finished
shot and a written account of what was awkward.

Everything under `verification/B-12a_*` and `B-12b_*` says that each command does what it claims
when something asks for it correctly. None of it says the run can be completed by a person. That
is what this page is for, and it is written to be used while sitting in front of the window: for
each of W-01's thirteen steps, where the control is, what to do, what a correct result looks like,
and what a wrong one looks like. **Nothing here is a test.** If a step does not behave as this
page says, the page or the build is wrong and both are worth knowing about; write down what
happened rather than working around it.

## Before starting

**The build.** `cargo build -p anime_compositor_app --release`, then run
`target/release/anime_compositor_app.exe`. The window opens on the reference shot, because that is
what it opens with nothing else asked for.

**The project.** Use the reference shot: 240 frames, 24 fps, 1080p, four layers, with drawing 7 of
layer3 deliberately absent. Save a copy first — **Save As…** to somewhere under `target/`, so the
run cannot damage `Fixtures/`.

**What to write down as you go**: how long each step took, anything you had to hunt for, anything
you expected to be there and was not, anything you did by accident. Awkwardness is the finding
here, not a complaint. A step you could not complete at all is the most valuable line on the sheet.

**The step that used to have no control.** Step 3, creating a composition, had none when this
sheet was first written: a composition came from the fixture scripts and this window only edited
one that already existed. B-12d built it, and step 3 below is now a step to take like the rest.
Two things about it are worth knowing before the run. It makes a composition **inside the project
already open** — there is still no New Project, so open the reference shot first. And it is a
**new capability the owner may cut**: nothing else in the build depends on it, and if the answer
after the run is that a composition should come from somewhere else, saying so costs nothing.
`verification/B-12d_new_composition_table.md` is what it does, checked.

## The thirteen steps at a glance

| # | W-01 step | Where |
| --- | --- | --- |
| 1 | Import media | **Import drawings…**, top left, or Ctrl+I |
| 2 | Review sequence grouping and missing-frame warnings | the DRAWINGS list, and the notes along the bottom |
| 3 | Create a composition | **New composition…**, top left, or Ctrl+Shift+N |
| 4 | Assign exposures | EXPOSURES, right-hand column |
| 5 | Stack layers | **Add layer**, **Forward**, **Back** on the bottom bar; the layer list above it |
| 6 | Adjust anchors and transforms | LAYER, right-hand column |
| 7 | Apply a matte | LAYER, the **Matte** chooser at the bottom of it |
| 8 | Add one blur | EFFECTS, **Add effect… → Blur** |
| 9 | Add one colour operation | EFFECTS, **Add effect… → Tint** (or Exposure) |
| 10 | Inspect alpha | **Alpha only** under the picture, or A |
| 11 | Preview the work area | **Play**, the arrows and the frame number, or space and the arrow keys |
| 12 | Save, close, reopen | **Save**, **Save As…**, **Recent…** on the bottom bar; Ctrl+S |
| 13 | Export a PNG sequence | **Export…**, or Ctrl+M |

Every one of these can be reached with Tab and answered with Enter or the space bar; which keys
each control takes is `verification/B-12c_keyboard_table.md`, and the three gestures that are
genuinely mouse-only are named there too.

## 1. Import media

**Where.** *Import drawings…* under the DRAWINGS list at the top left. Ctrl+I does the same.

**What to do.** Choose the PNG files of one numbered sequence. Windows draws the dialog, so it is
the file dialog you already know; more than one file at a time is expected.

**Right.** A new row appears in DRAWINGS with the sequence's name and its frame count, and the
status line says what was imported. The name comes from the files, not from the folder.

**Wrong.** Files chosen and no row appearing; a row whose frame count is not the number of files
you chose; two sequences merged into one row, or one sequence split into two.

## 2. Review sequence grouping and missing-frame warnings

**Where.** The DRAWINGS list itself, and the notes along the very bottom of the window.

**What to do.** Read the row, then read the notes. This step is looking, not doing.

**Right.** *layer3* and *11 frames* on the row, where twelve files were numbered 0 to 11 with 7
absent. A note along the bottom naming the gap. A frame that asks for the missing drawing renders
without that layer and says so, rather than showing the drawing either side of it.

**Wrong.** A count that hides the gap by counting to the highest number. A gap silently filled by
sliding the following drawings up — the thing W-02 forbids and the reason the reference shot has
a hole in it on purpose. Twenty identical warnings for a 240-frame shot: the rate limit should
give you a few and then a summary with the frame ranges.

## 3. Create a composition

**Where.** **New composition…** at the top of the left-hand column, above DRAWINGS, or Ctrl+Shift+N.

**What to do.** The button opens five fields — a name, a width, a height, a frame rate and a
length in frames — filled in with 1920 by 1080 at 24 fps for 240 frames, which is the reference
shot's own shape. *Create it* makes it; *Leave it* closes the fields and makes nothing. The new
composition becomes the one on screen, and the line above the button says which one that is.

**Right.** The composition line changes to the name you typed and the size, rate and length you
asked for. The picture goes empty, and the window says so in words rather than leaving you looking
at a blank rectangle wondering whether something broke. The composition the project already had is
still in the project — making one does not replace one. Ctrl+Z takes it back and puts you on the
composition you were on before.

**Wrong.** A blank picture with nothing said. A refusal with no sentence — a zero width, a
ridiculous size and a ridiculous length are all meant to be turned down in a sentence that names
the limit, and a control that just springs back is the failure. The composition you had being
replaced rather than added to. Undo leaving the window on a composition that is no longer there.

**Worth writing down.** Whether 240 frames at 1920 by 1080 is the right thing to be offered
before you have said anything, and whether five fields is too many to be asked at once.

## 4. Assign exposures

**Where.** EXPOSURES, the middle section of the right-hand column, with a layer selected.

**What to do.** *Add an exposure* appends a span after the last one. Each row is *frames* **from**
**to**, then *show drawing* **n**, then ✕ to remove it. Type into a field and press Enter or Tab
out of it; nothing is sent while you are still typing.

**Right.** The picture changes to the drawing you named as soon as the field is committed, and the
frame counter under the picture stays where it was. Two frames on one drawing is a hold, which is
what the exposure sheet is for, and holds survive everything afterwards.

**Wrong.** A span you typed being renumbered by the window. A change taking effect on every
keystroke rather than when you commit it. The ✕ removing a different row than the one you clicked.
Undo (Ctrl+Z) leaving the panel and the picture disagreeing.

## 5. Stack layers

**Where.** *Add layer* on the bottom bar, with a sequence chosen in DRAWINGS. *Forward* and *Back*
(Ctrl+] and Ctrl+[) move the selected layer. The layer list sits above the bar: the eye hides a
layer, the padlock locks it, F2 renames, Delete deletes.

**Right.** The new layer appears at the front, named for the sequence, and the picture gains it.
The list reads front to back, so the top row is what is drawn last and covers the others.

**Wrong.** *Add layer* enabled with nothing chosen. A layer added to the wrong end. The list order
and the picture disagreeing about which layer is in front. A hidden layer still visible, or a
locked layer still editable.

## 6. Adjust anchors and transforms

**Where.** LAYER, the top of the right-hand column: Anchor, Position, Scale, Rotation, Opacity,
each a number you can type into, each with a handle beside it you can drag.

**What to do.** Both ways are worth trying, because they behave differently on purpose. Typing a
number is one undo step when you commit it. Dragging is one undo step for the whole drag, however
far it goes. With the keyboard, focus a handle and use the arrow keys; Shift makes each press ten
times larger.

**Right.** The picture follows the drag continuously while the mouse is down, and one Ctrl+Z puts
back everything that drag did. A value the rules refuse — a scale of zero, say — is refused in a
sentence naming the rule, and the field goes back to what it was.

**Wrong.** A drag leaving twenty entries in the history. A refusal that leaves the field showing a
number the project does not have. Rotation or scale pivoting somewhere other than the anchor.

## 7. Apply a matte

**Where.** LAYER, at the bottom: the **Matte** chooser, and the *matte only* checkbox beside it.

**What to do.** Choose another layer as this layer's matte. *matte only* controls whether the matte
layer is still drawn in its own right.

**Right.** The chooser offers only layers that are allowed to be a matte, so an arrangement that
would be a cycle cannot be built by choosing it. The layer is shaped by the matte's alpha
immediately. Choosing *none* clears it.

**Wrong.** A layer offering itself. A choice that is accepted and then refused by the renderer at
frame time. The matte applied but the matte layer's own drawing left in the picture when
*matte only* is ticked.

**Not in this build:** drawing a mask outline. W-01 asks for a matte, which is one layer shaping
another, and that is what this step is. Drawing a polygon by hand is deliberately out of B-12a's
scope; a mask that is already in the project file is honoured.

## 8. Add one blur

**Where.** EFFECTS, the bottom of the right-hand column: *Add effect…* → **Blur**.

**Right.** The blur is added at the setting that changes no pixels, so the picture does not jump
when you add it — the first change you see is the one you typed. *Radius σ, in source pixels* takes
a number. The dot beside the name bypasses the effect, the ✕ removes it.

**Wrong.** The picture changing the moment the effect is added. A number outside the range being
quietly turned into a different one instead of refused in words. The order of the stack changing
by itself.

## 9. Add one colour operation

**Where.** The same chooser: **Tint**, or **Exposure**.

**Right.** Tint takes a colour in linear RGB and an amount from 0 to 1; both travel together on
every change, so setting one does not reset the other. The effects apply in the order they are
listed, top first.

**Wrong.** A setting resetting when you change the other one. Two effects with the same identifier
after you have deleted one from the middle of the stack.

`verification/B-12c_effects_panel.png` is a photograph of this panel with both effects on a layer,
for comparison.

## 10. Inspect alpha

**Where.** *Alpha only* under the picture, or A. *Hide grid* (G) turns the transparency
checkerboard off.

**Right.** The picture becomes the alpha channel: white where the frame is opaque, black where it
is empty. The status line says so. **Nothing about the project changes** — the title does not gain
an unsaved-work mark, and what is exported is unaffected. The grid is drawn behind the frame and
is never part of it.

**Wrong.** Alpha inspection marking the project dirty. The checkerboard appearing in an exported
frame. The alpha view surviving into the export.

## 11. Preview the work area

**Where.** *Play*, the two arrows and the frame number under the picture. Space plays and pauses,
the arrow keys step one frame, D switches between draft and full resolution.

**Right.** Playback runs the whole work area and loops. The frame number is the frame you are on.
Draft says in words that it is a draft and not final pixels, which is the one thing a preview must
never lie about. Dropped frames are reported rather than hidden.

**Wrong.** The frame number and the picture disagreeing. A draft frame presented as final.
Playback that misses the deliberately absent drawing 7 rather than rendering that frame without
layer3.

## 12. Save, close, reopen

**Where.** *Save* and *Save As…* on the bottom bar, Ctrl+S and Ctrl+Shift+S. *Recent…* beside them
reopens. A project file can also be dropped onto the window.

**Right.** The bar names the file you are on and marks unsaved work. Save writes back to that file
and the mark clears; a second Save with nothing changed writes an identical file. Save As writes
where it was told and the window is then on the new file. Reopening gives back everything,
including the effect order and anything in the file this build does not understand.

**Wrong.** A save that appears to work and writes nothing (this happened once and is
`verification/B-12b_save_works.md`; the check that would have caught it now exists). A reopened
project missing an effect, or having lost the settings of an effect this build cannot draw. A
failed save that leaves a half-written file, or that loses the last good one.

## 13. Export a PNG sequence

**Where.** *Export…* on the bottom bar, or Ctrl+M. The checkbox beside it — *Write frames whose
drawing is missing* — is off by default and should stay off for the first attempt.

**Right.** The export is **refused**, in a sentence, because the reference shot asks layer3 for
drawing 7 on twenty of its frames and that drawing does not exist. Nothing is written. That refusal
is the specified behaviour, not a defect: a hole in the artwork is not something a compositor is
allowed to paper over silently. Tick the checkbox — which says in words what it does — and the
export then runs, writing 240 files named for the project and a four-digit frame number, and still
reporting each missing drawing. The window says it is exporting and offers to cancel; cancelling
between frames keeps every frame already finished.

**Wrong.** An export that succeeds with the checkbox off. A missing frame quietly written as
transparent with no report. Fewer or more than 240 files, or a gap in the numbering. Editing the
project while the job runs changing what the job writes — the export takes a copy on purpose.

## After the run

The three things B-12 owes: **the finished shot**, **the written account of what was awkward**, and
the decisions this run forces. `verification/B-12c_owner_brief.md` is the brief for the three that
are already open and waiting.

## What this page is not

It is not evidence. Every table under `verification/` is produced by a test and can be re-run;
this is a script for a person, and the only thing it produces is what the person writes down.
It is also written from the build of 2026-09-08 — if a control moves, this page goes stale
silently, in the same way the photographs did, and the fix is to correct it here when it does.
