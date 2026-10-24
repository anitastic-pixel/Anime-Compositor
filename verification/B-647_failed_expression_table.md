# B-647 table: a failed expression says so (D-767)

Made by `a_failed_expression_says_so_beside_the_frame` in `app/src/tests/serving.rs`. A four-pixel red solid whose Opacity carries the expression; the window is asked for frame 0 as the page asks for it. The note is the code part of `x-frame-notes`, which the page lists under the error details and `tools/app_snapshot.py` reads as `frame_notes`.

| Check | Expected | Got | Result |
|---|---|---|---|
| Opacity typed `if (`, which cannot be read: the note beside the frame | EXPRESSION_SYNTAX: The expression on Box's opacity does not work at frame 0. Correct the expression, or switch it off. | EXPRESSION_SYNTAX: The expression on Box's opacity does not work at frame 0. Correct the expression, or switch it off. | pass |
| Opacity typed `if (`, which cannot be read: the first pixel | 255, 0, 0, 255 | 255, 0, 0, 255 | pass |
| Opacity typed `effect("Slider Control")("Slider")` with no Slider Control on the layer: the note beside the frame | EXPRESSION_REFERENCE_MISSING: The expression on Box's opacity does not work at frame 0. Correct the expression, or switch it off. | EXPRESSION_REFERENCE_MISSING: The expression on Box's opacity does not work at frame 0. Correct the expression, or switch it off. | pass |
| Opacity typed `effect("Slider Control")("Slider")` with no Slider Control on the layer: the first pixel | 255, 0, 0, 255 | 255, 0, 0, 255 | pass |
| Opacity typed `if (1 == 1) 50 else 0`, which works: the note beside the frame | no note | no note | pass |
| Opacity typed `if (1 == 1) 50 else 0`, which works: the first pixel | 255, 0, 0, 128 | 255, 0, 0, 128 | pass |

6 of 6 pass.
