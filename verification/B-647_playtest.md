# B-647 playtest: a failed expression says so (D-767)

## What changed for you

When an expression cannot work on the frame you are looking at (a typing mistake, a name that is
not there, a loop between properties), the viewer now says so beside the frame, as it already did
for a missing font: a line such as "The expression on Box's opacity does not work at frame 0.
Correct the expression, or switch it off." in the notes, the warning count in the top bar goes up,
and the error details show its code (for example `EXPRESSION_REFERENCE_MISSING`).

The picture itself does not change: the property is still drawn at its keyed value, as before.
Before, that was all you got, so a wrong picture looked like a right one. Exporting such a frame was
already refused with the reason, and still is.

The checking tool `tools/app_snapshot.py` now marks such a shot FAILED, so a test render with a
broken expression can no longer pass quietly.

## What to look at

`verification/B-647 pictures/1_failed_expression_before_after.png`: the same project (a box whose
Opacity expression misspells "Checkbox" as "Checkbx"), rendered by the build before this change and
after it. The box is identical in both. Before: no note, verdict OK. After: the note
`EXPRESSION_REFERENCE_MISSING` and verdict FAILED.

Also rendered (not pictured): lane q's tutorial project (`scratchpad wave4/q/out/proj_ae.json`)
frames 0 and 24. Before: OK with no note. After: FAILED with `EXPRESSION_SYNTAX` (`marker.numKeys`
on "mark") and `EXPRESSION_REFERENCE_MISSING` (an `effect("Slider Control")` on a layer with no
Slider Control). Every pixel is identical before and after on all four frames compared.

`verification/B-647_failed_expression_table.md`: the app check, 6 of 6 pass (two broken expressions
give their note with the pixel at its keyed value; a working one gives no note).

## Try it

1. Add a solid. Alt-click its Opacity stopwatch and type `if (`. The solid stays fully visible, and
   a note appears beside the viewer: "The expression on ...'s opacity does not work at frame ...".
   Open the error details: it says `EXPRESSION_SYNTAX`.
2. Change the expression to `50`. The solid goes half see-through and the note goes away.
3. Type `effect("Slider Control")("Slider")` on a layer with no Slider Control: the note says
   `EXPRESSION_REFERENCE_MISSING`.

## Known differences (not fixed here)

- The note names the property and the frame, not the reason (for example which name was not
  found).
- A note shows for the frame on screen only; other frames are checked when you look at them or
  export.
