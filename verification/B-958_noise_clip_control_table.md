# B-958: Noise, Clip Result Values in Effect Controls (D-1078)

Written by `b958_noise_clip_control` in `app/src/tests/w12x.rs`.

After Effects' Noise > Clip Result Values (lHQvi94dpvM about 4:20 on, f80 about 4:00 off). The window's own reading of what Effect Controls sends; the drawing itself is `verification/B-958_noise_clip_table.md`.

**6 of 6 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| a window from before D-1078 sends no clip: Clip on | on, drawn | on, drawn | pass |
| Clip Result Values On | on, drawn | on, drawn | pass |
| Clip Result Values Off | off, drawn | off, drawn | pass |
| a word that is not a choice | wrap, refused | wrap, refused | pass |
| a Noise just added: Clip Result Values | on | on | pass |
| Effect Controls shows Clip Result Values, On first (a file without it reads On) | true | true | pass |
