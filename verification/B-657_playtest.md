# B-657 playtest: Radio Waves, Parameters Are Set At and Sides up to 128 (D-777)

## What changed for you

Radio Waves has a new setting, **Parameters Are Set At**, as After Effects has:

- **Birth** (After Effects' own, and what a Radio Waves you add from now on uses): each ring keeps
  the settings it had when it was sent out. Move the Producer Point and the old rings stay where
  they were born, so a moving point draws a lattice of circles. Key the Opacity down and only the
  rings sent later are fainter.
- **Each Frame** (what ours always did): every ring follows the settings at the frame you are on,
  so moving the point moves all the rings with it.

With nothing keyed, the two look exactly the same.

**Sides** now goes up to 128, After Effects' top (it stopped at 64).

A project you saved before this change looks exactly as it did: it has no Parameters Are Set At in
the file, so it opens as Each Frame, and the panel shows Each Frame.

## What to look at

Each picture in `verification/B-657 pictures/` is After Effects on the left, ours before in the
middle, ours after on the right.

| Picture | Tutorial | What to judge | Measured | Verdict |
|---|---|---|---|---|
| `rw_mobox_birth.jpg` | MOBOX, uZlCVIRsKeA (panel "Birth", the point circling 400 px out) | After Effects draws a lattice of circles round many centres; ours before drew one set of rings round today's point; ours after draws the lattice | 46% of pixels changed; ring centres spread round the 400 px circle after, none before | close (the orbit speed is assumed, not read) |
| `rw_jake_128.jpg` | Jake In Motion, wf8XFPvUptc, about 4:05 (Sides 128) | rings as round as After Effects'; before, 128 was refused and the picture is at 64 | 1.5% of pixels changed, only at the ring edges | matches |
| `rw_eim_72.jpg` | Edit in Minutes, otdpArskcJ0, 2;08 (Sides 72) | as above at 72 | 0.65% of pixels changed, ring edges only | matches |
| `rw_truevis_birth.jpg` | True Visuals, offPhZy7-NY, 0:00:05:19 (panel "Birth", nothing keyed) | Birth with nothing keyed is the same picture as before | 0 pixels changed | matches |
| `rw_boone_birth.jpg` | Boone Loves Video, OkiI7OwJO94 (defaults) | the same | 0 pixels changed | matches |

The tables: `verification/B-657_radio_waves_birth_table.md` (FX-RWAVE-028 to 035, the old file, the
refusals; 39 of 39 pass), `verification/B-657_radio_waves_panel_table.md` (the window: a new one is
Birth, Each Frame and 128 typed in the panel are kept, "death" refused, an old project opens as Each
Frame and saves without the word), and `verification/B-135_radio_waves_table.md` (all 35 fixtures).

## Try it

1. Add Radio Waves to a layer. Parameters Are Set At shows Birth.
2. Key the Producer Point from the left side to the right side over two seconds and play: the rings
   stay where each was sent. Switch to Each Frame: all rings slide with the point.
3. Type 128 in Sides: kept. Type 129: refused.
4. Open a project saved before today with a Radio Waves: it looks the same and shows Each Frame.

## Known differences from After Effects (not fixed here)

- Sides, Profile and Colour follow the frame you are on even at Birth; no tutorial shows whether
  After Effects holds them too.
- Render Quality, Wave Type Image Contour and Mask, Star, Curve Size and Reflection are still
  missing.
- A build from before today opening a project set at Birth draws Each Frame without a warning (that
  older build does not know the word).
- Birth reads the settings once for every ring alive, so a very short Interval with a long Lifespan
  costs more each frame.
