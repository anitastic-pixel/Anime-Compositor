# D-332 / B-213: keys on every keyable setting survive a save

From P-26's tutorial 1 (the Video Copilot shockwave), rebuilt from scratch on 2026-10-06 after D-328 to D-330.

## What was wrong

The app saved the rebuilt shockwave, and the saved file would not open:

> This project file cannot be opened, because part of it does not match the project format. At /compositions/0/layers/0/effects/0/parameters/offset: expected an array.

Saving was right. Opening was wrong. The file reader only read keys on an effect setting whose name was on a list of its own, and that list had fallen behind the settings a key can be put on. Twenty-three settings were missing, among them three that the tutorial keys:

- Fractal Noise's **Offset Turbulence**
- Bulge's **Vertical Radius**
- Bulge's **Taper Radius**

Any project with a key on one of those 23 could be saved but not opened again.

## What changed

- The reader now reads keys on **any** setting that is saved with them. There is no list to fall behind.
- Keys on a setting the effect does not have are dropped, as before (P-17).
- Older files open, draw and save exactly as before.

## Checks

**FX-KEYALL-001 to 004:** `tests/b213_keyed_settings.rs` opens `Fixtures/projects/keyed_settings_project.json`. That file is the tutorial's own Wave layer, as the app saved it, plus a Bulge with keyed radii. The results are in `verification/D-332_keyed_settings_table.md`: **4 of 4 pass**.

| Check | Result |
| --- | --- |
| The file opens | yes |
| Offset Turbulence: 0, 0 at frame 0 to 0, 410 at frame 34, eased | yes |
| Vertical Radius 190 to 120, Taper Radius 0 to 240, frames 0 to 12 | yes |
| Saved and opened again: the same keys, and the same text | yes |

**FX-KEYALL-005:** the app's `keyed_settings` test takes every effect the app can add (99). On each one it puts two keys on every setting a key can go on (457 settings in all), using the same commands the window uses. It then saves, opens the file again and checks every key came back. **It passes.** Run against the old reader, the same test fails with the same "expected an array" message the shockwave gave. So this test would have caught the bug, and it covers effects added later too.

## Picture (`verification/D-332 pictures/shockwave_reopened.png`)

![The rebuilt shockwave, saved and opened again](D-332%20pictures/shockwave_reopened.png)

This is tutorial 1's shockwave, built in our app step by step, saved, opened again and drawn over black:

- **Top left: SW.** The single ring.
- **Top middle: SW 2.** The ring nested, scaled and time-remapped.
- **Top right and bottom row: SW 3.** Six copies, Glow and colour, at frames 16, 20, 24 and 28.

Before this fix the file behind these frames could not be opened at all.

## How to check it yourself

1. Open `Downloads\shockwave_tutorial1.json` in the app.
2. It should open with no error.
3. Select the Wave layer in composition SW and open its Fractal Noise. Offset Turbulence should show its two keys, at frame 0 and frame 34.
4. Save the project under a new name, close it, and open it again. The keys should still be there.

**Awaiting the owner's playtest.**
