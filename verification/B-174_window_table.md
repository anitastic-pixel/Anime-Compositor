# B-174: where the window opens

Written by `cargo test -p anime_compositor_app window_place`. The window remembers its place in `window.txt` beside the recent list (`%APPDATA%\dev.anitastic.anime-compositor\`): one line, the outer corner and inner size in pixels and `normal` or `maximized`. **The rule: it opens where it was left, unless the middle of its title bar would be on no display, or the file is missing or damaged; then it opens maximized on the main display, as the very first launch does.** Displays here: the main one 3840 by 2088 (less its taskbar) and, where named, a second one 1920 by 1040 to its right. Sizes are width by height, at the window's corner.

**22 of 22 checks pass.**

| Case | Expected | Opens | Result |
|---|---|---|---|
| No file (the very first launch): maximized, Restore goes to three quarters of the main display | 2880 by 1566 at 480,261, maximized | 2880 by 1566 at 480,261, maximized | PASS |
| Written as `300 250 2400 1500 normal`, read back | 2400 by 1500 at 300,250 | 2400 by 1500 at 300,250 | PASS |
| Written as `-11 0 1931 2077 normal`, read back | 1931 by 2077 at -11,0 | 1931 by 2077 at -11,0 | PASS |
| Written as `4000 100 1600 900 maximized`, read back | 1600 by 900 at 4000,100, maximized | 1600 by 900 at 4000,100, maximized | PASS |
| Damaged: `` | 2880 by 1566 at 480,261, maximized | 2880 by 1566 at 480,261, maximized | PASS |
| Damaged: `garbage` | 2880 by 1566 at 480,261, maximized | 2880 by 1566 at 480,261, maximized | PASS |
| Damaged: `300 250 2400` | 2880 by 1566 at 480,261, maximized | 2880 by 1566 at 480,261, maximized | PASS |
| Damaged: `300 250 wide 1500 normal` | 2880 by 1566 at 480,261, maximized | 2880 by 1566 at 480,261, maximized | PASS |
| Damaged: `300 250 2400 1500 sideways` | 2880 by 1566 at 480,261, maximized | 2880 by 1566 at 480,261, maximized | PASS |
| Damaged: `300 250 2400 1500 normal extra` | 2880 by 1566 at 480,261, maximized | 2880 by 1566 at 480,261, maximized | PASS |
| Damaged: `300 250 50 1500 normal` | 2880 by 1566 at 480,261, maximized | 2880 by 1566 at 480,261, maximized | PASS |
| Damaged: `300 250 2400 1500000 normal` | 2880 by 1566 at 480,261, maximized | 2880 by 1566 at 480,261, maximized | PASS |
| Damaged: `300.5 250 2400 1500 normal` | 2880 by 1566 at 480,261, maximized | 2880 by 1566 at 480,261, maximized | PASS |
| Left at 20000,20000, where no display is | 2880 by 1566 at 480,261, maximized | 2880 by 1566 at 480,261, maximized | PASS |
| Left on the second display, still attached | 1600 by 900 at 4000,100 | 1600 by 900 at 4000,100 | PASS |
| Left on the second display, since unplugged | 2880 by 1566 at 480,261, maximized | 2880 by 1566 at 480,261, maximized | PASS |
| Left maximized on the second display, since unplugged | 2880 by 1566 at 480,261, maximized | 2880 by 1566 at 480,261, maximized | PASS |
| Title bar above the top of the screen | 2880 by 1566 at 480,261, maximized | 2880 by 1566 at 480,261, maximized | PASS |
| Title bar just inside the top of the screen | 2400 by 1500 at 300,-10 | 2400 by 1500 at 300,-10 | PASS |
| Bigger than the display it is on: made to fit it | 1920 by 1040 at 3900,0 | 1920 by 1040 at 3900,0 | PASS |
| At the edge of what a number holds | 2880 by 1566 at 480,261, maximized | 2880 by 1566 at 480,261, maximized | PASS |
| No display reported but the main one's work area | 2880 by 1566 at 480,261, maximized | 2880 by 1566 at 480,261, maximized | PASS |
