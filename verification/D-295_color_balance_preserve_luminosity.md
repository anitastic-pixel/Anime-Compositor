# D-295 / B-180: Color Balance keeps each pixel's brightness

Found by P-26, tutorial 5 (Film Riot lightsaber): shadows pushed +100 blue turned the black backing solid mid-blue, so Screen flooded the whole frame blue. After Effects' Color Balance has a Preserve Luminosity box, ticked when the effect is added, which keeps black black.

## What changed

- Color Balance has a new switch, **Preserve Luminosity**, Off or On.
- On: each pixel's colour is pushed as before, then moved back to the pixel's own brightness and, if that pushes a channel past black or white, pulled toward grey until it fits. Black and white stay black and white.
- A new Color Balance starts **On**, as After Effects starts it. Files saved before this read **Off** and draw exactly as before; the file only gains the word when it is On.
- The graphics card draws it the same way as the processor.

## Checks (cargo test)

`tests/b180_color_balance_luminosity.rs`, 3 of 3 pass. Three 4x4 solids, each worked out by hand (display values, brightness = 0.2126 red + 0.7152 green + 0.0722 blue):

| Solid | Push | Off (as before) | On |
|---|---|---|---|
| black | shadows blue +100 | 0, 0, 0.5 | 0, 0, 0 (stays black) |
| grey 0.5 | midtones red +100 | 1, 0.5, 0.5 | 0.8937, 0.3937, 0.3937 (same brightness, redder) |
| white | highlights blue -100 | 1, 1, 0.5 | 1, 1, 1 (stays white) |

- The processor matches each number to within 0.00001; the card matches to within one of 255 levels.
- A file without the switch saves without it; On is kept through save and load; a wrong word ("maybe") is reported in a sentence rather than guessed at.

`tests/b73_color_balance.rs` (the D-130 checks) passes unchanged with the switch Off.

## For the owner to try

1. Put a black solid in a composition and add Color Balance. Preserve Luminosity shows **On**.
2. Push Shadows Blue to +100. The solid stays black.
3. Switch Preserve Luminosity to **Off**. The solid turns dark blue, the old behaviour.

## Pictures from the window (test copy, 2026-10-04)

A 1280x720 black solid with Color Balance, Shadows Blue +100:

- `D-295 pictures/1_on_stays_black.png`: a new Color Balance shows Preserve Luminosity **On**; the frame stays black. Pass.
- `D-295 pictures/2_off_turns_blue.png`: switched **Off**, the same frame turns dark blue, as before. Pass.

Fixtures are unchanged.
