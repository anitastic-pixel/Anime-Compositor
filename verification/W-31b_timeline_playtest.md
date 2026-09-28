# W-31b: the timeline, built (playtest)

Written on 2026-09-28. You accepted the drawings in `verification/W-31a_timeline_proposal.md`
(D-176, "ok, let's accept"). This is the same timeline in the real program. The pictures were
taken of the real window, with `verification/B-08a_project.json` open, then ` pressed so the
timeline fills the window.

## Pictures

At 100 per cent display scaling:

![The timeline at 100 per cent](W-31b%20pictures/timeline_100.png)

At 150 per cent:

![The timeline at 150 per cent](W-31b%20pictures/timeline_150.png)

At 200 per cent:

![The timeline at 200 per cent](W-31b%20pictures/timeline_200.png)

The whole window at each size, not maximised: `W-31b pictures/window_100.png`, `window_150.png`
and `window_200.png`.

## What to check, point by point

| # | What was accepted | Where to look |
|---|---|---|
| 1 | One header row | Timeline/Graph switch, the frame number in blue, then New layer, Hide shy, the zoom and "frames 0–239 of 240". At 200 per cent the row wraps onto two lines. |
| 2 | One New layer menu | Click **+ New layer ▾**. Drawings, Adjustment layer, Solid, Shape layer, Null. Drawings is greyed and says why until you choose drawings in the Project panel. |
| 3 | Delete, Forward and Back leave the header | They are gone from the header. Right-click a layer, press Delete, Ctrl+] or Ctrl+[, or find them in Ctrl+Shift+P. |
| 4 | Column headings | The eye, solo, lock, #, Layer, shy, Mode and Parent sit over their columns. |
| 5 | One set of icons | Eye, solo and lock are drawn the same way. Solo turns orange when on. |
| 6 | Timing tag | layer4 *irregular*, layer3 *on 2s*, layer2 *on 1s*, layer1 *held*. |
| 7 | Drawings easier to count | Two blue shades alternate. layer1's one drawing is a green bar with its number, 0. Keys, where a layer has them, are small marks on the lower edge. |
| 8 | Missing drawing striped red, Relink… | layer3's red stripes and its red **Relink…** button. |
| 9 | Seconds on the ruler, flag on the playhead | "1s · 24", "2s · 48" …; the red flag reads 0. Play or scrub and it follows. |
| 10 | Quiet mode and parent | "normal" and "None" in grey text. Choose another mode and it turns orange. |
| 11 | Empty state | Not pictured: make a new composition. It says it has no layers and offers Import drawings… (Ctrl+I) and + New layer. |
| 12 | Columns at most 45 per cent | At 200 per cent the bars keep more than half the width. |

## Where it differs from the drawings

- **At 200 per cent the Parent column shows only its whip**, not the parent's name, so the layer's
  own name still fits. Point at the whip to read the parent. From 150 per cent up, it shows the
  name as drawn.
- **At 200 per cent the timing tag and Relink… shorten** ("irregul…", "Relink…" cut off) before
  the layer's name does. Both still work.
- **The red stripes mean a drawing number the sequence does not have.** The report under the
  panels also says some files are "not where the project expects them" for all four layers. That
  second kind of missing is not drawn on the bar, today or before. Say if you want it to be.
- **At 150 and 200 per cent the report under the panels takes half the window.** That strip is
  W-36's screen and is left as it is here.

## What did not change

- Every command the timeline sends. Document 24 is unchanged.
- Every shortcut, right-click menu and drag.
- The keyboard-reach check, `verification/B-12c_keyboard_table.md`, passes. It now lists the three
  new buttons (New layer and the empty state's two), and nothing in it says **no** that did not
  before.
- All 70 of the program's own tests pass.

## What to answer

"accept", or the numbers above you want changed and how.
