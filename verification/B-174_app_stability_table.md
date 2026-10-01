# B-174 - the application itself: start-up, dragging the window, stability

Written by `tools/app_stability.ps1` (parts startup, remember, window only), 2026-09-30 21:24 to 21:31.

Machine: AMD Ryzen 9 9900X 12-Core Processor, NVIDIA GeForce RTX 4070 Ti SUPER (driver 32.0.16.1088), 62 GB, Microsoft Windows 11 Education 10.0.26300. Main display 3840 by 2160 at 150%. Build: release `anime_compositor_app.exe` built 2026-09-30 16:55, at commit 357cc2e. Other programs running, as measured before each part: start of the run - 6% of the whole processor; busiest: Discord 0.90 cores, cam_helper 0.12 cores, claude 0.09 cores; before start-up - 9% of the whole processor; busiest: Discord 0.82 cores, cam_helper 0.16 cores, claude 0.09 cores; before the remembered-window part - 7% of the whole processor; busiest: Discord 0.82 cores, cam_helper 0.16 cores, chrome 0.09 cores; before the window parts - 6% of the whole processor; busiest: Discord 0.83 cores, cam_helper 0.15 cores, claude 0.04 cores.

No target for start-up time or window responsiveness exists in the requirements (documents 03 and 08, T-06 and P-20 were checked), so every time below is a measurement only, with no pass or fail. The pass/fail lines are things that either happened or did not: a crash, a hang, a blank window, a window in the wrong place, a process left running, and memory growth against T-06's bound.

## Summary

| Check | Result |
|---|---|
| Start-up: every launch came up, settled and closed cleanly | PASS (18 of 18) |
| Start-up to settled picture, median (no file / reference / heavy) | 2,187 ms / 1,955 ms / 2,010 ms (no target exists; measured only) |
| The viewer has room for the picture at the size the window opens at | FAIL (30 by 17 points at 496,132) |
| Remembered window: First launch (no remembered window) | FAIL (1522 by 1016 pixels at 266,266; expected maximized) |
| Remembered window: Closed at 2400 by 1500 pixels at 300,250, started again | FAIL (1522 by 1016 pixels at 494,494; expected 2400 by 1500 pixels at 300,250) |
| Remembered window: Closed maximized, started again | FAIL (1522 by 1016 pixels at 114,114; expected maximized) |
| Remembered window: ... then Restore pressed | FAIL (1522 by 1016 pixels at 114,114; expected back to 2400 by 1500 pixels at 300,250) |
| Remembered window: Remembered at 20000,20000 (no display there) | PASS (1522 by 1016 pixels at 228,228) |
| Remembered window: Remembered-window file damaged | FAIL (1522 by 1016 pixels at 342,342; expected maximized, as a first launch) |
| Remembered window: every launch closed cleanly | PASS (6 of 6) |
| Window parts: every drag ran (none skipped) | PASS (6 of 6) |
| Never "Not responding" while dragged or resized | PASS |
| Longest unanswered while dragged or resized | 74 ms (no target exists; measured only) |
| No blank or white photographs while dragged or resized | PASS |
| Window ended where it was dragged / at its size | PASS |
| Layout the same after dragging and resizing | FAIL |
| Playback kept going while the window was dragged | PASS (frames behind 1 s after release: 0, 0, 0) |
| Maximize/minimize: every restore repainted the same picture | FAIL |
| No crash reports (event log, Windows Error Reporting) | PASS (none) |

## 1. Start-up

