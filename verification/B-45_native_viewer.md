# B-45: the graphics card paints the viewer's picture into the window

Built on 2026-09-26 at the owner's word ("proceed with the native viewer surface"). D-102, proposed, is the decision it rests on. **It changes no export, no fixture and no CPU picture.** The switch still starts on CPU, and with the switch on CPU nothing here runs.

## What changed

- **Before (B-44):** with the switch on GPU, the card drew the picture and then copied it back into the computer's memory. The page then copied it again onto its own canvas. At Full that is 8 MB there and 8 MB back for every frame.
- **Now:** the card paints the picture straight into the window. The page lies on top of it, see-through only where the picture is, so the panels, scrollbars, menus and the command search still draw on top as before.
- **What the page tells the card:** the page tells the card where the picture lies and what is under it: the background colours and the checkerboard. Zooming or scrolling the viewer sends only that, and the picture already on the card is painted in its new place without being made again.
- **If the card cannot paint the window:** it stops trying for the rest of the session. The status line says why, and the switch's tooltip ends with "not painting the window: …". The picture then goes to the page exactly as in B-44. It never silently shows something else.

## How to judge it: the same picture

The same running app photographed the screen twice for each layout:

1. first with the card's picture handed to the page (B-44's way)
2. then painted by the card (B-45's way)

The viewer's area of the two photographs is compared pixel by pixel. The pictures are in `B-45 pictures/`: `…_page.png` is B-44's way, `…_card.png` is B-45's way, and `…_difference.png` is black where they agree, with a white square around every pixel where they do not.

| Layout | Pixels compared | Pixels that differ | Largest difference (of 255) | Why they differ |
|---|---:|---:|---:|---|
| Fit, Draft | 238,209 | 775 | 152 | Columns where a screen pixel falls exactly between two picture pixels: the page and the card take neighbouring ones |
| Zoomed to 300% and scrolled | 238,209 | **0** | 0 | — |
| Alpha only, one layer soloed | 238,209 | 30 | 27 | The same in-between columns, on the soft edge of the circle |
| Fit, Full | 238,209 | 656 | 70 | The same in-between columns |
| Checkerboard, one layer soloed | 238,209 | 3,932 | 27 | 3,908 of them by 1 level: the page and the card round a see-through pixel over the checkerboard differently. The rest are the in-between columns |

**About the in-between columns.** At a fit of 456 screen pixels for 480 picture pixels, every 19th screen column lands exactly between two picture pixels. The card now takes the first of the two, as the page does, which I measured. The page is not fully consistent about it: at some sizes it takes the second one every third time. Where they disagree, that column shows the picture pixel next door. The picture shifts by one pixel in one column, and no colour changes. At zoom 300% there are no such columns, and the two ways agree exactly.

**Around the picture**, the panel's background, the scrollbars and the viewer's edges matched exactly in the fit layouts.

## How fast it is

Measured with the session log (P-19). The machine and settings:

| | |
|---|---|
| Processor | AMD Ryzen 9 9900X |
| Graphics card | RTX 4070 Ti SUPER, driver 610.74, Vulkan |
| System | Windows 11 Education 10.0.26200 |
| Build | release build, rustc 1.89.0 |
| Window | 1600×1000 at 150% display scale |
| Shot | the reference shot at Full (1920×1080) |

Each run pressed Play, played for 8 seconds, and paused.

| | B-44's way (picture back to the page) | B-45's way, fresh launch | B-45's way, after the B-44 run |
|---|---:|---:|---:|
| Frames the clock skipped | **82** | **0** | **0** |
| Frames answered | 110 | 990 | 987 |
| Median time to answer a frame | 18.3 ms | 0.80 ms | 0.74 ms |
| Slowest 5% | 28.7 ms | 7.2 ms | 6.3 ms |
| "GPU: paint the picture into the window", in the slowest frames | — | 0.23 to 0.26 ms | 0.14 to 0.20 ms |

**How to read "frames answered":** the page asks for a frame once per screen refresh, and asking again for the frame already showing is cheap. The number to read is **frames skipped**. At Full, B-44's way could not keep up and skipped 82 frames in 8 seconds. B-45's way skipped none.

Resizing the window, minimising it and restoring it were also checked by photograph. The picture followed the window, and a frame stepped while minimised appeared on restore.

## Limits

- **Text looks slightly different while the card paints.** Windows smooths text edges with a little colour (ClearType), and it cannot do that over a see-through page. Text in the window is therefore smoothed in grey while the switch is on GPU. You can see it only zoomed right in. Switching to CPU brings ClearType back.
- **One machine.** Only this graphics card and driver, through Vulkan, have been tried. On another card, if painting the window fails, the card says so and hands the picture to the page as in B-44.
- **No second try.** Once the card has stopped painting the window, it does not try again until the app is reopened.
- **The page's background colours are read once.** A panel whose colour changed while the switch was on would be painted in its old colour behind the picture. No panel under the picture changes colour today.
- **The harness is not in the repository.** It drove the app through its developer port and photographed the screen. It can be run again on request.

## Checks run

- **The app's tests:** 65 pass, 3 ignored. The page and routes tables now list `/place`, the page's way of telling the card where the picture lies:
  - `B-12b_routes_table.md`: 36 of 36
  - `B-12b_page_table.md`: 123 of 123
  - `B-12c_keyboard_table.md`: 142 of 142
- **The GPU preview tests,** including the long one: 2 of 2. `B-44_gpu_preview_table.md` is unchanged: the card's picture before it reaches the window is the same as B-44b's.
- **The core tests:** all 78 test groups pass, none fail.
