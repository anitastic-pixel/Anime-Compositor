# D-290 / B-175: a frame rate of 23.976 can be typed

Found by P-26 (the After Effects tutorial playtest): Video Copilot's Shockwave, Advanced Electric and Colorful Glitch tutorials each start with a 23.976 fps composition. Before this change the New composition window refused it ("needs whole numbers"), so every one of those tutorials had to be followed at 24 instead.

## What changed

- `src/time.rs` `FrameRate::parse` reads a typed rate. A decimal within 0.005 of a broadcast rate (×1.001 lands on a whole number) becomes the exact n×1000/1001 fraction that document 20 requires. Any other decimal is kept exactly; a fraction can be typed as `24000/1001`.
- `app/src/main.rs`: `composition.create` and `composition.set_settings` use it. Messages show the rate as 23.976.
- `app/ui/index.html`: the rate boxes take decimals, the list offers 23.976, 29.97 and 59.94, and the header shows 23.976.

## Checks (cargo test)

`tests/b175_frame_rate_text.rs`, 3 of 3 pass:

| Typed | Kept as |
|---|---|
| 23.976, 23.98, 23.976024 | 24000/1001 |
| 29.97 | 30000/1001 |
| 59.94 | 60000/1001 |
| 47.952 | 48000/1001 |
| 24, " 30 " | 24/1, 30/1 |
| 24000/1001 | 24000/1001 |
| 48/2 | 24/1 |
| 12.5 | 25/2 |
| "", 0, 0.0, -24, abc, 24/0, nan, inf, 1e9 | refused |

App checks (`cargo test` in `app/`): 88 of 88 pass. Two new lines in `a_composition_can_be_made_and_the_window_moves_into_it`:
- "a composition at 23.976 fps is made, not refused": the window answers "New composition Film, 1920x1080 at 23.976 fps, 48 frames."
- "and its rate is the exact 24000/1001, not a rounded decimal".
The existing "a frame rate of zero is refused" line passes unchanged.

## For the owner to try

1. Composition > New composition. In Frame rate, type 23.976 (or pick it from the list). Make it.
2. The header above the viewer shows 23.976.
3. Composition settings shows 23.976; change the name only and press OK. The rate stays 23.976.
4. Type 0 as a frame rate: a sentence refuses it.

Pixels, exports, saved files and fixtures are unchanged.
