# T-06: the viewer's own memory, watched by hand

Written on 2026-09-16. Never performed.

No photograph: the thing being watched is in Task Manager rather than in the window, and the
capture script photographs the window.

## What this is for

`Markdown/11_Verification_Plan.md` names one thing T-06 asks for that nothing in this build
measures: *the viewer's own memory across ten loops of a window left playing*, which it says
"still needs a harness nobody has written". A person with Task Manager is that harness, and
this sheet is the walk. It is the last unmeasured piece of T-06. Everything else T-06 asks for
has a number against it already, and the list below is here so that this is not walked under the
impression that more is missing than is.

- **Cached playback** is measured on the reference shot in
  `verification/T-06_performance_envelope.md` and on the ten-layer fixture document 08 actually
  declares in `verification/T-06_declared_fixture.md`.
- **p95 seek latency** is measured in both, at four budgets in the first and eight in the second.
- **Repeated-loop memory** is measured in both, ten loops each - but **headless**, in a process
  with no window and no WebView in it. That is the gap this sheet fills.
- **End to end, in a window**, playback is photographed in `verification/B-08_window_shell.md`.

What the headless runs came back with, so that there is something to compare against:

| | End of loop 2 | End of loop 10 | Growth | Peak working set |
|---|---|---|---|---|
| Reference shot, four layers | 1019.5 MiB | 1021.8 MiB | 2.3 MiB | 1180.1 MiB |
| The declared fixture, ten layers | 1023.0 MiB | 1023.2 MiB | 0.2 MiB | 1435.8 MiB |

Both tests fail if the growth from loop 2 to loop 10 exceeds one cel, 33,177,600 bytes, which is
31.6 MiB. Loop 1 is excluded on purpose: it is the only loop that pays to read every drawing off
the disk for the first time.

## Before you start

**Use the release build.** `target/release/anime_compositor_app.exe`. A debug build's numbers say
more about the compiler than about the program - `verification/B-08_window_shell.md` records a
debug build at 6 frames played and 97 dropped, against 42 and 31 in a release build.

**Playback loops on its own.** You do not have to press anything ten times and there is no loop
button to find: `src/preview.rs` takes the elapsed time modulo the length of the work area, so a
window left playing runs the shot round and round until something stops it. The reference shot is
240 frames at 24 fps, exactly ten seconds, so ten loops is a hundred seconds of leaving it alone.

**Expect more than one process.** The shell is Tauri, so the window's page runs in a WebView2
process of its own - usually `msedgewebview2.exe` - alongside `anime_compositor_app.exe`. Task
Manager's **Processes** tab groups the children under the app and gives you one figure for the
lot, which is the figure to read. The **Details** tab lists them separately and does not add them
up. The headless tests measured a process with none of this in it, so the total here will be
larger than the table above and that by itself means nothing.

## What to check by hand

1. **Get the window up and settled.** Start the release build and open `Fixtures/reference_shot`
   as a project. Leave the preview at draft. Wait until frame 0 is on screen and the indicator
   beside the frame number reads **Draft**.
2. **Read the starting figure.** In Task Manager's Processes tab, find the app and write down the
   memory it shows. This is before any playback at all.
3. **Start it and leave it.** Press the space bar. The shot plays and, on reaching the end of the
   work area, carries straight on from the beginning. **Do not touch the window after this**: any
   arrow key, any click on the ruler and any scrub stops playback, because everything that moves
   the playhead by hand goes through the same place and that place stops the clock.
4. **Read it again after the first loop**, about ten seconds in. Write it down. This is the
   figure the comparison is actually against, for the same reason the headless test starts at
   loop 2: the first loop is the one paying to read every drawing off the disk.
5. **Leave it for a hundred seconds.** Ten loops. This is the whole point of the sheet and it is
   the part that cannot be hurried, because what is being looked for is a slow climb rather than
   a jump.
6. **Read it a third time** and write it down. Then **subtract the figure from step 4**. That
   difference is the measurement.
