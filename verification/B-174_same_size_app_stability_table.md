# B-174_same_size - the application itself: start-up, dragging the window, stability

Written by `tools/app_stability.ps1` (parts startup only), 2026-09-30 22:35 to 22:49.

Machine: AMD Ryzen 9 9900X 12-Core Processor, NVIDIA GeForce RTX 4070 Ti SUPER (driver 32.0.16.1088), 62 GB, Microsoft Windows 11 Education 10.0.26300. Main display 3840 by 2160 at 150%. Build: release `anime_compositor_app.exe` built 2026-09-30 22:29, at commit 18d417a. Other programs running, as measured before each part: start of the run - 7% of the whole processor; busiest: Discord 0.98 cores, cam_helper 0.14 cores, chrome 0.05 cores; before start-up - 7% of the whole processor; busiest: Discord 0.77 cores, cam_helper 0.15 cores, claude 0.04 cores.

No target for start-up time or window responsiveness exists in the requirements (documents 03 and 08, T-06 and P-20 were checked), so every time below is a measurement only, with no pass or fail. The pass/fail lines are things that either happened or did not: a crash, a hang, a blank window, a window in the wrong place, a process left running, and memory growth against T-06's bound.

## Summary

| Check | Result |
|---|---|
| Start-up: every launch came up, settled and closed cleanly | PASS (60 of 60) |
| Start-up to settled picture, median (no file / reference / heavy) | 2,015 ms / 1,973 ms / 1,955 ms (no target exists; measured only) |
| The viewer has room for the picture at the size the window opens at | FAIL (30 by 17 points at 496,132) |
| No crash reports (event log, Windows Error Reporting) | PASS (none) |

## 1. Start-up

