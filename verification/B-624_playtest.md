# B-624 playtest: Wave Warp gains After Effects' Circle and Pinning (D-745)

## What to try

1. Add **Wave Warp** to a picture layer. Hover over **Width**: the hint reads "one wave spans Width pixels, about twice AE's Width for the same look". To copy an After Effects tutorial, type twice its Wave Width.
2. Set **Shape** to **Circle**. Each half wave becomes half a circle: rounded bites with steep sides instead of the sine's gentle slopes.
3. Set **Pinning** to **All Edges**. The layer's outline holds still; the wave fades in over the first quarter of the layer from each edge. Try **Left Edge**, **Top Edge**, **Right Edge**, **Bottom Edge**, **Horizontal Edges** (top and bottom) and **Vertical Edges** (left and right): only those edges hold still.
4. Open a project saved before this build that has a Wave Warp. It looks exactly as before and shows Pinning **None**. Saving it writes no pinning, so the file stays as it was.

Before this build, Wave Warp had only Sine and Triangle and no Pinning, so a tutorial using All Edges or Circle could not be copied.

## What the check found

Fixture check `tests/b624_wave_warp.rs`: 127 of 127 (see `B-624_wave_warp_table.md`). FX-WAVE-001 to 038 are pixel-compared with `tools/wave_warp_reference.py` (tolerance 2e-5). FX-WAVE-001 to 024 are the values from before D-745, unchanged, so nothing that drew before draws differently.

Other checks:
- An old project (`fx_wave_003.json`, no pinning) opens with pinning none and saves without one. A file that wrote pinning none keeps it.
- Pinning "center", "All Edges" or "", and shape "square", are refused with a sentence. Every pinning word and circle are taken.
- The picture is the same however it is cut into tiles.

`tests/b92_wave_warp.rs` and the card's check `tests/b107_gpu_fx.rs` also pass; the card's check now includes FX-WAVE-025 to 038 drawn on the graphics card. The app's tests: 112 passed, 5 ignored.

## Against the tutorials (three stills from Wave 1)

Each sheet shows three panels, all drawn by the real app on the CPU:
- **After Effects:** the still from the video.
- **Before:** the build before this one (old exe), at twice the tutorial's Width, with no Pinning or Circle.
- **After:** this build, at twice the Width, with the tutorial's Pinning or Circle.

Our purple stand-in is lighter than After Effects', so a plain grey difference mostly measures colour. The measure used instead is the share of the picture where the empty (black) area disagrees with After Effects'. Lower is closer.

| Still | Setting | AE empty | Before: empty, disagrees | After: empty, disagrees | Verdict |
| --- | --- | --- | --- | --- | --- |
| Motion Academy 5:30 | Sine, H100 W100, All Edges | 0.3 % | 5.7 %, 5.7 % | 0.0 %, 0.3 % | matches: no bites at the top and bottom, edges held |
| Motion Academy 2:00 | Circle, H100 W100 | 7.4 % | 5.7 %, 1.8 % (sine) | 7.1 %, 0.5 % | matches: the round bites are the same shape and size |
| FLOWTUTS 1:36 | Sine, H30 W250, All Edges | 29.7 % | 27.0 %, 3.6 % | 24.7 %, 5.1 % | close (see below) |

FLOWTUTS 1:36 is close rather than a match:
- Edge waviness (`wwedge.py`), measured as the spread of the rectangle's edge on screen:
  - After Effects: 3 px at both the top and the bottom.
  - Before (no pinning): 11 px.
  - After: 0 px at the top, 1 px at the bottom.
- Why ours is flatter: our stand-in rectangle reaches the composition's edge, where the pin is total. After Effects' rectangle sits about 8 screen pixels in from the edge.
- At that distance the rule predicts about 3 px of spread, which is what After Effects shows.
- The higher disagreement share comes from that stand-in placement, not from the pin.

On Motion Academy 5:30, After Effects' Wave Speed -1 has moved the wave by frame 11. Our phase is not After Effects', so phases 0, 165 and -165 were tried, and 165 was kept as the closest. The before panel uses the same phase.

## Pictures (`B-624 pictures/`)

- `ww_ma_t330_alledges.png`: All Edges. The old build cuts black bites into the top and bottom; After Effects and this build keep the edges straight and the wave inside.
- `ww_ma_t120_circle.png`: Circle. The old build could only show the sine's pointed bites; this build's round bites match After Effects'.
- `ww_flow_t96_alledges.png`: All Edges on a white rectangle. The old build's top and bottom wave; After Effects' and this build's are nearly flat.

## How this differs from After Effects

- Width is one whole wave here, After Effects' Wave Width half of one. Type twice After Effects' number. This is kept on purpose so no saved project changes.
- After Effects' other wave types (Square, Sawtooth, Semicircle, Uncircle, Noise, Smooth Noise), Pinning Center and Antialiasing are not added. Our Triangle is kept as it was.
- No tutorial shows Horizontal Edges or Vertical Edges. We read them as the edges that run that way: top and bottom, and left and right.
- The quarter-of-the-layer fade is read off one tutorial's grid (Motion Academy). FLOWTUTS agrees with it.
- HOOP Studios' Left Edge (1:10) has no saved still, so it is not on a sheet.
