# D-305 / B-190: Shift Channels

Found by P-26, tutorial 3 (Video Copilot, Colorful Glitch). The tutorial uses After Effects' Shift Channels to take a layer's alpha from its brightness, so the dark parts drop out. The app had no such effect.

## What changed

There is a new effect, **Shift Channels**, under Color Correction. It has After Effects' four choices: Take Alpha From, Take Red From, Take Green From and Take Blue From.

Each choice can be set to Alpha, Red, Green, Blue, Luminance, Hue, Lightness, Saturation, Full, Half or Off. Each starts at its own channel, so a newly added Shift Channels changes nothing.

It works on the colour as the eye sees it, the same way Channel Mixer does. The new colour is multiplied by the new alpha, so a fully clear pixel made solid comes out black. The choices take no keys, as in After Effects. The CPU draws this effect, not the graphics card; the preview and the export match.

## Checks (cargo test)

`tests/b190_shift_channels.rs`, 3 of 3 pass. Each was worked out by hand on a single-colour square:

| Check | Expected pixel | Got |
|---|---|---|
| Nothing set, on red | red, unchanged | so |
| Red from green, green from red, on red | green | so |
| Blue from Full, on red | magenta | so |
| Red from Half | red at 0.5 as the eye sees it | so |
| Alpha from Half | red at half covering | so |
| Alpha from green, on red | nothing | so |
| Red from Luminance, on red | 0.2126, the luma's red weight | so |
| Red from Hue, on green | one third | so |
| Red from Lightness, on red | 0.5 | so |
| Green from Saturation, on red / on grey | full / none | so |
| Green from Off | no green | so |
| Alpha from Full where a wipe cleared the drawing | black | so |
| The graphics-card preview, swapped and blue full | cyan, 0 255 255 | so |
| Saving and a wrong word ("purple") | kept / reported, effect left out | so |

The whole core suite and the app suite still pass, with only the two known scratch failures noted in earlier reports. The app's command check includes the new effect.

## Pictures (the test copy, never the owner's app)

The pictures use a white layer with Fractal Noise smoke (size 120, complexity 6, contrast 140, brightness -10) over a dark blue backing.

| Picture | Settings | Look for | Pass? |
|---|---|---|---|
| `D-305 pictures/1_as_added_nothing_changes.png` | as added | grey smoke covering everything, as without the effect | pass |
| `D-305 pictures/2_alpha_from_luminance.png` | Take Alpha From Luminance | the dark smoke gone, the blue backing showing through; white clouds left | pass |
| `D-305 pictures/3_alpha_from_luminance_red_full_blue_off.png` | also Red Full, Blue Off | the clouds turned warm orange and yellow; the gaps still blue | pass |

## For the owner to try

1. Put Fractal Noise on a white solid over a coloured one.
2. Add **Shift Channels** and set **Take Alpha From** to **Luminance**. The dark parts disappear.

No new FX fixtures; a fixture set with a reference script, as earlier effect batches had, can be a later unit. Fixtures are unchanged.
