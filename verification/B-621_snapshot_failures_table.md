# B-621: the picture tool says FAILED when a render is not the project's

Audit row 3 (22 October 2026). `tools/app_snapshot.py` used to finish with exit code 0 when the
project did not open or a drawing was missing, so a wrong picture could pass for a right one.

Three real runs, 2026-10-23, with the "before" exe (built from fee505f4; the app is unchanged by
B-621, only the tool is), `--frames 0 --on cpu`, project
`verification/app_snapshots/add_grain/add_grain.json`:

| Case | What was done | Tool's last line | Exit code | Pictures still written |
|---|---|---|---|---|
| Good | as is | `...; OK` | 0 | yes |
| Drawings missing | `--media` pointed at an empty folder | `frame_0000_cpu: ... FAILED: note MEDIA_MISSING` then `...; FAILED` | 1 | yes |
| Cannot be opened | a copy with one layer's blend mode set to `bogus` | `frame_0000_cpu: ... FAILED: cannot be opened: This project file cannot be opened, because part of it does not match the project format. ...; note PROJECT_SCHEMA_INVALID` then `...; FAILED` | 1 | yes |

Before B-621 the last two printed no warning and exited 0.

The "cannot be opened" case is the dangerous one: the app still shows a full, believable picture
(`B-621 pictures/failed_cannot_be_opened.png`), but it is not the project that was asked for: it
has no grain.

What counts as failed (`problems()` in the tool):
- the shot has an `error`;
- "cannot be opened" in the report, the status line or a note;
- a note whose code is in `FAIL_CODES`: the document 28 codes that mean the picture is not what
  the project asks for (MEDIA_MISSING, MEDIA_SEQUENCE_GAP, PROJECT_SCHEMA_INVALID,
  EFFECT_UNSUPPORTED, a missing or circular matte, parent, precomp or effect layer, a missing
  font, and the rest listed there). Information-only notes such as PROJECT_RECOVERY_AVAILABLE
  do not fail a run.

`python -I tools/app_snapshot.py --self-check` checks this rule without starting the app:
`self-check passed`.

No script in the repository calls the tool, so no caller needed changing for the new exit code.
