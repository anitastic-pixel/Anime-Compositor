# B-832: the viewer's resolution list (D-952)

Written by `b832_resolution_list` in `app/src/tests/w12r.rs`.

The first rows run the page's own list and its sizing rule under Node; the window rows ask the reference shot's frame 30 at Third and Full (Third's mean difference from Full averaged over 3x3 squares was 0.96 levels of 255); the rest pin the wiring. The private-app check is `verification/B-832_private_app_table.md`.

**11 of 11 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| the list, as After Effects names it | Full, Half, Third, Quarter, Auto | Full, Half, Third, Quarter, Auto | pass |
| Auto is Full drawn at the size shown (Full pixels unticked) | {"quality":"full","fullPixels":false} | {"quality":"full","fullPixels":false} | pass |
| a size from the list is that size at any zoom (Full pixels ticked) | {"quality":"third","fullPixels":true} | {"quality":"third","fullPixels":true} | pass |
| the line lit for Full unticked, Full ticked, Half unticked, Third, Quarter | auto, full, half, third, draft | auto, full, half, third, draft | pass |
| Auto at 33% draws Third, at 40% Half; Third chosen at 25% draws Quarter, at 100% Third; a drag on Third is In motion | /frame/7?q=third /frame/7?q=half /frame/7?q=draft /frame/7?q=third /frame/7?q=motion | /frame/7?q=third /frame/7?q=half /frame/7?q=draft /frame/7?q=third /frame/7?q=motion | pass |
| frame 30 at Third: its size, its label and not final | 640 360 Third true, 921600 bytes | 640 360 Third true, 921600 bytes | pass |
| Third against Full averaged over each 3x3 square: mean difference under 3 levels of 255 | true | true | pass |
| and Full asked after it is still Full, the same as export | Full false | Full false | pass |
| the list sits above the D key's Full/Draft line in the quality menu | true | true | pass |
| the lit line follows each frame that comes back | true | true | pass |
| and the Full pixels tick | true | true | pass |
