# B-621 playtest: the picture tool says FAILED when a render is not the project's

## What changed for you

Nothing in the app. The tool Claude uses to take pictures from the real app,
`tools/app_snapshot.py`, now ends with **FAILED** (and exit code 1) when the project did not
open, a drawing was missing, or the app reported another note that means the picture is wrong.
It still writes every picture and file, so the wrong picture can be looked at. A good run ends
with **OK**.

## What to look at

`verification/B-621_snapshot_failures_table.md` has three real runs: one OK and two FAILED.

- `B-621 pictures/ok_add_grain.png`: the project as asked for, with grain. The tool said OK.
- `B-621 pictures/failed_cannot_be_opened.png`: the project could not be opened, but the app
  still showed a picture without grain. Before this change the tool called this a success. Now it
  says FAILED.
- `B-621 pictures/failed_media_missing.png`: the drawings were not there. The tool says FAILED.

## Nothing to try in the app

Any comparison Claude shows you from now on came from a run that ended with OK.
