# B-291: Map Chromatic Displacement

Built on 2026-10-10 as D-412, under your /loop request: your plugin pick #19, Map Chromatic
Displacement, where a displacement moves red, green and blue by different amounts so the edges
fringe into colour, as light does through water or glass. PLUGINS.md said to merge it into
Displacement Map, so it is not a new effect: **Displacement Map** (in **Distort**) gains four
settings.

- **Red Amount**, **Green Amount**, **Blue Amount** (100 each): how far each colour is moved, in
  per cent of the displacement the other settings give. All at 100 is Displacement Map as it was.
  40, 100, 160 moves red less and blue more than the rest.
- **Spectrum** (3): how many colour samples run from red to blue. 3 parts the picture into three
  hard colour fringes; 12 to 16 gives a smooth rainbow edge.

A Displacement Map in a project saved before this has every colour at 100 and draws exactly as it
did.

The check, `verification/D-412_map_chromatic_table.md` (175 of 175), holds every pixel to
numbers worked out by a separate program before the code existed, checks that all older
Displacement Map test files open, save and draw exactly as before, and holds the graphics card's
picture to within 1 level of the processor's. The pictures are in `verification/D-412 pictures/`
(`town.png` is the layer, `map_waves.png` and `map_ball.png` the two maps).

## What to check

1. **Alike.** `1_waves_alike.png` is the town moved 8 pixels by the waves, every colour at 100:
   the wobble with no colour fringes, as Displacement Map always drew it.
2. **Split.** `2_waves_split.png` is the same with Red 40, Green 100, Blue 160, Spectrum 3: the
   edges show separate red and blue fringes either side.
3. **Rainbow.** `3_waves_spectrum.png` is the same with Spectrum 16: the fringes become a smooth
   rainbow, like light through rippling water.
4. **Glass ball.** `4_ball_alike.png` is the town seen through a glass ball map, 30 pixels, every
   colour at 100; `5_ball_prism.png` is Red 70, Green 100, Blue 130, Spectrum 12: the ball's rim
   splits into a rainbow, as real glass does.
5. **In the app.** Add Displacement Map to a layer with a map layer chosen; set Blue Amount to
   150: blue fringes appear on one side. Raise Spectrum to 12: they smooth into a rainbow. Key
   Red Amount from 100 to 50 and play: the fringe grows.
6. **An older project.** Open a project saved before today with a Displacement Map in it: the
   three amounts show 100, Spectrum 3, and the picture is the same as before.
7. **Out of range.** Type 1001 in Red Amount: it is refused with a sentence saying it runs from
   -1000 to 1000, and the card keeps its old number.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-291_map_chromatic_timing_table.md` (quiet machine): the reference shot with a moving Noise and a Displacement Map on three
layers, played again, 53.1 ms a frame on the card with the colours parted into three against
52.5 with them alike, and 66.2 with a 16-sample rainbow; on the processor 83.4, 76.9 and 145.4.
