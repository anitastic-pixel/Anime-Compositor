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

## The decisions this run forces

Three are already open and waiting in `verification/B-12c_owner_brief.md`. Two more come from
B-12d:

- **Whether `composition.create` stays.** It is a new capability, not a fix. Nothing depends on
  it and cutting it is a revert.
- **The three size limits** — 16384 a side, 67108864 pixels, 10000 frames. Document 19 said
  "bounded by implementation safety limits" and never gave numbers; these are the build's, not
  the specification's.

## Anything else