3 launches of each case for the four stages, in alternating rounds (each round starts the three cases in a different order), and 3 more of each with the debugging port for the first picture. Every time is from the moment the program was asked to start. Each launch is photographed continuously for its first 15 s; **settled** is the last time the picture changed in that watch (a pause while a project is still loading does not count, and a launch still changing in its last 500 ms would read as never settling). **First** is round 1, the first launch of that case in this run; the median and the worst are over all 3. A true cold start (after a restart of the computer, with nothing in Windows' file cache) was not measured: a script cannot make one without restarting the machine, and the program had already been started minutes before. Memory is the program plus every web view process it started, read once its picture had settled. The first picture is when the page had one to show (read through the debugging port, so those launches are separate); the port answering is itself a few hundred ms after the window appears, so a picture earlier than that would read as that moment.

| Case | Stage | First | Median | Worst |
|---|---|---|---|---|
| No file given (opens the built-in reference shot) | Window exists | 63 ms | 69 ms | 74 ms |
| No file given (opens the built-in reference shot) | Window visible | 94 ms | 90 ms | 94 ms |
| No file given (opens the built-in reference shot) | First photograph that is not blank | 872 ms | 947 ms | 982 ms |
| No file given (opens the built-in reference shot) | Picture settled (its last change in the first 15 s) | 2,187 ms | 2,187 ms | 2,270 ms |
| No file given (opens the built-in reference shot) | Debugging port answered (port launches) | 666 ms | 630 ms | 666 ms |
| No file given (opens the built-in reference shot) | First picture in the viewer (port launches) | 1,619 ms | 1,595 ms | 1,619 ms |
| No file given (opens the built-in reference shot) | Memory at settled: working set | 835 MiB | 831 MiB | 835 MiB |
| No file given (opens the built-in reference shot) | Memory at settled: private bytes | 1,348 MiB | 1,347 MiB | 1,348 MiB |
| No file given (opens the built-in reference shot) | Processes (program + web view) | 7 | 7 | 7 |
| No file given (opens the built-in reference shot) | Close (WM_CLOSE to the program ending) | 343 ms | 340 ms | 343 ms |
| The reference shot, opened from its file | Window exists | 80 ms | 80 ms | 95 ms |
| The reference shot, opened from its file | Window visible | 114 ms | 114 ms | 115 ms |
| The reference shot, opened from its file | First photograph that is not blank | 933 ms | 933 ms | 952 ms |
| The reference shot, opened from its file | Picture settled (its last change in the first 15 s) | 1,955 ms | 1,955 ms | 1,967 ms |
| The reference shot, opened from its file | Debugging port answered (port launches) | 633 ms | 633 ms | 650 ms |
| The reference shot, opened from its file | First picture in the viewer (port launches) | 1,549 ms | 1,549 ms | 1,607 ms |
| The reference shot, opened from its file | Memory at settled: working set | 830 MiB | 830 MiB | 831 MiB |
| The reference shot, opened from its file | Memory at settled: private bytes | 1,347 MiB | 1,347 MiB | 1,347 MiB |
| The reference shot, opened from its file | Processes (program + web view) | 7 | 7 | 7 |
| The reference shot, opened from its file | Close (WM_CLOSE to the program ending) | 344 ms | 318 ms | 344 ms |
| Heavy: the reference shot with 11 card effects | Window exists | 95 ms | 95 ms | 109 ms |
| Heavy: the reference shot with 11 card effects | Window visible | 120 ms | 114 ms | 120 ms |
| Heavy: the reference shot with 11 card effects | First photograph that is not blank | 938 ms | 907 ms | 938 ms |
| Heavy: the reference shot with 11 card effects | Picture settled (its last change in the first 15 s) | 2,010 ms | 2,010 ms | 2,038 ms |
| Heavy: the reference shot with 11 card effects | Debugging port answered (port launches) | 616 ms | 661 ms | 669 ms |
| Heavy: the reference shot with 11 card effects | First picture in the viewer (port launches) | 1,579 ms | 1,579 ms | 1,592 ms |
| Heavy: the reference shot with 11 card effects | Memory at settled: working set | 839 MiB | 841 MiB | 842 MiB |
| Heavy: the reference shot with 11 card effects | Memory at settled: private bytes | 1,319 MiB | 1,319 MiB | 1,329 MiB |
| Heavy: the reference shot with 11 card effects | Processes (program + web view) | 7 | 7 | 7 |
| Heavy: the reference shot with 11 card effects | Close (WM_CLOSE to the program ending) | 294 ms | 294 ms | 301 ms |

Launches: 18. Did not come up as measured: 0. Exit code other than 0: 0. Left a process running 5 s after closing: 0.

Every launch began with no remembered window, as a first launch does. The window opened: 1522 by 1016 pixels. The viewer canvas at that size, 1.5 s after the first picture: 30 by 17 points at 496,132.

The photographs are round 1 of each case at its settled moment: `B-174_startup_none.png`, `B-174_startup_reference.png`, `B-174_startup_heavy.png`.

## 1b. Remembering the window

No file given, no debugging port. Each row starts the program, waits for its picture to settle and reads where Windows put the window; the steps run in this order, each closing the window (WM_CLOSE) before the next start. Placing the window is SetWindowPos, as dragging it ends; maximize and Restore are sent as the title-bar buttons send them. Where the program remembers its window: `%APPDATA%\dev.anitastic.anime-compositor\window.txt`, written here by hand for the last two rows.

| Step | Expected | Window | Result | Photograph |
|---|---|---|---|---|
| First launch (no remembered window) | maximized | 1522 by 1016 pixels at 266,266 | FAIL | `B-174_remember_first_launch.png` |
| Closed at 2400 by 1500 pixels at 300,250, started again | 2400 by 1500 pixels at 300,250 | 1522 by 1016 pixels at 494,494 | FAIL | `B-174_remember_size.png` |
| Closed maximized, started again | maximized | 1522 by 1016 pixels at 114,114 | FAIL | `B-174_remember_maximized.png` |
| ... then Restore pressed | back to 2400 by 1500 pixels at 300,250 | 1522 by 1016 pixels at 114,114 | FAIL | `B-174_remember_restored.png` |
| Remembered at 20000,20000 (no display there) | on screen | 1522 by 1016 pixels at 228,228 | PASS | `B-174_remember_off_screen.png` |
| Remembered-window file damaged | maximized, as a first launch | 1522 by 1016 pixels at 342,342 | FAIL | `B-174_remember_damaged.png` |

Closing these windows: exit code 0, 0 left; exit code 0, 0 left; exit code 0, 0 left; exit code 0, 0 left; exit code 0, 0 left; exit code 0, 0 left.

## 2-4. Dragging and resizing the window

These launches use the debugging port (to read the frame number the page shows and to press Play). The window is first placed at 300,250 and made 2400 by 1500 pixels (1600 by 1000 points at 150%), so the viewer has room to show the picture. The mouse is real: SendInput presses the title bar (or the bottom-right corner) at a point Windows itself calls that, moves in small steps, and lets go. Throughout, every 50 ms: the time the window took to answer an empty message (**longest unanswered**), whether Windows called it hung (the test behind 'Not responding'), and the program's processor use (100% = one core). A photograph of the window every 100 ms is checked for blank (one or two colours, or nine tenths white) (playback is followed by the frame number instead: a photograph of a moving window can be a pixel out, so comparing them would mislead). The title-bar path is an arc of 3 s in 300 steps ending 200 right and 120 down; the corner path is 40 steps of 75 ms, out to 300 by 200 bigger, back, in to 300 by 200 smaller and back.

| What | Longest unanswered | Hung reports | CPU mean / max | Blank photographs (window / viewer) | Ended where expected | Layout after = before | Picture after = before | Photograph after |
|---|---|---|---|---|---|---|---|---|
| Title bar, idle | 21 ms | 0 | 10% / 89% | 0 / 0 of 55 | yes | yes | NO (20 pixels within 2378x10 at 0,1434) | `B-174_drag_idle.png` |
| Title bar, idle, second time | 20 ms | 0 | 26% / 1,563% | 0 / 0 of 55 | yes | yes | NO (17 pixels within 2378x4 at 0,1440) | `B-174_drag_idle_2.png` |
| Title bar, playing (No file given (opens the built-in reference shot)) | 34 ms | 0 | 39% / 186% | 0 / 0 of 55 | yes | yes | n/a (playing) | `B-174_drag_playing_none.png` |
| Bottom-right corner, idle | 57 ms | 0 | 28% / 657% | 0 / 0 of 55 | yes | NO | NO (398825 pixels within 2369x1302 at 9,142) | `B-174_resize_idle.png` |
| Bottom-right corner, playing | 74 ms | 0 | 55% / 322% | 0 / 0 of 55 | yes | yes | n/a (playing) | `B-174_resize_playing.png` |
| Title bar, playing (Heavy: the reference shot with 11 card effects) | 23 ms | 0 | 51% / 320% | 0 / 0 of 55 | yes | yes | n/a (playing) | `B-174_drag_playing_heavy.png` |

Layout of Bottom-right corner, idle, in points (the window's inner size, then the viewer canvas and the panel that holds it), before: `{"w":1586,"h":963,"dpr":1.5,"view":{"x":516.3333740234375,"y":106.5,"width":576,"height":324,"top":106.5,"right":1092.3333740234375,"bottom":430.5,"left":516.3333740234375},"stage":{"x":323,"y":94.5,"width":978,"height":330.3333435058594,"top":94.5,"right":1301,"bottom":424.8333435058594,"left":323}}`; after: `{"w":1586,"h":963,"dpr":1.5,"view":{"x":540,"y":106.66667175292969,"width":544,"height":306,"top":106.66667175292969,"right":1084,"bottom":412.6666717529297,"left":540},"stage":{"x":323,"y":94.5,"width":978,"height":330.3333435058594,"top":94.5,"right":1301,"bottom":424.8333435058594,"left":323}}`. Photograph before: `B-174_resize_idle_before.png`.

Playback during the drags (frame numbers read from the page every 50 ms; **behind** is the frame the page showed 1 s after letting go against the frame 24 frames a second should have reached from the last frame read before pressing; positive means behind):

| What | Different frames shown during the drag | Longest the frame number stood still during it | Frames behind 1 s after release | Photographs during |
|---|---|---|---|---|
| Title bar, playing (No file given (opens the built-in reference shot)) | 55 in 3,339 ms | 201 ms | 0 | `B-174_drag_playing_none_during_1.png`, `B-174_drag_playing_none_during_2.png`, `B-174_drag_playing_none_during_3.png` |
| Bottom-right corner, playing | 45 in 3,334 ms | 100 ms | 0 | `B-174_resize_playing_during_1.png`, `B-174_resize_playing_during_2.png`, `B-174_resize_playing_during_3.png` |
| Title bar, playing (Heavy: the reference shot with 11 card effects) | 59 in 3,328 ms | 151 ms | 0 | `B-174_drag_playing_heavy_during_1.png`, `B-174_drag_playing_heavy_during_2.png`, `B-174_drag_playing_heavy_during_3.png` |

Maximize and minimize, five of each, sent as the title-bar buttons send them (WM_SYSCOMMAND), on the reference shot, idle, at the placed size. **There** is the time to the maximized picture settling (or to Windows reporting the window minimized); **back** is the time from Restore to the picture settling; **same** means the restored picture is identical, pixel for pixel, to the one before.

| Cycle | There | Reached | Back | Restored picture same | Blank after | Frame shown (before) | Window after |
|---|---|---|---|---|---|---|---|
| maximize 1 | 401 ms | yes | 307 ms | NO (343509 pixels within 2369x1284 at 9,160) | no | 37 (37) | 900,611 2400x1500 |
| minimize 1 | 18 ms | yes | 243 ms | NO (343506 pixels within 816x563 at 810,160) | no | 37 (37) | 900,611 2400x1500 |
| maximize 2 | 289 ms | yes | 319 ms | NO (343509 pixels within 2369x1284 at 9,160) | no | 37 (37) | 900,611 2400x1500 |
| minimize 2 | 22 ms | yes | 197 ms | NO (343506 pixels within 816x563 at 810,160) | no | 37 (37) | 900,611 2400x1500 |
| maximize 3 | 431 ms | yes | 311 ms | NO (343509 pixels within 2369x1284 at 9,160) | no | 37 (37) | 900,611 2400x1500 |
| minimize 3 | 23 ms | yes | 315 ms | NO (343506 pixels within 816x563 at 810,160) | no | 37 (37) | 900,611 2400x1500 |
| maximize 4 | 389 ms | yes | 318 ms | NO (343509 pixels within 2369x1284 at 9,160) | no | 37 (37) | 900,611 2400x1500 |
| minimize 4 | 5 ms | yes | 227 ms | NO (343506 pixels within 816x563 at 810,160) | no | 37 (37) | 900,611 2400x1500 |
| maximize 5 | 305 ms | yes | 305 ms | NO (343509 pixels within 2369x1284 at 9,160) | no | 37 (37) | 900,611 2400x1500 |
| minimize 5 | 10 ms | yes | 235 ms | NO (343506 pixels within 816x563 at 810,160) | no | 37 (37) | 900,611 2400x1500 |

Closing these windows afterwards: none exit code 0, 0 processes left; heavy exit code 0, 0 processes left.

## 6. Crash reports and files left behind

Application event log entries (Application Error, Application Hang, Windows Error Reporting) naming this program or its web view since the run began: 0. Windows Error Reporting folders for them: 0.

Files the runs created, apart from the photographs (searched: the program's settings and local data folders apart from the web view's own profile, verification/, Fixtures/reference_shot/ and the repository's top folder), all deleted: 56 files (443 MiB) in `%LOCALAPPDATA%\dev.anitastic.anime-compositor\decoded drawings`. The copies of the projects and drawings under %TEMP%, and anything written beside them, were deleted. The recent-projects list: unchanged. The remembered window (`window.txt`): there was none before, and none is left.

