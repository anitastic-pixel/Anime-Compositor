# T-07, export half — a project saved, reopened and exported to the same files

**23 of 23 checks passed.**

Produced by `tests/t07e_roundtrip_export.rs`. Closes the half of T-07 that document 11 line 80 said was still owed.

## What this is

Saving a project and getting the same text back is one thing, and it is already checked: `B-09_persistence_table.md`, 92 checks. Getting the same **picture** back is a different thing, and it is the one that would hurt. A field that the file quietly rounds, drops or reorders does not damage the text — it damages an export, weeks later, in a shot nobody thought to compare against anything.

So the same six frames were exported twice: once from the project as it was built, and once from the project read back out of a file on disk. Every pair had to match byte for byte, and one of them had to match a file that was exported yesterday and is already committed.

## What to look at

- **`T-07e_project.json`** and **`T-07e_reopened.json`** — the project as it was saved, and the same project after being reopened and saved again. Open both in any diff tool. They are the same file. If your tool reports a single differing line, something in this build is losing data.
- **`T-08_frames/shot_0012.png`** — one row here says that picture is exactly what comes out of a project that has been through the file format. It was not exported again for this table; it was compared against.

## What is still owed on T-07

**Q-01** — "no known reproducible project corruption in the release candidate" — is a statement about a release candidate, not a check that can be run. It stays open until there is one.

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| opening the saved project reports nothing wrong | `` | `` | pass |
| saving the reopened project reproduces the file it came from, byte for byte | `identical` | `identical` | pass |
| the reopened composition has the same size and length | `1920x1080, 240 frames from 0` | `1920x1080, 240 frames from 0` | pass |
| both layers came back, in the order they composite in | `layer-hidden, layer-cels` | `layer-hidden, layer-cels` | pass |
| the switched-off layer is still in the file and still switched off | `false` | `false` | pass |
| the exposure sheet came back whole: 120 spans, two frames each | `120 spans, first drawing 0 over frames 0 to 1` | `120 spans, first drawing 0 over frames 0 to 1` | pass |
| layer 3's asset still has no drawing 7, because that file is a deliberate defect | `0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11` | `0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11` | pass |
| the project in memory exports six frames | `Completed, 6 files` | `Completed, 6 files` | pass |
| and the project read back out of the file exports six frames too | `Completed, 6 files` | `Completed, 6 files` | pass |
| both report the same warnings: the two frames whose drawing is missing | `MEDIA_SEQUENCE_GAP, MEDIA_SEQUENCE_GAP` | `MEDIA_SEQUENCE_GAP, MEDIA_SEQUENCE_GAP` | pass |
| shot_0012.png is the same file whether it was exported before or after the round trip | `identical` | `identical` | pass |
| shot_0013.png is the same file whether it was exported before or after the round trip | `identical` | `identical` | pass |
| shot_0014.png is the same file whether it was exported before or after the round trip | `identical` | `identical` | pass |
| shot_0015.png is the same file whether it was exported before or after the round trip | `identical` | `identical` | pass |
| shot_0016.png is the same file whether it was exported before or after the round trip | `identical` | `identical` | pass |
| shot_0017.png is the same file whether it was exported before or after the round trip | `identical` | `identical` | pass |
| the frames exported from the file are the frames T-08 committed as its artifact | `identical` | `identical` | pass |
| frame 12 exported from the file shows drawing 6's bar at half opacity | `R 255, B 0, A 128` | `R 255, B 0, A 128` | pass |
| frame 14, which asks for the drawing the fixture does not have, is empty there | `R 0, B 0, A 0` | `R 0, B 0, A 0` | pass |
| and frame 16, two drawings later, has moved the bar two bar-widths right | `R 0, B 0, A 0` | `R 0, B 0, A 0` | pass |
| which is where drawing 8's bar now is | `R 255, B 0, A 128` | `R 255, B 0, A 128` | pass |
| by default the same range is refused, and nothing is written | `Blocked, 0 files` | `Blocked, 0 files` | pass |
| naming the frames whose drawing is missing | `EXPORT_BLOCKED_MISSING_MEDIA: Frames 14 to 15.` | `EXPORT_BLOCKED_MISSING_MEDIA: Frames 14 to 15.` | pass |
