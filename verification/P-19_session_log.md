# P-19: the session log

Asked for by the owner on 2026-09-25 ("a performance/backlog with timings/etc that is temporarily created when the app is opened, but when closed, it resets") and accepted the same day ("proceed"). **No pixel changes, no file format changes, no fixture changes.**

## What changed

The menu bar has a new button, **Session log…**, which is also in the command search (Ctrl+Shift+P). It opens a panel that sits over the top right of the window. It is not a modal dialog, so the viewer can still be played underneath it.

- **The switch**, "Record where each frame's time goes", is off every time the window opens. While it is off, nothing is recorded and the stopwatch is not running.
- **While it is on**, every frame the viewer draws becomes one row. The row holds:
  - when the frame was drawn, which frame it was, Draft or Full, and whether it was playing or stepped to
  - how long the whole frame took (the same number the frame's own reply gives the page), and how long each stage of it took
  - drawings found in the cache and drawings read from disk, and effect results found in the cache
  - whether reading ahead started after it, whether an export was running, and how many frames the clock dropped just before it
  - its warnings, with D-25's cap already applied
- **The panel** shows how many frames are logged, the median time, the time the slowest 5% took, the frames the clock dropped, and the stage that took the most time in all. Below that, it lists the 20 slowest frames and the three stages each spent longest in. It refreshes once a second while it is open.
- **Clear** empties the log.
- **Save a copy…** opens the Save dialog and writes every row as one Markdown table. The table is headed with the date (UTC), the operating system, the number of processor cores, and the build's version and whether it is a release build. `P-19_session_log_example.md` is a real one, from the test below.
- **The log lives in memory only.** It keeps the last 5,000 frames, about three and a half minutes of playback. After that the oldest rows go, and the panel says how many went. Closing the window empties it.

## One change from the plan

The accepted entry warned that the stopwatch's totals are shared by every thread, so reading ahead and exporting would leak into whichever frame was being drawn at the time. That is fixed rather than just written on the panel. The stopwatch now also collects on the thread that draws the frame, so a row is that frame's own work only. The table below checks this while reading ahead runs beside every frame. The panel's wording says what is still true: reading ahead and exporting share the processor with the frame, so each row says whether either was running.

## How to judge it

- `P-19_session_log_table.md`, 10 checks, all passing. Produced by `tests/p19_session_log.rs`, the core's half. With the switch off, 48 frames of the reference shot record nothing and the stopwatch stays off. With it on, the same 48 frames give 48 rows. In every row the stages add up to no more than the frame's time. Every frame is byte-identical with the log on and off. The 5,000-row cap holds, the saved copy has its header and one line per row, and Clear works.
- `P-19_session_log_window_table.md`, 20 checks, all passing. The window's half. The frames go through the same function every frame on screen comes from, and each row's time matches the time that frame's reply gave the page. The panel's wording is pinned there too.
- `P-19_session_log_playtest.md`, for you, at the window.

## What having it on costs

Measured, not assumed: `P-19_session_log_cost_table.md`, written by `tests/p19_session_log.rs` (`cargo test --release --test p19_session_log -- --ignored`).

- Machine: AMD Ryzen 9 9900X, 12 cores and 24 threads, Windows 11 Education 10.0.26200
- Build: rustc 1.89.0, release profile, all threads
- Workload: the reference shot, 240 frames a pass, the viewer's cache emptied before each pass, reading ahead on. Passes alternate off and on, three rounds each, at Draft and at Full.

| Quality | Median, off (3 rounds) | Median, on (3 rounds) |
|---|---|---|
| Draft | 13.65, 6.59, 6.47 ms | 7.18, 6.16, 5.97 ms |
| Full | 22.43, 23.19, 23.80 ms | 22.67, 23.57, 23.56 ms |

The first Draft pass is slow either way because it is the first to read the drawings off the disk. After that, the difference between on and off is smaller than the difference between two rounds of the same setting, and it goes both ways. On this machine, the cost of the log is too small to see. That is a measurement on one machine, not a promise about others.

## Limits

- **A log from ordinary use is a diagnostic, not a benchmark.** Other programs, the screen and the disk all show up in it. A claim about speed still goes through `verification/` on the D-01 machine.
- **What the page does after the frame arrives is not in it**: receiving the bytes, drawing them, waiting for the screen. The row stops where the window's own work stops, as P-15's number does.
- **The saved copy's date is UTC**, not local time. The build has no time zone library and this was not worth adding one for.
- **A frame the window could not draw is not logged.** Its sentence goes on screen as before.
- **The table for the saved copy is wide.** Each row lists every stage that took measurable time, in one cell.
