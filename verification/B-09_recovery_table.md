# B-09, autosave and recovery in the window

The core has known how to write a recovery snapshot for some time; until now nothing called it. This is the window's half — a clock that decides when, and a way back in from a snapshot. Produced by `cargo test -p anime_compositor_app`, from `app/src/main.rs`.

The promise is document 07's, in two parts. **A snapshot is written after two minutes of unsaved work, and it never overwrites the last manual save.** And **recovering from a snapshot does not save it**: the window comes back pointing at the project file with the recovered work outstanding, so the person decides whether it becomes the project. The two minutes are a parameter here rather than a wait, which is the only thing about this table that is not what the window does.

| Check | Expected | Actual | Result |
|---|---|---|---|
| a project with nothing outstanding writes no snapshot, however long it sits | nothing | nothing | pass |
| there is now unsaved work | true | true | pass |
| one minute of unsaved work is not enough (document 07 asks for two) | nothing | nothing | pass |
| and no file has appeared beside the project | false | false | pass |
| after two minutes a snapshot is written, in the first free slot | Recovery snapshot written to <a temporary directory>\shot.autosave-0.json | Recovery snapshot written to <a temporary directory>\shot.autosave-0.json | pass |
| the work is still unsaved afterwards (document 26) | true | true | pass |
| and the project file has not been touched (document 07) | unchanged | unchanged | pass |
| the snapshot keeps the effect this build does not understand | true | true | pass |
| six snapshots leave five files, not six | 5 | 5 | pass |
| and the window is offering all five to recover from | 5 | 5 | pass |
| recovering says which snapshot was opened | Recovered <a temporary directory>\shot.autosave-0.json | Recovered <a temporary directory>\shot.autosave-0.json | pass |
| the recovered work is what was in the snapshot | Renamed again, 5 | Renamed again, 5 | pass |
| Save would write to the project, not back into the snapshot | <a temporary directory>\shot.json | <a temporary directory>\shot.json | pass |
| and the window calls the project by its own name | shot.json | shot.json | pass |
| the recovered work counts as unsaved, because the project file does not have it | true | true | pass |
| recovering wrote nothing: the project file is still byte for byte what it was | unchanged | unchanged | pass |
| saving after a recovery writes the project | Saved to <a temporary directory>\shot.json | Saved to <a temporary directory>\shot.json | pass |
| the project file now holds the recovered work | true | true | pass |
| and there is nothing outstanding any more | false | false | pass |
| a project with no file of its own writes no snapshot; there is nowhere beside it | nothing | nothing | pass |
| and it cannot be recovered into either | There is no project to recover into. | There is no project to recover into. | pass |

**21 of 21 checks pass.**

## What this does not cover

The two minutes passing. The timer is a thread that sleeps ten seconds at a time and asks the same question this table asks; what is checked here is the question, with the clock supplied. A thread that never started would not fail this table, and only the running window shows that it did.

**Nothing in this window makes a project dirty yet.** It is a viewer: it opens, shows, plays and saves, and no control in it changes a project. So in ordinary use today the timer has nothing to write, and it stays quiet. What it protects is the editing that B-13 onwards adds, and it is built now because the alternative is building it after the first afternoon somebody loses.

Where a row says *a temporary directory*, the real value was this machine's scratch directory, which differs on every machine and every run.
