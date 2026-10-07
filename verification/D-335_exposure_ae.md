# D-335 / B-217: Exposure's Offset, Gamma Correction and Bypass

From P-26's tutorial 2 (Advanced Electric), 2026-10-07.

## What was missing

Tutorial 2 brightens its ground texture with After Effects' Exposure at 2.47 and **Gamma Correction 1.69**. This program's Exposure had only the stops, so the ground came out darker and flatter than the tutorial's.

## What changed

Exposure now has After Effects' other three settings:

- **Offset**, -0.5 to 0.5: added to every colour after the brightening. Below 0 the darks go to black.
- **Gamma Correction**, 0.01 to 9.99: above 1 lifts the middle tones and leaves black and white where they are.
- **Bypass Linear Light Conversion**, Off or On: at 8 bpc and 32 bpc (After Effects) it works on the colours as you see them. In Float it changes nothing.

Offset and Gamma Correction can be keyframed. A setting is written into the project file only when it is not at its default, or when the file already had it, so every older project opens, renders and saves exactly as before.

**This program's own rule:** a colour that the offset takes below 0 keeps its minus sign through the gamma. After Effects' help says nothing about this case.

## Checks

`tests/b217_exposure_ae.rs` holds the app to `Fixtures/exposure_ae/expected_exposure_ae.json`, which `tools/exposure_ae_reference.py` wrote before the code existed. Results are in `verification/D-335_exposure_ae_table.md`: **53 of 53 pass**.

| Check | Result |
| --- | --- |
| FX-EXPAE-001 and 002: Gamma 1.69 alone; Offset 0.1 alone | matches |
| FX-EXPAE-003: all three in order, below 0 kept | matches |
| FX-EXPAE-004: the tutorial's ground, 2.47 and gamma 1.69, on half-covering patches | matches |
| FX-EXPAE-005 and 006: 32 bpc (After Effects), bypass off and on | matches |
| FX-EXPAE-007: bypass in Float changes nothing | matches |
| FX-EXPAE-008 and 009: 8 bpc, bypass off and on | matches |
| FX-EXPAE-010: gamma keyframed | matches |
| FX-EXPAE-011: the defaults written out give the old Exposure exactly | matches |
| FX-EXPAE-012 to 016: settings out of range are kept, left out of the frame, and warned about | matches |
| Opened and saved; drawn in tiles | the same |

## Pictures (`verification/D-335 pictures/`)

The test town in a 32 bpc (After Effects) composition.

| No Exposure | Exposure 2.47 alone (before D-335) |
| --- | --- |
| ![none](D-335%20pictures/1_none.png) | ![exposure](D-335%20pictures/2_exposure.png) |

| Exposure 2.47 + Gamma Correction 1.69 (tutorial 2's ground) | Offset -0.1 alone |
| --- | --- |
| ![gamma](D-335%20pictures/3_gamma.png) | ![offset](D-335%20pictures/4_offset.png) |

**What to look for:** with Gamma Correction added, the picture is lighter and flatter than with Exposure alone, and the dark windows open up. With Offset -0.1 the darks go black. On the road, the red reads 60 out of 255 with no Exposure, 131 with Exposure alone, 172 with Gamma Correction added, and 0 with Offset -0.1.

## How to try it in the app

Add Exposure to a layer. Below Exposure (stops) there are now Offset, Gamma Correction and Bypass Linear Light Conversion. Raise Gamma Correction to about 1.7: the middle tones lift. Move Offset below 0: the shadows go black.
