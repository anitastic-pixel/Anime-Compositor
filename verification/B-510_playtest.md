# B-510: Wave World

Built on 2026-10-12 as D-630, the first of the P0-17 group (D-600, your plan: "how AE treats
them/are done"), approved by the coordinator for your /loop request: Wave World, under
**Generate** next to CC Rainfall, our version of After Effects' effect of that name. It makes
water on a grid: ripples spread from up to two producers, bounce off a shore and die away. The
layer's own picture is replaced by a grey map of the water's height (bright crests, dark
troughs), which is meant to feed another effect, such as a Displacement Map, later Caustics.

- **Brightness** (0.5), **Contrast** (0.5), **Gamma Adjustment** (1): the grey of still water,
  how strongly the waves lighten and darken it, and the middle greys.
- **Render Dry Areas As** (Level) and **Transparency** (0): land drawn as still water or left
  clear; how much shallow water fades.
- **Grid Resolution** (60) and **Grid Res Downsamples** (Off): how fine the water is (finer is
  slower); On makes a Draft preview coarser too.
- **Wave Speed** (0.25), **Damping** (0.05), **Reflect Edges** (None), **Pre-roll** (0 seconds):
  how fast waves cross, how fast they die, whether they bounce off the picture's edges, and how
  long the water runs before the layer starts.
- **Ground Layer**, **Steepness** (0.25), **Height** (0.5), **Wave Strength** (0): a picture whose
  bright parts raise the floor, so waves slow in the shallows and stop at a shore.
- **Producer 1 and 2**: Type (Ring or Line), Position, Height/Length, Width, Angle, Amplitude,
  Frequency, Phase. Producer 2 is off when added (Amplitude 0).

Adobe gives no formula, ranges or settings when added, so those are ours. Each frame runs the
water again from the layer's start, so frames late in a long layer take longer to draw (see
Speed). Not built yet: Wireframe Preview and its three controls (a later unit, D-635), and lights
from the composition.

The check, `verification/D-630_wave_world_table.md` (207 of 207), holds every pixel to numbers
worked out by a separate program before the code existed. The pictures are in
`verification/D-630 pictures/`, drawn 480 by 270.

## What to check

1. **As added.** `1_as_added_ring.png` (frame 36): grey water with faint rings spreading from the
   middle.
2. **Two producers.** `2_two_producers_crossing.png`: two sets of rings, left and right, beating at
   different speeds, crossing in the middle.
3. **A line that bounces.** `3_line_reflect_edges.png`: a tilted line making straight waves, which
   bounce off the picture's edges into a checked pattern.
4. **An island.** `4_shore_round_an_island.png`: big rings from the left; a flat grey island on the
   right (a bright logo used as the ground) that the waves do not cross, with only faint ripples
   behind it.
5. **Land clear.** `5_island_clear.png`: the same, with the island left clear (see-through), which a
   picture viewer shows as white or a checkerboard.
6. **Shallows.** `6_shallows_fading.png`: the island now under shallow water, so waves pass over it
   and it shows as a fainter patch, its water faded by Transparency.
7. **In the app.** Add Wave World (Generate) to a layer and play: rings spread from the middle and
   keep coming. Move Producer 1's position and the rings start from there.
8. **With a displacement.** Put Wave World on a layer, switch that layer's eye off, and give the
   layer above a Displacement Map that reads it: the picture should ripple like water.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: how strong the ripples look as added, how fast they
spread, and how quickly they die away.

## Speed

Measured on a busy machine, so the numbers are provisional
(`verification/B-510_wave_world_timing_table.md`): as added, 10 seconds into the layer, Wave World
added roughly 80 to 140 ms to a frame of the reference shot; at Grid 145 roughly 370 to 430 ms.
The cost grows the further the frame is from the layer's start. Playing in the app is the real
test: if a long Wave World layer feels too slow, say so, and a store of the water every so many
frames can be added (it would give the same pictures).
