# B-12a: looking at the alpha channel, and the grid behind the frame

W-01 ends with the artist inspecting alpha, and document 05 line 31 lists the two views this is about: the transparency grid, and the alpha-only display. Both are in the viewer now, as document 24's `viewer.toggle_checkerboard` and `viewer.toggle_alpha`. Produced by `cargo test -p anime_compositor_app`, from `app/src/main.rs`.

The promise these rows are about is document 21 line 97's, which R-10 states as a requirement: **checkerboard and alpha-only inspection must not alter export.** A view that quietly leaked into the file, or into the frames the preview keeps, would be found by whoever opened the exported sequence, long after the person who turned it on had forgotten it was on. So the grid is drawn by the page behind a canvas that already carries transparency and touches no pixel at all, and the alpha view is applied to the copy of the frame that is on its way to the screen, after the cache has been handed the picture it keeps.

The fixture is four pixels of half-transparent red, drawn by this test. The reference shot cannot be used for this: every pixel in it is either opaque or empty, so an alpha view of it that was subtly wrong would still look right.

| Check | Expected | Actual | Result |
|---|---|---|---|
| the window opens looking at the picture itself | false | false | pass |
| half-transparent red arrives as the red it was drawn as | 255, 0, 0, 128 | 255, 0, 0, 128 | pass |
| and the transparency grid starts on, because a transparent frame that reads as black is a frame somebody misjudges | true | true | pass |
| turning alpha inspection on says what will be on screen and what will not change | Alpha-only inspection is on. The picture is the alpha channel, white where the frame is opaque and black where it is empty; what is exported is unchanged. | Alpha-only inspection is on. The picture is the alpha channel, white where the frame is opaque and black where it is empty; what is exported is unchanged. | pass |
| and the frame that comes back says it is being looked at that way | true | true | pass |
| a pixel that is half transparent is drawn as the grey half way up | 128, 128, 128, 255 | 128, 128, 128, 255 | pass |
| the alpha view is opaque, so what is being measured cannot itself be see-through | true | true | pass |
| it is the same picture, at the same size | 4x4 | 4x4 | pass |
| turning it off says so | Alpha-only inspection is off. | Alpha-only inspection is off. | pass |
| and the frame comes back byte for byte as it was, so what the cache kept was the picture and never the view | true | true | pass |
| looking at the alpha channel is not unsaved work | false | false | pass |
| and it is not in the undo history either, which is what document 24's own table says: undoable, no | 0 | 0 | pass |
| turning the grid off says what it was | The transparency grid is off. | The transparency grid is off. | pass |
| the frame says the grid is off | false | false | pass |
| and not one byte of the frame is different, because the grid was never in it | true | true | pass |
| turning it back on says so too | The transparency grid is on. It is drawn behind the frame and is not part of it. | The transparency grid is on. It is drawn behind the frame and is not part of it. | pass |
| an export writes the frames of the work area | 1 | 1 | pass |
| and exporting while looking at the alpha channel writes the same file, byte for byte, which is what R-10 asks for | true | true | pass |

**18 of 18 checks pass.**

## What this does not cover

What the grid looks like. It is eight-pixel squares of two greys in the page's stylesheet, fixed to the screen rather than to the picture so that zooming does not stretch them, and no test can see it. The photographs of the window are where it is judged.

Zoom and pan, which document 05 lists in the same line: since W-06 the page zooms with the wheel and scrolls the stage around it, and the grid, fixed to the screen, is what says the picture and not the window was zoomed. No test can see that either.
