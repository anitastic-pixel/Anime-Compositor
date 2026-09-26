# P-19: the session log, checked

Written by `tests/p19_session_log.rs` under `cargo test --test p19_session_log`. Each frame is drawn the way the window draws one: the viewer's cache, the next frame's drawings read ahead beside it, then encoded. Timings are not asserted and not printed here; they differ from run to run and machine to machine (document 12). The cost of the log is measured in `verification/P-19_session_log_cost_table.md`.

| # | Check | Result |
|---|---|---|
| 1 | A new log is off. Playing the reference shot for 48 frames with it off records nothing, and the stopwatch stays off. | 0 rows; stopwatch off |
| 2 | Switched on, playing the same 48 frames records one row for each. | 48 rows |
| 3 | In every row, the stages add up to no more than the frame's own time, while the next frame's drawings are being read on another thread at the same moment. | 48 of 48 |
| 4 | Every frame drawn with the log on is byte-identical to the same frame with it off. | 48 of 48 identical |
| 5 | A frame stepped to at Full quality is logged as Full, not playing, not read ahead. | Full, playing false, read ahead false |
| 6 | Switched off again, the 49 rows stay to be read or saved, 4 more frames add none, and the stopwatch stops. | 49 rows; stopwatch off |
| 7 | A saved copy is headed with the date, operating system, core count, version and build, and has one table line per row after its header and rule. | 49 lines for 49 rows |
| 8 | Clear empties the log. | 0 rows |
| 9 | 5,003 rows pushed keep the newest 5,000 and count the 3 let go. | 5000 kept, 3 let go, oldest kept is row 3 |
| 10 | On those rows, taking 3 ms to 5,002 ms, the median is 2,502 ms, the slowest 5% take 4,752 ms or more, and the slowest listed first is the 5,002 ms one. | 2502 / 4752 / 5002 |
