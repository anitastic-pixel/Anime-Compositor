# B-174 - the application itself: start-up, dragging the window, stability

Written by `tools/app_stability.ps1` (every part), 2026-09-30 22:49 to 23:33.

Machine: AMD Ryzen 9 9900X 12-Core Processor, NVIDIA GeForce RTX 4070 Ti SUPER (driver 32.0.16.1088), 62 GB, Microsoft Windows 11 Education 10.0.26300. Main display 3840 by 2160 at 150%. Build: release `anime_compositor_app.exe` built 2026-09-30 22:29, at commit 18d417a. Other programs running, as measured before each part: start of the run - 8% of the whole processor; busiest: Discord 0.77 cores, cam_helper 0.16 cores, explorer 0.06 cores; before start-up - 10% of the whole processor; busiest: Discord 0.80 cores, explorer 0.13 cores, claude 0.11 cores; before the remembered-window part - 9% of the whole processor; busiest: Discord 0.77 cores, cam_helper 0.18 cores, chrome 0.09 cores; before the window parts - 26% of the whole processor; busiest: chrome 2.38 cores, Discord 0.85 cores, chrome 0.83 cores; before the launch-and-close cycles - 12% of the whole processor; busiest: Discord 0.81 cores, chrome 0.46 cores, chrome 0.38 cores; before the soak - 7% of the whole processor; busiest: Discord 0.83 cores, cam_helper 0.15 cores, NZXT CAM 0.09 cores.

No target for start-up time or window responsiveness exists in the requirements (documents 03 and 08, T-06 and P-20 were checked), so every time below is a measurement only, with no pass or fail. The pass/fail lines are things that either happened or did not: a crash, a hang, a blank window, a window in the wrong place, a process left running, and memory growth against T-06's bound.

## Summary

| Check | Result |
|---|---|
| Start-up: every launch came up, settled and closed cleanly | PASS (60 of 60) |
| Start-up to settled picture, median (no file / reference / heavy) | 2,017 ms / 2,109 ms / 2,048 ms (no target exists; measured only) |
| The viewer has room for the picture at the size the window opens at | PASS (1,524 by 857 points at 537,76) |
| Remembered window: First launch (no remembered window) | PASS (maximized) |
| Remembered window: Closed at 2400 by 1500 pixels at 300,250, started again | PASS (2400 by 1500 pixels at 300,250) |
| Remembered window: Closed maximized, started again | PASS (maximized) |
| Remembered window: ... then Restore pressed | PASS (2400 by 1500 pixels at 300,250) |
| Remembered window: Remembered at 20000,20000 (no display there) | PASS (maximized) |
| Remembered window: Remembered-window file damaged | PASS (maximized) |
| Remembered window: every launch closed cleanly | PASS (6 of 6) |
| Window parts: every drag ran (none skipped) | PASS (6 of 6) |
| Never "Not responding" while dragged or resized | PASS |
| Longest unanswered while dragged or resized | 48 ms (no target exists; measured only) |
| No blank or white photographs while dragged or resized | PASS |
| Window ended where it was dragged / at its size | PASS |
| Layout the same after dragging and resizing | PASS |
| Playback kept going while the window was dragged | PASS (frames behind 1 s after release: 0, 1, -1) |
| Maximize/minimize: every restore repainted the same picture | PASS (10 of 10) |
| Launch and close 50 times: all clean | PASS (50 of 50 exit 0, nothing left running) |
| Soak: growth over the first ten loops within T-06's bound (31.6 MiB) | PASS (-66 MiB private) |
| Soak: memory trend over the second half | 1.66 MiB a minute private, 1.35 working set (T-06 has a bound for ten loops only; measured only) |
| Soak 20 min: still playing at the end, never hung, every drag ran | PASS |
| No crash reports (event log, Windows Error Reporting) | PASS (none) |

## 1. Start-up