7. **Read the sentence along the bottom of the window.** It says how many frames were played in
   real time and how many were dropped. Note that it describes **only the run you are in**: the
   page restarts its clock every time playback starts, so this is not a total across the ten
   loops. Write down what it says anyway - it is the same sentence
   `verification/B-08_window_shell.md` photographs, and a second reading of it on another day is
   worth having, given that artifact's two retakes disagree by a machine-day.
8. **Stop it.** Press an arrow key or click the ruler. Playback stops.
9. **Do it once more at full resolution**, if you have the patience: switch the preview off draft
   and repeat steps 2 to 6. Full frames cost more to make and are the same cels underneath, so
   the memory should land in the same place while the dropped count gets worse. If memory behaves
   differently at full resolution, that is worth saying.

## How to read what you wrote down

**The number that matters is the difference between step 4 and step 6, not the total.** A window
holding something over a gigabyte is the cel cache doing exactly what it was told: the default
budget is 1 GiB and `verification/T-06_declared_fixture.md` is the measurement that raised it
there. A large steady figure is the feature. 

**What would be a defect** is a figure that climbs loop after loop and does not level off. The
headless runs put the growth at 2.3 MiB and 0.2 MiB across 1,920 renders each, against a bound of
one cel. Something in the window that the headless path does not have - the page, the transport,
the WebView - holding on to every frame it is shown would look like a climb that never settles,
and no table in this project can see it. That is the whole reason this sheet exists.

**A little drift is not a climb.** The WebView is a browser and browsers breathe. What is being
looked for is a trend across ten loops, which is why step 5 is a hundred seconds rather than
twenty.

## What checks it by machine

- `verification/T-06_performance_envelope.md` and `verification/T-06_declared_fixture.md`: ten
  loops each, memory measured against the operating system's working set and **asserted** rather
  than reported, with the bound argued in `tests/t06_envelope.rs` rather than picked. Both are
  headless.
- `verification/B-08b_cache_table.md`: that ten loops of the same frames end holding exactly what
  one loop held, in the same number of cels, and that the nine loops after the first decode
  nothing at all. That is the cache's own accounting rather than the process's appetite.
- **Not** checked by a test: everything above. There is no harness that runs a window for a
  hundred seconds and watches what the operating system thinks of it, which is what
  `Markdown/11_Verification_Plan.md` says and why this is a sheet rather than a table.

## What this does not settle

**Two of T-06's targets are missed, and this sheet does not touch either.** They are missed on
the ten-layer fixture document 08 declares, they are measured and written down, and what to do
about them is a decision rather than a measurement:

- **Warm playback at 24 fps.** On the declared fixture this build renders 264.17 ms a frame at
  draft against the 41.7 ms a 24 fps clock allows, and 0 of ten loops came in under the deadline.
  On the four-layer reference shot it sits *on* the deadline and flips either side of it between
  runs. `verification/T-06_declared_fixture.md` says plainly that this is a shot which is more
  work rather than a build that got worse, and that whether a 24 fps target for a ten-layer shot
  is one this project keeps "is a register entry somebody has to open, and opening it is the
  owner's".
- **p95 cached seek at or below 100 ms.** Met on the reference shot at every budget, worst 85.37
  ms. Not met on the declared fixture: 328.48 ms at the viewer's default, and 267.23 ms even at a
  6 GiB probe that holds every drawing in the shot. That is **D-40**, and what is left in it is
  whether the budget becomes a setting with a stated cost.

Neither is this sheet's to answer and neither is an agent's. `Markdown/14_Decisions_Risks.md` is
where a register entry goes, and `Markdown/00` is why an agent does not open one on its own
reading of the evidence.

**And one record is behind the evidence.** `Markdown/11_Verification_Plan.md` still marks T-06
PARTLY RUN and its entry names neither `verification/T-06_performance_envelope.md` nor
`verification/T-06_declared_fixture.md`, which are the two artifacts that answer most of what it
says is owed. Correcting that is a write-up once the verdict above exists, because what it should
say depends on what the verdict is.
