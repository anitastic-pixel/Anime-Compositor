# B-634 playtest: Wave Warp gains After Effects' remaining wave types (D-755)

## What to try

1. Add **Wave Warp** to a picture layer. Open **Shape**. It now lists After Effects' nine types in After Effects' order: Sine, Square, Triangle, Sawtooth, Circle, Semicircle, Uncircle, Noise and Smooth Noise.
2. Pick **Square**. The layer is pushed one way for half of each wave and the other way for the other half, so its edges step in blocks.
3. Pick **Sawtooth**. The push climbs evenly across each wave and drops back at once, so the edges become slanted teeth.
4. Pick **Semicircle**. Each wave is one round arch, and the arches meet in sharp points. **Uncircle** is the same arch turned over.
5. Pick **Noise**, then **Smooth Noise**. The push is uneven: jagged for Noise, rolling for Smooth Noise. Play the composition with a **Speed** other than 0 and the pattern slides along. It is the same on every play and on every machine.
6. Open a project saved before this build that has a Wave Warp. It looks exactly as before, and saving it writes the same file.

Before this build, any of these six shapes was refused: the effect was kept in the file but left out of the picture, with a warning.

## What the check found

Fixture check `tests/b634_wave_warp.rs`: 148 of 148 (see `B-634_wave_warp_table.md`). FX-WAVE-001 to 047 are pixel-compared with `tools/wave_warp_reference.py` (tolerance 2e-5; the largest difference is 2.5e-7). FX-WAVE-001 to 038 draw the same values as before D-755. FX-WAVE-023 still tests a refused shape, but its word changed from "square", which is now a choice, to "hexagon". Its expected picture is the same.

Other checks:
- An old project (`fx_wave_003.json`) opens as a sine, as it always did. A circle saved before this build saves again as a circle.
- Every fixture file reads and writes back unchanged, including the new ones.
- The shapes "Smooth Noise", "Square" and "hexagon" are refused with a sentence. The words are exact: the panel's names are not the stored words. All six new words are taken.
- The picture is the same however it is cut into tiles.

`tests/b92_wave_warp.rs`, `tests/b624_wave_warp.rs` and the card's check `tests/b107_gpu_fx.rs` also pass. The card's check now includes FX-WAVE-039 to 047 drawn on the graphics card: no pixel is more than 1 level apart, and 7352 of 7352 checks pass. The app's tests: 113 passed, 5 ignored.

## Against the tutorial (Motion Academy, Wave Warp at 1:20, 1:48 and 2:04)

Each sheet has three panels:
- **After Effects:** the still from the video. The settings were Height 100, Width 100, Direction 90, Pinning None.
- **Before:** the earlier build, drawn by the real app on the CPU. It refuses the shape (`EFFECT_PARAMETER_INVALID`, "shape is sine or triangle"), so the layer is drawn flat.
- **After:** this build, at height 100 and width 200 (twice After Effects' Width, as D-745 says).

Our purple stand-in is lighter than After Effects', so colour is not compared. The measure is each picture's shape:
- Read each column's top edge minus its bottom edge, divided by the height.
- Correlate that outline between After Effects' still and our picture.
- Separately, fit every candidate shape to After Effects' still at every whole degree of phase.

| Still | After Effects' best fitting shape and phase | Ours: best shape and phase | AE vs ours, correlation | Average edge difference | Verdict |
| --- | --- | --- | --- | --- | --- |
| 2:44 Sine (control, unchanged) | sine, 0° (0.990) | sine, 0° | 0.990 | 5.1 px | matches (before and after identical) |
| 1:20 Square | square, 359° (0.981) | square, 0° | 0.981 | 6.7 px | matches |
| 1:48 Sawtooth | rising sawtooth, 359° (0.968); falling only 0.486 | rising sawtooth, 0° | 0.968 | 6.3 px | matches |
| 2:04 Semicircle | arch pushed away from points, 0° (0.977); turned over 0.746 | the same, 0° | 0.976 | 6.7 px | matches |
| 2:12 Noise | no shape fits (best 0.36) | no shape fits | 0.03 | 41.9 px | differs: in spirit only |

The edge differences are in composition pixels on a wave 100 pixels high. The sine control, which nothing in this build changes, shows 5.1 px. That is the floor set by the video's softness, so square, sawtooth and semicircle sit within about 1.5 px of it.

After Effects' Noise is random and changes every frame, with fine spikes a few pixels apart. Ours is fixed: a new level every sixteenth of a wave, joined by straight lines. It is not claimed to match.

## Pictures (`B-634 pictures/`)

- `ww_square_3way.png`: Square. The old build draws the layer flat. After Effects and this build cut the same square blocks into the top and bottom, at the same places.
- `ww_sawtooth_3way.png`: Sawtooth. After Effects and this build both show slanted teeth that lean the same way.
- `ww_semicircle_3way.png`: Semicircle. After Effects and this build both show round arches at the bottom and sharp points at the top.

## How this differs from After Effects

- Width is one whole wave here, and After Effects' Wave Width is half of one, as since D-745.
- Noise and Smooth Noise are fixed patterns, not random ones, so a render is the same every time. No tutorial shows Smooth Noise.
- No tutorial shows Uncircle. Ours is the Semicircle turned over.
- Our Triangle is still a quarter wave from After Effects' Triangle, kept as it was.
- After Effects' Antialiasing is not added.