10 launches of each case for the four stages, in alternating rounds (each round starts the three cases in a different order), and 10 more of each with the debugging port for the first picture. Every time is from the moment the program was asked to start. Each launch is photographed continuously for its first 15 s; **settled** is the last time the picture changed in that watch (a pause while a project is still loading does not count, and a launch still changing in its last 500 ms would read as never settling). **First** is round 1, the first launch of that case in this run; the median and the worst are over all 10. A true cold start (after a restart of the computer, with nothing in Windows' file cache) was not measured: a script cannot make one without restarting the machine, and the program had already been started minutes before. Memory is the program plus every web view process it started, read once its picture had settled. The first picture is when the page had one to show (read through the debugging port, so those launches are separate); the port answering is itself a few hundred ms after the window appears, so a picture earlier than that would read as that moment.

| Case | Stage | First | Median | Worst |
|---|---|---|---|---|
| No file given (opens the built-in reference shot) | Window exists | 60 ms | 69 ms | 109 ms |
| No file given (opens the built-in reference shot) | Window visible | 79 ms | 93 ms | 137 ms |
| No file given (opens the built-in reference shot) | First photograph that is not blank | 966 ms | 967 ms | 1,066 ms |
| No file given (opens the built-in reference shot) | Picture settled (its last change in the first 15 s) | 2,010 ms | 2,017 ms | 2,175 ms |
| No file given (opens the built-in reference shot) | Debugging port answered (port launches) | 688 ms | 638 ms | 688 ms |
| No file given (opens the built-in reference shot) | First picture in the viewer (port launches) | 1,723 ms | 1,602 ms | 1,723 ms |
| No file given (opens the built-in reference shot) | Memory at settled: working set | 830 MiB | 830 MiB | 832 MiB |
| No file given (opens the built-in reference shot) | Memory at settled: private bytes | 1,467 MiB | 1,456 MiB | 1,467 MiB |
| No file given (opens the built-in reference shot) | Processes (program + web view) | 7 | 7 | 7 |
| No file given (opens the built-in reference shot) | Close (WM_CLOSE to the program ending) | 336 ms | 332 ms | 360 ms |
| The reference shot, opened from its file | Window exists | 84 ms | 71 ms | 118 ms |
| The reference shot, opened from its file | Window visible | 116 ms | 89 ms | 149 ms |
| The reference shot, opened from its file | First photograph that is not blank | 1,011 ms | 974 ms | 1,011 ms |
| The reference shot, opened from its file | Picture settled (its last change in the first 15 s) | 2,145 ms | 2,109 ms | 2,230 ms |
| The reference shot, opened from its file | Debugging port answered (port launches) | 631 ms | 637 ms | 673 ms |
| The reference shot, opened from its file | First picture in the viewer (port launches) | 1,631 ms | 1,587 ms | 1,741 ms |
| The reference shot, opened from its file | Memory at settled: working set | 831 MiB | 831 MiB | 835 MiB |
| The reference shot, opened from its file | Memory at settled: private bytes | 1,466 MiB | 1,466 MiB | 1,473 MiB |
| The reference shot, opened from its file | Processes (program + web view) | 7 | 7 | 7 |
| The reference shot, opened from its file | Close (WM_CLOSE to the program ending) | 298 ms | 311 ms | 351 ms |
| Heavy: the reference shot with 11 card effects | Window exists | 62 ms | 86 ms | 105 ms |
| Heavy: the reference shot with 11 card effects | Window visible | 95 ms | 101 ms | 141 ms |
| Heavy: the reference shot with 11 card effects | First photograph that is not blank | 908 ms | 966 ms | 1,063 ms |
| Heavy: the reference shot with 11 card effects | Picture settled (its last change in the first 15 s) | 1,993 ms | 2,048 ms | 2,137 ms |
| Heavy: the reference shot with 11 card effects | Debugging port answered (port launches) | 629 ms | 636 ms | 694 ms |
| Heavy: the reference shot with 11 card effects | First picture in the viewer (port launches) | 1,608 ms | 1,610 ms | 1,734 ms |
| Heavy: the reference shot with 11 card effects | Memory at settled: working set | 843 MiB | 841 MiB | 843 MiB |
| Heavy: the reference shot with 11 card effects | Memory at settled: private bytes | 1,447 MiB | 1,431 MiB | 1,447 MiB |
| Heavy: the reference shot with 11 card effects | Processes (program + web view) | 7 | 7 | 7 |
| Heavy: the reference shot with 11 card effects | Close (WM_CLOSE to the program ending) | 326 ms | 333 ms | 343 ms |

Launches: 60. Did not come up as measured: 0. Exit code other than 0: 0. Left a process running 5 s after closing: 0.

Every launch began with no remembered window, as a first launch does. The window opened: maximized. The viewer canvas at that size, 1.5 s after the first picture: 1,524 by 857 points at 537,76.

The photographs are round 1 of each case at its settled moment: `B-174_startup_none.png`, `B-174_startup_reference.png`, `B-174_startup_heavy.png`.

## 1b. Remembering the window

No file given, no debugging port. Each row starts the program, waits for its picture to settle and reads where Windows put the window; the steps run in this order, each closing the window (WM_CLOSE) before the next start. Placing the window is SetWindowPos, as dragging it ends; maximize and Restore are sent as the title-bar buttons send them. Where the program remembers its window: `%APPDATA%\dev.anitastic.anime-compositor\window.txt`, written here by hand for the last two rows.

| Step | Expected | Window | Result | Photograph |
|---|---|---|---|---|
| First launch (no remembered window) | maximized | maximized | PASS | `B-174_remember_first_launch.png` |
| Closed at 2400 by 1500 pixels at 300,250, started again | 2400 by 1500 pixels at 300,250 | 2400 by 1500 pixels at 300,250 | PASS | `B-174_remember_size.png` |
| Closed maximized, started again | maximized | maximized | PASS | `B-174_remember_maximized.png` |
| ... then Restore pressed | back to 2400 by 1500 pixels at 300,250 | 2400 by 1500 pixels at 300,250 | PASS | `B-174_remember_restored.png` |
| Remembered at 20000,20000 (no display there) | on screen | maximized | PASS | `B-174_remember_off_screen.png` |
| Remembered-window file damaged | maximized, as a first launch | maximized | PASS | `B-174_remember_damaged.png` |

Closing these windows: exit code 0, 0 left; exit code 0, 0 left; exit code 0, 0 left; exit code 0, 0 left; exit code 0, 0 left; exit code 0, 0 left.

## 2-4. Dragging and resizing the window

These launches use the debugging port (to read the frame number the page shows and to press Play). The window is first placed at 300,250 and made 2400 by 1500 pixels (1600 by 1000 points at 150%), so the viewer has room to show the picture. The mouse is real: SendInput presses the title bar (or the bottom-right corner) at a point Windows itself calls that, moves in small steps, and lets go. Throughout, every 50 ms: the time the window took to answer an empty message (**longest unanswered**), whether Windows called it hung (the test behind 'Not responding'), and the program's processor use (100% = one core). A photograph of the window every 100 ms is checked for blank (one or two colours, or nine tenths white) (playback is followed by the frame number instead: a photograph of a moving window can be a pixel out, so comparing them would mislead). The title-bar path is an arc of 3 s in 300 steps ending 200 right and 120 down; the corner path is 40 steps of 75 ms, out to 300 by 200 bigger, back, in to 300 by 200 smaller and back.

| What | Longest unanswered | Hung reports | CPU mean / max | Blank photographs (window / viewer) | Ended where expected | Layout after = before | Picture after = before | Photograph after |
|---|---|---|---|---|---|---|---|---|
| Title bar, idle | 18 ms | 0 | 5% / 74% | 0 / 0 of 55 | yes | yes | yes | `B-174_drag_idle.png` |
| Title bar, idle, second time | 18 ms | 0 | 8% / 79% | 0 / 0 of 55 | yes | yes | yes | `B-174_drag_idle_2.png` |
| Title bar, playing (No file given (opens the built-in reference shot)) | 18 ms | 0 | 52% / 220% | 0 / 0 of 55 | yes | yes | n/a (playing) | `B-174_drag_playing_none.png` |
| Bottom-right corner, idle | 48 ms | 0 | 23% / 289% | 0 / 0 of 55 | yes | yes | yes | `B-174_resize_idle.png` |
| Bottom-right corner, playing | 41 ms | 0 | 51% / 455% | 0 / 0 of 55 | yes | yes | n/a (playing) | `B-174_resize_playing.png` |
| Title bar, playing (Heavy: the reference shot with 11 card effects) | 20 ms | 0 | 67% / 447% | 0 / 0 of 55 | yes | yes | n/a (playing) | `B-174_drag_playing_heavy.png` |

Playback during the drags (frame numbers read from the page every 50 ms; **behind** is the frame the page showed 1 s after letting go against the frame 24 frames a second should have reached from the last frame read before pressing; positive means behind):

| What | Different frames shown during the drag | Longest the frame number stood still during it | Frames behind 1 s after release | Photographs during |
|---|---|---|---|---|
| Title bar, playing (No file given (opens the built-in reference shot)) | 59 in 3,341 ms | 199 ms | 0 | `B-174_drag_playing_none_during_1.png`, `B-174_drag_playing_none_during_2.png`, `B-174_drag_playing_none_during_3.png` |
| Bottom-right corner, playing | 49 in 3,344 ms | 151 ms | 1 | `B-174_resize_playing_during_1.png`, `B-174_resize_playing_during_2.png`, `B-174_resize_playing_during_3.png` |
| Title bar, playing (Heavy: the reference shot with 11 card effects) | 57 in 3,346 ms | 201 ms | -1 | `B-174_drag_playing_heavy_during_1.png`, `B-174_drag_playing_heavy_during_2.png`, `B-174_drag_playing_heavy_during_3.png` |

Maximize and minimize, five of each, sent as the title-bar buttons send them (WM_SYSCOMMAND), on the reference shot, idle, at the placed size. **There** is the time to the maximized picture settling (or to Windows reporting the window minimized); **back** is the time from Restore to the picture settling; **same** means the restored picture is identical, pixel for pixel, to the one before, apart from the window's four 16 by 16 pixel corners, which Windows 11 rounds and smooths itself (every comparison of two photographs in this table leaves them out).

| Cycle | There | Reached | Back | Restored picture same | Blank after | Frame shown (before) | Window after |
|---|---|---|---|---|---|---|---|
| maximize 1 | 388 ms | yes | 332 ms | yes | no | 38 (38) | 900,611 2400x1500 |
| minimize 1 | 18 ms | yes | 264 ms | yes | no | 38 (38) | 900,611 2400x1500 |
| maximize 2 | 310 ms | yes | 335 ms | yes | no | 38 (38) | 900,611 2400x1500 |
| minimize 2 | 39 ms | yes | 255 ms | yes | no | 38 (38) | 900,611 2400x1500 |
| maximize 3 | 287 ms | yes | 338 ms | yes | no | 38 (38) | 900,611 2400x1500 |
| minimize 3 | 5 ms | yes | 261 ms | yes | no | 38 (38) | 900,611 2400x1500 |
| maximize 4 | 285 ms | yes | 300 ms | yes | no | 38 (38) | 900,611 2400x1500 |
| minimize 4 | 38 ms | yes | 249 ms | yes | no | 38 (38) | 900,611 2400x1500 |
| maximize 5 | 353 ms | yes | 343 ms | yes | no | 38 (38) | 900,611 2400x1500 |
| minimize 5 | 23 ms | yes | 238 ms | yes | no | 38 (38) | 900,611 2400x1500 |

Closing these windows afterwards: none exit code 0, 0 processes left; heavy exit code 0, 0 processes left.

## 5a. 50 launches and closes

No file given (the built-in reference shot), no debugging port. Each launch waits for the picture to settle, then sends WM_CLOSE (what the close button and the rebuild watcher send), waits for the program to end, and 5 s after asking looks for any of its processes still running. Crash reports were searched for over the whole run (section 6).

| Launches | Came up and settled | Clean exits (code 0) | Other exit codes | Did not end within 15 s | Processes left 5 s after close | Close time, median / worst | Start to the first 500 ms without a change, median / worst |
|---|---|---|---|---|---|---|---|
| 50 | 50 | 50 | none | 0 | 0 | 230 ms / 514 ms | 997 ms / 1,124 ms |

## 5b. 20-minute soak

The reference shot (no file given, debugging port) playing in a loop at the placed size for 20 minutes, a title-bar drag of 3 s every 60 s (150 pixels right, then back), memory (program plus web view) every 10 s. One loop of the shot is 10 s, so the soak is 120 loops. T-06 measured the core alone, without a window, over ten loops of the same shot: 2.3 MiB of growth against a bound of 31.6 MiB.

| | Working set | Private bytes |
|---|---|---|
| At the start (playing) | 2,677 MiB | 4,914 MiB |
| After ten loops (100 s) | 2,684 MiB | 4,848 MiB |
| Growth over the first ten loops | 8 MiB | -66 MiB |
| At the end (1190 s) | 2,717 MiB | 4,882 MiB |
| Growth, start to end | 41 MiB | -32 MiB |
| Peak | 2,724 MiB | 4,914 MiB |
| Trend over the second half (straight-line fit) | 1.35 MiB a minute | 1.66 MiB a minute |

Processes in the tree: 7 (from 7 to 7). Hung reports at the samples: 0. Frames read 200 ms apart at the end: 48, 54, 59, 64, 68. The page's played-and-dropped sentence after stopping: "Played 61797 frames in real time and dropped 91 to keep the timing true. Step through the frames to see every drawing, or switch the preview to draft resolution.". Closing it: exit code 0, 0 processes left.

Drags during the soak (when, longest unanswered): 60 s 14 ms; 120 s 16 ms; 180 s 15 ms; 240 s 16 ms; 300 s 16 ms; 360 s 16 ms; 420 s 16 ms; 480 s 16 ms; 540 s 18 ms; 600 s 16 ms; 660 s 17 ms; 720 s 17 ms; 780 s 16 ms; 840 s 20 ms; 900 s 14 ms; 960 s 15 ms; 1020 s 17 ms; 1080 s 16 ms; 1140 s 19 ms.

Every sample (s, working set MiB, private MiB, frame): 0 2,677 4,914 48; 10 2,681 4,854 51; 20 2,689 4,853 50; 30 2,690 4,855 48; 40 2,692 4,857 51; 50 2,690 4,855 51; 60 2,679 4,854 50; 70 2,682 4,856 53; 80 2,684 4,856 51; 90 2,686 4,850 51; 100 2,684 4,848 49; 110 2,686 4,850 51; 120 2,686 4,848 49; 130 2,688 4,851 51; 140 2,687 4,849 50; 150 2,690 4,852 49; 160 2,692 4,854 52; 170 2,693 4,855 50; 180 2,692 4,853 52; 190 2,692 4,857 49; 200 2,693 4,854 49; 210 2,693 4,854 52; 220 2,692 4,853 49; 230 2,694 4,853 53; 240 2,692 4,853 52; 250 2,694 4,854 51; 260 2,693 4,853 49; 270 2,694 4,855 51; 280 2,694 4,854 52; 290 2,697 4,858 51; 300 2,696 4,857 49; 310 2,698 4,863 51; 320 2,699 4,860 48; 330 2,701 4,862 52; 340 2,698 4,860 50; 350 2,699 4,860 49; 360 2,701 4,863 49; 370 2,702 4,863 53; 380 2,701 4,864 52; 390 2,702 4,864 52; 400 2,699 4,862 50; 410 2,700 4,864 50; 420 2,700 4,864 52; 430 2,700 4,871 50; 440 2,703 4,865 50; 450 2,704 4,868 49; 460 2,704 4,867 48; 470 2,704 4,867 52; 480 2,704 4,867 49; 490 2,705 4,875 52; 500 2,705 4,868 52; 510 2,706 4,868 50; 520 2,705 4,866 53; 530 2,705 4,868 50; 540 2,705 4,868 50; 550 2,706 4,873 50; 560 2,707 4,869 52; 570 2,707 4,870 50; 580 2,707 4,869 48; 590 2,706 4,870 52; 600 2,706 4,870 51; 610 2,708 4,874 52; 620 2,709 4,871 51; 630 2,709 4,873 50; 640 2,708 4,872 48; 650 2,709 4,873 52; 660 2,710 4,872 51; 670 2,711 4,873 48; 680 2,711 4,874 49; 690 2,711 4,874 51; 700 2,712 4,874 49; 710 2,711 4,873 52; 720 2,711 4,874 51; 730 2,712 4,875 49; 740 2,715 4,878 54; 750 2,716 4,878 51; 760 2,716 4,878 50; 770 2,716 4,879 49; 780 2,716 4,880 53; 790 2,716 4,884 50; 800 2,716 4,880 48; 810 2,718 4,882 51; 820 2,718 4,882 51; 830 2,718 4,882 50; 840 2,718 4,881 50; 850 2,718 4,888 51; 860 2,718 4,882 51; 870 2,718 4,882 51; 880 2,723 4,886 51; 890 2,710 4,876 50; 900 2,711 4,877 47; 910 2,711 4,876 51; 920 2,711 4,878 50; 930 2,713 4,878 49; 940 2,713 4,877 49; 950 2,714 4,878 49; 960 2,715 4,879 51; 970 2,717 4,881 51; 980 2,716 4,881 49; 990 2,716 4,881 51; 1000 2,716 4,882 50; 1010 2,717 4,884 48; 1020 2,717 4,881 53; 1030 2,717 4,882 51; 1040 2,719 4,883 50; 1050 2,721 4,886 49; 1060 2,720 4,885 53; 1070 2,722 4,886 50; 1080 2,721 4,885 50; 1090 2,722 4,895 52; 1100 2,722 4,886 51; 1110 2,722 4,887 49; 1120 2,722 4,887 49; 1130 2,723 4,889 49; 1140 2,723 4,888 49; 1150 2,723 4,896 50; 1160 2,723 4,887 49; 1170 2,724 4,889 52; 1180 2,724 4,887 50; 1190 2,717 4,882 50.

## 6. Crash reports and files left behind

Application event log entries (Application Error, Application Hang, Windows Error Reporting) naming this program or its web view since the run began: 0. Windows Error Reporting folders for them: 0.

Files the runs created, apart from the photographs (searched: the program's settings and local data folders apart from the web view's own profile, verification/, Fixtures/reference_shot/ and the repository's top folder), all deleted: 56 files (443 MiB) in `%LOCALAPPDATA%\dev.anitastic.anime-compositor\decoded drawings`. The copies of the projects and drawings under %TEMP%, and anything written beside them, were deleted. The recent-projects list: unchanged. The remembered window (`window.txt`): there was none before; the one the runs wrote was deleted, so the next start is a first launch.

