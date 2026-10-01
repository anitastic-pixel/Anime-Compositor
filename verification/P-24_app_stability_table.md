# P-24 - the application itself: start-up, dragging the window, stability

Written by `tools/app_stability.ps1` (every part), 2026-09-30 17:42 to 18:25.

Machine: AMD Ryzen 9 9900X 12-Core Processor, NVIDIA GeForce RTX 4070 Ti SUPER (driver 32.0.16.1088), 62 GB, Microsoft Windows 11 Education 10.0.26300. Main display 3840 by 2160 at 150%. Build: release `anime_compositor_app.exe` built 2026-09-30 16:55, at commit ec106ce. Other programs running, as measured before each part: start of the run - 11% of the whole processor; busiest: Discord 1.01 cores, chrome 0.34 cores, chrome 0.16 cores; before start-up - 17% of the whole processor; busiest: Discord 0.93 cores, claude 0.18 cores, cam_helper 0.16 cores; before the window parts - 11% of the whole processor; busiest: Discord 0.80 cores, chrome 0.67 cores, chrome 0.16 cores; before the launch-and-close cycles - 10% of the whole processor; busiest: Discord 0.90 cores, chrome 0.19 cores, cam_helper 0.16 cores; before the soak - 6% of the whole processor; busiest: Discord 0.79 cores, cam_helper 0.07 cores, discord_clips 0.05 cores.

No target for start-up time or window responsiveness exists in the requirements (documents 03 and 08, T-06 and P-20 were checked), so every time below is a measurement only, with no pass or fail. The pass/fail lines are things that either happened or did not: a crash, a hang, a blank window, a window in the wrong place, a process left running, and memory growth against T-06's bound.

## Summary

| Check | Result |
|---|---|
| Start-up: every launch came up, settled and closed cleanly | PASS (60 of 60) |
| Start-up to settled picture, median (no file / reference / heavy) | 1,911 ms / 1,926 ms / 1,990 ms (no target exists; measured only) |
| The viewer has room for the picture at the size the window opens at | FAIL (30 by 17 points at 496,132) |
| Window parts: every drag ran (none skipped) | PASS (6 of 6) |
| Never "Not responding" while dragged or resized | PASS |
| Longest unanswered while dragged or resized | 55 ms (no target exists; measured only) |
| No blank or white photographs while dragged or resized | PASS |
| Window ended where it was dragged / at its size | PASS |
| Layout the same after dragging and resizing | FAIL |
| Playback kept going while the window was dragged | PASS (frames behind 1 s after release: -1, -1, 0) |
| Maximize/minimize: every restore repainted the same picture | FAIL |
| Launch and close 50 times: all clean | PASS (50 of 50 exit 0, nothing left running) |
| Soak: growth over the first ten loops within T-06's bound (31.6 MiB) | PASS (-62 MiB private) |
| Soak: memory trend over the second half | 0.93 MiB a minute private, 0.46 working set (T-06 has a bound for ten loops only; measured only) |
| Soak 20 min: still playing at the end, never hung, every drag ran | PASS |
| No crash reports (event log, Windows Error Reporting) | PASS (none) |

## 1. Start-up

