# B-779: the viewer draws at the size it is shown (D-899)

Written by `b779_below_100_percent_the_viewer_draws_at_the_size_shown` in `app/src/tests/shown_size.rs`.

Below 100% zoom a still frame, or a drag's, is asked for at the coarsest quality that still has a picture pixel for every screen pixel, never finer than the one chosen; "shown at" is screen pixels per composition pixel. Ticking Full pixels keeps the quality chosen at any zoom. The first rows run the page's own `sizedAs` under Node; the others ask the real request handler on the reference shot.

**25 of 25 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| `/frame/7?q=full` shown at 100% | /frame/7?q=full | /frame/7?q=full | pass |
| `/frame/7?q=full` shown at 200% | /frame/7?q=full | /frame/7?q=full | pass |
| `/frame/7?q=full` shown at 60% | /frame/7?q=full | /frame/7?q=full | pass |
| `/frame/7?q=full` shown at 50% | /frame/7?q=half | /frame/7?q=half | pass |
| `/frame/7?q=full` shown at 30% | /frame/7?q=half | /frame/7?q=half | pass |
| `/frame/7?q=full` shown at 25% | /frame/7?q=draft | /frame/7?q=draft | pass |
| `/frame/7?q=full` shown at 10% | /frame/7?q=draft | /frame/7?q=draft | pass |
| `/frame/7?q=half` shown at 50% | /frame/7?q=half | /frame/7?q=half | pass |
| `/frame/7?q=half` shown at 25% | /frame/7?q=draft | /frame/7?q=draft | pass |
| `/frame/7?q=draft` shown at 25% | /frame/7?q=draft | /frame/7?q=draft | pass |
| `/frame/7?q=full` shown at any zoom, Full pixels ticked | /frame/7?q=full | /frame/7?q=full | pass |
| `/at/400?q=full` shown at 25% | /at/400?q=full | /at/400?q=full | pass |
| `/play/7?q=full` shown at 25% | /play/7?q=full | /play/7?q=full | pass |
| frame 30 at 50%, Full chosen: drawn at Half, a quarter of the pixels, and says it is not final | 960 540 Half true, 2073600 of 8294400 bytes | 960 540 Half true, 2073600 of 8294400 bytes | pass |
| the quality chosen stays Full for the window, the read-ahead and exports | Full false | Full false | pass |
| and Full asked after it is byte for byte Full asked for alone | true | true | pass |
| the page asks for the size shown | true | true | pass |
| which a picture of that size does not make the quality chosen | true | true | pass |
| the label says so | true | true | pass |
| a zoom asks again when the picture drawn is the wrong size | true | true | pass |
| but never for a stand-in, the card's Draft for In motion included, which is sharpened anyway | true | true | pass |
| only on the processor: a frame the card drew keeps the quality chosen, as Half there is no quicker | true | true | pass |
| which each frame's x-drawn-on says | true | true | pass |
| Full pixels is a tick in the quality menu, off unless ticked | true | true | pass |
| and remembered | true | true | pass |

## What this does not cover

How quick it is, which the lag harness measures (D-899's evidence), and how it looks, which is `verification/B-779_playtest.md`.
