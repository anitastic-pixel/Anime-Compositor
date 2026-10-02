# W-41a: the viewer's row, slate, guides and sheet strip (D-248)

The third screen of the redesign, first part. The viewer now matches the Sandbox's Viewer A:

- one row of small icons **above** the picture, in place of the row of word buttons under it;
- the cut's **slate** just above the picture;
- a **grid** and a **field guide** that can be laid over it;
- a **sheet strip** under it.

None of these goes into the picture's pixels, the project file or an export. They are looks kept with your preferences, like Hide shy layers.

## Before, after, and the Sandbox

| | Picture |
|---|---|
| Before W-41a | `W-40c pictures/scale_1.0.png` |
| The Sandbox board it copies | `W-38 pictures/sandbox_compose.png` |
| The real window, playing | `W-41a pictures/scale_1.0.png` |

In `scale_1.0.png`, left to right along the row:

| What | What it shows |
|---|---|
| ⏮ ⏸ ⏭ | step back, play/stop (lit while playing), step forward |
| **146** | the frame number |
| 6s 2f | the time |
| **● 24.0** | in green: how fast it really plays |
| **● Draft, not final** | in orange, with a ▾ menu |
| Auto | what the preview is drawn on |
| four look buttons | checkerboard (lit), alpha, grid, field guide |
| 47%, fit ▾ | the picture size menu |
| two buttons | slate (lit) and sheet strip (lit) |

Around the picture:

- Above it, the slate reads "**reference shot** · sheet 6 + 2".
- Under it, the strip reads "Sheet · layer1, held", with frames 139 to 154 and frame 146 outlined.
- The tab at the top reads "VIEWER  reference shot", with "1920×1080 · 24 fps · 240 frames" at its right.

## What it does, checked in the running app

| Step | What the page said |
|---|---|
| Opened | play not lit; fps "24" in grey (the composition's rate); quality "Draft, not final", with the full sentence as the tip ("Draft — 480×270, not final pixels, drawn on GPU…") |
| Play for 3 seconds | play lit; fps "24.0" in green; tip "Playing at 24.0 of the composition's 24 frames a second" |
| Stop | play not lit; fps back to "24" in grey |
| Grid, field guide, Alpha only | all three shown and lit; the guides exactly the picture's size (1219.56 by 686 px) |
| The quality ▾ | opens with Full resolution and Refine when idle |
| The size ▾ | opens with Fit, Fit ≤100%, Picture at 100%, Zoom in, Zoom out and the zoom slider |
| A cell of the strip | goes to that frame; the outline moves to it |
| Errors on the page | none |

## Kept as it was

- **Every control keeps its name and what it sends.** The checkerboard and Alpha only buttons still send the same two commands.
- **The quality is still never subtle and never absent (D-33).** The chip says "not final" in orange whenever the picture differs from an export.
- **A CPU picture is still said beside the chip.** "drawn on the CPU, not the card" appears there whenever a frame the card should draw was drawn on the CPU.
- **The motion blur and frame blending switches** stay in the timeline's header, where W-40a put them.

## Not in this part

- **Turn, mirror and the reset readout, snapshot and compare:** W-41b. They need a small change in how the card paints, checked first.
- **The Map tab, and the Hand and Zoom tools:** W-41c.
- **The region box and the one-layer onion skin:** engine changes, each with its own proposal.

## Click-through

| Clicked | What runs | What you see |
|---|---|---|
| ⏮ / ⏭ | one frame back / forward, as before | the frame number changes |
| ▶ | play, as before | the button lights; the fps turns green |
| Draft, not final ▾ → Full resolution | `D`, as before | the chip says "Full" |
| Auto ▾ | Draw on, as before | — |
| checkerboard / alpha | the same two commands as before | the button lights |
| grid / field guide | nothing is sent | lines over the picture |
| 47%, fit ▾ → Fit, Fit ≤100%, 100%, in, out, slider | as before | the picture's size changes |
| slate / strip | nothing is sent | the slate or strip shows or hides |
| a strip cell | go to that frame | the picture and the playhead move |

## Keyboard reach

- **Every new button is a button Tab stops at.** Each has a name a screen reader says.
- **The two ▾ menus work like the top bar's.** Enter opens them, arrows move, Esc closes.
- **The four looks are in the View menu and in Find any command (Ctrl+Shift+P):**
  - "Grid over the picture"
  - "Field guide and safe areas"
  - "Cut slate above the picture"
  - "Sheet strip under the picture"
- **No new key.**

## Sizes

`scale_1.0.png`, `scale_1.5.png` and `scale_2.0.png` show a 1280 by 800 window at 100%, 150% and 200%, playing. At 200% the row wraps onto a second line, as the top bar does, and every control stays visible.

## Checks

- All 81 of the app's tests pass. The wiring table lists the four new buttons: `gridbtn`, `guidebtn`, `slatebtn`, `stripbtn` (`verification/B-12c_keyboard_table.md`).
- The export tests pass: `t08_export`, `h04_exported_file`, `t07e_roundtrip_export` and `d48_export_threads`. The exported pictures are unchanged.

## Playtest

Open the app as you normally do, on a project with drawings.

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Look above the picture | one row of small icons; the tab says "Viewer" and the composition's name, with its size at the right | |
| 2 | Look just above the picture's top-left corner | the slate: EP · SC · **CUT** (in amber) · sheet seconds + frames. If Ctrl+K has no cut, it shows the composition's name in amber | |
| 3 | Press Space | ▶ becomes a lit ⏸; the grey number turns into a green one near your frame rate | |
| 4 | Press Space again | it stops; the number goes grey | |
| 5 | Click the quality chip (the dot and "Draft…") | a small menu with Full resolution and Refine when idle; pick Full, and the chip says "Full" | |
| 6 | Click the grid button, then the field-guide button | a 12×12 grid; green 12F/10F/8F frames, amber and blue safe areas, a small centre cross | |
| 7 | Click both again | they go away | |
| 8 | Click the size chip ("…%, fit ▾") | Fit, Fit ≤100%, 100%, in, out and the slider | |
| 9 | Look under the picture; click a cell in the strip | the strip shows your selected layer's Sheet column (or the first); clicking a cell goes to that frame | |
| 10 | Click the slate and strip buttons | each hides; close and reopen the app, and they stay as you left them | |

Anything marked ✗, tell me the row number.