10 launches of each case for the four stages, in alternating rounds (each round starts the three cases in a different order), and 10 more of each with the debugging port for the first picture. Every time is from the moment the program was asked to start. Each launch is photographed continuously for its first 15 s; **settled** is the last time the picture changed in that watch (a pause while a project is still loading does not count, and a launch still changing in its last 500 ms would read as never settling). **First** is round 1, the first launch of that case in this run; the median and the worst are over all 10. A true cold start (after a restart of the computer, with nothing in Windows' file cache) was not measured: a script cannot make one without restarting the machine, and the program had already been started minutes before. Memory is the program plus every web view process it started, read once its picture had settled. The first picture is when the page had one to show (read through the debugging port, so those launches are separate); the port answering is itself a few hundred ms after the window appears, so a picture earlier than that would read as that moment.

| Case | Stage | First | Median | Worst |
|---|---|---|---|---|
| No file given (opens the built-in reference shot) | Window exists | 79 ms | 78 ms | 107 ms |
| No file given (opens the built-in reference shot) | Window visible | 98 ms | 100 ms | 121 ms |
| No file given (opens the built-in reference shot) | First photograph that is not blank | 942 ms | 912 ms | 1,010 ms |
| No file given (opens the built-in reference shot) | Picture settled (its last change in the first 15 s) | 1,919 ms | 1,911 ms | 2,037 ms |
| No file given (opens the built-in reference shot) | Debugging port answered (port launches) | 684 ms | 641 ms | 684 ms |
| No file given (opens the built-in reference shot) | First picture in the viewer (port launches) | 1,592 ms | 1,571 ms | 1,778 ms |
| No file given (opens the built-in reference shot) | Memory at settled: working set | 830 MiB | 830 MiB | 833 MiB |
| No file given (opens the built-in reference shot) | Memory at settled: private bytes | 1,346 MiB | 1,339 MiB | 1,349 MiB |
| No file given (opens the built-in reference shot) | Processes (program + web view) | 7 | 7 | 7 |
| No file given (opens the built-in reference shot) | Close (WM_CLOSE to the program ending) | 306 ms | 307 ms | 330 ms |
| The reference shot, opened from its file | Window exists | 83 ms | 82 ms | 126 ms |
| The reference shot, opened from its file | Window visible | 102 ms | 105 ms | 143 ms |
| The reference shot, opened from its file | First photograph that is not blank | 928 ms | 936 ms | 985 ms |
| The reference shot, opened from its file | Picture settled (its last change in the first 15 s) | 2,001 ms | 1,926 ms | 2,007 ms |
| The reference shot, opened from its file | Debugging port answered (port launches) | 630 ms | 620 ms | 663 ms |
| The reference shot, opened from its file | First picture in the viewer (port launches) | 1,564 ms | 1,592 ms | 1,684 ms |
| The reference shot, opened from its file | Memory at settled: working set | 829 MiB | 830 MiB | 837 MiB |
| The reference shot, opened from its file | Memory at settled: private bytes | 1,329 MiB | 1,347 MiB | 1,351 MiB |
| The reference shot, opened from its file | Processes (program + web view) | 7 | 7 | 7 |
| The reference shot, opened from its file | Close (WM_CLOSE to the program ending) | 336 ms | 317 ms | 343 ms |
| Heavy: the reference shot with 11 card effects | Window exists | 100 ms | 70 ms | 100 ms |
| Heavy: the reference shot with 11 card effects | Window visible | 119 ms | 87 ms | 130 ms |
| Heavy: the reference shot with 11 card effects | First photograph that is not blank | 946 ms | 946 ms | 1,061 ms |
| Heavy: the reference shot with 11 card effects | Picture settled (its last change in the first 15 s) | 1,993 ms | 1,990 ms | 2,276 ms |
| Heavy: the reference shot with 11 card effects | Debugging port answered (port launches) | 626 ms | 629 ms | 702 ms |
| Heavy: the reference shot with 11 card effects | First picture in the viewer (port launches) | 1,585 ms | 1,604 ms | 1,758 ms |
| Heavy: the reference shot with 11 card effects | Memory at settled: working set | 843 MiB | 842 MiB | 848 MiB |
| Heavy: the reference shot with 11 card effects | Memory at settled: private bytes | 1,331 MiB | 1,324 MiB | 1,331 MiB |
| Heavy: the reference shot with 11 card effects | Processes (program + web view) | 7 | 7 | 7 |
| Heavy: the reference shot with 11 card effects | Close (WM_CLOSE to the program ending) | 326 ms | 316 ms | 330 ms |

Launches: 60. Did not come up as measured: 0. Exit code other than 0: 0. Left a process running 5 s after closing: 0.

The viewer canvas at the size the window opens at (1000 by 640 points), 1.5 s after the first picture: 30 by 17 points at 496,132.

The photographs are round 1 of each case at its settled moment: `P-24_startup_none.png`, `P-24_startup_reference.png`, `P-24_startup_heavy.png`.

## 2-4. Dragging and resizing the window

These launches use the debugging port (to read the frame number the page shows and to press Play). The window is first placed at 300,250 and made 2400 by 1500 pixels (1600 by 1000 points at 150%), so the viewer has room to show the picture. The mouse is real: SendInput presses the title bar (or the bottom-right corner) at a point Windows itself calls that, moves in small steps, and lets go. Throughout, every 50 ms: the time the window took to answer an empty message (**longest unanswered**), whether Windows called it hung (the test behind 'Not responding'), and the program's processor use (100% = one core). A photograph of the window every 100 ms is checked for blank (one or two colours, or nine tenths white) (playback is followed by the frame number instead: a photograph of a moving window can be a pixel out, so comparing them would mislead). The title-bar path is an arc of 3 s in 300 steps ending 200 right and 120 down; the corner path is 40 steps of 75 ms, out to 300 by 200 bigger, back, in to 300 by 200 smaller and back.

| What | Longest unanswered | Hung reports | CPU mean / max | Blank photographs (window / viewer) | Ended where expected | Layout after = before | Picture after = before | Photograph after |
|---|---|---|---|---|---|---|---|---|
| Title bar, idle | 17 ms | 0 | 15% / 101% | 0 / 0 of 55 | yes | yes | NO (20 pixels within 2378x10 at 0,1434) | `P-24_drag_idle.png` |
| Title bar, idle, second time | 17 ms | 0 | 9% / 76% | 0 / 0 of 55 | yes | yes | NO (17 pixels within 2378x4 at 0,1440) | `P-24_drag_idle_2.png` |
| Title bar, playing (No file given (opens the built-in reference shot)) | 17 ms | 0 | 49% / 236% | 0 / 0 of 55 | yes | yes | n/a (playing) | `P-24_drag_playing_none.png` |
| Bottom-right corner, idle | 55 ms | 0 | 22% / 298% | 0 / 0 of 55 | yes | NO | NO (398825 pixels within 2369x1302 at 9,142) | `P-24_resize_idle.png` |
| Bottom-right corner, playing | 43 ms | 0 | 57% / 310% | 0 / 0 of 55 | yes | yes | n/a (playing) | `P-24_resize_playing.png` |
| Title bar, playing (Heavy: the reference shot with 11 card effects) | 20 ms | 0 | 47% / 232% | 0 / 0 of 55 | yes | yes | n/a (playing) | `P-24_drag_playing_heavy.png` |

Layout of Bottom-right corner, idle, in points (the window's inner size, then the viewer canvas and the panel that holds it), before: `{"w":1586,"h":963,"dpr":1.5,"view":{"x":516.3333740234375,"y":106.5,"width":576,"height":324,"top":106.5,"right":1092.3333740234375,"bottom":430.5,"left":516.3333740234375},"stage":{"x":323,"y":94.5,"width":978,"height":330.3333435058594,"top":94.5,"right":1301,"bottom":424.8333435058594,"left":323}}`; after: `{"w":1586,"h":963,"dpr":1.5,"view":{"x":540,"y":106.66667175292969,"width":544,"height":306,"top":106.66667175292969,"right":1084,"bottom":412.6666717529297,"left":540},"stage":{"x":323,"y":94.5,"width":978,"height":330.3333435058594,"top":94.5,"right":1301,"bottom":424.8333435058594,"left":323}}`. Photograph before: `P-24_resize_idle_before.png`.

Playback during the drags (frame numbers read from the page every 50 ms; **behind** is the frame the page showed 1 s after letting go against the frame 24 frames a second should have reached from the last frame read before pressing; positive means behind):

| What | Different frames shown during the drag | Longest the frame number stood still during it | Frames behind 1 s after release | Photographs during |
|---|---|---|---|---|
| Title bar, playing (No file given (opens the built-in reference shot)) | 61 in 3,327 ms | 201 ms | -1 | `P-24_drag_playing_none_during_1.png`, `P-24_drag_playing_none_during_2.png`, `P-24_drag_playing_none_during_3.png` |
| Bottom-right corner, playing | 42 in 3,323 ms | 101 ms | -1 | `P-24_resize_playing_during_1.png`, `P-24_resize_playing_during_2.png`, `P-24_resize_playing_during_3.png` |
| Title bar, playing (Heavy: the reference shot with 11 card effects) | 54 in 3,336 ms | 208 ms | 0 | `P-24_drag_playing_heavy_during_1.png`, `P-24_drag_playing_heavy_during_2.png`, `P-24_drag_playing_heavy_during_3.png` |

Maximize and minimize, five of each, sent as the title-bar buttons send them (WM_SYSCOMMAND), on the reference shot, idle, at the placed size. **There** is the time to the maximized picture settling (or to Windows reporting the window minimized); **back** is the time from Restore to the picture settling; **same** means the restored picture is identical, pixel for pixel, to the one before.

| Cycle | There | Reached | Back | Restored picture same | Blank after | Frame shown (before) | Window after |
|---|---|---|---|---|---|---|---|
| maximize 1 | 303 ms | yes | 331 ms | NO (343509 pixels within 2369x1284 at 9,160) | no | 37 (37) | 900,611 2400x1500 |
| minimize 1 | 23 ms | yes | 268 ms | NO (343506 pixels within 816x563 at 810,160) | no | 37 (37) | 900,611 2400x1500 |
| maximize 2 | 297 ms | yes | 303 ms | NO (343509 pixels within 2369x1284 at 9,160) | no | 37 (37) | 900,611 2400x1500 |
| minimize 2 | 16 ms | yes | 215 ms | NO (343506 pixels within 816x563 at 810,160) | no | 37 (37) | 900,611 2400x1500 |
| maximize 3 | 313 ms | yes | 321 ms | NO (343509 pixels within 2369x1284 at 9,160) | no | 37 (37) | 900,611 2400x1500 |
| minimize 3 | 19 ms | yes | 270 ms | NO (343506 pixels within 816x563 at 810,160) | no | 37 (37) | 900,611 2400x1500 |
| maximize 4 | 305 ms | yes | 343 ms | NO (343509 pixels within 2369x1284 at 9,160) | no | 37 (37) | 900,611 2400x1500 |
| minimize 4 | 44 ms | yes | 301 ms | NO (343506 pixels within 816x563 at 810,160) | no | 37 (37) | 900,611 2400x1500 |
| maximize 5 | 316 ms | yes | 328 ms | NO (343509 pixels within 2369x1284 at 9,160) | no | 37 (37) | 900,611 2400x1500 |
| minimize 5 | 6 ms | yes | 229 ms | NO (343506 pixels within 816x563 at 810,160) | no | 37 (37) | 900,611 2400x1500 |

Closing these windows afterwards: none exit code 0, 0 processes left; heavy exit code 0, 0 processes left.

## 5a. 50 launches and closes

No file given (the built-in reference shot), no debugging port. Each launch waits for the picture to settle, then sends WM_CLOSE (what the close button and the rebuild watcher send), waits for the program to end, and 5 s after asking looks for any of its processes still running. Crash reports were searched for over the whole run (section 6).

| Launches | Came up and settled | Clean exits (code 0) | Other exit codes | Did not end within 15 s | Processes left 5 s after close | Close time, median / worst | Start to the first 500 ms without a change, median / worst |
|---|---|---|---|---|---|---|---|
| 50 | 50 | 50 | none | 0 | 0 | 228 ms / 328 ms | 977 ms / 2,092 ms |

## 5b. 20-minute soak

The reference shot (no file given, debugging port) playing in a loop at the placed size for 20 minutes, a title-bar drag of 3 s every 60 s (150 pixels right, then back), memory (program plus web view) every 10 s. One loop of the shot is 10 s, so the soak is 120 loops. T-06 measured the core alone, without a window, over ten loops of the same shot: 2.3 MiB of growth against a bound of 31.6 MiB.

| | Working set | Private bytes |
|---|---|---|
| At the start (playing) | 2,675 MiB | 4,915 MiB |
| After ten loops (100 s) | 2,696 MiB | 4,854 MiB |
| Growth over the first ten loops | 21 MiB | -62 MiB |
| At the end (1190 s) | 2,720 MiB | 4,888 MiB |
| Growth, start to end | 45 MiB | -27 MiB |
| Peak | 2,726 MiB | 4,915 MiB |
| Trend over the second half (straight-line fit) | 0.46 MiB a minute | 0.93 MiB a minute |

Processes in the tree: 7 (from 7 to 7). Hung reports at the samples: 0. Frames read 200 ms apart at the end: 51, 56, 60, 65, 70. The page's played-and-dropped sentence after stopping: "Played 61281 frames in real time and dropped 96 to keep the timing true. Step through the frames to see every drawing, or switch the preview to draft resolution.". Closing it: exit code 0, 0 processes left.

Drags during the soak (when, longest unanswered): 60 s 17 ms; 120 s 15 ms; 180 s 16 ms; 240 s 17 ms; 300 s 15 ms; 360 s 19 ms; 420 s 17 ms; 480 s 20 ms; 540 s 19 ms; 600 s 19 ms; 660 s 18 ms; 720 s 17 ms; 780 s 19 ms; 840 s 16 ms; 900 s 17 ms; 960 s 15 ms; 1020 s 16 ms; 1080 s 20 ms; 1140 s 14 ms.

Every sample (s, working set MiB, private MiB, frame): 0 2,675 4,915 48; 10 2,681 4,858 51; 20 2,691 4,859 50; 30 2,692 4,860 53; 40 2,693 4,862 53; 50 2,691 4,861 52; 60 2,690 4,860 51; 70 2,693 4,869 49; 80 2,696 4,862 48; 90 2,696 4,864 51; 100 2,696 4,854 50; 110 2,698 4,855 50; 120 2,697 4,854 51; 130 2,699 4,859 50; 140 2,699 4,854 50; 150 2,701 4,857 48; 160 2,702 4,857 52; 170 2,703 4,858 49; 180 2,703 4,859 53; 190 2,704 4,861 52; 200 2,704 4,858 50; 210 2,704 4,858 49; 220 2,703 4,858 49; 230 2,704 4,860 51; 240 2,703 4,859 50; 250 2,704 4,863 50; 260 2,704 4,858 53; 270 2,705 4,861 49; 280 2,705 4,860 53; 290 2,708 4,862 51; 300 2,708 4,862 51; 310 2,709 4,868 53; 320 2,709 4,863 51; 330 2,712 4,866 49; 340 2,711 4,865 52; 350 2,712 4,866 51; 360 2,703 4,867 50; 370 2,704 4,877 49; 380 2,704 4,867 51; 390 2,705 4,866 51; 400 2,705 4,866 49; 410 2,705 4,869 53; 420 2,706 4,869 52; 430 2,706 4,876 49; 440 2,708 4,871 53; 450 2,710 4,873 50; 460 2,710 4,873 52; 470 2,710 4,873 50; 480 2,710 4,873 50; 490 2,710 4,881 53; 500 2,711 4,874 51; 510 2,712 4,874 50; 520 2,712 4,876 48; 530 2,712 4,874 53; 540 2,713 4,875 51; 550 2,713 4,878 48; 560 2,715 4,877 52; 570 2,715 4,877 49; 580 2,715 4,877 51; 590 2,714 4,876 50; 600 2,714 4,876 49; 610 2,714 4,885 49; 620 2,714 4,876 51; 630 2,715 4,877 50; 640 2,715 4,879 50; 650 2,714 4,879 50; 660 2,717 4,881 50; 670 2,717 4,883 52; 680 2,717 4,879 50; 690 2,716 4,879 50; 700 2,715 4,881 50; 710 2,715 4,880 49; 720 2,715 4,881 48; 730 2,716 4,890 52; 740 2,718 4,883 51; 750 2,719 4,885 52; 760 2,720 4,886 50; 770 2,720 4,886 51; 780 2,720 4,886 49; 790 2,720 4,890 48; 800 2,721 4,888 51; 810 2,721 4,884 48; 820 2,722 4,886 53; 830 2,721 4,886 49; 840 2,722 4,888 53; 850 2,722 4,896 51; 860 2,723 4,889 49; 870 2,722 4,887 49; 880 2,723 4,888 53; 890 2,709 4,876 48; 900 2,710 4,877 53; 910 2,710 4,882 50; 920 2,712 4,879 49; 930 2,712 4,879 49; 940 2,712 4,880 50; 950 2,711 4,878 49; 960 2,713 4,881 53; 970 2,713 4,884 51; 980 2,714 4,882 50; 990 2,714 4,882 49; 1000 2,715 4,883 49; 1010 2,715 4,883 51; 1020 2,716 4,883 50; 1030 2,718 4,894 49; 1040 2,720 4,887 52; 1050 2,720 4,888 51; 1060 2,720 4,888 51; 1070 2,720 4,888 52; 1080 2,720 4,888 49; 1090 2,720 4,891 48; 1100 2,720 4,888 52; 1110 2,721 4,889 51; 1120 2,721 4,889 53; 1130 2,721 4,889 52; 1140 2,721 4,889 49; 1150 2,722 4,892 54; 1160 2,722 4,888 53; 1170 2,722 4,889 52; 1180 2,726 4,894 50; 1190 2,720 4,888 50.

## 6. Crash reports and files left behind

Application event log entries (Application Error, Application Hang, Windows Error Reporting) naming this program or its web view since the run began: 0. Windows Error Reporting folders for them: 0.

Files the runs created, apart from the photographs (searched: the program's settings and local data folders apart from the web view's own profile, verification/, Fixtures/reference_shot/ and the repository's top folder), all deleted: 108 files (854 MiB) in `%LOCALAPPDATA%\dev.anitastic.anime-compositor\decoded drawings`. The copies of the projects and drawings under %TEMP%, and anything written beside them, were deleted. The recent-projects list: unchanged.

## 7. Where the memory goes (by hand, after the run)

Everything above this section was written by the script. This section and the next were written by hand from the photographs, one more short measurement, and the page and program source; nothing in the program was changed.

The soak holds about 2.7 GB of working set and 4.9 GB of private bytes, against about 0.8 and 1.3 GB at start-up. One more launch on the same build (the reference shot, debugging port) split it between the program and its web view:

| Moment | Program: working set / private | Web view, 6 processes: working set / private |
|---|---|---|
| Opening size, idle | 486 / 1,115 MiB | 384 / 278 MiB |
| Placed at 2400 by 1500, idle | 442 / 1,071 MiB | 387 / 317 MiB |
| Playing for 20 s (two loops) | 2,279 / 4,582 MiB | 401 / 277 MiB |
| Stopped for 15 s | 2,279 / 4,582 MiB | 407 / 275 MiB |

The growth is all in the program, it arrives in the first two loops, and it then stays flat for 20 minutes (section 5b). That is the memory setting doing what D-105 and B-154 built it to do: on Automatic the viewer may keep a quarter of the machine's memory, here 16 GB, in drawings and finished frames, so that a loop played again is not drawn again. The program uses less than a third of what that setting allows, and the soak shows no growth past it. **Not a problem**, so no fix is proposed; a smaller footprint is one Preferences change away (the memory setting) for anyone who wants it.

## Problems found

### 1. At the size it opens at, the viewer has no room for the picture

**What was seen.** The window opens at 1000 by 640 points (`app/tauri.conf.json`), every time: it does not remember a larger size from last time. At that size the menu bar wraps to three rows, the panels share what is left, and the composition panel is all controls: the viewer canvas is 30 by 17 points (section 1), too small to see at all. `P-24_startup_none.png`, `P-24_startup_reference.png` and `P-24_startup_heavy.png` show it: no picture anywhere, in all 60 start-ups. The picture appears only once the window is made bigger or maximized. Everything else at start-up is healthy (window in about 0.1 s, interface in about 0.9 s, picture ready in about 1.6 s).

**Why it matters.** It is the first thing seen on every start, and it looks as if the program has failed to show the shot.

**Proposed fix (for the owner to decide).** Remember the window's size, place and maximized state when it closes and reopen it the same way; for the very first start, open it maximized (or at most of the screen) instead of 1000 by 640. Either half alone removes the problem from the second start on. A third option, a smallest height for the viewer so the controls scroll instead, would also guarantee a picture but makes the panels scroll at small sizes. This is a change to `app/` and is not made here.

**How it would be checked.** This script's start-up summary line ("The viewer has room for the picture at the size the window opens at") turns to PASS, and the start-up photographs show the picture.

### 2. The viewer's "fit" zoom depends on how the window got to its size

**What was seen.** At the same window size (2400 by 1500 pixels) the viewer's fit zoom read 30%, 28% or 27% depending on what happened before:

- `P-24_resize_idle_before.png`: 30%, fit, and the picture is a little taller than its panel, so the panel shows a scroll bar on its right and the bottom of the picture is cut off.
- `P-24_resize_idle.png`, after the corner was dragged out, in and back to exactly the same size: 28%, fit, no scroll bar.
- `P-24_before_maximize.png` (28%) against `P-24_restored_from_maximize.png` (27%): after maximize and restore, the picture comes back slightly smaller and higher.

That is why the summary says FAIL for "Layout the same after dragging and resizing" and "every restore repainted the same picture": the panel itself is the same size each time (the layout lines in section 2-4), only the picture inside it is scaled differently. The frame shown and everything else were the same.

**Likely cause (read from the page, not proved by a test).** The page works out "fit" only when the window's own size changes (`app/ui/index.html`, `applyZoom` on the window's resize event), from the panel's size at that moment. The panel can change size without the window changing: for example the "Played 707 frames ..." line appears under the timeline when playback stops, which makes the viewer panel shorter, and the picture is not fitted again. A scroll bar showing or not showing at the moment of fitting changes the room by a few points too.

**Why it matters.** Cosmetic: a point or two of zoom, and at worst the bottom few rows of the picture behind a scroll bar until the next resize. No crash, no wrong pixels in the picture itself.

**Proposed fix (for the owner to decide).** Fit the picture again whenever the viewer panel changes size, not only when the window does (a ResizeObserver on the viewer panel calling the same `applyZoom`). This is a change to `app/` and is not made here.

**How it would be checked.** The two FAIL lines above turn to PASS on a re-run of this script, and the zoom figure under the viewer reads the same before and after a resize or a maximize and restore.

### Looked at and not a problem

- **The idle title-bar drags left 17 and 20 pixels different**, all within the bottom 4 to 10 rows of the photograph and spread across its whole width. Windows 11 rounds the window's bottom corners, so the photograph of the window's inner area shows a few pixels of whatever is behind it at those two corners; after a drag that is a different part of the desktop. The window's own pixels did not change.
- **Frames behind after release of -1** means the picture was one frame ahead of the clock's count when read, which is rounding between the page's clock and the 50 ms reading. Playback never paused during a drag.
- **A true cold start** (straight after restarting the computer) was not measured, because a script cannot make one without restarting the machine. Every start-up here is warm.

