# B-302: Light Rays with CC Light Rays' settings

Built on 2026-10-10 as D-423, under your /loop request: Light Rays, in **Generate**, now has the
settings of CycoreFX's CC Light Rays. Light comes from a round or square source and streams outward
from it as rays.

- **Intensity** (100): how bright the rays are; 0 leaves the layer as it was.
- **Center** (50, 50 per cent): where the light source sits.
- **Radius** (50 pixels): how big the source is; bigger reaches further and is brighter.
- **Warp Softness** (50): melts neighbouring rays together into larger, softer rays.
- **Shape** (Round): Round or Square.
- **Direction** (0 degrees): turns a square source; it does nothing for a round one.
- **Color from Source** (On): the rays take the layer's own colours; off, they are all **Color**.
- **Allow Brightening** (On): off, Intensity above 100 does no more than 100.
- **Color** (white): the rays' colour when Color from Source is off.
- **Transfer Mode** (None): None lays the rays over the layer, Add adds them, Screen screens them,
  Lighten keeps the lighter of the two.

CycoreFX's manual says what each setting does in words but gives no formula, ranges or starting
values, so those are ours, and the look is not matched pixel for pixel. How Allow Brightening, Warp
Softness and Transfer Mode None work is our reading of the manual's words: all logged.

**Your older projects.** A Light Rays saved before today keeps its old settings (Length,
Threshold, Intensity, Color) and draws exactly as before. A new Light Rays gets the new settings.

The check, `verification/D-423_light_rays_cc_table.md` (277 of 277), holds every pixel to numbers
worked out by a separate program before the code existed, checks that every older Light Rays test
project opens and saves unchanged, and holds the graphics card's picture to within 1 level of the
processor's. The pictures are in `verification/D-423 pictures/` (`town.png` is the layer).

## What to check

1. **Before.** `1_before.png`: the street with no effect.
2. **As added.** `2_as_added.png`: a small round source in the middle of the picture; its light
   streams outward as soft rays laid over the street. The blue house's left edge runs exactly
   through the centre, so it stays sharp: rays only run straight out from the centre.
3. **Bigger source.** `3_big_source.png`, Radius 300, Warp Softness 200: the rays reach the whole
   picture, melted into broad soft streaks.
4. **Square.** `4_square.png`, a square source 200 turned 30 degrees, no warp: hard streaks
   fanning out across the whole picture.
5. **Orange, screened.** `5_orange_screen.png`, Color from Source off, orange, Radius 250, Screen,
   centre 30, 40: the street washed with orange light, fading toward the edges; the street is
   solid, so there are no streaks (on text or a logo the rays take its shape).
6. **Added, bright.** `6_add_bright.png`, Intensity 300, Radius 250, Add: the middle blown out to
   white.
7. **In the app.** Add a text layer with a word, add Light Rays (Generate) to it, and drag Center:
   the rays follow. Raise Radius and Warp Softness, switch Shape to Square and turn Direction, turn
   Color from Source off and pick a colour, and try each Transfer Mode.
8. **An older project.** Open a project saved before today that holds a Light Rays: it shows
   Length, Threshold, Intensity and Color, and looks as it did.
9. **Out of range.** Type 10001 in Radius: it is refused with a sentence saying it runs from 0 to
   10000.
10. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-302_light_rays_cc_timing_table.md` (card quiet, processor PROVISIONAL): the reference shot with a
moving Noise and a Light Rays on three layers, played again, 36.3 ms a frame on the card as
added against 12.3 for the Noise alone; on the processor 616.0 against 41.1.
Radius 400, Warp Softness 300, Add takes 49.2 on the card against the processor's 1905.5:
the card is quicker than the processor for this effect.