10 launches of each case for the four stages, in alternating rounds (each round starts the three cases in a different order), and 10 more of each with the debugging port for the first picture. Every time is from the moment the program was asked to start. Each launch is photographed continuously for its first 15 s; **settled** is the last time the picture changed in that watch (a pause while a project is still loading does not count, and a launch still changing in its last 500 ms would read as never settling). **First** is round 1, the first launch of that case in this run; the median and the worst are over all 10. A true cold start (after a restart of the computer, with nothing in Windows' file cache) was not measured: a script cannot make one without restarting the machine, and the program had already been started minutes before. Memory is the program plus every web view process it started, read once its picture had settled. The first picture is when the page had one to show (read through the debugging port, so those launches are separate); the port answering is itself a few hundred ms after the window appears, so a picture earlier than that would read as that moment.

| Case | Stage | First | Median | Worst |
|---|---|---|---|---|
| No file given (opens the built-in reference shot) | Window exists | 91 ms | 88 ms | 113 ms |
| No file given (opens the built-in reference shot) | Window visible | 105 ms | 113 ms | 148 ms |
| No file given (opens the built-in reference shot) | First photograph that is not blank | 962 ms | 964 ms | 1,023 ms |
| No file given (opens the built-in reference shot) | Picture settled (its last change in the first 15 s) | 2,019 ms | 2,015 ms | 2,110 ms |
| No file given (opens the built-in reference shot) | Debugging port answered (port launches) | 635 ms | 640 ms | 693 ms |
| No file given (opens the built-in reference shot) | First picture in the viewer (port launches) | 1,490 ms | 1,590 ms | 1,660 ms |
| No file given (opens the built-in reference shot) | Memory at settled: working set | 830 MiB | 831 MiB | 834 MiB |
| No file given (opens the built-in reference shot) | Memory at settled: private bytes | 1,350 MiB | 1,349 MiB | 1,350 MiB |
| No file given (opens the built-in reference shot) | Processes (program + web view) | 7 | 7 | 7 |
| No file given (opens the built-in reference shot) | Close (WM_CLOSE to the program ending) | 294 ms | 315 ms | 344 ms |
| The reference shot, opened from its file | Window exists | 81 ms | 91 ms | 122 ms |
| The reference shot, opened from its file | Window visible | 112 ms | 113 ms | 140 ms |
| The reference shot, opened from its file | First photograph that is not blank | 937 ms | 938 ms | 1,016 ms |
| The reference shot, opened from its file | Picture settled (its last change in the first 15 s) | 1,975 ms | 1,973 ms | 2,059 ms |
| The reference shot, opened from its file | Debugging port answered (port launches) | 625 ms | 636 ms | 683 ms |
| The reference shot, opened from its file | First picture in the viewer (port launches) | 1,616 ms | 1,569 ms | 1,629 ms |
| The reference shot, opened from its file | Memory at settled: working set | 832 MiB | 832 MiB | 832 MiB |
| The reference shot, opened from its file | Memory at settled: private bytes | 1,348 MiB | 1,348 MiB | 1,351 MiB |
| The reference shot, opened from its file | Processes (program + web view) | 7 | 7 | 7 |
| The reference shot, opened from its file | Close (WM_CLOSE to the program ending) | 312 ms | 307 ms | 342 ms |
| Heavy: the reference shot with 11 card effects | Window exists | 66 ms | 84 ms | 109 ms |
| Heavy: the reference shot with 11 card effects | Window visible | 83 ms | 97 ms | 134 ms |
| Heavy: the reference shot with 11 card effects | First photograph that is not blank | 913 ms | 924 ms | 1,086 ms |
| Heavy: the reference shot with 11 card effects | Picture settled (its last change in the first 15 s) | 1,943 ms | 1,955 ms | 2,182 ms |
| Heavy: the reference shot with 11 card effects | Debugging port answered (port launches) | 588 ms | 631 ms | 649 ms |
| Heavy: the reference shot with 11 card effects | First picture in the viewer (port launches) | 1,771 ms | 1,597 ms | 1,771 ms |
| Heavy: the reference shot with 11 card effects | Memory at settled: working set | 842 MiB | 842 MiB | 844 MiB |
| Heavy: the reference shot with 11 card effects | Memory at settled: private bytes | 1,325 MiB | 1,326 MiB | 1,330 MiB |
| Heavy: the reference shot with 11 card effects | Processes (program + web view) | 7 | 7 | 7 |
| Heavy: the reference shot with 11 card effects | Close (WM_CLOSE to the program ending) | 286 ms | 310 ms | 337 ms |

Launches: 60. Did not come up as measured: 0. Exit code other than 0: 0. Left a process running 5 s after closing: 0.

Every launch began remembering the window as `300 250 1500 960 normal`. The window opened: 1522 by 1016 pixels. The viewer canvas at that size, 1.5 s after the first picture: 30 by 17 points at 496,132.

The photographs are round 1 of each case at its settled moment: `B-174_same_size_startup_none.png`, `B-174_same_size_startup_reference.png`, `B-174_same_size_startup_heavy.png`.

## 6. Crash reports and files left behind

Application event log entries (Application Error, Application Hang, Windows Error Reporting) naming this program or its web view since the run began: 0. Windows Error Reporting folders for them: 0.

Files the runs created, apart from the photographs (searched: the program's settings and local data folders apart from the web view's own profile, verification/, Fixtures/reference_shot/ and the repository's top folder), all deleted: `%LOCALAPPDATA%\dev.anitastic.anime-compositor\decoded drawings\4e5841d2f4d88e956ba33b7d847a7ffbb008868b019a59cc5d77cb1b844d2b54.cel`, `%LOCALAPPDATA%\dev.anitastic.anime-compositor\decoded drawings\81de23817cdaac605b4e41a7a90ddc366bd4a4313fe24334b29055388b791941.cel`, `%LOCALAPPDATA%\dev.anitastic.anime-compositor\decoded drawings\c2ea3e6bc7ba7f93b1a5cb13e5c77ddb706c16a5f9183d9351852208e944ee6a.cel`, `%LOCALAPPDATA%\dev.anitastic.anime-compositor\decoded drawings\d3d1cde3aca816c66d0b779ec84cc0425a408638e7c79123a33d2b553c554481.cel`. The copies of the projects and drawings under %TEMP%, and anything written beside them, were deleted. The recent-projects list: unchanged. The remembered window (`window.txt`): there was none before; the one the runs wrote was deleted, so the next start is a first launch.

